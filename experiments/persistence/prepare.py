"""Create isolated profiles for before/after and 100-tab restoration measurements."""
import argparse
import json
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('--before', required=True, type=Path)
parser.add_argument('--after', required=True, type=Path)
parser.add_argument('--fixture', required=True, type=Path)
parser.add_argument('--output', required=True, type=Path)
args = parser.parse_args()
base = args.output.resolve()
base.mkdir(parents=True, exist_ok=True)
fixture = args.fixture.resolve()
tabs = []
for index in range(100):
    path = base / f'document-{index}.md'
    path.write_bytes(fixture.read_bytes())
    tabs.append({'path': str(path), 'scroll': 0})
apps = []
for name, executable in [('before', args.before), ('after', args.after), ('restore100', args.after)]:
    profile = base / name
    profile.mkdir(exist_ok=True)
    if name == 'restore100':
        folder = profile / 'FastMarkdownViewer'
        folder.mkdir(exist_ok=True)
        (folder / 'state.json').write_text(json.dumps({
            'version': 1, 'windows': [{'tabs': tabs, 'active': 0, 'outline': True}]
        }), encoding='utf-8')
    apps.append({
        'id': name,
        'command': [str(executable.resolve())] + ([] if name == 'restore100' else ['{fixture}']),
        'env': {'APPDATA': str(profile), 'LOCALAPPDATA': str(profile)},
        'preserve_profile': True,
    })
(base / 'config.json').write_text(json.dumps({
    'fixtures': str(fixture.parent), 'files': [fixture.name], 'apps': apps
}), encoding='utf-8')
