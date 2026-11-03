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
- `src/lib.rs` — config file handling (`check_settings`, `create_settings_file`, `read_settings`) and `update_pkg`, which executes a command.

Config lives at `~/.config/lazy-up/pkgs.yml`, a flat `name: command` map (read via the `config` crate with `preserve_order`). On first run a default (`rust: rust up`) is written and the program exits asking the user to edit it.

Gotchas:
- `run_command` splits on whitespace (no shell, so no quoting/pipes); a blank command returns `InvalidInput`. Non-zero exit statuses of the update command are ignored.
- `HOME` is required (`expect`).
