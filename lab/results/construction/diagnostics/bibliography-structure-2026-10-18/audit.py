from pathlib import Path
import gzip,json,datetime
p=Path(__file__).resolve().parent;o=p.with_name('bibliography-indent-2026-10-18')
def load(f,folder=p):return json.loads(gzip.decompress((folder/f).read_bytes()))
def index(d):return {c['label']:c for c in d['cases']}
def words(result):return [w for page in result['pages'] for w in page['words']]
before=index(load('before.json.gz'));after=index(load('after.json.gz'));rows=[]
for label,c in after.items():
 t,v=(c['results'][key] for key in ('tekt','vanilla'));assert t['exit']==v['exit']==0,label
 row={'label':label,'whole_text_equal':t['text']==v['text'],'before_text_equal':before[label]['results']['tekt']['text']==before[label]['results']['vanilla']['text']}
 if label.startswith('ieee'):
  tw,vw=words(t),words(v)
  ta=next(w for w in tw if w['text']=='A.');va=next(w for w in vw if w['text']=='A.');assert abs(ta['bounds'][0]-va['bounds'][0])<1e-4,(label,ta,va)
  row['first_body']={'tekt':ta,'vanilla':va}
  if '-multi-' in label:
   tj=next(w for w in tw if w['text']=='J.');vj=next(w for w in vw if w['text']=='J.');assert abs(tj['bounds'][0]-vj['bounds'][0])<1e-4,(label,tj,vj);row['second_body']={'tekt':tj,'vanilla':vj}
 rows.append(row)
# Paragraph spacing changes inter-entry distance, not the language's line strategy.
spacing=[]
for style,marker in [('ieee','A.'),('custom.csl','CUSTOM-BIB:')]:
 gaps={}
 for setting in ('default','0pt','2em'):
  result=after[f'{style}-multi-{setting}']['results']['tekt'];ws=words(result)
  if style=='ieee':a=next(w for w in ws if w['text']=='A.');b=next(w for w in ws if w['text']=='J.')
  else:a,b=[w for w in ws if w['text']==marker]
  gaps[setting]=b['bounds'][1]-a['bounds'][1]
 assert abs(gaps['2em']-gaps['0pt']-24.0)<1e-4,(style,gaps)
 spacing.append({'style':style,'first_to_second_top_pt':gaps})
pages=index(load('pagination-after.json.gz'));pagination=[]
for label,c in pages.items():
 t,v=(c['results'][key] for key in ('tekt','vanilla'));assert t['exit']==v['exit']==0,label;assert t['text']==v['text'],label;assert len(t['pages'])==len(v['pages']),(label,len(t['pages']),len(v['pages']))
 if label=='ieee-height90':
  body=[]
  for w in t['pages'][1]['words']:
   if w['text']=='OUTSIDE':break
   body.append(w)
  assert body and all(w['bounds'][0]>=29.924-1e-4 for w in body),(label,body)
 pagination.append({'label':label,'pages':len(t['pages']),'text_equal':True})
extra=index(load('extra-after.json.gz'));extra_rows=[]
for label,c in extra.items():
 t,v=(c['results'][key] for key in ('tekt','vanilla'));assert t['exit']==v['exit']==0,label
 if label=='prefix-only':
  labels=[w for w in words(t) if w['text'] in ('[1]','[2]')];assert len(labels)==2 and labels[1]['bounds'][1]>labels[0]['bounds'][1],labels
 if label=='rich-prefix':
  for marker in ('BIBWORD','SECONDWORD'):
   a=[w for w in words(t) if w['text']==marker][-1];b=[w for w in words(v) if w['text']==marker][-1];assert abs(a['bounds'][0]-b['bounds'][0])<1e-4,(label,a,b)
 extra_rows.append({'label':label,'text_equal':t['text']==v['text'],'tekt_text':t['text'],'vanilla_text':v['text']})
# Former corpus: type errors stay errors, all positive text remains equal.
legacy=index(load('legacy-after.json.gz'));old=index(load('after.json.gz',o));legacy_rows=[]
for label,c in legacy.items():
 t,v=(c['results'][key] for key in ('tekt','vanilla'))
 if label.startswith('invalid-'):
  assert t['exit'] and v['exit'];assert t['stderr'].splitlines()[0]==v['stderr'].splitlines()[0],label
 else:
  assert t['exit']==v['exit']==0,label;assert t['text']==v['text'],label
  if label.startswith('par-'):assert t['pages']==old[label]['results']['tekt']['pages'],label
 legacy_rows.append(label)
controls=index(load('textual-controls-after.json.gz'));baseline=index(load('textual-controls-after.json.gz',o))
for label,c in controls.items():
 for kind in ('tekt','vanilla'):
  a,b=(d[label]['results'][kind] for d in (controls,baseline));assert(a['exit'],a['text'],a['stderr'].splitlines()[:1])==(b['exit'],b['text'],b['stderr'].splitlines()[:1]),label
custom=index(load('custom-after.json.gz'))
for label,c in custom.items():
 t,v=(c['results'][key] for key in ('tekt','vanilla'));assert t['exit']==v['exit']==0 and t['text']==v['text'],label
colors=load('colors-audit.json.gz');assert colors['direct_style_and_outside_preserved'] and colors['alias_style_still_open']
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'column_cases':rows,'spacing_cases':spacing,'pagination':pagination,'extra':extra_rows,'legacy_count':len(legacy_rows),'textual_controls':len(controls),'custom_count':len(custom),'known_open':['Hanging physical application and occurrence overrides','Merging adjacent citation clusters','RTL bracket order, present before this stage','title spacing and multiple bibliographies','alias style insertion R3-T1 pending']}
(p/'audit.json.gz').write_bytes(gzip.compress(json.dumps(result,indent=2,ensure_ascii=False).encode()));print(json.dumps({'columns':len(rows),'spacing':spacing,'pagination':pagination,'extra_text_equal':[(r['label'],r['text_equal']) for r in extra_rows],'legacy':len(legacy_rows),'textual':len(controls),'custom':len(custom)},indent=2))
