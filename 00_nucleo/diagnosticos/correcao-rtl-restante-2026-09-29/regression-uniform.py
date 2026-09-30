from pathlib import Path
import subprocess,json,hashlib,datetime,xml.etree.ElementTree as ET,concurrent.futures
r=Path.cwd();d=r/'00_nucleo/diagnosticos/correcao-rtl-restante-2026-09-29/uniform';d.mkdir(parents=True,exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
run=lambda args:subprocess.check_output(args,text=True)
bins={'tekt':r/'target/release/typst','compact':r/'target/release/typst','vanilla':r/'lab/typst-original/target/release/typst'}
fonts=r/'00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/fonts'
prov={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':run(['git','rev-parse','HEAD']).strip(),'working_tree':'não commitada','status':run(['git','status','--porcelain=v1','-uall']),'diff_HEAD_stat':run(['git','diff','HEAD','--stat']),'binary_sha256':{k:sha(v) for k,v in bins.items()},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','source_sha256':{str(p.relative_to(r)):sha(p) for top in ['00_nucleo/prompts','01_core','02_shell','03_infra','04_wiring'] for p in (r/top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml'] and 'target' not in p.parts}}
(d/'initial.json').write_text(json.dumps(prov,ensure_ascii=False,indent=2))
contexts={'single':'#text("representation")','markup':'representation end','literal-one':'#text("representation end")','literal-two':'#text("representation  end")','literal-three':'#text("representation   end")','adjacent':'#text[representation]#text[end]'}
cases=[(name,t,body) for name,body in contexts.items() for t in [0,.2,-.2,.333,1.2,-1.2]]
def probe(case):
 name,t,body=case;id=name+'-'+str(t);src=d/(id+'.typ');src.write_text(f'#set page(width:760pt,height:400pt,margin:8pt)\n#set text(font:"Noto Sans",size:12pt,lang:"en",hyphenate:false,tracking:{t}pt)\n'+body+'\n')
 row={'case':id,'context':name,'tracking_pt':t,'source_sha256':sha(src),'runs':{}}
 for lane,bin in bins.items():
  pdf=d/(id+'-'+lane+'.pdf');cmd=[str(bin),'compile','--font-path',str(fonts),*(['--compact'] if lane=='compact' else []),str(src),str(pdf)];res=subprocess.run(cmd,text=True,capture_output=True);assert res.returncode==0,(cmd,res.stderr)
  trace=pdf.with_suffix('.trace.xml');subprocess.run(['mutool','draw','-q','-F','trace','-o',str(trace),str(pdf)],check=True,capture_output=True)
  glyphs=[]
  for paint in ET.parse(trace).getroot().iter('fill_text'):
   a,b,c,e,tx,ty=map(float,paint.attrib.get('transform','1 0 0 1 0 0').split())
   for span in paint.findall('span'):
    for g in span.findall('g'):
     if 'glyph' not in g.attrib:continue
     x=float(g.attrib['x']);y=float(g.attrib['y']);glyphs.append({'unicode':g.attrib.get('unicode'),'glyph':g.attrib['glyph'],'x':a*x+c*y+tx,'y':b*x+e*y+ty})
  row['runs'][lane]={'command':cmd,'exit':res.returncode,'stderr':res.stderr,'pdf_sha256':sha(pdf),'glyphs':glyphs,'text':run(['pdftotext','-enc','UTF-8',str(pdf),'-'])}
 return row
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(probe,cases))
(d/'results.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2));(d/'runner.py.txt').write_text(Path('/tmp/rtl-tracking-uniform.py').read_text())

summary=[]
for row in rows:
 ink=lambda lane:[g for g in row['runs'][lane]['glyphs'] if g['unicode']!=' ']
 a=ink('tekt');v=ink('vanilla');c=ink('compact')
 same=[g['unicode'] for g in a]==[g['unicode'] for g in v]
 error=max((abs(x['x']-y['x']) for x,y in zip(a,v)),default=0) if same else None
 mode=max((abs(x['x']-y['x']) for x,y in zip(a,c)),default=0)
 summary.append({'case':row['case'],'same_ink_unicode':same,'max_ink_origin_error_pt':error,'mode_error_pt':mode})
 assert same and error<.001 and mode<.001,(row['case'],error,mode)
(d/'summary.json').write_text(json.dumps(summary,indent=2));print('36 controls max vanilla:',max(x['max_ink_origin_error_pt'] for x in summary));assert all(sha(r/k)==v for k,v in prov['source_sha256'].items());(d/'completed.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_unchanged':True,'cases':len(rows),'pdf_runs':len(rows)*len(bins),'results_sha256':sha(d/'results.json')},indent=2))
