# Privacy and network behavior

FastMarkdownViewer has no telemetry, analytics, crash reporting, update checks, browsing-history database or persistent content cache.

Opening a local document reads that file and any local images it references. Both portable and installed builds save preferences and the last session locally on normal exit. The settings file contains file paths and reading positions, but no document contents or search queries. Locations and reset instructions are in [Saved settings and sessions](README.md#saved-settings-and-sessions). The portable build does not write to the registry. The installer additionally writes program files, uninstall metadata, a Start Menu shortcut, and per-user `Open with` registration.

Mermaid rendering runs diagrams offline using a hidden child process of the same executable. Diagram source, geometry, PNG tiles, and any rendering error are exchanged through a temporary directory. Successful diagrams retain their tiles until tab close, reload, or a theme/font change; cleanup then runs in the background. Failed renders clean up their temporary data. Abrupt application termination can leave temporary files behind. Rendering is serialized, starts when visible, and supports cancellation without an automatic time deadline. There is no application source/output byte limit, whole-diagram megapixel limit, or diagram-count cache limit. Available RAM and temporary disk space still constrain rendering, and the flowchart crash guards remain; see [limits and compatibility](README.md#limits-and-compatibility).

The explicit **Rename file…** tab action changes the filename on disk in the same folder. It does not edit document contents or rewrite other documents' links. **Show in Explorer** passes the local path to the operating system's file manager.

## Remote images

A Markdown document can reference a remote image. Visible remote images load automatically by default on a background thread and therefore disclose the user's IP address and request time to that image host. **Settings → Automatically load remote images** turns automatic loading off for all windows in the session. Disabled images, including cached ones, show a **Load image** button; clicking it allows that URL for the session. Changing the toggle clears individual approvals. Requests already running may finish. The automatic-loading preference is saved between launches; individual URL approvals are not. Restoring a session can load remote images in visible documents when automatic loading is enabled. The request:

- allows only HTTP and HTTPS;
- contains a minimal product user agent and image `Accept` header;
- sends no cookies, URL credentials, referrer, or application state;
- revalidates every redirect and rejects private-network destinations;
- follows up to five redirects, with an eight-second connection timeout and a twenty-second idle-read timeout; and
- is cached only in memory, subject to eviction and tab-close/reload invalidation.

Local and remote images have no application input-byte or decoded-pixel cutoff. Decoding happens in the background; output is sized for the display and the graphics hardware's maximum texture dimension. Raster formats can require their full decoded image in RAM before downsampling. SVGs rasterize at the requested display size. Invalid data and allocation failures can still fail to load.

Raw image caches have 32 MiB payload targets each for local and remote data, decoded images have a 192 MiB target, and image textures have a 192 MiB target; each cache also targets at most 256 entries. These are soft eviction targets allocated on demand, not admission limits, reservations, or total-process memory limits. The newest individual result may exceed a target so a large resource is not immediately evicted and repeatedly loaded. Pending work, active decoding, graphics-driver storage, fonts, math, and allocator-retained pages are additional memory. At most four image I/O workers and two decoding workers run at once. Raw and decoded copies are released after texture upload. Closing or reloading a tab invalidates its images; another tab sharing a URL may reload it.

Links are never fetched speculatively. HTTP(S) links open in the system browser only after a click. Local `.md` and `.markdown` links open or activate a tab in the current window. Other schemes are inert.

Raw HTML is displayed as inert source text. FastMarkdownViewer does not embed a browser engine or execute scripts.
