# Desktop installation and support

Desktop releases package Windows x64, macOS Apple Silicon/Intel, and Ubuntu x64 together.
Download packages and SHA256SUMS.txt from the same [GitHub release](https://github.com/Quetzalcohuatl/fastmarkdownviewer/releases).
All platform builds and package checks must succeed before publication.

| Platform | Supported baseline | Package |
|:--|:--|:--|
| Windows x64 | Windows 10 22H2 or Windows 11 | Portable EXE/ZIP or per-user installer |
| Apple Silicon Mac | macOS 15 or later | macos-aarch64.zip app bundle |
| Intel Mac | macOS 15 or later | macos-x86_64.zip app bundle |
| Linux x64 | Ubuntu 24.04, X11 or Wayland | .deb or linux-x86_64.tar.gz |

The test baseline is macOS 15 and Ubuntu 24.04. Later compatible OS versions are
expected to work; every OS, desktop, and graphics driver combination is not tested.
Other Linux distributions, older macOS versions, Linux ARM64, and Windows ARM64
are outside the supported release matrix.

## Windows

Run the portable EXE, or install the per-user setup. The installer registers .md
and .markdown under Open with without changing your default application.
Windows builds are unsigned. Checksums and GitHub provenance establish origin
and unchanged bytes, but do not remove SmartScreen prompts.

Starting in v0.2.5, Windows tries OpenGL first, then Direct3D 12 if graphics
initialization fails. If no usable hardware adapter is available, Windows' WARP
software renderer can draw the viewer on the CPU. This supports basic display
drivers and virtual machines without usable OpenGL; software rendering can be
slower and use more CPU. No separate graphics runtime download is needed on the
supported Windows baseline. macOS and Linux retain their existing renderer.

For troubleshooting, set `FMV_GRAPHICS=software` before launching to force WARP,
or `FMV_GRAPHICS=direct3d` to bypass OpenGL. Remove the variable to restore
automatic selection. `FMV_GRAPHICS=opengl` disables fallback for diagnosis.
Startup diagnostics identify the selected backend and, for Direct3D, the adapter.

## macOS

Extract the ZIP, move FastMarkdownViewer.app to Applications, and open it.
Use Finder's Open With → FastMarkdownViewer for .md and .markdown files;
choosing it as the default remains your decision. Finder can send additional
files to the running app, reusing a tab when its canonical path is already open.
In-app Cmd+O and drag-and-drop also open documents.

The bundle is ad-hoc signed for integrity. It is not Developer ID signed or
notarized, so downloaded-app security checks can require approval in macOS
Privacy & Security. No signing credentials are included in the repository.

Primary shortcuts use Cmd on macOS; tab cycling uses Ctrl+Tab.
Cmd+Shift+O toggles the outline; Cmd+H retains the system Hide behavior.
The CLI executable is FastMarkdownViewer.app/Contents/MacOS/FastMarkdownViewer.

## Ubuntu

Prefer the Debian package, which installs the executable and desktop entry and
declares the runtime libraries:

```sh
sudo apt install ./FastMarkdownViewer-0.2.6-linux-x86_64.deb
sudo apt remove fast-markdown-viewer
```

The package offers Markdown opening without changing your default application.
For the tar archive, install libxkbcommon-x11-0, libegl1, libgl1, and your
desktop's XDG portal backend if missing. Extract and run
`./FastMarkdownViewer path/to/document.md`. The executable requires glibc 2.39 or
newer, working OpenGL/EGL, and a desktop session; it is not a static universal
Linux binary. Dialogs use XDG Desktop Portal (for example its GTK or KDE backend).

To integrate the tar archive with the desktop, put its executable on PATH (for
example ~/.local/bin) and its .desktop file in ~/.local/share/applications.

## Preferences, sessions, and limits

All packages save appearance and session state on normal exit; see
[saved settings and sessions](../README.md#saved-settings-and-sessions).
Independent processes share that state file: the last normal exit wins.
Forced termination can lose changes since the last exit.

Installed fonts and script coverage depend on local fonts. Bundled default text
and emoji remain available; Mermaid uses bundled Latin fonts when system fonts
are unavailable. Full mixed-direction paragraph layout and native accessibility
in detached child windows retain the documented renderer limitations.

## Verification and development

The [desktop workflow](../.github/workflows/experimental-desktop.yml) runs native
builds, formatting, Clippy, application tests, graphics captures, and packaging
on Ubuntu and both Mac architectures. Package checks exercise Finder document
events and session restoration on Mac, and install/uninstall plus the native
portal/clipboard/session workflow on Linux. Linux graphics captures cover X11
and a Weston Wayland compositor. Exact validation results and coverage limits
are recorded in [desktop acceptance](DESKTOP_ACCEPTANCE.md).

For development, install Rust 1.95.0. Ubuntu requires build-essential, pkg-config,
libx11-dev, libxi-dev, libxcursor-dev, libxrandr-dev, libxinerama-dev,
libxkbcommon-dev, libwayland-dev, libgl1-mesa-dev, libegl1-mesa-dev, and
libdbus-1-dev. Mac builds require the Xcode command line tools and
MACOSX_DEPLOYMENT_TARGET=15.0.

```sh
cargo fmt -p fast-markdown-viewer -p fmv-macos-events -- --check
cargo clippy --locked --all-targets --no-deps -- -D warnings
cargo test --locked --all-targets
cargo build --locked --release
python3 scripts/package-unix.py --platform linux-x86_64 --binary target/release/FastMarkdownViewer
```

On a Mac, substitute macos-aarch64 or macos-x86_64 in the packaging command.
When building from WSL with the source on a Windows drive, set --output to a
Linux-filesystem directory so executable and Debian package permissions survive.
