# Architecture

FastMarkdownViewer separates document policy from its UI so the same core can serve later macOS and Linux builds.

## Runtime flow

1. `cli` accepts no path, one path, or `--version`.
2. `document` validates regular-file status, the 32 MiB limit, UTF-8/BOM encoding, and the base URL.
3. `render_source` turns fenced `math` into display math and preserves other code language hints.
4. `egui_commonmark` owns the pulldown-cmark render parse. The stable renderer stays inside an egui scroll area to preserve nested parser state. The vendored parser records headings and, while Find is open, text geometry.
5. `math` starts one background RaTeX worker only when visible math is requested and caches theme/DPI-specific SVG in memory.
6. `network` provides memory-only local/remote byte, decoded-image, and texture loaders with redirect, private-network, byte, format, and pixel limits. LRU caches bound retained payloads, and lazy worker permits cap I/O at four jobs and decoding at two. Per-entry tickets prevent stale completions from restoring invalidated images. A shared context policy and renderer image gate implement optional automatic remote loading, including placeholders for cached images.
7. The backend starts a bounded syntect worker lazily for visible fenced code, with shared immutable syntax/theme definitions and per-tab layout caches.
8. `links` classifies every click before `platform` opens a browser or starts another viewer process.

The application has no persistence feature and disables eframe window/memory persistence.

## Renderer choice

Glow is the shipping backend for v0.1. The reproducible eframe Glow, eframe wgpu, and Direct2D/DirectWrite bake-off lives under `experiments/architecture`. Direct2D may replace egui only if its geometric-mean median first-content time is at most half egui's and it has no selection, accessibility, math, image, table, or high-DPI blocker. No unrun or incomparable result is presented as a measurement.

## Windows boundary

Native file selection and shell launching live in `platform`. The build script embeds version metadata, an icon, and the per-monitor-v2 manifest. Installer-only file associations live in `packaging/windows`; portable builds perform no registration.


## Document tabs and navigation

`DocumentTab` owns the document, CommonMark cache, math renderer, find state, and scroll position. `ViewerWindow` owns tab selection, outline visibility, and drag state. `ViewerApp` hosts deferred native viewports that repaint independently. Tear-out moves the owned tab without reloading or cloning the document; only viewport-specific geometry is reset. Tab IDs, theme, font context, and zoom are shared within the process. Child callbacks queue lifetime changes for the root host. Because eframe requires its root window to survive, closing the original hides it and releases its documents until the last detached window closes. No new dependency is needed for multiple windows.

The outline comes from the renderer's actual heading events and positions. Find uses rendered text and per-widget galleys, preserves selection/accessibility, matches across inline styles, and computes per-row highlight rectangles. Geometry refreshes with layout; code matches also request scrolling inside their nested horizontal area. Search text/geometry is collected only while Find is open. The regex escapes literal text and translates asterisk wildcards to greedy, single-line matches; empty matches are excluded. Matching remains Unicode-aware and optionally case-insensitive.

Windows font fallback reads Segoe UI/Arial plus on-demand Gothic, Microsoft YaHei, and Malgun Gothic from the Windows fonts directory, retaining bundled emoji. Large fonts are checked once per opened tab and installed once per context. This improves character coverage but does not add full Unicode bidi paragraph layout to egui.
