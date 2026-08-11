# Contributing to Encoding Toolbox

Thank you for helping improve Encoding Toolbox. Small, problem-focused pull
requests are easier to review and release.

## Before Opening an Issue

- Use [GitHub Discussions](https://github.com/Tinkora/encoding_toolbox/discussions)
  for questions and early ideas.
- Search existing issues before filing a bug or feature request.
- Report vulnerabilities through the private process in [SECURITY.md](./SECURITY.md).

Feature requests should describe a repeatable user problem. A new algorithm or
output format needs a concrete workflow that is not already covered by common
system tools or this project's current interface.

## Development Setup

Prerequisites:

- Rust 1.95 with `rustfmt`, `clippy`, and the `wasm32-unknown-unknown` target.
- `wasm-pack` 0.15.0.
- Node.js 24 or newer for browser tests.

```bash
git clone https://github.com/YOUR-ACCOUNT/encoding_toolbox.git
cd encoding_toolbox
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --version 0.15.0 --locked
cd crates/encoding_toolbox_web && npm ci && cd ../..
```

## Pull Request Workflow

1. Create a branch from the current `main`.
2. Add outcome-focused tests before or with behavior changes.
3. Keep public algorithm keys, error codes, and JSON schema compatibility intact.
4. Update both README files when user-facing behavior changes.
5. Run the relevant checks below.
6. Open a pull request using the repository template and link the issue it solves.

Use English [Conventional Commits](https://www.conventionalcommits.org/) such as
`feat: add Crockford Base32 decoding` or `fix: reject oversized stdin input`.

## Required Checks

```bash
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo check -p encoding_toolbox_web --target wasm32-unknown-unknown --locked
wasm-pack test --node crates/encoding_toolbox_web --locked
```

For frontend changes:

```bash
cd crates/encoding_toolbox_web
npm test
```

Frontend pull requests must also be checked at widths 375, 768, 1024, and 1440
pixels for keyboard access, overflow, console errors, and unexpected network traffic.

## Review Expectations

Maintainers may ask to reduce scope when a pull request combines unrelated changes
or adds speculative functionality. Reviews prioritize correctness, deterministic
automation behavior, bounded resource use, and honest documentation.

By contributing, you agree that your contribution is licensed under this project's
MIT license and that you will follow [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).
