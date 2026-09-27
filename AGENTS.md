# Repository Guidelines

## Project Structure & Module Organization

Kanri is a Rust 2024 CLI for managing local project directories on Windows, Linux, and macOS. `src/main.rs` dispatches commands defined in `src/cli.rs`; handlers live in `src/commands/`. `src/library.rs` manages projects, while configuration, profiles, backups, migrations, and platform integration have dedicated modules. `src/blueprints/` contains the Lua engine, storage, and exposed API modules. Unit tests live in `src/tests/`. User documentation is in `docs/`; CI and release workflows are in `.github/workflows/`. Build output goes into `target/`.

## Build, Test, and Development Commands

Install stable Rust and a C/C++ compiler; see `docs/BUILDING.md` for platform prerequisites.

- `cargo run -- --help`: run the CLI locally and inspect commands.
- `cargo build`: compile a debug binary.
- `cargo build --release`: create an optimized binary in `target/release/`.
- `cargo fmt --all`: format Rust code; append `-- --check` to verify formatting.
- `cargo clippy --all-targets --all-features -- -D warnings`: run CI-equivalent lint checks. Prefer Clippy over `cargo check`.
- `cargo test`: run the test suite.
- `cargo nextest run`: run tests using the runner used in CI; requires cargo-nextest.

Skip builds and Clippy for small changes that do not affect logic.

## Coding Style & Naming Conventions

Follow rustfmt defaults with four-space indentation. Use `snake_case` for modules and functions, `PascalCase` for types, and `SCREAMING_SNAKE_CASE` for constants. Prefer `anyhow::Result` for application-level fallible operations; preserve existing typed domain errors. Avoid `unwrap()` unless failure is demonstrably impossible. Keep changes focused, follow existing module boundaries, and avoid speculative abstractions. Prefer PowerShell for scripts.

## Testing Guidelines

Tests use Rust's built-in `#[test]` framework and `tempfile` for filesystem isolation. Follow `src/tests/test_<module>.rs` and `test_<behavior>` naming; register new test modules in `src/tests/mod.rs`. Add tests only when requested, targeting observable behavior. Run focused tests with `cargo test test_library`. CI tests all three supported operating systems; no coverage threshold is configured.

## Commit & Pull Request Guidelines

Use the history's Conventional Commit style: `feat:`, `fix:`, `refactor:`, `docs:`, or `chore:`, optionally scoped, such as `feat(config): ...`. Keep subjects concise and imperative. PRs should explain the problem, resulting behavior, and validation performed; link relevant issues. Update `docs/` when CLI behavior or configuration changes, and note platform-specific effects.
