# Persistence validation — 2026-09-11

Saved settings and session restoration use a single local JSON file, read once
at startup and replaced atomically on normal exit. No document contents, rendered
images, or search queries are serialized. There is no timer, file watcher, periodic
write, or per-frame session snapshot. The CPU repaint-loop fix remains intact.

Bare launches restore tab order, active tabs, vertical reading positions, outline
visibility and window grouping. Explicit-file launches restore preferences only.
Inactive restored tabs retain paths and small UI state; their files are read on
selection. Each visible window loads its selected document. Restoring many visible
windows, a large active document, or a selected font still has startup costs.
Restored scrolling requests one additional layout frame to handle native startup
sizing; it does not keep repainting afterward.

## Validation

- Windows: 86 tests passed with `cargo test --locked --all-targets`; Clippy with
  `-D warnings`, formatting, and the optimized release build passed.
- Linux/Ubuntu WSL: 83 tests passed with `cargo test --locked --all-targets
  --message-format=short`. macOS has not been run for this change.
- Tests cover atomic replacement, invalid/future state, preference and session
  restoration, lazy loading, changed/missing files, retry, closed tabs, detached
  windows, quitting, fonts that are no longer installed, native sizing passes,
  and returning to idle after restoration.
- A native Windows check restored two tabs, Georgia/Consolas, 125% zoom, and Light;
  changed the theme to Dark; closed and reopened; and verified Dark and a saved
  scroll position of 500 points. File → Quit application exited normally and
  wrote the expected paths, fonts, theme, zoom, and reading position. The initial
  capture could be blank, as in the earlier competitor study; fresh captures and
  the accessibility tree confirmed the displayed document.

## Windows resource check

| Workload | Median working set MiB (range) | Median CPU ms/s |
|---|---:|---:|
| before | 129.03 (128.95–129.35) | 0.0 |
| after | 129.01 (128.93–129.01) | 0.0 |
| restore100 | 129.78 (128.34–130.24) | 0.0 |

All nine CPU samples rounded to zero. The small memory differences overlap the
run-to-run variation; no memory improvement is claimed. The 100-tab state file
was 13.6 KiB. Startup and exit latency were not measured.

Raw measurements are in [windows-idle.csv](windows-idle.csv). Three fresh launches
per workload, seeded randomized order, ten seconds settling and a five-second
process-tree CPU sample, using the existing [CPU harness](../competitors/idle-cpu.py).
No compilation or GUI interaction ran during the samples. The baseline is the
CPU-fixed build at `c8fe131`; the candidate is the optimized persistence build.
Binary and fixture hashes are recorded per row.

`before` and `after` open `ordinary-5k.md` explicitly with isolated empty profiles.
`restore100` starts without a file, with 100 tabs containing distinct local copies
of that same fixture and the first tab selected. It uses default appearance, one
window, and a dedicated seeded profile. Inactive tabs are not opened during the
measurement. These measurements describe settled CPU and working set, not startup
latency, peak memory, all 100 documents loaded, or battery consumption. Zero CPU
means below the measurement resolution.

To reproduce on Windows, retain the baseline executable before rebuilding, then:

```powershell
python experiments/persistence/prepare.py --before path/to/baseline.exe --after target/release/FastMarkdownViewer.exe --fixture target/competitors/fixtures/ordinary-5k.md --output target/persistence-check
python experiments/competitors/idle-cpu.py target/persistence-check/config.json --output target/persistence-check/idle.csv
```

Use a new output directory to keep profiles isolated. The harness terminates each
process after sampling, so exit-time persistence is verified by the separate
normal-exit tests and native check, not by those resource samples.
