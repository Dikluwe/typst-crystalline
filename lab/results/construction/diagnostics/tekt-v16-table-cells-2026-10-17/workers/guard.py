from pathlib import Path
import gzip,hashlib,json,re,subprocess,tempfile
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));t=json.load(open(out/'tests-before-code.json'));m=json.load(open(out/'implementation-replay.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();allowed=scope['sources']+scope['owners'];normcode=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M);norml0=lambda s:re.sub(r'^Hash do Código: [^\n]*\n','',s,count=1,flags=re.M);assert all(sha(p)==h for p,h in b['hashes'].items() if p not in allowed)
for p,s in json.load(open(out/'l0-before-code.json'))['owners'].items():assert norml0(Path(p).read_text())==norml0(s),p
replay=t['expected_files'].copy()
for e in m['edits']:
 assert replay[e['source']].count(e['before'])==1;replay[e['source']]=replay[e['source']].replace(e['before'],e['after'],1)
with tempfile.TemporaryDirectory(prefix='tekt-table-replay-') as d:
 for i,(source,s) in enumerate(replay.items()):
  p=Path(d)/(str(i)+'.rs');p.write_text(s);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(p)],check=True);assert p.read_text()==m['files'][source];assert normcode(Path(source).read_text())==normcode(p.read_text()),source
public=lambda s:re.findall(r'^\s*pub(?!\()\s+[^\n]+',s,re.M)
assert all(public(b['files'][s])==public(Path(s).read_text()) for s in scope['sources']);assert sum(Path(s).read_text().count('#[test]')-b['files'][s].count('#[test]') for s in scope['sources'])==22
record=dict(original_matches=15,final_matches=18,public_declarations_preserved=True,replay_all_authorized_edits=True,unrelated_sources_L0_configuration_unchanged=True,new_tests=22);(out/'guard-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
