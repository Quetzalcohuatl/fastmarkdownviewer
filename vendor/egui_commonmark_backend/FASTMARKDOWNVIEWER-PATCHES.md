# FastMarkdownViewer patches

Vendored `egui_commonmark_backend` 0.25.0, under its original MIT OR Apache-2.0 license.

1. Forward image alt text to egui for failure/accessibility output.
2. Keep code lines unwrapped in a horizontal scroll area.
3. Record selectable text galleys and search geometry while Find is active; preserve egui's selection and AccessKit paths.
4. Record link captions through the same geometry path while preserving safe URL interception and fragment links.
5. Load syntect syntax/theme sets on a background worker only for visible tagged code. Bound work queues, block size, and per-tab caches; keep plain text until results arrive and for unsupported languages.
6. Reveal code search targets inside the nested horizontal scroll area.
7. Invoke an optional image gate before image loading or cached texture display, preserving alt text for placeholders.
8. Forward fragment link clicks through the application's link interceptor, which resolves explicit IDs and generated heading slugs consistently across tabs.
9. Balance nested delayed-event delimiters, preserve inline whitespace while wrapping selectable labels, and constrain image widths to their container.
10. Show readable image failure placeholders with alt text, hover details, and retry through the existing loader cache.
11. Keep code backgrounds and copy buttons within the code scroll viewport rather than extending across the entire unwrapped line.
12. Size code backgrounds from actual content height, honor the wrapping setting, and omit closed-cell delimiters/empty terminal rows from parsed tables.

The upstream optional `better_syntax_highlighting` API is retained but is not enabled by this application; the local lazy highlighter uses syntect's default syntax/theme dumps and pure-Rust regex engine directly. Original licenses are preserved. No upstream submission is claimed.
