FastMarkdownViewer v0.2.9 opens documents in the running viewer and lets you move tabs between its windows.

- Open multiple files from the desktop, the command line, or the file picker. The viewer adds them to the most recently focused window and selects an existing tab for a file that is already open.
- Drag a tab into another viewer window to move it with its reading position and Find state. Drop it outside the viewer windows to create a window. Native Wayland supports opening files in the running viewer, but does not expose the window positions needed for cross-window drops.
- Update the Markdown and TLS dependencies to address RustSec advisories.
- Keep dates consistently colored in Python console output.

Install using the platform download, `cargo binstall fast-markdown-viewer`, or `cargo install fast-markdown-viewer --locked`. Use the Windows installer, Mac app bundle, or Linux Debian package for desktop integration. See the [installation guide](https://github.com/Quetzalcohuatl/fastmarkdownviewer/blob/v0.2.9/docs/CROSS_PLATFORM.md) for platform requirements and verification steps.
