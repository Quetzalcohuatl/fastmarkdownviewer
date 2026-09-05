# Privacy and network behavior

FastMarkdownViewer has no telemetry, analytics, crash reporting, update checks, recent-file list, settings store, or persistent cache.

Opening a local document reads that file and any local images it references. The portable build does not write to the registry or AppData. The installed build writes only its program files, uninstall metadata, Start Menu shortcut, and its own per-user `Open with` registration.

## Remote images

A Markdown document can reference a remote image. Visible remote images load automatically on a background thread and therefore disclose the user's IP address and request time to that image host. The request:

- allows only HTTP and HTTPS;
- contains a minimal product user agent and image `Accept` header;
- sends no cookies, URL credentials, referrer, or application state;
- revalidates every redirect and rejects private-network destinations;
- accepts at most 10 MiB of response data and at most 40 megapixels after decoding; and
- remains only in memory until the process exits.

Links are never fetched speculatively. HTTP(S) links open in the system browser only after a click. Relative `.md` and `.markdown` links open a separate FastMarkdownViewer process. Other schemes are inert.

Raw HTML is displayed as inert source text. FastMarkdownViewer does not embed a browser engine or execute scripts.
