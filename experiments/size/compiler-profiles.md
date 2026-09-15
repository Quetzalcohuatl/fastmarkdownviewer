# Compiler profile comparison — 2026-09-15

This experiment starts from bf8bbc15468f2b2c4a8f984fc63187cdec8739fd,
after the duplicate image-loader cleanup. Application source, dependencies,
features, and fonts are identical in all four builds. The only variables are
the release profile's `opt-level` and `lto` settings.

**Selected: opt-level 3 with full (`fat`) LTO.** It saves 931,328 bytes (5.1%)
against the current Windows executable, with similar reading medians in this
sample. The smaller `s` and `z` builds incur substantial reading costs and are
not selected. No feature or font coverage was removed.

## Results

| Profile | Executable bytes | MiB | Saving | ~5 KiB reading median (ms) | ~100 KiB reading median (ms) |
|:--|--:|--:|--:|--:|--:|
| 3 / thin, baseline | 18,371,584 | 17.5205 | — | 0.7716 | 11.8194 |
| **3 / fat, selected** | **17,440,256** | **16.6323** | **5.1%** | **0.7573** | **11.7458** |
| s / fat | 15,005,696 | 14.3105 | 18.3% | 0.9562 | 15.4335 |
| z / fat | 14,757,376 | 14.0737 | 19.7% | 1.1643 | 20.6073 |

| Profile | ~5 KiB per-run p95 median (ms) | ~100 KiB per-run p95 median (ms) |
|:--|--:|--:|
| 3 / thin | 2.5912 | 39.9707 |
| 3 / fat | 2.3238 | 36.0701 |
| s / fat | 3.4644 | 48.9879 |
| z / fat | 3.8805 | 57.2152 |

The selected profile's median differences are small (−1.9% and −0.6%); they
do not establish a speed improvement. The `s`/`z` medium-input medians rise
30.6%/74.4%. All runs reached the same scroll offsets for each fixture and
completed the tab lifecycle assertions. See the limitations below before
generalizing these CPU-only results to responsiveness.

[All 120 timing runs](compiler-reading.csv) and
[binary hashes, sizes, and visual comparisons](compiler-windows.json) are retained.

The combined Windows saving from the two passes is 2,036,224 bytes (10.5%)
against the preceding experiment's 19,476,480-byte local build. The published
v0.2.2 EXE is a separate 19,485,184-byte artifact and has not been replaced.

## Rendering and acceptance

The baseline and all three profiles produced pixel-identical captures of
`docs/demo/technical-guide.md` and `tests/fixtures/multilingual-ui.md` in both
Light and Dark themes at Windows 150% scale. The multilingual capture includes
Chinese Find text. The production EXE's standalone Mermaid helper also produced
an identical diagram for the selected profile. These are sampled visual checks,
not exhaustive coverage of all documents and languages.

The selected profile is also checked with the full optimized Windows test suite
and the pull request's native macOS ARM/Intel and Ubuntu acceptance workflows.
Their completion and native package measurements are recorded in the pull request.

## Method

Windows 11, Ryzen 9 9950X, 32 logical processors, approximately 254 GiB RAM;
Rust 1.95.0 (59807616e), LLVM 22.1.2, default x86_64-pc-windows-msvc host target.
All builds retain one codegen unit, aborting panics, stripped symbols, and the
repository's static CRT setting. Build commands omit an explicit `--target`.

Build each variant, retaining all three outputs before starting the next:

```powershell
# baseline: 3 / thin; fat-3: 3 / fat; fat-s: s / fat; fat-z: z / fat
$env:CARGO_PROFILE_RELEASE_OPT_LEVEL = '3'
$env:CARGO_PROFILE_RELEASE_LTO = 'fat'
cargo build --locked --release --bin FastMarkdownViewer --example reading_benchmark --example visual_check
# Save target/release/FastMarkdownViewer.exe and the two EXEs in
# target/release/examples/ under a directory named for this variant.
Remove-Item Env:CARGO_PROFILE_RELEASE_OPT_LEVEL
Remove-Item Env:CARGO_PROFILE_RELEASE_LTO
```

The baseline uses the preserved, unchanged build from the preceding experiment;
its EXE SHA-256 is e2c94a686806b7d69247767f16223bc2ec66302a42dafa2c6f644cf4b7a60b95.
Rebuild with `3` / `thin` to reproduce that configuration. Binary metadata is
retained alongside the raw timing data. Executable byte lengths measure neither
installer downloads nor allocated filesystem blocks.

After all compilation and visual captures finish, run the existing CPU-side
reading harness with 15 repetitions per variant and fixture, randomized across
all four variants using seed 20260917:

```powershell
experiments/architecture/fixtures/generate.ps1 -OutputDirectory target/size-study/fixtures
python experiments/size/measure-profiles.py --binary baseline=target/size-study/compiler/baseline/reading_benchmark.exe --binary fat-3=target/size-study/compiler/fat-3/reading_benchmark.exe --binary fat-s=target/size-study/compiler/fat-s/reading_benchmark.exe --binary fat-z=target/size-study/compiler/fat-z/reading_benchmark.exe --fixtures target/size-study/fixtures --output experiments/size/compiler-reading.csv --repetitions 15 --seed 20260917
```

Each process performs 30 scrolling frames, plus opening and closing five tabs.
Reported summaries are medians across per-run measurements; the p95 summary is
the median of each run's p95, not a pooled percentile. The ~100 KiB fixture's
filename mentions images, but it contains Markdown, code, math and Unicode,
without image links. These measurements exclude startup, GPU work, and displayed
frame latency. Normal desktop background activity and asynchronous math work
make tail timings noisy. Binary and fixture hashes accompany every raw row.
