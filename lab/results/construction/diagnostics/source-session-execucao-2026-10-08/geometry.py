from pathlib import Path
import datetime,json,hashlib,subprocess,xml.etree.ElementTree as ET
D=Path(__file__).resolve().parent;W=Path('/tmp/p6-watch-performance-jitter');pairs={}
for name in ['eval_import','parse_import','minimal_import']:
 for round in range(3):pairs[f'{name}-{round}']={e:W/f'{name}-{e}-{round}/main.pdf' for e in ['previous','current']}
for name in ['mutations','resources']:pairs[name]={'previous':Path('/tmp/source-session-behavior')/f'{name}-previous/main.pdf','current':Path('/tmp/source-session-behavior')/f'{name}-tekt/main.pdf'}
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'binary_sha256':hashlib.sha256(Path('target/release/typst').read_bytes()).hexdigest(),'cases':{}}
for name,engines in pairs.items():
 group={}
 for e,pdf in engines.items():
  root=ET.fromstring(subprocess.check_output(['pdftotext','-bbox',str(pdf),'-'],text=True));ns={'x':'http://www.w3.org/1999/xhtml'}
  pages=[{'dimensions':p.attrib,'words':[{'box':w.attrib,'text':w.text} for w in p.findall('.//x:word',ns)]} for p in root.findall('.//x:page',ns)]
  prefix=pdf.parent/f'{name}-geometry';subprocess.run(['pdftoppm','-r','36',str(pdf),str(prefix)],check=True,capture_output=True)
  group[e]={'pages':pages,'rasters':[hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(pdf.parent.glob(prefix.name+'-*.ppm'))],'fonts':subprocess.check_output(['pdffonts',str(pdf)],text=True)}
 assert group['previous']==group['current'],name
 result['cases'][name]=group
(D/'geometry.json').write_text(json.dumps(result,ensure_ascii=False,indent=2));print('geometry',len(pairs),'pairs equal')
# Full expected output of all successful mutation cases, and PNG witnesses.
a=json.loads((D/'mutations.json').read_text());checks=[]
for prev,cur in zip(a['previous']['cases'],a['tekt']['cases']):
 assert prev['case']==cur['case']
 if 'text' in prev:assert prev['text']==cur['text'];checks.append(prev['case'])
 else:assert prev['last_artifact_preserved']==cur['last_artifact_preserved']
r=json.loads((D/'resources.json').read_text());assert r['previous']['cases'][0]==r['tekt']['cases'][0]
(D/'language-checks.json').write_text(json.dumps({'successful_mutations':checks,'image_witness_equal':True,'geometry_pairs':len(pairs)},indent=2))
