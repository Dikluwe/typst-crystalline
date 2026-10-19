from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,os,re,xml.etree.ElementTree as E,shutil
R=Path.cwd();H=R/'00_nucleo/diagnosticos/bidi-contexto-2026-10-05';T=Path('/tmp/tekt-bidi-contexto');H.mkdir(exist_ok=True);T.mkdir(exist_ok=True)
git=lambda *a:subprocess.check_output(['git',*a],text=True);names=git('diff','--name-only').splitlines()+git('ls-files','--others','--exclude-standard').splitlines();names=sorted(set(n for n in names if n.startswith(('01_core/','03_infra/','00_nucleo/prompts/')) and n.endswith(('.rs','.md','.toml'))))
bins={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'};shutil.copy2(bins['tekt'],T/'baseline-typst');common='#set page(width: 220pt, height: 600pt, margin: 20pt)\n#set text(font: "DejaVu Sans")\n'
f={
'rtl_auto_mixed':'#set text(dir: rtl)\nABC 123 مرحبا',
'rtl_override_mixed':'#set text(dir: rtl)\n#text(dir: ltr)[ABC 123] مرحبا',
'rtl_override_prefix':'#set text(dir: rtl)\nمرحبا #text(dir: ltr)[ABC 123]',
'rtl_override_adjacent':'#set text(dir: rtl)\n#text(dir: ltr)[ABC]#text(dir: ltr)[123]',
'rtl_override_one':'#set text(dir: rtl)\n#text(dir: ltr)[ABC123]',
'rtl_override_space':'#set text(dir: rtl)\n#text(dir: ltr)[ABC] #text(dir: ltr)[123]',
'ltr_auto_mixed':'#set text(dir: ltr)\nمرحبا ABC 123',
'ltr_override_mixed':'#set text(dir: ltr)\n#text(dir: rtl)[مرحبا] ABC 123',
'rtl_numeric':'#set text(dir: rtl)\n123 456 مرحبا',
'ltr_numeric':'#set text(dir: ltr)\n123 456 مرحبا',
'rtl_terminal_space':'#set text(dir: rtl)\n#text(dir: ltr)[ABC 123 ] مرحبا',
'rtl_literal_embedding':'#set text(dir: rtl)\n\u202aABC 123\u202c مرحبا',
'rtl_forced_mark':'#set text(dir: rtl)\n\u200fABC 123 مرحبا',
}
f={n:common+s for n,s in f.items()};(H/'fixtures.json').write_text(json.dumps(f,ensure_ascii=False,indent=2));env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1');d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'sources':{n:(R/n).read_text() for n in names},'binaries':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in bins.items()},'cases':{}}
for n,s in f.items():
 fp=T/(n+'.typ');fp.write_text(s);c={'source':s,'engines':{}}
 for k,b in bins.items():
  p=T/(n+'-'+k+'.pdf');cmd=[str(b),'compile',str(fp),str(p),'--pdf-tagged=false','--pretty=false'];run=subprocess.run(cmd,env=env,capture_output=True,text=True);assert run.returncode==0,run.stderr
  xml=subprocess.check_output(['pdftotext','-bbox',str(p),'-'],text=True);words=[{'text':e.text,**e.attrib} for e in E.fromstring(xml).iter() if e.tag.endswith('}word')];c['engines'][k]={'command':cmd,'stderr':run.stderr,'bbox':xml,'text':subprocess.check_output(['pdftotext',str(p),'-'],text=True),'words':words}
 d['cases'][n]=c;print(n,{k:[(w['text'],round(float(w['xMin']),2),round(float(w['xMax']),2)) for w in z['words']] for k,z in c['engines'].items()},flush=True)
gzip.open(H/'initial.json.gz','wt').write(json.dumps(d,ensure_ascii=False));(H/'audit.py').write_text(Path(__file__).read_text())
