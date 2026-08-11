# Security Policy

## Supported versions

Security fixes are provided for the latest published minor release. Users should
upgrade to the newest patch before reporting a problem.

| Version              | Supported |
| -------------------- | :-------: |
| Latest `1.x` release |    Yes    |
| Older releases       |    No     |

## Reporting a vulnerability

Do not open a public issue or discussion for a suspected vulnerability.

Use GitHub's
[private vulnerability reporting](https://github.com/ThreatFlux/threatflux-hashing/security/advisories/new).
If that is unavailable, email `security@threatflux.ai` with the repository name
in the subject.

Include, when possible:

- affected versions, features, and algorithms;
- impact and realistic attack conditions;
- a minimal reproducer or sample input;
- suggested mitigations; and
- whether the issue is already public.

Remove credentials, personal information, and unrelated production data. Encrypt
especially sensitive material before sending it and ask for a preferred key or
transfer method.

We aim to acknowledge reports within three business days. Validation,
remediation, disclosure timing, and credit are coordinated privately. Please
allow a reasonable remediation window before public disclosure.

## Security model

ThreatFlux Hashing computes file digests. It is not an authentication,
integrity-verification, or access-control boundary on its own. Applications
remain responsible for:

- choosing an algorithm appropriate to the threat model. MD5 and SHA-1 style
  digests are not collision resistant, so MD5 output from this crate is suitable
  for deduplication, corruption checks, and legacy interoperability only;
- authenticating digests they transmit or store, because an unsigned digest
  proves nothing about origin;
- bounding the paths, sizes, and concurrency of untrusted inputs, since hashing
  reads whatever file the caller names;
- handling symlinks, special files, and time-of-check-to-time-of-use races
  before passing a path to this crate; and
- treating digests of sensitive content as sensitive, because a digest can
  confirm a guessed value.

## Dependency disclosures

Reports that only repeat a dependency advisory should explain whether the
vulnerable code is reachable in this crate. Automated scanner output is useful,
but reachability and impact help maintainers prioritize the fix.
