"""Small, credential-safe helpers for release distribution scripts (Python 3.11+)."""

import json
import os
import re
import urllib.error
import urllib.request
from pathlib import Path

REPOSITORY = "Quetzalcohuatl/fastmarkdownviewer"
PACKAGE_ID = "Quetzalcohuatl.FastMarkdownViewer"
USER_AGENT = "FastMarkdownViewer-release-distribution"


class SafeRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        redirected = super().redirect_request(req, fp, code, msg, headers, newurl)
        # Asset downloads redirect off GitHub; never forward API credentials.
        if redirected is not None:
            redirected.remove_header("Authorization")
        return redirected


def request(url, *, token=None, data=None, method=None):
    headers = {"User-Agent": USER_AGENT, "Accept": "application/vnd.github+json"}
    if token:
        headers["Authorization"] = "Bearer " + token
    if data is not None:
        headers["Content-Type"] = "application/json"
    req = urllib.request.Request(url, data=None if data is None else json.dumps(data).encode(),
                                 headers=headers, method=method)
    return urllib.request.build_opener(SafeRedirect()).open(req, timeout=60)


def github(path, *, token=None, data=None, method=None, missing_ok=False):
    try:
        with request("https://api.github.com/" + path, token=token, data=data, method=method) as response:
            body = response.read()
            return json.loads(body) if body else None
    except urllib.error.HTTPError as error:
        if missing_ok and error.code == 404:
            return None
        raise RuntimeError(f"GitHub {method or 'GET'} {path}: HTTP {error.code}") from None


def stable_version(tag):
    if not re.fullmatch(r"v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)", tag):
        raise ValueError("Distribution requires a stable vMAJOR.MINOR.PATCH release tag.")
    return tag[1:]


def output(name, value):
    value = str(value)
    if "\n" in value or "\r" in value:
        raise ValueError("Invalid multiline workflow output")
    print(f"{name}={value}", flush=True)
    if os.environ.get("GITHUB_OUTPUT"):
        with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as stream:
            stream.write(f"{name}={value}\n")


def summary(message):
    print(message, flush=True)
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with Path(os.environ["GITHUB_STEP_SUMMARY"]).open("a", encoding="utf-8") as stream:
            stream.write(message + "\n")
