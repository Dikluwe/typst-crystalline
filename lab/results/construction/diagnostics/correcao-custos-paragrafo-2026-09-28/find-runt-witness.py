from pathlib import Path
import subprocess,json,datetime,hashlib,xml.etree.ElementTree as ET
P=Path('00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28');Q=P/'runt-search';Q.mkdir(exist_ok=True);oracle=Path('lab/typst-original/target/release/typst').resolve();font=(P/'before-pdf/fonts').resolve();rows=[]
text='alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu'
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'oracle_sha256':hashlib.sha256(oracle.read_bytes()).hexdigest(),'source_state':'after-pdf/proveniencia.json (unchanged)'}
found=None
for width in range(70,241,10):
 readings=[]
 for ratio in ['0%','1000000%']:
  stem=f'{width}-{ratio}';src=Q/(stem+'.typ');pdf=Q/(stem+'.pdf');xml=Q/(stem+'.xml')
  src.write_text(f'#set page(width:{width}pt,height:500pt,margin:8pt)\n#set text(font:"Noto Sans",size:12pt,hyphenate:false,costs:(runt:{ratio}))\n#set par(justify:false,linebreaks:"optimized")\n{text}\n')
  cmd=[str(oracle),'compile','--font-path',str(font),str(src.resolve()),str(pdf.resolve())];r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,r.stderr
  subprocess.run(['pdftotext','-bbox-layout',str(pdf),str(xml)],check=True,capture_output=True);doc=ET.parse(xml);words=[dict(w.attrib,text=''.join(w.itertext())) for w in doc.findall('.//{*}word')]
  rows.append({'width':width,'ratio':ratio,'command':cmd,'source_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'words':words});readings.append(words)
 if readings[0]!=readings[1]:found=width;break
state.update(measurements=rows,witness_width=found,body=text);(P/'runt-search.json').write_text(json.dumps(state,indent=2)+'\n');print('oracle witness width',found);assert found
old=(P/'after-pdf/sondas.py').read_text();start=old.index("text='representation");end=old.index('rows=[]');new=f"text={text!r}\nfor ratio in ['0%','1000000%']:\n add('runt-witness-'+ratio,{found},500,f'#set text(hyphenate:false,costs:(runt:{{ratio}}))\\n#set par(justify:false,linebreaks:\"optimized\")',text)\n";s=(old[:start]+new+old[end:]).replace('correcao-custos-paragrafo-2026-09-28/after-pdf','correcao-custos-paragrafo-2026-09-28/runt-witness');out=P/'runt-witness';out.mkdir(exist_ok=True);(out/'sondas.py').write_text(s)
