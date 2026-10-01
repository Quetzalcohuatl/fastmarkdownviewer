# Appearance changes — local build, 2026-09-09

Version 0.1.4 adds five palettes to the existing System/Light/Dark choices:
Solarized Light, Solarized Dark, Quiet Light, Monokai, and Tomorrow Night Blue.
Settings has separate text and code font submenus. See the appearance section
of [README](../README.md) for available families. Since v0.2.0, appearance is saved on normal exit.

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

## Document width and Mermaid appearance — 2026-10-01

Settings → Document width offers Comfortable (960), the existing default, and
Fit window. Word wrap stays enabled by default and remains independent of width.
The width choice is shared by all windows and saved on normal exit; older saved
settings retain Comfortable. Prose reflows when the window or width setting changes.

Mermaid now receives the actual active palette and selected text font, including
the resolved system theme. Its canvas matches the document background; labels,
edges, nodes, and notes use reader colors. Explicit diagram-source colors are
preserved. Code font still controls the source displayed below the diagram.
The bundled proportional font is used for Default or a missing selected font.

Appearance changes discard obsolete cached diagrams and render replacements in
the existing isolated background helper. Completions from an older appearance
generation are discarded. Unchanged frames and window resizing reuse textures.
At this stage the 32-diagram / 32 MiB cache, 64 KiB input, 4-megapixel output,
and 10-second helper limits were unchanged. The subsequent progressive-rendering
change described below removes them. No dependencies or startup font scans were added.

### Verification

- All 99 tests pass with `cargo test --locked --all-targets` on Windows. Coverage
  includes 900 → 3440 → 900 logical-pixel resizing, restoring the comfortable
  column, the real Settings menu, independent wrapping, saved-width restoration,
  every palette's raster background/foreground, font changes, missing-font
  fallback, and cache invalidation.
- CI checks pass: `cargo fmt -p fast-markdown-viewer -p fmv-macos-events -- --check`
  and `cargo clippy --locked --all-targets --no-deps -- -D warnings`.
  Broader workspace formatting/lint checks encounter pre-existing issues in the
  bundled rendering crates; those unrelated files were left unchanged.
- Inspected production OpenGL framebuffer captures in Monokai and Solarized
  Light. Diagram backgrounds blend into the document and labels are readable.
- The shared implementation applies on Windows, macOS, and Linux; this change
  has not yet been run on a Mac or Linux desktop.

### Exploratory performance comparison

Same Windows machine, Rust 1.95.0, optimized builds with the unchanged release
profile. Baseline is commit `5cb8ff3`; after is that revision plus this change.
Builds and test runs finished before sampling. These small warm-filesystem samples
are regression checks, not cold-start or GPU-presentation claims.

Nine fresh helper processes per diagram/variant after one excluded warmup, with
randomized variant order. Timing includes process launch, source reading, layout,
font loading, rasterization, PNG encoding/writing, and process exit. It excludes
the parent UI's polling delay, PNG decoding, and texture upload.

| Diagram | Before (ms) | After light (ms) | After Monokai (ms) | Monokai + Georgia (ms) |
|---|---:|---:|---:|---:|
| Flowchart | 77.23 | 77.53 | 78.74 | 76.50 |
| Sequence | 77.85 | 77.40 | 77.52 | 77.04 |

Raw data: [helper timings](../experiments/architecture/appearance-mermaid-2026-10-01.csv).
Reproduce with `experiments/architecture/compare-mermaid-appearance.ps1`, passing
`-Before`, `-After`, and `-OutputDirectory`. The script creates both fixtures.

Three reader-layout runs per variant/fixture, default width and wrapping, at
900 × 700 logical pixels using `compare-reader.ps1 -Runs 0 -LayoutRuns 3`:

| Fixture | Before scroll-layout median (ms) | After (ms) |
|---|---:|---:|
| ordinary-5k.md | 0.94 | 0.50 |
| gfm-math-images-100k.md | 8.70 | 8.82 |

Raw data: [layout and memory samples](../experiments/architecture/appearance-layout-2026-10-01.csv).
The small-fixture timing improvement is not treated as a speedup claim. These
measurements show no material regression in the sampled workloads; ultrawide
behavior is covered by layout tests, not a separate timing claim. Executable
size changed from 20,802,560 to 20,854,784 bytes (+52,224 bytes).

## Default theme and document text sizing — 2026-10-01

New settings default to Monokai. Existing saved theme choices, including System,
remain unchanged. Missing or invalid settings fall back to Monokai.

Settings → Document text size is a 75–200% slider with an editable numeric value;
100% retains the existing typography. It scales prose, headings, table text,
inline/fenced code, and math. Document text reflows inside the chosen column
width. Menus, tabs, outline controls, images, and Mermaid diagram dimensions
remain unchanged. The existing whole-interface zoom controls and shortcuts are
retained and now labeled Interface zoom. The two settings are independent,
shared across windows, and saved on normal exit.

Scaling is scoped to the document's text styles, with no global font rebuild,
file reload, or Mermaid re-render. Math reuses its existing SVG at the chosen
display size. The unchanged 100% setting skips style mutation entirely.

All 103 tests pass on Windows, including actual slider interaction, proportional
scaling of headings/body/table/code text, unchanged toolbar fonts and zoom,
reflow, repeated-frame stability, reset to 100%, preference restoration, old
settings compatibility, bounds checking, and the actual fresh-window Monokai
palette. The CI formatting and application Clippy commands above also pass.

For repeatable framebuffer checks, the `visual_check` example accepts
`FMV_VISUAL_TEXT_SIZE=100` or `150`. Its default palette now matches the native
application's Monokai default; `FMV_VISUAL_THEME` still overrides it.

Inspected 100% and 150% production OpenGL captures: document typography and
math enlarge and reflow, while the toolbar, tabs, and outline remain unchanged.

An additional three-run optimized CPU-layout comparison against the preceding
width/Mermaid build, using the same generated fixtures and 900 × 700 viewport,
recorded the following medians at the unchanged 100% text size:

| Fixture | Before (ms) | After (ms) |
|---|---:|---:|
| ordinary-5k.md | 0.47 | 0.64 |
| gfm-math-images-100k.md | 11.12 | 8.81 |

[Raw layout/memory samples](../experiments/architecture/text-size-layout-2026-10-01.csv).
The mixed timing changes in this small sample are not a speedup claim or an
end-to-end startup benchmark. No builds or tests overlapped sampling.
Before executable SHA-256:
`B490A87963EF7BF808412E5761B346B47AE2645E5EA001AF6FCFAE25D2E624E5`.
After executable SHA-256:
`013BCF55F52A93660940F3F87EB212008E49B727B0B6DB1C131ED0543745557E`.

## Progressive diagrams and size-cutoff removal — 2026-10-01

The reader no longer applies MB cutoffs to Markdown, Mermaid source/generated
SVG, or local/downloaded image inputs. The four-megapixel Mermaid surface check
and forty-megapixel image check are removed, as is the raster decoder's default
allocation budget. These changes remove policy refusals; they do not make memory,
temporary disk, or underlying renderer dimensions unlimited. The complete current
boundaries are listed in [README](../README.md#limits-and-compatibility).

Mermaid starts only when visible and renders in the existing isolated helper,
one diagram at a time. Geometry is published after layout, followed by atomic
512-pixel tiles at display resolution. Only visible tiles are requested by the
viewer; older GPU tiles can be evicted by the image cache. The helper allocates
one tile bitmap at a time. A Cancel control replaces the automatic ten-second
deadline. Closing/reloading a tab or changing its diagram appearance cancels stale
work. Temporary tiles are cleaned in the background when no longer needed.

Images decode in the background, then fit the requested display size and the
graphics hardware's maximum texture dimension. SVG rasterization uses that size
directly; raster decoding still needs its full source image before resizing.
Original source dimensions are preserved for document layout. Cache byte budgets
are soft eviction targets and retain the newest individual result even if it
exceeds the target, avoiding an endless load/eviction loop for a single large item.

All **109 application tests pass** on Windows, including the former size failure
cases, complete large-diagram tile coverage, offscreen deferral, stale helper
results, and cache behavior. The CI formatting and application Clippy commands
pass; existing dependency warnings remain. Built the optimized executable and
inspected actual OpenGL screenshots of a small flowchart and the large sequence
diagram with Monokai. macOS and Linux were not run locally for this change.

### Release measurements

Windows x64; five measured fresh helper processes per fixture/variant after one
excluded warmup round, randomized variant order. No builds/tests overlapped
sampling. The small fixture is a three-node flowchart; the large fixture is a
70-message sequence diagram with long labels, naturally 2,357 × 2,974 pixels
(about seven megapixels). Process launch, font loading, layout, rasterization,
PNG encoding, disk writes, and exit are included in total time. First-tile times
are disk-availability observations with roughly five-millisecond polling
granularity, not first pixels presented on screen; the viewer polls its helper
every fifty milliseconds and image decode/presentation add latency.

| Fixture / mode | Previous helper median total (ms) | New first tile (ms) | New total (ms) |
|---|---:|---:|---:|
| Small / 800-pixel display width | 76.67 | 27.47 | 76.44 |
| Small / natural resolution | 76.67 | 27.53 | 77.14 |
| Large / 800-pixel display width | Rejected | 59.16 | 169.42 |
| Large / natural resolution | Rejected | 41.97 | 343.51 |

Display-width tiling produces an 800 × 1,010 image for the large fixture;
natural-resolution tiling exercises the former four-megapixel cutoff directly.
Sampled peak working set was around 31 MiB for both successful large-diagram
variants. Short process lifetimes and polling make this a diagnostic lower bound,
not a process memory guarantee. These small samples support keeping the changes;
they do not establish arbitrary-document load times or a general speedup.

[Raw helper samples](../experiments/architecture/mermaid-tiles-2026-10-01.csv).
Reproduce with `experiments/architecture/compare-mermaid-tiles.ps1`, passing
`-Before`, `-After`, and a new `-OutputDirectory`. The script generates fixtures
and has a benchmark-only sixty-second deadline; the application does not.

Reader CPU-layout samples used the existing `compare-reader.ps1 -Runs 0`, default
settings, and a 900 × 700 viewport. The initial three-run medium-fixture result
was noisy, so six further runs were retained together with all original samples.
Nine-run medians and ranges of each process's scroll median:

| Fixture | Before median / range (ms) | After median / range (ms) |
|---|---:|---:|
| ordinary-5k.md | 0.55 / 0.47–1.20 | 0.52 / 0.46–1.22 |
| gfm-math-images-100k.md | 13.86 / 8.71–16.47 | 9.31 / 8.38–13.65 |

[All layout and memory samples](../experiments/architecture/large-diagram-layout-2026-10-01.csv).
The overlapping ranges and unchanged ordinary-document layout code do not support
a speedup claim. These are CPU-layout checks, not end-to-end document-open timing.
The executable grew from 20,902,912 to 20,952,064 bytes (+49,152 bytes).
Before SHA-256:
`013BCF55F52A93660940F3F87EB212008E49B727B0B6DB1C131ED0543745557E`.
After SHA-256:
`9CEE606AB5E2F4395B0F8CCD832386673E166E70FF072662140C4E5EA4DA7264`.

## Deferred large-code highlighting — 2026-10-01

Large tagged code blocks now use the same plain-first, background-highlighting
path as small blocks. The former 256 KiB input, 16,384-section, and 8 MiB
formatted-output fallbacks are removed. An individual completed result can exceed
the cache's soft target. The request queue holds four items and the completed
result queue holds one, providing backpressure when a tab is not being drawn.
Changing palette/font size or dropping the document cache cancels obsolete work
between lines. Repaint notifications target the window that requested the work,
including detached windows. Unknown languages still use plain text; computation
and memory scale with the source, and a single complex line can delay cancellation.

The new integration regression uses an 820,000-byte Rust block. It verifies an
initial plain frame, eventual colors through the final byte, a result exceeding
both old formatting cutoffs, and reuse of that result on the next frame. Two
backend tests check cache-release and obsolete-generation cancellation.
Windows passes 110 application tests plus both backend tests. Local Ubuntu 24.04
under WSL passes 107 application tests plus both backend tests; the differences
are existing platform-specific tests. Formatting and application Clippy pass.
Local WSL builds used `RUSTFLAGS=--diagnostic-width=140` to avoid a Rust 1.95
diagnostic-rendering panic; this changes diagnostic formatting, not runtime code.

Native macOS Apple Silicon, Intel Mac, and Ubuntu validation runs against the
same source snapshot in [desktop run 36908874422](https://github.com/Quetzalcohuatl/fastmarkdownviewer/actions/runs/36908874422),
commit `a630223ccb7b4db872fedaac17a18ad746d2c11d`. The native matrix includes the
appearance, width, text-size, image-size, Mermaid-tile, and highlighting
regressions, with large-content graphics fixtures added for both Mac architectures
and Linux X11/Wayland.

All three native jobs completed successfully, including source-package/registry
installation, packaged-binary installation, native graphics captures, Finder
opening/session checks on both Macs, and Linux package/session checks. Inspected
the large-code and large-Mermaid screenshots from both Mac architectures and
Linux X11/Wayland. Candidate packages were downloaded and verified against their
SHA-256 files. These are validation artifacts, not a newly published release.

Actual Windows/OpenGL and local Linux/X11/Mesa screenshots were inspected before
and after highlighting: plain text becomes Monokai-colored text without reflow.
The large tiled diagram also renders with the Monokai canvas on Linux.

Three fresh runs of the optimized Windows highlighting test, with no builds or
other local test work overlapping, measured:

| Sample | Initial plain layout (ms) | Completed test (ms) |
|---|---:|---:|
| 1 | 8.68 | 1,149.34 |
| 2 | 7.92 | 1,248.08 |
| 3 | 7.96 | 1,148.12 |

[Raw samples](../experiments/architecture/progressive-highlighting-2026-10-01.csv).
Reproduce by building `cargo test --locked --release --test highlighting --no-run`
and running the resulting test executable three times with `--nocapture`.
These are headless CPU-layout measurements, not file-open or application-startup
times. Completion includes the test's 100 ms polling and final cache-retention
check; this is a small repeated-source fixture, not a general performance claim.
The Windows executable SHA-256 is
`A4949097897F99F3FBD35A475C7AE290314BCA2EAF7E265F13AEDB95D435C383`.
