"""Install packaged FMV through a loopback Cargo mirror, without workspace paths.

Run `cargo package --workspace --locked` first. The pinned Cargo 1.95 toolchain
provides the staging index in target/package/tmp-registry. Existing crates.io
archives are reused, with HTTPS fallback for missing public dependencies.
No credentials or machine-wide Cargo configuration are changed.
"""

import argparse
import functools
import http.server
import json
import os
from pathlib import Path
import subprocess
import tempfile
import threading
import tomllib
import urllib.error
import urllib.request


ROOT = Path(__file__).resolve().parents[1]
STAGING = ROOT / "target/package/tmp-registry"
CARGO_HOME = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))


class Mirror(http.server.BaseHTTPRequestHandler):
    def log_message(self, *_args):
        pass

    def do_GET(self):
        path = self.path.split("?", 1)[0]
        if ".." in path.split("/"):
            self.send_error(400)
            return
        try:
            if path == "/index/config.json":
                body = json.dumps({"dl": f"http://127.0.0.1:{self.server.server_port}/crates"}).encode()
            elif path.startswith("/index/"):
                relative = path.removeprefix("/index/")
                local = STAGING / "index" / relative
                if local.is_file():
                    body = local.read_bytes()
                else:
                    body = public_index(relative)
            elif path.startswith("/crates/"):
                _, _, name, version, action = path.split("/")
                if action != "download":
                    self.send_error(404)
                    return
                archive = f"{name}-{version}.crate"
                local = STAGING / archive
                if not local.is_file():
                    local = next((CARGO_HOME / "registry/cache").glob(f"index.crates.io-*/{archive}"), local)
                if local.is_file():
                    body = local.read_bytes()
                else:
                    with urllib.request.urlopen(f"https://static.crates.io/crates/{name}/{archive}", timeout=60) as response:
                        body = response.read()
            else:
                self.send_error(404)
                return
            self.send_response(200)
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
        except urllib.error.HTTPError as error:
            self.send_error(error.code)
        except (OSError, ValueError) as error:
            print(f"Mirror request failed: {path}: {error}", flush=True)
            self.send_error(502)


@functools.lru_cache(maxsize=None)
def public_index(relative):
    with urllib.request.urlopen("https://index.crates.io/" + relative, timeout=60) as response:
        return response.read()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--release", action="store_true", help="Test an optimized Cargo install (default: debug).")
    args = parser.parse_args()
    version = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))["package"]["version"]
    if not (STAGING / f"fast-markdown-viewer-{version}.crate").is_file():
        raise SystemExit("Run cargo package --workspace --locked first.")
    with http.server.ThreadingHTTPServer(("127.0.0.1", 0), Mirror) as server:
        server.daemon_threads = True
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            # Owned temporary directory outside the checkout: Cargo cannot discover
            # the repository's paths, patches, or .cargo/config.toml here.
            with tempfile.TemporaryDirectory(prefix="fmv-registry-test-") as temporary:
                work = Path(temporary)
                index = f"sparse+http://127.0.0.1:{server.server_port}/index/"
                config = work / "registry.toml"
                config.write_text(f'''[registries.fmv-test]
index = "{index}"
[source.crates-io]
replace-with = "fmv-test"
[source.fmv-test]
registry = "{index}"
''', encoding="utf-8")
                command = ["cargo", "install", "fast-markdown-viewer", "--version", version,
                           "--registry", "fmv-test", "--locked", "--config", str(config),
                           "--root", str(work / "installed"),
                           "--target-dir", str(ROOT / "target/registry-install")]
                if not args.release:
                    command.append("--debug")
                subprocess.run(command, cwd=work, check=True)
                binary = work / "installed/bin/FastMarkdownViewer"
                if os.name == "nt":
                    binary = binary.with_suffix(".exe")
                actual = subprocess.check_output([str(binary), "--version"], text=True).strip()
                assert actual == f"FastMarkdownViewer {version}", actual
                diagram = work / "diagram.md"
                diagram.write_text("flowchart LR\n  A[Registry install] --> B[Patched renderer]\n", encoding="utf-8")
                # CLI worker smoke uses the actual registry-installed executable.
                subprocess.run([str(binary), "--internal-mermaid", str(diagram), str(work / "diagram.png")], check=True)
                assert (work / "diagram.png").read_bytes().startswith(b"\x89PNG\r\n\x1a\n")
                print(f"CARGO_REGISTRY_INSTALL=passed version={version}", flush=True)
        finally:
            server.shutdown()
            thread.join()


if __name__ == "__main__":
    main()
