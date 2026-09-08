FastMarkdownViewer v0.1.1 adds remote-image controls and reading conveniences without adding dependencies.

- **Settings → Automatically load remote images** defaults to on. Turn it off for individual **Load image** buttons. The setting applies to all windows in the session; requests already running may finish. Ordinary links remain click-only.
- Image caches and concurrent loading/decoding are bounded. Redundant image copies are released after texture upload; closing or reloading a tab invalidates its images.
- **F5 / Ctrl+R** reloads the current file while preserving the tab and reading position. A failed reload keeps the current document open.
- Drag a tab onto another tab to reorder it. Dragging outside the window still creates an independent native window.
- Local image inputs now have a 10 MiB limit, and SVG raster dimensions are checked before allocation.

All 52 tests pass. The repository includes exploratory comparisons against v0.1.0 with raw samples: startup proxies are similar and the local-image fixture uses less memory. These are small same-machine comparisons, not first-content timing or broad performance claims. Idle CPU and large-document layout remain performance follow-ups.

Download the portable EXE, portable ZIP, or per-user installer below. The executable and installer remain unsigned. Verify downloads with `SHA256SUMS.txt` and GitHub build-provenance attestations.

Existing limitations: tabs cannot move into an existing window; detached child windows have limited native accessibility (use a separate launch for screen-reader access). Emoji are monochrome, full bidirectional layout remains limited, and glyph coverage depends on installed fonts. There is no editor, watcher, updater, telemetry, or persistent settings database. See `PRIVACY.md` for network behavior.
