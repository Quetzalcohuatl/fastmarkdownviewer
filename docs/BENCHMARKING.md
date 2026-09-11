# Benchmark methodology

Startup claims are based on fresh-process launches, never an in-process loop or a single best run.

## Fixed inputs

Run `experiments/architecture/fixtures/generate.ps1` to generate deterministic documents of approximately 5 KiB, 100 KiB, and 2 MiB. These repeat GFM, math, code, and multilingual text; despite its historical filename, the medium fixture contains no images. A separate `images.md` fixture references 24 generated local SVGs. The large fixture stresses full-document layout.

## Exploratory reader comparisons

[The v0.1.0 comparison](../experiments/architecture/reader-results.md) records five native launches and three CPU-layout/lifecycle runs per variant and fixture, with raw CSVs. Use `experiments/architecture/compare-reader.ps1` with optimized viewer binaries and matching `reading_benchmark` examples. This small same-machine comparison helps catch regressions; it does not replace the formal backend selection protocol below. Native window discovery is a startup proxy, and headless layout excludes GPU work and presentation.

## Machine record

The [September 11 competitor study](../experiments/competitors/README.md) compares
Windows and Linux VM process resources across open-source viewers/editors, with
raw samples, pinned downloads, rendering observations, and a sourced feature
matrix. It is an exploratory five-sample study using window-creation proxies and
fixed resource checkpoints, not the first-content certification protocol below.

The [September 9 rendering-fix comparison](../experiments/architecture/layout-fixes-2026-09-09.md) records a focused v0.1.2 before/after CPU-layout and executable-size check, with raw samples and limitations.

Copy `experiments/architecture/hardware.example.toml` to `hardware.toml` and record Windows build, CPU, physical memory, GPU/driver, display scale, power mode, security software state, Rust version, commit, and exact build commands. Do not publish a device serial number or username.

## Protocol

1. Build locked release binaries for egui/Glow, egui/wgpu, and the Direct2D prototype.
2. Reboot; close interactive applications; connect AC power; use the documented power mode.
3. Open each fixture once to warm only the filesystem cache, then close the process.
4. Randomize variants and run 30 fresh processes per variant/fixture.
5. Capture process-entry to first content-bearing present with Windows Performance Recorder/Analyzer ETW present events. The included PowerShell harness records a coarser visible-window proxy, size, first-idle working set, and peak working set; it must not be mislabeled as present timing.
6. Report every raw sample, median per fixture, and the geometric mean of those three medians. Report raw EXE size, ZIP-compressed size, first-idle working set, and peak working set separately.

Direct2D is selected only if its geometric-mean median first-content time is at most half the faster egui result and it has no blocker for selection, accessibility, math, images, tables, or high-DPI rendering. If egui Glow and wgpu are within 10%, select Glow for its smaller dependency surface.

Absolute timing is informational, not a release gate. Regression comparisons are valid only on the same recorded machine and protocol.
