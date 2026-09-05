# Architecture bake-off

This directory preserves the deterministic inputs, harness, hardware template, decision rule, and non-shipping Direct2D prototype.

- **egui/Glow:** build the repository default.
- **egui/wgpu:** build the same source with `--no-default-features --features renderer-wgpu`.
- **Direct2D/DirectWrite:** `direct2d-prototype` deliberately tests the native rendering path. Its missing selection/accessibility/math/image/table parity counts as a blocker, not as an ignored benchmark caveat.

Run the protocol in `docs/BENCHMARKING.md`. Populate `RESULTS.md` only from committed raw CSV output. Until then, the shipping implementation stays on egui/Glow because the Direct2D selection threshold has not been met and its feature blockers remain.

## Build commands

```powershell
cargo build --locked --release --target-dir target/glow
cargo build --locked --release --no-default-features --features renderer-wgpu --target-dir target/wgpu
cargo build --locked --release --manifest-path experiments/architecture/direct2d-prototype/Cargo.toml
```
