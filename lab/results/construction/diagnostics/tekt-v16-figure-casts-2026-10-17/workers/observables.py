from pathlib import Path
import gzip,json,datetime,re
out=Path(__file__).resolve().parents[1];before=json.load(gzip.open(out/'before-expanded-measurement.json.gz'));after=json.load(gzip.open(out/'after-measurement.json.gz'));key=lambda r:(r['name'],r['binary']);a={key(r):r for r in before['cases']};z={key(r):r for r in after['cases']};assert a.keys()==z.keys()
names={n for n,k in a};expected={n for n in names if n.startswith('figure-') and a[n,'vanilla']['exit']==1 and (a[n,'tekt']['exit']==0 or n in ['figure-missing','figure-missing-kind'])};expected.update(['figure-body-array-kind','figure-body-bool-kind','figure-body-integer-kind']);changed={n for n,k in a if k=='tekt' and any(a[n,k][f]!=z[n,k][f] for f in ['exit','stdout','stderr'])};assert changed==expected,(sorted(changed-expected),sorted(expected-changed))
for n in names:
 for k in ['tekt','vanilla']:
  if k=='tekt' and n in expected:continue
  assert all(a[n,k][f]==z[n,k][f] for f in ['exit','stdout','stderr']),(n,k)
def diagnostic(r):
 lines=r['stderr'].splitlines();error=next((x for x in lines if x.startswith('error:')),None);anchor=next((m.group(1) for x in lines if (m:=re.search(r'<input-expression>:(\d+:\d+)',x))),None);hints=[x for x in lines if x.startswith(('hint:','help:'))];return dict(exit=r['exit'],error=error,anchor=anchor,hints=hints)
repairs=[]
for n in sorted(expected):
 t=diagnostic(z[n,'tekt']);v=diagnostic(z[n,'vanilla']);assert t==v,(n,t,v);repairs.append(dict(name=n,before=diagnostic(a[n,'tekt']),after=t,vanilla=v))
gaps=[dict(name=n,tekt=diagnostic(z[n,'tekt']),vanilla=diagnostic(z[n,'vanilla'])) for n in sorted(names) if diagnostic(z[n,'tekt'])!=diagnostic(z[n,'vanilla'])]
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(names),invocations=len(z),repairs=repairs,unchanged_tekt_cases=len(names)-len(repairs),oracle_unchanged=True,no_unexpected_changes=True,remaining_diagnostic_gaps=gaps,criterion='All full stdout/stderr/exits unchanged within backend except listed intended figure rejections. Repaired cases match pinned primary error, first source anchor and hint lines. Remaining JSON fields/serialization/type/synthesis/relative semantics are not certified as parity.')
(out/'observables-result.json').write_text(json.dumps(r,indent=2)+'\n');print(dict(pairs=len(names),repaired=len(repairs),unchanged=len(names)-len(repairs),remaining_diagnostic_gaps=len(gaps)))
