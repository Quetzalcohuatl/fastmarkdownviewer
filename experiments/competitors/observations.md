# Competitor observations — 2026-09-11

These are spot checks, not a conformance certification. Documented support in the
feature matrix is separate from these observations.

## Windows small fixture

- FastMarkdownViewer: heading, table, checked tasks, inline/display math, code
  highlighting and alert rendered. Find opened. An initial blank capture was
  resolved by requesting Find; do not classify the capture as a rendering failure.
- aydiler/md-viewer 0.2.0: heading, table, tasks, inline/display math, highlighted
  code, alert, file explorer and outline rendered. Initial capture also appeared
  blank; a subsequent fresh launch/capture displayed the document.
- MarkMello 0.4.0: heading, table, prose, code and minimap rendered. Task items
  appeared as ordinary bullets; the fixture's inline/display math was absent.
- MarkText 0.19.1: rendered editor displayed heading, table, checked tasks, code,
  inline math; this fixture's single-line display-math delimiters remained source,
  as did the GitHub alert marker. Find opened. Captures initially appeared blank
  until Find was requested; this is not counted as an app failure.
- MD Preview 1.4.1: table, checked tasks, inline/display math, highlighted code and
  alert rendered, confirmed both visually and in the accessibility document.
- mdview-zig Windows 0.2.0: heading/prose/code rendered, while table pipes, task
  markers, math and alert markers remained literal. Its smaller resource footprint
  must be understood in light of this narrower rendering coverage.
- ekino/MarkdownViewer 0.11.1: the documented `mdv file.md` launch opened its welcome
  page with disabled Find instead of the fixture. That pilot is excluded from
  desktop comparisons. This is a CLI-path outcome, not a claim that manual opening
  cannot work.
- Obsidian 1.13.7: the isolated small document rendered table, checked tasks, code,
  inline/display math and callout in reading mode. A vault containing the separate
  stress document crashed its renderer during indexing, so each final sample has
  only its own fixture plus SVG assets. Renderer presence is checked explicitly;
  surviving blank outer windows do not count as valid samples.
- VS Code 1.130.0: the built-in preview rendered headings, tables, code and math.
  Checked-task and alert markers remained source in the tested default preview.
  A minimal local startup extension opens that preview without requiring manual
  keyboard timing; the ordinary source tab also remains open.
- Typora Windows 1.14.10: headings, tables, checked tasks, code and the alert
  rendered. Math remained source under the trial's default configuration. The
  introductory window was closed before final measurements; licensing was not
  changed and the profile was preserved.
- MD Preview exited in all five Windows 2 MiB cases. A separate diagnostic launch
  returned exit code 3221226505 and a panic at `src/main.rs:4640`: failed to build
  WebView with HRESULT `0x80070057` (invalid parameter). The
  [v1.4.1 source](https://github.com/vorojar/md-preview/blob/v1.4.1/src/main.rs)
  sends the initial document through `with_html`; this is consistent with the
  [WebView2 NavigateToString 2 MiB limit](https://learn.microsoft.com/en-us/dotnet/api/microsoft.web.webview2.core.corewebview2.navigatetostring).
  That mechanism is an inference from source and the error, not a debugger-proven
  root cause. Original Windows CSV rows retain `window-observed` for these samples
  because the process exited after detection; zero checkpoint processes marks
  them invalid and the summary excludes them. The harness now also records exits
  occurring after window detection explicitly.

## Linux pilot

- The mdview-zig 0.4.0 Linux package exited with SIGSEGV on the small fixture.
  Zero memory after exit is a failed run, not a performance result.
- MarkText's unpacked archive initially failed because the Chromium sandbox helper
  was not installed with its required root ownership and setuid permission. The
  stable archive launched after installing that helper correctly. No no-sandbox
  flag was used.
- Pilot screenshots were obscured by desktop package-update notifications. The
  disposable benchmark guest was rebooted before the main measurements.
- After reboot, FMV, aydiler, MarkMello, Marky, MD Preview and stable MarkText all
  displayed the small fixture. Marky rendered the table/tasks/code/outline, but
  inline and display math and the alert marker remained source in its default
  configuration. MarkMello again omitted math. MarkText again left this display
  math delimiter variant and alert marker as source. See the retained
  [Linux pilot screenshots](evidence/README.md).
- VS Code's Linux preview displayed the same table/code/math and literal task/alert
  markers as Windows. Typora Linux displayed table/tasks/code/alert, with math still
  literal under the default configuration.
- Obsidian Linux rendered the small document behind a keyring-creation prompt.
  No credentials were entered or keyring settings changed. Its raw attempts are
  retained, but Linux Obsidian is excluded from the resource comparison because
  the setup was not complete. This is not classified as a rendering failure.
- Its five Linux stress attempts lost their renderer. The [guest kernel log](evidence/linux-oom.txt)
  records five out-of-memory kills of Obsidian, with roughly 3 GiB of anonymous
  resident memory in each killed process. This 4 GiB/no-swap VM limit is distinct
  from the Windows renderer-crash observation; it does not establish a shared
  root cause across operating systems. Memory pressure can also evict filesystem
  caches between runs, another limitation of this exploratory VM dataset.

## Longer CPU diagnostic

On Windows, a separate small-document check after 30 seconds measured FMV at
976.4 ms CPU/s over five seconds (about one core), versus 0.0 for aydiler/md-viewer.
This confirms that FMV's high CPU reading was not confined to the first four
seconds in this workflow. It warrants profiling; this study does not establish
whether redraw scheduling, graphics behavior, occlusion or another cause is
responsible. Each is a single diagnostic, not a repeated distribution.

In the Linux software-rendered VM, the matching later check measured FMV at
1959.9 ms CPU/s (about two cores) and aydiler at 0.0. This reinforces the need to
profile FMV, but software graphics and different rendering coverage prevent
attributing the difference to a specific code path from these samples alone.

## Separate 2 MiB visual checks

These are additional pilots, excluded from the repeated resource medians. Linux
screenshots were requested about 12 seconds after launching the harness. They show
the top viewport, not completion of every document element or responsive scrolling.

- FMV and aydiler displayed the stress document with table, tasks, math and code.
- Marky displayed document content, but math remained literal and the visible code
  was not highlighted at the capture. MD Preview displayed document content and
  the alert, but math and code highlighting had not appeared in the capture.
- MarkMello had a window frame but an unpainted content area. MarkText and VS Code
  preview showed blank content areas. These are time-bounded observations, not
  proof that they can never finish. Their checkpoint resources remain marked as
  unfinished/unverified rendering in the results.
- Typora explicitly displayed “The file is too large to render in Typora.” on both
  Linux and Windows. Its stress rows are excluded from summarized resource values;
  the surviving process was displaying an error screen, not the document.
- A separate Windows MarkText pilot also exposed no document content after
  activation and Find. No content-ready latency was established.

See the [stress screenshots](evidence/README.md#stress-document-pilots). Windows
Typora and MarkText pilots were performed after the retained measurements finished.

The corrected image fixture uses relative SVG paths. Separate Linux pilots showed
the visible image in both FMV and aydiler; see [image evidence](evidence/README.md#portable-image-fixture).
This does not certify every image in every competitor at the checkpoint.

## Interpretation

Window creation is only a proxy; no first-content timing or scroll-FPS measurement
has been established. A successful pilot display does not prove every large file
has finished rendering at the resource checkpoint. CPU is measured during a fixed
one-second interval starting three seconds after window detection, not certified
steady-state idle. 1000 ms CPU/s equals one fully occupied CPU core.
