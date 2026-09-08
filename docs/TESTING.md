# Testing and release gates

FastMarkdownViewer treats regressions in opening, rendering, scrolling, and
basic interaction as release blockers. Pull requests and tagged releases run
the same locked Rust suite with `cargo test --locked --all-targets`.

## Deterministic UI interactions

`tests/ui_interactions.rs` feeds real egui input events through the production
`ViewerApp` surface. It currently verifies:

- the empty landing view paints the full viewport and exposes File and Settings;
- mouse-wheel scrolling of the nested-list feature fixture without a panic;
- Ctrl+mouse-wheel zoom in and out;
- Page Up, Page Down, Up, Down, Home, End, and Space scrolling behavior;
- multiple-file drag-and-drop, duplicate-tab activation, and invalid-file errors that preserve documents;
- Ctrl+O file selection through an injected platform service;
- safe HTTP(S) and relative-Markdown link activation, including AccessKit
  interaction, while unsafe schemes remain inert;
- selectable rendered text and copy output;
- system light/dark changes;
- actual fallback font glyph coverage for English, Japanese, Chinese, Korean, Arabic, Hebrew, and emoji;
- Ctrl+F across formatted text and code, next/previous results, Escape, and horizontal match reveal;
- tab switching/closing and per-tab scroll preservation;
- duplicate and Setext outline navigation, Ctrl+H toggling, and empty-window shortcut help;
- repeated Ctrl+F toggling and keyboard scrolling after Find closes;
- tab tear-out with preserved state, cancellation, negative monitor origins, repeated child tear-out, and original/last-window close behavior; and
- rendering at 100%, 150%, and 200% scale factors.

`tests/render_smoke.rs` separately renders the complete feature matrix. It is
the focused regression for the v0.1 nested-list crash in
`egui_commonmark::show_scrollable`.

## Native Windows process smoke test

`scripts/test-windows-ui.ps1` launches the compiled executable with the feature
fixture, waits for the content window, sends native Page Down, Down, End, and
Home key messages, confirms the process stays alive, and verifies that two
separate launches create two independent processes and windows. It always
closes the processes it creates.

Run the local gates from a Visual Studio developer shell:

```powershell
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked
.\scripts\test-windows-ui.ps1 `
  -Binary .\target\debug\FastMarkdownViewer.exe `
  -Fixture .\tests\fixtures\feature-matrix.md
```

Before a release, run the native smoke test locally against the exact optimized
binary that will be represented by the tag. GitHub-hosted Windows workers do
not provide a usable interactive graphics desktop, so hosted CI uses
`-Headless` to verify the exact executable starts and reports its version; the
deterministic egui tests remain the hosted scrolling and interaction gate.
Native window/input coverage is required on an interactive Windows machine and
in clean-VM acceptance.

`scripts/test-windows-installer.ps1` then silently installs the produced setup,
checks both Open-with registrations without changing either default
association, exercises the installed binary, uninstalls it, and verifies that
the files and registry entries are removed. Hosted CI invokes its exact-binary
check with `-Headless`; local and clean-VM runs exercise the actual window. This
also blocks publication.

## Manual clean-VM acceptance

Automation does not replace the final Windows 10 22H2 and Windows 11 clean-VM
matrix. Before the public v0.1 tag, record the portable, installer, association,
uninstaller, SmartScreen, theme, and DPI observations described in the project
acceptance plan. Keep those results with the release notes.


## Reading-tool visual checks

`tests/render_smoke.rs` also verifies that off-screen code does not start a highlight worker, visible Rust code gains multiple syntax colors, and theme changes refresh the result.

Use the framebuffer capture example when desktop capture cannot read an OpenGL surface:

```powershell
cargo run --locked --example visual_check -- tests/fixtures/navigation.md target/reading-tools.png greeting
```

The example opens the production UI, optionally searches for the final argument, captures egui's actual framebuffer, and closes itself. Inspect the generated PNG for heading layout, script glyphs, syntax colors, and match placement. This is a visual regression aid, not a startup benchmark. Full mixed-direction text layout still requires a separate renderer improvement; glyph coverage tests do not claim bidi conformance.
