# RUST/src/

## Responsibility

Not a real workspace crate — a leftover scaffold containing a 5-line `main.rs` stub (`fn main() { println!("Hello, world!"); }`) without a `[package]`/`[[bin]]` entry in the root `Cargo.toml`. It is not built by `cargo check --workspace` and not referenced by `BUILD_STATUS.md`.

## Design

The root `Cargo.toml` is a pure virtual workspace (`[workspace]` only, resolver 2), so `RUST/src/` is orphaned: no crate target maps to it. Real binary entry points are `cnc-launcher/src/main.rs`, `cnc-asset/src/cli.rs` and `cnc-net-directory/src/main.rs`.

## Flow

Dead code. Compiling it standalone would fail against no dependencies; nothing imports it.

## Integration

None. Candidates: fold it into `cnc-launcher` or delete. The `codemap.md` here exists only so the directory is documented.
