from pathlib import Path
import concurrent.futures,datetime,hashlib,json,subprocess,xml.etree.ElementTree as ET
r=Path.cwd();d=r/'00_nucleo/diagnosticos/paragrafo-tracking-global-2026-09-29';base=(r/'00_nucleo/diagnosticos/correcao-viuvas-orfas-2026-09-28/after-pdf/tracking.typ').read_text();inputs=d/'focused-inputs';out=d/'focused-after';inputs.mkdir(exist_ok=True);out.mkdir(exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();fonts=r/'00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/fonts';bins={'tekt':r/'target/release/typst','vanilla':r/'lab/typst-original/target/release/typst'}
rows=[]
for t in [0,0.2,-0.2,1]:
 for strategy in ['simple','optimized']:
  source=base.replace('linebreaks:"simple"',f'linebreaks:"{strategy}"').replace('tracking:1pt',f'tracking:{t}pt');name=f't{t}-{strategy}';p=inputs/(name+'.typ');p.write_text(source);rows.append((name,p))
def probe(case):
 name,src=case;row={'case':name,'source_sha256':sha(src),'runs':{}}
 for lane,binary in bins.items():
  pdf=out/(name+'-'+lane+'.pdf');cmd=[str(binary),'compile','--font-path',str(fonts),str(src),str(pdf)];res=subprocess.run(cmd,capture_output=True,text=True);assert res.returncode==0,(name,lane,res.stderr);xml=pdf.with_suffix('.xml');subprocess.run(['pdftotext','-bbox-layout',str(pdf),str(xml)],check=True,capture_output=True);tree=ET.parse(xml).getroot();pages=tree.findall('.//{*}page');words=[{'page':i,'text':''.join(w.itertext()),'x':float(w.attrib['xMin']),'y':float(w.attrib['yMin']),'right':float(w.attrib['xMax']),'bottom':float(w.attrib['yMax'])} for i,page in enumerate(pages) for w in page.findall('.//{*}word')];row['runs'][lane]={'command':cmd,'pdf_sha256':sha(pdf),'stderr':res.stderr,'pages':[p.attrib for p in pages],'words':words,'text':subprocess.check_output(['pdftotext',str(pdf),'-'],text=True)}
 a=row['runs']['tekt'];b=row['runs']['vanilla'];row['same_text']=a['text']==b['text'];row['same_words']=[w['text'] for w in a['words']]==[w['text'] for w in b['words']];row['distribution']={k:[sum(w['page']==i for w in v['words']) for i in range(len(v['pages']))] for k,v in row['runs'].items()};row['max_box_delta_pt']=max(max(abs(x[q]-y[q]) for q in ['x','y','right','bottom']) for x,y in zip(a['words'],b['words'])) if row['same_words'] else None
 return row
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:results=list(pool.map(probe,rows))
(out/'results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n');(out/'summary.json').write_text(json.dumps([{k:v for k,v in x.items() if k!='runs'} for x in results],ensure_ascii=False,indent=2)+'\n')
for x in results: print(x['case'],x['distribution'],x['max_box_delta_pt'])
