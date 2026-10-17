from pathlib import Path
import json,gzip,re,subprocess,tempfile,datetime,hashlib
out=Path('00_nucleo/diagnosticos/tekt-v12-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));scope=json.loads((out/'scope.json').read_text());m=json.loads((out/'materialization.json').read_text());l0=json.loads((out/'l0-before-code.json').read_text());strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M);sha=lambda s:hashlib.sha256(s.encode()).hexdigest()
assert len(m['transfers'])==11
for p,t in m['before'].items():assert t==before['files'][p],p
replay=m['before'].copy()
for tr,g in zip(m['transfers'],scope['groups']):
 assert tr['source']==g['source'] and tr['helper']==g['helper'];data=m['before'][g['source']].encode()
 for r in sorted(g['items'],key=lambda r:r['start'],reverse=True):
  assert data[r['start']:r['end']].decode()==r['text'];assert '#[test]' not in r['text'];data=data[:r['start']]+data[r['end']:]
 parent=data.decode();a=re.search(r'^use ',parent,re.M).start();parent=parent[:a]+tr['import_insertion']+parent[a:]
 for e in tr['unused_import_edits']:
  assert e['old'].startswith('use std::') and (not e['new'] or e['new'].startswith('use std::'));assert parent.count(e['old'])==1;parent=parent.replace(e['old'],e['new'])
 replay[g['source']]=parent
 helper=f'//! Crystalline Lineage\n//! @prompt {g["helper_owner"]}\n//! @prompt-hash 00000000\n//! @layer L3\n//! @updated 2026-10-16\n//! Infraestrutura de teste; compilada apenas pelo harness L4.\n\n'+g['imports']+'\n'
 for item in tr['items']:
  orig=item['original'];assert orig in g['items'];converted=item['converted'];assert converted.replace('pub(super) ','')==orig['text'],(g['source'],orig['name']);helper+=converted+'\n\n'
 assert 'use super' not in helper and 'CARGO_BIN_EXE' not in helper and '#[test]' not in helper
 replay[g['helper']]=helper
p='04_wiring/src/main.rs';assert m['published_old']=='enum Published {\n    Html(String),\n    Bundle(Vec<BundleFile>),\n}\n';replay[p]=replay[p].replace(m['published_old'],'').replace('use typst_infra::pipeline::BundleFile;','use typst_infra::watch::Published;')
p='03_infra/src/watch.rs';new=m['published_new'];assert new=='/// Successful output prepared for publication by the preview server.\npub enum Published {\n    Html(String),\n    Bundle(Vec<BundleFile>),\n}\n\n';anchor='#[derive(Clone, Debug, PartialEq, Eq)]\nstruct Fingerprint';replay[p]=replay[p].replace(anchor,new+anchor);assert replay==m['unformatted']
with tempfile.TemporaryDirectory(prefix='tekt-v12-replay-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'unit{i}.rs';f.write_text(t);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True);assert strip(f.read_text())==strip(Path(p).read_text()),p
for p,h in l0['owners'].items():
 prior=before['files'].get(p,'Hash do Código: 00000000\n');oldhash=re.search(r'^Hash do Código: (.+)$',prior,re.M)[1];text=re.sub(r'^Hash do Código: .+$','Hash do Código: '+oldhash,Path(p).read_text(),count=1,flags=re.M);assert sha(text)==h,p
paths={str(p) for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}|set(p for p in before['files'] if not p.startswith(('00_nucleo/','01_core/','02_shell/','03_infra/','04_wiring/','benches/')))
newpaths=paths-set(before['files']);assert newpaths==set(scope['new_sources']+scope['new_owners']),newpaths
changed={p for p,t in before['files'].items() if Path(p).read_text()!=t};assert changed|newpaths==set(scope['sources']+scope['owners']),(changed|newpaths)^set(scope['sources']+scope['owners'])
assert Path('03_infra/src/lib.rs').read_text()==before['files']['03_infra/src/lib.rs'];assert all(Path(g['helper']).parent==Path('03_infra/tests/support/wiring') for g in scope['groups'])
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_replay=True,helpers_only_visibility_changes=True,test_cases_fixtures_assertions_and_expected_results_preserved=True,published_only_approved_contract_and_import=True,product_library_excludes_test_support=True,no_upward_import_from_helpers=True,l0_body_unchanged_since_prescription=True,unscoped_sources_and_configuration_unchanged=True,consumers=26,owners=26,new_test_only_modules=11,matched_v12=19)
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
