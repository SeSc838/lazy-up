# lazy-up

An updater for lazy people. Define your update commands once, then run them all with a single command.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
lazy-up            # run every configured update command
lazy-up --pkg rust # run only the entry named "rust"
```

On first run, `lazy-up` creates a default config at `~/.config/lazy-up/pkgs.yml` and exits so you can edit it. Re-run afterwards.

## Configuration

`~/.config/lazy-up/pkgs.yml` is a flat map of `name: command`. Entries run in file order.

```yaml
rust: rustup update
brew: brew upgrade
```

Commands are split on whitespace into a program and its arguments (e.g. `brew upgrade --quiet`). Shell features such as quoting, pipes and `&&` are not supported; wrap those in a script.

## Development

```sh
cargo build
cargo test
cargo clippy
```

## Releasing

Releases are created by CI when a `v*` tag is pushed (see `.github/workflows/release.yml`). The workflow first runs the full CI suite, then builds binaries and publishes a GitHub release with generated notes.

1. Move the `[Unreleased]` entries in `CHANGELOG.md` under a new version heading and update the compare links at the bottom.
2. Bump `version` in `Cargo.toml`, run `cargo build` so `Cargo.lock` is updated, and commit both on `main`.
3. Tag and push:

```sh
git tag v0.2.1
git push origin v0.2.1
```

The tag must match the version in `Cargo.toml`.
