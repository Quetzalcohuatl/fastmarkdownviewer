"""Verify an existing, successfully tested GitHub release before distribution."""

import argparse
import hashlib
import json
import os
import tomllib
from pathlib import Path
from urllib.parse import urlencode

from distribution_common import REPOSITORY, github, output, request, stable_version, summary


def verify_run(run, tag, sha):
    if not (run["conclusion"] == "success" and run["event"] == "push"
            and run["path"] == ".github/workflows/release.yml"
            and run["head_branch"] == tag and run["head_sha"] == sha
            and run["head_repository"]["full_name"] == REPOSITORY):
        raise ValueError("The release must have a successful official Release workflow for this exact tag and commit.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tag", required=True)
    parser.add_argument("--run-id")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    version = stable_version(args.tag)
    token = os.environ.get("GH_TOKEN")
    api = lambda path, **kwargs: github(f"repos/{REPOSITORY}/{path}", token=token, **kwargs)
    ref = api(f"git/ref/tags/{args.tag}")
    if ref["object"]["type"] != "tag":
        raise ValueError("The release tag must be annotated.")
    tag = api("git/tags/" + ref["object"]["sha"])
    if tag["object"]["type"] != "commit":
        raise ValueError("The release tag must point directly to a commit.")
    sha = tag["object"]["sha"]
    if args.run_id:
        run = api(f"actions/runs/{int(args.run_id)}")
        verify_run(run, args.tag, sha)
    else:
        query = urlencode({"head_sha": sha, "event": "push", "status": "success", "per_page": 100})
        runs = api("actions/workflows/release.yml/runs?" + query)["workflow_runs"]
        matches = [run for run in runs if run["head_branch"] == args.tag]
        if not matches:
            raise ValueError("No successful Release workflow exists for this release.")
        run = matches[0]
        verify_run(run, args.tag, sha)
    release = api(f"releases/tags/{args.tag}")
    if release["draft"] or release["prerelease"]:
        raise ValueError("Only published stable releases can be distributed.")
    with request(f"https://raw.githubusercontent.com/{REPOSITORY}/{sha}/Cargo.toml") as response:
        package = tomllib.loads(response.read().decode())["package"]
    if package["name"] != "fast-markdown-viewer" or package["version"] != version:
        raise ValueError("Release tag and Cargo package version disagree.")
    filename = f"FastMarkdownViewer-Setup-{version}.exe"
    assets = {asset["name"]: asset for asset in release["assets"]}
    installer = assets[filename]
    expected_url = f"https://github.com/{REPOSITORY}/releases/download/{args.tag}/{filename}"
    if installer["browser_download_url"] != expected_url:
        raise ValueError("Unexpected installer URL")
    with request(assets["SHA256SUMS.txt"]["browser_download_url"]) as response:
        checksums = response.read().decode().splitlines()
    expected = [line.split()[0].lower() for line in checksums if line.split()[-1].lstrip("*") == filename]
    with request(expected_url) as response:
        digest = hashlib.file_digest(response, "sha256").hexdigest()
    if expected != [digest]:
        raise ValueError("Released installer does not match SHA256SUMS.txt.")
    if installer.get("digest") and installer["digest"] != "sha256:" + digest:
        raise ValueError("Released installer does not match GitHub's asset digest.")
    result = {"tag": args.tag, "version": version, "sha": sha,
              "installer_url": expected_url, "installer_sha256": digest.upper(),
              "release_date": release["published_at"][:10], "release_url": release["html_url"],
              "release_run": run["html_url"]}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    for name in ("tag", "version", "sha"):
        output(name, result[name])
    summary(f"Verified [{args.tag}]({release['html_url']}), source `{sha}`, and installer SHA-256.")


if __name__ == "__main__":
    main()
