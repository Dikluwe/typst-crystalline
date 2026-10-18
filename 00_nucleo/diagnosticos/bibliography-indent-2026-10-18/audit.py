from pathlib import Path
import gzip,json,datetime
p=Path(__file__).resolve().parent
def load(f):return json.loads(gzip.decompress((p/f).read_bytes()))
def index(d):return {c['label']:c for c in d['cases']}
b=index(load('before.json.gz'));a=index(load('after.json.gz'));summary={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'flow':[],'invalid':[],'unchanged_par_geometry':[],'text':[]}
for name,c in a.items():
 t,v=(c['results'][k] for k in ('tekt','vanilla'))
 if name.startswith('invalid-'):
  assert t['exit'] and v['exit'],name
  assert t['stderr'].splitlines()[0]==v['stderr'].splitlines()[0],name
  assert not b[name]['results']['tekt']['exit'],name
  summary['invalid'].append(name);continue
 assert not t['exit'] and not v['exit'],name
 summary['text'].append({'label':name,'equal':t['text']==v['text']})
 assert t['text']==v['text'],name
 if name.startswith('par-'):
  assert t['pages']==b[name]['results']['tekt']['pages'],name
  summary['unchanged_par_geometry'].append(name);continue
 words=t['pages'][0]['words'];cite=words[0]
 title=next((w for w in words if w['text']=='Bibliography'),None)
 if name.startswith('mla-'):body=next(w for w in words if w['text']=='Silva,')
 elif name.startswith('ieee-'):body=[w for w in words if w['text']=='[1]'][1]
 else:body=next(w for w in words if w['text']=='CUSTOM-BIB:')
 assert abs(body['bounds'][0]-8)<.001,name
 assert body['bounds'][1]>cite['bounds'][1]+1,name
 if title:
  assert abs(title['bounds'][0]-8)<.001,name
  assert cite['bounds'][1]+1<title['bounds'][1]<body['bounds'][1]-1,name
 summary['flow'].append({'label':name,'citation':cite,'title':title,'body':body})
args=index(load('argument-after.json.gz'))
for name,c in args.items():
 t,v=(c['results'][k] for k in ('tekt','vanilla'))
 if 'valid' in name and 'invalid' not in name:assert t['exit']==v['exit']==0,name
 else:
  assert t['exit'] and v['exit'],name
  assert t['stderr'].splitlines()[0]==v['stderr'].splitlines()[0],name
old=p.with_name('bibliography-a2-2026-10-18');baseline=index(json.loads(gzip.decompress((old/'textual-controls-after.json.gz').read_bytes())))
controls=index(load('textual-controls-after.json.gz'))
for name,c in controls.items():
 for binary in ('tekt','vanilla'):
  r,z=(x[name]['results'][binary] for x in (controls,baseline))
  assert (r['exit'],r['text'],r['stderr'].splitlines()[:1])==(z['exit'],z['text'],z['stderr'].splitlines()[:1]),name
custom=index(load('custom-after.json.gz'))
for name,c in custom.items():
 t,v=(c['results'][k] for k in ('tekt','vanilla'));assert t['exit']==v['exit']==0,name;assert t['text']==v['text'],name
summary.update(arguments=len(args),textual_controls=len(controls),custom_csl=len(custom),known_open=['hanging-indent physical application and precedence','CSL prefixes/entry blocks','multiple bibliographies','location query positions'])
(p/'audit.json.gz').write_bytes(gzip.compress(json.dumps(summary,indent=2).encode()))
print(json.dumps({k:len(v) if isinstance(v,list) else v for k,v in summary.items()},indent=2))
