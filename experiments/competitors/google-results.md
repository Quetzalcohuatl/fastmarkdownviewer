# Expanded Windows resource results — September 14, 2026

Five fresh launches per app/document, all 12 entries randomized in one batch.
All **120 attempts** are retained in [the raw CSV](google-windows.csv).
[Method, hardware and reproduction](google-method.md);
[selection and unmeasured products](google-discovery.md);
[rendering observations](google-observations.md); [features](google-features.md).

Values are **median (minimum–maximum)**. Window discovery is not first-content
latency. Checkpoint CPU is not settled idle or battery use. MiB = 2²⁰ bytes.

**MarkLite is not ranked:** its window appeared, but the document area remained
blank in the pilot, including a 30-second follow-up with default environment.
Its ten window-only samples stay visible here for audit, not as a reading-performance
verdict. The README ranks eleven apps (FMV plus ten competitors).

## ordinary-5k.md

| App | Window observed / attempts | Working set MiB | Private memory MiB | Checkpoint CPU ms/s | Window proxy ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| FastMarkdownViewer 0.2.1 | 5/5 | 132.3 (132.2–132.6) | 159.4 (159.3–160.6) | 0.0 (0.0–0.0) | 22.8 (21.9–41.6) |
| aydiler/md-viewer 0.2.0 | 5/5 | 232.0 (223.5–232.3) | 319.1 (311.4–320.2) | 0.0 (0.0–0.0) | 23.7 (21.6–45.7) |
| MarkLite 1.1.1 (unverified content) | 5/5 | 144.3 (144.1–145.2) | 238.8 (238.4–239.9) | 0.0 (0.0–61.6) | 1196.0 (1176.6–1299.2) |
| MarkMello 0.4.0 | 5/5 | 135.0 (134.6–137.4) | 176.2 (175.9–192.5) | 0.0 (0.0–15.4) | 306.4 (277.8–1020.6) |
| Markpad 2.7.6 | 5/5 | 438.6 (438.3–441.0) | 269.9 (265.4–273.3) | 107.9 (0.0–262.0) | 21.9 (21.2–25.4) |
| MarkText 0.19.1 | 5/5 | 471.1 (468.6–475.4) | 303.3 (300.9–307.1) | 0.0 (0.0–0.0) | 544.2 (493.4–718.6) |
| MD Preview 1.4.1 | 5/5 | 401.8 (399.7–405.5) | 230.6 (228.7–231.9) | 0.0 (0.0–138.7) | 21.4 (20.8–55.4) |
| mdview-zig 0.2.0 | 5/5 | 42.3 (42.2–42.4) | 59.8 (59.2–60.9) | 0.0 (0.0–15.5) | 23.7 (21.8–42.1) |
| Moji 1.0.7 | 5/5 | 432.0 (427.4–434.1) | 332.0 (324.9–335.6) | 30.9 (0.0–46.3) | 478.9 (424.9–2378.3) |
| Obsidian 1.13.7 | 5/5 | 458.1 (456.8–464.5) | 442.3 (435.2–446.2) | 0.0 (0.0–30.9) | 555.0 (481.0–561.8) |
| Typora 1.14.10 | 5/5 | 594.7 (578.4–596.6) | 466.1 (445.8–468.0) | 15.4 (0.0–30.9) | 354.2 (328.5–832.9) |
| VS Code 1.130.0, Markdown preview | 5/5 | 1972.6 (1946.1–2175.2) | 1701.6 (1632.6–1826.2) | 694.5 (432.6–2642.1) | 378.3 (300.6–419.0) |

## gfm-math-images-100k.md

| App | Window observed / attempts | Working set MiB | Private memory MiB | Checkpoint CPU ms/s | Window proxy ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| FastMarkdownViewer 0.2.1 | 5/5 | 138.7 (138.2–148.0) | 167.4 (166.0–169.1) | 0.0 (0.0–0.0) | 39.0 (21.1–290.5) |
| aydiler/md-viewer 0.2.0 | 5/5 | 256.3 (255.3–266.5) | 349.4 (342.2–350.1) | 0.0 (0.0–0.0) | 22.4 (20.7–73.0) |
| MarkLite 1.1.1 (unverified content) | 5/5 | 151.7 (150.2–154.3) | 245.5 (243.4–249.1) | 0.0 (0.0–15.4) | 1253.2 (1199.9–1424.4) |
| MarkMello 0.4.0 | 5/5 | 285.6 (281.6–291.4) | 330.2 (327.2–332.2) | 0.0 (0.0–0.0) | 296.4 (231.5–311.5) |
| Markpad 2.7.6 | 5/5 | 615.7 (603.1–632.7) | 438.0 (425.0–456.8) | 123.2 (0.0–123.7) | 22.8 (22.5–23.2) |
| MarkText 0.19.1 | 5/5 | 628.2 (607.9–640.1) | 460.3 (444.9–463.5) | 0.0 (0.0–15.5) | 615.2 (495.6–781.2) |
| MD Preview 1.4.1 | 5/5 | 540.5 (532.8–562.8) | 364.6 (361.0–390.1) | 92.8 (77.1–154.6) | 41.9 (21.1–162.7) |
| mdview-zig 0.2.0 | 5/5 | 43.4 (43.3–43.9) | 61.2 (60.3–62.7) | 0.0 (0.0–0.0) | 23.6 (21.5–55.8) |
| Moji 1.0.7 | 5/5 | 606.6 (603.6–609.7) | 514.3 (509.6–519.2) | 46.4 (15.4–123.4) | 509.4 (448.6–548.0) |
| Obsidian 1.13.7 | 5/5 | 800.1 (776.9–811.3) | 793.0 (769.2–814.1) | 0.0 (0.0–231.9) | 488.9 (462.4–868.1) |
| Typora 1.14.10 | 5/5 | 679.4 (672.9–680.0) | 550.5 (544.2–554.7) | 0.0 (0.0–15.4) | 386.0 (368.2–881.7) |
| VS Code 1.130.0, Markdown preview | 5/5 | 2271.8 (2206.8–2321.2) | 1934.9 (1898.6–2007.3) | 1295.0 (864.3–1551.0) | 353.2 (336.4–768.6) |

## Download and distributed app sizes

Selected newly acquired products, with the published FMV portable binary as baseline.
These are file-byte counts, not total installed footprints. Compression and packaging
differ; shared runtimes and system libraries are excluded. In particular, Markpad
requires WebView2 in addition to its smaller executable.

| App | Download MiB | Distributed app files MiB | Packaging |
| --- | ---: | ---: | --- |
| FMV 0.2.1 | 18.2 | 18.2 | Portable executable, 19,061,248 bytes |
| Markpad 2.7.6 | 13.4 | 13.4 | WebView2 is not included in this portable exe |
| Moji 1.0.7 | 125.2 | 447.5 | OS and shared system libraries excluded |
| MarkLite 1.1.1 | 13.6 | 37.8 | OS and shared system libraries excluded |

[Exact download hashes and byte counts](google-downloads.json).

## Audit

| App | Executable SHA-256 |
| --- | --- |
| FastMarkdownViewer 0.2.1 | `f10ddcf8f482b3c50ae1df905a04c24441ceb1fb02370742f8ca38a459529d66` |
| aydiler/md-viewer 0.2.0 | `c5fe8c07f3cd1678bd24c1b5ab55f5374932c0cba81deff5021b23270f30e44a` |
| MarkLite 1.1.1 (unverified content) | `98a5cda2c1c4b884c8c94c5b8a5a1d1749df1cecb58c2bcf1e211e73cbde4fa5` |
| MarkMello 0.4.0 | `01cd62372517971d003c37cd1191d9290107e8944de6a8a586157923937880fd` |
| Markpad 2.7.6 | `8cbde9e19bc01fad952f7f3b3276ebc003ac963483334f64f9f1c95be4b146b7` |
| MarkText 0.19.1 | `f783de63f5bf170e7a2ab58b3755425ddf339a6628923664957121fc779ecdfa` |
| MD Preview 1.4.1 | `d24eb9ea9b38f1e57f40d76eec919ae319494a4764ef1e64975e50f5a4b1b10f` |
| mdview-zig 0.2.0 | `29f2b042e99deccc584ce0f44fe553320bae7682727dc2c9de9977ae59bc77be` |
| Moji 1.0.7 | `38af3fbe00fe2a1f9c39dd0e681a3195e4ba26f32b1cb250385393bead04bcb5` |
| Obsidian 1.13.7 | `a00f15ad21a289d4db67df1196d5d99ec5cf31a6972f7777fb9b41e20d86088e` |
| Typora 1.14.10 | `3a1807daf4f0f9e8d1257fac5f61429e9535debf230ffece8b7d931749108a57` |
| VS Code 1.130.0, Markdown preview | `c933b1301979abb102ded0b5c28c5c417fcb09411b15f5ff2484c8bb896cf9ef` |

Fixture SHA-256 values:

- `ordinary-5k.md`: `0b7ad3f74990c566da95984bd29019d7c5399c2ac2995e08b1dd6c01d315731d`
- `gfm-math-images-100k.md`: `771478d23dc1d5ab205db29e23c97f4eb2e28d7a6ecf37cf87298b647bc42594`

Launch-status failures: **0**. Separately, **10 MarkLite attempts** lack a validated reading setup.

A visible live window does not establish complete rendering or equivalent feature coverage.
