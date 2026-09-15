"""Read-only validation of the cross-repository WinGet automation credential."""

import json
import os

from distribution_common import request, summary

token = os.environ.get("WINGET_GITHUB_TOKEN")
if not token:
    raise SystemExit("The repository Actions secret WINGET_GITHUB_TOKEN is not configured.")
with request("https://api.github.com/user", token=token) as response:
    identity = json.load(response)
    scopes = {scope.strip() for scope in response.headers.get("X-OAuth-Scopes", "").split(",")}
if identity["login"] != "Quetzalcohuatl":
    raise SystemExit("The WinGet token must belong to Quetzalcohuatl.")
if not scopes.intersection({"public_repo", "repo"}):
    raise SystemExit("The WinGet classic token requires public_repo permission.")
summary("WinGet credential identity and public repository scope verified; no repository changes made.")
