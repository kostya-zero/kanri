# AGENTS.md

## Project

Kanri is Rust CLI for managing local project directories. It uses `clap`, `serde`, TOML/JSON config, and platform helpers for paths, shells, and editors.

## Key Files

- `src/main.rs` — entry point and command dispatch.
- `src/cli.rs` — CLI commands, flags, aliases, help text.
- `src/commands/` — command handlers.
- `src/config.rs` — config schema, defaults, load/save.
- `src/templates.rs` — templates schema and persistence.
- `src/library.rs` — project directory operations.
- `src/platform.rs` — OS-specific behavior.
- `src/tests/` — unit tests and helpers.
- `docs/` — user docs.

## Commands

Run before handoff:

```shell
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

CI also uses:

```shell
cargo nextest run
```

Build:

```shell
cargo build
cargo build --release
```

## Rules

- Keep Rust idiomatic and `rustfmt` clean.
- Return `Result`; do not panic for normal user/filesystem errors.
- Use explicit domain errors with `thiserror`; use `anyhow` near app boundaries.
- Preserve cross-platform behavior, especially `platform.rs`, shell/editor logic, and paths.
- Keep CLI changes synced with README/docs.
- Preserve config compatibility: structs use `serde(default, deny_unknown_fields)`.
- Avoid new dependencies unless clearly needed.

## Tests and Docs

- Add tests in `src/tests/` for library/config/template behavior changes.
- Use `TestContext` and temp dirs; never touch real user config in tests.
- Update `README.md`, `docs/`, and `CHANGELOG.md` for user-visible changes.

## Safety

Be careful with destructive paths: `remove`, `rename`, config reset, template clear, backup import. Preserve `NO_COLOR` behavior and terminal output style.
