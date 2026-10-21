# Stable dependency and security review

Reviewed 2026-10-21 from fetched `main` at `28ab175`. The complete paginated GitHub open-PR and code-scanning inventories each returned zero entries. No alerts were dismissed and there are no superseded dependency PRs to close. This inventory does not assert that unreported vulnerabilities cannot exist.

Rust build, CI, documentation and release pins move from 1.97.1 to stable 1.99.1. The consumer `rust-version = "1.95.0"` remains unchanged; the build toolchain and library MSRV serve different purposes. The official [stable channel manifest](https://static.rust-lang.org/dist/channel-rust-stable.toml) identifies Rust 1.99.1 (2026-10-15 release).

All direct and development dependencies were checked against the official [crates.io sparse index](https://index.crates.io/config.json), selecting the latest non-yanked stable version. This refreshes BLAKE3, SHA-2, Tokio, thiserror, futures and serde_json, normalizes the current MD5 requirement, and updates the lockfile's compatible transitive dependencies. CI tools cargo-llvm-cov and cargo-semver-checks move to 0.9.1 and 0.51.0. Existing GitHub Actions SHA pins match the current stable releases checked for this review. This repository is a library with no shipped Docker/native runtime base to migrate.

Validation results are recorded in the pull request after running the repository's pre-commit hook and the CI feature/MSRV/security checks. No real credentials or cloud requests are needed for these checks.

Local validation passed on Rust 1.99.1: `make pre-commit` (formatting, Clippy, tests); `cargo +1.95.0 check --locked --all-targets --all-features`; `cargo hack check --feature-powerset --depth 2 --locked`; `cargo audit --deny warnings`; `cargo deny check`; and `actionlint`. The unused BSD-2-Clause license allowance warning is existing policy noise, not an advisory or rejected license. The repository-provided pre-commit hook is installed and runs again for the commit.

The provided hook also needed a portability fix: on macOS, its empty-input `xargs` pipeline reported success and blocked dependency-only commits as if Rust TODO markers existed. The template now loops over NUL-delimited staged Rust paths and fails only for an actual marker match; deleted paths are excluded. The installed hook uses the corrected repository template.
