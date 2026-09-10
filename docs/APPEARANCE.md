# Appearance changes — local build, 2026-09-09

Version 0.1.4 adds five palettes to the existing System/Light/Dark choices:
Solarized Light, Solarized Dark, Quiet Light, Monokai, and Tomorrow Night Blue.
Settings has separate text and code font submenus. See the appearance section
of [README](../README.md) for available families and session scope.

Palette references: VS Code's [Solarized Light](https://github.com/microsoft/vscode/blob/main/extensions/theme-solarized-light/themes/solarized-light-color-theme.json),
[Solarized Dark](https://github.com/microsoft/vscode/blob/main/extensions/theme-solarized-dark/themes/solarized-dark-color-theme.json),
[Quiet Light](https://github.com/microsoft/vscode/blob/main/extensions/theme-quietlight/themes/quietlight-color-theme.json),
[Monokai](https://github.com/microsoft/vscode/blob/main/extensions/theme-monokai/themes/monokai-color-theme.json),
and [Tomorrow Night Blue](https://github.com/microsoft/vscode/blob/main/extensions/theme-tomorrow-night-blue/themes/tomorrow-night-blue-color-theme.json).
The native UI uses a small palette rather than importing VS Code's workbench or
theme loader. Both Solarized syntax themes come from the already bundled syntect
theme set; Quiet Light uses its InspiredGitHub syntax theme; Monokai and Tomorrow
Night Blue map the existing Ocean token categories into their corresponding
palettes. These are native interpretations, not exact VS Code token-scope parity.
No new dependencies or downloaded font assets were added.

## Pre-release feature verification

- Formatting and `cargo clippy --locked --all-targets -- -D warnings` pass.
- All 73 tests pass: 72 in the full optimized suite, followed by the additional
  font-menu interaction test. Tests exercise real menu selection, same-brightness
  syntax-palette changes, returning to System light/dark, independent text/code
  fonts, late multilingual fallback loading, missing-font failure, and reset.
- `cargo audit --ignore RUSTSEC-2026-0192` passes using the existing CI exception;
  the previously allowed unmaintained-bincode warning remains. `cargo deny check`
  passes with existing duplicate-dependency warnings. Cargo.lock is unchanged.
- Built `target/release/FastMarkdownViewer.exe` and checked its `--version` through
  `test-windows-ui.ps1 -Headless`. This check is process/version coverage, not a
  native keyboard-interaction test. Real renderer framebuffer captures of the
  [appearance fixture](../tests/fixtures/appearance.md) were inspected in Solarized
  Light and Monokai. The existing user-open debug process was left running.

Reproduce the visual check using the release `visual_check` example and setting
`FMV_VISUAL_THEME` to a theme label. The environment variable affects only that
example, not the application. For example, in PowerShell:

```powershell
$env:FMV_VISUAL_THEME = 'Solarized Light'
./target/release/examples/visual_check.exe ./tests/fixtures/appearance.md ./target/solarized-light.png
```

The feature-validation executable was built before the release version bump
and reports 0.1.3. It is not the published v0.1.4 artifact. Its SHA-256 is
`D593909F2C2F5C06F48041A4FB7759DCBFD8E2AE87911F354CE35DDA96A89764`.

## Exploratory size and CPU-layout check

Three runs per variant/fixture with `compare-reader.ps1`, `-Runs 0 -LayoutRuns 3`.
This checks default-theme/default-font CPU layout and process memory, not GPU
presentation, startup, or the cost of selecting every installed font. Raw data:
[appearance-layout-2026-09-09.csv](../experiments/architecture/appearance-layout-2026-09-09.csv).

| Fixture | Previous local build: scroll median ms | New build: scroll median ms | Previous/new one-tab private memory MiB |
|---|---:|---:|---:|
| ordinary-5k.md | 0.587 | 0.589 | 54.26 / 54.38 |
| gfm-math-images-100k.md | 9.997 | 9.853 | 66.71 / 66.96 |

Executable size: 16,820,224 → 16,885,760 bytes (+65,536 bytes). These small samples
do not suggest a material default-layout change, but are not a performance claim.
The previous local EXE and benchmark report v0.1.2 and predate the latest release;
their exact source state was not reconstructed, so this is not a clean A/B of
only the appearance patch. Both use the existing optimized local toolchain.
Fixture generation uses `experiments/architecture/fixtures/generate.ps1`.

Baseline EXE SHA-256:
`C7B952247312293E21E998965E38642635D41853F8F797958C35F25E1F840854`.
Baseline layout EXE SHA-256:
`6165F35558C6A884C55636F5CB0AB8643F6DD0BF5DE472AEEA090AEF10A837A3`.
New layout EXE SHA-256:
`064296A8FF57CC71AD481C38009470771E8A8522CAC949EEA451AB36B2B6E5CE`.
