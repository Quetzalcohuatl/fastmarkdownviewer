# Measurement environment

Date: 2026-09-11. This is an exploratory desktop resource comparison, not the
30-launch, first-content/ETW performance certification described in the main
[benchmarking protocol](../../docs/BENCHMARKING.md).

## Windows

- Windows 11 Pro 10.0.26200.
- AMD Ryzen 9 9950X, 16 physical cores / 32 logical processors.
- OS-visible RAM: 265,920,948 KiB (about 253.6 GiB).
- NVIDIA RTX 5080 driver 32.0.15.9186; AMD integrated graphics driver
  32.0.21030.2001. Per-application GPU selection was not instrumented.
- Balanced power scheme. Security software unchanged; background desktop apps
  were present. Display scale and window dimensions were app defaults, not normalized.
  Launches were programmatic; foreground focus and occlusion were not normalized.
  CPU conclusions describe this launch/checkpoint workflow, not every interactive state.
- Python 3.12, psutil 7.2.2, pythonnet 3.1.0.
- Installed WebView2 runtime directories: 151.0.4129.107 and 152.0.4191.66;
  the selected runtime version was not logged per sample.
- FMV rebuilt with `cargo build --locked --release` at source commit
  `9d452057d08d8f253ede96679b532e55be46f901` before the retained run.
  Earlier pilot measurements of an older local binary are excluded.

## Linux

- Ubuntu 24.04 guest, kernel 6.8.0-139-generic, Xfce/X11 desktop, 1280×800.
- QEMU/KVM guest inside Ubuntu WSL2 on the same Windows host; 4 virtual CPUs,
  4 GiB configured RAM, virtio disk/display, no physical GPU passthrough.
- Mesa 25.2.8, llvmpipe LLVM 20.1.2 software renderer; accelerated: **no**.
- WebKitGTK 4.1 package 2.52.6-0ubuntu0.24.04.1; Python 3.12.3,
  psutil 5.9.8, xdotool 3.20160805.1 (Ubuntu package 1:3.20160805.1-5build1).
- FMV installed from the experimental CI `.deb` built at the same source commit.
  Executable SHA-256:
  `60dc3377d0280fb2e48f54660d7cfebc5ee5f1334d2e23be9a844bde60045d7a`.
- Other apps use official release assets. MarkMello's AppImage was extracted;
  MarkText's tar archive used a correctly installed Chromium sandbox helper.
- These are **Linux VM results**, not native-hardware Linux results. Windows and
  Linux memory definitions and graphics paths differ; do not rank platforms.

## Reference-app configuration

VS Code 1.130.0 (commit `1b6a188127eeaf9194f945eb6eb89a657e93c54c`)
uses a dedicated profile, no user extensions, optional onboarding disabled, and a
minimal local adapter invoking its built-in Markdown preview. The adapter declares
support for untrusted workspaces; Restricted Mode itself is left enabled. The
Windows pilot retained source and preview tabs; the Linux pilot showed the preview
tab. This added adapter and the IDE's built-in services
are included in measured resources. It is not a pure Markdown parser benchmark.

Obsidian 1.13.7 uses a generated vault per sample with only that sample's document
and local SVG assets, plus an explicit reading-mode workspace. Full-vault indexing
is therefore excluded except for the selected document/resources. Renderer exits
are invalid samples even if the main application window remains open.
Linux Obsidian also displayed a keyring-creation prompt over its reading view.
Its raw attempts are retained but excluded from the summarized resource comparison;
no credentials were supplied or keyring settings altered.

Typora uses a persistent dedicated profile and its ordinary unregistered/trial
state, without clearing licensing or trial data. Windows 1.14.10 was extracted
from the official installer with innoextract; the administrative installer was
canceled. Linux's official package was 1.14.9. These different patch versions are
reported explicitly. The one-time introduction was dismissed before measurement.

The smaller Windows apps and reference apps were measured in two successive
randomized batches. All image cases were subsequently replaced with a separate
randomized batch on each OS after fixing machine-specific image links. These
successive batches introduce additional between-batch drift.

The separate CLI workload uses Glow 3.0.0 (`-s dark -w 80`) and mdcat 2.7.1
(`--ansi --local --columns 80 --no-pager`) on both platforms, with ten launches per
fixture. Standard input is explicitly `/dev/null`/NUL and standard output is
captured through a pipe. The duration includes
process launch, parsing/formatting and writing that output, but no terminal paint,
interactive scrolling or desktop resources. Output byte counts are retained;
the tools produce different formatting and output volumes. After timing, the
harness strips ANSI control sequences and verifies that the fixture's heading is
present. This catches empty output; it is not full formatting conformance.

An initial Linux CLI batch inherited SSH's empty input pipe. Glow returned two
newline bytes and exit code zero, so those readings were discarded. Both CLI
batches were rerun with explicit null stdin and content validation. The corrected
relative-path image cases replaced the earlier image rows on Windows as well.

## Protocol and limitations

Five fresh process launches per application and fixture; seeded randomized order
(`20260911`). Four deterministic fixtures from
`../architecture/fixtures/generate.ps1`: 5,214-byte small document, 102,858-byte
medium document, 2,097,450-byte stress document, and 676-byte document referencing
24 local SVG files. The medium filename contains “images” for historical reasons,
but the repeated text fixture has no images. `portable-images.py` replaces the
generator's machine-specific Windows file URIs with relative SVG references before
measurement/copying to Linux. The same final Markdown bytes and SVG files are used
on both platforms. All files are synthetic and local.
Image loading may be deferred to the visible viewport; the checkpoint does not
certify that all 24 images were decoded or displayed in each app.

Dedicated per-sample profile directories isolate session/cache state for apps
that honor environment/CLI profile overrides. No user profile is deleted. OS file
caches are not flushed; this is filesystem-warm testing. App defaults, including
different window sizes, sidebars, themes and supported syntax, are retained.
Network access was available; app startup/update traffic was not controlled or
instrumented. The fixtures themselves contain no external image downloads.

Window detection uses .NET `Process.MainWindowHandle` on Windows and xdotool's
visible-window PID lookup on Linux. This is **window creation**, not first content
or time until usable. The Windows property does not independently prove every
returned handle is an unobscured content window. Screenshots/interaction checks
were performed separately, not timed in every run.

Resources are sampled three seconds after window detection, then again one second
later. The reported working set/RSS is the sum of live descendant processes at
the second checkpoint. Shared pages can be counted repeatedly in that sum. Windows
private memory means committed private bytes; Linux private memory is USS. Neither
includes driver allocations, GPU memory, services outside the descendant tree, or
earlier exited children. These are checkpoint values, not peaks.

CPU is summed user+kernel CPU-time change across the live process tree divided by
the actual sample duration. 1000 ms CPU/s is one full core; it is not 100% of a
32-thread machine. Children exiting or starting between snapshots can distort
this delta. Values are not a battery-life measurement or proof of steady-state idle.
An app may still be doing background work on the stress document at the checkpoint.
Negative CPU deltas from disappearing descendants are retained in raw data but
excluded from medians; the dataset audit reports their count. Zero displayed CPU
can also mean activity below the timer resolution, not literally no instructions.

The separate `late-cpu.py` diagnostic waits 30 seconds after process launch, then
measures five seconds on the small fixture for FMV and aydiler. There is one
diagnostic sample per app/platform. It is not mixed into the repeated checkpoint
medians. The guest has no swap; allocation pressure can terminate large workloads.

Five samples support exploratory medians and ranges, not stable p95 claims. No
first-content timing, scroll frame rate, search latency, power use, cold-boot disk
load, or whole-document completion timing was established by this harness.
