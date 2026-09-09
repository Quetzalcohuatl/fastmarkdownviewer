FastMarkdownViewer v0.1.3 improves Markdown layout and adds optional word wrapping, with no new viewer dependencies.

- **Settings → Word wrap** is enabled by default for prose, table cells, and code. Disable it to keep long lines intact and scroll horizontally. The preference is shared across windows for the current session.
- Tables keep cells aligned beneath their headers and size rows for wrapped content. Code backgrounds surround the full block.
- Document tabs, tables, and code blocks have visible, space-reserving scrollbars when content overflows. Long unbroken words wrap without clipping ordinary prose.
- Nested blockquotes, definition lists, and nested inline formatting preserve their structure and spacing.
- Failed images show readable alt-text placeholders, error details on hover, and Retry.
- Hindi/Devanagari and Thai font fallbacks load on demand from Windows. Glyph coverage depends on installed fonts; full mixed-direction paragraph layout remains limited.

All 70 tests pass, including real UI input, wrapping toggles, table geometry, full-height code backgrounds, and navigation. Exploratory CPU-layout comparisons remain comparable to v0.1.2; raw measurements and limitations are in the repository. The app remains a lightweight, read-only Markdown viewer.

Download the portable EXE, portable ZIP, or per-user installer below. Builds remain unsigned; verify with SHA256SUMS.txt and GitHub provenance attestations. Settings are session-only. Automatic remote images remain enabled by default and can be disabled in Settings; see PRIVACY.md.
