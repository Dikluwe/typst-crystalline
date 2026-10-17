from pathlib import Path
import hashlib,json,re,tomllib,gzip,datetime
R=Path.cwd();D=Path(__file__).parent;P=R/'00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
m=json.loads((P/'manifest.json').read_text());assert sha(P/'manifest.json')=='3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df'
assert sha(R/m['measurement'])==m['measurement_sha256']
def framed(data,domain,deps):
 h=hashlib.sha256(data);h.update(b'\0'+domain+b'\0')
 for path,digest in sorted(deps):
  name=path.encode();h.update(len(name).to_bytes(8,'big'));h.update(name);h.update(b' ');h.update(digest)
 return h.digest()
seen=set()
def nucleus(path,stack=()):
 assert path not in stack
 p=P/path if (P/path).exists() else R/path;data=p.read_bytes();doc=tomllib.loads(data.decode());assert doc['tekt']==1 and doc['kind']=='nucleus';deps=[]
 for ref in doc.get('depends',[]):
  digest=nucleus(ref['path'],(*stack,path));assert digest.hex()==ref['sha256'];deps.append((ref['path'],digest))
 return framed(data,b'TEKT-NUCLEUS-DEPS-V1',deps)
assert sha(P/m['nucleus']['path'])==m['nucleus']['candidate_sha256'];assert nucleus(m['nucleus']['path']).hex()==m['nucleus']['pin']
assert len({o['owner'] for o in m['owners']})==len(m['owners'])==len({o['consumer'] for o in m['owners']})
users=[]
for row in m['owners']:
 f=row['consumer'];o=row['owner'];assert sha(R/f)==row['base_source_sha256'];assert sha(R/o)==sha(P/'base'/o)==row['base_l0_sha256'];assert sha(P/o)==row['candidate_sha256']
 assert re.search(r'@prompt (\S+)',(R/f).read_text())[1]==o
 spec=(P/o).read_text();refs=re.findall(r'(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([a-f0-9]{64})$',spec);assert refs==sorted(set(refs));deps=[]
 for path,h in refs:
  digest=nucleus(path);assert digest.hex()==h;deps.append((path,digest))
  if path==m['nucleus']['path']:users.append(o)
 clean=re.sub(r'(?m)^Hash do Código: [^\n]*\n','',spec).encode();ph=(framed(clean,b'TEKT-PROMPT-NUCLEI-V1',deps) if deps else hashlib.sha256(clean).digest()).hex()[:8];assert ph==row['candidate_prompt_hash']
assert len(users)>=2
baseline=json.loads(gzip.decompress((D/'before-state.json.gz').read_bytes()));changed=[p for p,h in baseline['sha256'].items() if p.startswith(('01_core/','02_shell/','03_infra/','04_wiring/','00_nucleo/prompts/')) and (not (R/p).exists() or sha(R/p)!=h)];assert not changed,changed
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'manifest_sha256':sha(P/'manifest.json'),'owners':len(m['owners']),'nucleus_consumers':users,'base_and_candidate_hashes_verified':True,'productive_changes_since_baseline':changed,'implementation_status':'not-started-pending-public-contract-ratification'}
(D/'package-verification.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps(result,ensure_ascii=False,indent=2))
