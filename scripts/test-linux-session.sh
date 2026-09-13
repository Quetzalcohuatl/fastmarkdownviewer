#!/usr/bin/env bash
# Run inside a disposable X11 display and D-Bus session (see desktop workflow).
set -euo pipefail
export XDG_CURRENT_DESKTOP=XFCE LIBGL_ALWAYS_SOFTWARE=1
export XDG_RUNTIME_DIR="$(mktemp -d)"
dbus-update-activation-environment DISPLAY XAUTHORITY XDG_CURRENT_DESKTOP XDG_RUNTIME_DIR
openbox > target/evidence/openbox.log 2>&1 &
manager=$!
trap 'kill "$manager"' EXIT
sleep 1
python3 scripts/test-linux-desktop.py /usr/bin/FastMarkdownViewer target/evidence/installed-desktop
