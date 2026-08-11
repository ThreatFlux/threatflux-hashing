# Contributing

Thank you for improving ThreatFlux Hashing. Bug reports, documentation fixes,
tests, and focused implementation changes are welcome.

By participating, you agree to follow the
[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Report security issues privately as
described in [`SECURITY.md`](SECURITY.md).

## Before opening an issue

- Search existing issues and pull requests.
- Confirm the behavior on the latest release or `main`.
- Reduce bugs to a small reproducible example when possible.
- Include the Rust version, crate version, enabled features, algorithm, and
  platform.

Do not include secrets or sensitive file contents in a public report. A digest
of sensitive data is itself sensitive.

## Development workflow

1. Fork the repository and create a branch from `main`.
2. Make one focused change with tests and documentation.
3. Run the checks described in [`DEVELOPMENT.md`](DEVELOPMENT.md).
4. Review the diff for generated files, credentials, and unrelated edits.
5. Open a pull request explaining the problem, approach, compatibility impact,
   and validation performed.

Use clear commit messages written in the imperative mood. Commit subjects follow
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/), because
release automation derives the next version from them: `fix:` produces a patch,
`feat:` a minor, and a `!` suffix or `BREAKING CHANGE:` trailer a major release.

## Local checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo bench            # optional, benchmarks are not part of CI gating
```

## Pull-request checklist

- [ ] Public behavior and compatibility impact are documented.
- [ ] Tests cover new behavior and regressions.
- [ ] Default, no-default, and all-feature configurations pass.
- [ ] Digest values for existing algorithms are unchanged, or the change is
      called out as breaking.
- [ ] Formatting, Clippy, test, and dependency-policy checks pass.
- [ ] No generated build output or sensitive sample data is included.

## Release process

Releases are automated. Merging to `main` runs the ThreatFlux auto-release
workflow, which derives the version from the conventional commits since the last
tag, updates `Cargo.toml`, and publishes the tag and GitHub Release. Do not bump
the version by hand in a pull request.
