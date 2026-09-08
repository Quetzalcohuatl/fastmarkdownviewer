# v0.1 release checklist

FastMarkdownViewer uses a deliberately small release bar. A release is ready
when its core job is reliable; optional platform expansion and exhaustive
benchmarking do not block v0.1.

## Automated gates

- [x] Locked build, formatting, Clippy, Rust tests, and dependency-policy checks
  pass.
- [x] The full feature fixture renders and scrolls without the nested-list
  regression.
- [x] Deterministic tests cover opening, drag-and-drop, links, selection/copy,
  keyboard and wheel scrolling, themes, DPI scaling, and visible errors.
- [x] Native smoke tests exercise the exact executable, independent windows,
  installer, Open-with registration, uninstall, packaging, and checksums.
- [x] Tagged builds and GitHub provenance are produced only by GitHub Actions.

## Beta and stable decision

- [x] The product name and repository metadata are fixed for v0.1.
- [x] The website, privacy behavior, unsigned status, and verification steps are
  clear.
- [x] Publish `v0.1.0-beta.1` and confirm its hosted CI, installer, assets, and
  Pages deployment.
- [x] Resolve any reproducible crash, unsafe network behavior, or broken core
  interaction reported against the beta.
- [x] Do one clean Windows 10 or 11 install/open/scroll/uninstall pass against
  the exact stable candidate.
- [x] Prepare version 0.1.0, changelog, site, and release notes for the
  annotated `v0.1.0` tag. Publication is verified by the Release workflow.

## Stable candidate validation — 2026-09-08

- Version 0.1.0 passes formatting, Clippy, all 46 tests, cargo-audit, and cargo-deny.
  The two documented unmaintained-dependency exceptions remain unchanged.
- The optimized portable binary passes the native Windows 11 Pro (build 26200)
  open/scroll/two-window smoke test.
- A fresh per-user installation (no existing application or ProgID) passes the
  same native smoke test. Both Open-with registrations are verified, defaults
  remain unchanged, and uninstall removes the application and registrations.
- Find wildcards, outline toggling, tab transfer/lifetime behavior, font glyphs,
  and DPI cases pass the production-UI regression suite.
- This is a clean application installation on the existing Windows host, not a
  pristine OS VM or an exhaustive Windows compatibility matrix. Native tab
  dragging and SmartScreen prompts are not asserted by the scripted smoke test.
- Release publication, installer verification, checksums, and provenance are
  additionally gated by the annotated-tag GitHub workflow.

Startup benchmarking, Windows ARM64, macOS/Linux packages, code signing,
package-manager listings, high-contrast design, and safe viewport
virtualization are valuable follow-ups. They are not required for the small,
honestly described Windows x64 v0.1 release.
