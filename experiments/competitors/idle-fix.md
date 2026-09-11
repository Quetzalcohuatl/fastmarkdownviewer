# Sustained CPU investigation — 2026-09-11

The application continuously rescheduled its own frames by sending the native
window title during every UI pass. In egui 0.36.1,
`Context::send_viewport_cmd_to` unconditionally requests a repaint, including for
an unchanged `Title` command. The next repaint sent the same title again.

Each `ViewerWindow` now remembers its last native title and sends a command only
when that title changes. Synchronization happens after the UI handles navigation
and rename actions, so the resulting title is sent in the same pass. Detached
windows have independent title state. This removes the feedback loop without
throttling frames or suppressing input, animation, image, math or syntax-worker
repaint requests.

## Evidence and validation

The new idle regression test failed before the fix with a repaint delay of `0ns`
after twelve input-free frames. It passes after the fix for both an open document
and the empty landing window. Additional tests verify title changes on opening,
tab switching, renaming and closing the last tab, plus detached-window renaming
and return to an idle repaint delay.

Windows: all 82 tests passed (`cargo test --locked --all-targets`), Clippy passed
with `-D warnings`, and the optimized release build passed. The existing Mermaid
dependency emits warnings; no new application warnings were introduced. A native
spot check displayed the table, math, code and alert, and Find responded after
the viewer had settled. As in the original study, the first Windows capture was
blank; a fresh frame after Find displayed the document.

The fixed Linux binary also displayed the small fixture, including math and code,
without an input action in its pilot. Captures: [Windows](evidence/idle-fixed-windows.jpg)
and [Linux](evidence/idle-fixed-linux.png).

## Before/after CPU measurements

These follow-up samples are separate from the historical competitor dataset.
`idle-cpu.py` launches each release binary three times in seeded randomized order
on `ordinary-5k.md`, waits ten seconds, then samples process-tree CPU over five
seconds. Both versions use the same fixture and benchmark environment on each OS.
Platforms were measured serially after compilation finished. 1000 ms CPU/s equals
one fully occupied core; zero means below the measurement resolution.

Raw samples: [Windows](idle-fix-windows.csv), [Linux VM](idle-fix-linux.csv).

| Platform | Before median CPU ms/s (range) | Fixed median CPU ms/s (range) | Samples per version |
|---|---:|---:|---:|
| Windows | 1088.5 (1069.9–1141.5) | 0.0 (0.0–0.0) | 3 |
| Linux software-rendered VM | 2246.1 (2192.0–2250.2) | 0.0 (0.0–2.0) | 3 |

The fixed build returns to approximately idle CPU in this workload. These samples
do not establish a precise zero-power state or performance on every document.

Windows baseline is the original benchmark executable (source `9d45205`); the
candidate is a local locked release build of the title-change fix. Linux baseline
is the original CI-built executable at that source commit; the candidate is built
with Rust 1.95.0 in Ubuntu WSL and tested in the same 4 GiB Xfce/llvmpipe VM.
This Linux build provenance differs, so the regression test and isolated code
change supplement the measurements. Executable and fixture hashes are in the CSVs.

The first Linux build attempt hit a Rust 1.95.0 panic inside the warning snippet
renderer (`annotate_snippets`); changing terminal width did not resolve it.
`cargo build --locked --release --message-format=short` avoided that formatting
path and built successfully, without changing compiler optimization flags,
application source, dependencies or warning levels.

For reproduction, use the same config schema as `measure.py`, with two `apps`
entries named `before` and `after`, their binary paths and dedicated benchmark
environment. Run `python experiments/competitors/idle-cpu.py config.json --output results.csv`.
These are idle CPU measurements, not first-content latency, battery life, or a
rerun of every competitor case. No release tag is created by this fix.
