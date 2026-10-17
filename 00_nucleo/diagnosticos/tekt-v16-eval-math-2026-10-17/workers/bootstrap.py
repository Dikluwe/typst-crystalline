from pathlib import Path
import json,gzip,hashlib,subprocess,re,shutil,datetime
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-eval-math-2026-10-17';out.mkdir();(out/'workers').mkdir();prev=root/'00_nucleo/diagnosticos/tekt-v16-eval-helpers-2026-10-17';last=json.load(gzip.open(prev/'final-state.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in last['source_hashes'].items())
sources=['01_core/src/compiler/eval/tests.rs','01_core/src/compiler/introspect/walk.rs'];owners=[re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1) for p in sources]
b={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'hashes':last['source_hashes'],'files':{p:Path(p).read_text() for p in sources+owners},'linter':last['linter']}
with gzip.open(out/'before.json.gz','wt') as f:json.dump(b,f)
shutil.copyfile(prev/'validation-architecture.log',out/'baseline-architecture.log')
for name in ['run.py','seal_owned.py','strict.py','suite.py']:
 s=(prev/'workers'/name).read_text().replace(prev.name,out.name).replace('TEKT_EVAL_HELPERS_MEMORY_LIMIT','TEKT_EVAL_MATH_MEMORY_LIMIT');(out/'workers'/name).write_text(s)
s=(prev/'workers/audit.rs').read_text();(out/'workers/audit.rs').write_text(s)
sarif=json.load(open(out/'baseline-architecture.log'));rows=[r for r in sarif['runs'][0]['results'] if r['ruleId']=='V16' and r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')==sources[0]];assert len(rows)==52;(out/'input.sarif.json').write_text(json.dumps({'runs':[{'results':rows}]}));(out/'latent.json').write_text('[]\n')
cache=Path('/tmp/tekt-linter-none-target/debug/deps');cmd=['rustc','--edition=2021','--crate-name','eval_math_audit','-L','dependency='+str(cache)];record={}
for name in ['tree_sitter','tree_sitter_rust','serde_json']:
 paths=list(cache.glob('lib'+name+'-*.rlib'));assert len(paths)==1;cmd+=['--extern',name+'='+str(paths[0])];record[name]={'path':str(paths[0]),'sha256':sha(paths[0])}
cmd+=[str(out/'workers/audit.rs'),'-o','/tmp/tekt-eval-math-audit'];(out/'audit-recipe.json').write_text(json.dumps({'command':cmd,'externs':record},indent=2));shutil.copyfile('/tmp/tekt-eval-math-bootstrap.py',out/'workers/bootstrap.py');print('Prepared 52 diagnostic inputs; all sources unchanged')
