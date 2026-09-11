"""Download pinned official release assets into ignored target/competitors."""
import argparse, concurrent.futures, hashlib, json, pathlib, urllib.request
ROOT=pathlib.Path(__file__).resolve().parent
DEST=ROOT.parents[1]/'target'/'competitors'
PICKS={
 'dartdavros/MarkMello':['MarkMello-setup-win-x64.exe','MarkMello-linux-x86_64.AppImage'],
 'aydiler/md-viewer':['md-viewer-0.2.0-windows-x86_64.zip','md-viewer-0.2.0-linux-x86_64.tar.gz'],
 'GRVYDEV/marky':['marky_0.1.3_amd64.deb'],
 'marktext/marktext':['marktext-win-x64-0.20.0-rc.2.zip','marktext-linux-0.20.0-rc.2.tar.gz'],
 'nathannncurtis/mdview-zig':['mdview-linux-x86_64.deb'],
 'vorojar/md-preview':['MD-Preview-windows-x64.exe','MD-Preview-linux-x64.tar.gz'],
 'charmbracelet/glow':['glow_3.0.0_Windows_x86_64.zip','glow_3.0.0_Linux_x86_64.tar.gz'],
 'swsnr/mdcat':['mdcat-2.7.1-x86_64-pc-windows-msvc.zip','mdcat-2.7.1-x86_64-unknown-linux-gnu.tar.gz']}
def main():
 parser=argparse.ArgumentParser()
 parser.add_argument('--include-pilots',action='store_true',help='Also fetch excluded MarkText RC and ekino pilot assets')
 args=parser.parse_args()
 manifest=ROOT/'downloads.json'
 if manifest.exists():
  for record in json.loads(manifest.read_text()):
   if not args.include_pilots and ('-rc.' in record['version'] or record['repo']=='ekino/MarkdownViewer'):continue
   dest=DEST/record['repo'].replace('/','__')/record['asset'];dest.parent.mkdir(parents=True,exist_ok=True)
   if not dest.exists():
    headers={'User-Agent':'Mozilla/5.0','Referer':'https://typora.io/'} if record['repo']=='typora' else {'User-Agent':'FMV-competitor-benchmark'}
    request=urllib.request.Request(record['url'],headers=headers)
    with urllib.request.urlopen(request,timeout=120) as response, dest.open('wb') as output:
     while chunk:=response.read(1024*1024):output.write(chunk)
   assert dest.stat().st_size==record['bytes'], 'Wrong size: '+str(dest)
   assert hashlib.file_digest(dest.open('rb'),'sha256').hexdigest()==record['sha256'], 'Wrong digest: '+str(dest)
   print('Verified',record['repo'],record['asset'],flush=True)
  return
 jobs=[]
 for repo,names in PICKS.items():
  meta=json.loads((ROOT/'upstream'/repo.replace('/','__')/'metadata.json').read_text())
  for name in names:
   release,asset=next((r,a) for r in meta['releases'] for a in r['assets'] if a['name']==name)
   jobs.append((repo,release['tag_name'],asset))
 def download(job):
  repo,version,asset=job; dest=DEST/repo.replace('/','__')/asset['name']; dest.parent.mkdir(parents=True,exist_ok=True)
  if not dest.exists() or dest.stat().st_size!=asset['size']:
   with urllib.request.urlopen(asset['browser_download_url'],timeout=120) as r, dest.open('wb') as f:
    while chunk:=r.read(1024*1024): f.write(chunk)
  sha=hashlib.file_digest(dest.open('rb'),'sha256').hexdigest()
  if (asset.get('digest') or '').startswith('sha256:'): assert asset['digest']=='sha256:'+sha
  print('Downloaded',repo,asset['name'],flush=True)
  return {'repo':repo,'version':version,'asset':asset['name'],'url':asset['browser_download_url'],'bytes':dest.stat().st_size,'sha256':sha,'github_digest':asset.get('digest')}
 with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool: records=list(pool.map(download,jobs))
 (ROOT/'downloads.json').write_text(json.dumps(records,indent=2)+'\n')
if __name__=='__main__': main()
