# Privacy and network behavior

FastMarkdownViewer has no telemetry, analytics, crash reporting, update checks, recent-file list, settings store, or persistent cache.

Opening a local document reads that file and any local images it references. The portable build does not write to the registry or AppData. The installed build writes only its program files, uninstall metadata, Start Menu shortcut, and its own per-user `Open with` registration.

## Remote images

A Markdown document can reference a remote image. Visible remote images load automatically by default on a background thread and therefore disclose the user's IP address and request time to that image host. **Settings → Automatically load remote images** turns automatic loading off for all windows in the session. Disabled images, including cached ones, show a **Load image** button; clicking it allows that URL for the session. Changing the toggle clears individual approvals. Requests already running may finish. These choices are not saved between launches. The request:

- allows only HTTP and HTTPS;
- contains a minimal product user agent and image `Accept` header;
- sends no cookies, URL credentials, referrer, or application state;
- revalidates every redirect and rejects private-network destinations;
- accepts at most 10 MiB of response data and at most 40 megapixels after decoding; and
- is cached only in memory, subject to eviction and tab-close/reload invalidation.

Local images also have a 10 MiB input limit. Raw image caches have 32 MiB payload budgets each for local and remote data, decoded images have a 192 MiB budget, and image textures have a 192 MiB budget; each cache also has a 256-entry limit. These are ceilings allocated on demand, not reservations or total-process memory limits. Active decoding, graphics-driver storage, fonts, math, and allocator-retained pages are additional memory. At most four image I/O workers and two decoding workers run at once. Raw and decoded copies are released after texture upload. Closing or reloading a tab invalidates its images; another tab sharing a URL may reload it.

Links are never fetched speculatively. HTTP(S) links open in the system browser only after a click. Relative `.md` and `.markdown` links open a separate FastMarkdownViewer process. Other schemes are inert.

Raw HTML is displayed as inert source text. FastMarkdownViewer does not embed a browser engine or execute scripts.
