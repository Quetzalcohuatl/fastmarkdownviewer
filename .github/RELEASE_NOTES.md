FastMarkdownViewer v0.2.0 brings the native reader to macOS and Ubuntu, remembers your reading session, and fixes unnecessary idle CPU usage.

- Windows 10 22H2/11 x64: portable EXE, ZIP, and per-user installer.
- macOS 15+: separate Apple Silicon and Intel app ZIPs, with Finder Markdown associations and opening into the running app.
- Ubuntu 24.04 x64: Debian package with runtime dependencies, plus a tar archive; X11 and Wayland rendering.
- Themes, text/code fonts, zoom, wrapping, remote-image preference, tabs, windows, and reading positions are saved on normal exit. Bare launches restore the session; inactive tabs load lazily.
- Unchanged window titles no longer trigger continuous repainting.

All platforms retain tabs, Find, outline navigation, math, offline Mermaid with source fallback, and bounded asynchronous images. There is no editor, file watcher, updater, or telemetry.

Windows builds remain unsigned. Mac apps are ad-hoc signed, but are not Developer ID signed or notarized; downloaded-app approval may be required. Other Linux distributions and older macOS versions are outside the supported matrix. Mermaid is a supported subset, not full Mermaid.js compatibility; mixed-direction text and detached-window accessibility retain documented limits.

See [desktop installation](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.0/docs/CROSS_PLATFORM.md) and [acceptance coverage](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.0/docs/DESKTOP_ACCEPTANCE.md). Download SHA256SUMS.txt with your package and verify the checksum and GitHub build-provenance attestation.
