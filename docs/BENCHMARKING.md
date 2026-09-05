# Benchmark methodology

Startup claims are based on fresh-process launches, never an in-process loop or a single best run.

## Fixed inputs

Run `experiments/architecture/fixtures/generate.ps1` to generate deterministic documents of approximately 5 KiB, 100 KiB, and 2 MiB. The medium fixture includes GFM, math, local images, and remote-image placeholders. The large fixture stresses layout and virtualization.

## Machine record

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
