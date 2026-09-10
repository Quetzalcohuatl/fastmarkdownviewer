# Optional remote-image exercise

Opening this file with automatic remote images enabled may make HTTPS requests
to raw.githubusercontent.com and example.invalid. The latter is a reserved
invalid domain and is deliberately expected to fail. A failure to reach the real
host is a network result, not by itself proof of a renderer defect.

1. Before opening this file, turn off Settings → Automatically load remote images.
2. Open it: both remote references should show individual Load image controls.
3. Click Load image on the first reference: it should load if the host is reachable.
4. Enable automatic images. The second reference should fail nonfatally with a
   useful placeholder; try Retry and confirm the document remains responsive.
5. Disable automatic images again. The already-loaded remote image should hide.
6. Repeat across a detached window to check the session-wide setting.
7. F5 reload and tab close/reopen should not show stale image state or crash.

Use a network inspector if asserting that a request did or did not occur; visual
placeholders alone cannot prove the absence of network requests. Requests already
in flight may finish after disabling the toggle, as documented in PRIVACY.md.

![Public GitHub image, network dependent](https://raw.githubusercontent.com/github/explore/main/topics/markdown/markdown.png)

![Deliberate DNS failure](https://example.invalid/torturetest2-missing.png)

NETWORK-END

[Return](../../../torturetest2.md)
