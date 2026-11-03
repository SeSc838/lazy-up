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
