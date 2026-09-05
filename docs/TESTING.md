# Testing and release gates

FastMarkdownViewer treats regressions in opening, rendering, scrolling, and
basic interaction as release blockers. Pull requests and tagged releases run
the same locked Rust suite with `cargo test --locked --all-targets`.

## Deterministic UI interactions

`tests/ui_interactions.rs` feeds real egui input events through the production
`ViewerApp` surface. It currently verifies:

- mouse-wheel scrolling of the nested-list feature fixture without a panic;
- Page Up, Page Down, Up, Down, Home, End, and Space scrolling behavior;
- one-file drag-and-drop, multiple-drop rejection, and invalid-file errors;
- Ctrl+O file selection through an injected platform service;
- safe HTTP(S) and relative-Markdown link activation, including AccessKit
  interaction, while unsafe schemes remain inert;
- selectable rendered text and copy output;
- system light/dark changes; and
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

The release workflow repeats the native smoke test against the exact optimized
binary that is subsequently packaged. A failure prevents release publication.

`scripts/test-windows-installer.ps1` then silently installs the produced setup,
checks both Open-with registrations without changing either default
association, exercises the installed binary, uninstalls it, and verifies that
the files and registry entries are removed. This also blocks publication.

## Manual clean-VM acceptance

Automation does not replace the final Windows 10 22H2 and Windows 11 clean-VM
matrix. Before the public v0.1 tag, record the portable, installer, association,
uninstaller, SmartScreen, theme, and DPI observations described in the project
acceptance plan. Keep those results with the release notes.
