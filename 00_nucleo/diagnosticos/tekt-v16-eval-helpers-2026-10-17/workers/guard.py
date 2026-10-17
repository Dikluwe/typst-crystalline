from pathlib import Path
import json,gzip,hashlib,re,subprocess,tempfile
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));m=json.load(open(out/'materialization.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();normcode=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M);norml0=lambda s:re.sub(r'^Hash do Código: [^\n]*\n','',s,count=1,flags=re.M)
assert normcode(Path(m['source']).read_text())==normcode(m['expected_after'])
assert norml0(Path(scope['owners'][0]).read_text())==norml0(json.load(open(out/'l0-before-code.json'))['expected_text'])
allowed=scope['sources']+scope['owners'];assert all(sha(p)==h for p,h in b['hashes'].items() if p not in allowed)
# Independent replay from the frozen input, using only the eleven measured pattern spans.
s=b['files'][scope['sources'][0]].encode()
for r in sorted(scope['targets'],key=lambda x:x['pattern_start_byte'],reverse=True):
 a,z=r['pattern_start_byte'],r['pattern_end_byte'];assert s[a:z]==b'_';original=b['files'][scope['sources'][0]].encode();indent=original[original.rfind(b'\n',0,a)+1:a].decode();group=('\n'+indent+'| ').join(scope['variants'][r['enum']][n] for n in r['complement']);s=s[:a]+group.encode()+s[z:]
s=re.sub(rb'^//! @updated [0-9-]+$',b'//! @updated 2026-10-17',s,count=1,flags=re.M)
assert s.decode()==m['unformatted_pattern_only']
with tempfile.TemporaryDirectory(prefix='tekt-eval-replay-') as t:
 p=Path(t)/'tests.rs';p.write_bytes(s);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(p)],check=True);assert p.read_text()==m['expected_after']
record={'only_selected_patterns_and_metadata_changed':True,'unchanged_assertions_bodies_order':True,'unrelated_sources_L0_configuration_unchanged':True,'enum_contracts_unchanged':True};(out/'guard-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
