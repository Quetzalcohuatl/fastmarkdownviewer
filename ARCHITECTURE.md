# Architecture

FastMarkdownViewer separates document policy from its UI so the same core can serve later macOS and Linux builds.

## Runtime flow

1. `cli` accepts no path, one path, or `--version`.
2. `document` validates regular-file status, the 32 MiB limit, UTF-8/BOM encoding, and the base URL.
3. `render_source` turns fenced `math` into display math and deliberately removes code language hints.
4. `egui_commonmark` owns the pulldown-cmark render parse. Its scrollable path virtualizes widgets so off-screen images and math do not start expensive work.
5. `math` starts one background RaTeX worker only when visible math is requested and caches theme/DPI-specific SVG in memory.
6. `network` provides memory-only local/remote byte and image loaders with redirect, private-network, byte, format, and pixel limits.
7. `links` classifies every click before `platform` opens a browser or starts another viewer process.

The application has no persistence feature and disables eframe window/memory persistence.

## Renderer choice

Glow is the shipping backend for v0.1. The reproducible eframe Glow, eframe wgpu, and Direct2D/DirectWrite bake-off lives under `experiments/architecture`. Direct2D may replace egui only if its geometric-mean median first-content time is at most half egui's and it has no selection, accessibility, math, image, table, or high-DPI blocker. No unrun or incomparable result is presented as a measurement.

## Windows boundary

Native file selection and shell launching live in `platform`. The build script embeds version metadata, an icon, and the per-monitor-v2 manifest. Installer-only file associations live in `packaging/windows`; portable builds perform no registration.
