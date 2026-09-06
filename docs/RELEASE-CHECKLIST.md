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
- [ ] Publish `v0.1.0-beta.1` and confirm its hosted CI, installer, assets, and
  Pages deployment.
- [ ] Resolve any reproducible crash, unsafe network behavior, or broken core
  interaction reported against the beta.
- [ ] Do one clean Windows 10 or 11 install/open/scroll/uninstall pass against
  the exact stable candidate.
- [ ] Update the version, changelog, site, and release notes, then publish the
  annotated `v0.1.0` tag through the same workflow.

Startup benchmarking, Windows ARM64, macOS/Linux packages, code signing,
package-manager listings, high-contrast design, and safe viewport
virtualization are valuable follow-ups. They are not required for the small,
honestly described Windows x64 v0.1 release.
