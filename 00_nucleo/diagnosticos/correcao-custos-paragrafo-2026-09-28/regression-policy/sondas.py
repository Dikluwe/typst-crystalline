from pathlib import Path
import subprocess,json,hashlib,datetime,xml.etree.ElementTree as ET,re
r=Path.cwd();out=r/'00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28/regression-policy';fonts=out/'fonts';fonts.mkdir(exist_ok=True);font=r/'03_infra/fixtures/fonts/NotoSans-Regular.ttf';(fonts/font.name).write_bytes(font.read_bytes());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
bins={'tekt':r/'target/release/typst','vanilla':r/'lab/typst-original/target/release/typst'}
prov={'utc_start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','status':subprocess.check_output(['git','status','--porcelain=v1','-uall'],text=True),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binary_sha256':{k:sha(p) for k,p in bins.items()},'font_sha256':sha(font),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','source_sha256':{str(p.relative_to(r)):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in (r/top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml'] and 'target' not in p.parts}}
(out/'proveniencia.json').write_text(json.dumps(prov,ensure_ascii=False,indent=2))
cases=[]
def add(name,width,height,settings,body,expected=None):cases.append((name,width,height,settings,body,expected if expected is not None else body))
paragraphs='One short paragraph.\n\nTwo more words here.\n\nThree final words.'
for name,tail in [('terminal',''),('terminal-continuation','\n\nFour additional words.'),('terminal-pagebreak','\n#pagebreak()'),('terminal-pagebreak-content','\n#pagebreak()\nAFTER'),('terminal-two-pagebreaks','\n#pagebreak()\n#pagebreak()'),('terminal-final-parbreak','\n\n')]:
 add(name,140,100,'',paragraphs+tail)
add('empty',140,100,'','')
add('empty-explicit-pagebreak',140,100,'','#pagebreak()')
add('ordinary-continuation',140,60,'','alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu nu xi omicron')
for justify in [False,True]:
 for mode in ['absent','auto','false','true']:
  settings=('#set par(justify: true)\n' if justify else '')+('' if mode=='absent' else f'#set text(hyphenate: {mode})')
  add(f'hyphen-{mode}-justify-{justify}',100,400,settings,'A extraordinary internationalization representation')
for mode in ['auto','false','true']:
 add(f'origin-long-{mode}',80,120,f'#set text(hyphenate: {mode})','internationalization')
add('origin-unknown-language',80,120,'#set text(lang: "zz",hyphenate: true)','internationalization')
add('invalid-hyphen-set',140,120,'#set text(hyphenate: 1)','alpha beta')
add('invalid-breaks-native',140,120,'','#par(linebreaks: "nonsense")[alpha beta]')
add('invalid-justify-native',140,120,'','#par(justify: 1)[alpha beta]')
add('invalid-justify-set',140,120,'#set par(justify: 1)','alpha beta')
add('native-par-boundary',140,400,'','BEFORE #par[MIDDLE] AFTER')
for tracking in [-0.333,0,0.333,3]:
 add(f'tracking-{tracking}',180,400,f'#set text(tracking: {tracking}pt)','office efficient áb AV')
add('terminal-columns',140,100,'',f'#columns(2,gutter: 8pt)[{paragraphs}]')
add('terminal-bottom-float',140,100,'',f'#place(bottom,float: true)[FLOAT]\n{paragraphs}')
add('terminal-footnote',140,100,'',paragraphs+'#footnote[NOTE]')
add('terminal-numbering',140,100,'#set page(numbering: "1")',paragraphs)
add('terminal-odd-break',140,100,'',paragraphs+'\n#pagebreak(to: "odd")')
add('terminal-forced-line',140,100,'',paragraphs+' \\')

add('invalid-hyphen-native',140,120,'','#text(hyphenate: 1)[alpha beta]')
add('invalid-breaks-set',140,120,'#set par(linebreaks: "nonsense")','alpha beta')
add('invalid-breaks-type-set',140,120,'#set par(linebreaks: 1)','alpha beta')
add('invalid-breaks-type-native',140,120,'','#par(linebreaks: 1)[alpha beta]')
add('invalid-hyphen-string',140,120,'#set text(hyphenate: "false")','alpha beta')
add('invalid-justify-auto',140,120,'#set par(justify: auto)','alpha beta')
for mode in ['false','true','auto']:
 add('native-hyphen-'+mode,100,400,'',f'#text(hyphenate: {mode})[A extraordinary internationalization representation]')
 add('native-par-justify-'+mode,100,400,'',f'#par(justify: {mode})[A extraordinary internationalization representation]')
add('nested-hyphen-override',100,400,'#set text(hyphenate: true)','A #text(hyphenate: false)[extraordinary] internationalization representation')
add('showset-hyphen-override',100,400,'#set text(hyphenate: true)\n#show "extraordinary": set text(hyphenate: false)','A extraordinary internationalization representation')
add('nested-justify-override',100,400,'#set par(justify: true)','A #par(justify: false)[extraordinary internationalization representation]')
add('explicit-simple-auto',100,400,'#set par(linebreaks: "simple", justify: true)','A extraordinary internationalization representation')

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
