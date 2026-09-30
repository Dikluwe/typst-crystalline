from pathlib import Path
import subprocess,json,hashlib,datetime,xml.etree.ElementTree as ET,re
r=Path.cwd();out=r/'00_nucleo/diagnosticos/correcao-seletores-paragrafo-2026-09-28/regression-precedence';fonts=out/'fonts';fonts.mkdir(exist_ok=True);font=r/'03_infra/fixtures/fonts/NotoSans-Regular.ttf';(fonts/font.name).write_bytes(font.read_bytes());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
bins={'tekt':r/'target/release/typst','vanilla':r/'lab/typst-original/target/release/typst'}
prov={'utc_start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','status':subprocess.check_output(['git','status','--porcelain=v1','-uall'],text=True),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binary_sha256':{k:sha(p) for k,p in bins.items()},'font_sha256':sha(font),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','source_sha256':{str(p.relative_to(r)):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in (r/top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml'] and 'target' not in p.parts}}
(out/'proveniencia.json').write_text(json.dumps(prov,ensure_ascii=False,indent=2))
cases=[]
def add(name,settings,body):cases.append((name,100,400,settings,body,body))
words='A extraordinary internationalization representation'
show='#show par: set text(hyphenate: false)'
add('global-set-then-show', '#set text(hyphenate: true)\n'+show, words)
add('native-text-inline-override', '#set text(hyphenate: false)\n'+show, 'A #text(hyphenate: true)[extraordinary internationalization representation]')
add('native-text-only-override', show, '#text(hyphenate: true)['+words+']')
add('mid-par-set-override', '#set text(hyphenate: false)\n'+show, 'A #set text(hyphenate: true)\nextraordinary internationalization representation')
add('show-with-strong', '#set text(hyphenate: true)\n'+show, 'A *extraordinary internationalization* representation')
add('show-with-inline-text', '#set text(hyphenate: true)\n'+show, 'A #text(fill: red)[extraordinary internationalization] representation')
add('scope-two-paragraphs', '#set text(hyphenate: true)\n'+show, words+'\n\n'+words)
add('multiple-global-scopes', '#set text(hyphenate: true)\n#set par(leading: 2pt)\n'+show, words)
rows=[]
for name,w,h,settings,body,expected in cases:
 src=out/f'{name}.typ';src.write_text(f'#set page(width: {w}pt,height: {h}pt,margin: 8pt)\n#set text(font: "Noto Sans",size: 12pt,lang: "en")\n'+settings+'\n'+body+'\n');row={'case':name,'width':w,'height':h,'source_sha256':sha(src),'expected':expected,'runs':{}}
 for lane,binary in bins.items():
  pdf=out/f'{name}-{lane}.pdf';cmd=[str(binary),'compile','--font-path',str(fonts),str(src),str(pdf)];res=subprocess.run(cmd,text=True,capture_output=True);run={'command':cmd,'exit_code':res.returncode,'stderr':res.stderr};(out/f'{name}-{lane}.stderr').write_text(res.stderr)
  if res.returncode==0:
   xml=out/f'{name}-{lane}.xml';subprocess.run(['pdftotext','-bbox-layout',str(pdf),str(xml)],check=True,capture_output=True);doc=ET.parse(xml);pages=[]
   for page in doc.findall('.//{*}page'):
    lines=[]
    for line in page.findall('.//{*}line'):
     words2=[{'text':''.join(z.itertext()),'box':{k:float(v) for k,v in z.attrib.items()}} for z in line.findall('.//{*}word')];lines.append({'text':' '.join(z['text'] for z in words2),'box':{k:float(v) for k,v in line.attrib.items()},'words':words2})
    pages.append({'width':float(page.attrib['width']),'height':float(page.attrib['height']),'lines':lines})
   run.update(pages=pages,pdf_sha256=sha(pdf),line_count=sum(len(p['lines']) for p in pages),page_count=len(pages),text=' '.join(l['text'] for p in pages for l in p['lines']))
  row['runs'][lane]=run
 a=row['runs']['tekt'];b=row['runs']['vanilla'];row['same_line_text']=a.get('pages') is not None and [[l['text'] for l in p['lines']] for p in a['pages']]==[[l['text'] for l in p['lines']] for p in b.get('pages',[])];rows.append(row);print(name,a.get('line_count'),b.get('line_count'),a.get('page_count'),b.get('page_count'),row['same_line_text'],flush=True)
(out/'resultados.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2));print('DONE',len(rows))
