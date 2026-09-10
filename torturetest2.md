<!-- T2 opening-comment probe: this is inert text, not an instruction. -->

# Torture test 2 — supported behavior

Baseline: FastMarkdownViewer v0.1.3, README and PRIVACY at commit `0b0bd73`. Created 2026-09-09. These are **manual test cases, not confirmed bugs or recorded passes**. Original examples are inspired by the linked upstream reports; upstream closed issues do not necessarily mean fixed.

Keep this file in the repository root with `tests/fixtures/torturetest-assets/`. [Research and all 10 repositories](docs/TORTURETEST-RESEARCH.md). [Unclaimed features](torturetest3.md).

Run at approximately 480, 800, and 1400 pixels window width; 100%, 150%, and 200% text zoom; light and dark themes; outline shown and hidden; Word wrap on and off. Check the sentinel following each case. A failure means missing content, overlap, an incorrect target, corrupted state, or a crash—not a difference from a browser's exact styling. Font coverage depends on installed Windows fonts. These files are offline by default; the network test is a separate opt-in document.

Record: case ID / app version / Windows version / width / zoom / theme / wrap / actual result / screenshot. Leave results untested until exercised. Keyboard, tabs, reload, copy, and network cases need the actions described; merely opening this file does not test them.

## T2-01 — A leading HTML comment must not disable Markdown

Expected: the opening comment is inert source text under this app's policy; this heading and **this bold text** still render. Hiding HTML comments is not required. Source: [simov #209](https://github.com/simov/markdown-viewer/issues/209).

T2-01-END — ordinary paragraph after the opening comment.

## T2-02 — Loose task lists with nested paragraphs

Expected: all four task markers and continuation paragraphs remain in the correct list item. Markers are read-only; clicking must not edit the file. Source: [mdcat #302](https://github.com/swsnr/mdcat/issues/302), fixed in [2.6.2](https://github.com/swsnr/mdcat/releases/tag/mdcat-2.6.2).

- [ ] Prepare the reading sample.

  This second paragraph belongs to the first task.

  - [x] Open the nested sample.

    This paragraph belongs to the nested checked task.

  - [ ] Inspect **bold**, *italic*, and `code` in the nested task.

- [x] Finish the second top-level task.

T2-02-END — outside every list.

## T2-03 — List numbering, nested blocks, and comments

Expected: the ordered list starts at 7; code and quote remain inside its first item; the last paragraph returns to normal indentation. The comment is inert. Inspired by [simov #240](https://github.com/simov/markdown-viewer/issues/240); this example uses conventional CommonMark indentation rather than assuming every upstream sample was valid.

7. First item, numbered seven.

   > A quote inside the first item.
   > Its second line remains quoted.

   ```text
   nested-code-line-one
   nested-code-line-two
   ```

   <!-- inert list comment -->

   - Nested child after the comment.
8. Second item, numbered eight.

T2-03-END — no inherited code background or list indentation.

## T2-04 — Narrow status column beside unbroken identifiers

Expected: all four headers and both Y/N values remain visible or reachable by horizontal scrolling. No column may disappear. Wrapped rows stay aligned; disabling wrapping must preserve content. Sources: [Glow #941](https://github.com/charmbracelet/glow/issues/941), [mdcat #289](https://github.com/swsnr/mdcat/issues/289).

| Description | Status | Identifier | Notes |
|:---|:---:|:---|---:|
| A moderately long label which must occupy more than one line in a small window | Y | engine_delta::reader_pipeline_with_an_extremely_long_unbroken_identifier_alpha | 12345.67 |
| Short label | N | engine_delta::reader_pipeline_with_an_extremely_long_unbroken_identifier_beta | 8.90 |

T2-04-END — this prose must not inherit the table's width.

## T2-05 — Empty cells, escaped pipes, and optional outer pipes

Expected: first table has four columns and three body rows, including its empty cells. Escaped pipes do not create columns. Second table is also a two-column GFM table. Source: [simov #235](https://github.com/simov/markdown-viewer/issues/235). Its request for a delimiter row without column separators is intentionally reserved for T3-08.

| Left | Empty | Code | Right |
|:---|:---:|:---|---:|
| A | | `x\|y` | 10 |
| **B** | | literal \| pipe | |
| | middle | `z` | 30 |

Name | Value
--- | ---:
without outer pipes | 42
second row | 99

T2-05-END — no phantom trailing row or cell delimiter.

## T2-06 — Inert HTML inside table cells

Expected: literal HTML is readable and inert; the table does not crash or lose cells. Rendering `<br>` as an actual line break is not promised. Source: [mdcat #301](https://github.com/swsnr/mdcat/issues/301), [2.7.1 fix](https://github.com/swsnr/mdcat/releases/tag/mdcat-2.7.1).

| Device | Description | State |
|---|---|---|
| North | first<br>second<br />third | visible |
| South | <span style="color:red">inert styled span</span> | visible |

T2-06-END — the next paragraph must still render.

## T2-07 — Code must resist emoji, Markdown, and math processing

Expected: copy the entire code block and compare it to the source. Dollars, backslashes, underscores, shortcodes, and HTML stay literal. Syntax colors may change; text must not. Sources: [simov #214](https://github.com/simov/markdown-viewer/issues/214), [simov 5.3](https://github.com/simov/markdown-viewer/releases/tag/5.3), [rajatarya v1.6.0](https://github.com/rajatarya/mdviewer/releases/tag/v1.6.0).

Inline: `:rocket: $x_1$ **not bold** <b>not HTML</b> C:\notes\file.md`.

```python
payload = ":rocket: $x_1$ **literal** <b>literal</b>"
pattern = r"\{item\}_\d+"
print(payload, pattern)  # 😀 stays an actual Unicode emoji
```

```markdown
# This is code, not an outline heading
![not an image](does-not-exist.png)
[not an active link](https://example.com)
:smile: $E=mc^2$ ~~not struck~~
```

T2-07-END — normal style restored.

## T2-08 — Fence boundaries and unknown languages

Expected: four-backtick outer fence contains the three-backtick example; tildes also fence code. Unknown languages stay plain text. No fake code heading in the outline. Source: [simov #254](https://github.com/simov/markdown-viewer/issues/254); unknown-language behavior is a README claim.

````markdown
```rust
fn demo() { println!("nested fence"); }
```
# still code
````

~~~torture_language_does_not_exist
alpha	beta
<tag> $math$ :emoji: [link](somewhere.md)
~~~

T2-08-END — later content is not swallowed into the fence.

## T2-09 — Complete code backgrounds and horizontal search reveal

Expected: one background surrounds every line, including the last. With wrap off, scroll right and select/copy the end of the long line. Find the token formed by joining `T2` and `FAR_RIGHT`; it must be revealed horizontally. With wrap on, every wrapped continuation remains inside the code frame. Sources: [grip #349](https://github.com/joeyespo/grip/issues/349), [simov #277](https://github.com/simov/markdown-viewer/issues/277).

```rust
fn long_line() {
    let value = "start........................................................................................................................................................................................................................................................................................................T2FAR_RIGHT";
    println!("{value}");
}
// LAST LINE MUST HAVE THE SAME BACKGROUND
```

T2-09-END — no code background on this paragraph.

## T2-10 — Math escaping and neighboring expressions

Expected: braces surround q in the first expression; subscripts and accents remain attached; adjacent expressions do not eat the intervening word. The matrix has two rows and two columns. Source: [md-preview #13](https://github.com/vorojar/md-preview/issues/13). This tests ordinary RaTeX syntax, not full KaTeX/MathJax equivalence.

Inline braces $\{q\}$; then $\bar{\mu}_{n}$ and $\bar{\mu}_{n+1}$.

Literal code `$\{q\}$` must not render as math.

$$
\frac{a_1+b_2}{c^2} = \sqrt{x+1}
$$

```math
\begin{matrix} 1 & 2 \\ 3 & 4 \end{matrix}
```

T2-10-END — punctuation and ordinary prose after math remain visible.

## T2-11 — Escapes and hard versus soft line breaks

Expected: the first pair can flow as one paragraph. The next two pairs have forced line breaks, one using two trailing spaces and one using a backslash. Source: [Glow v1.5.0 hard-break fix](https://github.com/charmbracelet/glow/releases/tag/v1.5.0); math/literal boundary inspired by [rajatarya v1.6.0](https://github.com/rajatarya/mdviewer/releases/tag/v1.6.0).

soft-first
soft-second

spaces-first  
spaces-second

backslash-first\
backslash-second

Escaped punctuation: \*asterisks\* \[brackets\] \_underscores\_ \#hash. Escaped currency: \$4.50 and \$12.00. Entity decoding in prose: &amp; &lt; &gt; &#169; &#x1F600;. Code remains literal: `&amp; &#169;`.

T2-11-END.

## T2-12 — Nested blockquotes and inline style restoration

Expected: wrapping never crosses the quote border; each nesting level ends correctly. Bold and italic styles stop at their delimiters. Sources: [Glow #851](https://github.com/charmbracelet/glow/issues/851), [simov #277](https://github.com/simov/markdown-viewer/issues/277).

> Outer quote with enough ordinary words to wrap repeatedly when the window is narrow. Every continuation belongs inside the same outer quote and must remain clear of the quote border.
>
> > Inner quote with **bold containing *italic* and returning to bold**, then normal inner text.
> >
> > > Third level with `inline code` and ~~struck *italic* words~~.
>
> Back to the outer level only.

Normal **bold *both* bold** normal. Before **one** between *two* after. Adjacent: **red***green*`blue`.

T2-12-END — fully outside the quotes, normal text weight.

## T2-13 — Footnotes in several containers

Expected: both definitions are present and readable; the repeated reference denotes the same note. Quote/list/table references must not drop the definition. This case does not require a particular numbering style or a backlink UI. Sources: [Glow #971](https://github.com/charmbracelet/glow/issues/971), [simov #258](https://github.com/simov/markdown-viewer/issues/258), [trsdn #12](https://github.com/trsdn/mdviewer/issues/12).

First reference.[^t2-long] Repeated reference.[^t2-long]

> Quoted reference.[^t2-short]

- List reference.[^t2-short]

| Place | Note |
|---|---|
| table cell | reference[^t2-long] |

[^t2-long]: T2-FOOTNOTE-LONG: first paragraph with **bold** and `code`.

    Second paragraph belonging to the same definition.

[^t2-short]: T2-FOOTNOTE-SHORT: the other definition.

T2-13-END.

## T2-14 — All documented alert types in both themes

Expected: types remain distinguishable and text readable in light and dark themes. Text/background contrast survives zoom and wrapping. Sources: [md-preview #24](https://github.com/vorojar/md-preview/issues/24), [#25](https://github.com/vorojar/md-preview/issues/25), [grip #364](https://github.com/joeyespo/grip/issues/364).

> [!NOTE]
> Note with **bold**, a [section link](#t2-15), and a second line.
>
> Second note paragraph.

> [!TIP]
> Tip with `code` and enough words to wrap when the outline is open in a narrow window.

> [!IMPORTANT]
> Important information remains visible after a theme change.

> [!WARNING]
> Warning with *emphasis*.

> [!CAUTION]
> Caution with a small list:
> - first item
> - second item

T2-14-END — no alert tint or indentation should leak here.

## T2-15 — Heading links and outline geometry {#t2-15}

Expected: links reach their respective target headings. The outline includes formatted and Setext headings, but not headings inside code. Repeat after changing width, zoom, and outline visibility. Sources: [Frogmouth #91](https://github.com/Textualize/frogmouth/issues/91), [#110](https://github.com/Textualize/frogmouth/issues/110), [simov 5.1](https://github.com/simov/markdown-viewer/releases/tag/5.1).

[First duplicate](#repeat-target) / [second duplicate](#repeat-target-1) / [explicit ID](#t2-explicit) / [Setext](#setext-target) / [Unicode](#caf%C3%A9-%E6%97%A5%E6%9C%AC%E8%AA%9E).

### Repeat target

FIRST-DUPLICATE-TARGET.

### Repeat target

SECOND-DUPLICATE-TARGET.

### **Formatted** heading with `code` {#t2-explicit}

EXPLICIT-TARGET.

Setext target
-------------

SETEXT-TARGET.

### Café 日本語

UNICODE-TARGET. This app documents practical slugs, not exact GitHub slug parity.

T2-15-END.

## T2-16 — Document-relative paths and canonical tab reuse

Expected: open this file from a working directory other than the repository. Both chapter links reach the same chapter tab and target; the two same-name files remain distinct tabs. Missing destinations produce a nonfatal error. Sources: [Frogmouth #52](https://github.com/Textualize/frogmouth/issues/52), [v0.7.0](https://github.com/Textualize/frogmouth/releases/tag/v0.7.0), [md-preview #41](https://github.com/vorojar/md-preview/issues/41), [v1.4.1](https://github.com/vorojar/md-preview/releases/tag/v1.4.1).

- [Chapter with percent-encoded space](tests/fixtures/torturetest-assets/chapter%20one.md#destination)
- [Same chapter through dot segments](tests/fixtures/torturetest-assets/./left/../chapter%20one.md#destination)
- [Left same-name document](tests/fixtures/torturetest-assets/left/same.md)
- [Right same-name document](tests/fixtures/torturetest-assets/right/same.md)
- [Missing file: deliberate error](tests/fixtures/torturetest-assets/intentionally-missing.md)
- [Missing heading: deliberate error](#intentionally-absent-heading)

T2-16-END — return to this tab; document and Find state remain intact.

## T2-17 — Images with spaces, Unicode, and relative bases

Expected: these three references display the same four-quadrant PNG. The top-left is red, top-right green, bottom-left blue, bottom-right yellow. No stretching or broken base paths. Source: [simov #242](https://github.com/simov/markdown-viewer/issues/242), [ekino v0.8.1](https://github.com/ekino/MarkdownViewer/releases/tag/v0.8.1). Use valid angle-bracket or percent-encoded destinations for spaces.

![Angle-bracket space path](<tests/fixtures/torturetest-assets/image space.png>)

![Encoded space path](tests/fixtures/torturetest-assets/image%20space.png)

![Unicode filename](tests/fixtures/torturetest-assets/café-日本語.png)

T2-17-END.

## T2-18 — Image descriptions containing markup and line breaks

Expected: image parsing never crashes; the valid PNG appears and missing images show useful alt-text placeholders and Retry. The description's text remains meaningful when flattened. Sources: [mdcat #285](https://github.com/swsnr/mdcat/issues/285), [#194](https://github.com/swsnr/mdcat/issues/194), [2.3.1](https://github.com/swsnr/mdcat/releases/tag/mdcat-2.3.1).

![**strong description** with `code` and *emphasis*](tests/fixtures/torturetest-assets/quadrants.png)

![description first line
description second line](tests/fixtures/torturetest-assets/missing-description.png)

![**Missing bold label** plus `identifier`](tests/fixtures/torturetest-assets/missing-alt.png)

T2-18-END — subsequent text must survive failed image loading.

## T2-19 — Image formats, SVG bounds, and malformed input

Expected: JPEG and WebP show the same quadrant pattern as PNG; the GIF displays its red first frame (animation is not required). SVG shows a wide blue rectangle with a yellow circle on the left; its aspect ratio stays 4:1. Corrupt PNG produces a readable error, not a crash. Sources: [md-preview #30](https://github.com/vorojar/md-preview/issues/30), [mdcat 2.1.0](https://github.com/swsnr/mdcat/releases/tag/mdcat-2.1.0); format and failure expectations are from this app's README.

![JPEG quadrants](tests/fixtures/torturetest-assets/quadrants.jpg)

![WebP quadrants](tests/fixtures/torturetest-assets/quadrants.webp)

![GIF first frame must be red](tests/fixtures/torturetest-assets/two-frame.gif)

![SVG nonzero viewBox origin](tests/fixtures/torturetest-assets/viewbox.svg)

![Deliberately corrupt PNG](tests/fixtures/torturetest-assets/corrupt.png)

T2-19-END.

## T2-20 — Search across formatting and one-to-zero transitions

Action: concatenate `T2` and `NEEDLE` in Find. There are exactly three occurrences in this file, below. Next/previous wraps through three results; selection lands on the visible text, including the formatted occurrence. Then append `Z` to the query: count becomes zero and all old highlights disappear. Also search the concatenation of `T2` and `FAR_RIGHT`: one result in T2-09. Append `Z` again to exercise the one-to-zero transition specifically. Clear Find and switch to the companion chapter. No stale highlights may leak there. Sources: [ekino v0.10.0](https://github.com/ekino/MarkdownViewer/releases/tag/v0.10.0), [v0.11.0](https://github.com/ekino/MarkdownViewer/releases/tag/v0.11.0).

T2NEEDLE

T2**NEE**DLE

```text
T2NEEDLE
```

T2-20-END.

## T2-21 — Wildcard search, literal punctuation, and case

Action: search the concatenation of `WILD` and `*FINISH`. It matches the two same-line samples; it must not span the separate paragraphs. Search `\*` to find actual asterisks, and `[square]` to verify punctuation is literal rather than regex syntax. Toggle case matching for the three case variants. Scope extension from [Frogmouth #16](https://github.com/Textualize/frogmouth/issues/16) and ekino's search fixes; wildcard semantics are specific to this app's README.

WILDalphaFINISH

WILDFINISH

WILD

FINISH

Literal punctuation sample: \* [square] (round) a+b? a.b ^cash$.

CaseBeacon casebeacon CASEBEACON.

T2-21-END.

## T2-22 — Unicode glyphs, combining marks, and table metrics

Expected: installed fallback fonts supply the documented scripts without blank boxes for these ordinary samples; combining accents stay attached and rows do not overlap. Exact mixed-direction paragraph ordering is reserved for T3-14. Source: [simov #275](https://github.com/simov/markdown-viewer/issues/275); corpus extended to this app's documented fallback fonts.

| Script | Sample |
|---|---|
| Latin and combining accents | café / café / naïve / Å |
| Japanese | 日本語の文章を表示します。 |
| Chinese | 中文阅读测试，检查换行。 |
| Korean | 한국어 문장 표시 테스트 |
| Arabic | العربية |
| Hebrew | עברית |
| Devanagari | हिन्दी नमस्ते |
| Thai | ภาษาไทย สวัสดี |
| Actual Unicode emoji | 😀 🚀 ✅ ⚠️ ❤️ |

T2-22-END — glyph coverage is distinct from full language-layout conformance.

## T2-23 — Definition lists with multiple definitions

Expected: two definitions belong to the first term; its continuation paragraph remains indented. The final ordinary paragraph is not part of a definition. This is a project-specific extension case derived from nested-block failures in [mdcat #302](https://github.com/swsnr/mdcat/issues/302), not an upstream definition-list bug claim.

First term with **bold**
: First definition with `code`.
: Second definition.

    Continued paragraph of the second definition.

Second term
: Another definition with enough ordinary words to wrap across several lines at a small width while remaining indented beneath its term.

T2-23-END — ordinary paragraph outside the definitions.

## T2-24 — Tabs, detach, focus, and session settings

Action: open the two same-name documents from T2-16; give each a different search and scroll position. Reorder tabs, switch repeatedly, then drag one outside the window. Cancel a second drag with Escape. Close the original window while the detached window remains. Expected: content and per-tab state survive; the detached window remains usable. Change theme, zoom, and wrapping across two windows: these are session-wide; outline visibility is per-window. Sources: [rajatarya v1.4.0](https://github.com/rajatarya/mdviewer/releases/tag/v1.4.0) (multiple-file open hang), [v1.8.5](https://github.com/rajatarya/mdviewer/releases/tag/v1.8.5) (focused-window close), [ekino v0.11.0](https://github.com/ekino/MarkdownViewer/releases/tag/v0.11.0).

Use only the disposable companion files for Rename file. Rename the left file to `renamed` without an extension; its local image should still load. Reject a collision with `occupied.md` in that folder. Reject an unsupported `.exe` extension. Restore its original name afterward. Reading and tab movement must never rewrite document contents. This rename subcase comes from the README, not an upstream bug report.

T2-24-END.

## T2-25 — Explicit reload after external replacement

Action: use a disposable copy of [reload sample](tests/fixtures/torturetest-assets/reload.md). Open it, scroll, and replace `RELOAD-VALUE-A` with `RELOAD-VALUE-B` in an external editor; save. Press F5, then repeat using Ctrl+R. Expected: new content appears, the tab identity and reading position are retained as far as the new document allows, and Ctrl+O still works. Temporarily move the disposable file elsewhere; reload must show an error while keeping the old readable document. Restore it and retry.

Sources: [md-preview #14](https://github.com/vorojar/md-preview/issues/14), [#18](https://github.com/vorojar/md-preview/issues/18). Automatic file watching is **not** being required here; replacement-save behavior is adapted to explicit reload.

T2-25-END.

## T2-26 — Resize and scroll stability under bounded stress

Open [stress sample](tests/fixtures/torturetest-assets/stress.md). It contains 160 numbered paragraphs, a 1024-character token in prose and a table, and a Rust fence whose content exceeds 256 KiB. Expected: sequential paragraphs never duplicate or reorder; no blank view after resize; long tokens remain reachable. The large fence stays readable and selectable while skipping highlighting, as documented. Do not impose a made-up timing or total-memory limit.

Alternate mouse-wheel, Page Down, Page Up, Home, and End. Toggle wrapping, outline, and zoom while scrolled down, then search for `STRESS-END`. Sources: [Glow #554](https://github.com/charmbracelet/glow/issues/554), [#983](https://github.com/charmbracelet/glow/issues/983), [#878](https://github.com/charmbracelet/glow/issues/878), [md-preview #42](https://github.com/vorojar/md-preview/issues/42). This app intentionally caps prose reading width at 960 logical pixels; wide margins alone are not a failure.

T2-26-END.

## T2-27 — Remote images, opt-in network exercise

This root document contains no automatic remote image requests. For this case only, open [network sample and steps](tests/fixtures/torturetest-assets/network.md). Expected: default-on loading, session-wide disabling, individual Load image, and Retry follow PRIVACY.md; network errors remain nonfatal. Sources: [grip #395](https://github.com/joeyespo/grip/issues/395), [ekino v0.8.1](https://github.com/ekino/MarkdownViewer/releases/tag/v0.8.1). Remote controls themselves are a project-specific extension of the image failure category.

T2-27-END.

## T2-28 — Safe fallback at end of file

Open [unclosed fence sample](tests/fixtures/torturetest-assets/unclosed-fence.md). Expected: CommonMark treats everything after the opening fence as code through EOF; the app remains responsive and can switch back to this tab. Source: [simov #254](https://github.com/simov/markdown-viewer/issues/254); this is a new boundary variant, not a verbatim upstream reproducer.

T2-28-END — final root-document sentinel. Reaching this is a visibility check, not proof that all cases passed.
