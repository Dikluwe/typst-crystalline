"""External entry counts only; never use debugger duration as performance.

Requires baseline/candidate release trees in TEKT_STDLIB_RAM; outputs stay there.
"""
from pathlib import Path
import subprocess as sp,os,time,json,signal,shutil
D=Path(os.environ.get('TEKT_STDLIB_RAM','/dev/shm/tekt-stdlib-reuse-20261105')).resolve();R=Path(__file__).resolve().parents[3];F=R/'lab/experiments/stdlib-reuse/tests/fixtures';rows=[]
assert D.is_dir() and D.is_relative_to(Path('/dev/shm')) and D!=Path('/dev/shm'), 'exclusive RAM area required'
def symbols(b):
 out=[]
 for line in sp.check_output(['nm','-C','--defined-only',str(b)],text=True).splitlines():
  c=line.split(None,2)
  if len(c)!=3 or c[1] not in ['t','T']:continue
  name=c[2];label=None
  if name=='typst_core::compiler::stdlib::registry::make_stdlib_with_features':label='stdlib.build'
  elif name=='typst_core::entities::source::Source::new_with_parser':label='source.new'
  elif name=='typst_core::compiler::eval::modules::eval_imported_file':label='eval.import'
  elif name in ['typst_core::compiler::layout::convergence::run','typst_core::compiler::layout::convergence::run_planned']:label='convergence.run'
  elif name=='typst_core::compiler::layout::Layouter<M,S>::new':label='layouter.new'
  if label:out.append((int(c[0],16),label))
 assert any(x[1]=='stdlib.build' for x in out);return out
for variant,target in [('baseline','base-target'),('candidate','candidate-target')]:
 b=D/target/'release/typst';syms=symbols(b)
 for name,mode in [(f'imports_{n:02}.typ','cli') for n in [0,1,4,8,16,64]]+[(n+'.typ','cli') for n in ['nested_64','repeated_64']]+[(f'imports_{n:02}.typ','watch') for n in [0,8,16,64]]:
  P=D/'counts-source'/f'{variant}-{Path(name).stem}-{mode}';P.mkdir(parents=True);inputs=P/'inputs';shutil.copytree(F,inputs);main=inputs/name;original=main.read_text();pdf=P/'out.pdf';out=P/'counts.json';env=os.environ|{'TMPDIR':str(D/'tmp'),'RAYON_NUM_THREADS':'1','SOURCE_DATE_EPOCH':'0','STDLIB_COUNT_OUT':str(out),'STDLIB_COUNT_SYMBOLS':json.dumps(syms),'PYTHONDONTWRITEBYTECODE':'1'}
  with (P/'gdb.log').open('w') as log:
   p=sp.Popen(['gdb','--batch','-x',str(Path(__file__).with_name('count_gdb.py')),'--args',str(b),'compile' if mode=='cli' else 'watch',str(main),str(pdf),'--root',str(inputs),'--pdf-tagged=false'],stdout=log,stderr=log,env=env,start_new_session=True)
   def stamp():
    try:return pdf.stat().st_ino,pdf.stat().st_mtime_ns
    except FileNotFoundError:return None
   def wait(old):
    end=time.monotonic()+60
    while True:
     if stamp()!=old and pdf.exists() and pdf.read_bytes().rstrip().endswith(b'%%EOF'):return
     assert p.poll() is None and time.monotonic()<end, P
     time.sleep(.005)
   try:
    if mode=='cli':p.wait(timeout=60);assert p.returncode==0
    else:wait(None)
    snapshots=[]
    for cycle in range(1 if mode=='cli' else 3):
     if cycle:
      old=stamp();main.write_text(original if cycle==1 else original+'\nRevision 1\n');wait(old)
     time.sleep(.03);snapshots.append(json.loads(out.read_text())['counts'])
    rows.append(dict(variant=variant,fixture=name,mode=mode,snapshots=snapshots));(D/'call-counts-source.json').write_text(json.dumps(rows));print(rows[-1],flush=True)
   finally:
    if p.poll() is None:
     if out.exists():
      try:os.kill(json.loads(out.read_text())['inferior'],signal.SIGTERM)
      except ProcessLookupError:pass
     os.killpg(p.pid,signal.SIGTERM);p.wait(timeout=5)
