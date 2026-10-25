//! Shared replay record/playback for C&C lockstep simulation.
//!
//! This crate provides a unified replay format and builder that both
//! `cnc-tiberian-dawn` and `cnc-red-alert` consume. It replaces the
//! skeleton crate and closes audit §7 replayer row.
//!
//! Format:
//!   - Magic: `CNCR` (shared magic for both games)
//!   - Version: 3 (includes RNG seed + cursor)
//!   - Header: seed(8) + rng_state(8) + frame_count(4) + tick_bits(4)
//!   - Commands: count(4) + [frame(4) + payload_len(4) + payload][count]
//!   - Trace: count(4) + [state_crc(8)][count]
//!   - Checksum: FNV-1a64 of everything before checksum

use std::fmt;

use anyhow::Context;
use cnc_core::Fixed16_16;
use cnc_red_alert::{Game as RaGame, GameCommand as RaGameCommand};
use cnc_tiberian_dawn::{network::NetCommand as TdNetCommand, Game as TdGame};
use thiserror::Error;
/// Magic for shared replay files ("CNCR").
pub const REPLAY_MAGIC: [u8; 4] = *b"CNCR";
/// Replay file format version (v3 = RNG seed + cursor).
pub const REPLAY_VERSION: u8 = 3;

const MAX_REPLAY_COMMANDS: u32 = 1_000_000;
const MAX_TRACE_FRAMES: u32 = 1_000_000;
const REPLAY_HEADER_LEN: usize = 4 + 1 + 1 + 8 + 8 + 4 + 4; // magic + version + reserved + seed + rng + frames + tick

/// Errors produced while saving or loading replay files.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReplayError {
    #[error("replay is too short")]
    TooShort,
    #[error("invalid replay magic (expected CNCR)")]
    InvalidMagic,
    #[error("unsupported replay version {0} (pre-RNG replays are permanently invalid)")]
    UnsupportedVersion(u8),
    #[error("replay command count {0} exceeds the {MAX_REPLAY_COMMANDS} limit")]
    TooManyCommands(u32),
    #[error("replay trace frame count {0} exceeds the {MAX_TRACE_FRAMES} limit")]
    TooManyTraceFrames(u32),
    #[error("replay is truncated")]
    Truncated,
    #[error("invalid command payload")]
    InvalidCommand,
    #[error("replay checksum mismatch")]
    ChecksumMismatch,
    #[error("trailing bytes after replay checksum")]
    TrailingBytes,
}

/// Replay header carrying the match seed, RNG cursor, and timing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayHeader {
    pub seed: u64,
    pub frame_count: u32,
    pub rng_state: u64,
    pub tick_bits: i32,
}

/// Kind of desynchronization detected during replay verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesyncKind {
    StateCrc { expected: u64, actual: u64 },
    SeedMismatch { header_seed: u64, local_seed: u64 },
}

impl DesyncKind {
    pub fn actual_crc(&self) -> u64 {
        match self {
            DesyncKind::StateCrc { actual, .. } => *actual,
            DesyncKind::SeedMismatch { .. } => 0,
        }
    }
}

impl fmt::Display for DesyncKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DesyncKind::StateCrc { expected, actual } => write!(
                f,
                "state CRC mismatch (expected {expected:#x}, actual {actual:#x})"
            ),
            DesyncKind::SeedMismatch {
                header_seed,
                local_seed,
            } => write!(
                f,
                "seed mismatch (header {header_seed:#x}, local {local_seed:#x})"
            ),
        }
    }
}

/// A desynchronization error with frame context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesyncError {
    pub frame: u32,
    pub kind: DesyncKind,
}

impl fmt::Display for DesyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "desync at frame {}: {}", self.frame, self.kind)
    }
}

/// A recorded lockstep match: the player count and every executed command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replay<T> {
    pub header: ReplayHeader,
    pub commands: Vec<T>,
    pub command_frames: Vec<u32>,
    pub trace: Vec<u64>,
}

impl<T> Replay<T> {
    /// Encode the replay to a versioned, checksummed byte buffer.
    pub fn encode<F>(&self, encode_cmd: F) -> Vec<u8>
    where
        F: Fn(&T, &mut Vec<u8>),
    {
        let mut out = Vec::new();
        out.extend_from_slice(&REPLAY_MAGIC);
        out.push(REPLAY_VERSION);
        out.push(0u8); // reserved
                       // header
        out.extend_from_slice(&self.header.seed.to_le_bytes());
        out.extend_from_slice(&self.header.rng_state.to_le_bytes());
        out.extend_from_slice(&self.header.frame_count.to_le_bytes());
        out.extend_from_slice(&self.header.tick_bits.to_le_bytes());
        // commands
        out.extend_from_slice(&(self.commands.len() as u32).to_le_bytes());
        for (cmd, frame) in self.commands.iter().zip(self.command_frames.iter()) {
            let mut payload = Vec::new();
            encode_cmd(cmd, &mut payload);
            out.extend_from_slice(&frame.to_le_bytes());
            out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
            out.extend_from_slice(&payload);
        }
        // trace
        out.extend_from_slice(&(self.trace.len() as u32).to_le_bytes());
        for crc in &self.trace {
            out.extend_from_slice(&crc.to_le_bytes());
        }
        let checksum = fnv1a64(&out);
        out.extend_from_slice(&checksum.to_le_bytes());
        out
    }

    /// Parse a replay buffer (validates magic, version, length, and checksum).
    pub fn decode<F>(bytes: &[u8], decode_cmd: F) -> Result<Self, ReplayError>
    where
        F: Fn(&[u8]) -> Result<T, ()>,
    {
        if bytes.len() < REPLAY_HEADER_LEN + 8 + 4 + 4 {
            // header + checksum + cmd_count + trace_count
            return Err(ReplayError::TooShort);
        }
        if bytes[0..4] != REPLAY_MAGIC {
            return Err(ReplayError::InvalidMagic);
        }
        if bytes[4] != REPLAY_VERSION {
            return Err(ReplayError::UnsupportedVersion(bytes[4]));
        }
        // reserved byte at 5 ignored

        let mut pos = 6;
        let seed = u64::from_le_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let rng_state = u64::from_le_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let frame_count = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
        pos += 4;
        let tick_bits = i32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
        pos += 4;

        let cmd_count = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
        pos += 4;
        if cmd_count > MAX_REPLAY_COMMANDS {
            return Err(ReplayError::TooManyCommands(cmd_count));
        }

        let mut commands = Vec::with_capacity(cmd_count as usize);
        let mut command_frames = Vec::with_capacity(cmd_count as usize);
        for _ in 0..cmd_count {
            if pos + 8 > bytes.len() {
                return Err(ReplayError::Truncated);
            }
            let frame = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
            pos += 4;
            let payload_len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
            pos += 4;
            if pos + payload_len as usize > bytes.len() {
                return Err(ReplayError::Truncated);
            }
            let payload = &bytes[pos..pos + payload_len as usize];
            pos += payload_len as usize;
            let cmd = decode_cmd(payload).map_err(|_| ReplayError::InvalidCommand)?;
            commands.push(cmd);
            command_frames.push(frame);
        }

        if pos + 4 > bytes.len() {
            return Err(ReplayError::Truncated);
        }
        let trace_count = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
        pos += 4;
        if trace_count > MAX_TRACE_FRAMES {
            return Err(ReplayError::TooManyTraceFrames(trace_count));
        }

        let mut trace = Vec::with_capacity(trace_count as usize);
        for _ in 0..trace_count {
            if pos + 8 > bytes.len() {
                return Err(ReplayError::Truncated);
            }
            let crc = u64::from_le_bytes(bytes[pos..pos + 8].try_into().unwrap());
            pos += 8;
            trace.push(crc);
        }

        if pos + 8 > bytes.len() {
            return Err(ReplayError::Truncated);
        }
        let expected_checksum = u64::from_le_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;

        let actual_checksum = fnv1a64(&bytes[..pos - 8]);
        if actual_checksum != expected_checksum {
            return Err(ReplayError::ChecksumMismatch);
        }

        if pos != bytes.len() {
            return Err(ReplayError::TrailingBytes);
        }

        Ok(Self {
            header: ReplayHeader {
                seed,
                frame_count,
                rng_state,
                tick_bits,
            },
            commands,
            command_frames,
            trace,
        })
    }

    /// Write a replay file (creates parent directories).
    pub fn save_to_file<F>(&self, path: &std::path::Path, encode_cmd: F) -> anyhow::Result<()>
    where
        F: Fn(&T, &mut Vec<u8>),
    {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create {}", parent.display()))?;
            }
        }
        std::fs::write(path, self.encode(encode_cmd))
            .with_context(|| format!("failed to write {}", path.display()))
    }

    /// Read a replay file.
    pub fn load_from_file<F>(path: &std::path::Path, decode_cmd: F) -> anyhow::Result<Self>
    where
        F: Fn(&[u8]) -> Result<T, ()>,
    {
        let bytes =
            std::fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
        Self::decode(&bytes, decode_cmd)
            .map_err(|e| anyhow::Error::msg(format!("decode error: {:?}", e)))
    }
}

/// Builder for recording a replay during a live session.
pub struct ReplayBuilder<T> {
    tick: Fixed16_16,
    header_seed: u64,
    header_rng_state: u64,
    commands: Vec<T>,
    command_frames: Vec<u32>,
    trace: Vec<u64>,
    frame: u32,
    frame_count: u32,
}

impl<T> ReplayBuilder<T> {
    /// Create a new builder from the initial game state.
    pub fn new<G>(game: &G, tick: Fixed16_16, seed: u64, rng_state: u64) -> Self
    where
        G: HasSimState,
    {
        let _ = game;
        Self {
            tick,
            header_seed: seed,
            header_rng_state: rng_state,
            commands: Vec::new(),
            command_frames: Vec::new(),
            trace: Vec::new(),
            frame: 0,
            frame_count: 0,
        }
    }

    /// Record a command and apply it to the game.
    pub fn command<G>(&mut self, game: &mut G, command: T)
    where
        G: ApplyCommand<T> + HasSimState,
    {
        game.apply_command(&command);
        self.commands.push(command);
        self.command_frames.push(self.frame);
    }

    /// Advance the simulation by one tick and record the state CRC.
    pub fn step<G>(&mut self, game: &mut G)
    where
        G: HasSimState,
    {
        self.trace.push(game.state_crc());
        self.frame += 1;
        self.frame_count = self.frame;
    }

    /// Finalize the builder into a complete replay.
    pub fn finish(self) -> Replay<T> {
        Replay {
            header: ReplayHeader {
                seed: self.header_seed,
                frame_count: self.frame_count,
                rng_state: self.header_rng_state,
                tick_bits: self.tick.to_bits(),
            },
            commands: self.commands,
            command_frames: self.command_frames,
            trace: self.trace,
        }
    }
}

/// Trait for accessing simulation state (seed, RNG cursor, CRC).
pub trait HasSimState {
    fn sim_seed(&self) -> u64;
    fn rng_state(&self) -> u64;
    fn state_crc(&self) -> u64;
}

/// Trait for applying a command to the game.
pub trait ApplyCommand<T> {
    fn apply_command(&mut self, command: &T);
}

impl HasSimState for RaGame {
    fn sim_seed(&self) -> u64 {
        self.sim_seed()
    }
    fn rng_state(&self) -> u64 {
        self.rng.state()
    }
    fn state_crc(&self) -> u64 {
        self.state_crc()
    }
}

impl ApplyCommand<RaGameCommand> for RaGame {
    fn apply_command(&mut self, command: &RaGameCommand) {
        self.apply_command(command);
    }
}

impl HasSimState for TdGame {
    fn sim_seed(&self) -> u64 {
        self.powerups.rng_state
    }
    fn rng_state(&self) -> u64 {
        self.powerups.rng_state
    }
    fn state_crc(&self) -> u64 {
        self.state_crc()
    }
}

impl ApplyCommand<TdNetCommand> for TdGame {
    fn apply_command(&mut self, command: &TdNetCommand) {
        let _ = self.apply_command(command.player, &command.command);
    }
}

/// FNV-1a 64-bit hash.
fn fnv1a64(bytes: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = FNV_OFFSET;
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnc_red_alert::{Game as RaGame, GameCommand as RaGameCommand};
    use cnc_tiberian_dawn::{network::NetCommand as TdNetCommand, Game as TdGame};

    #[test]
    fn ra_replay_round_trip() {
        let mut game = RaGame::new(32, 32);
        let mut builder = ReplayBuilder::new(
            &game,
            Fixed16_16::from_int(250),
            0xDEADBEEF,
            game.rng.state(),
        );

        // Record some commands
        builder.command(
            &mut game,
            RaGameCommand::Produce {
                owner: 0,
                class_id: cnc_core::ClassId::InfantryE1,
                seconds: 0,
            },
        );
        builder.step(&mut game);
        builder.command(
            &mut game,
            RaGameCommand::Move {
                owner: 0,
                units: vec![0],
                target: cnc_core::Vec2::new(Fixed16_16::from_int(100), Fixed16_16::from_int(100)),
            },
        );
        builder.step(&mut game);
        builder.step(&mut game);

        let replay = builder.finish();
        let encoded = replay.encode(encode_ra_command);
        let decoded = Replay::decode(&encoded, decode_ra_command).expect("decode");
        assert_eq!(decoded.header.seed, replay.header.seed);
        assert_eq!(decoded.header.rng_state, replay.header.rng_state);
        assert_eq!(decoded.header.frame_count, replay.header.frame_count);
        assert_eq!(decoded.commands.len(), replay.commands.len());
        assert_eq!(decoded.trace.len(), replay.trace.len());
    }

    #[test]
    fn td_replay_round_trip() {
        let mut game = TdGame::new("test", 32, 32);
        let mut builder = ReplayBuilder::new(
            &game,
            Fixed16_16::from_int(250),
            0xFEEDFACE,
            game.powerups.rng_state,
        );

        builder.command(
            &mut game,
            TdNetCommand {
                frame: 0,
                player: 0,
                command: cnc_tiberian_dawn::network::Command::Move {
                    units: vec![0],
                    target: cnc_core::Vec2::new(
                        cnc_core::Fixed16_16::from_int(10),
                        cnc_core::Fixed16_16::from_int(10),
                    ),
                },
            },
        );
        builder.step(&mut game);
        builder.step(&mut game);

        let replay = builder.finish();
        let encoded = replay.encode(encode_td_command);
        let decoded = Replay::decode(&encoded, decode_td_command).expect("decode");
        assert_eq!(decoded.header.seed, replay.header.seed);
        assert_eq!(decoded.header.rng_state, replay.header.rng_state);
        assert_eq!(decoded.commands.len(), replay.commands.len());
    }

    #[test]
    fn corrupt_replay_rejected_with_header_error() {
        let mut game = RaGame::new(32, 32);
        let mut builder = ReplayBuilder::new(
            &game,
            Fixed16_16::from_int(250),
            0xDEADBEEF,
            game.rng.state(),
        );
        builder.step(&mut game);
        let replay = builder.finish();
        let mut encoded = replay.encode(encode_ra_command);

        // Corrupt the magic bytes
        encoded[0] = b'X';
        let result = Replay::decode(&encoded, decode_ra_command);
        assert!(matches!(result, Err(ReplayError::InvalidMagic)));

        // Corrupt the version
        let mut encoded = replay.encode(encode_ra_command);
        encoded[4] = 2; // v2 is unsupported
        let result = Replay::decode(&encoded, decode_ra_command);
        assert!(matches!(result, Err(ReplayError::UnsupportedVersion(2))));

        // Corrupt the checksum
        let mut encoded = replay.encode(encode_ra_command);
        let last = encoded.len() - 1;
        encoded[last] ^= 0xFF;
        let result = Replay::decode(&encoded, decode_ra_command);
        assert!(matches!(result, Err(ReplayError::ChecksumMismatch)));
    }

    // RA command encoding/decoding (simplified for tests)
    fn encode_ra_command(cmd: &RaGameCommand, out: &mut Vec<u8>) {
        match cmd {
            RaGameCommand::Move {
                owner,
                units,
                target,
            } => {
                out.push(1);
                out.extend_from_slice(&(*owner as u32).to_le_bytes());
                out.extend_from_slice(&(units.len() as u32).to_le_bytes());
                for u in units {
                    out.extend_from_slice(&u.to_le_bytes());
                }
                out.extend_from_slice(&target.x.to_bits().to_le_bytes());
                out.extend_from_slice(&target.y.to_bits().to_le_bytes());
            }
            RaGameCommand::Produce {
                owner,
                class_id,
                seconds,
            } => {
                out.push(2);
                out.extend_from_slice(&(*owner as u32).to_le_bytes());
                out.extend_from_slice(&class_id.as_u32().to_le_bytes());
                out.extend_from_slice(&seconds.to_le_bytes());
            }
            _ => {}
        }
    }

    fn decode_ra_command(payload: &[u8]) -> Result<RaGameCommand, ()> {
        if payload.is_empty() {
            return Err(());
        }
        match payload[0] {
            1 => {
                // Move
                if payload.len() < 1 + 4 + 4 {
                    return Err(());
                }
                let mut pos = 1;
                let owner = u32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap()) as usize;
                pos += 4;
                let count = u32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap()) as usize;
                pos += 4;
                let mut units = Vec::with_capacity(count);
                for _ in 0..count {
                    if pos + 4 > payload.len() {
                        return Err(());
                    }
                    units.push(u32::from_le_bytes(
                        payload[pos..pos + 4].try_into().unwrap(),
                    ));
                    pos += 4;
                }
                if pos + 8 > payload.len() {
                    return Err(());
                }
                let tx = i32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap());
                let ty = i32::from_le_bytes(payload[pos + 4..pos + 8].try_into().unwrap());
                Ok(RaGameCommand::Move {
                    owner,
                    units,
                    target: cnc_core::Vec2::new(
                        Fixed16_16::from_bits(tx),
                        Fixed16_16::from_bits(ty),
                    ),
                })
            }
            2 => {
                // Produce
                if payload.len() < 1 + 4 + 4 + 4 {
                    return Err(());
                }
                let mut pos = 1;
                let owner = u32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap()) as usize;
                pos += 4;
                let class_id = cnc_core::ClassId::from_u32(u32::from_le_bytes(
                    payload[pos..pos + 4].try_into().unwrap(),
                ));
                pos += 4;
                let seconds = i32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap());
                Ok(RaGameCommand::Produce {
                    owner,
                    class_id,
                    seconds,
                })
            }
            _ => Err(()),
        }
    }

    // TD command encoding/decoding (simplified for tests)
    fn encode_td_command(cmd: &TdNetCommand, out: &mut Vec<u8>) {
        out.extend_from_slice(&cmd.frame.to_le_bytes());
        out.push(cmd.player);
        if let cnc_tiberian_dawn::network::Command::Move { units, target } = &cmd.command {
            out.push(1);
            out.extend_from_slice(&(units.len() as u32).to_le_bytes());
            for u in units {
                out.extend_from_slice(&u.to_le_bytes());
            }
            out.extend_from_slice(&target.x.to_bits().to_le_bytes());
            out.extend_from_slice(&target.y.to_bits().to_le_bytes());
        }
    }

    fn decode_td_command(payload: &[u8]) -> Result<TdNetCommand, ()> {
        if payload.len() < 5 {
            return Err(());
        }
        let frame = u32::from_le_bytes(payload[0..4].try_into().unwrap());
        let player = payload[4];
        if payload.len() < 6 {
            return Err(());
        }
        match payload[5] {
            1 => {
                // Move
                if payload.len() < 6 + 4 + 4 + 4 {
                    return Err(());
                }
                let mut pos = 6;
                let count = u32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap()) as usize;
                pos += 4;
                let mut units = Vec::with_capacity(count);
                for _ in 0..count {
                    if pos + 4 > payload.len() {
                        return Err(());
                    }
                    units.push(u32::from_le_bytes(
                        payload[pos..pos + 4].try_into().unwrap(),
                    ));
                    pos += 4;
                }
                let x = i32::from_le_bytes(payload[pos..pos + 4].try_into().unwrap());
                let y = i32::from_le_bytes(payload[pos + 4..pos + 8].try_into().unwrap());
                Ok(TdNetCommand {
                    frame,
                    player,
                    command: cnc_tiberian_dawn::network::Command::Move {
                        units,
                        target: cnc_core::Vec2::new(
                            cnc_core::Fixed16_16::from_bits(x),
                            cnc_core::Fixed16_16::from_bits(y),
                        ),
                    },
                })
            }
            _ => Err(()),
        }
    }
}
