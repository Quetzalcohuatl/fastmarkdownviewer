FastMarkdownViewer v0.1.4 adds more reading themes and separate text/code font choices, including Solarized Light.

- **Settings → Color theme:** Solarized Light, Solarized Dark, Quiet Light, Monokai, and Tomorrow Night Blue join System, Light, and Dark. Code highlighting follows the palette, including switches between two light or two dark themes.
- **Settings → Text font / Code font:** choose supported installed fonts independently. Available choices include Segoe UI, Arial, Calibri, Georgia, Times New Roman, Consolas, Courier New, Cascadia Code/Mono, JetBrains Mono, and Fira Code. Only installed choices appear; no fonts are downloaded or redistributed. Emoji and multilingual fallbacks are retained.
- Appearance changes apply across windows in the current process. Returning to System restores the standard Windows-following light/dark behavior. Settings remain session-only and reset on restart.
- The repository now includes `torturetest2.md` (28 supported-behavior cases), `torturetest3.md` (20 unclaimed/limited-feature cases), local companion assets, and research from 10 open-source viewers. These are manual test suites, not a claim that every exploratory case passes.

The themes are lightweight native interpretations of familiar editor palettes, not VS Code extension or arbitrary theme-file support. Font selection is a curated installed-font list. The release has 73 automated tests, including menu selection, syntax-palette changes, independent fonts, and fallback preservation. No new viewer dependencies were added.

Download the Windows x64 portable EXE, portable ZIP, or per-user installer below. Builds remain unsigned; verify with SHA256SUMS.txt and GitHub provenance attestations. Automatic remote images remain enabled by default and can be disabled in Settings; see PRIVACY.md.
