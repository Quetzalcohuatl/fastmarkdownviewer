# Direct2D/DirectWrite prototype

This non-shipping spike uses Microsoft's `windows-canvas` and `windows-window` crates, which are thin, safe wrappers over the windows-rs Direct2D, Direct3D 11, DXGI, DirectWrite, and Win32 bindings. It parses the same fixture Markdown with pulldown-cmark and presents its content through DirectWrite.

The prototype is intentionally preserved after losing the feature gate. It does not have production-quality selectable text/accessibility, structured tables, RaTeX composition, image resource policy, scrolling/virtualization, or complete DPI/theme behavior. Those are selection blockers under the bake-off rule, not deferred work in the shipping egui implementation.

Its launch numbers may still be recorded for architectural reference, but must be labeled as a capability-incomplete prototype.
