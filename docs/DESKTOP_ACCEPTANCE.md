# Desktop acceptance

## Release gates

Publication waits for Windows, macOS Apple Silicon, macOS Intel, Ubuntu, and
policy checks. All packages come from the same annotated tag. Checksums and
provenance cover every platform. The desktop workflow name is now Desktop builds;
its existing filename is retained so links and workflow history remain useful.

## v0.2.1 tab-label regression

The regression test first failed on an inactive Chinese tab whose document
contents were English. It now verifies actual font glyph coverage for that tab
and for a renamed Korean tab, while asserting that the inactive document stays
unloaded. All 88 Windows tests and 85 tests on each Mac architecture and Ubuntu
pass with the correction.

Windows and both Mac architectures' framebuffer captures show Japanese and Chinese tab
names without replacement squares. The Mac candidate uses an English document
named `First 日本語 中文 with spaces.md`, exercising filename-only fallback.
The [candidate checks and screenshots](https://github.com/Quetzalcohuatl/fastmarkdownviewer/actions/runs/34795939241)
also repeat the native package acceptance described below.

The Mac font inventory confirms that built-in Heiti is available when optional
PingFang is absent. Font discovery also includes system font asset folders for
Macs where PingFang is installed there. The tagged release repeats the platform
gates before publishing the final packages.

## Windows

The local v0.2.0 optimized x64 executable passed native open, scrolling, and
independent-window checks. Packaging/checksums and a fresh per-user
install/open/scroll/uninstall pass succeeded on Windows 11. This was an isolated
application profile on the existing host, not an exhaustive Windows version or
hardware matrix. Hosted release checks independently rebuild and verify their
exact assets.

## Ubuntu

A native acceptance pass on the installed candidate in an Ubuntu 24.04 Xfce/X11
VM passed Find, native clipboard, scrolling, normal exit, saved preferences,
reading-position/session restoration, the XDG portal file chooser, and opening
a second tab. Screenshots were inspected. The VM uses software Mesa graphics.
The earlier platform work also exercised detaching a tab and closing the original
window while its detached document remained open.

The permanent scripts/test-linux-desktop.py regression repeats the native
clipboard/dialog/session checks in an isolated X11 profile. CI additionally
captures real rendering through a Weston Wayland compositor; rendering-only
Wayland coverage is distinct from the X11 portal/clipboard acceptance.

## macOS

Both architectures run the Rust interaction suite and real framebuffer capture.
The extracted-package test uses LaunchServices (open -a), rather than CLI
arguments, for cold and running-app document opening. It verifies Unicode paths,
duplicate activation, normal Apple-event quit, saved-session/preferences, and
native Cmd+F/clipboard, pasting a path in the Cmd+O file picker, and quitting
while the native picker is still open. Native validation is
performed on macOS 15 CI desktops, not every Mac model or display arrangement.

The [completed candidate run](https://github.com/Quetzalcohuatl/fastmarkdownviewer/actions/runs/34792457119)
passed all three desktop jobs, including the native checks above and 84 Rust
tests on each Mac architecture and Ubuntu. Window readiness and separate input
steps allow the same native test to run on the slower Intel CI desktop.

## Coverage limits

Deterministic production-UI tests cover file drops, links, reload, search,
selection, themes, fonts, zoom, tab movement, window lifetimes, and errors on each
OS. Native package checks complement those tests; they do not claim exhaustive
physical drag gestures, multi-monitor arrangements, every portal backend, or
hardware GPU driver coverage. The Linux supported baseline is Ubuntu 24.04.
macOS deployment metadata is 15.0, matching the tested minimum.

Mac apps remain ad-hoc signed and unnotarized. Native tests do not claim that a
fresh quarantined internet download opens without an OS approval prompt.
