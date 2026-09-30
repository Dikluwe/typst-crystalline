from pathlib import Path
import subprocess,json,hashlib,datetime,xml.etree.ElementTree as ET,re
r=Path.cwd();out=r/'00_nucleo/diagnosticos/correcao-viuvas-orfas-2026-09-28/after-pdf';fonts=out/'fonts';fonts.mkdir(exist_ok=True);font=r/'03_infra/fixtures/fonts/NotoSans-Regular.ttf';(fonts/font.name).write_bytes(font.read_bytes());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
bins={'tekt':r/'target/release/typst','vanilla':r/'lab/typst-original/target/release/typst'}
prov={'utc_start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','status':subprocess.check_output(['git','status','--porcelain=v1','-uall'],text=True),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binary_sha256':{k:sha(p) for k,p in bins.items()},'font_sha256':sha(font),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','source_sha256':{str(p.relative_to(r)):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in (r/top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml'] and 'target' not in p.parts}}
(out/'proveniencia.json').write_text(json.dumps(prov,ensure_ascii=False,indent=2))
cases=[]
def add(name,width,height,settings,body):cases.append((name,width,height,settings,body,body))
short='alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu'
long=' '.join([short]*3)
for height in [55,65,75,85,100,115]:
 for widow,orphan in [('0%','0%'),('100%','0%'),('0%','100%'),('100%','100%')]:
  for prefix in ['', 'Prelude.\n\n']:
   add(f'page-{height}-{widow}-{orphan}-{bool(prefix)}',140,height,f'#set text(hyphenate:false,costs:(widow:{widow},orphan:{orphan}))\n#set par(linebreaks:"simple")',prefix+short)
for height in [45,70,100]:
 for widow,orphan in [('0%','0%'),('100%','100%')]:
  add(f'long-{height}-{widow}-{orphan}',140,height,f'#set text(hyphenate:false,costs:(widow:{widow},orphan:{orphan}))\n#set par(linebreaks:"simple")',long)
for name,setting,body,height in [
 ('default','',short,65),
 ('negative','#set text(costs:(widow:-100%,orphan:-100%))',short,65),
 ('tiny-positive','#set text(costs:(widow:0.01%,orphan:0.01%))',short,65),
 ('large-positive','#set text(costs:(widow:10000%,orphan:10000%))',short,65),
 ('native','',f'#text(costs:(widow:0%,orphan:0%))[{short}]',65),
 ('show-par','#show par: set text(costs:(widow:0%,orphan:0%))',short,65),
 ('inline','#set text(costs:(widow:100%,orphan:100%))',f'alpha #text(costs:(widow:0%,orphan:0%))[beta gamma] delta epsilon zeta eta theta iota kappa lambda mu',65),
 ('empty-fold','#set text(costs:(widow:0%,orphan:0%))\n#set text(costs:(:))',short,65),
 ('restore','#set text(costs:(widow:100%,orphan:100%))',f'#text(costs:(widow:0%,orphan:0%))[{short}]\n\n{short}',65),
 ('terminal-break','',short+'\n#pagebreak()',65),
 ('auto-height','#set page(height:auto)',short,65),
 ('footnote','',f'Prelude#footnote[Note.]\n\n{short}',100),
 ('columns','#set page(columns:2)',long,100),
 ('bold','',f'*{short}*',65),
 ('hyphen','#set text(hyphenate:true)\n#set par(justify:true,linebreaks:"optimized")','representation extraordinary beautiful acknowledgement characteristic international typography development functionality',100),
 ('tracking','#set text(tracking:1pt)',short,65),
 ('mixed-size','',f'alpha #text(size:20pt)[beta gamma] delta epsilon zeta eta theta iota kappa lambda mu',85),
 ('linebreak','', 'one\\\ntwo\\\nthree',55),
 ('two-lines','', 'alpha beta gamma delta epsilon',40),
 ('single-line','', 'alpha beta',30),
 ('empty','', '#par[]',40),
 ('native-par','', f'#par[{short}]',65),
 ('leading','#set par(leading:10pt)',short,85),
 ('spacing','#set par(spacing:20pt)',f'Prelude.\n\n{short}',100),
]:
 add(name,140,height,'#set par(linebreaks:"simple")\n#set text(hyphenate:false)\n'+setting,body)
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
