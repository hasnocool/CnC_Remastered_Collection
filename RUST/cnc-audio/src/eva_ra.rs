//! Red Alert EVA speech vocabulary.
//!
//! Red Alert has its own `VoxType` enum and its own `Speech[]` file table,
//! distinct from Tiberian Dawn's ([crate::eva]). Values here are transcribed
//! from `REDALERT/DEFINES.H` (`VOX_*`) and `REDALERT/AUDIO.CPP` (`Speech[]`).
//!
//! Entries the original maps to the literal string `"none"` have no sample
//! and are omitted.

use crate::eva::EvaQueue;

/// One Red Alert EVA speech event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RaEvaLine {
    MissionAccomplished,
    MissionFailed,
    UnableToComply,
    ConstructionComplete,
    UnitReady,
    NewConstructionOptions,
    CannotDeployHere,
    StructureDestroyed,
    InsufficientPower,
    InsufficientFunds,
    BattleControlTerminated,
    ReinforcementsArrived,
    Canceled,
    Building,
    LowPower,
    BaseUnderAttack,
    UnableToBuildMore,
    PrimaryBuildingSelected,
    MadTankDeployed,
    UnitLost,
    SelectTarget,
    EnemyApproaching,
    SilosNeeded,
    OnHold,
    Repairing,
    AircraftLost,
    AlliedForcesApproaching,
    AlliedReinforcementsArrived,
    BuildingInfiltrated,
    ChronosphereCharging,
    ChronosphereReady,
    CommandCenterUnderAttack,
    ControlCenterDeactivated,
    ConvoyApproaching,
    ConvoyUnitLost,
    ExplosivePlaced,
    MoneyStolen,
    ShipLost,
    SatelliteLaunched,
    SonarAvailable,
    SovietForcesApproaching,
    SovietReinforcementsArrived,
    Training,
    AtomicBombReady,
    AtomicBombLaunched,
    EnemyStructureDestroyed,
}

/// `Speech[]`: base file name for each line (`.AUD` extension), in the
/// original table order.
const RA_SPEECH_FILES: &[(&str, RaEvaLine)] = &[
    ("MISNWON1", RaEvaLine::MissionAccomplished),
    ("MISNLST1", RaEvaLine::MissionFailed),
    ("PROGRES1", RaEvaLine::UnableToComply),
    ("CONSCMP1", RaEvaLine::ConstructionComplete),
    ("UNITRDY1", RaEvaLine::UnitReady),
    ("NEWOPT1", RaEvaLine::NewConstructionOptions),
    ("NODEPLY1", RaEvaLine::CannotDeployHere),
    ("STRCKIL1", RaEvaLine::StructureDestroyed),
    ("NOPOWR1", RaEvaLine::InsufficientPower),
    ("NOFUNDS1", RaEvaLine::InsufficientFunds),
    ("BCT1", RaEvaLine::BattleControlTerminated),
    ("REINFOR1", RaEvaLine::ReinforcementsArrived),
    ("CANCLD1", RaEvaLine::Canceled),
    ("ABLDGIN1", RaEvaLine::Building),
    ("LOPOWER1", RaEvaLine::LowPower),
    ("BASEATK1", RaEvaLine::BaseUnderAttack),
    ("NOBUILD1", RaEvaLine::UnableToBuildMore),
    ("PRIBLDG1", RaEvaLine::PrimaryBuildingSelected),
    ("TANK01", RaEvaLine::MadTankDeployed),
    ("UNITLST1", RaEvaLine::UnitLost),
    ("SLCTTGT1", RaEvaLine::SelectTarget),
    ("ENMYAPP1", RaEvaLine::EnemyApproaching),
    ("SILOND1", RaEvaLine::SilosNeeded),
    ("ONHOLD1", RaEvaLine::OnHold),
    ("REPAIR1", RaEvaLine::Repairing),
    ("AUNITL1", RaEvaLine::AircraftLost),
    ("AAPPRO1", RaEvaLine::AlliedForcesApproaching),
    ("AARRIVE1", RaEvaLine::AlliedReinforcementsArrived),
    ("BLDGINF1", RaEvaLine::BuildingInfiltrated),
    ("CHROCHR1", RaEvaLine::ChronosphereCharging),
    ("CHRORDY1", RaEvaLine::ChronosphereReady),
    ("CMDCNTR1", RaEvaLine::CommandCenterUnderAttack),
    ("CNTLDED1", RaEvaLine::ControlCenterDeactivated),
    ("CONVYAP1", RaEvaLine::ConvoyApproaching),
    ("CONVLST1", RaEvaLine::ConvoyUnitLost),
    ("XPLOPLC1", RaEvaLine::ExplosivePlaced),
    ("CREDIT1", RaEvaLine::MoneyStolen),
    ("NAVYLST1", RaEvaLine::ShipLost),
    ("SATLNCH1", RaEvaLine::SatelliteLaunched),
    ("PULSE1", RaEvaLine::SonarAvailable),
    ("SOVFAPP1", RaEvaLine::SovietForcesApproaching),
    ("SOVREIN1", RaEvaLine::SovietReinforcementsArrived),
    ("TRAIN1", RaEvaLine::Training),
    ("AREADY1", RaEvaLine::AtomicBombReady),
    ("ALAUNCH1", RaEvaLine::AtomicBombLaunched),
    ("STRUSLD1", RaEvaLine::StructureDestroyed),
];

impl RaEvaLine {
    /// Base file name of the sample, without the `.AUD` extension.
    pub fn voice_file(self) -> &'static str {
        RA_SPEECH_FILES
            .iter()
            .find(|(_, line)| *line == self)
            .map(|(name, _)| *name)
            .expect("every RaEvaLine has a speech entry")
    }

    /// Full sample name, e.g. `MISNWON1.AUD`.
    pub fn sample_name(self) -> String {
        format!("{}.AUD", self.voice_file())
    }

    /// Every line with a sample, in table order.
    pub fn all() -> impl Iterator<Item = RaEvaLine> {
        RA_SPEECH_FILES.iter().map(|(_, line)| *line)
    }

    /// Look a line up by its `Speech[]` file name (case-insensitive).
    pub fn from_voice_file(name: &str) -> Option<Self> {
        RA_SPEECH_FILES
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, line)| *line)
    }
}

/// Red Alert's EVA queue.
///
/// Red Alert uses the same one-slot pending protocol as Tiberian Dawn, so
/// this is a thin wrapper that keeps the two vocabularies apart.
pub type RaEvaQueue = EvaQueue;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_ra_line_has_a_speech_file() {
        for line in RaEvaLine::all() {
            assert!(!line.voice_file().is_empty());
        }
    }

    #[test]
    fn ra_voice_names_match_the_original_table() {
        assert_eq!(RaEvaLine::MissionAccomplished.voice_file(), "MISNWON1");
        assert_eq!(RaEvaLine::UnitReady.voice_file(), "UNITRDY1");
        assert_eq!(RaEvaLine::BaseUnderAttack.voice_file(), "BASEATK1");
        assert_eq!(RaEvaLine::sample_name(RaEvaLine::LowPower), "LOPOWER1.AUD");
    }

    #[test]
    fn ra_and_td_use_different_samples_for_the_same_line() {
        // The two games ship different recordings; a mix-up here would be a
        // silent wrong-voice bug.
        assert_eq!(crate::eva::EvaLine::UnitReady.voice_file(), "UNITREDY");
        assert_eq!(RaEvaLine::UnitReady.voice_file(), "UNITRDY1");
    }

    #[test]
    fn ra_voice_file_lookup_is_case_insensitive() {
        assert_eq!(
            RaEvaLine::from_voice_file("misnwon1"),
            Some(RaEvaLine::MissionAccomplished)
        );
        assert_eq!(RaEvaLine::from_voice_file("nope"), None);
    }

    #[test]
    fn ra_queue_uses_the_shared_protocol() {
        let mut queue = RaEvaQueue::new();
        assert!(queue.speak(crate::eva::EvaLine::UnitReady));
        assert!(!queue.speak(crate::eva::EvaLine::Canceled));
        assert_eq!(queue.update(), Some(crate::eva::EvaLine::UnitReady));
    }
}
