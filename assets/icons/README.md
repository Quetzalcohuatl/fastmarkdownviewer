# Application icons

Keep the blue folded-document identity, with artwork suited to each desktop:

- **macOS:** a rounded blue tile with a transparent outer margin, standard and
  Retina representations from 16 to 1024 pixels. Packaging uses native `iconutil`
  to create `Contents/Resources/AppIcon.icns` and registers it in `Info.plist`.
- **Windows:** a transparent document silhouette with fifteen exact ICO sizes
  from 16 to 256 pixels, including common fractional display scales. The EXE and
  installer share the same asset. Small representations simplify and align the
  lines to pixels instead of shrinking a large bitmap.
- **Linux:** a document silhouette with restrained depth, eight PNG sizes from
  16 to 512 pixels, and an SVG in the `hicolor` fallback theme. The desktop entry,
  X11 window class, Wayland application ID, and icon name share `FastMarkdownViewer`.

Sources are the SVGs in this directory. Regenerate the committed assets with:

```sh
cargo run --locked --example generate_icons
python scripts/test-icons.py
```

Only developers regenerate artwork. Builds embed PNG/ICO files and copy or pack
the platform assets; startup never renders an SVG. The runtime PNG decodes once
per process and is reused by detached windows. macOS Dock/Finder and Wayland
launchers use the packaged icon; the runtime window icon API is unsupported on
those platforms. Install the app bundle or desktop package for that integration.

Design references: [Windows icon sizes and construction](https://learn.microsoft.com/en-us/windows/apps/design/iconography/app-icon-construction),
[Apple standard and Retina iconsets](https://developer.apple.com/library/archive/documentation/GraphicsAnimation/Conceptual/HighResolutionOSX/Optimizing/Optimizing.html),
[GNOME app icon guidance](https://developer.gnome.org/hig/guidelines/app-icons.html),
and the [freedesktop icon theme specification](https://specifications.freedesktop.org/icon-theme/latest/).

Native checks verify PNG bytes inside the Windows executable, extract all ten
representations from the bundled macOS ICNS, and verify installed icon paths in
the Debian package. Assets should also be viewed at actual small sizes on light
and dark backgrounds. Existing pinned shortcuts may retain an OS-cached icon
until the app is relaunched or the shortcut is pinned again.
