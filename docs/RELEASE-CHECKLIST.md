# v0.1 release checklist

This is the source of truth for deciding whether FastMarkdownViewer v0.1.0 is
ready for a public tag. A green local build is a release candidate, not a
substitute for the remaining platform and publication checks.

## Automated gates

- [x] Locked Rust 1.95 build, formatting, Clippy, and all Rust tests pass.
- [x] The full feature fixture renders without the nested-list scrolling panic.
- [x] Deterministic UI tests cover wheel and keyboard scrolling, drag-and-drop,
  Ctrl+O, safe and unsafe links, text selection/copy, system themes, and common
  scale factors.
- [x] A native Windows smoke test drives the compiled window and verifies two
  independent launches.
- [x] The exact optimized executable is native-window tested locally before
  packaging and CLI-smoke-tested on GitHub's headless Windows worker.
- [x] The installer is silently installed, exercised, and uninstalled; its
  Open-with registry entries and cleanup behavior are verified.
- [x] The four release assets and their SHA-256 checksums are verified.
- [x] RustSec, license, source, and dependency-policy checks pass. The allowed
  unmaintained `ttf-parser` advisory is documented in `deny.toml` and
  `THIRD_PARTY_NOTICES.md`.

## Required before the public v0.1 tag

- [x] Set the GitHub repository metadata to
  `Quetzalcohuatl/fastmarkdownviewer`. The release workflow deliberately refuses
  to publish if this metadata does not match the actual repository.
- [ ] Confirm the provisional `FastMarkdownViewer` product name before widening
  distribution beyond the alpha audience.
- [ ] Run the 30-launch architecture protocol on the documented Windows 11 x64
  machine, record ETW first-content-present results and working sets, and commit
  the hardware record, raw CSV, and decision to `experiments/architecture`.
- [ ] Resolve or explicitly accept the current performance tradeoff: the stable
  renderer lays out the complete document to avoid the upstream scroll
  virtualizer crash, so off-screen math/image deferral is not yet demonstrated.
- [ ] Add integration coverage for slow, broken, oversized, redirected, and
  private-network remote images. Existing tests cover URL policy and decoded
  pixel limits, but not the complete HTTP behavior matrix.
- [ ] Complete and record clean Windows 10 22H2 and Windows 11 x64 VM acceptance:
  portable launch, quoted/Unicode paths, installer/uninstaller, Open with,
  unchanged defaults, no console, SmartScreen, light/dark, and 100/150/200% DPI.
- [ ] Capture a genuine release screenshot and replace the representative CSS
  preview on the GitHub Pages site.
- [ ] Push the repository and confirm CI and Pages pass on GitHub-hosted runners.
- [ ] Prepare release notes that clearly say v0.1 is unsigned and describe
  checksum and provenance-attestation verification.

Only after every item above is checked should an annotated `v0.1.0` tag be
pushed. That tag is the sole supported way to build and publish the release.
