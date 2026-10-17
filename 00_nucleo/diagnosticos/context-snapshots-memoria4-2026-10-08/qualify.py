import datetime,gzip,hashlib,json,re,subprocess,xml.etree.ElementTree as ET
from pathlib import Path
D=Path(__file__).resolve().parent;O=Path('/tmp/context-snapshots-memoria4-2026-10-08');bench=json.loads(gzip.open(D/'bench.json.gz','rt').read());ns={'x':'http://www.w3.org/1999/xhtml'};cases={}
for name in sorted(set(x['fixture'] for x in bench['samples'])):
 f=Path(name+'.typ')
 group={}
 for engine in ['baseline','current']:
  pdf=O/(f.stem+'-'+engine+'.pdf');bbox=subprocess.check_output(['pdftotext','-bbox',str(pdf),'-'],text=True);root=ET.fromstring(bbox);pages=[{'size':page.attrib,'words':[{'text':w.text,'box':w.attrib} for w in page.findall('.//x:word',ns)]} for page in root.findall('.//x:page',ns)];xml=subprocess.check_output(['pdftohtml','-xml','-stdout','-i',str(pdf)],text=True);styles=[{k:re.sub(r'^[A-Z]{6}\+','',v) if k=='family' else v for k,v in x.attrib.items() if k!='id'} for x in ET.fromstring(xml).iter('fontspec')];prefix=O/(f.stem+'-'+engine+'-raster');subprocess.run(['pdftoppm','-r','36',str(pdf),str(prefix)],capture_output=True,check=True);group[engine]={'pages':pages,'text':subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True),'font_styles':styles,'rasters':[hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(O.glob(prefix.name+'-*.ppm'))],'pdffonts':subprocess.check_output(['pdffonts',str(pdf)],text=True),'pdf_sha256':hashlib.sha256(pdf.read_bytes()).hexdigest()}
 def compare(a,b):
  words=lambda x:[w for page in x['pages'] for w in page['words']];wa,wb=words(a),words(b);samewords=[w['text'] for w in wa]==[w['text'] for w in wb];samepages=[p['size'] for p in a['pages']]==[p['size'] for p in b['pages']];delta=max([abs(float(x['box'][k])-float(y['box'][k])) for x,y in zip(wa,wb) for k in ['xMin','xMax','yMin','yMax']],default=0) if samewords else None
  return {'text_equal':a['text']==b['text'],'word_sequence_equal':samewords,'page_dimensions_equal':samepages,'max_word_bbox_delta_pt':delta,'font_styles_equal':a['font_styles']==b['font_styles'],'rasters_equal':a['rasters']==b['rasters']}
 c={'engines':group,'baseline_current':compare(group['baseline'],group['current'])};cases[f.stem]=c;print(f.stem,c['baseline_current'],flush=True)
r={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'bench_sha256':hashlib.sha256((D/'bench.json.gz').read_bytes()).hexdigest(),'binaries':bench['binaries'],'cases':cases}
with gzip.open(D/'qualification.json.gz','wt') as fp:json.dump(r,fp,indent=2)
