"""Count complete paged layout wrapper entries outside the timing benchmark."""
import datetime,gzip,hashlib,json,os,re,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
OUT=Path('/tmp/tekt-initial-layout-2026-10-05')
paired=json.load(gzip.open(HERE/'bench-paired.json.gz','rt'))
engines={'baseline':OUT/'typst-baseline','candidate':ROOT/'target/release/typst'}
record={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':paired['head'],'working_tree_hashes':paired['working_tree_hashes'],'diff_stat':paired['diff_stat'],'cases':[]}
sources={name:paired['fixtures'][name]['source'] for name in ['grid320','show_grid320','multi_table','numbered_grid320','show_headings500']}
sources['footer_context']='#set page(footer: context [PAGE #here().page()])\nA#pagebreak()B'
for engine,exe in engines.items():
 symbols=subprocess.check_output(['nm',str(exe)],text=True)
 names=[line.split()[-1] for line in symbols.splitlines() if 'paged53layout_with_realized_math_callbacks_from_with_metrics' in line]
 if len(names)!=1:raise RuntimeError(names)
 script=OUT/f'layout-{engine}.gdb'
 script.write_text(f'''set pagination off
set breakpoint pending off
set $layout_calls = 0
break '{names[0]}'
commands
silent
set $layout_calls = $layout_calls + 1
continue
end
run
printf "\\nLAYOUT_CALLS=%d\\n", $layout_calls
''')
 for name,source in sources.items():
  fixture=OUT/f'count-{name}.typ';fixture.write_text(source)
  cmd=['gdb','--batch','-x',str(script),'--args',str(exe),'compile',str(fixture),str(OUT/f'count-{name}-{engine}.pdf'),'--pdf-tagged=false','--pretty=false']
  p=subprocess.run(cmd,cwd=ROOT,env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1',NO_COLOR='1'),capture_output=True,text=True,timeout=60)
  match=re.search(r'LAYOUT_CALLS=(\d+)',p.stdout)
  calls=int(match[1]) if match else None
  record['cases'].append({'fixture':name,'source':source,'engine':engine,'command':cmd,'binary_sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),'script':script.read_text(),'returncode':p.returncode,'calls':calls,'stdout':p.stdout,'stderr':p.stderr})
  print(name,engine,p.returncode,calls)
record['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(HERE/'count.json.gz','wt') as f:json.dump(record,f,ensure_ascii=False,indent=2)
assert all(x['returncode']==0 and x['calls'] for x in record['cases'])
