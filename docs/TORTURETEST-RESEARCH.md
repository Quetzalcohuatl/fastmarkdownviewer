# Torture-test research — 10 open-source Markdown viewers

Reviewed 2026-09-09. Target support contract: [README](../README.md) and [PRIVACY](../PRIVACY.md) at FastMarkdownViewer commit `0b0bd73` / v0.1.3. Outputs: [torturetest2.md](../torturetest2.md), [torturetest3.md](../torturetest3.md), and [companion assets](../tests/fixtures/torturetest-assets/README.md).

## Method and limits

The selection covers small dedicated native viewers, terminal readers, a browser extension, and a local preview server. “Lightweight” describes their product scope/architecture, not a measured ranking of download size or memory. A browser extension or preview server also depends on the browser runtime. This is not a performance comparison or an endorsement of all ten projects.

For each repository, checked GitHub repository/license metadata, its published Releases collection, and open and closed issue entries. Retrieved up to 12 recent releases and the 100 most recently updated issue/PR entries with `state=all`; excluded pull requests from issue counts. Also checked the open issue collection separately for eight projects, and both filtered issue pages for the two with no issue history. Older histories are sampled, not exhaustively audited; counts below are reviewed entries, not lifetime totals. Release notes can link to PRs; those are identified as release evidence, not counted as user issue reports. Issue bodies were read for selected reproduction cases. Comments, attachments, and upstream executable builds were not exhaustively inspected or run.

Examples in the suites are newly written adaptations. A symptom in another renderer suggests a useful probe; it does not establish a FastMarkdownViewer bug. An issue being closed is not proof that its requested behavior shipped. Only explicit release-note fixes are described as released fixes. Blank issue histories and boilerplate releases supply no invented evidence. Native/macOS or terminal-specific bugs are translated into the corresponding supported Windows interaction only when appropriate.

## 1. simov/markdown-viewer — browser extension, MIT

[Repository](https://github.com/simov/markdown-viewer) · [Releases](https://github.com/simov/markdown-viewer/releases) · [Open issues](https://github.com/simov/markdown-viewer/issues?q=is%3Aissue%20is%3Aopen) · [Closed issues](https://github.com/simov/markdown-viewer/issues?q=is%3Aissue%20is%3Aclosed).

Reviewed four release entries; 82 issues in the recent mixed-state sample, including 21 closed; separate open sample contained 82 issues.

- [5.3](https://github.com/simov/markdown-viewer/releases/tag/5.3) explicitly fixes shortcode processing inside highlighted code and raw-view front matter. [5.1](https://github.com/simov/markdown-viewer/releases/tag/5.1) mentions TOC detection and emoji exclusion fixes.
- Open reports: [#209](https://github.com/simov/markdown-viewer/issues/209), leading HTML comment prevents rendering; [#242](https://github.com/simov/markdown-viewer/issues/242), spaces in image destinations; [#258](https://github.com/simov/markdown-viewer/issues/258), footnote support; [#277](https://github.com/simov/markdown-viewer/issues/277), responsive wrapping.
- Closed [#214](https://github.com/simov/markdown-viewer/issues/214) reports emoji shortcodes corrupting highlighted fences. Open [#235](https://github.com/simov/markdown-viewer/issues/235) mixes a valid optional-outer-pipe table request with a non-GFM delimiter request; the suites separate these.
- Coverage: T2-01, 03, 05, 07–09, 11, 15, 17, 22, 28; T3 front matter, diagrams, wikilinks, custom styles, nonstandard tables, comments, and alternate math syntax.

## 2. charmbracelet/glow — Go terminal reader, MIT

[Repository](https://github.com/charmbracelet/glow) · [Releases](https://github.com/charmbracelet/glow/releases) · [Open issues](https://github.com/charmbracelet/glow/issues?q=is%3Aissue%20is%3Aopen) · [Closed issues](https://github.com/charmbracelet/glow/issues?q=is%3Aissue%20is%3Aclosed).

Reviewed 12 releases; 19 issues in the recent mixed-state sample, including four closed; separate open sample contained 42 issues.

- [v2.1.1](https://github.com/charmbracelet/glow/releases/tag/v2.1.1) records a viewport-sync fix for duplicated strings during half-page scrolling. [v1.5.0](https://github.com/charmbracelet/glow/releases/tag/v1.5.0) records hard-line-break support fixes.
- Closed [#554](https://github.com/charmbracelet/glow/issues/554) describes reordered lines after scrolling. Open [#983](https://github.com/charmbracelet/glow/issues/983) reports blanking on resize/duplicate text; [#941](https://github.com/charmbracelet/glow/issues/941) describes a short table column disappearing beside long identifiers; [#971](https://github.com/charmbracelet/glow/issues/971) reports inconsistent/missing footnotes; [#851](https://github.com/charmbracelet/glow/issues/851) reports quote-wrap clipping.
- Coverage: T2-04, 11–13, 26; T3 diagram/math expectations. Terminal escape-sequence details and pager-shell parsing are not imposed on the Windows GUI.

## 3. swsnr/mdcat — Rust terminal reader, MPL-2.0

[Repository](https://github.com/swsnr/mdcat) · [Releases](https://github.com/swsnr/mdcat/releases) · [Open issues](https://github.com/swsnr/mdcat/issues?q=is%3Aissue%20is%3Aopen) · [Closed issues](https://github.com/swsnr/mdcat/issues?q=is%3Aissue%20is%3Aclosed).

Reviewed 12 releases and 42 closed issues in the recent sample. The separate open query returned no issues.

- [2.6.2](https://github.com/swsnr/mdcat/releases/tag/mdcat-2.6.2) fixes task-list continuation crashes, matching [#302](https://github.com/swsnr/mdcat/issues/302).
- [2.7.1](https://github.com/swsnr/mdcat/releases/tag/mdcat-2.7.1) fixes inline HTML in table cells; closed [#301](https://github.com/swsnr/mdcat/issues/301) supplies the reported symptom.
- [2.3.1](https://github.com/swsnr/mdcat/releases/tag/mdcat-2.3.1) fixes inline markup in image descriptions. Closed [#285](https://github.com/swsnr/mdcat/issues/285) and [#194](https://github.com/swsnr/mdcat/issues/194) motivate formatted and multiline image-description probes.
- Coverage: T2-02, 04, 06, 18–19; T3 HTML rendering. “Do not crash on HTML” is kept separate from “render HTML.”

## 4. Textualize/frogmouth — Python terminal Markdown browser, MIT

[Repository](https://github.com/Textualize/frogmouth) · [Releases](https://github.com/Textualize/frogmouth/releases) · [Open issues](https://github.com/Textualize/frogmouth/issues?q=is%3Aissue%20is%3Aopen) · [Closed issues](https://github.com/Textualize/frogmouth/issues?q=is%3Aissue%20is%3Aclosed).

Reviewed eight releases; 64 issues in the recent mixed-state sample, including 26 closed; separate open sample contained 39 issues.

- [v0.7.0](https://github.com/Textualize/frogmouth/releases/tag/v0.7.0) fixes document-relative local links when the process working directory differs, referencing closed [#52](https://github.com/Textualize/frogmouth/issues/52). [v0.9.0](https://github.com/Textualize/frogmouth/releases/tag/v0.9.0) fixes local document loading.
- Closed [#91](https://github.com/Textualize/frogmouth/issues/91) and open [#110](https://github.com/Textualize/frogmouth/issues/110) concern internal links. Closed [#88](https://github.com/Textualize/frogmouth/issues/88) reports a wikilink-heading crash. Open [#115](https://github.com/Textualize/frogmouth/issues/115), [#71](https://github.com/Textualize/frogmouth/issues/71), and [#21](https://github.com/Textualize/frogmouth/issues/21) concern includes, collapsible headers, and wikilinks.
- Coverage: T2-15–16, 21; T3-04–06, 17–19. Extension requests are not reclassified as CommonMark requirements.

## 5. joeyespo/grip — Python local browser preview server, MIT

[Repository](https://github.com/joeyespo/grip) · [Releases](https://github.com/joeyespo/grip/releases) · [Open issues](https://github.com/joeyespo/grip/issues?q=is%3Aissue%20is%3Aopen) · [Closed issues](https://github.com/joeyespo/grip/issues?q=is%3Aissue%20is%3Aclosed).

The GitHub Releases collection returned no releases; no GitHub release-note fixes are attributed to this project. This does not mean it has never had package versions or tags. Reviewed 81 issues in the recent mixed-state sample, including 21 closed; separate open sample contained 79 issues.

- Open [#395](https://github.com/joeyespo/grip/issues/395) concerns rewritten image paths; [#364](https://github.com/joeyespo/grip/issues/364) concerns alert formatting; [#243](https://github.com/joeyespo/grip/issues/243) requests YAML metadata display.
- Closed [#349](https://github.com/joeyespo/grip/issues/349) reports code-block rendering problems. Its closed state is not evidence of a specific released fix.
- Coverage: T2-09, 14, 27; T3 metadata and print/export. GitHub API authentication/rate limits are not relevant requirements for FastMarkdownViewer's offline parser.

## 6. ekino/MarkdownViewer — native/Tauri macOS viewer, MIT

[Repository](https://github.com/ekino/MarkdownViewer) · [Releases](https://github.com/ekino/MarkdownViewer/releases) · [Open issues](https://github.com/ekino/MarkdownViewer/issues?q=is%3Aissue%20is%3Aopen) · [Closed issues](https://github.com/ekino/MarkdownViewer/issues?q=is%3Aissue%20is%3Aclosed).

Reviewed 12 releases and three issues: one open, two closed. The closed issues concern native menus and CI runtime migration, rather than Markdown parser regressions.

- [v0.10.0](https://github.com/ekino/MarkdownViewer/releases/tag/v0.10.0) fixes highlights persisting after Find clears. [v0.11.0](https://github.com/ekino/MarkdownViewer/releases/tag/v0.11.0) fixes lingering highlights after document changes and a one-to-zero match transition, and adds multiple windows. [v0.8.1](https://github.com/ekino/MarkdownViewer/releases/tag/v0.8.1) fixes relative images.
- Open [#23](https://github.com/ekino/MarkdownViewer/issues/23) reports dropped inline styles under CSP. FastMarkdownViewer has no CSS engine; this belongs to unclaimed HTML styling, not a missing supported feature.
- Coverage: T2-17, 20, 24, 27; T3-07.

## 7. newuni/md-viewer — native macOS viewer/Quick Look, MIT

[Repository](https://github.com/newuni/md-viewer) · [Releases](https://github.com/newuni/md-viewer/releases) · [Open issues](https://github.com/newuni/md-viewer/issues?q=is%3Aissue%20is%3Aopen) · [Closed issues](https://github.com/newuni/md-viewer/issues?q=is%3Aissue%20is%3Aclosed).

Reviewed 12 recent release entries. Their bodies repeat unsigned-build distribution instructions rather than describe bug fixes. Both open and closed issue pages returned no results; issue creation is restricted. **No issue-derived reproducer or particular released fix is attributed to this repository.** This is an evidence gap in the requested ten-project review, not proof of absence of bugs. Quick Look/CLI export product scope is outside the target's Windows reading contract; no macOS integration requirement is added.

## 8. rajatarya/mdviewer — native/Tauri macOS viewer, MIT

[Repository](https://github.com/rajatarya/mdviewer) · [Releases](https://github.com/rajatarya/mdviewer/releases) · [Open issues](https://github.com/rajatarya/mdviewer/issues?q=is%3Aissue%20is%3Aopen) · [Closed issues](https://github.com/rajatarya/mdviewer/issues?q=is%3Aissue%20is%3Aclosed).

Reviewed eight releases. Both filtered issue pages returned no issues, so evidence here comes from release notes, not user issue reports.

- [v1.4.0](https://github.com/rajatarya/mdviewer/releases/tag/v1.4.0) fixes a second-file-open hang and multiple-file launch handling. [v1.6.0](https://github.com/rajatarya/mdviewer/releases/tag/v1.6.0) protects inline code from emoji/math preprocessing and adds wikilink labels.
- [v1.8.5](https://github.com/rajatarya/mdviewer/releases/tag/v1.8.5) changes Close to target the focused window and adjusts PDF generation/layout.
- Coverage: T2-07, 11, 24; T3-04, 09, 15. Printing support in this project does not imply printing support in FastMarkdownViewer.

## 9. trsdn/mdviewer — minimal native macOS viewer, MIT

[Repository](https://github.com/trsdn/mdviewer) · [Releases](https://github.com/trsdn/mdviewer/releases) · [Open issues](https://github.com/trsdn/mdviewer/issues?q=is%3Aissue%20is%3Aopen) · [Closed issues](https://github.com/trsdn/mdviewer/issues?q=is%3Aissue%20is%3Aclosed).

Reviewed eight releases and 23 issues: one open, 22 closed. The open issue concerns repository quality, not a rendering reproducer.

- [v2.2.0](https://github.com/trsdn/mdviewer/releases/tag/v2.2.0) fixes a sidebar button that could hide but not restore the panel. [v2.0.0](https://github.com/trsdn/mdviewer/releases/tag/v2.0.0) describes Find, outline, footnotes, alerts, task lists, and code controls.
- Closed [#4](https://github.com/trsdn/mdviewer/issues/4) reports unsafe HTML insertion; [#12](https://github.com/trsdn/mdviewer/issues/12), [#18](https://github.com/trsdn/mdviewer/issues/18), [#21](https://github.com/trsdn/mdviewer/issues/21), and [#22](https://github.com/trsdn/mdviewer/issues/22) concern footnotes, metadata cards, image inspection, and accessible code controls. Several are implementation/feature tickets, not independent user bug reports.
- Coverage: T2-13; T3 metadata, inert HTML/CSS, image lightboxes, watching, navigation, and accessibility. No live exploit payload is needed for the inert-content probes.

## 10. vorojar/md-preview — Rust/system-WebView previewer, MIT

[Repository](https://github.com/vorojar/md-preview) · [Releases](https://github.com/vorojar/md-preview/releases) · [Open issues](https://github.com/vorojar/md-preview/issues?q=is%3Aissue%20is%3Aopen) · [Closed issues](https://github.com/vorojar/md-preview/issues?q=is%3Aissue%20is%3Aclosed).

Reviewed 12 releases and 35 closed issues; separate open query returned no issues. Mobile and desktop release entries share the collection; desktop fixes were selected for the GUI probes.

- [v1.4.1](https://github.com/vorojar/md-preview/releases/tag/v1.4.1) fixes document-relative links and changes zoom width handling. Closed [#41](https://github.com/vorojar/md-preview/issues/41) describes relative links opening blank; [#42](https://github.com/vorojar/md-preview/issues/42) discusses zoom/readable-width tradeoffs. The suites preserve FastMarkdownViewer's intentionally capped reading width.
- [v1.1.24](https://github.com/vorojar/md-preview/releases/tag/v1.1.24) fixes relative local images; [#30](https://github.com/vorojar/md-preview/issues/30) reports missing assets. Closed [#13](https://github.com/vorojar/md-preview/issues/13) gives math escaping examples; [#24](https://github.com/vorojar/md-preview/issues/24)/[#25](https://github.com/vorojar/md-preview/issues/25) describe alert contrast/labels; [#14](https://github.com/vorojar/md-preview/issues/14) describes a reload leaving a white window.
- Coverage: T2-10, 14, 16, 19, 25–26; T3 highlight syntax, math boundaries, watching/session behavior, and directory/instance routing.

## Classification and execution

Torture test 2 has 28 supported-behavior cases. Several combine upstream symptoms with explicit target claims: wildcard Find, font fallback scripts, rename validation, remote-image controls, highlighting cutoff, and definition lists are such extensions. They are labeled, rather than presented as exact upstream reproducers.

Torture test 3 has 20 unclaimed/limited-feature cases. Particularly important distinctions: inert HTML versus rendered HTML; first GIF frame versus animation; glyph coverage versus bidi conformance; explicit reload versus watchers; practical heading slugs versus exact GitHub parity; basic RaTeX expressions versus full TeX; and readable code versus Mermaid execution.

The primary files are manually reviewable Markdown, not automated assertions. Companion assets are regenerated by [the fixture generator](../scripts/generate-torturetest-assets.py). Fixtures, links, encodings, dimensions, fence boundaries, and case IDs can be validated without declaring the app passed. No application code or GitHub release is changed by this research.
