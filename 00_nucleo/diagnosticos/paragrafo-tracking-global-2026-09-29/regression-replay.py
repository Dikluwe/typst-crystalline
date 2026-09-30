from pathlib import Path
import json,subprocess,hashlib,datetime,xml.etree.ElementTree as ET,concurrent.futures,re
r=Path.cwd();d=r/'00_nucleo/diagnosticos/paragrafo-tracking-global-2026-09-29';out=d/'regressions-after';out.mkdir(exist_ok=True);old=r/'00_nucleo/diagnosticos/correcao-auto-espacos-2026-09-29/regressions/results.json';baseline=json.loads(old.read_text())['rows'];sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();binary=r/'target/release/typst';EPS=.001
prov={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','status':subprocess.check_output(['git','status','--porcelain=v1','-uall'],text=True),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binary_sha256':sha(binary),'baseline_sha256':sha(old),'baseline_record':str(old.relative_to(r))};(out/'provenance.json').write_text(json.dumps(prov,ensure_ascii=False,indent=2)+'\n')
def flatten(pages):return [{'width':p['width'],'height':p['height'],'words':sorted([w for line in p['lines'] for w in line['words']],key=lambda w:(round(w['box']['yMin'],3),w['box']['xMin'],w['text']))} for p in pages]
def equal(a,b):
 if isinstance(a,dict):return isinstance(b,dict) and a.keys()==b.keys() and all(equal(a[k],b[k]) for k in a)
 if isinstance(a,list):return isinstance(b,list) and len(a)==len(b) and all(equal(x,y) for x,y in zip(a,b))
 if isinstance(a,(float,int)):return isinstance(b,(float,int)) and abs(a-b)<=EPS
 return a==b
def error(s):
 m=re.search(r'(?m)^error:',s);return re.sub(r'(?m)(┌─ |--> )[^\n]*?\.typ:',r'\1<input>.typ:',s[m.start():]) if m else ''
def probe(row):
 oldrun=row['run'];cmd=oldrun['command'][:];src=Path(cmd[-2]);assert sha(src)==row['source_sha256'];pdf=out/(row['case'].replace('/','--')+'.pdf');cmd[0]=str(binary);cmd[-1]=str(pdf);res=subprocess.run(cmd,text=True,capture_output=True);now={'exit_code':res.returncode,'command':cmd,'stderr':res.stderr};checks={'same_input':True,'compile_status':res.returncode==oldrun['exit_code']}
 if not res.returncode:
  xml=pdf.with_suffix('.xml');subprocess.run(['pdftotext','-bbox-layout',str(pdf),str(xml)],check=True,capture_output=True);tree=ET.parse(xml);pages=[{'width':float(p.attrib['width']),'height':float(p.attrib['height']),'lines':[{'words':[{'text':''.join(w.itertext()),'box':{k:float(v) for k,v in w.attrib.items()}} for w in line.findall('.//{*}word')]} for line in p.findall('.//{*}line')]} for p in tree.findall('.//{*}page')];now['pages']=pages;checks['geometry_and_text_preserved']=equal(flatten(pages),flatten(oldrun['pages']))
 else:checks['error_preserved']=error(res.stderr)==error(oldrun['stderr'])
 return {'case':row['case'],'source':str(src),'source_sha256':sha(src),'run':now,'checks':checks}
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(probe,baseline))
receipt={'rows':rows,'inputs':len(rows),'tolerance_pt':EPS,'failed':[{'case':row['case'],'checks':[k for k,v in row['checks'].items() if not v]} for row in rows if not all(row['checks'].values())]};(out/'results.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n');print({k:receipt[k] for k in ['inputs','tolerance_pt','failed']})
