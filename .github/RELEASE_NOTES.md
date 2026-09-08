FastMarkdownViewer is a small Windows x64 Markdown reader focused on the core
double-click, open, render, and scroll experience.

This beta adds document tabs with drag-out native windows, Ctrl+F search with
highlighted matches, lazy syntect code highlighting, a Ctrl+H heading sidebar,
and on-demand Windows font fallbacks for multilingual documents. Ctrl+F toggles
Find open and closed, and the empty-window page lists keyboard shortcuts.
Detached tabs retain their reading position and search state. Theme and text size
are shared by windows within a process.

Moving tabs into existing windows and reordering tabs are not supported yet.
The current backend does not initialize native accessibility in detached child
windows; use a separate application launch for screen-reader access.
Emoji are monochrome; full bidirectional paragraph layout remains limited by the
renderer, and glyph coverage depends on installed Windows fonts.

The executable and installer are currently unsigned, so Windows SmartScreen may
warn. Verify downloads with `SHA256SUMS.txt` and the GitHub build-provenance
attestation.

Remote images make asynchronous HTTP(S) requests under the policy documented in
`PRIVACY.md`. There is no updater, file watcher, editor, telemetry, or persistent
settings database.
