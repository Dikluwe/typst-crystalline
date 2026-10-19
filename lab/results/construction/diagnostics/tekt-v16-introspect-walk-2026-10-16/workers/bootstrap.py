from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,re
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16';out.mkdir(exist_ok=False);(out/'workers').mkdir();(out/'fixtures').mkdir()
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
last=json.load(gzip.open(root/'00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16/final-state.json.gz'));assert all(sha(p)==h for p,h in last['source_hashes'].items())
sources=['01_core/src/compiler/introspect/walk.rs','01_core/src/compiler/introspect/extract_payload.rs','01_core/src/compiler/introspect/locatable.rs','01_core/src/compiler/layout/realized.rs'];owners=[re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1) for p in sources]
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'hashes':last['source_hashes'],'files':{p:Path(p).read_text() for p in sources+owners},'linter':last['linter'],'binaries':{p:sha(p) for p in ['target/debug/typst','lab/typst-original/target/release/typst']},'parity_pin':'9f2b6e8715237cb086899a42873660fe744622e8'}
assert record['binaries']['lab/typst-original/target/release/typst']=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
with gzip.open(out/'before.json.gz','wt') as f:json.dump(record,f)
for name in ['run.py','seal_owned.py','strict.py','suite.py']:
 s=(root/'00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16/workers'/name).read_text().replace('tekt-v16-introspect-origins-2026-10-16','tekt-v16-introspect-walk-2026-10-16').replace('TEKT_ORIGINS_MEMORY_LIMIT','TEKT_WALK_MEMORY_LIMIT');(out/'workers'/name).write_text(s)
cases={
'direct': '#show outline.entry: it => it.body()\n#outline.entry(1, heading[Direct]) <direct>\n#context [#repr(query(outline.entry).map(it => (it.level, it.element.body))) HEADINGS #query(heading).len()]\n',
'synthesized': '#show outline.entry: it => it.body()\n#outline(title: none)\n= Heading\n#context [#repr(query(outline.entry).map(it => (it.level, it.element.body))) HEADINGS #query(heading).len()]\n',
'mixed': '#show outline.entry: it => it.body()\n#outline(title: none)\n= Heading\n#outline.entry(1, heading[Direct]) <direct>\n#context [#repr(query(outline.entry).map(it => (it.level, it.element.body))) HEADINGS #query(heading).len()]\n'}
for name,s in cases.items():(out/'fixtures'/(name+'.typ')).write_text(s)
print('Snapshot and measurement fixtures prepared; no L0/productive code edits')
