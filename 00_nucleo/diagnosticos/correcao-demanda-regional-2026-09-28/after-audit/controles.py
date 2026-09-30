from pathlib import Path
import subprocess,json,hashlib,datetime,xml.etree.ElementTree as ET,re
r=Path.cwd();out=r/'00_nucleo/diagnosticos/correcao-demanda-regional-2026-09-28/after-audit';fonts=out/'fonts';fonts.mkdir(exist_ok=True);font=r/'03_infra/fixtures/fonts/NotoSans-Regular.ttf';(fonts/font.name).write_bytes(font.read_bytes());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
bins={'tekt':r/'target/release/typst','vanilla':r/'lab/typst-original/target/release/typst'}
prov={'utc_start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','status':subprocess.check_output(['git','status','--porcelain=v1','-uall'],text=True),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binary_sha256':{k:sha(p) for k,p in bins.items()},'font_sha256':sha(font),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','source_sha256':{str(p.relative_to(r)):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in (r/top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml'] and 'target' not in p.parts}}
(out/'proveniencia.json').write_text(json.dumps(prov,ensure_ascii=False,indent=2))
latin='The quick brown fox jumps over the lazy dog. A careful reader considers the whole paragraph before choosing a good line break.'
words='extraordinary internationalization representation development understanding typography'
cases=[]
def add(name,width,height,settings,body,expected=None):cases.append((name,width,height,settings,body,expected if expected is not None else body))
for width in [100,140,180]:
 for mode in ['simple','optimized']:
  add(f'break-{width}-{mode}',width,400,f'#set par(linebreaks: "{mode}")\n#set text(hyphenate: false)',latin)
  add(f'justify-{width}-{mode}',width,400,f'#set par(justify: true, linebreaks: "{mode}")\n#set text(hyphenate: false)',latin)
for setting in ['auto','false','true']:
 add(f'hyphen-{setting}',100,400,f'#set text(hyphenate: {setting})','A '+words)
add('long-word-alone',80,120,'#set text(hyphenate: true)','internationalization')
add('long-word-prefixed',80,120,'#set text(hyphenate: true)','A internationalization')
add('native-par-justify',140,400,'#set text(hyphenate: false)',f'#par(justify: true)[{latin}]',latin)
add('set-par-justify',140,400,'#set par(justify: true)\n#set text(hyphenate: false)',latin)
add('native-par-boundary',140,400,'','BEFORE #par[MIDDLE] AFTER','BEFORE MIDDLE AFTER')
for tracking in [0,1,3]:
 add(f'tracking-office-{tracking}',120,400,f'#set text(tracking: {tracking}pt, hyphenate: false)','office efficient affinity official office efficient affinity')
for height in [60,80,100]:
 add(f'pagination-{height}',140,height,'#set text(hyphenate: false)',(latin+' ')*4)
add('mixed-size',140,100,'#set text(hyphenate: false)',('alpha #text(size: 22pt)[BETA] gamma delta epsilon '*8),('alpha BETA gamma delta epsilon '*8))
add('paragraphs',140,100,'', '\n\n'.join(['One short paragraph.','Two more words here.','Three final words.']),'One short paragraph. Two more words here. Three final words.')
add('forced-line',140,100,'',r'alpha \ beta gamma','alpha beta gamma')
add('tracking-combining',120,400,'#set text(tracking: 3pt,hyphenate: false)','a\u0301b a\u0301b a\u0301b a\u0301b a\u0301b')
short='The old fox and the big dog run by the sea. The sun is low and the sky is red. We can see the bay and the old oak.'
for width in [85,100,115,130]:
 for mode in ['simple','optimized']:
  add(f'short-{width}-{mode}',width,400,f'#set text(lang: "zz",hyphenate: false)\n#set par(justify: true,linebreaks: "{mode}")',short)
for name,w,h,setting,body,expected in list(cases):
 if name in ['mixed-size','paragraphs']:
  add(name+'-no-final-newline',w,h,setting,body,expected)
add('invalid-linebreaks',140,200,'#set par(linebreaks: "nonsense")','A B C')
add('unknown-lang-longword',80,120,'#set text(lang: "zz",hyphenate: false)','internationalization')
cases=cases[30:]
rows=[]
for name,w,h,settings,body,expected in cases:
 src=out/f'{name}.typ';src.write_text(f'#set page(width: {w}pt,height: {h}pt,margin: 8pt)\n#set text(font: "Noto Sans",size: 12pt,lang: "en")\n'+settings+'\n'+body+('' if name.endswith('no-final-newline') else '\n'));row={'case':name,'width':w,'height':h,'source_sha256':sha(src),'expected':expected,'runs':{}}
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
(out/'resultados-controles.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2));print('DONE',len(rows))
