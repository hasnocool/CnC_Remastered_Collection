//! The scenario-side agent record and starter doctrines.

use serde::{Deserialize, Serialize};

use crate::script::{class_references, compile, Diagnostic};

/// Doctrines offered by the editor. A doctrine selects a starter script; it
/// does not constrain what the author can write.
pub const DOCTRINES: [&str; 5] = ["Balanced", "Aggressive", "Defensive", "Economy", "Advisor"];

/// An AI commander authored in a scenario.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentDefinition {
    /// Stable id inside the scenario.
    pub id: u32,
    pub name: String,
    /// House class name such as HouseGDI or HouseNod; empty means unassigned.
    pub owner: String,
    pub doctrine: String,
    pub script: String,
}

impl AgentDefinition {
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            owner: String::new(),
            doctrine: "Balanced".to_string(),
            script: starter_script("Balanced").to_string(),
        }
    }

    pub fn compile(&self) -> Result<crate::script::Program, Vec<Diagnostic>> {
        compile(&self.script)
    }

    /// Human-readable validation messages for the editor and scenario checks.
    ///
    /// The catalog argument lists the class names the active game plugin
    /// offers. Pass an empty slice to skip class checking.
    pub fn validate(&self, catalog: &[String]) -> Vec<String> {
        let mut issues = Vec::new();
        if self.id == 0 {
            issues.push("agent id must not be zero".to_string());
        }
        if self.name.trim().is_empty() {
            issues.push("agent name is empty".to_string());
        }
        if self.owner.trim().is_empty() {
            issues.push(format!("agent {} has no owner house", self.name));
        }
        if !DOCTRINES.iter().any(|doctrine| *doctrine == self.doctrine) {
            issues.push(format!(
                "agent {} has unknown doctrine {}",
                self.name, self.doctrine
            ));
        }
        match self.compile() {
            Ok(program) => {
                if !catalog.is_empty() {
                    for reference in class_references(&program) {
                        let known = catalog
                            .iter()
                            .any(|class| class.eq_ignore_ascii_case(&reference.name));
                        if !known {
                            issues.push(format!(
                                "agent {} {}:{}: unknown class {}",
                                self.name, reference.line, reference.column, reference.name
                            ));
                        }
                    }
                }
            }
            Err(diagnostics) => {
                for diagnostic in diagnostics {
                    issues.push(format!("agent {} {diagnostic}", self.name));
                }
            }
        }
        issues
    }
}

/// A compiling starter script for the given doctrine.
pub fn starter_script(doctrine: &str) -> &'static str {
    match doctrine {
        "Aggressive" => AGGRESSIVE,
        "Defensive" => DEFENSIVE,
        "Economy" => ECONOMY,
        "Advisor" => ADVISOR,
        _ => BALANCED,
    }
}

const BALANCED: &str = r#"# Balanced: steady economy, defend, then push.
on start {
    set_stance("defensive")
}

every 120 {
    if credits() < 800 {
        harvest()
    }
    if units("InfantryE1") < 4 {
        train("InfantryE1")
    }
    if credits() > 2000 {
        build("Refinery")
    }
    if tick() > 900 {
        attack(nearest_enemy_x(), nearest_enemy_y())
    }
}
"#;

const AGGRESSIVE: &str = r#"# Aggressive: keep a strike group moving at the enemy base.
on start {
    set_stance("aggressive")
}

every 120 {
    if units("InfantryE1") < 4 {
        train("InfantryE1")
    }
    if units("VehicleE1") < 3 {
        train("VehicleE1")
    }
    if tick() > 600 {
        attack(nearest_enemy_x(), nearest_enemy_y())
    }
}
"#;

const DEFENSIVE: &str = r#"# Defensive: hold the base and answer the biggest threat.
on start {
    set_stance("defensive")
}

every 90 {
    if threat() > 0 {
        defend(nearest_threat_x(), nearest_threat_y())
    }
    if units("InfantryE1") < 6 {
        train("InfantryE1")
    }
}
"#;

const ECONOMY: &str = r#"# Economy: harvest first, expand when rich, defend only when pressed.
every 120 {
    harvest()
    if credits() > 1500 {
        build("Refinery")
    }
    if buildings("Refinery") >= 2 and units("InfantryE1") < 3 {
        train("InfantryE1")
    }
    if threat() > 2 {
        defend(nearest_threat_x(), nearest_threat_y())
    }
}
"#;

const ADVISOR: &str = r#"# Advisor: an LLM proposes a mode out of band; the script stays deterministic.
# mode 0 = expand, 1 = attack, 2 = defend
every 300 {
    let mode = intent("mode", 0)
    if mode == 1 {
        attack(nearest_enemy_x(), nearest_enemy_y())
    } else if mode == 2 {
        defend(nearest_threat_x(), nearest_threat_y())
    } else {
        harvest()
        if credits() > 2000 {
            build("Refinery")
        }
    }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_starter_compiles() {
        for doctrine in DOCTRINES {
            compile(starter_script(doctrine))
                .unwrap_or_else(|errors| panic!("{doctrine} failed: {errors:?}"));
        }
    }

    #[test]
    fn validation_reports_script_errors_and_unknown_classes() {
        let mut agent = AgentDefinition::new(1, "Fist");
        agent.owner = "HouseNod".to_string();
        agent.script = "on start { train(\"Nope\") }".to_string();
        let issues = agent.validate(&["InfantryE1".to_string()]);
        assert!(issues
            .iter()
            .any(|issue| issue.contains("unknown class Nope")));

        agent.script = "on start { train( }".to_string();
        let issues = agent.validate(&[]);
        assert!(issues.iter().any(|issue| issue.contains("agent Fist")));
    }
}
