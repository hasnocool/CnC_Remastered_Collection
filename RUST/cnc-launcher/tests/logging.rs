//! Structured logging bootstrap tests for the launcher.
//!
//! `launcher_boot_writes_structured_entry` drives the same `cnc-logs`
//! subscriber the launcher installs at boot and checks the in-memory buffer
//! receives structured entries. `log_buffer_bounded_under_flood` proves the
//! bounded-memory rotation contract: oldest entries drop first, no panic, and
//! the retained entry count never exceeds capacity.

use cnc_launcher::logging::{launcher_log_config, LOG_BUFFER_CAPACITY};
use cnc_logs::{global_log_buffer, LogBuffer, LogEntry, LogLevel};

#[test]
fn launcher_boot_writes_structured_entry() {
    let _guard = cnc_logs::init_logging_with_buffer(
        launcher_log_config(),
        LogBuffer::new(LOG_BUFFER_CAPACITY),
    )
    .expect("launcher logging bootstrap succeeds");

    // Mirrors the boot line in main.rs: target plus structured field.
    tracing::info!(
        target: "cnc_launcher",
        boot = true,
        "cnc-launcher structured logging initialized"
    );

    let buffer = global_log_buffer().expect("global log buffer installed at boot");
    assert_eq!(buffer.capacity(), LOG_BUFFER_CAPACITY);

    let entries = buffer.drain();
    let boot = entries
        .iter()
        .find(|entry| entry.message.contains("structured logging initialized"))
        .expect("boot entry reached the log buffer");

    assert_eq!(boot.level, LogLevel::Info);
    assert_eq!(boot.target, "cnc_launcher");
    assert!(
        boot.fields
            .as_ref()
            .and_then(|fields| fields.get("boot"))
            .is_some(),
        "structured field `boot` is retained: {:?}",
        boot.fields
    );
}

#[test]
fn log_buffer_bounded_under_flood() {
    const CAPACITY: usize = 256;

    // 1. Deterministic flood: rotation drops the OLDEST entries first.
    let buffer = LogBuffer::new(CAPACITY);
    let total = 10_000usize;
    for index in 0..total {
        buffer.push(LogEntry::new(
            LogLevel::Debug,
            "cnc_launcher::flood",
            &format!("flood {index}"),
            None,
        ));
    }

    assert_eq!(buffer.len(), CAPACITY, "buffer stays at capacity");
    assert_eq!(buffer.dropped_count(), total - CAPACITY);

    let retained = buffer.snapshot();
    assert_eq!(
        retained.first().expect("at least one entry").message,
        format!("flood {}", total - CAPACITY),
        "oldest retained entry is the first one that fit"
    );
    assert_eq!(
        retained.last().expect("at least one entry").message,
        format!("flood {}", total - 1),
        "newest entry is always retained"
    );

    // 2. Concurrent flood: no panic under lock contention, still bounded.
    const THREADS: usize = 8;
    const PER_THREAD: usize = 5_000;

    let contended = LogBuffer::new(CAPACITY);
    std::thread::scope(|scope| {
        for thread in 0..THREADS {
            let buffer = &contended;
            scope.spawn(move || {
                for index in 0..PER_THREAD {
                    buffer.push(LogEntry::new(
                        LogLevel::Trace,
                        "cnc_launcher::flood",
                        &format!("thread {thread} message {index}"),
                        None,
                    ));
                }
            });
        }
    });

    assert!(contended.len() <= CAPACITY);
    assert_eq!(contended.len(), CAPACITY);
    assert_eq!(
        contended.dropped_count(),
        THREADS * PER_THREAD - CAPACITY,
        "every entry beyond capacity is accounted for as dropped"
    );
    assert_eq!(contended.drain().len(), CAPACITY);
}
