# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.7.1] - 2026-10-06

### Changed

- Refreshed every dependency requirement to the latest stable release,
  including `blake3` 1.8.6, `tokio` 1.53.1, `futures` 0.3.33, and `criterion`
  0.8.2 for benchmarks.
- Pinned the development toolchain to Rust 1.97.1 with `rust-toolchain.toml`.
- Releases are published to crates.io with trusted publishing (no stored
  registry token) after the crate builds and tests on Linux (x86_64 glibc and
  musl, aarch64), macOS (Apple silicon and Intel), and Windows; the GitHub
  Release carries the published crate, its checksum, and a CycloneDX SBOM.
- Refreshed the committed `Cargo.lock` to `blake3` 1.8.7 (which no longer
  depends on `arrayref`), `tokio` 1.53.2, `thiserror` 2.0.21, and `futures`
  0.3.34.

### Added

- `CONTRIBUTING.md`, `SECURITY.md`, and `CODE_OF_CONDUCT.md`, matching the
  documentation set used across ThreatFlux crates.
- Build, crate, documentation, license, and MSRV badges in the README.

### Fixed

- The README declared `## Features` twice; the cargo feature list is now
  `## Cargo features`.
- Corrected the re-export comment in `src/lib.rs`, which described `futures`
  while re-exporting `tokio`.

## [1.7.0] - 2026-08-05

### Changed

- Declared the stable 1.7 release line for the hashing API.
- Upgraded `md-5` and `sha2` from 0.10 to 0.11, aligning on RustCrypto
  `digest` 0.11.
- Replaced digest output formatting with an internal lowercase hexadecimal
  encoder for compatibility with the RustCrypto 0.11 output types.
- Updated dependency examples to use version 1.7.0.

### Compatibility

- MD5, SHA-256, and SHA-512 output values remain unchanged.

[Unreleased]: https://github.com/ThreatFlux/threatflux-hashing/compare/v1.7.1...HEAD
[1.7.1]: https://github.com/ThreatFlux/threatflux-hashing/compare/v1.7.0...v1.7.1
[1.7.0]: https://github.com/ThreatFlux/threatflux-hashing/releases/tag/v1.7.0
