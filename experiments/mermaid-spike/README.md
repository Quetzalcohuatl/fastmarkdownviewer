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

Select one renderer feature at a time. The baseline accepts SVG instead of Mermaid. All variants rasterize with the viewer's current resvg features, intentionally without SVG text support. Rendering uses no browser, Node, or network. The corpus includes six valid diagram types and two invalid cases; it does not prove full Mermaid compatibility or panic safety.
