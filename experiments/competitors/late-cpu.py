"""Diagnostic: process-tree CPU after 30 seconds on the small fixture."""
import argparse,csv,json,os,pathlib,subprocess,time
import psutil
from measure import snapshot,tree
p=argparse.ArgumentParser();p.add_argument('config');p.add_argument('--output',required=True);a=p.parse_args();c=json.loads(pathlib.Path(a.config).read_text())
with open(a.output,'w',newline='') as out:
 w=csv.DictWriter(out,lineterminator='\n',fieldnames=['app','delay_s','sample_s','cpu_ms_per_second','rss_mib','processes']);w.writeheader()
 for app in c['apps']:
  if app['id'] not in ['fmv','aydiler']:continue
  env=os.environ.copy();env.update(app.get('env',{}));cmd=[x.replace('{fixture}',str(pathlib.Path(c['fixtures'])/'ordinary-5k.md')) for x in app['command']]
  proc=subprocess.Popen(cmd,env=env,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL);root=psutil.Process(proc.pid)
  try:
   time.sleep(30);before=snapshot(root);start=time.perf_counter();time.sleep(5);after=snapshot(root);duration=time.perf_counter()-start
   w.writerow({'app':app['id'],'delay_s':30,'sample_s':round(duration,3),'cpu_ms_per_second':round((after['cpu_s']-before['cpu_s'])*1000/duration,3),'rss_mib':round(after['rss_bytes']/2**20,3),'processes':after['processes']});out.flush()
  finally:
   owned=tree(root)
   for child in reversed(owned):
    try:child.terminate()
    except psutil.Error:pass
   _,alive=psutil.wait_procs(owned,timeout=3)
   for child in alive:
    try:child.kill()
    except psutil.Error:pass
