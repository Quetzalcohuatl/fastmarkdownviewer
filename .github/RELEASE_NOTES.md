FastMarkdownViewer v0.2.6 adds cargo-binstall support, so installing the viewer no longer requires compiling it or upgrading an older Rust compiler.

Install [cargo-binstall](https://github.com/cargo-bins/cargo-binstall#installation) once using its precompiled installer, then run:

```sh
cargo binstall fast-markdown-viewer
FastMarkdownViewer document.md
```

- **Official binaries:** Downloads the matching GitHub release archive for Windows x64, Linux x64, and macOS Apple Silicon/Intel. Missing or unsupported binaries fail explicitly; source compilation and community quick-install fallbacks are disabled.
- **Installation checks:** Native checks install the packaged binary with no Rust or Cargo on PATH, compare its bytes against the archive, verify its version, and render a Mermaid diagram. After publication, the distribution workflow also tests real crates.io metadata and release downloads.
- **Future updates:** Each stable release publishes the version's binstall metadata with the crate automatically. Users rerun the install command to update. WinGet submission remains automatic, subject to Microsoft's validation, review, and indexing.

Binstall installs the executable into the Cargo bin directory. Use the desktop installer, full Mac app bundle, or Debian package for desktop integration. Existing OS requirements remain: Windows 10 22H2/11 x64, macOS 15+ Apple Silicon/Intel, or Ubuntu 24.04 x64 (glibc 2.39 and desktop libraries). Source builds still require Rust 1.95+.

Windows retains v0.2.5's automatic Direct3D/CPU fallback when OpenGL initialization fails. Windows builds remain unsigned; Mac apps are ad-hoc signed and unnotarized. The demo retains its v0.2.2 label, and performance comparisons retain their measured v0.2.1 baseline.

See the [installation guide](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.6/docs/REGISTRY_DISTRIBUTION.md#binstall-prebuilt-installation). Download SHA256SUMS.txt with your package and verify its checksum and GitHub build-provenance attestation.
