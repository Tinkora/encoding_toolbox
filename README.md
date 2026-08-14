# Encoding Toolbox

[中文](./README.zh-CN.md)

<!-- markdownlint-disable MD033 -->
<p align="center">
  <a href="https://ko-fi.com/tinkora" target="_blank" rel="noopener noreferrer">
    <img
      src="https://ko-fi.com/img/githubbutton_sm.svg"
      alt="Support Tinkora on Ko-fi"
      width="520"
    >
  </a>
</p>
<!-- markdownlint-enable MD033 -->

[![CI](https://github.com/Tinkora/encoding_toolbox/actions/workflows/ci.yml/badge.svg)](https://github.com/Tinkora/encoding_toolbox/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](https://www.rust-lang.org)

A local-first encoding, digest, and HMAC workbench for developers and AI agents.
Use the deterministic CLI in scripts or the browser-local WebAssembly app for
interactive work.

[Open the browser tool](https://tinkora.github.io/encoding_toolbox/) | [Download a release](https://github.com/Tinkora/encoding_toolbox/releases)

## Why It Exists

Agent workflows frequently need a small, deterministic step between tools:
decode a Base64 payload, hash a downloaded artifact, or produce an HMAC without
sending sensitive input to another service. Encoding Toolbox keeps those steps
local and exposes stable command output for automation.

## Features

| Operation | Algorithms |
| --- | --- |
| Encode / decode | Base64, Base64URL without padding, Base32, Base32Hex, Hex |
| Digest | SHA-256, SHA-384, SHA-512, BLAKE3, legacy MD5 |
| HMAC | HMAC-SHA-256, HMAC-SHA-512 |

- CLI input from stdin or a file, with stable exit codes and optional JSON output.
- Browser text and local-file workflows powered by the same Rust core through WASM.
- Binary-safe decode output in the CLI and downloadable binary output in the browser.
- No uploads, telemetry, cookies, browser storage, CDN, or third-party runtime assets.

MD5 is available only for comparing legacy checksums. It is not collision resistant
and must not be used for new security-sensitive designs.

## CLI Quick Start

Build from source:

```bash
git clone https://github.com/Tinkora/encoding_toolbox.git
cd encoding_toolbox
cargo build --release -p encoding_toolbox_cli
```

Encode and decode:

```bash
printf 'hello' | target/release/tinkora-encoding encode --algorithm base64
printf 'aGVsbG8=' | target/release/tinkora-encoding decode --algorithm base64
```

Hash a file and request the versioned JSON contract:

```bash
target/release/tinkora-encoding digest --algorithm sha256 ./artifact.bin
target/release/tinkora-encoding --json digest --algorithm blake3 ./artifact.bin
```

Read an HMAC key from an environment variable so it does not appear in command
history or the process argument list:

```bash
export TOOLBOX_HMAC_KEY='replace-me'
printf 'message' | target/release/tinkora-encoding hmac \
  --algorithm sha256 \
  --key-env TOOLBOX_HMAC_KEY
unset TOOLBOX_HMAC_KEY
```

Successful JSON output uses schema version 1:

```json
{"schema_version":1,"operation":"digest","algorithm":"sha256","result":"..."}
```

`decode` cannot be combined with `--json` because decoded output may contain
arbitrary bytes. Errors are written to stderr as `error [CODE]: message`; usage
errors exit with code 2 and operational errors exit with code 1.

## Browser Tool

The hosted tool processes input in the browser. To run it locally:

```bash
cargo install wasm-pack --version 0.15.0 --locked
cd crates/encoding_toolbox_web
npm ci
npm run prepare:wasm
npm run serve
```

Open `http://127.0.0.1:4197/static/`.

## Limits and Security Model

| Boundary | Limit |
| --- | ---: |
| CLI stdin or file input | 100 MiB |
| Browser text input | 1 MiB |
| Browser local file input | 20 MiB |
| Encoded text accepted for decoding | 8 MiB |

These limits bound memory use; this release does not provide streaming transforms.
A matching checksum can detect accidental changes, but it does not prove that a
file is safe or authentic. Verify signed provenance when authenticity matters.

## Development

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

See [CONTRIBUTING.md](./CONTRIBUTING.md) for the pull request workflow and
[SECURITY.md](./SECURITY.md) for private vulnerability reporting.

## License

MIT. See [LICENSE](./LICENSE).
