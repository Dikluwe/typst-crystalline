from common import *
import tempfile,xml.etree.ElementTree as ET
x=state(__file__);x['cases']={}
def inspect(pdf,prefix):
 text=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True);root=ET.fromstring(subprocess.check_output(['pdftotext','-bbox',str(pdf),'-']));ns={'x':'http://www.w3.org/1999/xhtml'};pages=[]
 for p in root.findall('.//x:page',ns):pages.append({'size':{k:float(v)for k,v in p.attrib.items()},'words':[{'text':w.text,'box':{k:float(v)for k,v in w.attrib.items()}}for w in p.findall('.//x:word',ns)]})
 subprocess.run(['pdftoppm','-r','36','-png',str(pdf),str(prefix)],check=True,capture_output=True)
 return {'pdf_sha256':sha(pdf),'layout_text_sha256':hashlib.sha256(text.encode()).hexdigest(),'pages':pages,'rasters':[sha(p)for p in sorted(prefix.parent.glob(prefix.name+'-*.png'))]}
with tempfile.TemporaryDirectory(prefix='cpu-qualify-')as temp:
 T=Path(temp)
 for f in F:
  rows={}
  for e in B:
   pdf=T/(f.stem+'-'+e+'.pdf');cmd=command(e,f,pdf);p=run(cmd);rows[e]={'command':cmd,'stderr':p.stderr,**inspect(pdf,pdf.with_suffix(''))}
  a,b=rows.values();wa=[w for p in a['pages']for w in p['words']];wb=[w for p in b['pages']for w in p['words']];same=[w['text']for w in wa]==[w['text']for w in wb];delta=max((abs(u['box'][k]-v['box'][k])for u,v in zip(wa,wb)for k in ['xMin','xMax','yMin','yMax']),default=0)if same else None
  samepages=[p['size']for p in a['pages']]==[p['size']for p in b['pages']];comparison={'same_words':same,'same_layout_text':a['layout_text_sha256']==b['layout_text_sha256'],'same_pages':samepages,'bbox_max_delta_pt':delta,'same_raster':a['rasters']==b['rasters'],'pages':{e:len(v['pages'])for e,v in rows.items()},'word_counts':{e:sum(len(p['words'])for p in v['pages'])for e,v in rows.items()}}
  comparison['equivalent_output_control']=same and samepages and delta==0 and comparison['same_raster'];x['cases'][f.stem]={'engines':rows,'comparison':comparison};print(f.stem,comparison,flush=True)
save('qualification.json.gz',x)
