# Repository Guide for AI Agents

## Project Overview

Encoding Toolbox is a local-first developer utility for reversible byte encodings,
cryptographic digests, and HMACs. It ships a deterministic CLI and a browser-local
WebAssembly application. It does not provide an MCP server.

## Architecture

```text
crates/
  encoding_toolbox_core/  Pure encoding, digest, and HMAC contracts
  encoding_toolbox_cli/   `tinkora-encoding` CLI and JSON output contract
  encoding_toolbox_web/   WASM bridge, static browser tool, and browser tests
docs/
  product_spec.zh-CN.md    Chinese product specification
```

The browser UI must call the Rust/WASM bridge for all transformations. Do not
reimplement encoding or digest behavior in JavaScript.

## Product Boundaries

- Encodings: Base64, unpadded Base64URL, Base32, Base32Hex, and lowercase Hex.
- Digests: SHA-256, SHA-384, SHA-512, BLAKE3, and legacy MD5.
- HMAC: HMAC-SHA-256 and HMAC-SHA-512.
- CLI input limit: 100 MiB. Browser text limit: 1 MiB. Browser file limit: 20 MiB.
- Decode input is text and is limited to 8 MiB by the core contract.
- MD5 is exposed only for legacy checksum comparison and must carry a visible warning.
- The CLI reads HMAC keys only from the environment through `--key-env`.
- Browser input is processed locally. Do not add telemetry, analytics, storage, or
  third-party runtime assets.

## Validation

```bash
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check -p encoding_toolbox_web --target wasm32-unknown-unknown --locked
wasm-pack test --node crates/encoding_toolbox_web --locked

cd crates/encoding_toolbox_web
npm ci
npm test
```

## Compatibility Contracts

- Keep public algorithm keys and error codes stable.
- Keep CLI JSON output at `schema_version: 1` unless a deliberate compatibility
  change is documented and tested.
- `decode` writes arbitrary bytes and therefore does not support `--json`.
- New algorithms require known-answer tests and documentation in both READMEs.
- Code comments and public code documentation must be written in English.

## Commit Language

- Write commit subjects and bodies in English and follow Conventional Commits.
- This repository-level rule overrides any global preference for another commit-message language.

## Frontend Design Requirement

- Before creating, modifying, reviewing, or debugging any HTML page or user-facing frontend, invoke the `ui-ux-pro-max` skill.
- Run the skill's required `--design-system` search before editing, followed by relevant stack and UX searches.
- If `ui-ux-pro-max` is unavailable, stop frontend work and report the missing prerequisite.
- Verify the rendered result in a real browser at 375, 768, 1024, and 1440 pixel widths, including console, keyboard, accessibility, and overflow checks.
