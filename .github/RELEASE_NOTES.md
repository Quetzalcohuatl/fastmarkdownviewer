FastMarkdownViewer v0.2.5 fixes Windows startup when the display driver cannot provide the OpenGL version the viewer needs, including the failure reported by WinGet's validation VM.

- **Automatic graphics fallback:** Keep OpenGL as the normal renderer; if initialization fails, try Direct3D 12 and Windows' WARP software renderer. Software rendering runs on the CPU and can be slower. No separate graphics-runtime download is needed on supported Windows versions.
- **Real startup checks:** Windows CI now opens the application with and without a document, checks it survives beyond WinGet's ten-second launch window, exercises scrolling and independent windows, and explicitly tests the CPU renderer. The installer check also opens the installed app.
- **Distribution:** This release triggers the configured Cargo publication and WinGet submission workflows. WinGet availability still depends on Microsoft's validation, review, and indexing.

The additional Windows renderer increases executable size. macOS and Linux retain their existing graphics path. Rust 1.95+ is still required to build from source; downloaded desktop packages do not require Rust.

Packages are available for Windows x64, macOS 15+ Apple Silicon/Intel, and Ubuntu 24.04 x64. Windows builds remain unsigned; Mac apps are ad-hoc signed and unnotarized. Multilingual text rendering still depends on installed system fonts; UI labels remain English.

The 20-second demo retains its v0.2.2 label, and competitor performance comparisons retain their measured v0.2.1 baseline.

See [desktop installation](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.5/docs/CROSS_PLATFORM.md) and the [code-signing policy](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.5/CODE_SIGNING.md). Download SHA256SUMS.txt with your package and verify its checksum and GitHub build-provenance attestation.
