#!/usr/bin/env python3
"""Exercise LaunchServices open events and persistence in the extracted shipping app.

Run only on an isolated macOS test account/CI worker: its FMV profile must not exist.
Native keyboard acceptance additionally requires accessibility permission for osascript.
"""
import json
from pathlib import Path
import plistlib
import subprocess
import sys
import tempfile
import time

archive = Path(sys.argv[1]).resolve()
profile = Path.home() / 'Library/Application Support/FastMarkdownViewer/state.json'
if profile.exists():
    raise SystemExit('Use an isolated test account: an existing viewer profile will not be replaced.')

def wait_for(predicate, description):
    deadline = time.monotonic() + 30
    while time.monotonic() < deadline:
        if predicate():
            return
        time.sleep(0.2)
    raise RuntimeError(f'Timed out: {description}')

with tempfile.TemporaryDirectory(prefix='fmv-package-') as work:
    work = Path(work).resolve()
    subprocess.run(['ditto', '-x', '-k', str(archive), str(work)], check=True)
    bundle = next(work.glob('*/FastMarkdownViewer.app'))
    info = plistlib.loads((bundle / 'Contents/Info.plist').read_bytes())
    assert info['LSMinimumSystemVersion'] == '15.0'
    assert info['CFBundleDocumentTypes'][0]['CFBundleTypeRole'] == 'Viewer'
    binary = bundle / 'Contents/MacOS/FastMarkdownViewer'
    subprocess.run(['codesign', '--verify', '--strict', str(bundle)], check=True)
    subprocess.run(['/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister', '-f', str(bundle)], check=True)
    first = work / 'First 日本語 with spaces.md'
    second = work / 'Second.markdown'
    third = work / 'Dialog document.md'
    first.write_text('# First\n\nHello from Finder.\n' * 80)
    second.write_text('# Second\n\nA second document.')
    third.write_text('# Dialog document\n\nOpened using Cmd+O.')

    def keys(script):
        subprocess.run(['osascript', '-e', 'tell application "System Events"\n'
                        'tell process "FastMarkdownViewer"\nset frontmost to true\n' + script +
                        '\nend tell\nend tell'], check=True, timeout=20)
        time.sleep(0.5)

    def running():
        return subprocess.run(['pgrep', '-f', str(binary)], stdout=subprocess.DEVNULL).returncode == 0

    def quit_and_read():
        subprocess.run(['osascript', '-e', f'tell application "{bundle}" to quit'], check=True, timeout=30)
        wait_for(lambda: not running(), 'normal application exit')
        wait_for(profile.exists, 'saved session')
        return json.loads(profile.read_text())

    try:
        # LaunchServices sends an open-documents event, not command-line arguments.
        subprocess.run(['open', '-a', str(bundle), str(first)], check=True)
        wait_for(running, 'Finder launch')
        time.sleep(3)
        subprocess.run(['open', '-a', str(bundle), str(second), str(first)], check=True)
        time.sleep(3)
        state = quit_and_read()
        tabs = state['windows'][0]['tabs']
        assert [tab['path'] for tab in tabs] == [str(first), str(second)], state
        assert state['windows'][0]['active'] == 0, state

        # Seed known preferences and verify the actual packaged app restores them.
        state.update(theme='SolarizedDark', zoom=1.25, automatic_images=False, word_wrap=False)
        state['windows'][0]['outline'] = False
        profile.write_text(json.dumps(state))
        subprocess.run(['open', '-a', str(bundle)], check=True)
        wait_for(running, 'bare launch')
        time.sleep(3)
        keys('keystroke "f" using command down\nkeystroke "Finder"\nkeystroke "a" using command down\nkeystroke "c" using command down')
        assert subprocess.check_output(['pbpaste'], text=True) == 'Finder'
        keys('key code 53')  # Escape returns focus to the document.
        keys('keystroke "o" using command down')
        time.sleep(2)
        keys('keystroke "g" using {command down, shift down}')
        subprocess.run(['pbcopy'], input=str(third), text=True, check=True)
        keys('keystroke "v" using command down\nkey code 36')
        time.sleep(1)
        keys('key code 36')
        time.sleep(2)
        restored = quit_and_read()
        for key in ['theme', 'zoom', 'automatic_images', 'word_wrap']:
            assert restored[key] == state[key], (key, restored)
        assert [tab['path'] for tab in restored['windows'][0]['tabs']] == [str(first), str(second), str(third)]
        assert not restored['windows'][0]['outline']
        print('PASS: extracted app signature, Finder launch, running-app opens, Unicode paths, tab deduplication, normal quit, session and preference restoration, native Find/clipboard and Cmd+O file dialog')
    finally:
        if running():
            subprocess.run(['osascript', '-e', f'tell application "{bundle}" to quit'], timeout=30)
            wait_for(lambda: not running(), 'test process cleanup')
        profile.unlink(missing_ok=True)
