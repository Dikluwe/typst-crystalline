from pathlib import Path
import json,gzip,hashlib,subprocess,collections,re
R=Path.cwd();D=Path(__file__).parent
read=lambda p:json.loads(gzip.decompress(p.read_bytes()))
sha=lambda b:hashlib.sha256(b).hexdigest()
before=read(D/'before-code.json.gz');tested=read(D/'tested-final.json.gz');owners=json.loads((D/'owners.json').read_text());allowed={p for pair in owners for p in pair};prefix=('01_core/','02_shell/','03_infra/','04_wiring/','00_nucleo/prompts/')
assert before['head']==tested['head']==subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
for p,h in before['sha256'].items():
 if p.startswith(prefix) and p not in allowed:assert sha((R/p).read_bytes())==h,p
for p,h in tested['sha256'].items():
 if p.startswith(prefix):assert sha((R/p).read_bytes())==h,p
changed=subprocess.check_output(['git','ls-files','--modified','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'],text=True).splitlines()+subprocess.check_output(['git','ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'],text=True).splitlines()
for p in changed:
 if p not in before['sha256']:assert p in allowed,p
receipt=json.loads((D/'seals.json').read_text())
for p,owner in owners:
 src=(R/p).read_text();spec=(R/owner).read_text();row=receipt[p]
 assert sha(src.encode())==row['source_sha256'];assert sha(spec.encode())==row['l0_sha256']
 assert re.search(r'(?m)^//! @prompt '+re.escape(owner)+r'$',src)
 assert sha(re.sub(r'(?m)^//! @prompt-hash [^\n]*\n','',src).encode())[:8]==row['code_hash']
 assert sha(re.sub(r'(?m)^Hash do Código: [^\n]*\n','',spec).encode())[:8]==row['prompt_hash']
lint=json.loads((D/'lint.json').read_text())['runs'][0]['results'];counts=dict(collections.Counter(r.get('level','warning') for r in lint));assert counts.get('error',0)==0
assert sha((R/'target/release/typst').read_bytes()) == json.loads((D/'performance-final/identity.json').read_text())['bin_sha256']['current']
source=(R/'03_infra/src/fontdb/selective.rs').read_text(); prefix_source=source[:source.index('#[cfg(test)]')].replace('//!','//');assert (D/'probe/src/instrumented.rs').read_text().startswith(prefix_source)
rows={'head':tested['head'],'tested_utc':tested['utc'],'preserved_received_productive_tree':True,'authorized_owner_pairs':owners,'lint':counts,'test_results':re.findall(r'test result:.*',(D/'workspace.log').read_text()),'cli_query_results':re.findall(r'test result:.*',(D/'cli-query.log').read_text()),'bin_sha256':sha((R/'target/release/typst').read_bytes()),'seals':receipt}
(D/'final-verification.json').write_text(json.dumps(rows,indent=2)+'\n');print(json.dumps(rows,indent=2))
