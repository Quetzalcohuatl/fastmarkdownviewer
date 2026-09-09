# Layout regression examples

This table has columns with wildly different natural widths.

| ID | Required | Description | Extremely Long Field | Notes |
| -: | -------- | ----------- | -------------------- | ----- |
| 1 | Yes | Short description | tiny | fine |
| 2 | No | This description is much longer and should ideally receive more width than the tiny columns around it. | ThisIsASingleVeryLongUnbrokenTokenDesignedToStressColumnSizingAndHorizontalOverflowBehavior1234567890 | some notes |
| 3 | Yes | A medium-length sentence that may wrap onto multiple lines depending on the available viewport width. | another moderately long field | Another note that could wrap |
| 4 | No | Short | `inline_code_with_a_really_really_really_long_identifier_name()` | end |

This is a deliberately long paragraph intended to test readable line width, wrapping, zoom behavior, and whether the viewer maintains a sensible content column as the window becomes extremely narrow or extremely wide. The paragraph keeps going with enough text to force multiple lines on almost any practical display. A good viewer should remain readable, avoid clipping, and avoid introducing strange horizontal scrolling for ordinary prose even when nearby tables or code blocks are much wider than the main text column.

## Nested quotes

> Outer quote begins.
>
> > Inner quote with **bold** text.
> >
> > > Deep quote.
> >
> > Inner quote continues.
>
> Outer quote continues.
>
> - A quoted list item
> - Another item with *emphasis*

Outside the quote.

## Definitions

Markdown
: A lightweight markup language with **formatted definitions**.
: A second definition for the same term.

Viewer
: A read-only application.

    A second paragraph inside this definition.

After the definitions.

## Nested formatting

**Bold with *italic inside* and `code inside bold` and [a link](https://example.com).**

**Bold with&#x20;*****italic inside*****&#x20;and&#x20;****`code inside bold`****&#x20;and&#x20;****[a link](https://example.com)****.**

~~Strikethrough with **bold inside**~~

## Missing images

![Missing diagram](does-not-exist.png)

![](also-missing.png)

## Script samples

English 日本語 中文 한국어 العربية עברית 🙂

हिन्दी देवनागरी

ภาษาไทย
