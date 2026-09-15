"""Generate release manifests and idempotently submit them to microsoft/winget-pkgs."""

import argparse
import base64
import json
import os
import re
from pathlib import Path
from urllib.parse import urlencode

from distribution_common import PACKAGE_ID, REPOSITORY, github, stable_version, summary

UPSTREAM = "microsoft/winget-pkgs"
FORK = "Quetzalcohuatl/winget-pkgs"
MANIFEST_ROOT = "manifests/q/Quetzalcohuatl/FastMarkdownViewer"


def generate(release, templates, output):
    version = stable_version(release["tag"])
    if version != release["version"]:
        raise ValueError("Inconsistent release version")
    expected_url = f"https://github.com/{REPOSITORY}/releases/download/v{version}/FastMarkdownViewer-Setup-{version}.exe"
    if release["installer_url"] != expected_url or not re.fullmatch(r"[A-F0-9]{64}", release["installer_sha256"]):
        raise ValueError("Invalid verified installer metadata")
    if not re.fullmatch(r"\d{4}-\d{2}-\d{2}", release["release_date"]):
        raise ValueError("Invalid release date")
    candidates = [p for p in templates.iterdir() if p.is_dir() and re.fullmatch(r"\d+\.\d+\.\d+", p.name)]
    template = max(candidates, key=lambda p: tuple(map(int, p.name.split("."))))
    destination = output / MANIFEST_ROOT / version
    destination.mkdir(parents=True, exist_ok=True)
    files = {}
    for path in sorted(template.glob("*.yaml")):
        text = path.read_text(encoding="utf-8")
        text = re.sub(r"(?m)^(PackageVersion|  DisplayVersion): .*", r"\g<1>: " + version, text)
        text = re.sub(r"(?m)^- DisplayName: FastMarkdownViewer version .*",
                      "- DisplayName: FastMarkdownViewer version " + version, text)
        text = text.replace(f"/v{template.name}/", f"/v{version}/")
        text = text.replace(f"/tag/v{template.name}\n", f"/tag/v{version}\n")
        text = text.replace(f"FastMarkdownViewer-Setup-{template.name}.exe", f"FastMarkdownViewer-Setup-{version}.exe")
        text = re.sub(r"(?m)^ReleaseDate: .*", "ReleaseDate: " + release["release_date"], text)
        text = re.sub(r"(?m)^  InstallerSha256: .*", "  InstallerSha256: " + release["installer_sha256"], text)
        relative = f"{MANIFEST_ROOT}/{version}/{path.name}"
        files[relative] = text
        (destination / path.name).write_text(text, encoding="utf-8", newline="\n")
    if len(files) != 3:
        raise ValueError("Expected installer, default locale, and version manifests.")
    return files


def existing_submission(version, token):
    branch = "fastmarkdownviewer-" + version
    # The same branch naming as the initial manual submission makes retries
    # discover that PR too. Refuse to silently reopen an intentionally closed PR.
    query = urlencode({"state": "all", "head": "Quetzalcohuatl:" + branch, "base": "master", "per_page": 100})
    prs = github(f"repos/{UPSTREAM}/pulls?{query}", token=token)
    for pr in prs:
        if pr["state"] == "open" or pr.get("merged_at"):
            return pr["html_url"]
    if prs:
        raise ValueError("The submission PR was closed without merging; review it before retrying.")
    found = github(f"repos/{UPSTREAM}/contents/{MANIFEST_ROOT}/{version}", token=token, missing_ok=True)
    if found:
        return f"https://github.com/{UPSTREAM}/tree/master/{MANIFEST_ROOT}/{version}"
    return None


def submit(release, files, token):
    version = release["version"]
    existing = existing_submission(version, token)
    if existing:
        summary(f"WinGet submission already exists: {existing}")
        return existing
    if not token:
        raise ValueError("Set the WINGET_GITHUB_TOKEN Actions secret to submit to Microsoft.")
    identity = github("user", token=token)
    if identity["login"] != "Quetzalcohuatl":
        raise ValueError("The WinGet credential must belong to the package maintainer Quetzalcohuatl.")
    branch = "fastmarkdownviewer-" + version
    base = github(f"repos/{UPSTREAM}/git/ref/heads/master", token=token)["object"]["sha"]
    ref = github(f"repos/{FORK}/git/ref/heads/{branch}", token=token, missing_ok=True)
    parent = ref["object"]["sha"] if ref else base
    # Build one atomic commit atop the existing branch (a partially completed
    # retry), or Microsoft's current master. Never force-push another branch.
    changes = []
    for path, content in files.items():
        old = github(f"repos/{FORK}/contents/{path}?ref={branch}", token=token, missing_ok=True) if ref else None
        if old and base64.b64decode(old["content"]).decode() == content:
            continue
        changes.append({"path": path, "mode": "100644", "type": "blob", "content": content})
    if changes:
        parent_tree = github(f"repos/{FORK}/git/commits/{parent}", token=token)["tree"]["sha"]
        tree = github(f"repos/{FORK}/git/trees", token=token, data={"base_tree": parent_tree, "tree": changes})
        commit = github(f"repos/{FORK}/git/commits", token=token, data={
            "message": f"New version: {PACKAGE_ID} version {version}", "tree": tree["sha"], "parents": [parent]})
        if ref:
            github(f"repos/{FORK}/git/refs/heads/{branch}", token=token,
                   data={"sha": commit["sha"], "force": False}, method="PATCH")
        else:
            github(f"repos/{FORK}/git/refs", token=token,
                   data={"ref": "refs/heads/" + branch, "sha": commit["sha"]})
    body = (f"Updates **{PACKAGE_ID}** to **{version}** from the official [release]({release['release_url']}).\n\n"
            f"The installer passed the project's Windows install/version/uninstall checks in the "
            f"[release workflow]({release['release_run']}). Its SHA-256 matches the published "
            "SHA256SUMS.txt and was checked again before generating these manifests.\n\n"
            "Per-user x64 Inno Setup installer. Existing default file associations are preserved.\n\n"
            "This automated submission still requires Microsoft's validation and review.\n")
    pr = github(f"repos/{UPSTREAM}/pulls", token=token, data={
        "title": f"New version: {PACKAGE_ID} version {version}", "head": "Quetzalcohuatl:" + branch,
        "base": "master", "body": body})
    summary(f"WinGet PR submitted: {pr['html_url']}")
    return pr["html_url"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--release", type=Path, required=True)
    parser.add_argument("--templates", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--submit", action="store_true")
    args = parser.parse_args()
    release = json.loads(args.release.read_text(encoding="utf-8"))
    files = generate(release, args.templates, args.output)
    if args.submit:
        submit(release, files, os.environ.get("WINGET_GITHUB_TOKEN"))
    else:
        existing = existing_submission(release["version"], os.environ.get("GH_TOKEN"))
        summary(f"WinGet dry run: generated {len(files)} manifests. Existing submission: {existing or 'none'}")


if __name__ == "__main__":
    main()
