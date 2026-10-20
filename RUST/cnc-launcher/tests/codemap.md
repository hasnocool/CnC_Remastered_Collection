# RUST/cnc-launcher/tests/

## Responsibility

One integration test file, `cli_campaign.rs` (38 lines): guarantees the RA campaign listing CLI stays window-free and complete.

## Design

Single test `list_campaign_reports_every_ra_mission()`: invokes the launcher's campaign listing path (per its module doc, "The RA campaign listing CLI must stay window-free and complete") and asserts every authored RA mission is reported.

## Flow

Run CLI in headless mode → parse listing output → compare against the expected RA campaign set; no GPU/window required, which is why the test runs on CI.

## Integration

Covers the headless listing branch of `cnc-launcher/src/main.rs` (campaign progress paths `default_campaign_progress_path`, `default_td_progress_path`); complements the 58 unit tests that live in `src/` (menu, sidebar, network configuration validation etc., per BUILD_STATUS).
