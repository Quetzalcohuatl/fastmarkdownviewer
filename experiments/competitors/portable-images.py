"""Make the generated image fixture portable before comparing operating systems."""
import argparse
import pathlib
import re

parser=argparse.ArgumentParser()
parser.add_argument('fixtures',type=pathlib.Path)
args=parser.parse_args()
path=args.fixtures/'images.md'
source=path.read_text(encoding='utf-8-sig')
portable=re.sub(r'\]\(file:///[^)\r\n]*/(image-\d+\.svg)\)',r'](\1)',source)
references=re.findall(r'\]\((image-\d+\.svg)\)',portable)
assert len(references)==24 and len(set(references))==24, 'Expected 24 relative SVG references'
assert all((args.fixtures/name).is_file() for name in references), 'Missing SVG fixture'
assert 'file:///' not in portable, 'Remaining machine-specific file URI'
path.write_text(portable,encoding='utf-8',newline='\n')
print('Prepared 24 relative SVG references:',path)
