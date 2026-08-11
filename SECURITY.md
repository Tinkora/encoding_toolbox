# Security Policy

## Supported Versions

Security fixes are provided for the latest published release. The `main` branch is
development code and is supported on a best-effort basis.

## Report a Vulnerability

Do not open a public issue for a suspected vulnerability. Use
[GitHub private vulnerability reporting](https://github.com/Tinkora/encoding_toolbox/security/advisories/new)
to share the affected version, impact, reproduction steps, and any suggested fix.

You should receive an initial response within seven days. We will coordinate
validation, a fix, and disclosure through the private advisory. Please do not
publish details before a coordinated disclosure date is agreed.

## Security Boundaries

In scope:

- Input validation or size-limit bypasses.
- Incorrect encoding, digest, or HMAC results.
- Browser behavior that uploads, persists, or exposes user input.
- Secret exposure through HMAC handling, errors, logs, or generated artifacts.
- Release workflow or artifact integrity weaknesses.

The browser application loads its own static JavaScript and WASM files over HTTP(S),
but it does not send user input to a server. It does not use analytics, telemetry,
cookies, local storage, IndexedDB, a CDN, or third-party runtime assets.

The CLI accepts untrusted bytes and paths, but it is not a sandbox. HMAC keys are
read from a named environment variable and are never included in successful output
or error messages. Environment variables can still be visible to privileged local
processes, so use an appropriately isolated execution environment.

Checksums detect changes; they do not establish that input is safe or that an
artifact came from a trusted publisher. Verify release attestations when provenance
matters.
