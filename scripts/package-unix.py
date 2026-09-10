#!/usr/bin/env python3
"""Package an already-built native executable; never rebuild during packaging."""
import argparse
import hashlib
from pathlib import Path
import plistlib
import shutil
import subprocess
import tarfile
import tomllib

parser = argparse.ArgumentParser(__doc__)
parser.add_argument('--platform', choices=['linux-x86_64', 'macos-aarch64', 'macos-x86_64'], required=True)
parser.add_argument('--binary', type=Path, required=True)
parser.add_argument('--output', type=Path, default=Path('dist'))
args = parser.parse_args()
root = Path(__file__).resolve().parent.parent
version = tomllib.loads((root / 'Cargo.toml').read_text())['package']['version']
name = f'FastMarkdownViewer-{version}-experimental-{args.platform}'
stage = args.output / name
stage.mkdir(parents=True, exist_ok=False)
if args.platform.startswith('macos'):
    bundle = stage / 'FastMarkdownViewer.app'
    contents = bundle / 'Contents'
    executable = contents / 'MacOS' / 'FastMarkdownViewer'
    executable.parent.mkdir(parents=True)
    info = {
        'CFBundleExecutable': 'FastMarkdownViewer',
        'CFBundleIdentifier': 'io.github.quetzalcohuatl.fastmarkdownviewer',
        'CFBundleName': 'FastMarkdownViewer',
        'CFBundleDisplayName': 'FastMarkdownViewer',
        'CFBundlePackageType': 'APPL',
        'CFBundleShortVersionString': version,
        'CFBundleVersion': version,
        'NSHighResolutionCapable': True,
        'LSMinimumSystemVersion': '12.0',
    }
    (contents / 'Info.plist').write_bytes(plistlib.dumps(info))
else:
    executable = stage / 'FastMarkdownViewer'
    (stage / 'FastMarkdownViewer.desktop').write_text(
        '[Desktop Entry]\nType=Application\nName=FastMarkdownViewer\n'
        'Comment=Read-only Markdown viewer\nExec=FastMarkdownViewer %f\n'
        'Terminal=false\nCategories=Office;Viewer;\nMimeType=text/markdown;\n', encoding='utf-8')
shutil.copy2(args.binary, executable)
executable.chmod(0o755)
for filename in ['LICENSE-MIT', 'LICENSE-APACHE', 'THIRD_PARTY_NOTICES.md', 'PRIVACY.md']:
    shutil.copy2(root / filename, stage / filename)
shutil.copy2(root / 'docs/CROSS_PLATFORM.md', stage / 'README.md')
if args.platform.startswith('macos'):
    subprocess.run(['codesign', '--force', '--sign', '-', str(bundle)], check=True)
    subprocess.run(['codesign', '--verify', '--strict', str(bundle)], check=True)
    archive = args.output / (name + '.zip')
    subprocess.run(['ditto', '-c', '-k', '--sequesterRsrc', '--keepParent', str(stage), str(archive)], check=True)
else:
    archive = args.output / (name + '.tar.gz')
    with tarfile.open(archive, 'w:gz') as tar:
        tar.add(stage, arcname=name)
archives = [archive]
if args.platform == 'linux-x86_64':
    deb_root = args.output / (name + '-deb')
    (deb_root / 'DEBIAN').mkdir(parents=True, exist_ok=False)
    (deb_root / 'DEBIAN/control').write_text(
        f'Package: fast-markdown-viewer\nVersion: {version}~experimental\n'
        'Architecture: amd64\nMaintainer: FastMarkdownViewer contributors\n'
        'Section: text\nPriority: optional\n'
        'Depends: libc6 (>= 2.39), libgcc-s1, libx11-6, libxi6, libxcursor1, '
        'libxrandr2, libxcb1, libxkbcommon0, libxkbcommon-x11-0, libegl1, libgl1, '
        'libwayland-client0, xdg-desktop-portal\n'
        'Recommends: xdg-desktop-portal-gtk | xdg-desktop-portal-kde | xdg-desktop-portal-gnome\n'
        'Description: Experimental native read-only Markdown viewer\n')
    for source, relative in [
        (executable, 'usr/bin/FastMarkdownViewer'),
        (stage / 'FastMarkdownViewer.desktop', 'usr/share/applications/FastMarkdownViewer.desktop'),
        (stage / 'README.md', 'usr/share/doc/fast-markdown-viewer/README.md'),
        (stage / 'THIRD_PARTY_NOTICES.md', 'usr/share/doc/fast-markdown-viewer/THIRD_PARTY_NOTICES.md'),
        (stage / 'LICENSE-MIT', 'usr/share/doc/fast-markdown-viewer/LICENSE-MIT'),
        (stage / 'LICENSE-APACHE', 'usr/share/doc/fast-markdown-viewer/LICENSE-APACHE'),
    ]:
        destination = deb_root / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, destination)
    deb = args.output / (name + '.deb')
    subprocess.run(['dpkg-deb', '--root-owner-group', '--build', str(deb_root), str(deb)], check=True)
    archives.append(deb)
for archive in archives:
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    (args.output / (archive.name + '.sha256')).write_text(f'{digest}  {archive.name}\n')
    print(archive)
