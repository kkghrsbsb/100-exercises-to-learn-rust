# Repository Guidelines

## Project Structure & Module Organization

- `exercises/<chapter>/<exercise>/` holds the Rust crates for each exercise, typically with `src/lib.rs` and a per-exercise `Cargo.toml`.
- `book/` is the mdBook project. Source chapters live in `book/src`, assets in `book/assets`, and config in `book/book.toml`.
- `helpers/` contains Rust tooling used by the book build (e.g., mdBook plugins in `helpers/mdbook-*`).
- `site/` stores deployment artifacts like `_redirects` generated from `book/link2alias.json`.
- Workspace settings are in the root `Cargo.toml`; build outputs land in `target/`.

## Build, Test, and Development Commands

- `cargo build` builds all workspace crates (exercises + helpers).
- `cargo test` runs unit and integration tests across the workspace.
- `cargo check` is a faster compile-only sanity check while iterating.
- `cd book && mdbook build` builds the book after installing mdBook and the helper plugins.
- `./helpers/json2redirects.sh book/link2alias.json > site/_redirects` regenerates redirects (used by CI).

## Coding Style & Naming Conventions

- Rust code follows standard Rust conventions; keep module/file names snake_case.
- Use `dprint` for Markdown and TOML formatting (see `dprint.json`).
- Exercise crates are named to match their folder (e.g., `exercises/04_traits/04_derive`).

## Testing Guidelines

- Tests live in `tests/` for integration tests and alongside code in `src/` for unit tests.
- Use `cargo test -p <crate>` to focus on a single exercise when iterating.
- Add or update tests when changing exercise behavior or shared helpers.

## Commit & Pull Request Guidelines

- Recent history uses concise, imperative messages like `feat: complete ...` or `Fix typo ...`; follow that tone.
- Include the exercise identifiers in the commit summary when relevant (e.g., `04_traits_04`).
- PRs should include a short summary, testing notes, and any required links (issues or exercise references). Screenshots are only needed for book rendering changes.
