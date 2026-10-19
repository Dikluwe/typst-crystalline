"""Read-only packet verifier; mirrors the active linter's framed nucleus hashes."""
from pathlib import Path
import hashlib,json,re,tomllib
R=Path(__file__).resolve().parents[3];P=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def nucleus(path,seen=()):
 assert path not in seen,'cycle'
 p=P/'candidates'/path
 if not p.exists():p=R/path
 data=p.read_bytes();d=tomllib.loads(data.decode());assert d['tekt']==1 and d['kind']=='nucleus'
 h=hashlib.sha256(data);h.update(b'\0TEKT-NUCLEUS-DEPS-V1\0')
 for dep in sorted(d.get('depends',[]),key=lambda x:x['path']):
  digest=nucleus(dep['path'],(*seen,path));assert digest.hex()==dep['sha256'];name=dep['path'].encode();h.update(len(name).to_bytes(8,'big'));h.update(name);h.update(b' ');h.update(digest)
 return h.digest()
if __name__=='__main__':
 m=json.loads((P/'manifest.json').read_text());assert sha(P/'manifest.json')==(P/'manifest.sha256').read_text().split()[0]
 assert len({x['consumer'] for x in m['owners']})==len(m['owners'])==len({x['owner'] for x in m['owners']})
 for row in m['owners']:
  f=R/row['candidate'];assert sha(f)==row['candidate_sha256']
  for field,path in [('base_l0_sha256','owner'),('base_consumer_sha256','consumer')]:
   if row[field]:assert sha(R/row[path])==row[field]
  for path,pin in re.findall(r'^- (00_nucleo/prompts/_nuclei/\S+) sha256:([0-9a-f]{64})$',f.read_text(),re.M):assert nucleus(path).hex()==pin,(f,path)
 for row in m['evidence']:assert sha(R/row['path'])==row['sha256']
 assert sha(R/m['contract']['path'])==m['contract']['sha256']
 for row in m['nuclei']:
  assert sha(R/row['candidate'])==row['candidate_sha256'];assert nucleus(row['path']).hex()==row['effective_sha256']
 print('Packet bases, owners, evidence, DAG and effective nucleus pins verified.')
