from pathlib import Path
import subprocess,json,hashlib,datetime,xml.etree.ElementTree as ET,concurrent.futures,sys
r=Path.cwd();d=r/'00_nucleo/diagnosticos/correcao-tracking-rtl-2026-09-29';stage=sys.argv[1];out=r/'00_nucleo/diagnosticos/correcao-rtl-restante-2026-09-29'/stage;out.mkdir(exist_ok=True);sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();fonts=r/'00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/fonts';binary=Path('/tmp/typst-before-tracking-scopes') if stage=='before-pdf' else r/'target/release/typst';bins={'tekt':binary,'compact':binary,'vanilla':r/'lab/typst-original/target/release/typst'}
prov={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--porcelain=v1','-uall'],text=True),'binary_sha256':{k:sha(p) for k,p in bins.items()},'source_sha256':{str(p.relative_to(r)):sha(p) for top in ['00_nucleo/prompts','01_core','02_shell','03_infra','04_wiring'] for p in (r/top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml'] and 'target' not in p.parts},'fonts_sha256':{str(p.relative_to(r)):sha(p) for p in fonts.rglob('*') if p.is_file()}}
(out/'provenance.json').write_text(json.dumps(prov,ensure_ascii=False,indent=2)+'\n')
def probe(src):
 row={'case':src.stem,'source_sha256':sha(src),'runs':{}}
 for lane,bin in bins.items():
  pdf=out/(src.stem+'-'+lane+'.pdf');cmd=[str(bin),'compile','--font-path',str(fonts),*(['--compact'] if lane=='compact' else []),str(src),str(pdf)];res=subprocess.run(cmd,text=True,capture_output=True);assert res.returncode==0,(cmd,res.stderr);trace=pdf.with_suffix('.xml');subprocess.run(['mutool','draw','-q','-F','trace','-o',str(trace),str(pdf)],check=True,capture_output=True);tree=ET.parse(trace).getroot();glyphs=[]
  for page_no,page in enumerate(tree.iter('page')):
   for paint in page.iter('fill_text'):
    a,b,c,e,tx,ty=map(float,paint.attrib.get('transform','1 0 0 1 0 0').split())
    for span in paint.findall('span'):
     for g in span.findall('g'):
      if 'glyph' not in g.attrib:continue
      x=float(g.attrib['x']);y=float(g.attrib['y']);glyphs.append({'unicode':g.attrib.get('unicode'),'x':a*x+c*y+tx,'y':b*x+e*y+ty,'page':page_no})
  row['runs'][lane]={'command':cmd,'exit':res.returncode,'stderr':res.stderr,'pdf_sha256':sha(pdf),'glyphs':glyphs,'pages':[p.attrib for p in tree.iter('page')],'text':subprocess.check_output(['pdftotext',str(pdf),'-'],text=True)}
 def ink(lane):return [g for g in row['runs'][lane]['glyphs'] if g['unicode']!=' ']
 a=ink('tekt');v=ink('vanilla');same=[g['unicode'] for g in a]==[g['unicode'] for g in v];row['same_ink_unicode']=same;row['max_origin_delta_pt']=max((max(abs(g['x']-h['x']),abs(g['y']-h['y'])) for g,h in zip(a,v)),default=0) if same else None;row['same_ink_pages']=same and [g['page'] for g in a]==[g['page'] for g in v]
 return row
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(probe,sorted((r/'00_nucleo/diagnosticos/correcao-tracking-escopos-2026-09-29/inputs').glob('*.typ'))))
(out/'results.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n');assert all(sha(r/p)==v for p,v in prov['source_sha256'].items());(out/'summary.json').write_text(json.dumps([{k:row[k] for k in ['case','same_ink_unicode','same_ink_pages','max_origin_delta_pt']} for row in rows],indent=2)+'\n');print([(x['case'],x['max_origin_delta_pt']) for x in rows if not x['same_ink_unicode'] or x['max_origin_delta_pt']>.001]);(out/'probe.py.txt').write_text(Path('/tmp/rtl-tracking-mixed.py').read_text())
