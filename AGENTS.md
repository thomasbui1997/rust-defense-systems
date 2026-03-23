# Repository Guidelines

## Project Structure & Module Organization
This repository mixes planning documents with an active Rust exercise project.

- `README.md` explains the overall learning program and current phase.
- `docs/` holds roadmap, sprint plans, issue packs, and AI usage guidance.
- `phase-1-taclane-log-ingestion-cli/README.md` describes the Phase 1 exercise.
- `phase-1-taclane-log-ingestion-cli/taclane_log_ingestion_cli/` is the Rust crate.
- `src/main.rs` is the current application entry point.
- `sample_logs.log` provides example TACLANE-style input data.
- `target/` is build output; do not edit it manually.

Keep new Rust modules under the crate’s `src/` directory. Add reusable parsing logic in separate modules rather than growing `main.rs` indefinitely.

## Build, Test, and Development Commands
Run commands from `phase-1-taclane-log-ingestion-cli/taclane_log_ingestion_cli/` unless noted otherwise.

- `cargo build` compiles the CLI in debug mode.
- `cargo run` builds and runs the current binary locally.
- `cargo test` runs unit and integration tests.
- `cargo fmt` applies standard Rust formatting.
- `cargo clippy -- -D warnings` catches style and correctness issues early.

Use `cargo run -- ../sample_logs.log` once CLI argument handling is added.

## Coding Style & Naming Conventions
Use Rust defaults: 4-space indentation, `snake_case` for functions and modules, `PascalCase` for types, and `SCREAMING_SNAKE_CASE` for constants. Favor small functions, explicit error handling, and `Result`-based flows over `panic!` for expected failures. Format with `cargo fmt` before submitting changes.

## Testing Guidelines
There are no committed tests yet, but new parser or filtering logic should ship with tests. Put unit tests beside the code with `#[cfg(test)]`, and add integration tests under `tests/` when behavior spans the CLI boundary. Name tests for the behavior they verify, for example `parses_warn_line` or `rejects_malformed_timestamp`.

## Commit & Pull Request Guidelines
Recent history uses short, imperative commit messages such as `Built project` and `Add sprint 01 issue pack`. Follow that pattern: start with a verb, keep the subject concise, and scope each commit to one logical change.

PRs should include a brief summary, note affected paths, list verification commands run, and attach sample input/output when CLI behavior changes. Link the related sprint ticket or issue when one exists.
