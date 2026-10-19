from pathlib import Path
import json,re,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/tekt-v12-2026-10-16');scope=json.loads((out/'scope.json').read_text());groups=scope['groups'];l0=json.loads((out/'l0-before-code.json').read_text());assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in l0['owners'].items());assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in l0['sources_existing'].items())
before={p:Path(p).read_text() for p in scope['sources'] if Path(p).exists()};after=before.copy();transfers=[]
for g in groups:
 data=before[g['source']].encode()
 for r in sorted(g['items'],key=lambda r:r['start'],reverse=True):
  assert data[r['start']:r['end']].decode()==r['text'];data=data[:r['start']]+data[r['end']:]
 parent=data.decode();anchor=re.search(r'^use ',parent,re.M).start();addition=f'#[path = "../../{g["helper"]}"]\nmod harness;\n\nuse harness::{{'+', '.join(g['exports'])+'};\n\n';parent=parent[:anchor]+addition+parent[anchor:]
 edits=[]
 for name in ['Child','Output']:
  for use in re.findall(r'^use std::process::\{[^\n]+\};\n',parent,re.M):
   rest=parent.replace(use,'');parts=use.removeprefix('use std::process::{').removesuffix('};\n').split(', ')
   if name in parts and not re.search(r'\b'+name+r'\b',rest):
    parts.remove(name);new='use std::process::{'+', '.join(parts)+'};\n';parent=parent.replace(use,new);edits.append(dict(old=use,new=new))
 line='use std::env;\n'
 if line in parent and not re.search(r'\benv::',parent.replace(line,'')):parent=parent.replace(line,'');edits.append(dict(old=line,new=''))
 line='use std::sync::atomic::{AtomicU64, Ordering};\n'
 if line in parent and not re.search(r'\b(?:AtomicU64|Ordering)\b',parent.replace(line,'')):parent=parent.replace(line,'');edits.append(dict(old=line,new=''))
 helper=f'//! Crystalline Lineage\n//! @prompt {g["helper_owner"]}\n//! @prompt-hash 00000000\n//! @layer L3\n//! @updated 2026-10-16\n//! Infraestrutura de teste; compilada apenas pelo harness L4.\n\n'+g['imports']+'\n'
 conversions=[]
 for r in g['items']:
  t=r['text']
  if r['kind'] in ['struct_item','enum_item']:
   t=re.sub(r'^(struct|enum) ',r'pub(super) \1 ',t,count=1,flags=re.M)
   if r['kind']=='struct_item' and r['name']!='Checks':
    t=re.sub(r'^(    )([a-z_]\w*):',r'\1pub(super) \2:',t,flags=re.M)
    t=re.sub(r'(struct \w+)\((PathBuf|Child)\);',r'\1(pub(super) \2);',t)
  elif r['kind']=='impl_item' and not t.startswith('impl Drop for '):t=re.sub(r'^(    )(fn|const) ',r'\1pub(super) \2 ',t,flags=re.M)
  elif r['kind']=='function_item' and r['name'] in g['exports']:t=re.sub(r'^fn ',r'pub(super) fn ',t,count=1)
  conversions.append(dict(original=r,converted=t));helper+=t+'\n\n'
 assert 'use super' not in helper and 'CARGO_BIN_EXE' not in helper and '#[test]' not in helper
 after[g['source']]=parent;after[g['helper']]=helper;transfers.append(dict(source=g['source'],helper=g['helper'],import_insertion=addition,unused_import_edits=edits,items=conversions))
p='04_wiring/src/main.rs';old='enum Published {\n    Html(String),\n    Bundle(Vec<BundleFile>),\n}\n';assert after[p].count(old)==1;after[p]=after[p].replace(old,'').replace('use typst_infra::pipeline::BundleFile;','use typst_infra::watch::Published;')
p='03_infra/src/watch.rs';anchor='#[derive(Clone, Debug, PartialEq, Eq)]\nstruct Fingerprint';assert after[p].count(anchor)==1;enum='/// Successful output prepared for publication by the preview server.\npub enum Published {\n    Html(String),\n    Bundle(Vec<BundleFile>),\n}\n\n';after[p]=after[p].replace(anchor,enum+anchor)
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before=before,unformatted=after,transfers=transfers,published_old=old,published_new=enum)
(out/'materialization.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
for p,t in after.items():
 path=Path(p);path.parent.mkdir(parents=True,exist_ok=True);path.write_text(t)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*scope['sources']],check=True)
print('Published transferido para L3 e 11 módulos privados de harness extraídos; lógica preservada.')
