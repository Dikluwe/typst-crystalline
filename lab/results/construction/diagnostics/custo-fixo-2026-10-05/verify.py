"""Baseline/candidate language output, font inventories and PDF geometry controls."""
import datetime,gzip,hashlib,json,subprocess,xml.etree.ElementTree as ET
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];O=Path('/tmp/tekt-fixed-cost-2026-10-05');paired=json.load(gzip.open(H/'paired.json.gz','rt'))
r={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'binaries':paired['binaries'],'head':paired['head'],'source_hashes':paired['source_hashes'],'cases':{}}
for n in paired['fixtures']:
 rows={}
 for e in ['baseline','candidate','vanilla']:
  pdf=O/f'{n}-{e}-paired.pdf';bbox=subprocess.check_output(['pdftotext','-bbox',str(pdf),'-'],text=True);root=ET.fromstring(bbox);ns={'x':'http://www.w3.org/1999/xhtml'}
  pages=[{'dimensions':p.attrib,'words':[{'box':w.attrib,'text':w.text} for w in p.findall('.//x:word',ns)]} for p in root.findall('.//x:page',ns)]
  row={'pages':pages,'text':subprocess.check_output(['pdftotext',str(pdf),'-'],text=True),'font_inventory':subprocess.check_output(['pdffonts',str(pdf)],text=True)}
  if e!='vanilla':
   prefix=O/f'{n}-{e}-verify';subprocess.run(['pdftoppm','-r','36',str(pdf),str(prefix)],check=True,capture_output=True);row['rasters']=[hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(O.glob(prefix.name+'-*.ppm'))]
  rows[e]=row
 rows['baseline_candidate_boxes_equal']=rows['baseline']['pages']==rows['candidate']['pages'];rows['baseline_candidate_text_equal']=rows['baseline']['text']==rows['candidate']['text'];rows['baseline_candidate_rasters_equal']=rows['baseline']['rasters']==rows['candidate']['rasters'];rows['baseline_candidate_fonts_equal']=rows['baseline']['font_inventory']==rows['candidate']['font_inventory'];rows['vanilla_text_equal']=rows['candidate']['text']==rows['vanilla']['text'];r['cases'][n]=rows
 print(n,rows['baseline_candidate_boxes_equal'],rows['baseline_candidate_rasters_equal'],rows['baseline_candidate_fonts_equal'],rows['vanilla_text_equal'])
r['inventory']={}
for e in ['baseline','candidate']:
 cmd=[paired['binaries'][e]['path'],'fonts','--variants'];p=subprocess.run(cmd,text=True,capture_output=True,check=True);r['inventory'][e]={'command':cmd,'stdout':p.stdout,'stderr':p.stderr}
r['inventory_equal']=r['inventory']['baseline']['stdout']==r['inventory']['candidate']['stdout'];r['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(H/'verification.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
assert r['inventory_equal'];assert all(all(x[k] for k in ['baseline_candidate_boxes_equal','baseline_candidate_text_equal','baseline_candidate_rasters_equal','baseline_candidate_fonts_equal']) for x in r['cases'].values())
print('inventory equal',r['inventory_equal'])
