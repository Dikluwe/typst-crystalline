from pathlib import Path
import json,gzip,subprocess,xml.etree.ElementTree as ET,collections,datetime
p=Path(__file__).resolve().parent
before=json.loads(gzip.decompress((p/'before.json.gz').read_bytes()));after=json.loads(gzip.decompress((p/'after.json.gz').read_bytes()))
b={x['label']:x for x in before['cases']};a={x['label']:x for x in after['cases']}
rows=[]
for label,case in a.items():
 t,v=(case['results'][k] for k in ('tekt','vanilla'))
 if label not in ('block-hidden','multiple'):
  assert not t['exit'] and not v['exit'],(label,t,v)
  assert t['text']==v['text'],(label,t['text'],v['text'])
 elif label=='block-hidden':
  assert t['exit'] and v['exit'] and t['stderr'].splitlines()[0]==v['stderr'].splitlines()[0]
 else:
  assert t['text']==b[label]['results']['tekt']['text'],'known multiple-bibliography divergence changed'
 rows.append({'label':label,'equal_text':t['text']==v['text'],'tekt_exit':t['exit'],'vanilla_exit':v['exit']})
colors={}
for label in ('baseline','where-style','bare-style'):
 colors[label]={}
 for name in ('tekt','vanilla'):
  pdf=p/'outputs'/f'{label}-{name}.pdf'
  z=subprocess.run(['pdftohtml','-xml','-hidden','-i','-stdout',str(pdf)],text=True,capture_output=True,check=True)
  (p/'outputs'/f'{label}-{name}-colors.xml').write_text(z.stdout)
  root=ET.fromstring(z.stdout);fonts={f.attrib['id']:f.attrib['color'] for f in root.iter('fontspec')}
  texts=[{'text':''.join(x.itertext()),'color':fonts[x.attrib['font']]} for x in root.iter('text')]
  colors[label][name]={'bibliography':[x['color'] for x in texts if 'BIBWORD' in x['text']],'outside':[x['color'] for x in texts if 'OUTSIDE' in x['text']]}
  assert colors[label][name]['bibliography'] and colors[label][name]['outside']
 assert colors[label]['tekt']==colors[label]['vanilla'],(label,colors[label])
# Compare the unchanged textual-rule corpus to its measured R3 baseline, without touching its artifacts.
old=json.loads(gzip.decompress((p.parents[0]/'textual-realization-2026-10-18/before.json.gz').read_bytes()))
new=json.loads(gzip.decompress((p/'textual-controls-after.json.gz').read_bytes()))
assert [x['label'] for x in old['cases']]==[x['label'] for x in new['cases']]
for x,y in zip(old['cases'],new['cases']):
 assert x['source_sha256']==y['source_sha256']
 for kind in ('tekt','vanilla'):
  u,v=x['results'][kind],y['results'][kind]
  assert (u['exit'],u['text'])==(v['exit'],v['text']),(x['label'],kind,u,v)
  if u['exit']:assert u['stderr'].splitlines()[0]==v['stderr'].splitlines()[0]
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cases':rows,'colors':colors,'textual_controls_unchanged':len(new['cases']),'open':['A2-Indent: geometry and hanging-indent support','A2-Multi: assignment and caches per bibliography occurrence'],'source':'after.json.gz has HEAD/working tree/status/diffstat/UTC/commands/binary hashes'}
(p/'audit.json.gz').write_bytes(gzip.compress(json.dumps(result,ensure_ascii=False,indent=2).encode()));print(json.dumps(result,ensure_ascii=False,indent=2))
