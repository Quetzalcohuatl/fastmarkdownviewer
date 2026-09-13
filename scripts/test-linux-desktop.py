#!/usr/bin/env python3
"""Native X11 acceptance of an installed binary in an isolated profile.

Requires a running desktop, xdotool, xclip and a working XDG file-chooser portal.
Screenshots and saved state remain in the supplied evidence directory.
"""
import json
import os
from pathlib import Path
import subprocess
import sys
import time

binary = str(Path(sys.argv[1]).resolve())
work = Path(sys.argv[2]).resolve()
work.mkdir(parents=True, exist_ok=False)
profile = work / 'profile'
profile.mkdir()
state_path = profile / 'FastMarkdownViewer/state.json'
first = work / 'First 日本語.md'
second = work / 'Second.markdown'
first.write_text('# Native acceptance\n\n' + 'Read and find needle in this paragraph.\n\n' * 200)
second.write_text('# Second\n\nOpened through the native portal.')
env = dict(os.environ, XDG_CONFIG_HOME=str(profile))

def command(*args):
    return subprocess.check_output(args, text=True, timeout=20).strip()

def wait_for(predicate, label):
    deadline = time.monotonic() + 20
    while time.monotonic() < deadline:
        result = predicate()
        if result:
            return result
        time.sleep(0.2)
    raise RuntimeError('Timed out: ' + label)

def windows():
    result = subprocess.run(['xdotool', 'search', '--onlyvisible', '--pid', str(process.pid)], capture_output=True, text=True)
    return [w for w in result.stdout.splitlines() if 'FastMarkdownViewer' in command('xdotool', 'getwindowname', w)]

def key(*keys):
    command('xdotool', 'key', '--clearmodifiers', *keys)
    time.sleep(0.3)

def capture(name):
    command('scrot', str(work / (name + '.png')))

def close(window):
    command('xdotool', 'windowactivate', '--sync', window)
    key('alt+F4')

log = (work / 'viewer.log').open('w')
process = subprocess.Popen([binary, str(first)], env=env, stdout=log, stderr=log)
try:
    root = wait_for(windows, 'native window')[0]
    command('xdotool', 'windowactivate', '--sync', root)
    key('ctrl+f')
    command('xdotool', 'type', '--clearmodifiers', 'needle')
    key('ctrl+a', 'ctrl+c')
    assert command('xclip', '-selection', 'clipboard', '-o') == 'needle'
    capture('find-and-clipboard')
    key('Escape', 'Next', 'Next')
    close(root)
    process.wait(timeout=20)
    assert process.returncode == 0
    state = json.loads(state_path.read_text())
    assert state['windows'][0]['tabs'][0]['scroll'] > 0, state
    state.update(theme='SolarizedDark', zoom=1.25, automatic_images=False, word_wrap=False)
    state_path.write_text(json.dumps(state))
    process = subprocess.Popen([binary], env=env, stdout=log, stderr=log)
    root = wait_for(windows, 'restored window')[0]
    command('xdotool', 'windowactivate', '--sync', root)
    time.sleep(2)
    capture('restored')
    key('ctrl+o')
    time.sleep(2)
    dialog = command('xdotool', 'getactivewindow')
    command('xdotool', 'windowsize', dialog, '900', '650')
    capture('portal-dialog')
    key('ctrl+l')
    command('xdotool', 'type', '--clearmodifiers', str(second))
    key('Return')
    time.sleep(1)
    # GTK first resolves the location entry, then activates the selected file.
    if command('xdotool', 'getactivewindow') == dialog:
        key('Return')
    capture('portal-selection')
    wait_for(lambda: 'Second.markdown' in command('xdotool', 'getwindowname', root), 'portal selected file')
    capture('second-tab')
    close(root)
    process.wait(timeout=20)
    assert process.returncode == 0
    restored = json.loads(state_path.read_text())
    assert [tab['path'] for tab in restored['windows'][0]['tabs']] == [str(first), str(second)], restored
    for field in ['theme', 'zoom', 'automatic_images', 'word_wrap']:
        assert restored[field] == state[field], (field, restored)
    print('PASS: native window, Find, clipboard, scrolling, clean exit, saved preferences/session, portal file dialog, multiple tabs')
finally:
    if process.poll() is None:
        for window in windows():
            close(window)
        try:
            process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            process.terminate()
            process.wait(timeout=10)
    log.close()
