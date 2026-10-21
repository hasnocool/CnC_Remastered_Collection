//! Network/Command queue for Tiberian Dawn

use crate::game::Game;
use cnc_core::{ClassId, Vec2};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::net::SocketAddr;
use std::path::Path;
use tokio::net::UdpSocket;

/// Command types for lockstep networking
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Move {
        units: Vec<u32>,
        target: Vec2,
    },
    Attack {
        units: Vec<u32>,
        target: u32,
    },
    Stop {
        units: Vec<u32>,
    },
    Guard {
        units: Vec<u32>,
        target: u32,
    },
    Build {
        building: ClassId,
        location: Vec2,
    },
    Sell {
        building: u32,
    },
    Repair {
        unit: u32,
        target: u32,
    },
    Power {
        building: u32,
        on: bool,
    },
    Produce {
        factory: u32,
        unit_type: ClassId,
    },
    Deploy {
        unit: u32,
    },
    Undeploy {
        unit: u32,
    },
    Sabotage {
        unit: u32,
        target: u32,
    },
    /// Repair one owned building to full health (RA structure repair).
    RepairStructure {
        building: u32,
    },
    /// Cancel the front production item, refunding its cost.
    CancelProduction,
    /// Place the owner's front completed structure at a world location (TD).
    PlaceBuilding {
        location: Vec2,
    },
    /// Board a passenger into a transport (APC/Chinook).
    Board {
        transport: u32,
        passenger: u32,
    },
    /// Unload a passenger from a transport at a sub-cell slot.
    Unload {
        transport: u32,
        slot: u8,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetCommand {
    pub frame: u32,
    pub player: u8,
    pub command: Command,
}

impl NetCommand {
    pub fn new(frame: u32, player: u8, command: Command) -> Self {
        Self {
            frame,
            player,
            command,
        }
    }

    /// Stable hash of the whole command record, used for desync detection.
    pub fn hash(&self) -> u64 {
        let mut hash = FNV_OFFSET;
        mix(&mut hash, u64::from(self.frame));
        mix(&mut hash, u64::from(self.player));
        mix(&mut hash, self.command.hash());
        hash
    }
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

pub const PROTOCOL_VERSION: u32 = 1;

fn mix(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(FNV_PRIME);
}

impl Command {
    /// Stable discriminator for checksums and replay diagnostics.
    pub fn tag(&self) -> u8 {
        match self {
            Command::Move { .. } => 1,
            Command::Attack { .. } => 2,
            Command::Stop { .. } => 3,
            Command::Guard { .. } => 4,
            Command::Build { .. } => 5,
            Command::Sell { .. } => 6,
            Command::Repair { .. } => 7,
            Command::Power { .. } => 8,
            Command::Produce { .. } => 9,
            Command::Deploy { .. } => 10,
            Command::Undeploy { .. } => 11,
            Command::Sabotage { .. } => 12,
            Command::RepairStructure { .. } => 13,
            Command::CancelProduction => 14,
            Command::PlaceBuilding { .. } => 15,
            Command::Board { .. } => 16,
            Command::Unload { .. } => 17,
        }
    }

    /// Deterministic FNV-1a hash over the command payload.
    pub fn hash(&self) -> u64 {
        let mut hash = FNV_OFFSET;
        mix(&mut hash, u64::from(self.tag()));
        match self {
            Command::Move { units, target } => {
                mix_ids(&mut hash, units);
                mix_vec2(&mut hash, target);
            }
            Command::Attack { units, target } => {
                mix_ids(&mut hash, units);
                mix(&mut hash, u64::from(*target));
            }
            Command::Stop { units } => mix_ids(&mut hash, units),
            Command::Guard { units, target } => {
                mix_ids(&mut hash, units);
                mix(&mut hash, u64::from(*target));
            }
            Command::Build { building, location } => {
                mix(&mut hash, *building as u64);
                mix_vec2(&mut hash, location);
            }
            Command::Sell { building } => mix(&mut hash, u64::from(*building)),
            Command::Repair { unit, target } => {
                mix(&mut hash, u64::from(*unit));
                mix(&mut hash, u64::from(*target));
            }
            Command::Power { building, on } => {
                mix(&mut hash, u64::from(*building));
                mix(&mut hash, u64::from(*on));
            }
            Command::Produce { factory, unit_type } => {
                mix(&mut hash, u64::from(*factory));
                mix(&mut hash, *unit_type as u64);
            }
            Command::Deploy { unit } => mix(&mut hash, u64::from(*unit)),
            Command::Undeploy { unit } => mix(&mut hash, u64::from(*unit)),
            Command::Sabotage { unit, target } => {
                mix(&mut hash, u64::from(*unit));
                mix(&mut hash, u64::from(*target));
            }
            Command::RepairStructure { building } => mix(&mut hash, u64::from(*building)),
            Command::CancelProduction => {}
            Command::PlaceBuilding { location } => mix_vec2(&mut hash, location),
            Command::Board {
                transport,
                passenger,
            } => {
                mix(&mut hash, u64::from(*transport));
                mix(&mut hash, u64::from(*passenger));
            }
            Command::Unload { transport, slot } => {
                mix(&mut hash, u64::from(*transport));
                mix(&mut hash, u64::from(*slot));
            }
        }
        hash
    }
}

fn mix_ids(hash: &mut u64, units: &[u32]) {
    mix(hash, units.len() as u64);
    for unit in units {
        mix(hash, u64::from(*unit));
    }
}

/// Game-side apply arm for the sabotage command.
///
/// The wire variant, stable tag (12) and the packet codec below already carry
/// [`Command::Sabotage`]; what was missing is the step that turns the decoded
/// command into a simulation order. INFANTRY.CPP:2891-2901
/// (`InfantryClass::What_Action`) is where the original engine rewrites a
/// commando's attack on a building into `ACTION_SABOTAGE`, so the command names
/// a unit index and a building index.
///
/// Only the issuing player's own living unit may be ordered, mirroring how the
/// other order dispatchers validate ownership. The resulting order carries no
/// random or time-derived data — `Game::order_sabotage` reads nothing but those
/// two indices — so two peers applying the same command stream reach identical
/// state, which is what the lockstep frame checksums compare.
pub fn apply_sabotage(game: &mut Game, command: &Command, player: u8) -> bool {
    let Command::Sabotage { unit, target } = command else {
        return false;
    };
    let owned = game
        .units
        .get(*unit as usize)
        .is_some_and(|unit| unit.is_alive() && unit.owner == player);
    if !owned {
        return false;
    }
    game.order_sabotage(*unit as usize, *target as usize)
}

fn mix_vec2(hash: &mut u64, vector: &Vec2) {
    mix(hash, vector.x.0 as u32 as u64);
    mix(hash, vector.y.0 as u32 as u64);
}

/// Error raised when a command cannot enter the lockstep stream.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LockstepError {
    #[error("command frame {frame} is in the past (current frame {current})")]
    StaleFrame { frame: u32, current: u32 },
    #[error("player {player} is not part of a {players}-player session")]
    InvalidPlayer { player: u8, players: u8 },
}

/// Command queue for lockstep networking.
///
/// Commands may arrive out of order (UDP); they are kept sorted by frame,
/// player and command hash so every peer executes the same order. Commands for
/// frames that have already been simulated are rejected as stale.
#[derive(Debug, Clone, Default)]
pub struct CommandQueue {
    pub queue: VecDeque<NetCommand>,
    pub current_frame: u32,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a command, preserving deterministic execution order.
    pub fn push(&mut self, command: NetCommand) {
        if command.frame < self.current_frame {
            return;
        }
        let position = self
            .queue
            .iter()
            .position(|queued| Self::ordering(queued, &command).is_gt());
        match position {
            Some(index) => self.queue.insert(index, command),
            None => self.queue.push_back(command),
        }
    }

    fn ordering(left: &NetCommand, right: &NetCommand) -> std::cmp::Ordering {
        (
            left.frame,
            left.player,
            left.command.hash(),
            left.command.tag(),
        )
            .cmp(&(
                right.frame,
                right.player,
                right.command.hash(),
                right.command.tag(),
            ))
    }

    /// Remove and return every command scheduled at or before the given frame.
    pub fn pop_frame(&mut self, frame: u32) -> Vec<NetCommand> {
        self.current_frame = frame;
        let mut commands = Vec::new();
        while let Some(cmd) = self.queue.front() {
            if cmd.frame <= frame {
                commands.push(self.queue.pop_front().expect("front exists"));
            } else {
                break;
            }
        }
        commands
    }

    pub fn peek(&self) -> Option<&NetCommand> {
        self.queue.front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

/// Deterministic lockstep session: frame gating, replay log and desync checks.
#[derive(Debug, Clone)]
pub struct LockstepSession {
    pub queue: CommandQueue,
    pub players: u8,
    /// Every executed command, in execution order.
    pub replay: Vec<NetCommand>,
    /// Checksum recorded for each executed frame.
    pub checksums: Vec<(u32, u32)>,
    /// World-state CRC recorded for each executed frame (FRAMESYNC-style).
    pub state_crcs: Vec<(u32, u64)>,
}

impl LockstepSession {
    pub fn new(players: u8) -> Self {
        Self {
            queue: CommandQueue::new(),
            players: players.max(1),
            replay: Vec::new(),
            checksums: Vec::new(),
            state_crcs: Vec::new(),
        }
    }

    pub fn current_frame(&self) -> u32 {
        self.queue.current_frame
    }

    /// Validate and schedule a command for a future frame.
    pub fn schedule(&mut self, command: NetCommand) -> Result<(), LockstepError> {
        if command.player >= self.players {
            return Err(LockstepError::InvalidPlayer {
                player: command.player,
                players: self.players,
            });
        }
        if command.frame < self.queue.current_frame {
            return Err(LockstepError::StaleFrame {
                frame: command.frame,
                current: self.queue.current_frame,
            });
        }
        self.queue.push(command);
        Ok(())
    }

    /// Execute all commands scheduled at the current frame, then advance one frame.
    pub fn advance(&mut self) -> Vec<NetCommand> {
        let frame = self.current_frame();
        let commands = self.queue.pop_frame(frame);
        self.record(frame, &commands);
        self.queue.current_frame = frame.saturating_add(1);
        commands
    }

    /// Advance until (but not past) the target frame, returning everything executed.
    pub fn advance_to(&mut self, target: u32) -> Vec<NetCommand> {
        let mut executed = Vec::new();
        while self.current_frame() <= target {
            let commands = self.advance();
            executed.extend(commands);
        }
        executed
    }

    /// Advance until (but not past) the target frame, recording world-state CRCs
    /// from `game` for every frame.
    pub fn advance_to_with_game(
        &mut self,
        target: u32,
        game: &crate::game::Game,
    ) -> Vec<NetCommand> {
        let mut executed = Vec::new();
        while self.current_frame() <= target {
            let commands = self.advance_with_game(game);
            executed.extend(commands);
        }
        executed
    }

    fn record(&mut self, frame: u32, commands: &[NetCommand]) {
        if commands.is_empty() {
            return;
        }
        self.replay.extend_from_slice(commands);
        self.checksums.push((frame, Self::frame_checksum(commands)));
    }

    /// Order-independent checksum for one frame's command set.
    pub fn frame_checksum(commands: &[NetCommand]) -> u32 {
        let mut hashes: Vec<u64> = commands.iter().map(NetCommand::hash).collect();
        hashes.sort_unstable();
        let mut hash = FNV_OFFSET;
        mix(&mut hash, hashes.len() as u64);
        for value in hashes {
            mix(&mut hash, value);
        }
        (hash & 0xFFFF_FFFF) as u32
    }

    /// Compare this session's frame checksum against a remote peer's value.
    pub fn desynced(&self, frame: u32, remote_checksum: u32) -> bool {
        self.checksums
            .iter()
            .find(|(recorded, _)| *recorded == frame)
            .is_some_and(|(_, local)| *local != remote_checksum)
    }

    /// Record a world-state CRC for `frame`, exchanged like FRAMESYNC.
    pub fn record_state_crc(&mut self, frame: u32, crc: u64) {
        if let Some(entry) = self.state_crcs.iter_mut().find(|(f, _)| *f == frame) {
            entry.1 = crc;
        } else {
            self.state_crcs.push((frame, crc));
        }
    }

    /// World-state CRC recorded for `frame`, if any.
    pub fn state_crc(&self, frame: u32) -> Option<u64> {
        self.state_crcs
            .iter()
            .find(|(recorded, _)| *recorded == frame)
            .map(|(_, crc)| *crc)
    }

    /// Compare this session's world-state CRC against a remote peer's value.
    pub fn desynced_state(&self, frame: u32, remote_crc: u64) -> bool {
        self.state_crcs
            .iter()
            .find(|(recorded, _)| *recorded == frame)
            .is_some_and(|(_, local)| *local != remote_crc)
    }

    /// Alias for [`Self::desynced_state`].
    pub fn state_desynced(&self, frame: u32, remote_crc: u64) -> bool {
        self.desynced_state(frame, remote_crc)
    }

    /// Alias for [`Self::desynced_state`].
    pub fn desynced_crc(&self, frame: u32, remote_crc: u64) -> bool {
        self.desynced_state(frame, remote_crc)
    }

    /// Advance using a game to record its world-state CRC for the frame.
    pub fn advance_with_game(&mut self, game: &crate::game::Game) -> Vec<NetCommand> {
        let frame = self.current_frame();
        let commands = self.queue.pop_frame(frame);
        self.record(frame, &commands);
        self.record_state_crc(frame, game.state_crc());
        self.queue.current_frame = frame.saturating_add(1);
        commands
    }

    /// Rebuild a session from a recorded replay (used by save/load and tests).
    pub fn from_replay(players: u8, replay: Vec<NetCommand>) -> Self {
        let mut session = Self::new(players);
        for command in replay {
            session.queue.push(command);
        }
        session
    }
}

/// Network packet for UDP transmission
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetPacket {
    pub sequence: u32,
    pub frame: u32,
    pub commands: Vec<NetCommand>,
    pub checksum: u32,
    /// World-state CRC carried by this frame (FRAMESYNC-style), beyond the
    /// command-stream checksum above. Zero when unset.
    pub state_crc: u64,
}

impl NetPacket {
    pub fn new(sequence: u32, frame: u32, commands: Vec<NetCommand>) -> Self {
        Self::with_state_crc(sequence, frame, commands, 0)
    }

    pub fn with_state_crc(
        sequence: u32,
        frame: u32,
        commands: Vec<NetCommand>,
        state_crc: u64,
    ) -> Self {
        let mut packet = Self {
            sequence,
            frame,
            commands,
            checksum: 0,
            state_crc,
        };
        packet.checksum = packet.calculate_checksum();
        packet
    }

    fn calculate_checksum(&self) -> u32 {
        let mut hash = FNV_OFFSET;
        mix(&mut hash, u64::from(self.sequence));
        mix(&mut hash, u64::from(self.frame));
        mix(&mut hash, self.commands.len() as u64);
        for command in &self.commands {
            mix(&mut hash, command.hash());
        }
        mix(&mut hash, self.state_crc);
        (hash & 0xFFFF_FFFF) as u32
    }

    pub fn verify(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }
}

/// Wire magic for lockstep datagrams ("CNCP").
pub const PACKET_MAGIC: [u8; 4] = *b"CNCP";
/// Wire protocol version.
pub const PACKET_VERSION: u8 = 1;
/// Header: magic(4) + version(1) + reserved(1) + sequence(4) + frame(4) + checksum(4) + count(2) + state_crc(8).
const PACKET_HEADER_LEN: usize = 28;
/// Defensive cap so a malformed datagram cannot force a huge allocation.
const MAX_COMMANDS_PER_PACKET: usize = 4_096;

/// Errors produced while encoding or decoding lockstep datagrams.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PacketError {
    #[error("packet is too short (need at least {PACKET_HEADER_LEN} bytes)")]
    TooShort,
    #[error("invalid packet magic (expected CNCP)")]
    InvalidMagic,
    #[error("unsupported protocol version {0}")]
    UnsupportedVersion(u8),
    #[error("invalid command tag {0}")]
    InvalidTag(u8),
    #[error("command count {0} exceeds the {MAX_COMMANDS_PER_PACKET} command limit")]
    TooManyCommands(usize),
    #[error("truncated command payload")]
    Truncated,
    #[error("trailing bytes after command payload")]
    TrailingBytes,
    #[error("packet checksum mismatch")]
    ChecksumMismatch,
    #[error("invalid command payload: {0}")]
    InvalidPayload(&'static str),
}

struct PayloadReader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> PayloadReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    fn take(&mut self, count: usize) -> Result<&'a [u8], PacketError> {
        let end = self
            .position
            .checked_add(count)
            .ok_or(PacketError::Truncated)?;
        if end > self.bytes.len() {
            return Err(PacketError::Truncated);
        }
        let slice = &self.bytes[self.position..end];
        self.position = end;
        Ok(slice)
    }

    fn u8(&mut self) -> Result<u8, PacketError> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, PacketError> {
        Ok(u16::from_le_bytes(
            self.take(2)?
                .try_into()
                .map_err(|_| PacketError::Truncated)?,
        ))
    }

    fn u32(&mut self) -> Result<u32, PacketError> {
        Ok(u32::from_le_bytes(
            self.take(4)?
                .try_into()
                .map_err(|_| PacketError::Truncated)?,
        ))
    }

    fn i32(&mut self) -> Result<i32, PacketError> {
        Ok(i32::from_le_bytes(
            self.take(4)?
                .try_into()
                .map_err(|_| PacketError::Truncated)?,
        ))
    }

    fn unit_ids(&mut self) -> Result<Vec<u32>, PacketError> {
        let count = usize::from(self.u16()?);
        if count > MAX_COMMANDS_PER_PACKET {
            return Err(PacketError::TooManyCommands(count));
        }
        let mut units = Vec::with_capacity(count);
        for _ in 0..count {
            units.push(self.u32()?);
        }
        Ok(units)
    }

    fn vec2(&mut self) -> Result<Vec2, PacketError> {
        let x = self.i32()?;
        let y = self.i32()?;
        // Wire values are raw 16.16 bits, so rebuild the fixed-point fields directly.
        Ok(Vec2::new(cnc_core::Fixed16_16(x), cnc_core::Fixed16_16(y)))
    }

    fn ensure_consumed(&self) -> Result<(), PacketError> {
        if self.remaining() == 0 {
            Ok(())
        } else {
            Err(PacketError::TrailingBytes)
        }
    }
}

fn push_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_i32(out: &mut Vec<u8>, value: i32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn push_units(out: &mut Vec<u8>, units: &[u32]) -> Result<(), PacketError> {
    let count =
        u16::try_from(units.len()).map_err(|_| PacketError::TooManyCommands(units.len()))?;
    push_u16(out, count);
    for unit in units {
        push_u32(out, *unit);
    }
    Ok(())
}

fn push_vec2(out: &mut Vec<u8>, vector: &Vec2) {
    push_i32(out, vector.x.0);
    push_i32(out, vector.y.0);
}

impl Command {
    /// Encode only the command payload; the tag is written by NetPacket::encode.
    pub fn encode_payload(&self, out: &mut Vec<u8>) -> Result<(), PacketError> {
        match self {
            Command::Move { units, target } => {
                push_units(out, units)?;
                push_vec2(out, target);
            }
            Command::Attack { units, target } => {
                push_units(out, units)?;
                push_u32(out, *target);
            }
            Command::Stop { units } => push_units(out, units)?,
            Command::Guard { units, target } => {
                push_units(out, units)?;
                push_u32(out, *target);
            }
            Command::Build { building, location } => {
                push_u32(out, building.as_u32());
                push_vec2(out, location);
            }
            Command::Sell { building } => push_u32(out, *building),
            Command::Repair { unit, target } => {
                push_u32(out, *unit);
                push_u32(out, *target);
            }
            Command::Power { building, on } => {
                push_u32(out, *building);
                out.push(u8::from(*on));
            }
            Command::Produce { factory, unit_type } => {
                push_u32(out, *factory);
                push_u32(out, unit_type.as_u32());
            }
            Command::Deploy { unit } => push_u32(out, *unit),
            Command::Undeploy { unit } => push_u32(out, *unit),
            Command::Sabotage { unit, target } => {
                push_u32(out, *unit);
                push_u32(out, *target);
            }
            Command::RepairStructure { building } => push_u32(out, *building),
            Command::CancelProduction => {}
            Command::PlaceBuilding { location } => push_vec2(out, location),
            Command::Board {
                transport,
                passenger,
            } => {
                push_u32(out, *transport);
                push_u32(out, *passenger);
            }
            Command::Unload { transport, slot } => {
                push_u32(out, *transport);
                out.push(*slot);
            }
        }
        Ok(())
    }

    /// Decode a payload previously written by encode_payload.
    pub fn decode_payload(tag: u8, bytes: &[u8]) -> Result<Self, PacketError> {
        let mut reader = PayloadReader::new(bytes);
        let command = match tag {
            1 => Command::Move {
                units: reader.unit_ids()?,
                target: reader.vec2()?,
            },
            2 => Command::Attack {
                units: reader.unit_ids()?,
                target: reader.u32()?,
            },
            3 => Command::Stop {
                units: reader.unit_ids()?,
            },
            4 => Command::Guard {
                units: reader.unit_ids()?,
                target: reader.u32()?,
            },
            5 => Command::Build {
                building: ClassId::from_u32(reader.u32()?),
                location: reader.vec2()?,
            },
            6 => Command::Sell {
                building: reader.u32()?,
            },
            7 => Command::Repair {
                unit: reader.u32()?,
                target: reader.u32()?,
            },
            8 => Command::Power {
                building: reader.u32()?,
                on: match reader.u8()? {
                    0 => false,
                    1 => true,
                    _ => return Err(PacketError::InvalidPayload("Power.on must be 0 or 1")),
                },
            },
            9 => Command::Produce {
                factory: reader.u32()?,
                unit_type: ClassId::from_u32(reader.u32()?),
            },
            10 => Command::Deploy {
                unit: reader.u32()?,
            },
            11 => Command::Undeploy {
                unit: reader.u32()?,
            },
            12 => Command::Sabotage {
                unit: reader.u32()?,
                target: reader.u32()?,
            },
            13 => Command::RepairStructure {
                building: reader.u32()?,
            },
            14 => Command::CancelProduction,
            15 => Command::PlaceBuilding {
                location: reader.vec2()?,
            },
            16 => Command::Board {
                transport: reader.u32()?,
                passenger: reader.u32()?,
            },
            17 => Command::Unload {
                transport: reader.u32()?,
                slot: reader.u8()?,
            },
            other => return Err(PacketError::InvalidTag(other)),
        };
        reader.ensure_consumed()?;
        Ok(command)
    }
}

impl NetPacket {
    /// Encode this packet into a self-contained UDP datagram.
    pub fn encode(&self) -> Result<Vec<u8>, PacketError> {
        if self.commands.len() > MAX_COMMANDS_PER_PACKET {
            return Err(PacketError::TooManyCommands(self.commands.len()));
        }
        let mut out = Vec::with_capacity(PACKET_HEADER_LEN + self.commands.len() * 16);
        out.extend_from_slice(&PACKET_MAGIC);
        out.push(PACKET_VERSION);
        out.push(0); // reserved
        push_u32(&mut out, self.sequence);
        push_u32(&mut out, self.frame);
        push_u32(&mut out, self.calculate_checksum());
        push_u16(&mut out, self.commands.len() as u16);
        out.extend_from_slice(&self.state_crc.to_le_bytes());
        for command in &self.commands {
            let mut payload = Vec::new();
            command.command.encode_payload(&mut payload)?;
            push_u32(&mut out, command.frame);
            out.push(command.player);
            out.push(command.command.tag());
            push_u16(&mut out, payload.len() as u16);
            out.extend_from_slice(&payload);
        }
        Ok(out)
    }

    /// Decode and validate a datagram produced by NetPacket::encode.
    pub fn decode(bytes: &[u8]) -> Result<Self, PacketError> {
        if bytes.len() < PACKET_HEADER_LEN {
            return Err(PacketError::TooShort);
        }
        if bytes[0..4] != PACKET_MAGIC {
            return Err(PacketError::InvalidMagic);
        }
        if bytes[4] != PACKET_VERSION {
            return Err(PacketError::UnsupportedVersion(bytes[4]));
        }
        let mut reader = PayloadReader::new(&bytes[5..]);
        let _reserved = reader.u8()?;
        let sequence = reader.u32()?;
        let frame = reader.u32()?;
        let checksum = reader.u32()?;
        let count = usize::from(reader.u16()?);
        if count > MAX_COMMANDS_PER_PACKET {
            return Err(PacketError::TooManyCommands(count));
        }
        let state_crc_bytes = reader.take(8)?;
        let state_crc = u64::from_le_bytes(
            state_crc_bytes
                .try_into()
                .map_err(|_| PacketError::Truncated)?,
        );

        let mut commands = Vec::with_capacity(count);
        for _ in 0..count {
            let command_frame = reader.u32()?;
            let player = reader.u8()?;
            let tag = reader.u8()?;
            let payload_len = usize::from(reader.u16()?);
            let payload = reader.take(payload_len)?;
            let command = Command::decode_payload(tag, payload)?;
            commands.push(NetCommand::new(command_frame, player, command));
        }
        if reader.remaining() != 0 {
            return Err(PacketError::TrailingBytes);
        }

        let packet = Self {
            sequence,
            frame,
            commands,
            checksum,
            state_crc,
        };
        if !packet.verify() {
            return Err(PacketError::ChecksumMismatch);
        }
        Ok(packet)
    }
}

/// UDP transport with an explicit peer list and encoded broadcast.
#[derive(Debug)]
pub struct UdpTransport {
    socket: UdpSocket,
    peers: Vec<SocketAddr>,
}

impl UdpTransport {
    /// Bind a UDP socket (use port 0 for an ephemeral test port).
    pub async fn bind(address: &str) -> std::io::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(address).await?,
            peers: Vec::new(),
        })
    }

    /// Add one peer; returns false when it was already registered.
    pub fn add_peer(&mut self, peer: SocketAddr) -> bool {
        if self.peers.contains(&peer) {
            false
        } else {
            self.peers.push(peer);
            true
        }
    }

    /// Remove a peer; returns whether it was registered.
    pub fn remove_peer(&mut self, peer: &SocketAddr) -> bool {
        match self.peers.iter().position(|candidate| candidate == peer) {
            Some(index) => {
                self.peers.remove(index);
                true
            }
            None => false,
        }
    }

    pub fn peers(&self) -> &[SocketAddr] {
        &self.peers
    }

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    /// Encode once and send to every registered peer. Returns the delivery count.
    pub async fn broadcast(&self, packet: &NetPacket) -> std::io::Result<usize> {
        let bytes = packet
            .encode()
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error))?;
        let mut delivered = 0;
        let mut first_error = None;
        for peer in &self.peers {
            match self.socket.send_to(&bytes, peer).await {
                Ok(_) => delivered += 1,
                Err(error) => {
                    if first_error.is_none() {
                        first_error = Some(error);
                    }
                }
            }
        }
        if delivered == 0 {
            if let Some(error) = first_error {
                return Err(error);
            }
        }
        Ok(delivered)
    }

    /// Send one raw datagram to a specific peer.
    pub async fn send_to(&self, peer: SocketAddr, bytes: &[u8]) -> std::io::Result<()> {
        self.socket.send_to(bytes, peer).await.map(|_| ())
    }

    /// Send a handshake message to a specific peer.
    pub async fn send_handshake(
        &self,
        peer: SocketAddr,
        message: &HandshakeMessage,
    ) -> std::io::Result<()> {
        self.send_to(peer, &message.encode()).await
    }

    /// Receive one command packet, skipping handshake traffic on the same socket.
    pub async fn recv(&self) -> std::io::Result<(NetPacket, SocketAddr)> {
        loop {
            match self.recv_datagram().await? {
                (Datagram::Command(packet), from) => return Ok((packet, from)),
                (Datagram::Handshake(_), _) => continue,
            }
        }
    }

    /// Receive either a lockstep command packet or a handshake message.
    pub async fn recv_datagram(&self) -> std::io::Result<(Datagram, SocketAddr)> {
        let mut buffer = [0u8; 65_507];
        let (length, from) = self.socket.recv_from(&mut buffer).await?;
        let datagram = Datagram::decode(&buffer[..length])
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
        Ok((datagram, from))
    }
}

/// A decoded UDP payload: either lockstep traffic or a join/leave control message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Datagram {
    Command(NetPacket),
    Handshake(HandshakeMessage),
}

impl Datagram {
    /// Dispatch on the packet magic used by command and handshake frames.
    pub fn decode(bytes: &[u8]) -> std::io::Result<Self> {
        if bytes.starts_with(&PACKET_MAGIC) {
            NetPacket::decode(bytes)
                .map(Datagram::Command)
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
        } else if bytes.starts_with(&HANDSHAKE_MAGIC) {
            HandshakeMessage::decode(bytes)
                .map(Datagram::Handshake)
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "datagram is neither CNCP nor CNCH",
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// Replay persistence
// ---------------------------------------------------------------------------

/// Magic for an on-disk replay ("CNCR").
pub const REPLAY_MAGIC: [u8; 4] = *b"CNCR";
/// Replay file format version.
pub const REPLAY_VERSION: u8 = 1;
/// Defensive cap on commands loaded from a replay file.
const MAX_REPLAY_COMMANDS: u32 = 1_000_000;

/// Errors produced while saving or loading replay files.
#[derive(Debug, thiserror::Error)]
pub enum ReplayError {
    #[error("replay I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("replay file is too short")]
    TooShort,
    #[error("invalid replay magic (expected CNCR)")]
    InvalidMagic,
    #[error("unsupported replay version {0}")]
    UnsupportedVersion(u8),
    #[error("replay command count {0} exceeds the {MAX_REPLAY_COMMANDS} limit")]
    TooManyCommands(u32),
    #[error("replay is truncated")]
    Truncated,
    #[error("replay command {index} is invalid: {source}")]
    InvalidCommand { index: usize, source: PacketError },
    #[error("replay checksum mismatch")]
    ChecksumMismatch,
    #[error("trailing bytes after replay checksum")]
    TrailingBytes,
}

/// A recorded lockstep match: the player count and every executed command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replay {
    pub players: u8,
    pub commands: Vec<NetCommand>,
}

impl Replay {
    pub fn new(players: u8, commands: Vec<NetCommand>) -> Self {
        Self {
            players: players.max(1),
            commands,
        }
    }

    /// Snapshot the ordered replay log of a live session.
    pub fn from_session(session: &LockstepSession) -> Self {
        Self::new(session.players, session.replay.clone())
    }

    /// Serialize to the versioned CNCR binary format.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(10 + self.commands.len() * 10 + 8);
        out.extend_from_slice(&REPLAY_MAGIC);
        out.push(REPLAY_VERSION);
        out.push(self.players.max(1));
        out.extend_from_slice(&(self.commands.len() as u32).to_le_bytes());
        for command in &self.commands {
            let mut payload = Vec::new();
            command
                .command
                .encode_payload(&mut payload)
                .expect("executed commands always encode");
            out.extend_from_slice(&command.frame.to_le_bytes());
            out.push(command.player);
            out.push(command.command.tag());
            out.extend_from_slice(&(payload.len() as u16).to_le_bytes());
            out.extend_from_slice(&payload);
        }
        let checksum = fnv_bytes(&out);
        out.extend_from_slice(&checksum.to_le_bytes());
        out
    }

    /// Parse and verify a CNCR replay.
    ///
    /// The trailing FNV-1a checksum covers the header and every command record,
    /// so a truncated or bit-flipped replay is rejected rather than replayed.
    pub fn decode(bytes: &[u8]) -> Result<Self, ReplayError> {
        if bytes.len() < 10 + 8 {
            return Err(ReplayError::TooShort);
        }
        if bytes[0..4] != REPLAY_MAGIC {
            return Err(ReplayError::InvalidMagic);
        }
        if bytes[4] != REPLAY_VERSION {
            return Err(ReplayError::UnsupportedVersion(bytes[4]));
        }
        let players = bytes[5].max(1);
        let count = u32::from_le_bytes(bytes[6..10].try_into().expect("four header bytes"));
        if count > MAX_REPLAY_COMMANDS {
            return Err(ReplayError::TooManyCommands(count));
        }

        let payload_end = bytes.len() - 8;
        let expected =
            u64::from_le_bytes(bytes[payload_end..].try_into().expect("checksum is u64"));
        if fnv_bytes(&bytes[..payload_end]) != expected {
            return Err(ReplayError::ChecksumMismatch);
        }

        let mut reader = PayloadReader::new(&bytes[10..payload_end]);
        let mut commands = Vec::with_capacity(count as usize);
        for index in 0..count as usize {
            let frame = reader.u32().map_err(|_| ReplayError::Truncated)?;
            let player = reader.u8().map_err(|_| ReplayError::Truncated)?;
            let tag = reader.u8().map_err(|_| ReplayError::Truncated)?;
            let length = usize::from(reader.u16().map_err(|_| ReplayError::Truncated)?);
            let payload = reader.take(length).map_err(|_| ReplayError::Truncated)?;
            let command = Command::decode_payload(tag, payload)
                .map_err(|source| ReplayError::InvalidCommand { index, source })?;
            commands.push(NetCommand::new(frame, player, command));
        }
        if reader.remaining() != 0 {
            return Err(ReplayError::TrailingBytes);
        }
        Ok(Self { players, commands })
    }

    /// Write the replay to disk.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ReplayError> {
        std::fs::write(path, self.encode())?;
        Ok(())
    }

    /// Read and verify a replay from disk.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ReplayError> {
        let bytes = std::fs::read(path)?;
        Self::decode(&bytes)
    }
}

fn fnv_bytes(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET;
    for byte in bytes {
        mix(&mut hash, u64::from(*byte));
    }
    hash
}

impl LockstepSession {
    /// Capture the current replay log for persistence or sharing.
    pub fn to_replay(&self) -> Replay {
        Replay::from_session(self)
    }

    /// Persist the replay log to a CNCR file.
    pub fn save_replay(&self, path: impl AsRef<Path>) -> Result<(), ReplayError> {
        self.to_replay().save(path)
    }

    /// Rebuild a session whose command queue holds every recorded command.
    pub fn load_replay(path: impl AsRef<Path>) -> Result<Self, ReplayError> {
        let replay = Replay::load(path)?;
        Ok(Self::from_replay(replay.players, replay.commands))
    }
}

// ---------------------------------------------------------------------------
// Peer join/leave handshake
// ---------------------------------------------------------------------------

/// Magic for handshake datagrams ("CNCH").
pub const HANDSHAKE_MAGIC: [u8; 4] = *b"CNCH";
/// Handshake protocol version.
pub const HANDSHAKE_VERSION: u8 = 1;
/// Encoded handshake length: magic(4) + version(1) + kind(1) + player(1) + value(1).
const HANDSHAKE_LEN: usize = 8;

/// Why a host refused a join request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinReject {
    RoomFull,
    InvalidPlayer,
    AlreadyJoined,
    RoomClosed,
    VersionMismatch,
}

impl JoinReject {
    const fn code(self) -> u8 {
        match self {
            Self::RoomFull => 1,
            Self::InvalidPlayer => 2,
            Self::AlreadyJoined => 3,
            Self::RoomClosed => 4,
            Self::VersionMismatch => 5,
        }
    }

    fn from_code(code: u8) -> Result<Self, HandshakeError> {
        match code {
            1 => Ok(Self::RoomFull),
            2 => Ok(Self::InvalidPlayer),
            3 => Ok(Self::AlreadyJoined),
            4 => Ok(Self::RoomClosed),
            5 => Ok(Self::VersionMismatch),
            other => Err(HandshakeError::UnknownReason(other)),
        }
    }
}

/// Join/leave control frames exchanged before lockstep traffic starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeMessage {
    /// A client asks to occupy a player slot in a fixed-size room.
    JoinRequest { player: u8, players: u8 },
    /// The host admits a player; host is the host's own slot index.
    JoinAccepted { player: u8, host: u8 },
    /// The host refused a player.
    JoinRejected { player: u8, reason: JoinReject },
    /// A peer leaves the room.
    Leave { player: u8 },
}

impl HandshakeMessage {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HANDSHAKE_LEN);
        out.extend_from_slice(&HANDSHAKE_MAGIC);
        out.push(HANDSHAKE_VERSION);
        match *self {
            Self::JoinRequest { player, players } => {
                out.push(1);
                out.push(player);
                out.push(players);
            }
            Self::JoinAccepted { player, host } => {
                out.push(2);
                out.push(player);
                out.push(host);
            }
            Self::JoinRejected { player, reason } => {
                out.push(3);
                out.push(player);
                out.push(reason.code());
            }
            Self::Leave { player } => {
                out.push(4);
                out.push(player);
                out.push(0);
            }
        }
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, HandshakeError> {
        if bytes.len() < HANDSHAKE_LEN {
            return Err(HandshakeError::TooShort);
        }
        if bytes[0..4] != HANDSHAKE_MAGIC {
            return Err(HandshakeError::InvalidMagic);
        }
        if bytes[4] != HANDSHAKE_VERSION {
            return Err(HandshakeError::UnsupportedVersion(bytes[4]));
        }
        if bytes.len() != HANDSHAKE_LEN {
            return Err(HandshakeError::TrailingBytes);
        }
        let kind = bytes[5];
        let player = bytes[6];
        let value = bytes[7];
        match kind {
            1 => Ok(Self::JoinRequest {
                player,
                players: value.max(1),
            }),
            2 => Ok(Self::JoinAccepted {
                player,
                host: value,
            }),
            3 => Ok(Self::JoinRejected {
                player,
                reason: JoinReject::from_code(value)?,
            }),
            4 => Ok(Self::Leave { player }),
            other => Err(HandshakeError::UnknownKind(other)),
        }
    }
}

/// Errors from decoding or applying handshake messages.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HandshakeError {
    #[error("handshake packet is too short")]
    TooShort,
    #[error("invalid handshake magic (expected CNCH)")]
    InvalidMagic,
    #[error("unsupported handshake version {0}")]
    UnsupportedVersion(u8),
    #[error("unknown handshake message kind {0}")]
    UnknownKind(u8),
    #[error("unknown join rejection reason {0}")]
    UnknownReason(u8),
    #[error("trailing bytes after handshake message")]
    TrailingBytes,
    #[error("invalid room configuration: player {player} in a {players}-slot room")]
    InvalidConfiguration { player: u8, players: u8 },
    #[error("player {player} is not valid for a {players}-slot room")]
    InvalidPlayer { player: u8, players: u8 },
    #[error("player {player} already joined from another address")]
    AlreadyJoined { player: u8 },
    #[error("join acceptance came from an unexpected host")]
    UnexpectedHost,
    #[error("message is not valid in the current handshake state")]
    InvalidState,
}

/// Local side of a matchmaking handshake.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerState {
    Idle,
    Joining,
    Joined,
    Closed,
}

/// Observable facts emitted by the handshake state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerEvent {
    JoinAccepted { host_player: u8, host: SocketAddr },
    JoinRejected { reason: JoinReject },
    PeerJoined { player: u8, address: SocketAddr },
    PeerLeft { player: u8, address: SocketAddr },
}

/// Result of feeding one message into the handshake state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandshakeStep {
    /// Message the caller should send back to the sender, if any.
    pub reply: Option<HandshakeMessage>,
    /// Local state change or peer event, if any.
    pub event: Option<PeerEvent>,
}

impl HandshakeStep {
    fn empty() -> Self {
        Self {
            reply: None,
            event: None,
        }
    }

    fn reply(message: HandshakeMessage) -> Self {
        Self {
            reply: Some(message),
            event: None,
        }
    }

    fn event(event: PeerEvent) -> Self {
        Self {
            reply: None,
            event: Some(event),
        }
    }
}

/// Per-peer join/leave state shared by host and client.
///
/// The host opens the room with PeerHandshake::host_room; a client calls
/// PeerHandshake::begin_join and sends the returned request. The host
/// validates slot numbers and duplicate addresses, then answers with
/// JoinAccepted or JoinRejected. Both sides track the same player/address
/// map so PeerEvent::PeerJoined and PeerEvent::PeerLeft drive the lockstep
/// player set.
#[derive(Debug, Clone)]
pub struct PeerHandshake {
    pub local_player: u8,
    pub players: u8,
    pub state: PeerState,
    host: Option<SocketAddr>,
    host_player: Option<u8>,
    peers: BTreeMap<u8, SocketAddr>,
}

impl PeerHandshake {
    /// Create an idle peer (client before joining, or host before opening).
    pub fn new(local_player: u8, players: u8) -> Result<Self, HandshakeError> {
        if players == 0 || local_player >= players {
            return Err(HandshakeError::InvalidConfiguration {
                player: local_player,
                players,
            });
        }
        Ok(Self {
            local_player,
            players,
            state: PeerState::Idle,
            host: None,
            host_player: None,
            peers: BTreeMap::new(),
        })
    }

    /// Create a host already accepting join requests.
    pub fn host_room(local_player: u8, players: u8) -> Result<Self, HandshakeError> {
        let mut handshake = Self::new(local_player, players)?;
        handshake.state = PeerState::Joined;
        Ok(handshake)
    }

    /// Open a previously idle/closed room for joins.
    pub fn open_room(&mut self) -> Result<(), HandshakeError> {
        if self.state == PeerState::Joining {
            return Err(HandshakeError::InvalidState);
        }
        self.state = PeerState::Joined;
        Ok(())
    }

    /// Start a client join and produce the request to send to the host.
    pub fn begin_join(&mut self, host: SocketAddr) -> Result<HandshakeMessage, HandshakeError> {
        if self.state == PeerState::Joined {
            return Err(HandshakeError::InvalidState);
        }
        self.state = PeerState::Joining;
        self.host = Some(host);
        Ok(HandshakeMessage::JoinRequest {
            player: self.local_player,
            players: self.players,
        })
    }

    /// Announce that this peer is leaving the room.
    pub fn leave(&mut self) -> HandshakeMessage {
        self.state = PeerState::Closed;
        HandshakeMessage::Leave {
            player: self.local_player,
        }
    }

    /// Apply one received message. Returns an optional reply plus any event.
    pub fn handle(
        &mut self,
        from: SocketAddr,
        message: HandshakeMessage,
    ) -> Result<HandshakeStep, HandshakeError> {
        match message {
            HandshakeMessage::JoinRequest {
                player,
                players: requested_players,
            } => {
                if self.state != PeerState::Joined {
                    return Ok(HandshakeStep::reply(HandshakeMessage::JoinRejected {
                        player,
                        reason: JoinReject::RoomClosed,
                    }));
                }
                if requested_players != self.players {
                    return Ok(HandshakeStep::reply(HandshakeMessage::JoinRejected {
                        player,
                        reason: JoinReject::VersionMismatch,
                    }));
                }
                if player >= self.players {
                    return Ok(HandshakeStep::reply(HandshakeMessage::JoinRejected {
                        player,
                        reason: JoinReject::InvalidPlayer,
                    }));
                }
                if let Some(existing) = self.peers.get(&player) {
                    if existing == &from {
                        // Retransmitted request: acknowledge idempotently.
                        return Ok(HandshakeStep::reply(HandshakeMessage::JoinAccepted {
                            player,
                            host: self.local_player,
                        }));
                    }
                    return Ok(HandshakeStep::reply(HandshakeMessage::JoinRejected {
                        player,
                        reason: JoinReject::AlreadyJoined,
                    }));
                }
                self.peers.insert(player, from);
                Ok(HandshakeStep {
                    reply: Some(HandshakeMessage::JoinAccepted {
                        player,
                        host: self.local_player,
                    }),
                    event: Some(PeerEvent::PeerJoined {
                        player,
                        address: from,
                    }),
                })
            }
            HandshakeMessage::JoinAccepted { player, host } => {
                if self.state != PeerState::Joining
                    || player != self.local_player
                    || self.host != Some(from)
                    || host == player
                    || host >= self.players
                {
                    return Err(HandshakeError::InvalidState);
                }
                self.state = PeerState::Joined;
                self.host_player = Some(host);
                self.peers.insert(host, from);
                Ok(HandshakeStep::event(PeerEvent::JoinAccepted {
                    host_player: host,
                    host: from,
                }))
            }
            HandshakeMessage::JoinRejected { player, reason } => {
                if player != self.local_player || self.state != PeerState::Joining {
                    return Err(HandshakeError::InvalidState);
                }
                self.state = PeerState::Closed;
                self.host_player = None;
                Ok(HandshakeStep::event(PeerEvent::JoinRejected { reason }))
            }
            HandshakeMessage::Leave { player } => {
                if let Some(address) = self.peers.remove(&player) {
                    if self.host_player == Some(player) {
                        self.state = PeerState::Closed;
                    }
                    Ok(HandshakeStep::event(PeerEvent::PeerLeft {
                        player,
                        address,
                    }))
                } else {
                    Ok(HandshakeStep::empty())
                }
            }
        }
    }

    pub fn peers(&self) -> &BTreeMap<u8, SocketAddr> {
        &self.peers
    }

    pub fn peer(&self, player: u8) -> Option<SocketAddr> {
        self.peers.get(&player).copied()
    }

    /// Address of the room host while this peer is a client.
    pub fn host_address(&self) -> Option<SocketAddr> {
        self.host
    }

    pub fn host_player(&self) -> Option<u8> {
        self.host_player
    }

    pub fn is_joined(&self) -> bool {
        self.state == PeerState::Joined
    }
}

// ---------------------------------------------------------------------------
// Host migration and mesh relay
// ---------------------------------------------------------------------------

/// Magic for host-migration control datagrams ("CNCX").
pub const MIGRATION_MAGIC: [u8; 4] = *b"CNCX";
/// Host-migration protocol version.
pub const MIGRATION_VERSION: u8 = 1;
/// magic(4) + version(1) + kind(1) + generation(4) + player(1).
const MIGRATION_LEN: usize = 11;

/// Errors produced while encoding or decoding migration datagrams.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MigrationError {
    #[error("migration datagram is too short (need {MIGRATION_LEN} bytes)")]
    TooShort,
    #[error("invalid migration magic (expected CNCX)")]
    InvalidMagic,
    #[error("unsupported migration version {0}")]
    UnsupportedVersion(u8),
    #[error("invalid migration kind {0}")]
    InvalidKind(u8),
}

/// Control frames used to move the room host when the old host drops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrateMessage {
    /// A peer reports that the host stopped responding. `generation` is the
    /// election epoch and `reporter` proves the sender is still alive.
    HostLost { generation: u32, reporter: u8 },
    /// The deterministically elected peer announces itself as the new host.
    HostClaim { generation: u32, host: u8 },
}

impl MigrateMessage {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(MIGRATION_LEN);
        out.extend_from_slice(&MIGRATION_MAGIC);
        out.push(MIGRATION_VERSION);
        match *self {
            Self::HostLost {
                generation,
                reporter,
            } => {
                out.push(1);
                out.extend_from_slice(&generation.to_le_bytes());
                out.push(reporter);
            }
            Self::HostClaim { generation, host } => {
                out.push(2);
                out.extend_from_slice(&generation.to_le_bytes());
                out.push(host);
            }
        }
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, MigrationError> {
        if bytes.len() < MIGRATION_LEN {
            return Err(MigrationError::TooShort);
        }
        if bytes[..4] != MIGRATION_MAGIC {
            return Err(MigrationError::InvalidMagic);
        }
        if bytes[4] != MIGRATION_VERSION {
            return Err(MigrationError::UnsupportedVersion(bytes[4]));
        }
        let generation = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
        let player = bytes[10];
        match bytes[5] {
            1 => Ok(Self::HostLost {
                generation,
                reporter: player,
            }),
            2 => Ok(Self::HostClaim {
                generation,
                host: player,
            }),
            other => Err(MigrationError::InvalidKind(other)),
        }
    }
}

/// Where a peer stands during a host migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationState {
    /// The original host is alive.
    Stable,
    /// The host dropped; peers are agreeing on a successor.
    AwaitingClaim,
    /// A new host has been accepted (or this peer is the new host).
    Migrated,
}

/// Deterministic host-election state machine.
///
/// Every peer keeps the same alive set and elects the **lowest player index**
/// still alive, so no tie-break traffic is needed. A host loss bumps the
/// generation exactly once per peer; claims from a later generation are
/// rejected until the matching `HostLost` is observed. That prevents a
/// split-brain where a peer that missed the loss message would crown itself.
#[derive(Debug, Clone)]
pub struct HostMigration {
    pub local_player: u8,
    pub players: u8,
    pub generation: u32,
    pub host: u8,
    pub alive: BTreeSet<u8>,
    pub state: MigrationState,
}

impl HostMigration {
    pub fn new(local_player: u8, players: u8, host: u8) -> Result<Self, MigrationError> {
        if players == 0 || local_player >= players || host >= players {
            return Err(MigrationError::InvalidKind(0));
        }
        let alive = (0..players).collect();
        Ok(Self {
            local_player,
            players,
            generation: 0,
            host,
            alive,
            state: MigrationState::Stable,
        })
    }

    /// Lowest living player index; deterministic on every peer.
    pub fn elected(&self) -> Option<u8> {
        self.alive.iter().next().copied()
    }

    pub fn is_host(&self) -> bool {
        self.local_player == self.host
    }

    /// Report that `player` stopped responding.
    ///
    /// Returns a `HostLost` frame only when the lost player was the host and
    /// the loss opens a new election generation.
    pub fn observe_loss(&mut self, player: u8) -> Option<MigrateMessage> {
        if player >= self.players || !self.alive.remove(&player) {
            return None;
        }
        if player != self.host {
            // A non-host peer leaving does not affect the host.
            return None;
        }
        self.generation = self.generation.wrapping_add(1);
        self.state = MigrationState::AwaitingClaim;
        Some(MigrateMessage::HostLost {
            generation: self.generation,
            reporter: self.local_player,
        })
    }

    /// If this peer won the election, announce a claim and become the host.
    pub fn claim(&mut self) -> Option<MigrateMessage> {
        if self.state != MigrationState::AwaitingClaim {
            return None;
        }
        let elected = self.elected()?;
        if elected != self.local_player {
            return None;
        }
        self.host = elected;
        self.state = MigrationState::Migrated;
        Some(MigrateMessage::HostClaim {
            generation: self.generation,
            host: elected,
        })
    }

    /// Apply a control frame from another peer; returns whether it was accepted.
    pub fn handle(&mut self, message: MigrateMessage) -> bool {
        match message {
            MigrateMessage::HostLost {
                generation,
                reporter,
            } => {
                if reporter >= self.players || reporter == self.local_player {
                    return false;
                }
                if !self.alive.contains(&reporter) {
                    return false;
                }
                if generation == self.generation + 1 {
                    // First report we see for this loss: retire the old host.
                    if self.state == MigrationState::Stable {
                        self.alive.remove(&self.host);
                        self.generation = generation;
                        self.state = MigrationState::AwaitingClaim;
                    }
                    true
                } else {
                    // Duplicate reports are harmless; future generations mean
                    // this peer missed a loss and must resynchronise first.
                    generation <= self.generation
                }
            }
            MigrateMessage::HostClaim { generation, host } => {
                if host >= self.players || generation != self.generation {
                    return false;
                }
                if !self.alive.contains(&host) || self.elected() != Some(host) {
                    return false;
                }
                self.host = host;
                self.state = MigrationState::Migrated;
                true
            }
        }
    }
}

/// Bounded duplicate cache used when relaying mesh traffic.
///
/// Peers forward lockstep packets they receive in a full mesh, so the same
/// datagram can arrive from several neighbours. The relay forwards the first
/// copy and drops repeats, keeping the forwarding loop bounded.
#[derive(Debug)]
pub struct MeshRelay {
    seen: BTreeSet<u64>,
    order: VecDeque<u64>,
    capacity: usize,
}

impl MeshRelay {
    pub fn new(capacity: usize) -> Self {
        Self {
            seen: BTreeSet::new(),
            order: VecDeque::new(),
            capacity: capacity.max(1),
        }
    }

    /// Record a datagram; true means "first copy, forward it".
    pub fn accept(&mut self, bytes: &[u8]) -> bool {
        let hash = fnv_bytes(bytes);
        if !self.seen.insert(hash) {
            return false;
        }
        self.order.push_back(hash);
        while self.order.len() > self.capacity {
            if let Some(expired) = self.order.pop_front() {
                self.seen.remove(&expired);
            }
        }
        true
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }

    /// Forget every remembered datagram (a reconnecting peer restarts its
    /// sequence numbers, so old entries would suppress its first packets).
    pub fn clear(&mut self) {
        self.seen.clear();
        self.order.clear();
    }
}

impl UdpTransport {
    /// Forward a datagram to every peer except `source`.
    ///
    /// Returns the number of copies sent; repeated datagrams are dropped by the
    /// shared [`MeshRelay`] cache and report 0.
    pub async fn relay_from(
        &self,
        source: SocketAddr,
        bytes: &[u8],
        relay: &mut MeshRelay,
    ) -> std::io::Result<usize> {
        if !relay.accept(bytes) {
            return Ok(0);
        }
        let mut sent = 0;
        for peer in &self.peers {
            if *peer != source && self.socket.send_to(bytes, peer).await.is_ok() {
                sent += 1;
            }
        }
        Ok(sent)
    }
}

/// Outgoing batch produced by `LiveSession::advance`: the drained commands,
/// the encoded packet bytes, and the peer targets to send them to.
pub type AdvanceBatch = (Vec<NetCommand>, Vec<u8>, Vec<SocketAddr>);

/// Combined lockstep + relay + host-migration state for a live session.
///
/// The launcher (or a headless harness) feeds received datagrams into
/// LiveSession::handle_datagram, calls LiveSession::maybe_claim after a host
/// loss, and broadcasts LiveSession::advance packets to
/// LiveSession::forward_targets. Relay duplicates are dropped and host address
/// lookups follow the elected host after a migration.
#[derive(Debug)]
pub struct LiveSession {
    pub local_player: u8,
    pub players: u8,
    pub local_address: Option<SocketAddr>,
    pub host_address: Option<SocketAddr>,
    pub peers: BTreeMap<u8, SocketAddr>,
    pub migration: HostMigration,
    pub relay: MeshRelay,
    pub queue: CommandQueue,
    pub frame: u32,
    pub state_crc: u64,
    next_sequence: u32,
}

/// Result of feeding one datagram into a live session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveEvent {
    Migration(MigrateMessage),
    Commands(NetPacket),
    Duplicate,
    Invalid,
}

impl LiveSession {
    pub fn new(
        local_player: u8,
        players: u8,
        local_address: Option<SocketAddr>,
    ) -> Result<Self, MigrationError> {
        let migration = HostMigration::new(local_player, players, 0)?;
        Ok(Self {
            local_player,
            players,
            local_address,
            host_address: if local_player == 0 {
                local_address
            } else {
                None
            },
            peers: BTreeMap::new(),
            migration,
            relay: MeshRelay::new(256),
            queue: CommandQueue::new(),
            frame: 0,
            state_crc: 0,
            next_sequence: 0,
        })
    }

    pub fn register_peer(&mut self, player: u8, address: SocketAddr) {
        self.peers.insert(player, address);
        if self.migration.host == player {
            self.host_address = Some(address);
        }
    }

    /// Recompute the current host address from the migration state.
    pub fn refresh_host_address(&mut self) {
        self.host_address = if self.migration.host == self.local_player {
            self.local_address
        } else {
            self.peers.get(&self.migration.host).copied()
        };
    }

    /// Report that a player stopped responding; clears the remembered host
    /// address when the host itself is the one being dropped.
    pub fn report_loss(&mut self, player: u8) -> Option<MigrateMessage> {
        let message = self.migration.observe_loss(player)?;
        self.host_address = None;
        Some(message)
    }

    /// Claim the host role if this peer won the election after a loss.
    pub fn maybe_claim(&mut self) -> Option<(MigrateMessage, Vec<SocketAddr>)> {
        let claim = self.migration.claim()?;
        self.refresh_host_address();
        Some((claim, self.peers.values().copied().collect()))
    }

    /// Queue a locally issued command for the current frame.
    pub fn queue_local(&mut self, command: Command) -> NetCommand {
        let command = NetCommand::new(self.frame, self.local_player, command);
        self.queue.push(command.clone());
        command
    }

    /// Feed one datagram received from a peer.
    pub fn handle_datagram(&mut self, source: SocketAddr, bytes: &[u8]) -> LiveEvent {
        if let Ok(message) = MigrateMessage::decode(bytes) {
            if self.migration.handle(message) {
                self.refresh_host_address();
                return LiveEvent::Migration(message);
            }
            return LiveEvent::Invalid;
        }
        let Ok(packet) = NetPacket::decode(bytes) else {
            return LiveEvent::Invalid;
        };
        if !packet.verify() {
            return LiveEvent::Invalid;
        }
        if !self.relay.accept(bytes) {
            return LiveEvent::Duplicate;
        }
        for command in &packet.commands {
            self.queue.push(command.clone());
        }
        let _ = source;
        LiveEvent::Commands(packet)
    }

    /// Peers that should receive a forwarded copy of a datagram from source.
    pub fn forward_targets(&self, source: SocketAddr) -> Vec<SocketAddr> {
        self.peers
            .values()
            .copied()
            .filter(|peer| *peer != source)
            .collect()
    }

    /// Drain this frame's commands, build the packet and advance the frame.
    pub fn advance(&mut self) -> Result<AdvanceBatch, PacketError> {
        let commands = self.queue.pop_frame(self.frame);
        let packet = NetPacket::with_state_crc(
            self.next_sequence,
            self.frame,
            commands.clone(),
            self.state_crc,
        );
        self.next_sequence = self.next_sequence.wrapping_add(1);
        self.frame = self.frame.wrapping_add(1);
        let bytes = packet.encode()?;
        Ok((commands, bytes, self.peers.values().copied().collect()))
    }
}

/// Bind/advertise settings used by a network setup screen or CLI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkConfig {
    pub bind_address: String,
    pub port: u16,
    pub advertised: Option<String>,
    pub max_players: u8,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 0,
            advertised: None,
            max_players: 4,
        }
    }
}

impl NetworkConfig {
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.bind_address, self.port)
    }

    /// Validate the settings before binding. Port 0 means "ephemeral", which is
    /// useful behind NAT or for tests.
    pub fn validate(&self) -> Result<(), String> {
        if self.bind_address.trim().is_empty() {
            return Err("bind address must not be empty".to_string());
        }
        if self
            .bind_address
            .trim()
            .parse::<std::net::IpAddr>()
            .is_err()
        {
            return Err("bind address must be an IP address".to_string());
        }
        if !(2..=8).contains(&self.max_players) {
            return Err("player count must be between 2 and 8".to_string());
        }
        if let Some(advertised) = &self.advertised {
            if advertised.trim().is_empty() {
                return Err("advertised address must not be empty".to_string());
            }
            if advertised.parse::<SocketAddr>().is_err() {
                return Err("advertised address must be host:port".to_string());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnc_core::Fixed16_16;

    fn point(x: i32, y: i32) -> Vec2 {
        Vec2::new(Fixed16_16::from_int(x), Fixed16_16::from_int(y))
    }

    fn move_command(frame: u32, player: u8, unit: u32) -> NetCommand {
        NetCommand::new(
            frame,
            player,
            Command::Move {
                units: vec![unit],
                target: point(4, 7),
            },
        )
    }

    #[test]
    fn command_hashes_are_stable_and_distinct() {
        let a = Command::Move {
            units: vec![1, 2],
            target: point(10, 20),
        };
        let b = a.clone();
        assert_eq!(a.hash(), b.hash());

        let different_target = Command::Move {
            units: vec![1, 2],
            target: point(10, 21),
        };
        assert_ne!(a.hash(), different_target.hash());

        let different_units = Command::Move {
            units: vec![2, 1],
            target: point(10, 20),
        };
        assert_ne!(a.hash(), different_units.hash());

        assert_eq!(Command::Deploy { unit: 3 }.tag(), 10);
        assert_eq!(Command::Sabotage { unit: 1, target: 2 }.tag(), 12);
        assert_eq!(Command::RepairStructure { building: 4 }.tag(), 13);
        assert_eq!(Command::CancelProduction.tag(), 14);
        assert_eq!(
            Command::PlaceBuilding {
                location: point(4, 6)
            }
            .tag(),
            15
        );
    }

    #[test]
    fn queue_inserts_out_of_order_commands_deterministically() {
        let mut queue = CommandQueue::new();
        queue.push(move_command(5, 1, 30));
        queue.push(move_command(2, 0, 10));
        queue.push(move_command(2, 1, 20));

        let executed = queue.pop_frame(5);
        assert_eq!(executed.len(), 3);
        assert_eq!(
            executed
                .iter()
                .map(|command| (command.frame, command.player))
                .collect::<Vec<_>>(),
            vec![(2, 0), (2, 1), (5, 1)]
        );
        assert!(queue.is_empty());
    }

    #[test]
    fn queue_keeps_future_commands() {
        let mut queue = CommandQueue::new();
        queue.push(move_command(1, 0, 1));
        queue.push(move_command(9, 0, 2));

        let executed = queue.pop_frame(1);
        assert_eq!(executed.len(), 1);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek().unwrap().frame, 9);
        assert_eq!(queue.current_frame, 1);
    }

    #[test]
    fn session_gates_commands_by_frame() {
        let mut session = LockstepSession::new(2);
        session.schedule(move_command(0, 0, 1)).unwrap();
        session.schedule(move_command(2, 1, 2)).unwrap();

        let frame_zero = session.advance();
        assert_eq!(frame_zero.len(), 1);
        assert_eq!(session.current_frame(), 1);

        assert!(session.advance().is_empty(), "frame 1 has no commands");
        assert_eq!(session.current_frame(), 2);

        let frame_two = session.advance();
        assert_eq!(frame_two.len(), 1);
        assert_eq!(frame_two[0].player, 1);
    }

    #[test]
    fn session_rejects_invalid_player_and_stale_frames() {
        let mut session = LockstepSession::new(2);
        let invalid = move_command(0, 5, 1);
        assert_eq!(
            session.schedule(invalid),
            Err(LockstepError::InvalidPlayer {
                player: 5,
                players: 2
            })
        );

        session.schedule(move_command(0, 0, 1)).unwrap();
        session.advance();
        let stale = move_command(0, 0, 2);
        assert_eq!(
            session.schedule(stale),
            Err(LockstepError::StaleFrame {
                frame: 0,
                current: 1
            })
        );
    }

    #[test]
    fn replay_records_executed_commands_in_order() {
        let mut session = LockstepSession::new(2);
        session.schedule(move_command(0, 1, 2)).unwrap();
        session.schedule(move_command(0, 0, 1)).unwrap();
        session.schedule(move_command(3, 1, 3)).unwrap();
        session.schedule(move_command(1, 0, 4)).unwrap();

        session.advance_to(3);
        assert_eq!(session.replay.len(), 4);
        assert_eq!(
            session
                .replay
                .iter()
                .map(|command| command.command.hash())
                .collect::<Vec<_>>(),
            vec![
                move_command(0, 0, 1).command.hash(),
                move_command(0, 1, 2).command.hash(),
                move_command(1, 0, 4).command.hash(),
                move_command(3, 1, 3).command.hash(),
            ]
        );
        assert_eq!(session.checksums.len(), 3);
        assert_eq!(session.checksums[0].0, 0);
    }

    #[test]
    fn replay_can_be_replayed_deterministically() {
        let mut original = LockstepSession::new(2);
        original.schedule(move_command(0, 0, 1)).unwrap();
        original.schedule(move_command(0, 1, 2)).unwrap();
        original
            .schedule(NetCommand::new(2, 0, Command::Stop { units: vec![1, 2] }))
            .unwrap();
        original.advance_to(2);

        let mut replay = LockstepSession::from_replay(2, original.replay.clone());
        replay.queue.current_frame = 0;
        let executed = replay.advance_to(2);
        assert_eq!(executed, original.replay);
        assert_eq!(replay.checksums, original.checksums);
    }

    #[test]
    fn frame_checksums_are_order_independent_and_detect_desync() {
        let first = move_command(0, 0, 1);
        let second = move_command(0, 1, 2);
        let a = LockstepSession::frame_checksum(&[first.clone(), second.clone()]);
        let b = LockstepSession::frame_checksum(&[second, first]);
        assert_eq!(a, b);

        let changed = LockstepSession::frame_checksum(&[move_command(0, 0, 99)]);
        assert_ne!(a, changed);

        let mut session = LockstepSession::new(2);
        session.schedule(move_command(0, 0, 1)).unwrap();
        session.advance();
        let local = session.checksums[0].1;
        assert!(!session.desynced(0, local));
        assert!(session.desynced(0, local ^ 1));
        assert!(!session.desynced(99, 0), "unknown frames are not claimed");
    }

    #[test]
    fn packets_checksum_and_verify() {
        let commands = vec![move_command(0, 0, 1), move_command(0, 1, 2)];
        let packet = NetPacket::new(42, 0, commands.clone());
        assert!(packet.verify());

        let mut tampered = packet.clone();
        tampered.frame = 1;
        assert!(!tampered.verify());

        let mut tampered_commands = packet.clone();
        tampered_commands.commands[0] = move_command(0, 0, 7);
        assert!(!tampered_commands.verify());

        let same = NetPacket::new(42, 0, commands);
        assert_eq!(packet.checksum, same.checksum);
    }
    /// Cell centre of a world cell (`Game::cell_center` is crate-private).
    fn cell_center(cell: i32) -> Vec2 {
        use crate::map::TILE_SIZE;
        let centre = cell as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        point(centre as i32, centre as i32)
    }

    /// One live commando, one enemy construction yard and nothing else, so the
    /// sabotage arm is the only thing that can change the state.
    fn sabotage_scene() -> Game {
        use crate::building::Building;
        use crate::td_rules::{StructureType, UnitType};
        use crate::unit::Unit;
        use crate::House;
        use cnc_core::ClassId;

        let mut game = Game::new("net-sabotage", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        game.houses
            .push(House::new(ClassId::HouseNod, "Nod".to_string()));
        game.buildings.push(Building::from_type(
            StructureType::ConstructionYard,
            cell_center(6),
            1,
        ));
        game.units
            .push(Unit::new(UnitType::Commando, cell_center(9), 0));
        game
    }

    /// Debug fingerprint of the simulation state the sabotage arm touches.
    fn sabotage_fingerprint(game: &Game) -> String {
        format!("{:?}|{:?}", game.buildings, game.units)
    }

    #[test]
    fn sabotage_command_decodes_and_reaches_the_simulation() {
        let mut game = sabotage_scene();
        let command = Command::Sabotage { unit: 0, target: 0 };
        let packet = NetPacket::new(3, 0, vec![NetCommand::new(3, 0, command.clone())]);
        let decoded = NetPacket::decode(&packet.encode().unwrap()).unwrap();
        assert_eq!(decoded.commands[0].command, command);
        assert_eq!(decoded.commands[0].command.tag(), 12);

        assert!(apply_sabotage(&mut game, &decoded.commands[0].command, 0));
        assert_eq!(game.units[0].special_target, Some(0));

        // Another player may not command someone else's commando, and the arm
        // rejects anything that is not a sabotage command.
        let mut other = sabotage_scene();
        let untouched = sabotage_fingerprint(&other);
        assert!(!apply_sabotage(&mut other, &command, 1));
        assert!(!apply_sabotage(&mut other, &Command::Deploy { unit: 0 }, 0));
        assert_eq!(
            sabotage_fingerprint(&other),
            untouched,
            "rejected commands must leave the state untouched"
        );
    }

    #[test]
    fn sabotage_apply_is_deterministic_across_peers() {
        // Two peers fed the same command stream must stay identical; that is
        // exactly what LockstepSession::frame_checksum compares between hosts
        // (see frame_checksums_are_order_independent_and_detect_desync).
        let mut first = sabotage_scene();
        let mut second = sabotage_scene();
        let command = Command::Sabotage { unit: 0, target: 0 };
        assert!(apply_sabotage(&mut first, &command, 0));
        assert!(apply_sabotage(&mut second, &command, 0));
        assert_eq!(
            sabotage_fingerprint(&first),
            sabotage_fingerprint(&second),
            "the apply arm itself must not diverge"
        );

        let mut armed_at = None;
        for step in 0..600 {
            let delta = Fixed16_16::from_f32(0.05);
            first.update(delta);
            second.update(delta);
            assert_eq!(
                sabotage_fingerprint(&first),
                sabotage_fingerprint(&second),
                "peers diverged at step {step}"
            );
            if armed_at.is_none() && first.buildings[0].is_going_to_blow {
                armed_at = Some(step);
            }
        }

        // The commando reached the yard, planted the charge
        // (INFANTRY.CPP:717-735) and the 5000-point detonation killed it
        // (BUILDING.CPP:1229-1246); both peers agree on every step.
        assert!(armed_at.is_some(), "the charge must be planted");
        assert!(!first.buildings[0].is_alive(), "the yard must be destroyed");
        assert!(!second.buildings[0].is_alive());
        assert_eq!(
            first.buildings[0].health.to_bits(),
            second.buildings[0].health.to_bits()
        );
    }
}

#[cfg(test)]
mod netcode_tests {
    use super::*;
    use cnc_core::Fixed16_16;
    use std::time::Duration;

    fn point(x: i32, y: i32) -> Vec2 {
        Vec2::new(Fixed16_16::from_int(x), Fixed16_16::from_int(y))
    }

    fn move_command(frame: u32, player: u8, unit: u32) -> NetCommand {
        NetCommand::new(
            frame,
            player,
            Command::Move {
                units: vec![unit],
                target: point(4, 7),
            },
        )
    }

    fn sample_commands() -> Vec<NetCommand> {
        vec![
            NetCommand::new(
                1,
                0,
                Command::Move {
                    units: vec![1, 2, 3],
                    target: point(-4, 9),
                },
            ),
            NetCommand::new(
                1,
                1,
                Command::Attack {
                    units: vec![4],
                    target: 9,
                },
            ),
            NetCommand::new(2, 0, Command::Stop { units: vec![] }),
            NetCommand::new(
                2,
                1,
                Command::Guard {
                    units: vec![7, 8],
                    target: 3,
                },
            ),
            NetCommand::new(
                3,
                0,
                Command::Build {
                    building: ClassId::PowerPlant,
                    location: point(12, 15),
                },
            ),
            NetCommand::new(3, 1, Command::Sell { building: 42 }),
            NetCommand::new(4, 0, Command::Repair { unit: 1, target: 2 }),
            NetCommand::new(
                4,
                1,
                Command::Power {
                    building: 5,
                    on: true,
                },
            ),
            NetCommand::new(
                5,
                0,
                Command::Produce {
                    factory: 6,
                    unit_type: ClassId::VehicleE2,
                },
            ),
            NetCommand::new(5, 1, Command::Deploy { unit: 11 }),
            NetCommand::new(6, 0, Command::Undeploy { unit: 12 }),
            NetCommand::new(
                6,
                1,
                Command::Sabotage {
                    unit: 13,
                    target: 14,
                },
            ),
            NetCommand::new(7, 0, Command::RepairStructure { building: 15 }),
            NetCommand::new(7, 1, Command::CancelProduction),
            NetCommand::new(
                8,
                0,
                Command::PlaceBuilding {
                    location: point(20, 24),
                },
            ),
        ]
    }

    #[test]
    fn all_command_variants_round_trip() {
        let packet = NetPacket::new(77, 6, sample_commands());
        let bytes = packet.encode().unwrap();
        let decoded = NetPacket::decode(&bytes).unwrap();
        assert_eq!(decoded, packet);
        assert_eq!(decoded.commands.len(), 15);
    }

    #[test]
    fn codec_rejects_bad_magic_version_and_truncation() {
        let packet = NetPacket::new(1, 0, sample_commands());
        let bytes = packet.encode().unwrap();

        let mut bad_magic = bytes.clone();
        bad_magic[0] = b'X';
        assert_eq!(
            NetPacket::decode(&bad_magic),
            Err(PacketError::InvalidMagic)
        );

        let mut bad_version = bytes.clone();
        bad_version[4] = 9;
        assert_eq!(
            NetPacket::decode(&bad_version),
            Err(PacketError::UnsupportedVersion(9))
        );

        assert_eq!(NetPacket::decode(&bytes[..10]), Err(PacketError::TooShort));

        let mut truncated = bytes.clone();
        truncated.pop();
        assert_eq!(NetPacket::decode(&truncated), Err(PacketError::Truncated));

        let mut trailing = bytes;
        trailing.push(0xAB);
        assert_eq!(
            NetPacket::decode(&trailing),
            Err(PacketError::TrailingBytes)
        );
    }

    #[test]
    fn codec_detects_corrupted_payload() {
        let packet = NetPacket::new(
            3,
            1,
            vec![NetCommand::new(1, 0, Command::Deploy { unit: 5 })],
        );
        let mut bytes = packet.encode().unwrap();
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;
        assert_eq!(
            NetPacket::decode(&bytes),
            Err(PacketError::ChecksumMismatch)
        );
    }

    #[test]
    fn codec_rejects_oversized_command_lists() {
        let commands = vec![
            NetCommand::new(0, 0, Command::Stop { units: vec![] });
            MAX_COMMANDS_PER_PACKET + 1
        ];
        let packet = NetPacket::new(0, 0, commands);
        assert!(packet.encode().is_err());
    }

    #[test]
    fn command_order_is_preserved_on_the_wire() {
        let commands = sample_commands();
        let packet = NetPacket::new(1, 6, commands.clone());
        let decoded = NetPacket::decode(&packet.encode().unwrap()).unwrap();
        assert_eq!(decoded.commands, commands);
    }

    #[tokio::test]
    async fn udp_broadcast_reaches_every_peer() {
        let mut sender = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        let receiver_a = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        let receiver_b = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        assert!(sender.add_peer(receiver_a.local_addr().unwrap()));
        assert!(sender.add_peer(receiver_b.local_addr().unwrap()));
        assert!(!sender.add_peer(receiver_a.local_addr().unwrap()));
        assert_eq!(sender.peers().len(), 2);

        let packet = NetPacket::new(15, 3, sample_commands());
        assert_eq!(sender.broadcast(&packet).await.unwrap(), 2);

        for receiver in [&receiver_a, &receiver_b] {
            let (received, from) = tokio::time::timeout(Duration::from_secs(2), receiver.recv())
                .await
                .expect("broadcast timed out")
                .unwrap();
            assert_eq!(received, packet);
            assert_eq!(from, sender.local_addr().unwrap());
        }
    }

    #[tokio::test]
    async fn udp_broadcast_without_peers_is_a_noop() {
        let sender = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        let packet = NetPacket::new(1, 0, vec![]);
        assert_eq!(sender.broadcast(&packet).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn udp_rejects_invalid_datagrams() {
        let receiver = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        let raw = tokio::net::UdpSocket::bind("127.0.0.1:0").await.unwrap();
        raw.send_to(b"not a lockstep packet", receiver.local_addr().unwrap())
            .await
            .unwrap();

        let error = tokio::time::timeout(Duration::from_secs(2), receiver.recv())
            .await
            .expect("recv timed out")
            .expect_err("garbage must be rejected");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    }

    #[tokio::test]
    async fn removed_peers_stop_receiving() {
        let mut sender = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        let receiver = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        let address = receiver.local_addr().unwrap();
        assert!(sender.add_peer(address));
        assert!(sender.remove_peer(&address));
        assert!(!sender.remove_peer(&address));
        assert_eq!(
            sender
                .broadcast(&NetPacket::new(1, 0, vec![]))
                .await
                .unwrap(),
            0
        );
    }

    fn temp_replay_path(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "cnc-replay-{tag}-{}-{nanos}.bin",
            std::process::id()
        ))
    }

    #[test]
    fn replay_round_trips_through_disk() {
        let mut session = LockstepSession::new(2);
        session.schedule(move_command(0, 0, 1)).unwrap();
        session.schedule(move_command(0, 1, 2)).unwrap();
        session.advance();
        session
            .schedule(NetCommand::new(2, 0, Command::Stop { units: vec![3] }))
            .unwrap();
        session.advance_to(2);

        let path = temp_replay_path("roundtrip");
        session.save_replay(&path).unwrap();

        let replay = Replay::load(&path).unwrap();
        assert_eq!(replay.players, 2);
        assert_eq!(replay.commands, session.replay);

        let mut restored = LockstepSession::load_replay(&path).unwrap();
        restored.queue.current_frame = 0;
        let executed = restored.advance_to(2);
        assert_eq!(executed, session.replay);
        assert_eq!(restored.checksums, session.checksums);

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn replay_rejects_corruption_and_bad_headers() {
        let replay = Replay::new(2, vec![move_command(0, 0, 1), move_command(1, 1, 2)]);
        let encoded = replay.encode();
        assert_eq!(Replay::decode(&encoded).unwrap(), replay);

        let mut corrupted = encoded.clone();
        corrupted[12] ^= 0xff;
        assert!(matches!(
            Replay::decode(&corrupted),
            Err(ReplayError::ChecksumMismatch)
        ));

        let mut bad_magic = encoded.clone();
        bad_magic[0] = b'X';
        assert!(matches!(
            Replay::decode(&bad_magic),
            Err(ReplayError::InvalidMagic)
        ));

        let mut bad_version = encoded.clone();
        bad_version[4] = 99;
        assert!(matches!(
            Replay::decode(&bad_version),
            Err(ReplayError::UnsupportedVersion(99))
        ));

        assert!(matches!(
            Replay::decode(&encoded[..8]),
            Err(ReplayError::TooShort)
        ));
    }

    #[test]
    fn replay_load_reports_missing_files() {
        let path = temp_replay_path("missing");
        assert!(matches!(Replay::load(&path), Err(ReplayError::Io(_))));
    }

    #[test]
    fn handshake_messages_round_trip() {
        let messages = [
            HandshakeMessage::JoinRequest {
                player: 1,
                players: 4,
            },
            HandshakeMessage::JoinAccepted { player: 1, host: 0 },
            HandshakeMessage::JoinRejected {
                player: 2,
                reason: JoinReject::RoomFull,
            },
            HandshakeMessage::Leave { player: 3 },
        ];
        for message in messages {
            let encoded = message.encode();
            assert_eq!(encoded.len(), HANDSHAKE_LEN);
            assert_eq!(HandshakeMessage::decode(&encoded).unwrap(), message);
        }

        assert!(matches!(
            HandshakeMessage::decode(b"CNCH"),
            Err(HandshakeError::TooShort)
        ));
        let mut bad = HandshakeMessage::Leave { player: 0 }.encode();
        bad[5] = 9;
        assert!(matches!(
            HandshakeMessage::decode(&bad),
            Err(HandshakeError::UnknownKind(9))
        ));
    }

    #[test]
    fn datagram_dispatch_routes_command_and_handshake_frames() {
        let command = Datagram::Command(NetPacket::new(1, 0, vec![move_command(0, 0, 1)]));
        let encoded = match &command {
            Datagram::Command(packet) => packet.encode().unwrap(),
            Datagram::Handshake(_) => unreachable!(),
        };
        assert_eq!(Datagram::decode(&encoded).unwrap(), command);

        let handshake = Datagram::Handshake(HandshakeMessage::Leave { player: 1 });
        let encoded = match &handshake {
            Datagram::Handshake(message) => message.encode(),
            Datagram::Command(_) => unreachable!(),
        };
        assert_eq!(Datagram::decode(&encoded).unwrap(), handshake);

        let error = Datagram::decode(b"NOPE").expect_err("unknown magic is rejected");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    }

    #[test]
    fn handshake_host_validates_slots_and_duplicates() {
        let mut host = PeerHandshake::host_room(0, 3).unwrap();
        let first = "127.0.0.1:9101".parse().unwrap();
        let second = "127.0.0.1:9102".parse().unwrap();

        let request = HandshakeMessage::JoinRequest {
            player: 1,
            players: 3,
        };
        let step = host.handle(first, request).unwrap();
        assert_eq!(
            step.event,
            Some(PeerEvent::PeerJoined {
                player: 1,
                address: first
            })
        );
        assert_eq!(
            step.reply,
            Some(HandshakeMessage::JoinAccepted { player: 1, host: 0 })
        );

        // Retransmission from the same address is acknowledged, not duplicated.
        let step = host.handle(first, request).unwrap();
        assert!(step.event.is_none());
        assert_eq!(
            step.reply,
            Some(HandshakeMessage::JoinAccepted { player: 1, host: 0 })
        );

        // A second address cannot steal the slot.
        let step = host.handle(second, request).unwrap();
        assert_eq!(
            step.reply,
            Some(HandshakeMessage::JoinRejected {
                player: 1,
                reason: JoinReject::AlreadyJoined
            })
        );

        // Out-of-range and mismatched-room requests are refused.
        let step = host
            .handle(
                second,
                HandshakeMessage::JoinRequest {
                    player: 3,
                    players: 3,
                },
            )
            .unwrap();
        assert_eq!(
            step.reply,
            Some(HandshakeMessage::JoinRejected {
                player: 3,
                reason: JoinReject::InvalidPlayer
            })
        );
        let step = host
            .handle(
                second,
                HandshakeMessage::JoinRequest {
                    player: 2,
                    players: 4,
                },
            )
            .unwrap();
        assert_eq!(
            step.reply,
            Some(HandshakeMessage::JoinRejected {
                player: 2,
                reason: JoinReject::VersionMismatch
            })
        );

        let leave = HandshakeMessage::Leave { player: 1 };
        let step = host.handle(first, leave).unwrap();
        assert_eq!(
            step.event,
            Some(PeerEvent::PeerLeft {
                player: 1,
                address: first
            })
        );
        assert!(host.peer(1).is_none());
    }

    #[test]
    fn handshake_rejects_closed_rooms() {
        let mut idle = PeerHandshake::new(1, 2).unwrap();
        let address = "127.0.0.1:9200".parse().unwrap();
        let step = idle
            .handle(
                address,
                HandshakeMessage::JoinRequest {
                    player: 0,
                    players: 2,
                },
            )
            .unwrap();
        assert_eq!(
            step.reply,
            Some(HandshakeMessage::JoinRejected {
                player: 0,
                reason: JoinReject::RoomClosed
            })
        );

        let request = idle.begin_join(address).unwrap();
        assert_eq!(
            request,
            HandshakeMessage::JoinRequest {
                player: 1,
                players: 2
            }
        );
        // Retrying while a join is already in flight is allowed; the host is
        // the one that resolves duplicates.
        assert_eq!(idle.begin_join(address).unwrap(), request);

        // A peer that is already in a room cannot begin a second join.
        let mut joined = PeerHandshake::host_room(0, 2).unwrap();
        assert!(matches!(
            joined.begin_join(address),
            Err(HandshakeError::InvalidState)
        ));
    }

    #[tokio::test]
    async fn handshake_join_and_leave_over_loopback() {
        let host_socket = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        let client_socket = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        let host_addr = host_socket.local_addr().unwrap();
        let client_addr = client_socket.local_addr().unwrap();

        let mut host = PeerHandshake::host_room(0, 4).unwrap();
        let mut client = PeerHandshake::new(1, 4).unwrap();

        let request = client.begin_join(host_addr).unwrap();
        client_socket
            .send_handshake(host_addr, &request)
            .await
            .unwrap();

        let (datagram, from) =
            tokio::time::timeout(Duration::from_secs(2), host_socket.recv_datagram())
                .await
                .expect("join request timed out")
                .unwrap();
        assert_eq!(from, client_addr);
        let Datagram::Handshake(message) = datagram else {
            panic!("expected a handshake datagram");
        };
        let step = host.handle(from, message).unwrap();
        assert!(step.event.is_some());
        let reply = step.reply.expect("host answers the join");
        host_socket.send_handshake(from, &reply).await.unwrap();

        let (datagram, from) =
            tokio::time::timeout(Duration::from_secs(2), client_socket.recv_datagram())
                .await
                .expect("join answer timed out")
                .unwrap();
        assert_eq!(from, host_addr);
        let Datagram::Handshake(message) = datagram else {
            panic!("expected a handshake datagram");
        };
        let step = client.handle(from, message).unwrap();
        assert_eq!(
            step.event,
            Some(PeerEvent::JoinAccepted {
                host_player: 0,
                host: host_addr
            })
        );

        assert!(host.is_joined());
        assert!(client.is_joined());
        assert_eq!(host.peer(1), Some(client_addr));
        assert_eq!(client.peer(0), Some(host_addr));
        assert_eq!(client.host_player(), Some(0));

        let leave = client.leave();
        assert_eq!(client.state, PeerState::Closed);
        client_socket
            .send_handshake(host_addr, &leave)
            .await
            .unwrap();

        let (datagram, from) =
            tokio::time::timeout(Duration::from_secs(2), host_socket.recv_datagram())
                .await
                .expect("leave timed out")
                .unwrap();
        let Datagram::Handshake(message) = datagram else {
            panic!("expected a handshake datagram");
        };
        let step = host.handle(from, message).unwrap();
        assert_eq!(
            step.event,
            Some(PeerEvent::PeerLeft {
                player: 1,
                address: client_addr
            })
        );
        assert!(host.peer(1).is_none());
    }
}

#[cfg(test)]
mod migration_tests {
    use super::*;

    #[test]
    fn migration_message_roundtrips() {
        for message in [
            MigrateMessage::HostLost {
                generation: 3,
                reporter: 2,
            },
            MigrateMessage::HostClaim {
                generation: 3,
                host: 1,
            },
        ] {
            let bytes = message.encode();
            assert_eq!(MigrateMessage::decode(&bytes).unwrap(), message);
        }
        assert_eq!(
            MigrateMessage::decode(&[0u8; 4]).unwrap_err(),
            MigrationError::TooShort
        );
        let mut bad = MigrateMessage::HostLost {
            generation: 0,
            reporter: 1,
        }
        .encode();
        bad[0] = b'X';
        assert_eq!(
            MigrateMessage::decode(&bad).unwrap_err(),
            MigrationError::InvalidMagic
        );
    }

    #[test]
    fn host_loss_elects_lowest_alive_and_claim_is_accepted() {
        let mut peer_one = HostMigration::new(1, 3, 0).unwrap();
        let mut peer_two = HostMigration::new(2, 3, 0).unwrap();

        let loss_one = peer_one.observe_loss(0).unwrap();
        let loss_two = peer_two.observe_loss(0).unwrap();
        assert_eq!(
            loss_one,
            MigrateMessage::HostLost {
                generation: 1,
                reporter: 1
            }
        );
        assert_eq!(
            loss_two,
            MigrateMessage::HostLost {
                generation: 1,
                reporter: 2
            }
        );

        // Only the lowest living player wins the election.
        assert_eq!(
            peer_one.claim(),
            Some(MigrateMessage::HostClaim {
                generation: 1,
                host: 1
            })
        );
        assert_eq!(peer_two.claim(), None);
        assert!(peer_one.is_host());
        assert_eq!(peer_one.state, MigrationState::Migrated);

        let claim = MigrateMessage::HostClaim {
            generation: 1,
            host: 1,
        };
        assert!(peer_two.handle(claim));
        assert_eq!(peer_two.host, 1);
        assert_eq!(peer_two.state, MigrationState::Migrated);
    }

    #[test]
    fn peer_that_missed_the_loss_resynchronises_before_claiming() {
        let mut peer_two = HostMigration::new(2, 3, 0).unwrap();
        let loss = MigrateMessage::HostLost {
            generation: 1,
            reporter: 1,
        };
        assert!(peer_two.handle(loss));
        assert_eq!(peer_two.generation, 1);
        assert_eq!(peer_two.state, MigrationState::AwaitingClaim);
        assert!(peer_two.alive.contains(&1));
        assert!(!peer_two.alive.contains(&0));
        assert_eq!(peer_two.elected(), Some(1));
    }

    #[test]
    fn non_host_departure_keeps_the_host_stable() {
        let mut peer = HostMigration::new(0, 3, 0).unwrap();
        assert_eq!(peer.observe_loss(2), None);
        assert_eq!(peer.state, MigrationState::Stable);
        assert_eq!(peer.host, 0);
        assert_eq!(peer.generation, 0);
    }

    #[test]
    fn stale_or_illegitimate_claims_are_rejected() {
        let mut peer = HostMigration::new(2, 3, 0).unwrap();
        peer.observe_loss(0).unwrap();

        // Wrong generation.
        assert!(!peer.handle(MigrateMessage::HostClaim {
            generation: 0,
            host: 1
        }));
        // Not the deterministic winner.
        assert!(!peer.handle(MigrateMessage::HostClaim {
            generation: 1,
            host: 2
        }));
        // Unknown player.
        assert!(!peer.handle(MigrateMessage::HostClaim {
            generation: 1,
            host: 9
        }));
        assert_eq!(peer.state, MigrationState::AwaitingClaim);
    }

    #[test]
    fn relay_cache_drops_duplicates_and_evicts() {
        let mut relay = MeshRelay::new(2);
        assert!(relay.accept(b"first"));
        assert!(!relay.accept(b"first"));
        assert!(relay.accept(b"second"));
        assert!(relay.accept(b"third"));
        // "first" was evicted, so it may be forwarded again.
        assert!(relay.accept(b"first"));
        assert_eq!(relay.len(), 2);
    }

    #[test]
    fn relay_clear_accepts_replayed_packets() {
        let mut relay = MeshRelay::new(2);
        assert!(relay.accept(b"before-reconnect"));
        assert!(!relay.accept(b"before-reconnect"));
        relay.clear();
        assert!(relay.is_empty());
        assert!(relay.accept(b"before-reconnect"), "cleared hashes are gone");
    }

    #[tokio::test]
    async fn relay_forwards_to_others_but_not_the_source() {
        let source = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        let target = UdpTransport::bind("127.0.0.1:0").await.unwrap();
        let mut relay = UdpTransport::bind("127.0.0.1:0").await.unwrap();

        let source_addr = source.local_addr().unwrap();
        let target_addr = target.local_addr().unwrap();
        let relay_addr = relay.local_addr().unwrap();
        relay.add_peer(source_addr);
        relay.add_peer(target_addr);

        let mut cache = MeshRelay::new(8);
        assert_eq!(
            relay
                .relay_from(source_addr, b"payload", &mut cache)
                .await
                .unwrap(),
            1
        );

        let mut buffer = [0u8; 32];
        let (length, from) = target.socket.recv_from(&mut buffer).await.unwrap();
        assert_eq!(&buffer[..length], b"payload");
        assert_eq!(from, relay_addr);

        // The source never receives its own datagram back.
        let leaked = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            source.socket.recv_from(&mut buffer),
        )
        .await;
        assert!(leaked.is_err(), "relay must not echo to the source");

        // Second copy is dropped by the cache.
        assert_eq!(
            relay
                .relay_from(source_addr, b"payload", &mut cache)
                .await
                .unwrap(),
            0
        );
    }
}
