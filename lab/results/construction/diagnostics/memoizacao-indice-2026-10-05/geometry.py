"""Compare baseline/candidate word boxes and page rasters of the paired fixtures."""
from pathlib import Path
import datetime,gzip,hashlib,json,subprocess,xml.etree.ElementTree as ET
HERE=Path(__file__).resolve().parent
OUT=Path('/tmp/tekt-index-reuse-2026-10-05')
paired=json.load(gzip.open(HERE/'bench-paired.json.gz','rt'))
receipt={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'benchmark':paired['binaries'],'head':paired['head'],'working_tree_hashes':paired['working_tree_hashes'],'cases':{}}
for fixture in paired['fixtures']:
 result={}
 for engine in ['baseline','candidate']:
  pdf=OUT/f'{fixture}-{engine}.pdf'
  bbox=subprocess.check_output(['pdftotext','-bbox',str(pdf),'-'],text=True)
  root=ET.fromstring(bbox); ns={'x':'http://www.w3.org/1999/xhtml'}
  pages=[]
  for page in root.findall('.//x:page',ns):
   pages.append({'dimensions':page.attrib,'words':[{'box':w.attrib,'text':w.text} for w in page.findall('.//x:word',ns)]})
  prefix=OUT/f'{fixture}-{engine}-raster'
  subprocess.run(['pdftoppm','-r','36',str(pdf),str(prefix)],check=True,capture_output=True)
  rasters=sorted(OUT.glob(prefix.name+'-*.ppm'))
  result[engine]={'pages':pages,'raster_sha256':[hashlib.sha256(p.read_bytes()).hexdigest() for p in rasters]}
 result['boxes_equal']=result['baseline']['pages']==result['candidate']['pages']
 result['rasters_equal']=result['baseline']['raster_sha256']==result['candidate']['raster_sha256']
 receipt['cases'][fixture]=result
 print(fixture,'boxes_equal',result['boxes_equal'],'rasters_equal',result['rasters_equal'],'pages',len(result['candidate']['pages']))
receipt['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(HERE/'geometry.json.gz','wt') as f:json.dump(receipt,f,ensure_ascii=False,indent=2)
