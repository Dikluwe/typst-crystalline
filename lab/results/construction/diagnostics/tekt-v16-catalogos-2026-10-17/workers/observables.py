from pathlib import Path
import gzip,json,datetime,re
out=Path(__file__).resolve().parents[1];before=json.load(gzip.open(out/'before-measurement.json.gz'));after=json.load(gzip.open(out/'after-measurement.json.gz'))
key=lambda r:(r['name'],r['binary']);a={key(r):r for r in before['cases']};z={key(r):r for r in after['cases']};assert a.keys()==z.keys()
changed=[k for k in a if any(a[k][f]!=z[k][f] for f in ['exit','stdout','stderr'])];assert not changed,changed
# Cross-backend criterion excludes terminal source excerpts, but includes primary error/hints.
def observable(r):
 lines=r['stderr'].splitlines();return r['exit'],r['stdout'],[x for x in lines if x.startswith(('error:','hint:','help:')) or re.match(r'^\s*=\s*(hint|help):',x)]
pairs={n:{k:z[n,k] for k in ['tekt','vanilla']} for n,k in z if k=='tekt'};gaps=[dict(name=n,tekt=observable(v['tekt']),vanilla=observable(v['vanilla'])) for n,v in pairs.items() if observable(v['tekt'])!=observable(v['vanilla'])]
assert len(gaps)==18,len(gaps)
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(pairs),invocations=len(z),all_within_backend_stdout_stderr_exit_preserved=True,matching_cross_backend_pairs=len(pairs)-len(gaps),preexisting_diagnostic_gaps=gaps,criterion='Within each backend: exact exit/stdout/full stderr before-after. Across backends: exit/stdout/primary diagnostic and hint lines; not source excerpt formatting. All gaps retained, no parity claim for omitted members.')
(out/'observables-result.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps({k:v for k,v in r.items() if k!='preexisting_diagnostic_gaps'}),flush=True)
