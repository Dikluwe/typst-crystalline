from pathlib import Path
import subprocess,json,hashlib,datetime,xml.etree.ElementTree as ET,re
r=Path.cwd();out=r/'00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28/regression-composition';fonts=out/'fonts';fonts.mkdir(exist_ok=True);font=r/'03_infra/fixtures/fonts/NotoSans-Regular.ttf';(fonts/font.name).write_bytes(font.read_bytes());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
bins={'tekt':r/'target/release/typst','vanilla':r/'lab/typst-original/target/release/typst'}
prov={'utc_start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','status':subprocess.check_output(['git','status','--porcelain=v1','-uall'],text=True),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binary_sha256':{k:sha(p) for k,p in bins.items()},'font_sha256':sha(font),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','source_sha256':{str(p.relative_to(r)):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in (r/top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml'] and 'target' not in p.parts}}
(out/'proveniencia.json').write_text(json.dumps(prov,ensure_ascii=False,indent=2))
cases=[]
def add(name,width,height,settings,body,expected=None):cases.append((name,width,height,settings,body,expected if expected is not None else body))
text='alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu'
for width in [140,180]:
 for justify in [False,True]:
  add(f'simple-{width}-{justify}',width,400,f'#set par(justify: {str(justify).lower()},linebreaks: "simple")\n#set text(hyphenate: false)',text)
add('native-boundary',140,400,'','BEFORE #par[MIDDLE] AFTER')
add('native-styled',140,400,'','BEFORE #par(leading: 2pt)[MIDDLE] AFTER')
add('native-empty',140,400,'','BEFORE #par[] AFTER')
add('native-empty-text',140,400,'','BEFORE #par("") AFTER')
add('native-empty-styled',140,400,'','BEFORE #par(leading: 2pt)[] AFTER')
add('native-nested',140,400,'','BEFORE #par[A #par[B] C] AFTER')
add('native-justify',140,400,'#set text(hyphenate: false)',f'BEFORE #par(justify: true)[{text}] AFTER')
add('native-override',140,400,'#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)',f'BEFORE #par(justify: false)[{text}] AFTER')
add('show-par',140,400,'#show par: set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)',text)
add('show-par-boundary',140,400,'#show par: set text(fill: red)','BEFORE #par[MIDDLE] AFTER')
add('show-par-repr',140,400,'#show par: it => [#repr(it)]','BEFORE #par[MIDDLE] AFTER')
add('last-short',180,400,'#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)','alpha beta')
add('last-trailing-text-spaces',180,400,'#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)','#text("alpha beta                         ")')
add('last-trailing-markup',180,400,'#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)','alpha beta    ')
add('parbreak-final',140,400,'#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)','alpha beta\n\ngamma delta')
add('explicit-false',180,400,'#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)','#text("alpha beta")#linebreak(justify: false)gamma delta')
add('explicit-true',180,400,'#set par(justify: false)','#text("alpha beta")#linebreak(justify: true)gamma delta')
add('paragraph-link',140,400,'#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)',f'#link("https://example.com")[{text}]')
add('paragraph-mixed-style',140,400,'#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)','alpha *beta* gamma delta epsilon zeta eta theta iota kappa')
add('paragraph-columns',180,160,'#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)',f'#columns(2,gutter: 8pt)[{text} {text}]')
add('paragraph-pages',140,100,'#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)',text+'\n\n'+text)
add('paragraph-numbering',140,400,'#set par.line(numbering: "1")\n#set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)',text)
add('paragraph-infinite',140,400,'#set page(width: auto)\n#set par(justify: true,linebreaks: "simple")',text)
add('native-morph',140,400,'','#repr(par[hello])')
add('native-leading-two-lines',140,400,'#set text(hyphenate: false)',f'BEFORE #par(leading: 2pt)[{text}] AFTER')

add('native-empty-start',140,400,'','#par[] AFTER')
add('native-empty-end',140,400,'','BEFORE #par[]')
add('native-empty-auto',140,400,'#set page(height: auto)','BEFORE #par[]')
add('native-auto-height',140,400,'#set page(height: auto)','BEFORE #par[MIDDLE] AFTER')
add('show-par-mixed',140,400,'#show par: set par(justify: true,linebreaks: "simple")\n#set text(hyphenate: false)','alpha *beta gamma* delta epsilon zeta eta theta iota kappa')
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
