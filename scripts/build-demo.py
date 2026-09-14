#!/usr/bin/env python3
"""Compose a silent 20-second walkthrough from real renderer captures.

Requires Pillow and FFmpeg with libx264. See docs/demo/README.md.
"""
import argparse
import json
import subprocess
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--ffmpeg', default='ffmpeg')
parser.add_argument('--font-directory', type=Path, default=Path('C:/Windows/Fonts'))
args = parser.parse_args()
OUT = ROOT / 'docs/demo'
WIDTH, HEIGHT, FPS = 1600, 1000, 24
PAPER, INK, MUTED, BLUE = '#f4f1ea', '#161b22', '#626971', '#145caa'


def font(size, bold=False):
    return ImageFont.truetype(str(args.font_directory / ('segoeuib.ttf' if bold else 'segoeui.ttf')), size)


def label(draw, pos, text, size, fill=INK, bold=False, spacing=12):
    draw.multiline_text(pos, text, font=font(size, bold), fill=fill, spacing=spacing)


def shell():
    frame = Image.new('RGB', (WIDTH, HEIGHT), PAPER)
    draw = ImageDraw.Draw(frame)
    draw.rounded_rectangle((52, 42, 103, 93), radius=11, fill=BLUE)
    label(draw, (60, 51), 'FM', 22, 'white', True)
    label(draw, (120, 48), 'FastMarkdownViewer', 28, bold=True)
    draw.line((52, 123, 1548, 123), fill='#cbc7bd', width=2)
    return frame, draw


scenes = [
    {'start': 0, 'duration': 5, 'image': 'reading.png', 'title': 'Open a file.\nStart reading.',
     'body': 'Clean Markdown.\nNo editor to set up.', 'label': '01 / READ'},
    {'start': 5, 'duration': 5, 'image': 'find.png', 'title': 'Find the line\nyou need.',
     'body': 'Search the document.\nSee matches in context.', 'label': '02 / FIND'},
    {'start': 10, 'duration': 6, 'image': 'technical.png', 'title': 'Code. Math.\nDiagrams.',
     'body': 'Read technical details\nin one clear view.', 'label': '03 / UNDERSTAND'},
]
frames = []
for scene in scenes:
    frame, draw = shell()
    label(draw, (52, 242), scene['label'], 22, BLUE, True)
    label(draw, (52, 302), scene['title'], 57, bold=True, spacing=14)
    label(draw, (55, 489), scene['body'], 28, MUTED, spacing=12)
    shot = Image.open(OUT / scene['image']).convert('RGB')
    shot.thumbnail((1008, 784), Image.Resampling.LANCZOS)
    x, y = 540, 157
    draw.rounded_rectangle((x-2, y-2, x+shot.width+2, y+shot.height+2), radius=4, fill='#cbc7bd')
    frame.paste(shot, (x, y))
    label(draw, (52, 945), 'Edited walkthrough | Real v0.2.2 renderer | No audio needed', 19, MUTED)
    frames.append((frame, scene['duration']))

frame, draw = shell()
label(draw, (90, 232), 'Markdown, made easy to read.', 77, bold=True)
label(draw, (95, 370), 'Free. Open source. No account.', 43, MUTED)
label(draw, (95, 462), 'Windows  |  macOS  |  Ubuntu', 32, BLUE, True)
draw.rounded_rectangle((92, 581, 566, 674), radius=14, fill=BLUE)
label(draw, (127, 602), 'Try FastMarkdownViewer', 31, 'white', True)
label(draw, (96, 734), 'github.com/Quetzalcohuatl/fastmarkdownviewer', 27, MUTED)
label(draw, (52, 945), 'v0.2.2 | See download page for OS requirements and signing status', 19, MUTED)
frames.append((frame, 4))

# Static poster: no misleading imitation of a native app button.
poster = frames[0][0].copy()
draw = ImageDraw.Draw(poster)
draw.rounded_rectangle((52, 661, 464, 741), radius=12, fill=BLUE)
label(draw, (79, 677), 'Watch the 20-second demo', 27, 'white', True)
poster.save(OUT / 'poster.png', optimize=True)

command = [args.ffmpeg, '-y', '-v', 'error', '-f', 'rawvideo', '-pixel_format', 'rgb24',
           '-video_size', f'{WIDTH}x{HEIGHT}', '-framerate', str(FPS), '-i', '-', '-an',
           '-c:v', 'libx264', '-preset', 'medium', '-crf', '19', '-pix_fmt', 'yuv420p',
           '-movflags', '+faststart', str(OUT / 'fastmarkdownviewer-20s.mp4')]
process = subprocess.Popen(command, stdin=subprocess.PIPE)
count = 0
try:
    for still, seconds in frames:
        for _ in range(seconds * FPS):
            frame = still.copy()
            draw = ImageDraw.Draw(frame)
            draw.rectangle((0, 994, WIDTH, 999), fill='#ddd9d1')
            draw.rectangle((0, 994, round(WIDTH * (count + 1) / (20 * FPS)), 999), fill=BLUE)
            process.stdin.write(frame.tobytes())
            count += 1
finally:
    process.stdin.close()
assert process.wait() == 0, 'FFmpeg encoding failed'
assert count == 20 * FPS
(OUT / 'timeline.json').write_text(json.dumps({'duration_seconds': 20, 'fps': FPS,
    'size': [WIDTH, HEIGHT], 'scenes': scenes + [{'start': 16, 'duration': 4, 'title': 'Try FastMarkdownViewer'}]},
    indent=2) + '\n', encoding='utf-8')
print(f'Created {count} frames / 20 seconds: {OUT / "fastmarkdownviewer-20s.mp4"}')
