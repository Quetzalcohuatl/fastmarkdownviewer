# Contributing

Thanks for helping keep FastMarkdownViewer small, predictable, and fast.

## Before opening a change

Please open an issue before adding user-facing scope. v0.1 intentionally excludes editing, export, annotations, search, file watching, history, updates, Mermaid, and syntax highlighting. Fixes, accessibility improvements, compatibility work, tests, and measured performance improvements are welcome.

## Local checks

Use the pinned toolchain and committed lockfile:

```powershell
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo audit
cargo deny check
```

Changes to startup, rendering, dependency features, or packaging should include before/after evidence using [docs/BENCHMARKING.md](docs/BENCHMARKING.md). Do not present a single launch as a benchmark.

## Pull requests

- Keep each pull request focused.
- Add tests for behavior and error paths.
- Update the changelog for user-visible changes.
- Preserve `MIT OR Apache-2.0` compatibility for dependencies and adapted code.
- Never commit generated release binaries, secrets, certificates, benchmark machine identifiers, or user documents.

By contributing, you agree that your contribution is licensed under the project's dual license.
