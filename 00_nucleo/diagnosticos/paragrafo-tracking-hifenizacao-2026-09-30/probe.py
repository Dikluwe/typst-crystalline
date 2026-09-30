from pathlib import Path
import datetime, hashlib, json, shutil, subprocess, xml.etree.ElementTree as ET

r = Path(__file__).resolve().parents[3]
d = Path(__file__).resolve().parent
out = d / 'probe'
out.mkdir(exist_ok=True)
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
run = lambda c: subprocess.check_output(c, text=True).strip()
font = r / '00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/fonts'
originals = {'tekt': r / 'target/release/typst', 'vanilla': r / 'lab/typst-original/target/release/typst'}
bins = {}
for lane, src in originals.items():
    dst = Path('/tmp') / f'tekt-tracking-hyphen-{lane}-{sha(src)[:12]}'
    if not dst.exists():
        shutil.copy2(src, dst)
    bins[lane] = dst
files = [r / f'01_core/src/compiler/layout/paragraph_{name}.rs' for name in ['prepare','select','hyphen','emit','plan']]
prov = dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), head=run(['git','-C',str(r),'rev-parse','HEAD']), status=run(['git','-C',str(r),'status','--porcelain=v1','-uall']), diff_head_stat=run(['git','-C',str(r),'diff','HEAD','--stat']), binary_sha256={k:sha(v) for k,v in bins.items()}, source_sha256={str(p.relative_to(r)):sha(p) for p in files}, font_sha256={str(p.relative_to(r)):sha(p) for p in font.rglob('*') if p.is_file()})
(out/'provenance.json').write_text(json.dumps(prov, ensure_ascii=False, indent=2)+'\n')

long = 'representation extraordinary beautiful acknowledgement characteristic international typography development functionality'
short = 'alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu'
cases = []
for family in ['hyphen','mixed']:
    for width in [100,140]:
        for strategy in ['simple','optimized']:
            for tracking in [0,0.2,-0.2,1]:
                name = f'{family}-w{width}-{strategy}-t{tracking}'
                if family == 'hyphen':
                    body = long
                    policy = 'true'
                    extra = ''
                else:
                    body = 'alpha beta gamma #text(tracking: 1pt)[delta epsilon] zeta eta theta iota kappa lambda mu'
                    policy = 'false'
                    extra = ''
                source = f'#set page(width:{width}pt,height:180pt,margin:8pt)\n#set text(font:"Noto Sans",size:12pt,lang:"en",hyphenate:{policy},tracking:{tracking}pt)\n#set par(justify:true,linebreaks:"{strategy}")\n{body}\n'
                path = out / (name+'.typ')
                path.write_text(source)
                cases.append((name,path))

def observe(pdf):
    xml = pdf.with_suffix('.xml')
    subprocess.run(['pdftotext','-bbox-layout',str(pdf),str(xml)],check=True,capture_output=True)
    tree = ET.parse(xml).getroot()
    pages = tree.findall('.//{*}page')
    words = [{'text':''.join(w.itertext()),'page':i,'x':float(w.attrib['xMin']),'y':float(w.attrib['yMin']),'right':float(w.attrib['xMax']),'bottom':float(w.attrib['yMax'])} for i,page in enumerate(pages) for w in page.findall('.//{*}word')]
    return {'pages':len(pages),'words':words,'text':run(['pdftotext',str(pdf),'-'])}

rows=[]
for name,path in cases:
    row={'case':name,'source_sha256':sha(path),'runs':{}}
    for lane,bin in bins.items():
        pdf=out/(name+'-'+lane+'.pdf')
        command=[str(bin),'compile','--font-path',str(font),str(path),str(pdf)]
        p=subprocess.run(command,text=True,capture_output=True)
        data={'command':command,'exit':p.returncode,'stderr':p.stderr}
        if p.returncode==0:
            data.update(observe(pdf))
            data['pdf_sha256']=sha(pdf)
        row['runs'][lane]=data
    a,b=row['runs']['tekt'],row['runs']['vanilla']
    row['same_status']=a['exit']==b['exit']
    if a['exit']==b['exit']==0:
        row['same_text']=a['text']==b['text']
        row['same_pages']=a['pages']==b['pages']
        row['same_word_sequence']=[w['text'] for w in a['words']]==[w['text'] for w in b['words']]
        row['same_word_page']=[(w['text'],w['page']) for w in a['words']]==[(w['text'],w['page']) for w in b['words']]
        row['max_box_delta_pt']=max((max(abs(x[k]-y[k]) for k in ['x','y','right','bottom']) for x,y in zip(a['words'],b['words'])),default=0) if row['same_word_sequence'] else None
    rows.append(row)
(out/'results.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n')
summary=[{k:v for k,v in row.items() if k!='runs'} for row in rows]
(out/'summary.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2)+'\n')
print('cases',len(rows),'errors',sum(not x['same_status'] for x in rows),'text differences',sum(not x.get('same_text',False) for x in rows),'page differences',sum(not x.get('same_pages',False) for x in rows),'word sequence differences',sum(not x.get('same_word_sequence',False) for x in rows),'box differences > .001pt',sum((x.get('max_box_delta_pt') or 0)>.001 for x in rows))
for row in rows:
    if not row.get('same_text',False) or not row.get('same_word_page',False) or (row.get('max_box_delta_pt') or 0)>.001:
        print(row['case'],row.get('same_text'),row.get('same_word_page'),row.get('max_box_delta_pt'))
assert all(sha(p)==prov['source_sha256'][str(p.relative_to(r))] for p in files)
