FastMarkdownViewer v0.2.2 improves multilingual input and updates dependencies, including an HTTPS security fix.

- **Multilingual Find and rename:** Script fallback fonts now load for text entered into Find and rename, including when the open document and existing filenames contain only English.
- **Broader desktop coverage:** Regression checks exercise script contexts, restored/renamed/detached tabs, font changes, headings, search, native clipboard input, and visual captures on Windows, macOS, and Ubuntu.
- **Dependency maintenance:** Update the egui stack, file-opening helper, SVG renderer, and pinned GitHub Actions. Update rustls to 0.23.45 for [RUSTSEC-2026-0285](https://github.com/rustls/rustls/security/advisories/GHSA-2mjx-qc3c-rqvc) in HTTPS image transport; upgrading from v0.2.1 is recommended.
- **Documented comparisons:** Expand the README with competitor resource measurements, feature comparisons, screenshots, and reproducible raw data. These benchmarks measured v0.2.1; they are not new v0.2.2 measurements.

Packages are available for Windows x64, macOS 15+ Apple Silicon/Intel, and Ubuntu 24.04 x64. Windows builds remain unsigned; Mac apps are ad-hoc signed and unnotarized, so downloaded-app approval may be required. Multilingual text rendering still depends on installed system fonts; UI labels remain English.

See [desktop installation](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.2/docs/CROSS_PLATFORM.md) and the [code-signing policy](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.2/CODE_SIGNING.md). Download SHA256SUMS.txt with your package and verify its checksum and GitHub build-provenance attestation.
