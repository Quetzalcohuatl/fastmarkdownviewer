"""Snapshot public upstream release metadata and READMEs; no credentials stored."""
import argparse, base64, concurrent.futures, json, pathlib, subprocess, urllib.request
ROOT = pathlib.Path(__file__).resolve().parent
REPOS = ['dartdavros/MarkMello', 'marktext/marktext', 'aydiler/md-viewer', 'GRVYDEV/marky', 'leaanthony/mdview', 'nathannncurtis/mdview-zig', 'vorojar/md-preview', 'charmbracelet/glow', 'swsnr/mdcat', 'Textualize/frogmouth', 'simov/markdown-viewer', 'joeyespo/grip', 'ekino/MarkdownViewer', 'newuni/md-viewer', 'rajatarya/mdviewer', 'trsdn/mdviewer']
def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--use-git-credentials', action='store_true', help='Use configured GitHub credentials for a higher public API rate limit')
    args=parser.parse_args()
    headers={'User-Agent':'FMV-competitor-research'}
    if args.use_git_credentials:
        cred = subprocess.run(['git','credential','fill'], input='protocol=https\nhost=github.com\n\n',text=True,capture_output=True,check=True)
        token = dict(x.split('=',1) for x in cred.stdout.splitlines() if '=' in x)['password']
        headers['Authorization']='Bearer '+token
    def get(path):
        req=urllib.request.Request('https://api.github.com/'+path,headers=headers)
        with urllib.request.urlopen(req,timeout=60) as r: return json.load(r)
    def collect(repo):
        info=get('repos/'+repo); releases=get('repos/'+repo+'/releases?per_page=5'); readme=get('repos/'+repo+'/readme')
        out=ROOT/'upstream'/repo.replace('/','__'); out.mkdir(parents=True,exist_ok=True)
        (out/'README.md').write_bytes(base64.b64decode(readme['content']))
        record={'repo':repo,'url':info['html_url'],'description':info['description'],'license':info.get('license'),'archived':info['archived'],'default_branch':info['default_branch'],'readme_sha':readme['sha'],'releases':releases}
        (out/'metadata.json').write_text(json.dumps(record,indent=2),encoding='utf-8')
        print(repo,[(r['tag_name'],[a['name'] for a in r['assets']]) for r in releases[:1]],flush=True)
    with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool:
        for result in pool.map(collect,REPOS): pass
if __name__=='__main__': main()
