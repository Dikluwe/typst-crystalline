"""Qualify scaled cases at language text/geometry and low-resolution raster levels."""
import datetime, gzip, hashlib, json, os, re, subprocess, tempfile, xml.etree.ElementTree as ET
from pathlib import Path
D=Path(__file__).resolve().parent; R=D.parents[2]; F=D/'fixtures'
FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
B={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
ENV=os.environ.copy()
for key in list(ENV):
    if key.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):ENV.pop(key)
ENV.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def execute(cmd):
    p=subprocess.run(cmd,cwd=R,env=ENV,capture_output=True,text=True,timeout=120)
    if p.returncode:raise RuntimeError((cmd,p.returncode,p.stderr))
    return p

def render(binary,fixture,out,engine):
    pdf=out/(fixture.stem+'-'+engine+'.pdf')
    cmd=[str(binary),'compile',str(fixture),str(pdf),'--root',str(F),'--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]
    if engine=='vanilla':cmd+=['--jobs','1']
    p=execute(cmd)
    layout=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True)
    root=ET.fromstring(subprocess.check_output(['pdftotext','-bbox',str(pdf),'-']))
    ns={'x':'http://www.w3.org/1999/xhtml'}
    pages=[]
    for page in root.findall('.//x:page',ns):
        pages.append(dict(size={k:float(v) for k,v in page.attrib.items()},
                          words=[dict(text=w.text,box={k:float(v) for k,v in w.attrib.items()}) for w in page.findall('.//x:word',ns)]))
    xml=subprocess.check_output(['pdftohtml','-xml','-stdout','-i',str(pdf)],text=True,stderr=subprocess.DEVNULL)
    styles=[{k:re.sub(r'^[A-Z]{6}\+','',v) if k=='family' else v for k,v in e.attrib.items() if k!='id'} for e in ET.fromstring(xml).iter('fontspec')]
    prefix=out/(fixture.stem+'-'+engine+'-raster')
    execute(['pdftoppm','-r','36','-png',str(pdf),str(prefix)])
    rasters=[sha(p) for p in sorted(out.glob(prefix.name+'-*.png'))]
    return dict(command=cmd,stderr=p.stderr,pdf_sha256=sha(pdf),layout_text_sha256=hashlib.sha256(layout.encode()).hexdigest(),
                layout_text_chars=len(layout),pages=pages,font_styles=styles,raster_sha256=rasters)

def compare(a,b):
    wa=[w for p in a['pages'] for w in p['words']]
    wb=[w for p in b['pages'] for w in p['words']]
    samewords=[w['text'] for w in wa]==[w['text'] for w in wb]
    bbox=max((abs(x['box'][k]-y['box'][k]) for x,y in zip(wa,wb) for k in ('xMin','xMax','yMin','yMax')),default=0) if samewords else None
    return dict(same_text=a['layout_text_sha256']==b['layout_text_sha256'],same_words=samewords,
                words=len(wa),same_page_sizes=[p['size'] for p in a['pages']]==[p['size'] for p in b['pages']],
                pages_tekt=len(a['pages']),pages_vanilla=len(b['pages']),
                bbox_max_delta_pt=bbox,same_font_styles=a['font_styles']==b['font_styles'],
                same_raster=a['raster_sha256']==b['raster_sha256'],
                raster_pages_different=sum(x!=y for x,y in zip(a['raster_sha256'],b['raster_sha256'])))

def main():
    status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R)
    stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R)
    (D/'qualification-git-status.txt').write_bytes(status)
    (D/'qualification-git-diff-stat.txt').write_bytes(stat)
    state=dict(utc_start=utc(),head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),working_tree='uncommitted',
               git_status_sha256=hashlib.sha256(status).hexdigest(),git_diff_stat_sha256=hashlib.sha256(stat).hexdigest(),
               binaries={e:sha(p) for e,p in B.items()},fixtures={f.name:sha(f) for f in F.glob('*.typ')},cases={})
    with tempfile.TemporaryDirectory(prefix='item7-quality-') as d:
        out=Path(d)
        for f in sorted(F.glob('*.typ')):
            pair={e:render(binary,f,out,e) for e,binary in B.items()}
            state['cases'][f.stem]={'engines':pair,'compare':compare(pair['tekt'],pair['vanilla'])}
            print(f.stem,state['cases'][f.stem]['compare'],flush=True)
    assert state['binaries']=={e:sha(p) for e,p in B.items()}
    assert state['fixtures']=={f.name:sha(f) for f in F.glob('*.typ')}
    state['utc_end']=utc()
    with gzip.open(D/'qualification.json.gz','wt') as p:json.dump(state,p,ensure_ascii=False,indent=2)
if __name__=='__main__':main()
