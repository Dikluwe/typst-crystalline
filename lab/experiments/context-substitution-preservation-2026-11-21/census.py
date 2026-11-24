from pathlib import Path
import sys,subprocess,json,os
D=Path(__file__).resolve().parent
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK,STATES,cycle,golden
out=D.parents[1]/'results/construction/diagnostics/context-substitution-preservation-2026-11-21'
names={'wait':'::watch::wait_for_change_since','substitution_roots':'::pipeline::substitute_context_blocks','substitution_nodes':'::pipeline::substitute_context_blocks_in_chain','realize':'::context_stabilization::Session::realize','preserved_subtrees':'::pipeline::lab_preserve_substitution'}
results={}
for arm in ['control','candidate']:
 table=subprocess.check_output(['nm','-n','-S','-C','--defined-only',str(WORK/arm)],text=True)
 entries=[x.split(maxsplit=3) for x in table.splitlines()]
 addresses={k:sorted({int(x[0],16) for x in entries if len(x)==4 and x[2] in ('t','T') and x[3].split('.llvm.',1)[0].endswith(v)}) for k,v in names.items()}
 assert all(v for k,v in addresses.items() if k!='preserved_subtrees'),addresses
 case=WORK/'census'/arm;(case/'edits').mkdir(parents=True)
 for n,(_,text,leaf) in enumerate(STATES):
  assert leaf is None
  (case/('main.typ' if n==0 else f'edits/{n}.typ')).write_bytes(text)
 (case/'addresses.json').write_text(json.dumps(addresses))
 env=cycle.environment(case,'candidate',False);env.update(TEKT_LAB_MEMO_PROTECT='1',TEKT_LAB_MEMO_BYTES=str(8*1024*1024))
 with (case/'gdb.log').open('wb') as f:
  subprocess.run(['taskset','-c','1','gdb','-q','-nx','-batch','-x',str(D/'census.gdb'),'--args',str(WORK/arm),'watch','main.typ','out.pdf','--root',str(case),'--pdf-tagged=true'],cwd=case,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=180,check=True)
 for n,(name,_,_) in enumerate(STATES):assert cycle.cycle.key('candidate',case/f'publication-{n}.pdf')==golden('candidate',name)
 results[arm]=json.loads((case/'counts.json').read_text())
 print(arm,results[arm],flush=True)
(out/'census.json').write_text(json.dumps(results,indent=2))
