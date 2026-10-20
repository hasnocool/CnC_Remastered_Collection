//! Publish an authored agent to the AI-only ranked ladder.
//!
//! A scenario stores AgentDefinition records. The directory's AI ladder wants a
//! compact card instead of the whole scenario: owner, doctrine, a fingerprint of
//! the exact script, and optional creator / scenario labels. Build it here, then
//! POST the registration to any directory node:
//!
//! ~~~no_run
//! # use cnc_ai::agent::AgentDefinition;
//! # use cnc_ai::ladder::registration;
//! let agent = AgentDefinition::new(3, "Agent K");
//! let body = registration(&agent, Some("Kane"), Some("SCG01EA"));
//! let json = serde_json::to_string(&body).expect("serialize");
//! // POST json to /api/v1/lfg/players
//! ~~~
//!
//! The directory rejects a human profile that carries a card and never pairs an
//! AI competitor with a human, so the two ladders stay independent.

use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::agent::AgentDefinition;

/// JSON card accepted by the directory's AI ladder.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentLadderCard {
    /// Map-local agent id from the scenario.
    pub agent_id: Option<u32>,
    /// House class such as HouseGDI or HouseNod.
    pub owner: String,
    /// Doctrine chosen in the editor.
    pub doctrine: String,
    /// Hex SHA-1 of the agent script.
    pub script_hash: String,
    /// Human author or creator of the agent.
    pub creator: Option<String>,
    /// Scenario the agent was authored in.
    pub scenario: Option<String>,
}

/// Registration body for POST /api/v1/lfg/players.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AiLadderRegistration {
    /// Display name shown on the AI leaderboard.
    pub name: String,
    /// Always ai; the directory routes this profile to the AI-only ladder.
    pub controller: &'static str,
    /// Agent provenance card.
    pub agent: AgentLadderCard,
}

impl AgentDefinition {
    /// Hex SHA-1 fingerprint of the agent script, stable across edits that do
    /// not change the script text.
    #[must_use]
    pub fn script_fingerprint(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(self.script.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Build the card the directory stores for this AI competitor.
    #[must_use]
    pub fn ladder_card(&self, creator: Option<&str>, scenario: Option<&str>) -> AgentLadderCard {
        AgentLadderCard {
            agent_id: Some(self.id),
            owner: self.owner.clone(),
            doctrine: self.doctrine.clone(),
            script_hash: self.script_fingerprint(),
            creator: creator.map(str::to_owned),
            scenario: scenario.map(str::to_owned),
        }
    }

    /// Build the complete registration body for the AI ladder.
    #[must_use]
    pub fn ladder_registration(
        &self,
        creator: Option<&str>,
        scenario: Option<&str>,
    ) -> AiLadderRegistration {
        AiLadderRegistration {
            name: self.name.clone(),
            controller: "ai",
            agent: self.ladder_card(creator, scenario),
        }
    }
}

/// Convenience wrapper around AgentDefinition::ladder_registration.
#[must_use]
pub fn registration(
    agent: &AgentDefinition,
    creator: Option<&str>,
    scenario: Option<&str>,
) -> AiLadderRegistration {
    agent.ladder_registration(creator, scenario)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registration_pins_the_script_and_doctrine() {
        let mut agent = AgentDefinition::new(3, "Agent K");
        agent.owner = "HouseNod".to_owned();
        agent.doctrine = "Aggressive".to_owned();
        let body = agent.ladder_registration(Some("Kane"), Some("SCG01EA"));
        assert_eq!(body.controller, "ai");
        assert_eq!(body.name, "Agent K");
        assert_eq!(body.agent.agent_id, Some(3));
        assert_eq!(body.agent.owner, "HouseNod");
        assert_eq!(body.agent.doctrine, "Aggressive");
        assert_eq!(body.agent.script_hash.len(), 40);
        assert!(body
            .agent
            .script_hash
            .chars()
            .all(|ch| ch.is_ascii_hexdigit()));
        let json = serde_json::to_value(&body).expect("serialize");
        assert_eq!(json["controller"], "ai");
        assert_eq!(json["agent"]["scenario"], "SCG01EA");
    }

    #[test]
    fn fingerprint_changes_with_the_script() {
        let mut agent = AgentDefinition::new(1, "Agent");
        let before = agent.script_fingerprint();
        agent.script.push_str("\n# changed\n");
        assert_ne!(before, agent.script_fingerprint());
    }
}
