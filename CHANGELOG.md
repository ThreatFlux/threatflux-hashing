# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/ThreatFlux/threatflux-hashing/compare/v1.7.0...HEAD
[1.7.0]: https://github.com/ThreatFlux/threatflux-hashing/releases/tag/v1.7.0
