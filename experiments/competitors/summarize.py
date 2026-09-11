"""Render medians and observed ranges, retaining failures separately."""
import collections,csv,pathlib,statistics
root=pathlib.Path(__file__).resolve().parent
parts=['# Measured desktop resources\n','Five samples per cell unless the count says otherwise. See [method/environment](environment.md) and [feature coverage](features.md). Window proxy is NOT first-content latency. Memory is process-tree RSS/working set at the fixed checkpoint; shared pages may be counted repeatedly. CPU is ms per second (1000 = one core). Live at checkpoint means a window and surviving process tree (plus an Electron renderer where checked), not successful whole-document rendering.\n']
for platform in ['windows','linux']:
 path=root/(platform+'.csv')
 if not path.exists():continue
 groups=collections.defaultdict(list)
 paths=[path]
 anchors=root/(platform+'-anchors.csv')
 if anchors.exists():paths.append(anchors)
 for source in paths:
  for r in csv.DictReader(source.open()):groups[(r['fixture'],r['app'])].append(r)
 parts+=['## '+platform.title()+'\n','| Fixture | App | Live at checkpoint / attempted | Window proxy median ms (range) | RSS median MiB | Private median MiB | CPU median ms/s |\n|---|---|---:|---:|---:|---:|---:|']
 for (fixture,app),rows in sorted(groups.items()):
  valid=[r for r in rows if r['status']=='window-observed' and int(r['checkpoint_processes'])>0]
  if platform=='linux' and app=='obsidian':
   parts.append(f'| {fixture} | {app} (keyring setup; excluded) | {len(valid)}/{len(rows)} | — | — | — | — |')
   continue
  if fixture=='stress-2m.md' and app=='typora':
   parts.append(f'| {fixture} | {app} (file rejected; excluded) | {len(valid)}/{len(rows)} | — | — | — | — |')
   continue
  def med(key):
   values=[float(r[key]) for r in valid if r[key] and (key!='cpu_ms_per_second' or float(r[key])>=0)]
   return f"{statistics.median(values):.1f}" if values else '—'
  times=[float(r['window_proxy_ms']) for r in valid]
  proxy=med('window_proxy_ms')+(f' ({min(times):.0f}–{max(times):.0f})' if times else '')
  label=app
  if fixture=='stress-2m.md' and (app=='marktext' or (platform=='linux' and app in ['markmello','vscode-preview'])):label+=' (blank pilot)'
  parts.append(f"| {fixture} | {label} | {len(valid)}/{len(rows)} | {proxy} | {med('checkpoint_rss_mib')} | {med('checkpoint_private_mib')} | {med('cpu_ms_per_second')} |")
 parts.append('')
 parts.append('Typora explicitly rejected the stress file in a separate pilot; its raw readings describe an error screen and are excluded. Rows marked “blank pilot” retain process-resource measurements, but the separate large-file spot check showed no document content; do not treat them as completed-render memory or speed results. See [observations](observations.md).\n')
 if platform=='linux':parts.append('Obsidian Linux resource samples remain in the raw CSV but are excluded above: a keyring-creation prompt appeared over the reading view in the pilot. No credentials were entered or keyring settings changed. This is an environment/setup limitation, not an app rendering failure.\n')
for platform in ['windows','linux']:
 path=root/(platform+'-cli.csv')
 if not path.exists():continue
 groups=collections.defaultdict(list)
 for r in csv.DictReader(path.open()):groups[(r['fixture'],r['app'])].append(r)
 parts+=['## '+platform.title()+' CLI conversion (separate workload)\n','Output captured through a pipe; no terminal painting, browser or GUI measured. Success requires exit code zero and the expected fixture heading in output after stripping ANSI sequences. These timings must not be ranked against desktop window proxies.\n','| Fixture | App | Successful/attempted | Process-to-exit median ms | Range ms |\n|---|---|---:|---:|---:|']
 for (f,app),rows in sorted(groups.items()):
  good=[float(r['process_to_exit_ms']) for r in rows if r['exit_code']=='0' and r.get('content_verified')=='True']
  parts.append(f"| {f} | {app} | {len(good)}/{len(rows)} | {statistics.median(good):.1f} | {min(good):.1f}–{max(good):.1f} |" if good else f'| {f} | {app} | 0/{len(rows)} | — | — |')
 parts.append('')
(root/'results.md').write_text('\n'.join(parts).rstrip()+'\n',encoding='utf-8',newline='\n')
