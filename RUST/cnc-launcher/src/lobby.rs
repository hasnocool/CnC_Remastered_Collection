use std::time::Instant;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Peer {
    pub id: Uuid,
    pub display_name: String,
    pub ready: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct ChatMessage {
    pub id: Uuid,
    pub from: Uuid,
    pub text: String,
    pub timestamp: Instant,
}

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LobbyError {
    UnknownPeer,
    ChatFull,
}

#[allow(dead_code)]
pub struct LobbyState {
    peers: Vec<Peer>,
    chat: Vec<ChatMessage>,
    max_chat: usize,
}

#[allow(dead_code)]
impl LobbyState {
    pub fn new(max_chat: usize) -> Self {
        Self {
            peers: Vec::new(),
            chat: Vec::new(),
            max_chat,
        }
    }

    pub fn add_peer(&mut self, id: Uuid, display_name: String) -> Result<(), LobbyError> {
        self.peers.push(Peer {
            id,
            display_name,
            ready: false,
        });
        Ok(())
    }

    pub fn remove_peer(&mut self, id: Uuid) {
        self.peers.retain(|p| p.id != id);
    }

    pub fn set_ready(&mut self, id: Uuid, ready: bool) -> Result<(), LobbyError> {
        let peer = self
            .peers
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or(LobbyError::UnknownPeer)?;
        peer.ready = ready;
        Ok(())
    }

    pub fn send_chat(&mut self, from: Uuid, text: String) -> Result<(), LobbyError> {
        if self.max_chat == 0 {
            return Err(LobbyError::ChatFull);
        }
        self.chat.push(ChatMessage {
            id: Uuid::new_v4(),
            from,
            text,
            timestamp: Instant::now(),
        });
        if self.chat.len() > self.max_chat {
            self.chat.remove(0);
        }
        Ok(())
    }

    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn ready_peer_count(&self) -> usize {
        self.peers.iter().filter(|p| p.ready).count()
    }

    pub fn chat_history(&self) -> Vec<ChatMessage> {
        self.chat.clone()
    }

    pub fn peers(&self) -> &[Peer] {
        &self.peers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_toggle_idempotent() {
        let mut s = LobbyState::new(10);
        assert!(s.add_peer(Uuid::new_v4(), "A".into()).is_ok());
        let id = s.peers()[0].id;
        assert!(s.set_ready(id, true).is_ok());
        assert!(s.set_ready(id, true).is_ok());
        assert_eq!(s.ready_peer_count(), 1);
    }

    #[test]
    fn chat_cap_prevents_unbounded_growth() {
        let mut s = LobbyState::new(2);
        let id = Uuid::new_v4();
        assert!(s.add_peer(id, "A".into()).is_ok());
        assert!(s.send_chat(id, "1".into()).is_ok());
        assert!(s.send_chat(id, "2".into()).is_ok());
        assert!(s.send_chat(id, "3".into()).is_ok());
        assert!(s.chat_history().len() <= 2);
    }

    #[test]
    fn history_preserves_append_order() {
        let mut s = LobbyState::new(10);
        let id = Uuid::new_v4();
        assert!(s.add_peer(id, "A".into()).is_ok());
        assert!(s.send_chat(id, "A".into()).is_ok());
        assert!(s.send_chat(id, "B".into()).is_ok());
        assert!(s.send_chat(id, "C".into()).is_ok());
        let hist = s.chat_history();
        assert_eq!(hist[0].text, "A");
        assert_eq!(hist[1].text, "B");
        assert_eq!(hist[2].text, "C");
    }

    #[test]
    fn unknown_peer_returns_error() {
        let mut s = LobbyState::new(10);
        assert_eq!(
            s.set_ready(Uuid::new_v4(), true),
            Err(LobbyError::UnknownPeer)
        );
    }

    #[test]
    fn empty_lobby_stats() {
        let s = LobbyState::new(10);
        assert_eq!(s.peer_count(), 0);
        assert_eq!(s.ready_peer_count(), 0);
    }
}
