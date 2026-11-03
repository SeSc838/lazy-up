# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

- Build: `cargo build`
- Run: `cargo run -- [-p <pkg>]` (default `--pkg all`)
- Lint/format: `cargo clippy`, `cargo fmt`
- Tests: `cargo test` (none exist yet); single test: `cargo test <name>`

## Architecture

`lazy-up` is a small Rust CLI (edition 2024) that runs user-defined package-update commands. It is a lib + bin crate:

- `src/main.rs` — clap arg parsing (`--pkg`), first-run flow, and dispatch (all packages, or one by name).
- `src/lib.rs` — config file handling (`check_settings`, `create_settings_file`, `read_settings`, `is_supported_os`) and `update_pkg`, which executes a command.

Config lives at `~/.config/lazy-up/pkgs.yml`, a flat `name: command` map (parsed with `serde_yaml_ng` into an order-preserving `IndexMap`, so keys may contain dots). On first run a default (`rust: rustup update`) is written and the program exits non-zero asking the user to edit it. The program exits with a hint on anything but macOS/Linux.

Gotchas:
- `run_command` splits on whitespace (no shell, so no quoting/pipes); a blank command returns `InvalidInput`. A spawn failure or non-zero exit of an update command is reported with the package name; `--pkg all` continues with the remaining packages and exits 1 if any failed.
- `HOME` is required (`expect`).
