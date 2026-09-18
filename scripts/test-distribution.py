"""Offline regression tests for irreversible release distribution decisions."""

import importlib.util
import hashlib
import io
import json
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch

from distribution_common import REPOSITORY, stable_version

ROOT = Path(__file__).resolve().parents[1]


def load(name):
    spec = importlib.util.spec_from_file_location(name, ROOT / "scripts" / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


cargo = load("publish-cargo")
winget = load("submit-winget")
prepare = load("prepare-distribution")


def archive(files):
    stream = io.BytesIO()
    with tarfile.open(fileobj=stream, mode="w:gz") as tar:
        for name, content in files.items():
            info = tarfile.TarInfo("crate-1.0.0/" + name)
            info.size = len(content)
            tar.addfile(info, io.BytesIO(content))
    return stream.getvalue()


class DistributionTests(unittest.TestCase):
    def test_only_stable_tags(self):
        self.assertEqual(stable_version("v1.2.3"), "1.2.3")
        for tag in ("v1.2.3-rc.1", "v1.2.3+build", "main", "v01.2.3", "v1.2.3\n", "v1.2.3; echo bad"):
            with self.subTest(tag=tag), self.assertRaises(ValueError):
                stable_version(tag)

    def test_reject_wrong_release_provenance(self):
        run = {"conclusion": "success", "event": "push", "path": ".github/workflows/release.yml",
               "head_branch": "v1.2.3", "head_sha": "abc", "head_repository": {"full_name": REPOSITORY}}
        prepare.verify_run(run, "v1.2.3", "abc")
        for key, value in (("conclusion", "failure"), ("event", "pull_request"),
                           ("path", ".github/workflows/ci.yml"), ("head_sha", "other"),
                           ("head_branch", "main"), ("head_repository", {"full_name": "other/fork"})):
            with self.subTest(key=key), self.assertRaises(ValueError):
                prepare.verify_run(dict(run, **{key: value}), "v1.2.3", "abc")

    def test_library_provenance_and_lock_can_change_but_sources_cannot(self):
        a = archive({"src/lib.rs": b"same", "Cargo.lock": b"a", ".cargo_vcs_info.json": b"a"})
        b = archive({"src/lib.rs": b"same", "Cargo.lock": b"b", ".cargo_vcs_info.json": b"b"})
        self.assertEqual(cargo.archive_files(a, library=True), cargo.archive_files(b, library=True))
        self.assertNotEqual(cargo.archive_files(a, library=False), cargo.archive_files(b, library=False))
        changed = archive({"src/lib.rs": b"changed", "Cargo.lock": b"a"})
        self.assertNotEqual(cargo.archive_files(a, library=True), cargo.archive_files(changed, library=True))

    def test_checkout_line_endings_but_not_binary_assets_are_normalized(self):
        a = archive({"src/lib.rs": b"line\r\n", "assets/test.png": b"\r\n"})
        b = archive({"src/lib.rs": b"line\n", "assets/test.png": b"\n"})
        before, after = cargo.archive_files(a, library=True), cargo.archive_files(b, library=True)
        self.assertEqual(before["src/lib.rs"], after["src/lib.rs"])
        self.assertNotEqual(before["assets/test.png"], after["assets/test.png"])

    def test_cargo_plan_skips_existing_and_rejects_changed_or_yanked_versions(self):
        published = archive({"src/lib.rs": b"original"})
        with tempfile.TemporaryDirectory() as temporary:
            staging = Path(temporary) / "package/tmp-registry"
            staging.mkdir(parents=True)
            local = staging / "test-library-1.0.0.crate"
            local.write_bytes(published)
            metadata = {"packages": [{"name": "test-library", "version": "1.0.0", "id": "lib", "publish": None}],
                        "workspace_members": ["lib"], "target_directory": temporary}
            remote = {"yanked": False, "checksum": hashlib.sha256(published).hexdigest()}
            with patch.object(cargo.subprocess, "check_output", return_value=json.dumps(metadata)), \
                    patch.object(cargo, "published_version", return_value=remote) as version, \
                    patch.object(cargo, "request", side_effect=lambda *_: io.BytesIO(published)):
                self.assertEqual(cargo.plan(Path(temporary)), ([], ["test-library"]))
                local.write_bytes(archive({"src/lib.rs": b"changed"}))
                with self.assertRaisesRegex(ValueError, "Bump its version"):
                    cargo.plan(Path(temporary))
                version.return_value = dict(remote, yanked=True)
                with self.assertRaisesRegex(ValueError, "yanked"):
                    cargo.plan(Path(temporary))
                version.return_value = None
                self.assertEqual(cargo.plan(Path(temporary)), (["test-library"], []))

    def test_manifest_versions_checksum_and_url_are_updated(self):
        release = {"tag": "v9.8.7", "version": "9.8.7", "release_date": "2026-09-16",
                   "installer_sha256": "A" * 64,
                   "installer_url": f"https://github.com/{REPOSITORY}/releases/download/v9.8.7/FastMarkdownViewer-Setup-9.8.7.exe"}
        with tempfile.TemporaryDirectory() as temporary:
            files = winget.generate(release, ROOT / "packaging/winget" / winget.MANIFEST_ROOT, Path(temporary))
            self.assertEqual(len(files), 3)
            for path, text in files.items():
                self.assertIn("/9.8.7/", path)
                self.assertIn("PackageVersion: 9.8.7\n", text)
                self.assertNotIn("0.2.3", text)
            installer = next(text for path, text in files.items() if path.endswith(".installer.yaml"))
            self.assertIn(release["installer_url"], installer)
            self.assertIn("InstallerSha256: " + "A" * 64, installer)
            self.assertIn("ReleaseDate: 2026-09-16", installer)
            self.assertIn("DisplayName: FastMarkdownViewer version 9.8.7", installer)
            self.assertIn("RelativeFilePath: FastMarkdownViewer.exe", installer)
            with self.assertRaises(ValueError):
                winget.generate(dict(release, installer_url="https://example.com/evil.exe"),
                                ROOT / "packaging/winget" / winget.MANIFEST_ROOT, Path(temporary))

    @patch.object(winget, "github")
    def test_existing_pr_is_reused_without_writes(self, api):
        api.return_value = [{"state": "open", "merged_at": None, "html_url": "https://github.com/pr/1"}]
        self.assertEqual(winget.submit({"version": "1.2.3"}, {}, None), "https://github.com/pr/1")
        api.assert_called_once()
        self.assertNotIn("data", api.call_args.kwargs)

    @patch.object(winget, "github")
    def test_closed_pr_is_not_reopened(self, api):
        api.return_value = [{"state": "closed", "merged_at": None, "html_url": "https://github.com/pr/1"}]
        with self.assertRaises(ValueError):
            winget.existing_submission("1.2.3", None)

    @patch.object(winget, "github")
    def test_merged_manifest_is_reused(self, api):
        api.side_effect = [[], [{"name": "manifest.yaml"}]]
        self.assertIn("/1.2.3", winget.existing_submission("1.2.3", None))
        self.assertTrue(all("data" not in call.kwargs for call in api.call_args_list))

    @patch.object(winget, "existing_submission", return_value=None)
    @patch.object(winget, "github")
    def test_new_submission_does_not_import_unrelated_upstream_workflows(self, api, _existing):
        api.side_effect = [
            {"login": "Quetzalcohuatl"}, {"object": {"sha": "fork-base"}}, None,
            {"tree": {"sha": "fork-tree"}}, {"sha": "new-tree"}, {"sha": "new-commit"},
            {}, {"html_url": "https://github.com/pr/3"},
        ]
        release = {"version": "1.2.3", "release_url": "https://github.com/release", "release_run": "https://github.com/run"}
        winget.submit(release, {"manifest.yaml": "new content"}, "test-token")
        self.assertEqual(api.call_args_list[1].args[0], f"repos/{winget.FORK}/git/ref/heads/master")
        writes = [call.kwargs["data"] for call in api.call_args_list if "data" in call.kwargs]
        self.assertEqual(writes[0]["base_tree"], "fork-tree")
        self.assertEqual(writes[1]["parents"], ["fork-base"])
        self.assertEqual(writes[2], {"ref": "refs/heads/fastmarkdownviewer-1.2.3", "sha": "new-commit"})

    @patch.object(winget, "existing_submission", return_value=None)
    @patch.object(winget, "github")
    def test_partial_branch_retry_uses_tree_sha_and_never_force_pushes(self, api, _existing):
        api.side_effect = [
            {"login": "Quetzalcohuatl"}, {"object": {"sha": "base"}}, {"object": {"sha": "partial"}},
            None, {"tree": {"sha": "parent-tree"}}, {"sha": "new-tree"}, {"sha": "new-commit"},
            {}, {"html_url": "https://github.com/pr/2"},
        ]
        release = {"version": "1.2.3", "release_url": "https://github.com/release", "release_run": "https://github.com/run"}
        winget.submit(release, {"manifest.yaml": "new content"}, "test-token")
        writes = [call.kwargs for call in api.call_args_list if "data" in call.kwargs]
        self.assertEqual(writes[0]["data"]["base_tree"], "parent-tree")
        self.assertEqual(writes[1]["data"]["parents"], ["partial"])
        self.assertFalse(writes[2]["data"]["force"])
        self.assertEqual(writes[3]["data"]["head"], "Quetzalcohuatl:fastmarkdownviewer-1.2.3")


if __name__ == "__main__":
    unittest.main()
