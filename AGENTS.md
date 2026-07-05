# AGENTS.md

## Project

Kanri is a Rust CLI for managing local project directories. It uses `clap`, `serde`, TOML/JSON config, Lua-powered blueprints, and platform helpers for paths, shells, and editors.

Current user-facing system:

- Projects are created, cloned, listed, opened, renamed, and removed from a configured workspace.
- Blueprints are Lua scripts used to initialize new projects.
- The old templates system is deprecated and remains only for migration via `kanri blueprints migrate-templates`.
- Backups include config and blueprints.

## Key Files

- `src/main.rs` — entry point, environment bootstrap, command dispatch.
- `src/cli.rs` and `src/cli/` — CLI commands, flags, aliases, help text.
- `src/commands/` — command handlers.
- `src/config.rs` — config schema, defaults, load/save.
- `src/migrations.rs` — config migrations.
- `src/library.rs` — project directory operations.
- `src/blueprints/` — Lua blueprint engine, storage, and Lua modules.
- `src/templates.rs` — legacy templates storage used for migration only.
- `src/backup.rs` — backup/import data model and persistence.
- `src/platform.rs` — OS-specific behavior.
- `src/program.rs` — external program launching.
- `src/terminal.rs` — terminal output, prompts, progress UI.
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

Build and package checks:

```shell
cargo build
cargo build --release
cargo package --no-verify
```

## Rules

- Keep Rust idiomatic and `rustfmt` clean.
- Return `Result`; do not panic for normal user/filesystem errors.
- Use explicit domain errors with `thiserror`; use `anyhow` near app boundaries.
- Preserve cross-platform behavior, especially `platform.rs`, shell/editor logic, process launching, and paths.
- Keep CLI changes synced with README/docs/help text.
- Preserve config compatibility: structs use `serde(default, deny_unknown_fields)`.
- Keep config migrations safe and idempotent.
- Avoid new dependencies unless clearly needed.
- Treat templates as deprecated legacy migration code; do not add new template features.

## Tests and Docs

- Add tests in `src/tests/` for library/config/blueprint/backup behavior changes.
- Use `TestContext` and temp dirs; never touch real user config in tests.
- Update `README.md`, `docs/`, and `CHANGELOG.md` for user-visible changes.
- Keep blueprint Lua API docs in `docs/BLUEPRINTS.md` synced with implementation.
- Keep profile/config docs synced with config schema and migrations.

## Safety

Be careful with destructive paths: `remove`, `rename`, config reset, blueprint remove/edit, template migration, backup import, and Lua blueprint filesystem operations. Preserve `NO_COLOR` behavior and terminal output style.
