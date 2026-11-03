# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions CI running tests (Linux and macOS), `rustfmt` and `clippy` on every pull request.
- Release workflow: pushing a `v*` tag runs CI, builds binaries and publishes a GitHub release.
- Unit tests.

### Changed
- Config is parsed with `serde_yaml_ng` instead of the `config` crate, so package names may contain dots.
- The default config written on first run is `rust: rustup update` (was `rust up`).
- First run exits non-zero after writing the default config.
- Unsupported operating systems (anything but macOS/Linux) exit with a hint.

### Fixed
- Spawn failures and non-zero exits of an update command are now reported with the package name. `--pkg all` continues with the remaining packages and exits 1 if any failed.

## [0.2.0]

### Added
- `lazy-up` CLI that runs user-defined package-update commands from `~/.config/lazy-up/pkgs.yml`.
- `--pkg` option to update a single package (default: `all`).

[Unreleased]: https://github.com/SeSc838/lazy-up/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/SeSc838/lazy-up/releases/tag/v0.2.0
