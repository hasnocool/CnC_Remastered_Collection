# RUST/cnc-core/tests/

## Responsibility
Integration tests for cnc-core trigger attachment system and visibility/shroud mechanics. Validates cross-module trigger logic and fog-of-war implementation.

## Design
- trigger_attach.rs (15,757 lines): Comprehensive tests for trigger attachment system - AttachTarget, AttachmentSpec, TriggerAttachment, TriggerAttachments, TriggerMode, TriggerSubject. Tests attachment firing, detachment, re-attachment, subject filtering, mode transitions.
- visibility.rs (12,497 lines): Tests for CellState, FogPolicy, SightSource, Visibility, shroud reveal/conceal logic. Validates fog-of-war updates, sight source priority, multi-house visibility snapshots.

## Flow
1. Tests construct trigger/visibility scenarios in memory
2. Execute trigger attachment logic or visibility updates
3. Assert state transitions, snapshot versions, event firing

## Integration
- Exercises cnc-core/src/trigger_attach.rs and cnc-core/src/trigger_apply.rs
- Exercises cnc-core/src/visibility.rs and cnc-core/src/shroud.rs
- Complements unit tests in src/ modules

## Key Files
- trigger_attach.rs - Trigger attachment integration tests
- visibility.rs - Visibility/shroud integration tests

