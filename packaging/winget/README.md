# WinGet manifest

The 0.2.3 manifest targets the existing official, per-user Windows x64 Inno Setup installer. Submission: https://github.com/microsoft/winget-pkgs/pull/435199

Validate from the repository root:

```powershell
winget validate --manifest packaging/winget/manifests/q/Quetzalcohuatl/FastMarkdownViewer/0.2.3
```

For an installation test, use a disposable Windows environment or verify that no existing FMV installation/registrations will be replaced. Enable `LocalManifestFiles` in an Administrator PowerShell if required by local policy, then run `winget install --manifest` with that directory. Restore the original setting afterward; disabling also requires an Administrator PowerShell:

```powershell
winget settings --disable LocalManifestFiles
```

For future stable releases, the `Package distribution` workflow generates a new version's manifests from these templates, verifies the released installer checksum, validates the manifests, and opens or reuses a PR against `microsoft/winget-pkgs`. See [setup and retry instructions](../../docs/REGISTRY_DISTRIBUTION.md#automated-releases). The generated files are available as workflow artifacts; no bot commit to this repository is required.

If installer metadata changes, update these templates before releasing. Check installed Apps & Features metadata (`scripts/test-windows-installer.ps1` prints it), the actual installation path, and file associations. Keep public availability wording synchronized with Microsoft's merge/indexing status.
