# Reader feature comparison — 2026-09-08

Exploratory comparison of published v0.1.0 against the working-tree image controls, bounded image resources, reload, and tab reordering. No dependency was added. This is not a new release or a comparison with other Markdown products.

## Results

Native measurements below are medians of five fresh processes per fixture and variant, with randomized order. Window discovery follows `WaitForInputIdle`; it does **not** measure first rendered content. Memory is sampled after a 1.5-second settling interval followed by a one-second CPU sample.

| Fixture | Window proxy ms, stable → candidate | Working set MiB, stable → candidate | Private bytes MiB, stable → candidate |
|:--|--:|--:|--:|
| 5 KiB | 115.23 → 117.56 | 130.64 → 130.48 | 158.75 → 158.27 |
| 100 KiB | 116.68 → 115.85 | 141.73 → 142.05 | 171.18 → 172.00 |
| 2 MiB | 117.68 → 114.15 | 323.67 → 323.39 | 378.07 → 377.94 |
| 24 local SVGs | 115.85 → 117.10 | 146.68 → 88.32 | 258.99 → 201.12 |

Startup proxies are similar in this small sample. Image-fixture working set decreased about 40%, consistent with releasing redundant raw and decoded copies after upload. This does not establish savings for every document. The portable EXE grew from 16,728,064 to 16,771,072 bytes: 42 KiB (0.26%).

CPU use was **not low**: both variants accumulated roughly 1,000–1,141 ms CPU per approximately one-second wall-clock sample at their per-fixture medians, around one logical core. The nominal `idle_cpu_ms` CSV field is an observation window, not proof of quiescence. Investigate repaint/event-loop and driver activity before making an idle-efficiency claim; this comparison does not establish the cause.

The production UI was also exercised without a native renderer: five warm-up frames, 30 wheel-scroll frames, then five tabs opened and closed. Each run pauses at lifecycle checkpoints for process-memory sampling. These timings include CPU layout and font/text work but **exclude GPU uploads, drawing, and presentation**. Values below are medians across three runs; p95 is the median of the three within-run p95 values.

| Fixture | Layout median ms, stable → candidate | Layout p95 ms, stable → candidate |
|:--|--:|--:|
| 5 KiB | 0.89 → 0.85 | 1.11 → 1.10 |
| 100 KiB | 9.94 → 9.22 | 10.88 → 10.56 |
| 2 MiB | 225.93 → 224.37 | 228.30 → 226.55 |
| 24 local SVGs | 0.22 → 0.29 | 0.32 → 0.42 |

The 2 MiB repeated rich-document workload remains slow in both builds. Full-document layout is the next useful performance target; these results do not claim smooth large-document scrolling. The image layout increase is about 0.07 ms and needs larger samples before interpreting a percentage change.

After all tabs close, the image fixture retains a median 30.78 MiB private bytes in the stable headless process versus 6.39 MiB in the candidate. This is a CPU-side lifecycle comparison, not native GPU-memory evidence. Text-heavy fixtures retain substantial font/allocator memory in both builds; closing tabs is not a promise that process working set returns to its initial value. Deterministic tests separately assert image texture-cache invalidation after closing a tab.

## Inputs and environment

- Baseline: v0.1.0, commit `50766028a338684f3d7c8080d287c7cd2d715ae2`; published Windows x64 portable binary SHA-256 `fa0f34e6dde0ceb710dc9d5124ee6eb89e3b41e50b5ffec836ff96cf261fdf95`.
- Candidate: working-tree changes accompanying this report, Rust 1.95.0, default Glow backend, `cargo build --locked --release`; portable SHA-256 `c835ab864d7d0828c2205cac062b6286958597059ad1a43058f9a7e164f558e1`.
- After measurement, an SVG sizing unit test and documentation were added. The final local build, with the same production behavior, is 16,724,480 bytes and has SHA-256 `412284edf036803b727c41bf19169a1670249c87f9259977f8f0adc96d5b175a`; measurements above belong to the frozen candidate hash. Final linking produced a slightly smaller executable than the frozen candidate, so the 42 KiB increase above describes that measured binary specifically.
- Both layout executables: identical `examples/reading_benchmark.rs` compiled against their respective sources with `cargo build --locked --release --example reading_benchmark`. Baseline source came from `git archive v0.1.0`; the benchmark example was copied into that checkout before building.
- Windows 11 Pro 10.0.26200; AMD Ryzen 9 9950X, 16 cores / 32 logical processors; approximately 254 GiB OS-visible RAM.
- Installed GPUs: NVIDIA GeForce RTX 5080 (driver 32.0.15.9186) and AMD Radeon Graphics (32.0.21030.2001). Active OpenGL adapter was not instrumented.
- Default 900×700 logical-pixel native window. Headless layout uses the same dimensions at default egui scale. Native display scale, power plan, and security/background activity were not controlled or recorded; no reboot or ETW capture was performed.
- Generated text fixtures: 5,214, 102,858, and 2,097,450 bytes, repeating tables, code, math, Japanese, and emoji. The historical `gfm-math-images-100k.md` filename does not imply image coverage. The separate image fixture uses 24 distinct local file URLs to 512×512 SVG rectangles, without network access. The generator now creates those images too; absolute URLs adapt to the output directory.

## Reproduce

Preserve each optimized viewer and layout example under a distinct path before building the other variant. Use an interactive Windows desktop for the native portion. The harness closes only processes it starts.

```powershell
.\experiments\architecture\fixtures\generate.ps1 -OutputDirectory .\target\benchmark-fixtures
$variants = @{
    stable = @{ Exe = 'path\to\stable.exe'; Layout = 'path\to\stable-layout.exe' }
    candidate = @{ Exe = 'path\to\candidate.exe'; Layout = 'path\to\candidate-layout.exe' }
}
.\experiments\architecture\compare-reader.ps1 -Variants $variants -Fixtures @(
    '.\target\benchmark-fixtures\ordinary-5k.md',
    '.\target\benchmark-fixtures\gfm-math-images-100k.md',
    '.\target\benchmark-fixtures\stress-2m.md',
    '.\target\benchmark-fixtures\images.md'
) -Runs 5 -LayoutRuns 3 -OutputDirectory .\target\benchmark-results
```

Raw samples: [native.csv](reader-samples/native.csv), [layout-lifecycle.csv](reader-samples/layout-lifecycle.csv). These are small, warm-machine exploratory samples, with occasional startup outliers. They support a local regression check, not statistical significance, cold-start marketing claims, or the formal Glow/wgpu/Direct2D decision.

## Functional validation

The measured feature source passes all 52 locked tests, formatting, Clippy with warnings denied, cargo audit (the two existing allowed unmaintained-dependency warnings), and cargo deny. The optimized local binary passes the interactive Windows process/scroll smoke test. A real framebuffer capture confirms local SVG texture display. Release preparation subsequently bumps the application version to v0.1.1; these measurements retain their original candidate hashes.
