# WinGet, Cargo, and internal distribution

## Availability

- Windows 0.2.3: [WinGet submission #435199](https://github.com/microsoft/winget-pkgs/pull/435199) is awaiting Microsoft's review and indexing. The manifest is in `packaging/winget/manifests/q/Quetzalcohuatl/FastMarkdownViewer/0.2.3`.
- Cargo 0.2.4: [published on crates.io](https://crates.io/crates/fast-markdown-viewer/0.2.4), together with all four supporting packages. The registry installation commands below are available now. The latest packaged desktop release remains 0.2.3.

After WinGet accepts and indexes the package:

```powershell
winget install --id Quetzalcohuatl.FastMarkdownViewer --exact --source winget
winget upgrade --id Quetzalcohuatl.FastMarkdownViewer --exact --source winget
winget uninstall --id Quetzalcohuatl.FastMarkdownViewer --exact
```

The Windows installer is x64 and **per-user**, without administrator privileges. It adds Start menu and Open With entries without changing default file associations. It is currently unsigned; WinGet publication does not add Authenticode signing. Machine-wide installation/MSI deployment is not provided.

## Cargo installation

Cargo compiles the viewer from source. Install Rust **1.95.0 or newer**, the platform's native compiler, and the [platform build dependencies](CROSS_PLATFORM.md#verification-and-development). On macOS, use the Xcode command line tools and set `MACOSX_DEPLOYMENT_TARGET=15.0` before building. On Windows, use the MSVC Rust toolchain and Visual Studio C++ Build Tools with the Windows SDK.

Install from crates.io:

```sh
cargo install fast-markdown-viewer --version 0.2.4 --locked
FastMarkdownViewer document.md
```

The installed command is case-sensitive on Linux: `FastMarkdownViewer`. Cargo puts it in `$CARGO_HOME/bin` (normally `~/.cargo/bin`); ensure that directory is on PATH. Cargo installation creates the executable. Use the release installer, `.app` bundle, or `.deb` for desktop shortcuts, file associations, Finder integration, and native uninstall support.

Cargo registry installs do not read this repository's `.cargo/config.toml`. To match the Windows release's static C runtime setting, set `RUSTFLAGS=-C target-feature=+crt-static` for the build, for example in PowerShell:

```powershell
$env:RUSTFLAGS = '-C target-feature=+crt-static'
cargo install fast-markdown-viewer --version 0.2.4 --locked
```

To build a source checkout, run `cargo install --path . --locked`. The included versioned path dependencies retain the patched renderer. A Git install also works from a reviewed commit: `cargo install --git https://github.com/Quetzalcohuatl/fastmarkdownviewer --rev COMMIT_SHA --locked`.

## General internal Cargo registries

No JFrog-specific service or configuration is required. A company can mirror crates.io into its chosen Cargo-compatible repository and approve/cache the viewer, its four supporting packages, and all locked dependencies. Configure the corporate sparse index in the user's or build agent's Cargo configuration:

```toml
# ~/.cargo/config.toml (Windows: %USERPROFILE%\.cargo\config.toml)
[registries.company]
index = "sparse+https://registry.example.com/cargo/index/"

[source.crates-io]
replace-with = "company"

[source.company]
registry = "sparse+https://registry.example.com/cargo/index/"
```

Replace the example URL with the endpoint supplied by IT; keep the trailing slash. This source replacement routes crates.io dependencies through the company mirror as well as the application. A mirror must serve unchanged crate archives/checksums and their index entries; configuring only `--registry company` is not a guarantee that dependencies avoid crates.io.

```sh
cargo install fast-markdown-viewer --version 0.2.4 --registry company --locked
```

For authenticated registries, follow the registry provider's instructions for a Cargo credential provider and `cargo login --registry company`. Keep credentials out of repository files, command arguments, and tickets. A fully disconnected environment must prepopulate the entire dependency graph and the Rust/native toolchains; a Cargo registry alone does not supply system libraries.

IT can alternatively publish these source packages to a private Cargo registry using `cargo publish --workspace --registry company --locked` from a clean reviewed checkout. This requires that registry to support Cargo publishing and dependency resolution against crates.io or its configured mirror. Validate with `--dry-run` against the actual endpoint first; authentication and mixed private/public registry policies vary by provider. Use a new version for company-modified sources, and retain license notices.

## Mirroring ready-to-run releases

For employee desktop deployment, an internal binary artifact repository can store the [release assets](https://github.com/Quetzalcohuatl/fastmarkdownviewer/releases), `SHA256SUMS`, and the release's GitHub artifact attestations. Preserve the original filenames and bytes; verify checksums and attestations before promotion. This route does not require Rust on employee PCs.

Windows uses a per-user Inno installer, macOS uses an application bundle, and Ubuntu uses a Debian package. Windows Authenticode signing and macOS Developer ID signing/notarization remain separate from registry availability. An internal WinGet REST source is another option for Windows; configure it according to IT policy and update installer URLs/checksums if hosting an internal copy.

## Maintainer packaging and verification

### Automated releases

The [Package distribution workflow](../.github/workflows/distribution.yml) runs after the existing **Release** workflow succeeds. Stable `vMAJOR.MINOR.PATCH` releases publish missing Cargo workspace versions and submit a WinGet pull request using the released Windows installer. Prereleases are excluded. Microsoft still validates and reviews each WinGet submission before it becomes installable.

The workflow verifies the annotated tag, exact source commit, successful release run, and installer checksum. Cargo publication uses crates.io trusted publishing with a temporary credential. Existing crate versions are compared against the packaged source: changed contents require a version bump. Unchanged supporting crates keep their versions. Checkout provenance and library-only lockfiles are excluded from that comparison; the application's lockfile is compared. Text line endings are normalized to match Git's checkout policy. WinGet retries reuse the existing version's PR and never force-push branches.

One-time account setup:

1. For each of the five crates listed below, add a **GitHub trusted publisher** in crates.io's crate settings: owner `Quetzalcohuatl`, repository `fastmarkdownviewer`, workflow filename `distribution.yml`, environment `distribution`.
2. The GitHub environment `distribution` must allow deployment from `main` only. The distribution workflow runs its tooling from the default branch and checks out the tested release commit separately for Cargo.
3. Save `WINGET_GITHUB_TOKEN` in the repository's Actions secrets. Use the maintainer's GitHub classic token with `public_repo`, as required to write the existing `Quetzalcohuatl/winget-pkgs` fork and open a PR against Microsoft's public repository. Set an expiry and renew the secret before it expires. Do not reuse a local Cargo token or put credentials in the repository.

For the next release, bump the application version (and any changed supporting crate versions), update the lockfile and release notes, and push the annotated release tag as usual. If the installer identity, architecture, installation path, or associations change, update the checked-in WinGet templates too. Registry errors appear as a failed **Package distribution** run; they do not undo the already-published GitHub release.

To retry a channel or inspect a release without writing to registries, open **Actions → Package distribution → Run workflow** on `main`, enter an existing stable release tag, select `cargo`, `winget`, or `both`, and leave **dry_run** enabled for verification only. Disable it to publish/submit. A successful official Release run for that exact tag and commit is required even for manual runs. The generated WinGet manifests are retained as workflow artifacts, and the job summaries list publications and the PR URL.

The initial Cargo-only `cargo-v0.2.4` tag predates this unified process; it is not a desktop release tag and does not trigger it. The next unified release must use a new application version because crates.io versions cannot be overwritten.

After setting up account credentials, run the workflow with channel `credentials` and an existing desktop release such as `v0.2.3`. This checks the crates.io OIDC exchange and WinGet token identity/scope without publishing or opening a PR. A successful OIDC exchange proves a matching trusted publisher exists; configure all five crates so future supporting-library updates are authorized too.

This follows the official [crates.io trusted publishing](https://crates.io/docs/trusted-publishing) and [WinGet CI/CD distribution](https://github.com/microsoft/winget-create#using-windows-package-manager-manifest-creator-in-a-cicd-pipeline) patterns. Package publication makes an update available; it does not automatically replace an installed application on users' computers.

### Workspace packages

The workspace contains these independently versioned packages, in dependency order:

| Package | Version | Purpose |
| --- | --- | --- |
| `fmv-egui-commonmark-backend` | `0.25.0-fmv.1` | Patched layout, selection, and highlighting backend |
| `fmv-egui-commonmark` | `0.25.0-fmv.1` | Runtime Markdown viewer using the patched backend |
| `fmv-macos-events` | `0.1.0` | Native Mac event adapter |
| `fmv-rusty-mermaid-diagrams` | `0.2.0-fmv.1` | Patched, bounded diagram renderer |
| `fast-markdown-viewer` | `0.2.4` | Desktop executable |

The fork names distinguish these packages from upstream releases; original licenses and patch records are included. Cargo's published manifests use versioned registry dependencies, with no reliance on `[patch.crates-io]`. The upstream compile-time Markdown macros are outside the runtime viewer fork's scope.

The published Cargo 0.2.4 workspace is recorded by [source tag `cargo-v0.2.4`](https://github.com/Quetzalcohuatl/fastmarkdownviewer/tree/cargo-v0.2.4) (commit `d60be0bd516439f1774fce99240cdadce1e4ac3b`). This tag identifies the Cargo sources separately from the packaged desktop release tags.

```sh
cargo package --workspace --locked
python scripts/test-cargo-registry.py
cargo publish --workspace --locked --dry-run
# After authenticated dry-run succeeds and the source revision is reviewed:
cargo publish --workspace --locked
```

Use Python 3.11 or newer (`python3` on systems where appropriate). The registry test starts a temporary loopback sparse mirror, serves the newly packaged FMV archives, and proxies/caches public dependencies. It installs from outside the checkout, checks the installed version, and renders a Mermaid diagram with the installed binary. It changes no global Cargo settings. Native CI verifies packaging and this installation path on Windows, Linux, and macOS. `--release` additionally tests an optimized Cargo install. A successful Windows package check does not verify macOS event delivery; the separate native desktop checks cover it.

Cargo package/publish workspace support is why the maintainer commands require Cargo 1.95 or later. See Cargo's [packaging](https://doc.rust-lang.org/cargo/commands/cargo-package.html), [registry](https://doc.rust-lang.org/cargo/reference/registries.html), and [source replacement](https://doc.rust-lang.org/cargo/reference/source-replacement.html) documentation.
