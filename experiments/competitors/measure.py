"""Fresh-process desktop resource samples. Window detection is NOT content readiness.

Requires psutil; Windows additionally pythonnet. Linux requires xdotool/X11.
Each app config must use a dedicated benchmark profile, and pass the fixture path.
"""
import argparse, csv, hashlib, json, os, pathlib, random, shutil, subprocess, time
import psutil

def tree(root):
    try: return [root]+root.children(recursive=True)
    except psutil.Error: return []

def snapshot(root):
    totals={'rss_bytes':0,'private_bytes':0,'cpu_s':0,'processes':0}
    for p in tree(root):
        try:
            m=p.memory_info(); c=p.cpu_times()
            totals['rss_bytes']+=m.rss
            # USS avoids double-counting shared pages on Linux; Windows PrivateUsage
            # is committed private memory, a different metric. Do not compare across OS.
            totals['private_bytes']+=getattr(m,'private',0) if os.name=='nt' else p.memory_full_info().uss
            totals['cpu_s']+=c.user+c.system; totals['processes']+=1
        except psutil.Error: pass
    return totals

def main():
    parser=argparse.ArgumentParser(); parser.add_argument('config'); parser.add_argument('--runs',type=int,default=10); parser.add_argument('--output',required=True); parser.add_argument('--app'); parser.add_argument('--fixture'); parser.add_argument('--hold',action='store_true'); args=parser.parse_args()
    config=json.loads(pathlib.Path(args.config).read_text()); root=pathlib.Path(config['fixtures']).resolve()
    if os.name=='nt':
        import clr
        from System.Diagnostics import Process
    jobs=[(a,f,n) for a in config['apps'] if not args.app or a['id']==args.app for f in config['files'] if not args.fixture or f==args.fixture for n in range(args.runs)]
    random.Random(20260911).shuffle(jobs)
    dest=pathlib.Path(args.output); dest.parent.mkdir(parents=True,exist_ok=True)
    fields=['app','fixture','run','status','window_proxy_ms','checkpoint_rss_mib','checkpoint_private_mib','checkpoint_processes','cpu_ms_per_second','sample_duration_s','exe_sha256','fixture_sha256']
    with dest.open('w',newline='') as stream:
      writer=csv.DictWriter(stream,lineterminator='\n',fieldnames=fields); writer.writeheader()
      for app,fixture,n in jobs:
        path=root/fixture; env=os.environ.copy(); overrides=app.get('env',{}).copy()
        # Distinct profile per sample prevents session-restored tabs and caches from
        # accumulating across fixtures. No existing user profile is deleted.
        profile=root.parent/'sample-profiles'/str(os.getpid())/app['id']/fixture/str(n)
        profile.mkdir(parents=True,exist_ok=True)
        for key in ['APPDATA','LOCALAPPDATA','WEBVIEW2_USER_DATA_FOLDER','XDG_CONFIG_HOME','XDG_CACHE_HOME','XDG_DATA_HOME']:
            if key in overrides and not app.get('preserve_profile'):
                overrides[key]=str(profile/key); pathlib.Path(overrides[key]).mkdir(exist_ok=True)
        env.update(overrides)
        command=[x.replace('{fixture}',str(path)) for x in app['command']]
        if not app.get('preserve_profile'):
            command=[('--user-data-dir='+str(profile/'electron')) if x.startswith('--user-data-dir=') else x for x in command]
        if app.get('prepare')=='vscode':
            settings=profile/'electron'/'User';settings.mkdir(parents=True,exist_ok=True)
            (settings/'settings.json').write_text(json.dumps({'workbench.startupEditor':'none','workbench.welcomePage.experimentalOnboarding':False,'workbench.editorAssociations':{'*.md':'vscode.markdown.preview.editor'}}))
            extensions=profile/'extensions';adapter=extensions/'fmv-benchmark.preview-0.0.1';adapter.mkdir(parents=True)
            (adapter/'package.json').write_text(json.dumps({'name':'preview','publisher':'fmv-benchmark','version':'0.0.1','engines':{'vscode':'^1.100.0'},'capabilities':{'untrustedWorkspaces':{'supported':True}},'activationEvents':['onStartupFinished'],'main':'index.js'}))
            script="const v=require('vscode'); exports.activate=async()=>{await v.commands.executeCommand('vscode.openWith',v.Uri.file("+json.dumps(str(path))+"),'vscode.markdown.preview.editor');};"
            (adapter/'index.js').write_text(script)
            command=[x for x in command if not x.startswith('--extensions-dir=')]+['--extensions-dir='+str(extensions)]
        if app.get('prepare')=='obsidian':
            vault=profile/'vault';vault.mkdir()
            shutil.copy2(path,vault/path.name)
            for resource in root.glob('*.svg'):shutil.copy2(resource,vault/resource.name)
            state=vault/'.obsidian';state.mkdir()
            (state/'app.json').write_text(json.dumps({'defaultViewMode':'preview'}))
            leaf={'id':'document','type':'leaf','state':{'type':'markdown','state':{'file':fixture,'mode':'preview','source':False}}}
            workspace={'main':{'id':'main','type':'split','children':[{'id':'tabs','type':'tabs','children':[leaf]}],'direction':'vertical'},'active':'document','lastOpenFiles':[fixture],'left-ribbon':{'hiddenItems':{}}}
            for side in ['left','right']:
                workspace[side]={'id':side,'type':'split','children':[],'direction':'horizontal','width':300,'collapsed':True}
            (state/'workspace.json').write_text(json.dumps(workspace))
            data=profile/'electron';data.mkdir(exist_ok=True)
            (data/'obsidian.json').write_text(json.dumps({'vaults':{'a1b2c3d4e5f60718':{'path':str(vault),'ts':1,'open':True}}}))
            command=[x for x in command if x!=str(path)]
        started=time.perf_counter(); proc=subprocess.Popen(command,env=env,stdin=subprocess.DEVNULL,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL); p=psutil.Process(proc.pid)
        window_ms=None; status='no-window'
        try:
          if os.name=='nt': dotnet=Process.GetProcessById(proc.pid)
          while time.perf_counter()-started<20:
            if proc.poll() is not None: status='exited-'+str(proc.returncode); break
            if os.name=='nt':
                dotnet.Refresh(); visible=int(dotnet.MainWindowHandle.ToInt64())!=0
            else:
                visible=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(proc.pid)],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL).returncode==0
            if visible: window_ms=(time.perf_counter()-started)*1000; status='window-observed'; break
            time.sleep(.01)
          # Fixed checkpoint after window creation; not a claim that async work is idle.
          time.sleep(3)
          before=snapshot(p); t=time.perf_counter(); time.sleep(1); after=snapshot(p); interval=time.perf_counter()-t
          if proc.poll() is not None: status='exited-'+str(proc.returncode)
          elif app.get('prepare') in ['obsidian','vscode'] or app['id'] in ['marktext','typora']:
            renderer=False
            for child in tree(p):
              try: renderer=renderer or '--type=renderer' in child.cmdline()
              except psutil.Error: pass
            if not renderer:status='renderer-exited'
          row=dict(app=app['id'],fixture=fixture,run=n+1,status=status,window_proxy_ms=round(window_ms,3) if window_ms else '',checkpoint_rss_mib=round(after['rss_bytes']/2**20,3),checkpoint_private_mib=round(after['private_bytes']/2**20,3),checkpoint_processes=after['processes'],cpu_ms_per_second=round((after['cpu_s']-before['cpu_s'])*1000/interval,3),sample_duration_s=round(interval,4),exe_sha256=hashlib.file_digest(open(command[0],'rb'),'sha256').hexdigest(),fixture_sha256=hashlib.file_digest(path.open('rb'),'sha256').hexdigest())
          writer.writerow(row); stream.flush(); print(json.dumps(row),flush=True)
          if args.hold: input('Inspect this app, then press Enter to close: ')
        finally:
          owned=tree(p)
          for child in reversed(owned):
            try: child.terminate()
            except psutil.Error: pass
          _,alive=psutil.wait_procs(owned,timeout=3)
          for child in alive:
            try: child.kill()
            except psutil.Error: pass
          time.sleep(.3)
if __name__=='__main__': main()
