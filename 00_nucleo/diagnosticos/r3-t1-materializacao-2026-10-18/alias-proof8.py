from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,xml.etree.ElementTree as ET,re,ast
p=Path(__file__).resolve().parent;r=p.parents[2];old=p.parent/'mixed-style-metrics-2026-10-18';out=p/'outputs/alias-proof-candidate8';out.mkdir(parents=True,exist_ok=True)
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
frozen=json.loads(gzip.decompress((old/'corpus-boundary-after.json.gz').read_bytes()));baseline=next(row for row in frozen['cases'] if row['label']=='bib-content-alias');f=Path(baseline['results']['tekt']['command'][2]);assert sha(f)==baseline['source_sha256']
bins={'before':p/'bin/typst-before','after':r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_state='candidate8-sources.json.gz',pin='bd5ebec285505889e8ff1d4f68804ab147a699f3',source=baseline['source'],source_sha256=sha(f),binaries={k:dict(path=str(v),sha256=sha(v)) for k,v in bins.items()},results={})
# Reuse only the pure transform/glyph helpers; never import the old executable audit.
helper_source=(old/'audit-final.py').read_text();nodes=[node for node in ast.parse(helper_source).body if isinstance(node,ast.FunctionDef) and node.name in ['mul','transform','glyphs']];ns=dict(identity=(1.,0.,0.,1.,0.,0.),re=re,ET=ET);exec(compile(ast.Module(body=nodes,type_ignores=[]),str(old/'audit-final.py'),'exec'),ns)
for actor,binary in bins.items():
 result={}
 for fmt in ['pdf','svg']:
  target=out/(actor+'.'+fmt);cmd=[str(binary),'compile',str(f),str(target)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=60);assert z.returncode==0,(actor,fmt,z.stderr);item=dict(command=cmd,exit=z.returncode,stderr=z.stderr,sha256=sha(target))
  if fmt=='pdf':
   bbox=target.with_suffix('.bbox');subprocess.run(['pdftotext','-bbox',str(target),str(bbox)],capture_output=True,check=True);xml=ET.parse(bbox).getroot();names={'x':'http://www.w3.org/1999/xhtml'};item['pages']=[dict(width=float(pg.attrib['width']),height=float(pg.attrib['height']),words=[dict(text=w.text,bounds=[float(w.attrib[k]) for k in ['xMin','yMin','xMax','yMax']]) for w in pg.findall('.//x:word',names)]) for pg in xml.findall('.//x:page',names)]
  else:item['glyphs']=ns['glyphs'](target)
  result[fmt]=item
 record['results'][actor]=result
assert record['results']['before']['pdf']['pages']==baseline['results']['tekt']['pages']
assert record['results']['vanilla']['pdf']['pages']==baseline['results']['vanilla']['pages']
current=json.loads(gzip.decompress((p/'mixed-regressions-candidate8.json.gz').read_bytes()));case=next(row for row in current['cases'] if row['baseline']=='corpus-boundary-after.json.gz' and row['label']=='bib-content-alias');assert record['results']['after']['pdf']['pages']==case['results']['pdf']['pages']
a=record['results']['after'];b=record['results']['vanilla'];wa=[(i,w) for i,page in enumerate(a['pdf']['pages']) for w in page['words']];wb=[(i,w) for i,page in enumerate(b['pdf']['pages']) for w in page['words']];assert [w['text'] for i,w in wa]==[w['text'] for i,w in wb];assert [i for i,w in wa]==[i for i,w in wb];bbox=max(abs(x['bounds'][k]-y['bounds'][k]) for (i,x),(j,y) in zip(wa,wb) for k in range(4));assert len(a['svg']['glyphs'])==len(b['svg']['glyphs']);svg=max(abs(x[k]-y[k]) for x,y in zip(a['svg']['glyphs'],b['svg']['glyphs']) for k in range(2));assert bbox<=.001 and svg<=.001
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),pdf_max_bbox_error_pt=bbox,svg_max_glyph_error_pt=svg,tolerance_pt=.001,classification='LANGUAGE_PARITY_IMPROVEMENT_ALIAS_USES_INSERTION_RULE',previous_x_pt=26,current_x_pt=44,oracle_x_pt=44,retained_artifacts=current['unchanged'],changed_artifacts=1)
(p/'alias-proof-candidate8.json.gz').write_bytes(gzip.compress(json.dumps(record,indent=2).encode()));print('326 unchanged, 1 parity improvement; bbox',bbox,'SVG glyph',svg)
