# Experimental macOS and Linux builds

These builds extend v0.1.5 and are preview artifacts, not a new stable release.
Windows remains the established supported platform. The native CI matrix builds,
lints and tests Linux x86_64, macOS Apple Silicon and macOS Intel separately.

## Linux

Initial baseline: Ubuntu 24.04 x86_64, glibc 2.39 or newer. The archive is not a
universal static Linux binary. X11 and Wayland are enabled; OpenGL/EGL and a
working desktop session are required. File dialogs use XDG Desktop Portal;
install the portal backend for your desktop (for example xdg-desktop-portal-gtk).

On Ubuntu 24.04, prefer the .deb: `sudo apt install ./FastMarkdownViewer-*.deb`.
It installs the executable and desktop entry and declares the runtime libraries,
including libxkbcommon-x11-0 (which is not present on every fresh desktop).
Remove it with `sudo apt remove fast-markdown-viewer`.
For the tar.gz, install libxkbcommon-x11-0, libegl1, libgl1 and your desktop portal
backend yourself if your distribution does not already include them.

Extract the tar.gz and run `./FastMarkdownViewer path/to/document.md` or open the
application and use Ctrl+O. To use the included desktop entry, put the executable
on your PATH (for example in ~/.local/bin) and copy the .desktop file into
~/.local/share/applications. It offers Markdown opening without changing your default.

## macOS

Choose aarch64 for Apple Silicon or x86_64 for Intel. Extract the ZIP and open
FastMarkdownViewer.app, then use Cmd+O or drag a file into the viewer.
The bundle is ad-hoc signed for integrity, not Developer ID signed or notarized.
Normal downloaded-app security checks therefore still apply.

Finder document associations and opening documents into an already-running app
are not advertised by this preview. Use the in-app file dialog or drag-and-drop.
The command-line executable lives in FastMarkdownViewer.app/Contents/MacOS.

Primary shortcuts use Cmd on macOS; tab cycling remains Ctrl+Tab.
Installed font choices and script coverage depend on local fonts. The bundled
default text and emoji fonts remain available on every platform; Mermaid uses
bundled Latin fonts when system fonts are unavailable. Font collection fallback
selection is heuristic and is not a promise of full multilingual typography.

## Build and verify

Install Rust 1.95.0. On Ubuntu install build-essential, pkg-config, libx11-dev,
libxi-dev, libxcursor-dev, libxrandr-dev, libxinerama-dev, libxkbcommon-dev,
libwayland-dev, libgl1-mesa-dev, libegl1-mesa-dev and libdbus-1-dev.
On macOS install the Xcode command line tools.

```
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release
python3 scripts/package-unix.py --platform linux-x86_64 --binary target/release/FastMarkdownViewer
```

For a Mac, substitute macos-aarch64 or macos-x86_64 in the packaging command.
Packages and checksums are uploaded by the Experimental desktop builds workflow.

## Desktop acceptance checklist

Before promoting a platform to supported, check the actual desktop: opening via
dialog and drag/drop, search, reload, local links, remote-image controls, math,
Mermaid success and error fallback, fonts, zoom, clipboard, tab detachment and
closing the original window. Check both X11 and Wayland on Linux. Check installing
and launching the extracted package rather than only the build directory.

WSL2/WSLg exercises a Linux executable and real graphics, but does not replace a
full desktop VM check of portals, desktop entries and file-manager integration.
