FastMarkdownViewer v0.2.1 fixes missing characters in tab names.

Script fallback fonts now load from every tab title, including inactive restored tabs and renamed files. Previously, a Chinese, Japanese, or Korean filename could show squares when the document contents were English, because font discovery only inspected the active document's text.

A regression test checks actual glyph coverage for Chinese and Korean tab labels while preserving lazy loading of inactive documents. Native macOS acceptance also captures a filename containing Japanese and Chinese characters.

Font discovery also includes macOS system font asset folders, where macOS 15 stores PingFang outside the traditional Fonts directory. Built-in Heiti provides Chinese and Japanese fallback when optional PingFang is absent.

Packages are available for Windows x64, macOS 15+ Apple Silicon/Intel, and Ubuntu 24.04 x64. Windows builds remain unsigned; Mac apps are ad-hoc signed and unnotarized, so downloaded-app approval may be required.

See [desktop installation](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.1/docs/CROSS_PLATFORM.md). Download SHA256SUMS.txt with your package and verify its checksum and GitHub build-provenance attestation.
