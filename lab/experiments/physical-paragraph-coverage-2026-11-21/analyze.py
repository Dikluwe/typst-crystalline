from pathlib import Path
import importlib.util,sys,json,tarfile
from collections import Counter
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('previous',D.parent/'references-500-visible-pre-pdf-2026-11-21/analyze.py');old=importlib.util.module_from_spec(spec);spec.loader.exec_module(old)
base=old.base
out=D.parents[1]/'results/construction/diagnostics/physical-paragraph-coverage-2026-11-21'
rows=[json.loads(x) for x in (out/'measurements.jsonl').read_text().splitlines()]
tables={a:base.symbols(out/f'{a}.symbols') for a in ['candidate','control']}
tables['vanilla']=base.symbols(out.parent/'references-500-visible-page-operator-cost-2026-11-21/vanilla-symbols-twin.txt')
profiles=[];stacks={a:Counter() for a in tables}
with tarfile.open(out/'profiles.tar.gz') as archive:
 for row in rows:
  if not row['period_ns']:continue
  counts,detail=base.stacks(archive,row,tables);hist=Counter()
  reservation=Counter()
  for stack,n in counts.items():
   kind=old.category(row['arm'],stack)
   if any('lab_substitution_inert' in f or 'lab_preserve_substitution' in f for f in stack):kind='context_substitution'
   if any('::paragraph_session::request' in f for f in stack):
    reservation['paragraph_request']+=n
   if any('::paragraph_identity' in f or '::measure_paragraph_ascii' in f for f in stack):
    reservation['paragraph_identity']+=n
   hist[kind]+=n;stacks[row['arm']][(kind,tuple(f for f in stack if 'typst' in f)[:14])]+=n
  phase={k:n*detail['main_ns']/1e6/counts.total() for k,n in hist.items()}
  phase.update({k:n*detail["main_ns"]/1e6/counts.total() for k,n in reservation.items()})
  profiles.append(dict(row,**detail,phase_ms=phase))
def pairs(items,key,other):
 a={r['block']:r[key] for r in items if r['arm']=='candidate'};b={r['block']:r[key] for r in items if r['arm']==other}
 return base.stats([a[k]-b[k] for k in a])
nominal=[dict(r,cpu=r['cpu_ns']/1e6) for r in rows if not r['period_ns']]
result={'nominal':{a:base.stats([r['cpu'] for r in nominal if r['arm']==a]) for a in tables},'paired':{a:pairs(nominal,'cpu',a) for a in ['control','vanilla']},'phases':{a:old.summarize([r for r in profiles if r['arm']==a]) for a in tables},'profiles':profiles}
result['phase_paired_control']={label:pairs([dict(r,value=r['phase_ms'].get(label,0)) for r in profiles],'value','control') for label in ('context_substitution','context_execution','physical','eval','pdf_and_wrapper','paragraph_request','paragraph_identity')}
(out/'timing.json').write_text(json.dumps(result,indent=2))
with (out/'phase-stacks.txt').open('w') as f:
 for a,c in stacks.items():
  print(a,file=f)
  for k,n in c.most_common(60):print(n,k,file=f)
print(json.dumps({k:v for k,v in result.items() if k!='profiles'},indent=2))
