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

For future releases, add a new version directory with the actual published installer URL and SHA-256. Check installed Apps & Features metadata (`scripts/test-windows-installer.ps1` prints it), validate, test install/uninstall, and submit only that version's manifest files to microsoft/winget-pkgs. Keep the public availability wording in README and the registry guide synchronized with Microsoft's merge/indexing status.
