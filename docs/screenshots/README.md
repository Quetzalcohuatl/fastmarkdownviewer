# Screenshot provenance

These are unedited captures from the real application renderer, using synthetic
repository fixtures. They contain no user documents.

- `windows-reading.png`: Windows 11, Light theme, 150% display scale;
  `tests/fixtures/readme-showcase.md`.
- `multilingual-dark.png`: Windows 11, Dark theme, 150% display scale;
  `tests/fixtures/multilingual-ui.md`, Find query `中文`.
- `macos-reading.png`: macOS 15, Apple Silicon, 100% scale, Light theme;
  `tests/fixtures/cross-platform.md`.
- `ubuntu-reading.png`: Ubuntu 24.04, X11/software OpenGL, 100% scale, Dark theme;
  the same cross-platform fixture.

All use the optimized `visual_check` example at application source `dda53cf`,
which embeds the production `ViewerApp`, fonts, image loaders, and renderer.
It captures the framebuffer without OS window decorations. The fixture text was
refined afterward; the application code matches the measured maintenance build.
They are examples of tested content, not Unicode conformance certificates.
The Mac and Linux captures came from [Desktop builds run 34800613312](https://github.com/Quetzalcohuatl/fastmarkdownviewer/actions/runs/34800613312),
which also tested the extracted Mac app and installed Ubuntu package.

```powershell
cargo build --locked --release --example visual_check
$env:FMV_VISUAL_THEME = 'Light'
target/release/examples/visual_check.exe tests/fixtures/readme-showcase.md docs/screenshots/windows-reading.png
$env:FMV_VISUAL_THEME = 'Dark'
target/release/examples/visual_check.exe tests/fixtures/multilingual-ui.md docs/screenshots/multilingual-dark.png 中文
Remove-Item Env:FMV_VISUAL_THEME
```
