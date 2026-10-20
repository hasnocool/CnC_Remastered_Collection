//! Speech pipeline tests: full EVA/VOX coverage, deterministic queueing and
//! mixer routing.

use cnc_audio::eva::{EvaLine, TdEvent, TdFaction};
use cnc_audio::eva_ra::{
    RaDirection, RaEvaLine, RaEvent, RaUpgrade, RA_SILENT_SLOTS, RA_VOX_COUNT,
};
use cnc_audio::mixer::{AudioMixer, Channel};
use cnc_audio::speech::{
    EvaVocabulary, RaSpeechDirector, SpeakOutcome, SpeechCategory, SpeechQueue, TdSpeechDirector,
};
use cnc_audio::SoundEffect;

// ---------------------------------------------------------------- coverage

#[test]
fn every_td_line_maps_to_a_voice_id_and_sample() {
    let mut ids = Vec::new();
    for line in EvaLine::all() {
        let file = line.voice_file();
        assert!(!file.is_empty(), "{line:?} has an empty sample file");
        assert!(line.sample_name().ends_with(".AUD"));
        let id = line.voice_id();
        assert_eq!(EvaLine::from_voice_id(id), Some(line));
        assert_eq!(EvaLine::from_voice_file(file), Some(line));
        ids.push(id);
    }
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), EvaLine::COUNT, "voice ids must be unique");
    assert_eq!(EvaLine::COUNT, 46);
}

#[test]
fn every_ra_line_maps_to_a_voice_id_and_sample() {
    let mut ids = Vec::new();
    for line in RaEvaLine::all() {
        let file = line.voice_file();
        assert!(!file.is_empty(), "{line:?} has an empty sample file");
        let id = line.voice_id();
        assert_eq!(RaEvaLine::from_voice_id(id), Some(line));
        // Two engine slots share the same recording ("insufficient funds" and
        // "need more funds" both play NOFUNDS1), so the name lookup is only
        // required to land on a line backed by that file.
        let found = RaEvaLine::from_voice_file(file).expect("file maps back to a line");
        assert_eq!(found.voice_file(), file);
        ids.push(id);
    }
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), RaEvaLine::COUNT, "voice ids must be unique");
    assert_eq!(RaEvaLine::COUNT, 109);
}

#[test]
fn ra_engine_table_shares_the_insufficient_funds_sample() {
    // Verbatim from REDALERT/AUDIO.CPP: slots 9 and 15 are both "NOFUNDS1".
    assert_eq!(RaEvaLine::InsufficientFunds.voice_file(), "NOFUNDS1");
    assert_eq!(RaEvaLine::NeedMoreFunds.voice_file(), "NOFUNDS1");
}

#[test]
fn ra_silent_slots_complete_the_engine_table() {
    let mut all: Vec<u16> = RaEvaLine::all().map(RaEvaLine::voice_id).collect();
    all.extend(RA_SILENT_SLOTS.iter().map(|(id, _)| *id));
    all.sort_unstable();
    assert_eq!(all.len(), RA_VOX_COUNT as usize);
    assert_eq!(all, (0..RA_VOX_COUNT).collect::<Vec<_>>());
}

#[test]
fn lineages_have_a_speech_category_and_priority() {
    assert_eq!(
        EvaLine::MissionAccomplished.category(),
        SpeechCategory::Mission
    );
    assert_eq!(
        EvaLine::BaseUnderAttack.category(),
        SpeechCategory::BaseAttack
    );
    assert_eq!(EvaLine::LowPower.category(), SpeechCategory::Power);
    assert_eq!(
        RaEvaLine::ChronosphereReady.category(),
        SpeechCategory::Superweapon
    );
    assert!(
        SpeechCategory::BaseAttack.priority() > SpeechCategory::Production.priority(),
        "a base attack must outrank production chatter"
    );
    assert!(
        SpeechCategory::Mission.priority() <= u8::MAX,
        "priorities stay in the Westwood 0-255 space"
    );
}

// ---------------------------------------------------------------- events

#[test]
fn td_events_map_to_lines() {
    let cases = [
        (TdEvent::ConstructionComplete, EvaLine::ConstructionComplete),
        (TdEvent::UnitReady, EvaLine::UnitReady),
        (
            TdEvent::UnitDestroyed(TdFaction::Gdi),
            EvaLine::GdiUnitDestroyed,
        ),
        (
            TdEvent::UnitDestroyed(TdFaction::Nod),
            EvaLine::NodUnitDestroyed,
        ),
        (
            TdEvent::UnitDestroyed(TdFaction::Civilian),
            EvaLine::CivilianKilled,
        ),
        (
            TdEvent::BuildingCaptured(TdFaction::Gdi),
            EvaLine::GdiBuildingCaptured,
        ),
        (
            TdEvent::StructureDestroyed(TdFaction::Nod),
            EvaLine::NodStructureDestroyed,
        ),
        (
            TdEvent::StructureDestroyed(TdFaction::Civilian),
            EvaLine::EnemyStructureDestroyed,
        ),
        (
            TdEvent::IncomingNuclearWeapon,
            EvaLine::IncomingNuclearWeapon,
        ),
        (TdEvent::SilosNeeded, EvaLine::SilosNeeded),
    ];
    for (event, line) in cases {
        assert_eq!(EvaLine::from_event(event), Some(line));
    }
    assert_eq!(
        EvaLine::from_event(TdEvent::BuildingCaptured(TdFaction::Civilian)),
        None
    );
}

#[test]
fn ra_events_map_to_lines() {
    let cases = [
        (RaEvent::UnitReady, RaEvaLine::UnitReady),
        (RaEvent::MadTankDeployed, RaEvaLine::MadTankDeployed),
        (RaEvent::ChronosphereReady, RaEvaLine::ChronosphereReady),
        (RaEvent::IronCurtainCharging, RaEvaLine::IronCurtainCharging),
        (
            RaEvent::AlliedForcesArrivedFrom(RaDirection::North),
            RaEvaLine::AlliedForcesNorth,
        ),
        (RaEvent::Signal(RaDirection::West), RaEvaLine::SignalWest),
        (RaEvent::ObjectiveMet(3), RaEvaLine::Objective3Met),
        (RaEvent::TimeWarning(10), RaEvaLine::Time10),
        (RaEvent::Upgrade(RaUpgrade::Armor), RaEvaLine::UpgradeArmor),
        (RaEvent::BuildingCaptured, RaEvaLine::MissionAccomplished),
    ];
    for (event, line) in cases.iter().take(9) {
        assert_eq!(RaEvaLine::from_event(*event), Some(*line));
    }
    // The captured-building announcement is one of the game's silent slots.
    assert_eq!(RaEvaLine::from_event(RaEvent::BuildingCaptured), None);
    // Values the table has no sample for are silent, not a panic.
    assert_eq!(RaEvaLine::from_event(RaEvent::TimeWarning(7)), None);
    assert_eq!(RaEvaLine::from_event(RaEvent::ObjectiveMet(0)), None);
    assert_eq!(RaEvaLine::from_event(RaEvent::ObjectiveMet(4)), None);
}

// ---------------------------------------------------------------- queue

#[test]
fn speak_enqueues_eva_line() {
    let mut td = TdSpeechDirector::new();
    assert_eq!(
        td.event(TdEvent::ConstructionComplete),
        Some(SpeakOutcome::Queued)
    );
    let playback = td.advance(false).expect("line starts");
    assert_eq!(playback.line, EvaLine::ConstructionComplete);
    assert_eq!(playback.sample, "CONSTRU1");
    assert_eq!(playback.voice_id, 3);
    assert_eq!(playback.channel(), Channel::Voice);
}

#[test]
fn three_queued_lines_play_in_arrival_order() {
    let mut queue = SpeechQueue::<EvaLine>::new();
    for line in [
        EvaLine::UnitReady,
        EvaLine::ConstructionComplete,
        EvaLine::NewConstructionOptions,
    ] {
        assert_eq!(queue.speak(line), SpeakOutcome::Queued);
    }
    // All three are production lines with the same priority, so arrival
    // order decides, deterministically.
    let first = queue.advance(false).expect("first line");
    assert_eq!(first.line, EvaLine::UnitReady);
    let second = queue.advance(true).expect("second line");
    assert_eq!(second.line, EvaLine::ConstructionComplete);
    let third = queue.advance(true).expect("third line");
    assert_eq!(third.line, EvaLine::NewConstructionOptions);
    assert!(queue.advance(true).is_none());
}

#[test]
fn higher_priority_preempts_the_playing_line() {
    let mut queue = SpeechQueue::<EvaLine>::new();
    queue.speak(EvaLine::UnitReady);
    let production = queue.advance(false).expect("unit ready starts");
    assert_eq!(production.priority, SpeechCategory::Production.priority());

    queue.speak(EvaLine::BaseUnderAttack);
    let urgent = queue.advance(false).expect("base attack preempts");
    assert_eq!(urgent.line, EvaLine::BaseUnderAttack);
    assert!(
        urgent.priority > production.priority,
        "preemption must strictly outrank the playing line"
    );
    assert_eq!(
        queue.history().iter().copied().collect::<Vec<_>>(),
        vec![EvaLine::UnitReady, EvaLine::BaseUnderAttack]
    );
}

#[test]
fn same_priority_does_not_preempt() {
    let mut queue = SpeechQueue::<EvaLine>::new();
    queue.speak(EvaLine::UnitReady);
    queue.advance(false);
    queue.speak(EvaLine::ConstructionComplete);
    assert!(
        queue.advance(false).is_none(),
        "equal priority waits for the current line"
    );
    assert_eq!(
        queue.advance(true).map(|playback| playback.line),
        Some(EvaLine::ConstructionComplete)
    );
}

#[test]
fn duplicate_lines_are_dropped() {
    let mut queue = SpeechQueue::<EvaLine>::new();
    assert_eq!(queue.speak(EvaLine::UnitReady), SpeakOutcome::Queued);
    assert_eq!(queue.speak(EvaLine::UnitReady), SpeakOutcome::Duplicate);
    queue.advance(false);
    assert_eq!(queue.speak(EvaLine::UnitReady), SpeakOutcome::Duplicate);
    assert_eq!(queue.pending_len(), 0);
}

#[test]
fn bounded_queue_keeps_the_most_important_lines() {
    let mut queue = SpeechQueue::<EvaLine>::with_capacity(2);
    assert_eq!(queue.speak(EvaLine::UnitReady), SpeakOutcome::Queued);
    assert_eq!(
        queue.speak(EvaLine::ConstructionComplete),
        SpeakOutcome::Queued
    );
    // The base attack evicts the newest lowest-priority waiting line.
    assert_eq!(
        queue.speak(EvaLine::BaseUnderAttack),
        SpeakOutcome::Replaced
    );
    let lines: Vec<_> = queue.pending().collect();
    assert_eq!(lines, vec![EvaLine::BaseUnderAttack, EvaLine::UnitReady]);
    // A low-priority line that cannot beat either entry is simply dropped.
    assert_eq!(queue.speak(EvaLine::Canceled), SpeakOutcome::Dropped);
    assert_eq!(queue.pending_len(), 2);
}

#[test]
fn stop_clears_current_and_pending() {
    let mut queue = SpeechQueue::<RaEvaLine>::new();
    queue.speak(RaEvaLine::UnitReady);
    queue.speak(RaEvaLine::ConstructionComplete);
    queue.advance(false);
    queue.stop();
    assert!(queue.current().is_none());
    assert_eq!(queue.pending_len(), 0);
    assert!(!queue.is_busy());
}

// ---------------------------------------------------------- silent samples

/// A vocabulary entry with no sample file, mirroring a `"none"` VOX slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TestLine {
    Silent,
    Audible,
}

impl EvaVocabulary for TestLine {
    fn voice_id(self) -> u16 {
        match self {
            Self::Silent => 0,
            Self::Audible => 1,
        }
    }

    fn sample_file(self) -> Option<&'static str> {
        match self {
            Self::Silent => None,
            Self::Audible => Some("UNITREDY"),
        }
    }

    fn category(self) -> SpeechCategory {
        SpeechCategory::Info
    }
}

#[test]
fn missing_sample_is_silently_skipped() {
    let mut queue = SpeechQueue::<TestLine>::new();
    assert_eq!(queue.speak(TestLine::Silent), SpeakOutcome::Queued);
    assert_eq!(queue.speak(TestLine::Audible), SpeakOutcome::Queued);
    let playback = queue.advance(false).expect("audible line still starts");
    assert_eq!(playback.line, TestLine::Audible);
    assert_eq!(
        queue.skipped().iter().copied().collect::<Vec<_>>(),
        vec![TestLine::Silent]
    );

    // A silent-only queue advances cleanly to "nothing to play".
    let mut silent_only = SpeechQueue::<TestLine>::new();
    silent_only.speak(TestLine::Silent);
    assert!(silent_only.advance(false).is_none());
    assert_eq!(
        silent_only.skipped().iter().copied().collect::<Vec<_>>(),
        vec![TestLine::Silent]
    );
}

#[test]
fn silent_ra_event_is_a_no_op() {
    let mut ra = RaSpeechDirector::new();
    assert_eq!(ra.event(RaEvent::BuildingCaptured), None);
    assert!(ra.advance(false).is_none());
    assert!(ra.queue().history().is_empty());
}

// ------------------------------------------------------------- mixer route

#[test]
fn playback_plays_on_the_voice_channel_through_the_mixer() {
    let mixer = AudioMixer::headless();
    let sample = SoundEffect::from_bytes(test_wav());
    let mut ra = RaSpeechDirector::new();
    ra.speak(RaEvaLine::BaseUnderAttack);
    let playback = ra.advance(false).expect("line starts");
    let handle = playback.play(&mixer, &sample).expect("mixer accepts voice");
    let plays = mixer.played();
    assert_eq!(plays.len(), 1);
    assert_eq!(plays[0].channel, Channel::Voice);
    assert_eq!(plays[0].handle, handle);
    assert_eq!(plays[0].priority, SpeechCategory::BaseAttack.priority());
}

/// Minimal valid 16-bit mono PCM WAV so a real decoder can run headless.
fn test_wav() -> Vec<u8> {
    use std::io::Write;

    let samples = 400usize;
    let rate = 8_000u32;
    let data_len = (samples * 2) as u32;
    let mut out = Vec::with_capacity(data_len as usize + 44);
    out.extend_from_slice(b"RIFF");
    out.write_all(&(36 + data_len).to_le_bytes()).unwrap();
    out.extend_from_slice(b"WAVEfmt ");
    out.write_all(&16u32.to_le_bytes()).unwrap();
    out.write_all(&1u16.to_le_bytes()).unwrap();
    out.write_all(&1u16.to_le_bytes()).unwrap();
    out.write_all(&rate.to_le_bytes()).unwrap();
    out.write_all(&(rate * 2).to_le_bytes()).unwrap();
    out.write_all(&2u16.to_le_bytes()).unwrap();
    out.write_all(&16u16.to_le_bytes()).unwrap();
    out.extend_from_slice(b"data");
    out.write_all(&data_len.to_le_bytes()).unwrap();
    for i in 0..samples {
        let t = i as f32 / rate as f32;
        let amp = (2.0 * std::f32::consts::PI * 1_000.0 * t).sin();
        let sample = (amp * 0.5 * i16::MAX as f32).round() as i16;
        out.write_all(&sample.to_le_bytes()).unwrap();
    }
    out
}
