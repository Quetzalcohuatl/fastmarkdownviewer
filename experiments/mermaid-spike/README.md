# Isolated Mermaid evaluation

This crate is not a viewer dependency. See [the decision and measurements](../../docs/MERMAID_EVALUATION.md).

```powershell
cargo build --locked --release --manifest-path experiments/mermaid-spike/Cargo.toml --target-dir target/mermaid-spike
Copy-Item target/mermaid-spike/release/mermaid-spike.exe target/mermaid-spike/baseline.exe
cargo build --locked --release --manifest-path experiments/mermaid-spike/Cargo.toml --target-dir target/mermaid-spike --features mmdr
Copy-Item target/mermaid-spike/release/mermaid-spike.exe target/mermaid-spike/mmdr.exe
cargo build --locked --release --manifest-path experiments/mermaid-spike/Cargo.toml --target-dir target/mermaid-spike --features merman
Copy-Item target/mermaid-spike/release/mermaid-spike.exe target/mermaid-spike/merman.exe
.\experiments\mermaid-spike\run.ps1
```

Select one renderer feature at a time. The baseline accepts SVG instead of Mermaid. Without `svg-text`, variants rasterize without SVG text support. Rendering uses no browser, Node, or network. The expanded corpus does not prove full Mermaid compatibility or panic safety.

## Native text prototype (2026-09-10)

```powershell
./experiments/mermaid-spike/generate-stress.ps1
cargo build --locked --release --manifest-path experiments/mermaid-spike/Cargo.toml --target-dir target/mermaid-spike --features mmdr,svg-text
Copy-Item target/mermaid-spike/release/mermaid-spike.exe target/mermaid-spike/mmdr-text.exe
./experiments/mermaid-spike/run.ps1 -Variants mmdr-text -OutputDirectory target/mermaid-text-results
```

Wait for the build to finish before copying the executable. Open the resulting PNGs for visual inspection. The executable is a command-line experiment taking an input file and output SVG path, not the viewer GUI. Fonts come from Windows and are not bundled. Supplemental CJK fonts are loaded only when the SVG contains relevant script ranges; this is a heuristic, not universal glyph coverage.

The runner gives each diagram a separate process and ten-second deadline. It samples the process's reported peak working set every approximately five milliseconds; short processes can exit before a useful sample, so this is only a lower-bound diagnostic. It does not impose an OS memory cap. Input/output/pixel checks do not bound the renderer's intermediate allocations. Pipeline timing excludes process launch and SVG/PNG file output. See [results and known limitations](../../docs/MERMAID_PROTOTYPE.md).

## Four-renderer comparison

The experiment now patches Rusty to a [local fixed copy](vendor/rusty-mermaid-diagrams/README.md). Rebuilding the Rusty variant therefore tests the fixed behavior; the stored original comparison CSV remains historical. See [fix validation](../../docs/RUSTY_FIXES.md).

Select `mmdr`, `rusty`, `selkie`, or `merman`, always alongside `svg-text`, building one variant at a time. Copy each completed binary to `target/mermaid-spike/NAME-text.exe`. The optional renderer dependencies must not be combined in one binary.

```powershell
foreach ($variant in @('mmdr', 'rusty', 'selkie', 'merman')) {
    cargo build --locked --release --manifest-path experiments/mermaid-spike/Cargo.toml --target-dir target/mermaid-spike --features "$variant,svg-text"
    if ($LASTEXITCODE -ne 0) { throw "Build failed for $variant" }
    Copy-Item target/mermaid-spike/release/mermaid-spike.exe "target/mermaid-spike/$variant-text.exe"
}
foreach ($run in 1..3) {
    ./experiments/mermaid-spike/run.ps1 -Variants mmdr-text,rusty-text,selkie-text,merman-text -OutputDirectory "target/mermaid-comparison/$run"
}
python experiments/mermaid-spike/compare.py
```

The script writes combined measurements to this experiment and an HTML gallery/JSON summary under `target/mermaid-comparison`. View that gallery on a white background to compare transparent outputs fairly. Read the [comparison report](../../docs/MERMAID_COMPARISON.md) for the reproducible aborts and incomplete parses; `ok` is an exit status, not a compatibility score.
