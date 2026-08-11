# Pull Request

## Problem and outcome

<!-- What real user problem does this solve, and what changes for the user? -->

## Scope

- [ ] Bug fix
- [ ] Feature
- [ ] Documentation
- [ ] Refactor or maintenance
- [ ] CI, release, or dependency update

## Validation

<!-- List exact commands and any manual checks. -->

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo test --workspace --locked`
- [ ] `cargo clippy --workspace --all-targets --locked -- -D warnings`
- [ ] WASM checks pass when affected
- [ ] Browser tests pass at 375, 768, 1024, and 1440 px when affected

## Compatibility and security

- [ ] Public algorithm keys, error codes, and JSON schema remain compatible or the change is documented.
- [ ] New algorithms have known-answer tests.
- [ ] Logs, fixtures, and screenshots contain no secrets or personal data.
- [ ] English and Chinese documentation are updated when user-facing behavior changes.

## Related issue

<!-- Use `Closes #123` when this PR fully resolves an issue. -->
