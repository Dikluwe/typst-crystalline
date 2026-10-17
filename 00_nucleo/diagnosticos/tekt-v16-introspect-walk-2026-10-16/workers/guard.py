from pathlib import Path
import json,gzip,re,subprocess,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');scope=json.load(open(out/'scope.json'));before=json.load(gzip.open(out/'before.json.gz'));material=json.load(open(out/'materialization.json'));receipt=json.load(open(out/'l0-before-code.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
strip_code=lambda s:re.sub(r'^//! @prompt-hash .*\n','',s,count=1,flags=re.M)
strip_owner=lambda s:re.sub(r'^Hash do Código: .*\n','',s,count=1,flags=re.M)
expected=material['expected_before_fmt']
ref=out/'refinement.json'
if ref.exists():
 r=json.load(open(ref));expected.update(r.get('sources',{}))
 for p,s in r.get('owners',{}).items():receipt['owners'].setdefault(p,{})['expected_text']=s
for source in scope['sources']:
 s=subprocess.check_output(['rustfmt','--edition','2021','--config-path',str(Path.cwd()/'rustfmt.toml'),'--emit','stdout'],input=expected[source],text=True)
 assert strip_code(s)==strip_code(Path(source).read_text()),source
for owner in scope['owners']:assert strip_owner(receipt['owners'][owner]['expected_text'])==strip_owner(Path(owner).read_text()),owner
changed=sorted(p for p,h in before['hashes'].items() if sha(p)!=h);assert changed==sorted(scope['sources']+scope['owners']),changed
strip_comments=lambda s:re.sub(r'\s+','',re.sub(r'//[^\n]*','',s))
p='01_core/src/compiler/introspect/locatable.rs';assert strip_comments(before['files'][p])==strip_comments(Path(p).read_text())
(out/'code-guard.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'changed':changed,'exact_prescribed_sources_and_owner_bodies':True,'locatable_executable_unchanged':True,'other_sources_and_config_unchanged':True},indent=2)+'\n');print('Scoped code/L0 guard PASS',flush=True)
