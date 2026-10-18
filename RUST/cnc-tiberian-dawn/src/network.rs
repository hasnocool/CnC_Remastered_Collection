//! Network/Command queue for Tiberian Dawn

use cnc_core::{ClassId, Vec2};
use std::collections::VecDeque;

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetCommand {
    pub frame: u32,
    pub player: u8,
    pub command: Command,
}

impl NetCommand {
    pub fn new(frame: u32, player: u8, command: Command) -> Self {
        Self { frame, player, command }
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
        (left.frame, left.player, left.command.hash(), left.command.tag())
            .cmp(&(right.frame, right.player, right.command.hash(), right.command.tag()))
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
}

impl LockstepSession {
    pub fn new(players: u8) -> Self {
        Self {
            queue: CommandQueue::new(),
            players: players.max(1),
            replay: Vec::new(),
            checksums: Vec::new(),
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
            .map_or(false, |(_, local)| *local != remote_checksum)
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
#[derive(Debug, Clone)]
pub struct NetPacket {
    pub sequence: u32,
    pub frame: u32,
    pub commands: Vec<NetCommand>,
    pub checksum: u32,
}

impl NetPacket {
    pub fn new(sequence: u32, frame: u32, commands: Vec<NetCommand>) -> Self {
        let mut packet = Self {
            sequence,
            frame,
            commands,
            checksum: 0,
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
        (hash & 0xFFFF_FFFF) as u32
    }

    pub fn verify(&self) -> bool {
        self.checksum == self.calculate_checksum()
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
}
