from pathlib import Path
import hashlib,json,re,tomllib
R=Path(__file__).resolve().parents[3]
D=Path(__file__).resolve().parent
def framed(data,domain,deps):
 h=hashlib.sha256(data);h.update(b'\0'+domain+b'\0')
 for path,digest in sorted(deps):
  name=path.encode();h.update(len(name).to_bytes(8,'big'));h.update(name);h.update(b' ');h.update(digest)
 return h.digest()
def nucleus(path):
 data=(R/path).read_bytes();doc=tomllib.loads(data.decode());deps=[]
 for ref in doc.get('depends',[]):
  digest=nucleus(ref['path']);assert digest.hex()==ref['sha256'];deps.append((ref['path'],digest))
 return framed(data,b'TEKT-NUCLEUS-DEPS-V1',deps)
consumer='01_core/src/compiler/realize/table_cells.rs'
owner='00_nucleo/prompts/compiler/realize/table_cells.md'
p=R/consumer;q=R/owner
source=re.sub(r'(?m)^//! @updated .*$', '//! @updated 2026-10-09', p.read_text())
spec=q.read_text()
code_hash=hashlib.sha256(re.sub(r'(?m)^//! @prompt-hash [^\n]*\n','',source).encode()).hexdigest()[:8]
spec=re.sub(r'(?m)^Hash do Código: .*$',f'Hash do Código: {code_hash}',spec)
refs=re.findall(r'(?m)^- (00_nucleo/prompts/_nuclei/\S+) sha256:([a-f0-9]{64})$',spec);deps=[]
for path,pin in refs:
 digest=nucleus(path);assert digest.hex()==pin;deps.append((path,digest))
clean=re.sub(r'(?m)^Hash do Código: [^\n]*\n','',spec).encode()
prompt_hash=(framed(clean,b'TEKT-PROMPT-NUCLEI-V1',deps) if deps else hashlib.sha256(clean).digest()).hex()[:8]
source=re.sub(r'(?m)^//! @prompt-hash .*$',f'//! @prompt-hash {prompt_hash}',source)
p.write_text(source);q.write_text(spec)
receipt={'consumer':consumer,'owner':owner,'code_hash':code_hash,'prompt_hash':prompt_hash,'source_sha256':hashlib.sha256(source.encode()).hexdigest(),'l0_sha256':hashlib.sha256(spec.encode()).hexdigest()}
(D/'seal-cell-defaults.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt,indent=2))
