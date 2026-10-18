from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,tarfile,xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent
b=json.loads(gzip.decompress((p/'before.json.gz').read_bytes()));a=json.loads(gzip.decompress((p/'after.json.gz').read_bytes()))
before={x['label']:x for x in b['cases']};after={x['label']:x for x in a['cases']}
controls=[]
for label in ('baseline','alias','alias-style','sequence','ieee','context'):
 x=before[label]['results']['tekt']['pages'];y=after[label]['results']['tekt']['pages'];assert len(x)==len(y)
 delta=0.;words=0
 for page1,page2 in zip(x,y):
  assert len(page1['words'])==len(page2['words'])
  delta=max(delta,abs(page1['width']-page2['width']),abs(page1['height']-page2['height']))
  for u,v in zip(page1['words'],page2['words']):
   assert u['text']==v['text'];words+=1;delta=max(delta,max(abs(c-d) for c,d in zip(u['bounds'],v['bounds'])))
 assert delta<1e-8,(label,delta)
 controls.append({'label':label,'max_delta_pt':delta,'words':words})
geometry=[]
for label in ('baseline','where-indent'):
 row={'label':label,'results':{}}
 for kind in ('tekt','vanilla'):
  ws=after[label]['results'][kind]['pages'][0]['words'];first=next(i for i,w in enumerate(ws) if w['text']=='Silva,');end=next(i for i,w in enumerate(ws) if w['text']=='OUTSIDE')
  bw=ws[first:end];lines={}
  for w in bw:lines.setdefault(round(w['bounds'][1],3),[]).append(w)
  row['results'][kind]={'citation_top_pt':ws[0]['bounds'][1],'bibliography_first_top_pt':bw[0]['bounds'][1],'line_start_x_pt':[min(w['bounds'][0] for w in line) for line in lines.values()]}
 geometry.append(row)
# Preserve and prove that the alias style defect pre-dates this patch.
evidence=p/'evidence';evidence.mkdir(exist_ok=True)
with tarfile.open(p/'before-artifacts.tar.gz') as tar:raw=tar.extractfile('outputs/alias-style-tekt.pdf').read()
assert hashlib.sha256(raw).hexdigest()==before['alias-style']['results']['tekt']['artifact_sha256']
oldpdf=evidence/'alias-style-before.pdf';oldpdf.write_bytes(raw)
colors={}
for label,pdf in [('before-tekt',oldpdf),('after-tekt',p/'outputs/alias-style-tekt.pdf'),('vanilla',p/'outputs/alias-style-vanilla.pdf')]:
 cmd=['pdftohtml','-xml','-hidden','-i','-stdout',str(pdf)];z=subprocess.run(cmd,text=True,capture_output=True,check=True);(evidence/(label+'-alias-colors.xml')).write_text(z.stdout)
 root=ET.fromstring(z.stdout);fonts={f.attrib['id']:f.attrib['color'] for f in root.iter('fontspec')}
 colors[label]={'command':cmd,'pdf_sha256':hashlib.sha256(pdf.read_bytes()).hexdigest(),'BIBWORD_colors':[fonts[x.attrib['font']] for x in root.iter('text') if 'BIBWORD' in ''.join(x.itertext())]}
assert colors['before-tekt']['BIBWORD_colors']==colors['after-tekt']['BIBWORD_colors']
assert colors['after-tekt']['BIBWORD_colors']!=colors['vanilla']['BIBWORD_colors']
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_provenance':'after.json.gz and complete-freeze.json.gz','unchanged_geometry_controls':controls,'open_geometry':geometry,'preexisting_alias_style_defect':colors,'classification':'Alias cache was already CSL before; new outer show-set fails to color the alias both before and after. A realization/lexical insertion defect, distinct from cache selection; retain in R3-T1 refuters and do not mark style parity.'}
(p/'geometry-controls.json.gz').write_bytes(gzip.compress(json.dumps(result,ensure_ascii=False,indent=2).encode()));print(json.dumps(result,ensure_ascii=False,indent=2))
