FastMarkdownViewer v0.2.3 reduces executable size while retaining the same reading features and font coverage.

- **Remove duplicate image loaders:** Keep FMV's existing bounded image-loading path and remove the redundant framework loaders and older SVG rendering dependencies. Local and remote images, PNG/JPEG/GIF/WebP/SVG, math, and Mermaid remain supported.
- **Optimize across the application:** Use full link-time optimization with the existing speed-focused compiler setting. More aggressive size settings were tested and rejected because they slowed reading.
- **Measured trade-offs:** The two passes reduced matched pre-release executable builds by approximately 10–14%, depending on platform. In the 120-run Windows compiler comparison, the selected profile retained similar CPU reading medians. These are executable-size and CPU-harness measurements, not claims of lower runtime memory or faster startup. [Methods and raw results](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.3/experiments/size/compiler-profiles.md) and [native package comparisons](https://github.com/Quetzalcohuatl/fastmarkdownviewer/pull/8) are available.
- **Validation:** The candidate passed 93 optimized Windows tests and native macOS ARM/Intel and Ubuntu acceptance. This release workflow repeats the versioned build, package, installation, and policy checks before publication.

Packages are available for Windows x64, macOS 15+ Apple Silicon/Intel, and Ubuntu 24.04 x64. Windows builds remain unsigned; Mac apps are ad-hoc signed and unnotarized. Multilingual text rendering still depends on installed system fonts; UI labels remain English.

The 20-second demo retains its v0.2.2 label, and competitor performance comparisons retain their measured v0.2.1 baseline.

See [desktop installation](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.3/docs/CROSS_PLATFORM.md) and the [code-signing policy](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.3/CODE_SIGNING.md). Download SHA256SUMS.txt with your package and verify its checksum and GitHub build-provenance attestation.
