from pathlib import Path
import concurrent.futures,datetime,hashlib,json,subprocess,xml.etree.ElementTree as ET
r=Path.cwd();d=r/'00_nucleo/diagnosticos/paragrafo-tracking-global-2026-09-29';inputs=d/'inputs';out=d/'after';inputs.mkdir(exist_ok=True);out.mkdir(exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
run=lambda cmd:subprocess.check_output(cmd,text=True).strip()
fonts=r/'00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/fonts'
bins={'tekt':r/'target/release/typst','vanilla':r/'lab/typst-original/target/release/typst'}
status=run(['git','status','--porcelain=v1','-uall']);diff=run(['git','diff','HEAD','--stat'])
provenance={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':run(['git','rev-parse','HEAD']),'working_tree':'não commitada','status':status,'diff_HEAD_stat':diff,'binaries_sha256':{k:sha(v) for k,v in bins.items()},'fonts_sha256':{str(p.relative_to(r)):sha(p) for p in fonts.rglob('*') if p.is_file()},'code_sha256':{s:sha(r/s) for s in ['01_core/src/compiler/layout/paragraph_prepare.rs','01_core/src/compiler/layout/paragraph_plan.rs','01_core/src/compiler/layout/paragraph_select.rs','01_core/src/compiler/layout/paragraph_emit.rs','01_core/src/compiler/layout/paragraph_pagination.rs','01_core/src/compiler/layout/word_layout.rs','01_core/src/compiler/layout/text_measure.rs']}}
(out/'provenance.json').write_text(json.dumps(provenance,ensure_ascii=False,indent=2)+'\n')
words='alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu'
cases=[]
for height in [65,80]:
 for tracking in [0,0.2,-0.2,1]:
  for strategy in ['simple','optimized']:
   for justify in [False,True]:
    name=f'h{height}-t{tracking}-s{strategy}-j{int(justify)}';src=inputs/(name+'.typ')
    src.write_text(f'#set page(width:140pt,height:{height}pt,margin:8pt)\n#set text(font:"Noto Sans",size:12pt,lang:"en",hyphenate:false,tracking:{tracking}pt)\n#set par(linebreaks:"{strategy}",justify:{str(justify).lower()})\n{words}\n')
    cases.append((name,src))
def probe(case):
 name,src=case;row={'case':name,'source_sha256':sha(src),'runs':{}}
 for lane,bin in bins.items():
  pdf=out/(name+'-'+lane+'.pdf');cmd=[str(bin),'compile','--font-path',str(fonts),str(src),str(pdf)];p=subprocess.run(cmd,text=True,capture_output=True)
  info={'command':cmd,'exit':p.returncode,'stderr':p.stderr}
  if p.returncode==0:
   xml=out/(name+'-'+lane+'.xml');subprocess.run(['pdftotext','-bbox-layout',str(pdf),str(xml)],check=True,capture_output=True);tree=ET.parse(xml).getroot();info['pages']=[{k:float(v) for k,v in page.attrib.items()} for page in tree.findall('.//{*}page')];info['words']=[{'text':''.join(w.itertext()),'page':i,'x':float(w.attrib['xMin']),'y':float(w.attrib['yMin']),'right':float(w.attrib['xMax']),'bottom':float(w.attrib['yMax'])} for i,page in enumerate(tree.findall('.//{*}page')) for w in page.findall('.//{*}word')];info['text']=run(['pdftotext',str(pdf),'-']);info['pdf_sha256']=sha(pdf)
  row['runs'][lane]=info
 a=row['runs']['tekt'];b=row['runs']['vanilla'];row['same_status']=a['exit']==b['exit']
 if a['exit']==0 and b['exit']==0:
  row['same_text']=a['text']==b['text'];row['same_pages']=a['pages']==b['pages'];row['same_word_sequence']=[w['text'] for w in a['words']]==[w['text'] for w in b['words']]
  row['max_box_delta_pt']=max((max(abs(x[k]-y[k]) for k in ['x','y','right','bottom']) for x,y in zip(a['words'],b['words'])),default=0) if row['same_word_sequence'] else None
  row['page_distribution']={lane:[sum(w['page']==i for w in rr['words']) for i in range(len(rr['pages']))] for lane,rr in [('tekt',a),('vanilla',b)]}
 return row
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(probe,cases))
(out/'results.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n')
(out/'summary.json').write_text(json.dumps([{k:v for k,v in row.items() if k!='runs'} for row in rows],ensure_ascii=False,indent=2)+'\n')
print('cases',len(rows),'errors',sum(not x['same_status'] for x in rows),'page differences',sum(not x.get('same_pages',False) for x in rows),'word/box differences',sum(not x.get('same_word_sequence',False) or (x.get('max_box_delta_pt') or 0)>.001 for x in rows))
for row in rows:
 if not row.get('same_pages',False) or (row.get('max_box_delta_pt') or 0)>.001:print(row['case'],row.get('page_distribution'),row.get('max_box_delta_pt'))
assert all(sha(r/s)==v for s,v in provenance['code_sha256'].items())
