# Expanded Windows comparison — September 14, 2026

## Selection and reproducibility

The [Google discovery log](google-discovery.md) accounts for the first three
result pages for `fast lightweight markdown viewer`. This study adds the official
Windows builds of Markpad 2.7.6, Moji 1.0.7 and MarkLite 1.1.1 to the eight
competitors from [the September 13 study](maintenance-method.md). The FMV entry
is the **published 0.2.1 executable**, not an unpublished build optimized for
this comparison. Historical CSVs are preserved unchanged.

There are 12 app entries, two documents, and five fresh launches per cell:
**120 attempts**, shuffled together using seed `20260911`. The raw CSV retains
every attempt. A failure is not a zero-memory or zero-CPU success.

Same Windows 11 Pro 10.0.26200 desktop: Ryzen 9 9950X (16 cores/32 threads),
approximately 253.6 GiB OS-visible RAM, RTX 5080/AMD graphics, Balanced power,
150% display scaling. [Environment details](environment.md) apply. Compilation,
extraction and native-app pilots finished before sampling. Light documentation
work and the normal desktop remained active; the machine was not otherwise
isolated, rebooted, or brought to a cold disk-cache state.

Use [the example config](configs/google-windows.example.json), replacing the
executable/profile/fixture locations for your machine. Original app acquisition
and preview adapters are described in [environment.md](environment.md) and
[downloads.json](downloads.json). The new downloads and exact hashes are in
[google-downloads.json](google-downloads.json). Moji was extracted with 7-Zip
(NSIS installer, then embedded `app-64.7z`); MarkLite with innounp 2.71.1; Markpad
is the official portable executable. No application code was patched.

```powershell
python -X utf8 experiments/competitors/measure.py target/competitors/google/benchmark.json --runs 5 --output experiments/competitors/google-windows.csv
python -X utf8 experiments/competitors/summarize-google.py
```

Generate the same fixtures using the [original generator](../architecture/fixtures/generate.ps1)
and [portable image helper](portable-images.py), as described in the original
environment record. `ordinary-5k.md` and `gfm-math-images-100k.md` contain repeated
GFM/code/math/multilingual text. Despite the second filename, **these two
documents contain no images or Mermaid diagrams**. Rendering support for those
features is not a resource benchmark of image- or diagram-heavy documents.

## Profiles and launch behavior

- The harness supplies per-sample profile paths where the runtime honors
  `APPDATA`, `LOCALAPPDATA`, `WEBVIEW2_USER_DATA_FOLDER` or `--user-data-dir`.
  Markpad's WebView2 descendants are included; measuring only its small parent
  process would be an incomplete comparison.
- VS Code uses its built-in Markdown preview through the same minimal adapter
  as before, in a dedicated profile without the user's extensions. Obsidian gets
  a generated one-document vault set to preview. These editors do more work
  than a dedicated reader.
- Typora retains its dedicated trial profile. We do not reset trial/licensing
  state between launches. Apps that do not honor redirected profile variables
  can retain state; OS/runtime caches are not reset.
- Processes and descendants created by each sample are closed before the next
  sample. An existing user browser's process tree is not counted as a competing
  app, and user applications are not killed to improve the numbers.
- Native applications use their default window sizes, themes, and layouts.
  This compares default reading workflows, not identical pixel counts or
  matched editor capabilities. Default chrome/sidebar area can affect costs.

## Metrics and limits

After observing a window, the harness waits three seconds, samples process-tree
CPU for about one second, and records memory at the end. Reports show medians
and full min–max ranges of five samples, plus attempt counts and input hashes.

| Metric | Meaning | Does not establish |
| --- | --- | --- |
| Working set, MiB | Resident memory summed over the application process tree; shared pages may be counted more than once | Unique physical memory, peak memory, or GPU memory |
| Private memory, MiB | Windows committed private memory summed over that tree | Resident RAM or Linux proportional set size |
| Checkpoint CPU, ms/s | CPU time used during the sampling interval; 1000 ms/s is one core | Settled idle CPU, battery life, or energy use |
| Window proxy, ms | Time from launch to observing a main-window handle | First rendered content, completed document layout, or cold startup |
| Distributed app bytes | Uncompressed files belonging to a downloaded app distribution | Total installed footprint including shared OS/runtime dependencies |

`0.0` CPU means below sampling resolution, not proof of literally no work.
The four-second checkpoint can catch initialization or animation. Do not relabel
it “idle CPU” or turn it into a battery-life estimate. With only five samples,
small differences and noisy ranges do not establish statistical superiority.

A live window is not evidence that all content rendered. Pilots confirmed
Markpad and Moji's initial document views, but MarkLite's document area remained
blank. Its ten window-only samples are retained and excluded from the README's
ranking. The [observation record](google-observations.md) states what was actually
seen. Unknown/unsupported constructs and failed
acquisition are retained, not silently treated as FMV victories.

These results compare the exact pinned Windows versions on this machine and
these two documents. They cannot support “best overall,” “fastest Markdown
viewer,” “lowest memory of every app,” or macOS/Linux/browser/mobile rankings.
Feature-rich editors and simpler readers do different work. The comparison
therefore includes [features and gaps](google-features.md), not just resource
numbers. A broader protocol needs first-content detection, scrolling latency,
long settled-idle samples, larger/image-heavy inputs, and separate native
Mac/Linux/browser cohorts.
