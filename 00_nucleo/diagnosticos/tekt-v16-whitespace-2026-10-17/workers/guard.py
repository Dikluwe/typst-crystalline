from pathlib import Path
import gzip,hashlib,json,re,subprocess,tempfile
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));t=json.load(open(out/'tests-before-code.json'));m=json.load(open(out/'materialization.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();allowed=scope['sources']+scope['owners'];normcode=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M);norml0=lambda s:re.sub(r'^Hash do Código: [^\n]*\n','',s,count=1,flags=re.M);assert all(sha(p)==h for p,h in b['hashes'].items() if p not in allowed)
for p,s in json.load(open(out/'l0-before-code.json'))['owners'].items():assert norml0(Path(p).read_text())==norml0(s),p
replay=t['expected_files'].copy()
for source in scope['sources']:
 raw=replay[source].encode();edits=[]
 for r in scope['targets']:
  if r['source']==source:
   a=r['pattern_start_byte'];old=r['pattern_token'].encode();assert raw[a:a+len(old)]==old;edits.append((a,a+len(old),r['replacement'].encode()))
 for a,z,v in sorted(edits,reverse=True):raw=raw[:a]+v+raw[z:]
 replay[source]=raw.decode()
for e in m['functional_edits']:
 assert replay[e['source']].count(e['old'])==e['count'];replay[e['source']]=replay[e['source']].replace(e['old'],e['new'])
with tempfile.TemporaryDirectory(prefix='tekt-whitespace-replay-') as d:
 for i,(source,s) in enumerate(replay.items()):
  s=re.sub(r'^//! @updated [0-9-]+$','//! @updated 2026-10-17',s,count=1,flags=re.M);assert s==m['files'][source]['unformatted'];p=Path(d)/(str(i)+'.rs');p.write_text(s);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(p)],check=True);assert p.read_text()==m['files'][source]['expected_after'];assert normcode(Path(source).read_text())==normcode(p.read_text()),source
public=lambda s:re.findall(r'^\s*pub(?!\()\s+[^\n]+',s,re.M)
assert all(public(b['files'][s])==public(Path(s).read_text()) for s in scope['sources']);assert sum(Path(s).read_text().count('#[test]')-b['files'][s].count('#[test]') for s in scope['sources'])==16
record=dict(original_arms=12,original_matches=11,auxiliary_helper=True,public_declarations_preserved=True,replay_all_authorized_edits=True,unrelated_sources_L0_configuration_unchanged=True,new_tests=16);(out/'guard-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
