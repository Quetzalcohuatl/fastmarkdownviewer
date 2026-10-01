FastMarkdownViewer v0.2.8 improves reading customization, large-content loading, and desktop icons on Windows, macOS, and Linux.

- **Reading preferences:** Monokai is the default for new settings. Choose a comfortable document width or fit the window, and adjust document text size independently of interface zoom. Word wrap stays enabled by default; saved preferences are preserved.
- **Mermaid appearance:** Diagram backgrounds, labels, colors, and fonts follow the chosen reader appearance.
- **Progressive rendering:** Large code blocks appear as plain text while syntax colors load in the background. Mermaid diagrams render progressively in background tiles, replacing the four-megapixel fallback and fixed timeout with loading progress and cancellation.
- **Large files and images:** Remove application byte-size cutoffs for Markdown, Mermaid, and local/downloaded images. Available memory, rendering complexity, graphics hardware, and documented renderer safeguards still apply.
- **Crisp desktop icons:** Rounded standard/Retina icons for the Mac app bundle, fifteen Windows EXE/installer resolutions, and installed PNG/SVG Linux launcher icons.

Install or update using the platform download, `cargo binstall fast-markdown-viewer`, or `cargo install fast-markdown-viewer --locked`. Binstall installs only the executable; use the Windows installer, Mac `.app`, or Linux `.deb` for desktop integration. WinGet submission is automatic after the release, subject to Microsoft's validation and review.

Supported baselines remain Windows 10 22H2/11 x64, macOS 15+ Apple Silicon/Intel, and Ubuntu 24.04 x64. Source builds require Rust 1.95+. Windows builds are unsigned and Mac apps are ad-hoc signed, not notarized.

The older competitor comparison remains labelled with its measured v0.2.1 baseline. See the [v0.2.8 performance check](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.8/experiments/releases/0.2.8/README.md) for fresh same-machine measurements and limitations, and the [installation guide](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.8/docs/CROSS_PLATFORM.md) for platform details. Download SHA256SUMS.txt with your package and verify its checksum and GitHub provenance attestation.
