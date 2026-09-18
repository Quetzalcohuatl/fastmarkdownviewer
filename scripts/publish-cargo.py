"""Plan or publish only missing workspace versions; reject changed published sources."""

import argparse
import hashlib
import io
import json
import os
import subprocess
import tarfile
import tempfile
import urllib.error
from pathlib import Path

from distribution_common import output, request, summary


def archive_files(data, *, library):
    # Cargo writes checkout provenance for each tag. Library lockfiles are not
    # used by consumers and can change with unrelated workspace app releases.
    ignored = {".cargo_vcs_info.json"}
    if library:
        ignored.add("Cargo.lock")
    result = {}
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        for member in archive.getmembers():
            name = member.name.split("/", 1)[1]
            if not member.isfile() or name in ignored:
                continue
            content = archive.extractfile(member).read()
            # The initial Windows publication contained a few CRLF text files;
            # a clean checkout applies .gitattributes eol=lf. Compare their text
            # consistently, while retaining exact bytes for binary assets.
            if Path(name).suffix in {".rs", ".md", ".toml", ".txt"} or Path(name).name.startswith("LICENSE"):
                content = content.decode("utf-8").replace("\r\n", "\n").encode("utf-8")
            result[name] = content
    return result


def published_version(name, version):
    try:
        with request(f"https://crates.io/api/v1/crates/{name}/{version}") as response:
            return json.load(response)["version"]
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return None
        raise


def published_archive(target, name, version):
    directory = Path(target) / "package"
    filename = f"{name}-{version}.crate"
    # Cargo publish stages single-crate uploads in tmp-crate; workspace
    # packaging uses tmp-registry, and cargo package -p uses package directly.
    for parent in (directory / "tmp-crate", directory / "tmp-registry", directory):
        candidate = parent / filename
        if candidate.is_file():
            return candidate
    raise FileNotFoundError(f"Cargo did not retain the uploaded archive for {name} {version} in {directory}")


def plan(source):
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--locked", "--no-deps", "--format-version", "1"], cwd=source))
    packages = [p for p in metadata["packages"] if p["id"] in metadata["workspace_members"] and p["publish"] != []]
    staging = Path(metadata["target_directory"]) / "package/tmp-registry"
    missing, existing = [], []
    for package in sorted(packages, key=lambda p: p["name"] == "fast-markdown-viewer"):
        name, version = package["name"], package["version"]
        local = (staging / f"{name}-{version}.crate").read_bytes()
        remote = published_version(name, version)
        if remote is None:
            missing.append(name)
            summary(f"Publish `{name} {version}`.")
            continue
        if remote["yanked"]:
            raise ValueError(f"{name} {version} is yanked; bump its version.")
        if name == "fast-markdown-viewer":
            # Workspace staging rebuilds library archives, changing their VCS
            # checksums even when sources are unchanged. A published app must be
            # packaged against the real registry before comparing its lockfile.
            with tempfile.TemporaryDirectory(prefix="fmv-cargo-compare-") as temporary:
                subprocess.run(["cargo", "package", "-p", name, "--locked", "--no-verify"], cwd=source,
                               env=dict(os.environ, CARGO_TARGET_DIR=temporary), check=True)
                local = (Path(temporary) / "package" / f"{name}-{version}.crate").read_bytes()
        with request(f"https://static.crates.io/crates/{name}/{name}-{version}.crate") as response:
            published = response.read()
        if hashlib.sha256(published).hexdigest() != remote["checksum"]:
            raise ValueError(f"Registry checksum mismatch for {name} {version}.")
        library = name != "fast-markdown-viewer"
        before, after = archive_files(published, library=library), archive_files(local, library=library)
        changed = sorted(key for key in before.keys() | after.keys() if before.get(key) != after.get(key))
        if changed:
            raise ValueError(f"{name} {version} is already published but differs in {', '.join(changed)}. Bump its version and release again.")
        existing.append(name)
        summary(f"Already published, unchanged: `{name} {version}`.")
    return missing, existing


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--publish", action="store_true")
    args = parser.parse_args()
    missing, existing = plan(args.source.resolve())
    output("needed", str(bool(missing)).lower())
    if not args.publish or not missing:
        return
    if not os.environ.get("CARGO_REGISTRY_TOKEN"):
        raise ValueError("Missing temporary crates.io publishing credential.")
    # CI has already verified these exact packages on all native platforms.
    # Cargo orders unpublished workspace dependencies and waits for indexing.
    command = ["cargo", "publish", "--workspace", "--locked", "--no-verify"]
    for name in existing:
        command.extend(["--exclude", name])
    # Fresh staging prevents an earlier all-workspace package operation from
    # substituting locally rebuilt versions of already-published dependencies.
    with tempfile.TemporaryDirectory(prefix="fmv-cargo-publish-") as temporary:
        subprocess.run(command, cwd=args.source,
                       env=dict(os.environ, CARGO_TARGET_DIR=temporary), check=True)
        metadata = json.loads(subprocess.check_output(
            ["cargo", "metadata", "--locked", "--no-deps", "--format-version", "1"], cwd=args.source))
        for package in metadata["packages"]:
            name, version = package["name"], package["version"]
            if name not in missing:
                continue
            archive = published_archive(temporary, name, version)
            remote = published_version(name, version)
            if not remote or remote["checksum"] != hashlib.sha256(archive.read_bytes()).hexdigest():
                raise ValueError(f"Published archive verification failed for {name} {version}.")
    summary("All release crates are published and the uploaded archive checksums verified.")


if __name__ == "__main__":
    main()
