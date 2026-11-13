## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- `entity` module: shared `Object` foundation, `TaskForce`/`TeamType`/`Team` mission-teams and core-level `Trigger` (mirrors OBJECT.H / Team / Trigger families; plain serde data, no unsafe)

#### Fixed
- `CCWeak` use-after-free: `ControlBlock` now tracks strong and weak counts and `CCWeak::upgrade` returns `None` once the last strong reference dies
- Clippy deny-level lints: `ObjectPool::deallocate_drop` and pointer `decode_ref` marked `unsafe` with `# Safety` contracts; inherent `IniFile::to_string` renamed to `to_ini_string` with `Display` delegating to it
