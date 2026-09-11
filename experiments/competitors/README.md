# Windows and Linux competitor comparison

This study combines measured process resources with a sourced feature comparison.
Read the limitations with the numbers; smaller memory use can come from rendering
less of the document.

**Follow-up:** the [sustained CPU investigation](idle-fix.md) identifies and fixes
a title-update repaint loop. The competitor numbers below intentionally retain
the original pre-fix binaries; follow-up CPU measurements are stored separately.

## What the measurements show

On the small Windows fixture, FMV's median process working set was **134.8 MiB**,
close to MarkMello's **137.1 MiB**, below aydiler's **228.0 MiB**, and below the
tested Electron editors. mdview-zig used **45.6 MiB**, but left tables, task markers,
math and alerts as literal source. These are resources at a fixed startup
checkpoint, not peak memory or proof of completed rendering.

Within the software-rendered Linux VM, the small-file medians were **180.9 MiB**
for FMV, **232.1 MiB** for MarkMello and **295.1 MiB** for aydiler. FMV used about
**2.18 CPU cores** during its checkpoint interval; the latter two rounded to zero.
These Linux values must not be ranked directly against Windows hardware results.
mdview-zig crashed in all 20 Linux attempts. Obsidian's Linux resource numbers are
excluded because its pilot displayed keyring setup over the reading view.

FMV's CPU use is the clearest issue to investigate: the Windows checkpoint was
about one core, and a separate check after 30 seconds still measured **976.4 ms
CPU/s**. aydiler measured 0.0 in that later check. Do not market these results as
an overall performance win or a battery-efficiency claim.

The 2 MiB stress fixture also exposed failures: MD Preview exited and Obsidian's
renderer crashed in all five Windows attempts. Live processes in other apps do
not establish that the whole stress document finished rendering. Typora explicitly
rejected the stress file on both platforms, so those readings are excluded.
Several other apps still showed blank content in the separate large-file pilots;
the results identify those rows rather than treating them as completed rendering.

The [feature matrix](features.md) distinguishes documented support from observed
behavior. The most useful next viewer features are automatic reload, saved
preferences/session state, folder navigation, export, frontmatter presentation,
and broader Mermaid compatibility. Profile the CPU issue before prioritizing
performance marketing or adding more rendering work.

## Data and scope

The completed dataset contains **380 desktop attempts** across ten applications,
**160 CLI samples** across two tools, and four longer CPU diagnostics. Windows has
nine desktop apps; Linux attempts all ten, including Linux-only Marky. The broader
feature catalog covers twenty competing projects/applications. Failed and excluded
attempts remain in the raw files; pilot measurements are not mixed into the medians.

- [Measured results](results.md): medians/ranges, separate Windows and Linux tables.
- [Feature comparison](features.md): direct competitors plus the wider GitHub set.
- [Observed rendering differences](observations.md).
- [Environment and measurement definitions](environment.md).
- Raw desktop samples: [Windows viewers](windows.csv), [Windows reference apps](windows-anchors.csv), [Linux](linux.csv).
- Raw CLI samples: [Windows](windows-cli.csv), [Linux](linux-cli.csv).
- Longer CPU diagnostics: [Windows](windows-late-cpu.csv), [Linux](linux-late-cpu.csv).
- [Pinned downloads and SHA-256 hashes](downloads.json), [upstream source index](sources.json).
- [Fixture and SVG hashes](fixture-manifest.json); image links use relative paths.
- [Dataset audit](audit.json); regenerate with `python experiments/competitors/audit.py`.

## Reproduce

Use a disposable test account/VM and dedicated app profiles. Install the pinned
official assets and their normal runtime dependencies. Do not disable Chromium
sandboxing to make an unpacked editor launch.

```powershell
python experiments/competitors/download.py
./experiments/architecture/fixtures/generate.ps1 -OutputDirectory target/competitors/fixtures
python experiments/competitors/portable-images.py target/competitors/fixtures
python -m pip install psutil pythonnet
python experiments/competitors/measure.py target/competitors/windows.json --runs 5 --output experiments/competitors/windows.csv
python experiments/competitors/measure-cli.py target/competitors/windows.json --runs 10 --output experiments/competitors/windows-cli.csv
python experiments/competitors/summarize.py
```

The local `target/competitors/windows.json` and guest `linux.json` contain machine
paths. Full configuration examples preserve the measured commands and profile
options: [Windows viewers](configs/windows.example.json),
[Windows reference apps](configs/windows-anchors.example.json), and
[Linux](configs/linux.example.json). Edit executable, fixture and session paths
to match your machine; download assets must first be installed/extracted into the
corresponding directories. Downloading alone does not install runtime dependencies.
The Windows reference batch must also be run with `--runs 5` and its own output
`windows-anchors.csv`. These examples exclude the unsuccessful ekino pilot.

The minimal schema is:

```json
{
  "fixtures": "/absolute/path/to/fixtures",
  "files": ["ordinary-5k.md", "gfm-math-images-100k.md", "stress-2m.md", "images.md"],
  "apps": [{"id": "fmv", "command": ["/absolute/path/to/FastMarkdownViewer", "{fixture}"], "env": {}}],
  "cli_apps": [{"id": "glow", "command": ["/absolute/path/to/glow", "-s", "dark", "-w", "80", "{fixture}"]}]
}
```

For Electron apps pass `--user-data-dir=/dedicated/path`; the harness replaces it
with a unique directory per sample. Windows native/WebView apps should include
`APPDATA`, `LOCALAPPDATA`, and `WEBVIEW2_USER_DATA_FOLDER` in their environment map;
Linux apps should include `XDG_CONFIG_HOME`, `XDG_CACHE_HOME`, and `XDG_DATA_HOME`.
The harness substitutes unique directories for those entries. Platform APIs may
ignore environment overrides, so verify each app's profile behavior when adapting
the study. Linux GUI environment must point to the actual display/session; the
runner's own `DISPLAY`/`XAUTHORITY` must also allow xdotool's read-only window lookup.

On Linux, use Python 3 + psutil and xdotool; pythonnet is unnecessary. Execute one
platform's measurements at a time on a shared physical host. Check actual document
display before interpreting samples. `--hold` supports an interactive pilot in a
real terminal; it waits for Enter and then terminates only the launched process tree.
Typora configurations use `preserve_profile: true` to keep its ordinary trial and
licensing state. Do not replace that with a new profile for every sample.

The fixed checkpoint measures resources after window creation, not settled idle,
whole-document completion, scroll performance, or first visible content. For public
startup-speed claims, extend this with content-aware presentation instrumentation
and the larger sampling protocol in [BENCHMARKING.md](../../docs/BENCHMARKING.md).
