# v0.2.8 performance check — October 1, 2026

The release retains ordinary-document performance in this Windows sample while
adding work that the previous release skipped: large-code highlighting, large
diagrams, and larger images. The dense 2 MiB rich-document fixture remains slow
to scroll. Removing input cutoffs does not make arbitrary documents cheap.

This is a local regression check, not a new competitor ranking or an absolute
startup-time claim. Native correctness is also checked on macOS and Linux by the
release workflow; these Windows timings must not be attributed to those systems.

## Native resource checkpoint

Five fresh launches per version/document, all 60 attempts in one randomized
batch using the existing competitor resource harness. Every attempt produced a
window and survived the checkpoint. Process-tree resources were sampled about
four seconds after window creation. CPU is measured over the final one-second
interval, not during the initial loading burst. Table values are medians.

| Fixture | 0.2.7 resident MiB | 0.2.8 resident MiB | 0.2.7 window proxy ms | 0.2.8 window proxy ms |
|:--|--:|--:|--:|--:|
| ~5 KiB document | 132.47 | 132.71 | 18.31 | 20.17 |
| ~100 KiB rich document | 138.75 | 139.56 | 19.56 | 17.47 |
| 24 SVG images | 89.93 | 94.57 | 19.94 | 16.77 |
| Large code | 83.52 | 120.77 | 18.11 | 19.92 |
| Large Mermaid | 83.43 | 84.63 | 16.08 | 17.10 |
| 48 MP / 48 MB PNG | 67.03 | 125.65 | 16.56 | 18.19 |

CPU medians were below sampling resolution for every case in both versions.
One baseline small-document sample recorded 15.55 CPU ms/s. This does not mean
zero CPU consumption, settled idle, or a battery-life result. These are neither
peak-memory measurements nor completed-render timings. Window detection precedes
content presentation, and outliers around 300 ms remain in the raw data.

The three large-content rows are **not equivalent completed workloads**: 0.2.7
leaves the large code unhighlighted and rejects the large diagram and image.
The candidate's additional memory is part of doing that work. The large-image
candidate's median private committed memory is 338.50 MiB, distinct from its
125.65 MiB resident working set. Its actual rendered framebuffer was inspected.

[All 60 native samples](windows-native.csv) include private memory, CPU, ranges,
and hashes. Baseline is the checksum-verified official v0.2.7 Windows executable;
candidate is a local optimized v0.2.8 build. Native results do not isolate
differences between the official and local build hosts.

## Scrolling CPU layout

The identical `reading_benchmark` source was built locally against v0.2.7 and
v0.2.8, with the same Rust 1.95 release settings. Five fresh processes per
version/fixture, randomized, each with five warmup frames and 30 scrolling
frames. The harness also opens and closes five tabs. No GPU or presentation
work is included. Medians below summarize per-process medians and p95s; they are
not percentiles pooled across all frames.

| Fixture | 0.2.7 median ms | 0.2.8 median ms | 0.2.7 p95 ms | 0.2.8 p95 ms |
|:--|--:|--:|--:|--:|
| ~5 KiB | 0.473 | 0.460 | 1.151 | 0.920 |
| ~100 KiB rich document | 13.264 | 8.693 | 16.100 | 14.072 |
| 2 MiB dense rich document | 240.584 | 241.986 | 293.837 | 294.132 |
| 24 SVG images | 0.258 | 0.131 | 0.683 | 0.629 |

The medium fixture's per-process median ranges overlap substantially:
8.41–15.44 ms before and 8.40–15.12 ms after. This small noisy sample does not
establish a general speedup. The dense stress fixture's approximately 242 ms
CPU layout per frame is a continuing limitation, not smooth scrolling.

[All 40 layout samples](windows-layout.csv).

## Progressive work

Five fresh optimized highlighting-test processes rendered an 820,000-byte Rust
block. Median first plain CPU-layout frame: **8.65 ms** (range 8.01–10.43 ms).
Median completed highlighting test: **1,250.36 ms** (range 1,147.23–1,253.29 ms).
Completion includes the test's 100 ms polling and final cache-retention check.
These are headless layout timings, excluding file reading, font setup in the
native app, GPU uploads, and screen presentation. The old version skips
highlighting at this size, so it has no equivalent completion time.

[Five highlighting samples](windows-highlighting.csv).

Mermaid helper measurements use one excluded warmup per case and five randomized
fresh-process samples. First PNG means a completely written PNG (including its
IEND marker); polling adds approximately 5 ms granularity. Completion includes
helper shutdown. Neither event is a displayed frame in the main app.

| Diagram / mode | First PNG median ms | Helper completion median ms |
|:--|--:|--:|
| Small, 0.2.7 | 21.16 | 86.37 |
| Small, 0.2.8 display width | 20.42 | 78.95 |
| Large, 0.2.7 | Rejected at 4 MP | Rejected |
| Large, 0.2.8 display width (800 × 1010) | 59.72 | 174.72 |
| Large, 0.2.8 natural resolution (2357 × 2974) | 45.39 | 397.72 |

The natural-resolution result exceeds seven megapixels. These fixtures and
small samples do not guarantee timing for arbitrary Mermaid graphs. The script
has a measurement-only deadline; the application has no fixed render timeout.

[All 30 helper samples](windows-mermaid.csv).

## Environment and reproduction

[Machine record](environment.json), [binary sizes and SHA-256](binaries.json),
[fixture sizes/hashes](fixture-manifest.json), and [summary with all ranges](summary.json).
The local Windows candidate is 20,998,144 bytes versus 20,806,656 bytes for the
published baseline: +191,488 bytes (under 1%). This is executable length, not
installer download size or total installed memory.

Measurements ran on an ordinary Windows 11 desktop with warm filesystem caches,
fresh processes, isolated application profiles, and no overlapping local builds
or tests. The machine was not rebooted or CPU-isolated. Other desktop activity
and scheduling still introduce noise. These are default-preference workloads.

To reproduce from the repository root:

1. Download the official v0.2.7 Windows EXE and verify its published SHA-256.
   Build optimized `reading_benchmark` binaries from v0.2.7 and v0.2.8 using
   Rust 1.95, and build the v0.2.8 viewer and highlighting test. When sharing a
   Cargo target directory between source snapshots, clean the local workspace
   packages before switching sources to avoid stale path artifacts.
2. Generate the ordinary fixtures with
   `experiments/architecture/fixtures/generate.ps1 -OutputDirectory target/release-study/fixtures`.
   Run `python scripts/generate-progressive-fixtures.py target/release-study/fixtures`
   and `python experiments/releases/generate-large-image.py target/release-study/fixtures`.
   The latter uses Pillow; all application binaries remain native.
3. Configure `experiments/competitors/measure.py` with the two executables, these
   six documents, and a separate APPDATA override for each version. Run five
   repetitions in a single invocation. The harness creates a fresh profile for
   every launch. Retain every sample.
4. Run `experiments/releases/measure-layout.py` with `--baseline`, `--candidate`,
   `--fixtures`, and `--output`, and `measure-mermaid.py` with the two app binaries,
   a new `--work` directory, and `--output`. Both default to five samples per case.
5. Build `cargo test --locked --release --test highlighting --no-run`, then run
   the produced test executable five times with `--nocapture`. Retain the first
   plain-frame and completed-test timings. Do not run builds alongside measurements.

The September competitor cohort remains dated and versioned separately; these
new FMV results must not be substituted into that historical ranking.
