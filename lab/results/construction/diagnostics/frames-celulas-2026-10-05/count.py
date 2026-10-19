"""Count isolated subframe entries in release, outside the wall-time samples."""
import datetime,gzip,hashlib,json,os,re,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
OUT=Path('/tmp/tekt-cell-frames-2026-10-05')
paired=json.load(gzip.open(HERE/'bench-paired.json.gz','rt'))
record={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':paired['head'],'working_tree_hashes':paired['working_tree_hashes'],'diff_stat':paired['diff_stat'],'cases':[]}
for engine,exe in {'baseline':OUT/'typst-baseline','candidate':ROOT/'target/release/typst'}.items():
 symbols=subprocess.check_output(['nm',str(exe)],text=True)
 names=[line.split()[-1] for line in symbols.splitlines() if 'layout_sub_frame_with_origin' in line and ' t ' in line]
 if not names:raise RuntimeError('No subframe symbol found')
 script=OUT/f'subframes-{engine}.gdb'
 breaks='\n'.join("break '"+name+"'\ncommands\nsilent\nset $frame_calls = $frame_calls + 1\ncontinue\nend" for name in names)
 script.write_text('set pagination off\nset breakpoint pending off\nset $frame_calls = 0\n'+breaks+'\nrun\nprintf "\\nFRAME_CALLS=%d\\n", $frame_calls\n')
 for name in ['grid320','show_grid320','numbered_grid320','plain_multi_table','wrapped_tokens','rtl_tokens','multi_table']:
  source=paired['fixtures'][name]['source'];fixture=OUT/f'count-{name}.typ';fixture.write_text(source)
  cmd=['gdb','--batch','-x',str(script),'--args',str(exe),'compile',str(fixture),str(OUT/f'count-{name}-{engine}.pdf'),'--pdf-tagged=false','--pretty=false']
  p=subprocess.run(cmd,cwd=ROOT,env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1',NO_COLOR='1'),capture_output=True,text=True,timeout=60)
  match=re.search(r'FRAME_CALLS=(\d+)',p.stdout);calls=int(match[1]) if match else None
  record['cases'].append({'fixture':name,'source':source,'engine':engine,'command':cmd,'binary_sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),'script':script.read_text(),'returncode':p.returncode,'calls':calls,'stdout':p.stdout,'stderr':p.stderr})
  print(name,engine,p.returncode,calls)
record['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(HERE/'count.json.gz','wt') as f:json.dump(record,f,ensure_ascii=False,indent=2)
assert all(x['returncode']==0 and x['calls'] for x in record['cases'])
