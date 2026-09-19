"""Install official release packages with cargo-binstall, with no Rust on PATH.

Before release, --packages serves the exact locally built archive over loopback;
the manifest's format and binary path are unchanged. --published tests crates.io
metadata and real GitHub downloads. All installs use an isolated Cargo home.
"""

import argparse
import functools
import hashlib
import http.server
import io
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
import threading
import tomllib
import urllib.request
import zipfile

ROOT = Path(__file__).resolve().parents[1]
TOOL_VERSION = "1.23.0"
# Official cargo-binstall release asset digests; never compile the installer.
TOOLS = {
    "windows-x86_64": ("x86_64-pc-windows-msvc", "zip", "f4641479477aca40387e88297e3813fab8e44a8d21f25faa44e0ff33e2bc1726"),
    "linux-x86_64": ("x86_64-unknown-linux-musl", "tgz", "64bf954c68bb558431deeabecaec7687edd5541c2189ee263bb8bc18bc4fdf55"),
    "macos-aarch64": ("aarch64-apple-darwin", "zip", "0f679c0bc992c6b84fdcbb0f65492588447d26596ef387cb6cb1ba41ad8ceb33"),
    "macos-x86_64": ("x86_64-apple-darwin", "zip", "5b4d6cd99651318e17a26ed9bab7505de0f496d88793a7735a0fc6e2079efe83"),
}
TARGETS = {
    "windows-x86_64": "x86_64-pc-windows-msvc",
    "linux-x86_64": "x86_64-unknown-linux-gnu",
    "macos-aarch64": "aarch64-apple-darwin",
    "macos-x86_64": "x86_64-apple-darwin",
}


def fetch(url):
    request = urllib.request.Request(url, headers={"User-Agent": "FMV-binstall-check"})
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read()


def member_bytes(data, archive_format, name):
    if archive_format == "zip":
        with zipfile.ZipFile(io.BytesIO(data)) as archive:
            return archive.read(name)
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        return archive.extractfile(name).read()


def bootstrap(destination, host):
    target, archive_format, digest = TOOLS[host]
    url = f"https://github.com/cargo-bins/cargo-binstall/releases/download/v{TOOL_VERSION}/cargo-binstall-{target}.{archive_format}"
    data = fetch(url)
    if hashlib.sha256(data).hexdigest() != digest:
        raise ValueError("cargo-binstall download checksum mismatch")
    name = "cargo-binstall.exe" if host.startswith("windows") else "cargo-binstall"
    path = destination / name
    path.write_bytes(member_bytes(data, archive_format, name))
    path.chmod(0o755)
    return path


def render(template, values):
    return re.sub(r"\{\s*([\w-]+)\s*\}", lambda match: values[match[1]], template)


class QuietHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, *_args):
        pass

    def copyfile(self, source, output):
        try:
            super().copyfile(source, output)
        except (BrokenPipeError, ConnectionResetError, ConnectionAbortedError):
            pass  # Binstall may cancel an archive probe before downloading it fully.


def run(command, env, cwd, timeout=120, expected_code=0):
    result = subprocess.run(command, env=env, cwd=cwd, text=True, encoding="utf-8",
                            errors="replace", stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                            timeout=timeout)
    print(result.stdout, flush=True)
    if result.returncode != expected_code:
        raise RuntimeError(f"Command exited {result.returncode}; expected {expected_code}")
    return result.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--platform", choices=TARGETS, required=True)
    parser.add_argument("--manifest", type=Path, default=ROOT / "Cargo.toml")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--packages", type=Path)
    mode.add_argument("--published", action="store_true")
    args = parser.parse_args()
    manifest = tomllib.loads(args.manifest.read_text(encoding="utf-8"))
    package = manifest["package"]
    metadata = package["metadata"]["binstall"]
    assert set(metadata["disabled-strategies"]) == {"quick-install", "compile"}
    assert set(metadata["overrides"]) == set(TARGETS.values())
    target = TARGETS[args.platform]
    settings = dict(metadata, **metadata["overrides"][target])
    version = package["version"]
    binary = "FastMarkdownViewer.exe" if args.platform.startswith("windows") else "FastMarkdownViewer"
    values = {"repo": package["repository"], "version": version, "bin": "FastMarkdownViewer",
              "binary-ext": ".exe" if args.platform.startswith("windows") else "", "target": target}
    url = render(settings["pkg-url"], values)
    extension = "tar.gz" if args.platform.startswith("linux") else "zip"
    filename = f"FastMarkdownViewer-{version}-{args.platform}.{extension}"
    assert url == f"https://github.com/Quetzalcohuatl/fastmarkdownviewer/releases/download/v{version}/{filename}", url
    member = render(settings["bin-dir"], values)
    data = (args.packages / filename).read_bytes() if args.packages else fetch(url)
    expected = member_bytes(data, settings["pkg-fmt"], member)

    with tempfile.TemporaryDirectory(prefix="fmv-binstall-") as temporary:
        folder = Path(temporary)
        tool = bootstrap(folder, args.platform)
        empty_path = folder / "no-toolchain"
        empty_path.mkdir()
        env = dict(os.environ, PATH=str(empty_path), CARGO_HOME=str(folder / "cargo-home"),
                   RUSTUP_HOME=str(folder / "rustup-home"))
        for variable in ("CARGO_INSTALL_ROOT", "RUSTUP_TOOLCHAIN", "CARGO", "RUSTC"):
            env.pop(variable, None)
        assert not shutil.which("cargo", path=env["PATH"])
        assert not shutil.which("rustc", path=env["PATH"])
        command = [str(tool), package["name"], "--version", version, "--root", str(folder / "install"),
                   "--no-confirm", "--disable-telemetry", "--no-discover-github-token"]
        server = None
        if args.packages:
            handler = functools.partial(QuietHandler, directory=str(args.packages.resolve()))
            server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
            threading.Thread(target=server.serve_forever, daemon=True).start()
            command += ["--manifest-path", str(args.manifest.resolve()), "--pkg-url",
                        f"http://127.0.0.1:{server.server_port}/{filename}",
                        "--allow-insecure-http"]  # Loopback fixture only; published downloads use HTTPS.
        try:
            if server:
                unsupported = command + ["--targets", "x86_64-unknown-freebsd"]
                unsupported[unsupported.index("--pkg-url") + 1] = f"http://127.0.0.1:{server.server_port}/unavailable.tar.gz"
                output = run(unsupported, env, folder, expected_code=94)
                assert "Fallback to cargo-install is disabled" in output, output
            run(command, env, folder)
        finally:
            if server:
                server.shutdown()
                server.server_close()
        installed = folder / "install/bin" / binary
        assert installed.read_bytes() == expected, "Installed binary differs from the release archive"
        output = run([str(installed), "--version"], env, folder)
        assert output.strip() == f"FastMarkdownViewer {version}", output
        source, image = folder / "diagram.mmd", folder / "diagram.png"
        source.write_text("flowchart LR\n A[Install] --> B[Read]\n", encoding="utf-8")
        run([str(installed), "--internal-mermaid", str(source), str(image)], env, folder)
        assert image.read_bytes().startswith(b"\x89PNG\r\n\x1a\n"), "Installed diagram renderer failed"
        print(f"BINSTALL_SMOKE=passed platform={args.platform} version={version} rust=absent bytes=verified published={args.published}")


if __name__ == "__main__":
    main()
