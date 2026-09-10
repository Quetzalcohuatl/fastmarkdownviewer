# v0.1 release checklist

## v0.1.4 candidate validation — 2026-09-09

- Formatting, Clippy, all 73 locked tests, and the optimized Windows build pass after the version bump. The lockfile changes only the application version; dependency policy checks passed for this feature set with existing exceptions.
- The exact optimized candidate passes native open/scroll/two-window checks. Local packaging/checksums and a fresh per-user install, native installed-app smoke test, Open-with registration checks, and uninstall pass (local Inno Setup 6.7.3).
- Solarized Light and Monokai framebuffer captures were inspected during feature validation. New UI tests cover theme/font selection, same-brightness syntax palette changes, fallback preservation, and returning to the system theme. Research-backed torture suites are manual probes, not assertions that all edge cases pass.
- This is validation on the existing Windows host, not a pristine OS VM. The annotated tag workflow separately builds and verifies the public artifacts with pinned Inno Setup 6.7.1 and build provenance.

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

## v0.1.1 candidate validation — 2026-09-08

- Version 0.1.1 passes formatting, Clippy, and all 52 locked tests. Dependency
  policy checks passed for this feature set; only the application version changed
  in the lockfile for release.
- The optimized portable binary passes the native Windows open/scroll/two-window
  smoke test. Local packaging and checksums pass, followed by a fresh per-user
  install, native smoke test, Open-with registration checks, and uninstall.
- Local installer validation used Inno Setup 6.7.3; the GitHub release workflow
  uses pinned 6.7.1 and independently verifies its exact published installer.
- Remote image controls, cache lifetime, reload, and tab reordering are covered
  by production-UI and resource tests. Exploratory performance evidence is in
  `experiments/architecture/reader-results.md`.
- Publication and build provenance are gated by the annotated `v0.1.1` tag workflow.

## v0.1.2 candidate validation — 2026-09-09

- Formatting, Clippy, all 59 locked tests, and the optimized native Windows
  open/scroll/two-window smoke test pass.
- Local packaging, checksums, fresh per-user installation, native installed-app
  smoke test, Open-with registration checks, and uninstall pass (Inno Setup 6.7.3).
- The GitHub workflow independently builds and tests the exact public assets
  using pinned Inno Setup 6.7.1 and generates provenance before publication.
- The lockfile changes only the application version. No Mermaid or other viewer
  dependency was added; the feature commit's GitHub policy checks passed.
- New link/anchor navigation and tab file actions have targeted UI/filesystem
  coverage. Rename's hard-link filesystem requirement is documented in the README
  and release notes.

Startup benchmarking, Windows ARM64, macOS/Linux packages, code signing,
package-manager listings, high-contrast design, and safe viewport
virtualization are valuable follow-ups. They are not required for the small,
honestly described Windows x64 v0.1 release.

## v0.1.3 candidate validation — 2026-09-09

- Formatting, Clippy, and all 70 locked tests pass after the version bump. Dependency audit/license/source checks pass with the existing exceptions; no dependencies were added.
- The supplied torture-test document was checked in real framebuffer captures with wrapping enabled and disabled. Code backgrounds, table alignment/row heights, scrollbars, and font fallbacks have targeted regression coverage.
- Repeated exploratory CPU-layout comparisons and raw measurements are recorded in experiments/architecture/layout-fixes-2026-09-09.md.
- The annotated tag triggers GitHub Actions to build the exact versioned Windows executable, smoke-test/package/verify assets, exercise installer installation and removal, attest provenance, and publish the release. Publication is confirmed from the workflow result.
