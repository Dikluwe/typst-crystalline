"""Explicit conversation scope; never stage the repository wholesale."""
from pathlib import Path
import subprocess,json,datetime,hashlib
ROOT=Path(__file__).resolve().parents[3];R=Path(__file__).resolve().parent;D=ROOT/'00_nucleo/diagnosticos'
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
assert not git('diff','--cached','--name-only').strip(),'Preserve unrelated staged changes'
(R/'before-status.txt').write_text(git('status','--porcelain=v1','--untracked-files=all'))
(R/'before-diff-stat.txt').write_text(git('diff','HEAD','--stat'))
sets=['performance-baseline','font-lifecycle-experiment','font-microbenchmarks','font-microbenchmarks-vanilla','vanilla-font-mechanics','reuse-trust-study','reuse-key-followup','shaping-axis-key-fix','shaper-prompt-audit','shaper-contract-fixes','infra-contract-review','infra-axis-gate-review','infra-performance-followup','shaper-language-fix','font-demand-geometry-study','subframe-font-demand-followup']
verified={}
for name in sets:
 p=D/name
 for line in (p/'MANIFEST.sha256').read_text().splitlines():
  digest,relative=line.split(maxsplit=1);assert sha(p/relative.lstrip('*'))==digest,(name,relative)
 audit=p/'completion-audit.json';a=json.loads(audit.read_text()) if audit.exists() else {}
 if a.get('report_sha256'):assert sha(p.with_suffix('.md'))==a['report_sha256'],name
 verified[name]={'manifest_sha256':sha(p/'MANIFEST.sha256'),'report_sha256':sha(p.with_suffix('.md'))}
(R/'prior-seals.json').write_text(json.dumps(verified,indent=2)+'\n')
def owners(paths):
 result=set()
 for path in paths:
  p=ROOT/path;result.add(path)
  if p.is_file() and p.suffix=='.rs':
   for line in p.read_text().splitlines()[:12]:
    if line.startswith('//! @prompt '):
     owner=line[len('//! @prompt '):];assert (ROOT/owner).is_file(),owner;result.add(owner)
 return sorted(result)
def artifacts(names):return [f'00_nucleo/diagnosticos/{name}{suffix}' for name in names for suffix in ['', '.md']]
life=json.loads((D/'font-lifecycle-experiment/completion-audit.json').read_text())['final_consumer_changes']
# Their actual modular composition requires the corresponding private descendants.
life+=['03_infra/src/world.rs','03_infra/src/pipeline.rs']
for folder in ['03_infra/src/world','03_infra/src/pipeline']:
 life+=[str(p.relative_to(ROOT)) for p in (ROOT/folder).glob('*.rs')]
shaper=['03_infra/src/shaper.rs']+[str(p.relative_to(ROOT)) for p in (ROOT/'03_infra/src/shaper').glob('*.rs')]
groups=[
 ('Benchmark suite', 'test(benches): guard release measurement and classify PDF export cohorts', ['benches','00_nucleo/diagnosticos/benchmark-suite-review.md']),
 ('Font catalog lifecycle', 'fix(fonts): preserve lazy per-world catalog acquisition and reuse', owners(life)+artifacts(['font-lifecycle-experiment'])),
 ('Pinned font measurements', 'perf(fonts): archive comparative microbenchmarks and vanilla mechanics', artifacts(['font-microbenchmarks','font-microbenchmarks-vanilla','vanilla-font-mechanics'])),
 ('Reuse and shaper contracts', 'fix(shaper): integrate cache identity and fallback contract repairs', owners(shaper)+artifacts(['reuse-trust-study','reuse-key-followup','shaping-axis-key-fix','shaper-prompt-audit','shaper-contract-fixes'])),
 ('Performance baseline', 'perf: archive baseline and post-correction phase investigation', artifacts(['performance-baseline','infra-performance-followup'])),
]
commits=[]
for label,message,paths in groups:
 for p in paths:assert (ROOT/p).exists(),p
 subprocess.run(['git','add','--',*sorted(set(paths))],cwd=ROOT,check=True)
 staged=git('diff','--cached','--name-only').splitlines()
 if not staged:continue
 with (R/(str(len(commits)+1)+'.commit.stdout')).open('w') as out:
  subprocess.run(['git','commit','-m',message],cwd=ROOT,check=True,stdout=out)
 record={'label':label,'head':git('rev-parse','HEAD').strip(),'message':message,'files':staged,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat()};commits.append(record);(R/'commits.json').write_text(json.dumps(commits,indent=2)+'\n');print(record['head'][:9],label,flush=True)
print('Explicit groups committed; unrelated remaining work preserved.',flush=True)
