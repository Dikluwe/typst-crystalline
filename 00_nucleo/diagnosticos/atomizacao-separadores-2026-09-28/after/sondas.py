import json,subprocess,hashlib,datetime,re,xml.etree.ElementTree as ET,shutil
from pathlib import Path
root=Path.cwd();out=root/'00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28/after';fonts=out/'fonts';fonts.mkdir(exist_ok=True);font=root/'03_infra/fixtures/fonts/NotoSans-Regular.ttf';shutil.copyfile(font,fonts/font.name)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
bins={'tekt':root/'target/release/typst','vanilla':root/'lab/typst-original/target/release/typst'}
prov={'utc_start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1'],text=True),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binary_sha256':{k:sha(p) for k,p in bins.items()},'font':{'path':str(font),'sha256':sha(font)},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','vanilla_tree':'6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4','tekt_build_command':'cargo build --release --offline -p typst-wiring --bin typst','tekt_build_exit_code':0,'source_sha256':{str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts','00_nucleo/adr'] for p in (root/top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml'] and 'target' not in p.parts}}
header='#set page(width: 300pt, height: 300pt, margin: 20pt)\n#set text(font: "Noto Sans", size: 12pt, lang: "en")\n'
cases={'01-ascii':('Hello world.','Hello world.'),'02-unicode':('Olá mundo! Ação, coração e café.','Olá mundo! Ação, coração e café.'),'03-newline':('Olá\nmundo','Olá mundo'),'04-paragraph':('Primeiro parágrafo.\n\nSegundo parágrafo.','Primeiro parágrafo. Segundo parágrafo.'),'05-inline':('a#("b")c e Olá #("mundo")!','abc e Olá mundo!'),'06-pdf-literal':('#text("Parênteses (a) e barra \\\\.")','Parênteses (a) e barra \\.'),'07-spaces':('A  B   C','A B C'),'08-wrap':('Palavra curta e palavras repetidas. '*14,('Palavra curta e palavras repetidas. '*14).strip()),'09-long-word':('Supercalifragilisticexpialidocious','Supercalifragilisticexpialidocious'),'10-wide-narrow':('iiiiiiii WWWWWWWW iiiiiiii WWWWWWWW','iiiiiiii WWWWWWWW iiiiiiii WWWWWWWW')}
cases.update({
 '11-single-spaces':('A B C','A B C'),
 '12-adjacent-control':('A#("B")C','ABC'),
 '13-text-literal':('#text("A B C")','A B C'),
 '14-long-controls':('ALPHA BETA GAMMA','ALPHA BETA GAMMA'),
 '15-group':('#scale(x: 120%)[A B C]','A B C'),
 '16-link':('#link("https://example.test")[A B C]','A B C'),
 '17-justify':('#set par(justify: true)\n'+('A B C D E F G '*8),('A B C D E F G '*8).strip()),
 '18-styles':('A *B C* D _E F_ G','A B C D E F G'),
 '19-leading-trailing':('#text("  A B  ")','A B'),
 '20-baseline':('A #super[A B] C','A A B C'),
 '21-rtl-control':('#set text(dir: rtl)\nA B C','A B C'),
 '22-tracking':('#set text(tracking: 1pt)\nA B C','A B C'),
 '23-kerning':('AV To WA','AV To WA'),
 '24-regions':('#set page(width: 80pt, height: 70pt, margin: 8pt)\n'+('A B C D E F '*16),('A B C D E F '*16).strip()),
})
results=[]
for name,(body,expected) in cases.items():
 source=out/(name+'.typ');source.write_text(header+body+'\n');entry={'case':name,'source_sha256':sha(source),'expected_text':expected,'runs':{}}
 for lane,binary in bins.items():
  pdf=out/(name+'-'+lane+'.pdf');args=[str(binary),'compile','--font-path',str(fonts),str(source),str(pdf)];r=subprocess.run(args,capture_output=True,text=True);(out/(name+'-'+lane+'.stderr')).write_text(r.stderr);run={'command':args,'exit_code':r.returncode,'stderr':r.stderr}
  if r.returncode==0 and pdf.exists():
   xml=out/(name+'-'+lane+'.xml');xr=subprocess.run(['pdftotext','-bbox-layout',str(pdf),str(xml)],capture_output=True,text=True);run['pdftotext_exit_code']=xr.returncode;run['pdf_sha256']=sha(pdf)
   if xr.returncode==0:
    tree=ET.parse(xml);pages=tree.findall('.//{*}page');words=[{'text':''.join(w.itertext()),**{k:float(w.attrib[k]) for k in ['xMin','xMax','yMin','yMax']}} for w in tree.findall('.//{*}word')];text=' '.join(w['text'] for w in words);run.update({'pages':len(pages),'words':words,'extracted_text':text,'expected_text_matches':re.sub(r'\s+',' ',text).strip()==re.sub(r'\s+',' ',expected).strip(),'lines':len(tree.findall('.//{*}line'))})
  entry['runs'][lane]=run
 results.append(entry);print(name,[(k,x.get('expected_text_matches'),x.get('pages'),x.get('lines')) for k,x in entry['runs'].items()],flush=True)
prov['utc_end']=datetime.datetime.now(datetime.timezone.utc).isoformat();(out/'proveniencia.json').write_text(json.dumps(prov,ensure_ascii=False,indent=2)+'\n');(out/'resultados.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
