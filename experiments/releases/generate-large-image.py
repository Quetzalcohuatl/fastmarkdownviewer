"""Generate the deterministic 48-megapixel/48 MB PNG used by the release check.

Development dependency: Pillow. The app does not depend on Python or Pillow.
"""
from pathlib import Path
import random
import sys
from PIL import Image

output = Path(sys.argv[1])
output.mkdir(parents=True, exist_ok=True)
image = Image.frombytes('L', (8000, 6000), random.Random(20261001).randbytes(48_000_000))
image.save(output / 'large-image.png', compress_level=0)
(output / 'large-image.md').write_text(
    '# Large image\n\nThis text can appear while the 48-megapixel image loads.\n\n'
    '![Large local image](large-image.png)\n', encoding='utf-8')
