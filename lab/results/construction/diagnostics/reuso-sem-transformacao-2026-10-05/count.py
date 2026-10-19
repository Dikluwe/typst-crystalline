"""Count initial pure introspection entry calls, without productive probes."""
import datetime,gzip,hashlib,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
OUT=Path('/tmp/tekt-noop-reuse-2026-10-05');OUT.mkdir(exist_ok=True)
fixture=OUT/'count.typ';source='#show heading: it => upper(it.body)\n= Original';fixture.write_text(source)
script=OUT/'count.gdb';script.write_text('''set pagination off
set breakpoint pending off
set $index_calls = 0
break 'typst_core::compiler::introspect::entrypoints::introspect_with_introspector'
commands
silent
set $index_calls = $index_calls + 1
continue
end
run
printf "\\nINDEX_CALLS=%d\\n", $index_calls
''')
engines={'baseline':OUT/'typst-baseline','candidate':ROOT/'target/release/typst'}
git=lambda *args:subprocess.check_output(['git',*args],cwd=ROOT,text=True)
r={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'source':source,'script':script.read_text(),'cases':[]}
for mode in ['pdf','html','query']:
 for name,exe in engines.items():
  args=['query',str(fixture),'heading','--field','body'] if mode=='query' else ['compile',str(fixture),str(OUT/f'count-{name}.{mode}'),'--root',str(OUT)]
  if mode=='pdf':args+=['--pdf-tagged=false','--pretty=false']
  if mode=='html':args+=['--features','html','--pretty=false']
  cmd=['gdb','--batch','-x',str(script),'--args',str(exe),*args]
  p=subprocess.run(cmd,cwd=ROOT,env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1',NO_COLOR='1'),capture_output=True,text=True,timeout=60)
  r['cases'].append({'mode':mode,'engine':name,'command':cmd,'binary_sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
  print(mode,name,p.returncode,[line for line in p.stdout.splitlines() if 'INDEX_CALLS=' in line],p.stderr[-180:] if p.returncode else '')
r['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(HERE/'count.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
