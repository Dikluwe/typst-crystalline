from pathlib import Path
import sys,subprocess,json,os,hashlib
D=Path(__file__).resolve().parent
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK,STATES,cycle,golden
out=D.parents[1]/'results/construction/diagnostics/physical-paragraph-coverage-2026-11-21'
names={
 'wait':'::watch::wait_for_change_since',
 'paragraph_identity':'::paragraph_identity',
 'ascii_masks':'::measure_paragraph_ascii',
 'line_commit':'::flush_line_with_outer_singleton',
 'emit_source_product':'::paragraph_emit::emit_source_product',
 'prepare_parts':'::paragraph_prepare::prepare_parts',
 'physical_root':'::convergence::run_planned_lab',
}
results={}
for arm in ['control','candidate']:
 table=subprocess.check_output(['nm','-n','-S','-C','--defined-only',str(WORK/arm)],text=True)
 entries=[x.split(maxsplit=3) for x in table.splitlines()]
 addresses={k:sorted({int(x[0],16) for x in entries if len(x)==4 and x[2] in ('t','T') and x[3].split('.llvm.',1)[0].endswith(v) and (
                     k != 'paragraph_identity' or 'FallbackFontMetrics' in x[3])}) for k,v in names.items()}
 assert all(v for k,v in addresses.items() if k!='ascii_masks' or arm=='candidate'),addresses
 label = 'census'
 if '--detail' in sys.argv:
  expected, probes = {
   'control': ('32e3d19e947177169330179781a849dc257bc04d5fa8538abdfd687dad74345e',
               {'cmap_calls':[0x2a18ff3]}),
   'candidate': ('e403f924330882bebbdcfd5d4ea36fc582a1bea7c272053417a493c7b35b0fea',
                 {'ascii_requests':[0x290a57a], 'cmap_calls':[0x290a66d]}),
  }[arm]
  assert hashlib.sha256((WORK/arm).read_bytes()).hexdigest() == expected
  addresses.update(probes)
  label = 'census-detail'
 case=WORK/label/arm;(case/'edits').mkdir(parents=True)
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
(out/f'{label}.json').write_text(json.dumps(results,indent=2))
