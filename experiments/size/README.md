# Image-loader size reduction — 2026-09-15

Remove redundant framework image loaders, retaining FMV's existing bounded byte,
image, and texture loaders. The Markdown component's `load-images` and `svg`
features install framework loaders; image rendering itself does not require
those features when the application supplies its own loaders.

The change removes nine package/version entries from Cargo.lock, including
resvg/usvg 0.45.1, tiny-skia 0.11.4, and png 0.17.16. FMV keeps resvg 0.48.1.
No dependencies were upgraded or added. Fonts and supported formats stay intact.

## Windows executable size

Both builds used Rust 1.95.0 on the same Windows 11 machine with the unchanged
release profile: opt-level 3, thin LTO, one codegen unit, aborting panics, stripped
symbols. Commands used the default host target, without an explicit `--target`.

| Build | Executable bytes | MiB |
|:--|--:|--:|
| Before, source f451f447 | 19,476,480 | 18.574 |
| After, redundant loaders removed | 18,371,584 | 17.520 |
| Saving | **1,104,896** | **1.054 (5.7%)** |

These are executable file lengths, not installer download sizes, allocated disk
blocks, or total system dependencies. The published v0.2.2 Windows EXE is
19,485,184 bytes; it is not substituted for the local before-build in this comparison.
Build hashes and removed packages are in [windows-size.json](windows-size.json).

## Reading performance check

The existing `reading_benchmark` harness measures CPU-side scrolling and exercises
opening/closing five tabs. It does not measure startup, displayed frames, or GPU
work. Each run contains 30 scrolling frames. Results below are medians across
the per-run measurements, not percentiles pooled across all frames.

The first run used five repetitions per build/fixture, randomized with seed
20260915. Its small-input medians were 0.8023 → 0.7476 ms; medium-input medians
were 11.4811 → 12.9395 ms. The mixed result prompted a separate confirmation
with 15 repetitions per build/fixture and seed 20260916; both datasets are retained.

| Confirmation workload | Before median (ms) | After median (ms) | Before per-run p95 median (ms) | After per-run p95 median (ms) |
|:--|--:|--:|--:|--:|
| ~5 KiB | 0.7197 | 0.7313 | 2.4905 | 2.4554 |
| ~100 KiB | 12.2424 | 12.1290 | 35.5292 | 41.8543 |

Median reading cost is similar in the confirmation sample. Tail timings are
noisy and the medium-input p95 summary is higher for the candidate; this does
**not** certify identical performance or establish a speed improvement. Medium
per-run medians ranged 10.6276–14.4931 ms before and 10.9789–18.6816 ms after.
The tests ran on an ordinary Windows desktop with background activity, without
CPU isolation. No compiler optimization setting changed.

Hardware: Ryzen 9 9950X, 32 logical processors, approximately 254 GiB RAM;
Windows 11. Inputs come from `experiments/architecture/fixtures/generate.ps1`;
the medium filename mentions images, but that fixture contains Markdown, code,
math, and Unicode, with no image links. Fixture and harness hashes accompany
every raw row:

- [Initial 20 runs](windows-reading.csv)
- [Confirmation 60 runs](windows-reading-confirmation.csv)

## Feature checks

- The full application test suite passes locally, including image permissions,
  reload/cache behavior, rendering, Mermaid, and multilingual glyph coverage.
- New loader-pipeline tests load included and local SVG, rendered math SVG,
  PNG, JPEG, GIF, and WebP through the registered application loaders.
- Optimized before/after framebuffer captures of `docs/demo/technical-guide.md`
  and `tests/fixtures/multilingual-ui.md` (Find: 中文) are pixel-identical at
  Windows 150% scale, Light theme. These exercise code, equations, a diagram,
  multilingual headings, and search highlights; they are not exhaustive visual coverage.
- Formatting, Clippy, dependency advisories, licenses, bans, and sources pass
  with the existing policy exceptions.
- Native macOS and Ubuntu acceptance runs in the pull request's desktop workflow.
  Consult its checks before treating these platforms as validated for this change.

## Reproduce the timing check

Save `FastMarkdownViewer.exe`, `reading_benchmark.exe`, and `visual_check.exe`
from an optimized before-build, then build the same targets after the change.
Use the same toolchain, host, fixtures, and release configuration for both:

```powershell
cargo build --locked --release --bin FastMarkdownViewer --example reading_benchmark --example visual_check
experiments/architecture/fixtures/generate.ps1 -OutputDirectory target/size-study/fixtures
python experiments/size/measure-reading.py --baseline C:/before/reading_benchmark.exe --candidate C:/after/reading_benchmark.exe --fixtures target/size-study/fixtures --output target/reading.csv --repetitions 15 --seed 20260916
```

This is the dependency-removal step. The subsequent
[compiler profile comparison](compiler-profiles.md) measures full LTO and
size-oriented optimization separately. Font compression remains a possible follow-up.
