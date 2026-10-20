//! Ready-up protocol for the host/join network lobby.
//!
//! Ready flags travel as out-of-band UDP datagrams, exactly like text chat:
//! peers exchange `CNCREADY 1 <player> <0|1>` lines that are consumed before
//! any datagram reaches the deterministic lockstep session, so they can never
//! desync a match. Roster state reuses the lobby module's `Peer` / ready model
//! (`crate::lobby`), keeping one peer identity per player name.
//!
//! Integration points for the network session (wired by the lobby lane):
//! - receive: check `parse_ready_line` before `LiveSession::handle_datagram`,
//!   mirroring the `parse_chat_line` guards in both poll loops.
//! - send: `format_ready_line` per peer address, mirroring `send_chat`.
//! - draw: `ReadyTracker::roster` feeds lobby rows; the host start gate reads
//!   `ReadyTracker::all_ready`.

use std::collections::BTreeMap;

use uuid::Uuid;

use crate::lobby::{LobbyError, LobbyState, Peer};

/// Ready-up datagram prefix; the trailing ` 1` is the protocol version.
/// Mirrors `CHAT_PREFIX` in `main.rs`.
pub const READY_PREFIX: &str = "CNCREADY 1";

/// Parse `CNCREADY 1 <player> <0|1>`.
///
/// Returns the player name and its ready flag. A trailing newline (the send
/// path appends one, like chat) is tolerated; anything malformed yields
/// `None` so callers fall through to the lockstep handler instead of
/// panicking.
pub fn parse_ready_line(bytes: &[u8]) -> Option<(String, bool)> {
    let text = std::str::from_utf8(bytes).ok()?;
    let rest = text.strip_prefix(READY_PREFIX)?.trim_start();
    let (player, flag) = rest.split_once(' ')?;
    let ready = match flag.trim() {
        "0" => false,
        "1" => true,
        _ => return None,
    };
    if player.is_empty() {
        return None;
    }
    Some((player.to_string(), ready))
}

/// Serialize a ready flag for the send path; mirrors `send_chat`'s line shape
/// (`format!("{CHAT_PREFIX} {local} {message}\n")`).
pub fn format_ready_line(player: &str, ready: bool) -> String {
    format!("{READY_PREFIX} {player} {}\n", u8::from(ready))
}

/// Host-side ready roster for one lobby.
///
/// Peers are tracked by player name through the lobby module's `Peer` / ready
/// model; identity is the name the peer sends in its ready datagram. Ready
/// state is a pure lobby concern: nothing here touches the lockstep command
/// stream. Player names are single tokens, like the chat player field.
pub struct ReadyTracker {
    roster: LobbyState,
    /// Player name -> roster identity, kept sorted for deterministic output.
    ids: BTreeMap<String, Uuid>,
    /// Number of players that must ready up before the host may start.
    expected: usize,
}

impl ReadyTracker {
    /// A tracker with an empty roster and no expected players.
    pub fn new() -> Self {
        Self {
            // max_chat 0: the ready tracker never keeps chat history; chat is
            // an out-of-band concern of the network session itself.
            roster: LobbyState::new(0),
            ids: BTreeMap::new(),
            expected: 0,
        }
    }

    /// Record a player's ready flag, registering first-time senders.
    pub fn mark_ready(&mut self, player: &str, ready: bool) -> Result<(), LobbyError> {
        let id = *self
            .ids
            .entry(player.to_string())
            .or_insert_with(Uuid::new_v4);
        if !self.roster.peers().iter().any(|peer| peer.id == id) {
            self.roster.add_peer(id, player.to_string())?;
        }
        self.roster.set_ready(id, ready)
    }

    /// Drop a player that left the lobby (its slot went away).
    pub fn unregister(&mut self, player: &str) {
        if let Some(id) = self.ids.remove(player) {
            self.roster.remove_peer(id);
        }
    }

    /// Forget every player and reset the expected count.
    pub fn clear(&mut self) {
        self.roster = LobbyState::new(0);
        self.ids.clear();
        self.expected = 0;
    }

    /// How many registered players are currently ready.
    pub fn ready_count(&self) -> usize {
        self.roster.ready_peer_count()
    }

    /// Set how many players must ready up (0 disables the start gate).
    pub fn set_expected(&mut self, count: usize) {
        self.expected = count;
    }

    /// The expected player count the start gate checks against.
    pub fn expected_count(&self) -> usize {
        self.expected
    }

    /// True once every expected player is ready.
    ///
    /// False while `expected` is 0: a lobby with no expected players can never
    /// be "all ready", so the host cannot start an empty match.
    pub fn all_ready(&self) -> bool {
        self.expected > 0 && self.ready_count() >= self.expected
    }

    /// Whether a specific player is currently marked ready.
    pub fn is_ready(&self, player: &str) -> bool {
        let Some(id) = self.ids.get(player) else {
            return false;
        };
        self.roster
            .peers()
            .iter()
            .any(|peer| peer.id == *id && peer.ready)
    }

    /// The roster as `(player, ready)` rows sorted by player name.
    ///
    /// Sorted output (not the peer vector's insertion order) keeps lobby
    /// rendering deterministic regardless of datagram arrival order.
    pub fn roster(&self) -> Vec<(String, bool)> {
        let mut rows: Vec<(String, bool)> = self
            .roster
            .peers()
            .iter()
            .map(|peer: &Peer| (peer.display_name.clone(), peer.ready))
            .collect();
        rows.sort();
        rows
    }
}

impl Default for ReadyTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_reads_ready_flag() {
        assert_eq!(
            parse_ready_line(b"CNCREADY 1 alice 1"),
            Some(("alice".to_string(), true))
        );
        assert_eq!(
            parse_ready_line(b"CNCREADY 1 alice 0"),
            Some(("alice".to_string(), false))
        );
    }

    #[test]
    fn parse_tolerates_trailing_newline_like_chat() {
        assert_eq!(
            parse_ready_line(b"CNCREADY 1 bob 1\n"),
            Some(("bob".to_string(), true))
        );
        assert_eq!(
            parse_ready_line(b"CNCREADY 1 bob 0\r\n"),
            Some(("bob".to_string(), false))
        );
    }

    #[test]
    fn parse_rejects_wrong_prefix() {
        assert_eq!(parse_ready_line(b"CNCCHAT 1 0 hello"), None);
        assert_eq!(parse_ready_line(b"cncready 1 alice 1"), None);
        assert_eq!(parse_ready_line(b"CNCREADY 2 alice 1"), None);
        assert_eq!(parse_ready_line(b"XCNCREADY 1 alice 1"), None);
    }

    #[test]
    fn parse_rejects_truncated_lines() {
        assert_eq!(parse_ready_line(b""), None);
        assert_eq!(parse_ready_line(b"CNCREADY 1"), None);
        assert_eq!(parse_ready_line(b"CNCREADY 1 alice"), None);
        // Prefix only: no player/flag pair survives the split.
        assert_eq!(parse_ready_line(b"CNCREADY 1  0"), None);
    }

    #[test]
    fn parse_rejects_bad_flag_values() {
        for line in [
            &b"CNCREADY 1 alice 2"[..],
            &b"CNCREADY 1 alice yes"[..],
            &b"CNCREADY 1 alice true"[..],
            &b"CNCREADY 1 alice 01"[..],
            &b"CNCREADY 1 alice "[..],
        ] {
            assert_eq!(parse_ready_line(line), None, "{line:?} should not parse");
        }
    }

    #[test]
    fn parse_rejects_extra_tokens() {
        assert_eq!(parse_ready_line(b"CNCREADY 1 alice 1 extra"), None);
    }

    #[test]
    fn parse_rejects_non_utf8() {
        assert_eq!(parse_ready_line(b"CNCREADY 1 \xff 1"), None);
    }

    #[test]
    fn format_emits_wire_line() {
        assert_eq!(format_ready_line("alice", true), "CNCREADY 1 alice 1\n");
        assert_eq!(format_ready_line("alice", false), "CNCREADY 1 alice 0\n");
    }

    #[test]
    fn format_round_trips_through_parse() {
        for (name, ready) in [("alice", true), ("Bob", false), ("p-3", true)] {
            let line = format_ready_line(name, ready);
            assert_eq!(
                parse_ready_line(line.as_bytes()),
                Some((name.to_string(), ready))
            );
        }
    }

    #[test]
    fn mark_ready_toggles_and_counts() {
        let mut tracker = ReadyTracker::new();
        assert!(tracker.mark_ready("alice", true).is_ok());
        assert_eq!(tracker.ready_count(), 1);
        assert!(tracker.is_ready("alice"));
        assert!(tracker.mark_ready("alice", false).is_ok());
        assert_eq!(tracker.ready_count(), 0);
        assert!(!tracker.is_ready("alice"));
    }

    #[test]
    fn marking_same_name_twice_keeps_one_roster_row() {
        let mut tracker = ReadyTracker::new();
        tracker.mark_ready("alice", true).unwrap();
        tracker.mark_ready("alice", true).unwrap();
        assert_eq!(tracker.roster().len(), 1);
        assert_eq!(tracker.ready_count(), 1);
    }

    #[test]
    fn unregister_drops_the_player() {
        let mut tracker = ReadyTracker::new();
        tracker.mark_ready("alice", true).unwrap();
        tracker.mark_ready("bob", true).unwrap();
        tracker.unregister("alice");
        assert_eq!(tracker.ready_count(), 1);
        assert_eq!(tracker.roster(), vec![("bob".to_string(), true)]);
        // Unregistering an unknown player is a no-op.
        tracker.unregister("carol");
        assert_eq!(tracker.roster().len(), 1);
    }

    #[test]
    fn all_ready_requires_positive_expected_count() {
        let mut tracker = ReadyTracker::new();
        tracker.mark_ready("alice", true).unwrap();
        assert!(!tracker.all_ready(), "zero expected means no start gate");
        tracker.set_expected(2);
        assert!(!tracker.all_ready(), "one of two ready");
        tracker.mark_ready("bob", true).unwrap();
        assert!(tracker.all_ready(), "both ready hits the boundary");
    }

    #[test]
    fn all_ready_saturates_when_extra_players_ready_up() {
        let mut tracker = ReadyTracker::new();
        tracker.set_expected(1);
        tracker.mark_ready("a", true).unwrap();
        tracker.mark_ready("b", true).unwrap();
        assert!(tracker.all_ready(), "ready_count >= expected");
    }

    #[test]
    fn all_ready_false_while_anyone_unreadies() {
        let mut tracker = ReadyTracker::new();
        tracker.set_expected(2);
        tracker.mark_ready("alice", true).unwrap();
        tracker.mark_ready("bob", true).unwrap();
        assert!(tracker.all_ready());
        tracker.mark_ready("bob", false).unwrap();
        assert!(!tracker.all_ready());
    }

    #[test]
    fn roster_output_is_sorted_not_arrival_order() {
        let mut tracker = ReadyTracker::new();
        for name in ["carl", "alice", "bob"] {
            tracker.mark_ready(name, true).unwrap();
        }
        assert_eq!(
            tracker.roster(),
            vec![
                ("alice".to_string(), true),
                ("bob".to_string(), true),
                ("carl".to_string(), true),
            ]
        );
    }

    #[test]
    fn clear_resets_roster_and_gate() {
        let mut tracker = ReadyTracker::new();
        tracker.set_expected(2);
        tracker.mark_ready("alice", true).unwrap();
        tracker.clear();
        assert_eq!(tracker.ready_count(), 0);
        assert_eq!(tracker.expected_count(), 0);
        assert_eq!(tracker.roster().len(), 0);
        assert!(!tracker.all_ready());
    }
}
