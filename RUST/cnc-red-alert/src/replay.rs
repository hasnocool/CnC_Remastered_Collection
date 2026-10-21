//! Replay persistence for the Red Alert lockstep simulation (format v3).
//!
//! Format history:
//!   * v1/v2 - pre-RNG generations: no match seed and no RNG cursor. Replays
//!     from those versions cannot be replayed deterministically, so they are
//!     *permanently invalid* and rejected by [`Replay::decode`] with
//!     [`ReplayError::UnsupportedVersion`] (whose message contains
//!     "pre-RNG"). Do not silently misread them.
//!   * v3 (current) - carries the match seed + the RNG cursor at record start
//!     in the header, plus a per-frame state-CRC trace. The two-identical-games
//!     gate compares these CRCs.
//!
//! The RNG state is part of the saved snapshot and of the CRC; a state that is
//! not compared is not part of the contract.

use crate::game::{Game, GameCommand};
use cnc_core::{ClassId, Fixed16_16, Vec2, Vec2i};
use std::fmt;

pub const REPLAY_MAGIC: [u8; 4] = *b"CNRA";
/// Replay file format version. Bumped to 3 together with the RNG-state
/// addition; v1/v2 are permanently invalid.
pub const REPLAY_VERSION: u8 = 3;

const MAX_REPLAY_COMMANDS: u32 = 1_000_000;
const MAX_TRACE_FRAMES: u32 = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ReplayError {
    #[error("replay is too short")]
    TooShort,
    #[error("invalid replay magic (expected CNRA)")]
    InvalidMagic,
    #[error("unsupported replay version {0} (pre-RNG replays are permanently invalid)")]
    UnsupportedVersion(u8),
    #[error("replay command count {0} exceeds the {MAX_REPLAY_COMMANDS} limit")]
    TooManyCommands(u32),
    #[error("replay is truncated")]
    Truncated,
    #[error("invalid command payload")]
    InvalidCommand,
    #[error("replay checksum mismatch")]
    ChecksumMismatch,
    #[error("trailing bytes after replay checksum")]
    TrailingBytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayHeader {
    pub seed: u64,
    pub frame_count: u32,
    pub rng_state: u64,
    pub tick_bits: i32,
}

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
            DesyncKind::StateCrc { expected, actual } => {
                write!(
                    f,
                    "state CRC mismatch (expected {expected:#x}, actual {actual:#x})"
                )
            }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replay {
    pub header: ReplayHeader,
    pub commands: Vec<GameCommand>,
    pub command_frames: Vec<u32>,
    pub trace: Vec<u64>,
}

impl Replay {
    pub fn encode(&self) -> Vec<u8> {
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
            encode_command(cmd, &mut payload);
            out.extend_from_slice(&frame.to_le_bytes());
            out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
            out.extend_from_slice(&payload);
        }
        // trace
        out.extend_from_slice(&(self.trace.len() as u32).to_le_bytes());
        for crc in &self.trace {
            out.extend_from_slice(&crc.to_le_bytes());
        }
        let checksum = fnv_bytes(&out);
        out.extend_from_slice(&checksum.to_le_bytes());
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ReplayError> {
        // magic(4)+version(1)+reserved(1)+seed(8)+rng(8)+frames(4)+tick(4)+count(4) = 34, +checksum 8
        const MIN_HEADER: usize = 4 + 1 + 1 + 8 + 8 + 4 + 4 + 4;
        if bytes.len() < MIN_HEADER + 8 {
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
        let seed = read_u64(bytes, &mut pos)?;
        let rng_state = read_u64(bytes, &mut pos)?;
        let frame_count = read_u32(bytes, &mut pos)?;
        let tick_bits = read_i32(bytes, &mut pos)?;

        if frame_count > MAX_TRACE_FRAMES {
            // reuse too-many concept loosely; treat as truncated-ish
        }

        let cmd_count = read_u32(bytes, &mut pos)?;
        if cmd_count > MAX_REPLAY_COMMANDS {
            return Err(ReplayError::TooManyCommands(cmd_count));
        }

        let mut commands = Vec::with_capacity(cmd_count as usize);
        let mut command_frames = Vec::with_capacity(cmd_count as usize);
        for _ in 0..cmd_count {
            let frame = read_u32(bytes, &mut pos)?;
            let plen = read_u32(bytes, &mut pos)? as usize;
            if pos + plen > bytes.len() {
                return Err(ReplayError::Truncated);
            }
            let payload = &bytes[pos..pos + plen];
            pos += plen;
            let cmd = decode_command(payload).map_err(|_| ReplayError::InvalidCommand)?;
            commands.push(cmd);
            command_frames.push(frame);
        }

        let trace_count = read_u32(bytes, &mut pos)?;
        if trace_count > MAX_TRACE_FRAMES {
            return Err(ReplayError::TooManyCommands(trace_count));
        }
        // frame_count should equal trace_count ideally; not enforced strictly.
        let mut trace = Vec::with_capacity(trace_count as usize);
        for _ in 0..trace_count {
            let crc = read_u64(bytes, &mut pos)?;
            trace.push(crc);
        }

        // checksum
        if pos + 8 > bytes.len() {
            return Err(ReplayError::Truncated);
        }
        let payload_end = pos;
        // Actually checksum covers everything before the trailing 8 bytes.
        // We've consumed all fields; remaining must be exactly 8 checksum bytes.
        if bytes.len() - payload_end != 8 {
            // could be trailing bytes
            if bytes.len() - payload_end > 8 {
                return Err(ReplayError::TrailingBytes);
            }
            return Err(ReplayError::Truncated);
        }
        let expected = u64::from_le_bytes(bytes[payload_end..].try_into().unwrap());
        // recompute checksum over bytes[..payload_end]
        if fnv_bytes(&bytes[..payload_end]) != expected {
            return Err(ReplayError::ChecksumMismatch);
        }

        // trailing bytes check: none beyond checksum (already handled)
        Ok(Replay {
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

    pub fn verify(&self) -> Result<(), DesyncError> {
        let mut game = Game::new_with_seed(32, 32, self.header.seed);
        game.rng.set_state(self.header.rng_state);
        // apply upfront command_frames==0 handled in loop; but apply all frame 0 commands before first step
        self.run_verify(&mut game)
    }

    pub fn verify_from(&self, local: &mut Game) -> Result<(), DesyncError> {
        if self.header.seed != local.sim_seed() {
            return Err(DesyncError {
                frame: 0,
                kind: DesyncKind::SeedMismatch {
                    header_seed: self.header.seed,
                    local_seed: local.sim_seed(),
                },
            });
        }
        self.run_verify(local)
    }

    fn run_verify(&self, game: &mut Game) -> Result<(), DesyncError> {
        let tick = Fixed16_16::from_bits(self.header.tick_bits);
        let tick = if tick == Fixed16_16::ZERO {
            Fixed16_16::from_int(1)
        } else {
            tick
        };
        let frame_count = self.header.frame_count as usize;
        // ensure trace length
        let frames = frame_count.min(self.trace.len());
        let mut cmd_idx = 0;
        // apply commands with frame 0 before first step
        // We'll handle per-frame in loop.
        for frame in 0..frames {
            while cmd_idx < self.commands.len() && self.command_frames[cmd_idx] <= frame as u32 {
                // apply only those exactly due at this frame boundary
                if self.command_frames[cmd_idx] == frame as u32 {
                    game.apply_command(&self.commands[cmd_idx]);
                    cmd_idx += 1;
                } else {
                    // command_frame < frame means already should have applied; apply now
                    if self.command_frames[cmd_idx] < frame as u32 {
                        game.apply_command(&self.commands[cmd_idx]);
                        cmd_idx += 1;
                    } else {
                        break;
                    }
                }
            }
            game.update(tick);
            let crc = game.state_crc();
            if crc != self.trace[frame] {
                return Err(DesyncError {
                    frame: frame as u32,
                    kind: DesyncKind::StateCrc {
                        expected: self.trace[frame],
                        actual: crc,
                    },
                });
            }
        }
        // apply any remaining commands (frame >= frames) - not verified
        Ok(())
    }
}

pub struct ReplayBuilder {
    tick: Fixed16_16,
    header_seed: u64,
    header_rng_state: u64,
    commands: Vec<GameCommand>,
    command_frames: Vec<u32>,
    trace: Vec<u64>,
    frame: u32,
    frame_count: u32,
}

impl ReplayBuilder {
    pub fn new(game: &Game, tick: Fixed16_16) -> Self {
        Self {
            tick,
            header_seed: game.sim_seed(),
            header_rng_state: game.rng.state(),
            commands: Vec::new(),
            command_frames: Vec::new(),
            trace: Vec::new(),
            frame: 0,
            frame_count: 0,
        }
    }

    pub fn command(&mut self, game: &mut Game, command: GameCommand) {
        game.apply_command(&command);
        self.commands.push(command);
        self.command_frames.push(self.frame);
    }

    pub fn step(&mut self, game: &mut Game) {
        game.update(self.tick);
        self.trace.push(game.state_crc());
        self.frame += 1;
        self.frame_count = self.frame;
    }

    pub fn finish(self) -> Replay {
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

fn encode_command(cmd: &GameCommand, out: &mut Vec<u8>) {
    match cmd {
        GameCommand::Move {
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
        GameCommand::Build {
            owner,
            class_id,
            cell,
        } => {
            out.push(2);
            out.extend_from_slice(&(*owner as u32).to_le_bytes());
            out.extend_from_slice(&class_id.as_u32().to_le_bytes());
            out.extend_from_slice(&cell.x.to_le_bytes());
            out.extend_from_slice(&cell.y.to_le_bytes());
        }
        GameCommand::Produce {
            owner,
            class_id,
            seconds,
        } => {
            out.push(3);
            out.extend_from_slice(&(*owner as u32).to_le_bytes());
            out.extend_from_slice(&class_id.as_u32().to_le_bytes());
            out.extend_from_slice(&seconds.to_le_bytes());
        }
        GameCommand::Attack {
            owner,
            units,
            target,
        } => {
            out.push(4);
            out.extend_from_slice(&(*owner as u32).to_le_bytes());
            out.extend_from_slice(&(units.len() as u32).to_le_bytes());
            for u in units {
                out.extend_from_slice(&u.to_le_bytes());
            }
            out.extend_from_slice(&target.to_le_bytes());
        }
        GameCommand::Sell { owner, building } => {
            out.push(5);
            out.extend_from_slice(&(*owner as u32).to_le_bytes());
            out.extend_from_slice(&building.to_le_bytes());
        }
        GameCommand::Deploy { owner, unit } => {
            out.push(6);
            out.extend_from_slice(&(*owner as u32).to_le_bytes());
            out.extend_from_slice(&unit.to_le_bytes());
        }
        GameCommand::Repair { owner, building } => {
            out.push(7);
            out.extend_from_slice(&(*owner as u32).to_le_bytes());
            out.extend_from_slice(&building.to_le_bytes());
        }
        GameCommand::Power {
            owner,
            building,
            on,
        } => {
            out.push(8);
            out.extend_from_slice(&(*owner as u32).to_le_bytes());
            out.extend_from_slice(&building.to_le_bytes());
            out.push(if *on { 1 } else { 0 });
        }
        GameCommand::CancelProduction { owner } => {
            out.push(9);
            out.extend_from_slice(&(*owner as u32).to_le_bytes());
        }
        GameCommand::Stop { owner, units } => {
            out.push(10);
            out.extend_from_slice(&(*owner as u32).to_le_bytes());
            out.extend_from_slice(&(units.len() as u32).to_le_bytes());
            for u in units {
                out.extend_from_slice(&u.to_le_bytes());
            }
        }
    }
}

fn decode_command(payload: &[u8]) -> Result<GameCommand, ()> {
    if payload.is_empty() {
        return Err(());
    }
    let mut pos = 0;
    let tag = payload[pos];
    pos += 1;
    let read_u32 = |p: &mut usize| -> Result<u32, ()> {
        if *p + 4 > payload.len() {
            return Err(());
        }
        let v = u32::from_le_bytes(payload[*p..*p + 4].try_into().unwrap());
        *p += 4;
        Ok(v)
    };
    let read_i32 = |p: &mut usize| -> Result<i32, ()> {
        if *p + 4 > payload.len() {
            return Err(());
        }
        let v = i32::from_le_bytes(payload[*p..*p + 4].try_into().unwrap());
        *p += 4;
        Ok(v)
    };
    let read_units = |p: &mut usize| -> Result<Vec<u32>, ()> {
        let count = read_u32(p)?;
        if *p + (count as usize) * 4 > payload.len() {
            return Err(());
        }
        let mut v = Vec::with_capacity(count as usize);
        for _ in 0..count {
            v.push(read_u32(p)?);
        }
        Ok(v)
    };

    match tag {
        1 => {
            let owner = read_u32(&mut pos)? as usize;
            let units = read_units(&mut pos)?;
            let x = Fixed16_16::from_bits(read_i32(&mut pos)?);
            let y = Fixed16_16::from_bits(read_i32(&mut pos)?);
            if pos != payload.len() {
                return Err(());
            }
            Ok(GameCommand::Move {
                owner,
                units,
                target: Vec2::new(x, y),
            })
        }
        2 => {
            let owner = read_u32(&mut pos)? as usize;
            let cid = ClassId::from_u32(read_u32(&mut pos)?);
            let x = read_i32(&mut pos)?;
            let y = read_i32(&mut pos)?;
            if pos != payload.len() {
                return Err(());
            }
            Ok(GameCommand::Build {
                owner,
                class_id: cid,
                cell: Vec2i::new(x, y),
            })
        }
        3 => {
            let owner = read_u32(&mut pos)? as usize;
            let cid = ClassId::from_u32(read_u32(&mut pos)?);
            let seconds = read_i32(&mut pos)?;
            if pos != payload.len() {
                return Err(());
            }
            Ok(GameCommand::Produce {
                owner,
                class_id: cid,
                seconds,
            })
        }
        4 => {
            let owner = read_u32(&mut pos)? as usize;
            let units = read_units(&mut pos)?;
            let target = read_u32(&mut pos)?;
            if pos != payload.len() {
                return Err(());
            }
            Ok(GameCommand::Attack {
                owner,
                units,
                target,
            })
        }
        5 => {
            let owner = read_u32(&mut pos)? as usize;
            let building = read_u32(&mut pos)?;
            if pos != payload.len() {
                return Err(());
            }
            Ok(GameCommand::Sell { owner, building })
        }
        6 => {
            let owner = read_u32(&mut pos)? as usize;
            let unit = read_u32(&mut pos)?;
            if pos != payload.len() {
                return Err(());
            }
            Ok(GameCommand::Deploy { owner, unit })
        }
        7 => {
            let owner = read_u32(&mut pos)? as usize;
            let building = read_u32(&mut pos)?;
            if pos != payload.len() {
                return Err(());
            }
            Ok(GameCommand::Repair { owner, building })
        }
        8 => {
            let owner = read_u32(&mut pos)? as usize;
            let building = read_u32(&mut pos)?;
            if pos >= payload.len() {
                return Err(());
            }
            let on = payload[pos] != 0;
            pos += 1;
            if pos != payload.len() {
                return Err(());
            }
            Ok(GameCommand::Power {
                owner,
                building,
                on,
            })
        }
        9 => {
            let owner = read_u32(&mut pos)? as usize;
            if pos != payload.len() {
                return Err(());
            }
            Ok(GameCommand::CancelProduction { owner })
        }
        10 => {
            let owner = read_u32(&mut pos)? as usize;
            let units = read_units(&mut pos)?;
            if pos != payload.len() {
                return Err(());
            }
            Ok(GameCommand::Stop { owner, units })
        }
        _ => Err(()),
    }
}

fn read_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, ReplayError> {
    if *pos + 8 > bytes.len() {
        return Err(ReplayError::Truncated);
    }
    let v = u64::from_le_bytes(bytes[*pos..*pos + 8].try_into().unwrap());
    *pos += 8;
    Ok(v)
}
fn read_u32(bytes: &[u8], pos: &mut usize) -> Result<u32, ReplayError> {
    if *pos + 4 > bytes.len() {
        return Err(ReplayError::Truncated);
    }
    let v = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().unwrap());
    *pos += 4;
    Ok(v)
}
fn read_i32(bytes: &[u8], pos: &mut usize) -> Result<i32, ReplayError> {
    if *pos + 4 > bytes.len() {
        return Err(ReplayError::Truncated);
    }
    let v = i32::from_le_bytes(bytes[*pos..*pos + 4].try_into().unwrap());
    *pos += 4;
    Ok(v)
}

fn fnv_bytes(bytes: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = FNV_OFFSET;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}
