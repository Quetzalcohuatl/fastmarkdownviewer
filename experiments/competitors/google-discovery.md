# Google discovery — September 14, 2026

Query: **fast lightweight markdown viewer**, entered without quotation marks in
[Google Search](https://www.google.com/search?q=fast+lightweight+markdown+viewer).
We followed Google's Page 2 and Page 3 links in an actual browser and recorded
the main organic result links below in their displayed order. Ads, related
searches, and nested sitelinks are not separate entries. Results can vary by
date, location, and personalization: these 29 links are a discovery sample,
not an exhaustive list of competitors or a reproducible Google ranking.
Account details, location labels, and tracking URLs are intentionally omitted.

## First three pages

| Page / position | Result | Disposition |
| --- | --- | --- |
| 1 / 1 | [MarkText](https://github.com/marktext/marktext) | Windows benchmark; feature comparison |
| 1 / 2 | [Simple, lightweight Markdown viewer for Windows — Reddit](https://www.reddit.com/r/Markdown/comments/1qc51dm/simple_lightweight_markdown_viewer_for_windows/) | Author identifies **Markpad**, formerly mdview; followed its official repository; Windows benchmark |
| 1 / 3 | [aydiler/md-viewer](https://github.com/aydiler/md-viewer) | Windows benchmark; feature comparison |
| 1 / 4 | [Typora](https://typora.io/) | Windows benchmark; feature comparison |
| 1 / 5 | [Marky — Hacker News](https://news.ycombinator.com/item?id=47795468) | Same product as 1 / 9; deduplicated |
| 1 / 6 | [Markdown Viewer Free — Microsoft Store](https://apps.microsoft.com/detail/9p9sdhx8tqvq?hl=en-US&gl=US) | Acquisition/launch attempted; see limitations below |
| 1 / 7 | [MarkLite](https://marklite.app/) | Windows resource attempts retained; blank pilot view, not ranked |
| 1 / 8 | [MD-Viewer — Mac App Store](https://apps.apple.com/us/app/md-viewer/id6752493034?mt=12) | macOS-only; documented features, no resource ranking |
| 1 / 9 | [GRVYDEV/marky](https://github.com/GRVYDEV/marky) | No Windows release asset in v0.1.3; historical Linux samples exist |
| 2 / 1 | [Offline viewer recommendations — Software Recommendations](https://softwarerecs.stackexchange.com/questions/71013/simple-open-source-offline-markdown-viewer-for-windows) | Discussion, not a standalone product |
| 2 / 2 | [Markdown Reader](https://md-reader.github.io/) | Browser extension; documented features |
| 2 / 3 | [Markdown Viewer — Chrome Web Store](https://chromewebstore.google.com/detail/markdown-viewer/ckkdlimhmcjmikdlpkmbgfkaikojcbjk) | simov's browser extension; documented features |
| 2 / 4 | [Nimbalyst editor guide](https://nimbalyst.com/blog/the-complete-guide-to-markdown-editors/) | Vendor's workspace app included in acquisition/launch checks |
| 2 / 5 | [Moji — Product Hunt](https://www.producthunt.com/products/moji-2) | Followed maker's repository; Windows benchmark |
| 2 / 6 | [Simple Markdown Viewer — Microsoft Store](https://apps.microsoft.com/detail/9p1l338qvhjw?hl=en-US&gl=US) | Separate MobileAnarchy product; documented features; no measured row |
| 2 / 7 | [Single-file editor discussion — Mac Power Users](https://talk.macpowerusers.com/t/anybody-have-suggestions-for-a-single-file-markdown-editor/36106) | Discussion, not a standalone product |
| 2 / 8 | [MacMD Viewer comparison](https://macmdviewer.com/blog/best-markdown-viewer) | Followed vendor's product page; paid macOS app, no trial |
| 2 / 9 | [Top five editors — Medium](https://medium.com/@x.line/my-top-5-markdown-editors-57630c996839) | Editorial, not a standalone product |
| 2 / 10 | [iA Writer](https://ia.net/writer) | Trial download attempted; documented features |
| 3 / 1 | [Markdown Live Preview](https://markdownlivepreview.com/) | Web editor; documented features |
| 3 / 2 | [Simple Markdown — Google Play](https://play.google.com/store/apps/details?id=com.wbrawner.simplemarkdown&hl=en_US) | Android editor; documented features |
| 3 / 3 | [Markdown Viewer — pages.dev](https://markdownviewer.pages.dev/) | Web editor, distinct from both Store apps and the extension |
| 3 / 4 | [Markdown editors — opensource.com](https://opensource.com/article/18/11/markdown-editors) | Editorial, not a standalone product |
| 3 / 5 | [StackEdit](https://stackedit.io/) | Web editor; documented features |
| 3 / 6 | [MarkView comparison](https://getmarkview.com/blog/best-markdown-viewers/) | Followed vendor's extension page; documented features |
| 3 / 7 | [Awesome Markdown Editors](https://github.com/mundimark/awesome-markdown-editors) | Directory, not a standalone product |
| 3 / 8 | [Markdown Guide tools](https://www.markdownguide.org/tools/) | Directory, not a standalone product |
| 3 / 9 | [Quill](https://quilljs.com/) | Rich-text editor library; not a standalone Markdown viewer |
| 3 / 10 | [Markoff — thoughtbot](https://thoughtbot.com/blog/markoff-free-markdown-previewer) | macOS previewer; documented features |

We compare the distinct products directly identified by these results, including
the vendors of comparison articles. We do not recursively benchmark every app
mentioned in every discussion, directory, or comment. Five additional products
from the earlier study remain in the Windows cohort: MarkMello, MD Preview,
mdview-zig, Obsidian, and VS Code. Keeping mdview-zig is particularly important:
it is a lower-memory alternative, despite its narrower tested rendering.

## Acquisition and coverage limits

- **Markpad 2.7.6:** official portable Windows x64 executable, from
  [the release](https://github.com/sftwrdotdev/Markpad/releases/tag/v2.7.6).
- **Moji 1.0.7:** official Windows installer from
  [the release](https://github.com/alexishida/Moji/releases/tag/v1.0.7), extracted
  with 7-Zip; the embedded `app-64.7z` supplies the unmodified application.
- **MarkLite 1.1.1:** [official installer](https://marklite-distro.s3.us-east-1.amazonaws.com/marklite-1.1.1-setup.exe)
  extracted with innounp 2.71.1. Older innoextract versions could not read its
  Inno Setup 6.6.1 format; that is an extraction-tool limitation, not an app failure.
  The extracted app created a titled window, but its document area remained
  blank at eight seconds and in a 30-second follow-up with default environment.
  Ten resource attempts are retained without ranking it as a working reader;
  this setup does not establish how a normally installed copy performs.
- **Nimbalyst 0.77.5:** official x64 installer downloaded and extracted. A pilot
  opened its welcome/setup screen rather than an established document-reading
  state. No resource result is presented as a Markdown benchmark. No agent was
  connected and no subscription was purchased.
- **Markdown Viewer Free 1.1.0.0:** the free Store package was acquired with
  WinGet. Direct executable launch produced a window, but the capture/harness
  did not establish a rendered document. Packaged-app activation needs a
  separate validated adapter. This is an unmeasured result, not a crash verdict.
- **Simple Markdown Viewer:** WinGet supplied MobileAnarchy's feature listing
  and reported no trial, while the browser Store page displayed a $1.99 purchase
  and a **Free trial** button. That button did not yield a standalone package
  in this session. The trial was not acquired and no purchase was made;
  performance remains unmeasured. The conflicting trial metadata is not treated
  as proof that the vendor offers no trial.
- **iA Writer:** the official Windows trial button did not produce an accessible
  download in this browser session. The vendor-hosted 2.1.9644.17275 installer
  URL in WinGet returned HTTP 403 to the downloader. No trial state was reset
  and no account created. This is an acquisition limitation, not poor app performance.
- **Marky 0.1.3:** its release has Mac/Linux packages and no Windows binary.
  [The earlier Linux study](results.md) includes it; those samples use an older
  FMV and different hardware and are not mixed into the current Windows ranking.
- **MacMD Viewer:** vendor lists a paid Mac license and explicitly no free trial.
  **MD-Viewer** and **Markoff** are Mac apps. We did not acquire/run them on a
  matched Mac machine for this study. None is assigned a made-up Windows score.
- **Extensions, web apps, and Android:** documented feature comparisons only.
  Measuring an existing browser tab's incremental cost, a whole fresh browser,
  and an Android emulator would answer different questions. A separate browser
  baseline or real-device protocol is needed before ranking them against FMV.

See [the feature comparison](google-features.md) and
[the Windows method](google-method.md). Unmeasured never means slower, heavier,
or less capable. Vendor performance claims are not substituted for measurements.
