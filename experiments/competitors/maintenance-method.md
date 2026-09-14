# Windows maintenance comparison — September 13, 2026

This rerun replaces historical numbers in the main README with fresh measurements
of the published **FMV 0.2.1** and the same pinned Windows competitors. A second
FMV row checks the dependency updates and multilingual input fix at source
`dda53cf`. It is a development build, not another published release.

## Reproduction and provenance

- Same Windows 11 Pro 10.0.26200 machine, Ryzen 9 9950X, approximately 253.6 GiB
  OS-visible RAM, RTX 5080/AMD graphics, and Balanced power scheme recorded in
  [environment.md](environment.md). Desktop background activity remains a source
  of noise; no reboot, cold-disk-cache protocol, or GPU isolation was performed.
- The baseline is the verified public
  [FMV 0.2.1 executable](https://github.com/Quetzalcohuatl/fastmarkdownviewer/releases/tag/v0.2.1).
  The candidate was built locally with Rust 1.95.0 and
  `cargo build --locked --release --bin FastMarkdownViewer --example visual_check`.
  Compilation and local visual captures finished before the measurement batch.
- [Pinned downloads](downloads.json) and [reference-app setup](environment.md#reference-app-configuration)
  identify competitor versions. These are comparisons to those exact versions,
  not a claim that every competitor is its latest release. Windows mdview-zig is
  0.2.0; the Linux 0.4.0 build from the earlier study is not used here.
- Combine the apps in [windows.example.json](configs/windows.example.json) and
  [windows-anchors.example.json](configs/windows-anchors.example.json), retaining
  their isolated profiles and preview setup. Point `fmv` at the candidate and add
  a cloned FMV entry named `fmv-0.2.1` pointing at the public binary. Select only
  `ordinary-5k.md` and `gfm-math-images-100k.md`. Resolve the example paths for your
  machine; local paths and generated profiles are not published.

```powershell
python experiments/competitors/measure.py target/competitors/maintenance.json --runs 5 --output experiments/competitors/maintenance-windows.csv
python -X utf8 experiments/competitors/summarize-maintenance.py
```

The existing harness uses seed `20260911` to shuffle all ten app entries, both
fixtures, and five fresh launches per cell into **one batch of 100 attempts**.
The original fixture generator plus `portable-images.py` supplies the same
deterministic inputs as the earlier study. Exact executable and fixture hashes
are audited in [maintenance-results.md](maintenance-results.md); the raw CSV
retains every attempt, including any failure.

## What the numbers mean

The small fixture is about 5 KiB and the medium fixture about 100 KiB. Both repeat
GFM, code, math, and multilingual text; the medium filename is historical and
contains no images. Each sample waits three seconds after observing a window,
then measures process-tree CPU for about one second and records memory at the
end. Results are medians of five samples; the detailed report includes ranges.
1000 ms CPU/s represents one occupied CPU core; 0.0 is below sampling resolution.

Working set is Windows resident process-tree memory, potentially counting shared
pages in multiple processes. Private memory is committed private memory, a
different metric. Neither is peak memory. Window discovery is a startup proxy,
not first-content latency; a live process does not prove that the whole document
finished rendering. CPU at this checkpoint is not a battery-life benchmark.

The competitors perform different work. Editors such as Obsidian, Typora,
MarkText, and VS Code provide features outside FMV's read-only scope. VS Code
uses its built-in preview through the same minimal adapter; Obsidian gets a
one-document vault. Typora retains its dedicated trial profile without resetting
licensing state. Other apps receive fresh profiles where their runtime honors
the overrides. See the [rendering observations](observations.md) for the pinned
competitors: mdview-zig, for example, leaves tables, math, task markers, and alerts
as source. Its lower memory use is retained in the README comparison.

These results support narrowly scoped memory comparisons on this Windows
machine. They do not establish an overall speed winner, equivalent rendering
coverage, performance on every document, or macOS/Linux rankings. Dependency
before/after differences also include the input-font fix and the difference
between a GitHub-built baseline and a local release build; small changes should
not be attributed to a particular library without further profiling.

The maintenance executable is 19,474,944 bytes versus 19,061,248 bytes for the
published baseline (about 2.2% larger). The renderer update temporarily retains
two resvg/usvg versions because egui_extras has not migrated. Median working sets
were 127.2/138.7 MiB for the candidate versus 127.6/138.5 MiB for the baseline on
the small/medium fixtures. Both measured below CPU sampling resolution at the
checkpoint in all ten samples. This small study shows no material resident-memory
regression; it does not establish a statistically significant speed improvement.
