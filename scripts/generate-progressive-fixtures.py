"""Generate large-content framebuffer fixtures identically on every desktop OS."""
from pathlib import Path
import sys

output = Path(sys.argv[1])
output.mkdir(parents=True, exist_ok=True)
(output / "large-code.md").write_text(
    "# Deferred syntax highlighting\n\n"
    "This text and the code appear before syntax colors finish loading.\n\n```rust\n"
    + "let value = 123; // readable immediately\n" * 20_000
    + "```\n",
    encoding="utf-8",
)
(output / "large-mermaid.md").write_text(
    "# Large diagram\n\nThe diagram loads in the background and follows this theme.\n\n"
    "```mermaid\nsequenceDiagram\nparticipant A as Reader\nparticipant B as Renderer\n"
    + ("A->>B: " + "wide label " * 22 + "\n") * 70
    + "```\n",
    encoding="utf-8",
)
