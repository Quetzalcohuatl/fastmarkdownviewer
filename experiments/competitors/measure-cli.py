"""Separate CLI process-to-exit measurement; NOT terminal paint or GUI latency."""
import argparse,csv,hashlib,json,os,pathlib,random,re,subprocess,time
p=argparse.ArgumentParser();p.add_argument('config');p.add_argument('--output',required=True);p.add_argument('--runs',type=int,default=10);p.add_argument('--fixture');a=p.parse_args()
c=json.loads(pathlib.Path(a.config).read_text()); root=pathlib.Path(c['fixtures'])
jobs=[(app,f,n) for app in c['cli_apps'] for f in c['files'] if not a.fixture or f==a.fixture for n in range(a.runs)]
random.Random(20260911).shuffle(jobs)
with open(a.output,'w',newline='') as out:
 w=csv.DictWriter(out,lineterminator='\n',fieldnames=['app','fixture','run','exit_code','content_verified','process_to_exit_ms','output_bytes','exe_sha256','fixture_sha256']);w.writeheader()
 for app,f,n in jobs:
  command=[x.replace('{fixture}',str(root/f)) for x in app['command']]
  start=time.perf_counter()
  try:
   r=subprocess.run(command,stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=60)
   ms=(time.perf_counter()-start)*1000; code=r.returncode; size=len(r.stdout)
   heading=(root/f).read_text(encoding='utf-8-sig').splitlines()[0].lstrip('# ').strip()
   plain=re.sub(rb'\x1b\[[0-?]*[ -/]*[@-~]',b'',r.stdout).decode('utf-8',errors='replace')
   verified=heading in plain
  except subprocess.TimeoutExpired: ms=60000;code='timeout';size=0;verified=False
  w.writerow(dict(app=app['id'],fixture=f,run=n+1,exit_code=code,content_verified=verified,process_to_exit_ms=round(ms,3),output_bytes=size,exe_sha256=hashlib.file_digest(open(command[0],'rb'),'sha256').hexdigest(),fixture_sha256=hashlib.file_digest((root/f).open('rb'),'sha256').hexdigest()));out.flush()
