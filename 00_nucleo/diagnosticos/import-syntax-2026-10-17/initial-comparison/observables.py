from pathlib import Path
import json,gzip,datetime,collections
from html.parser import HTMLParser
out=Path(__file__).resolve().parents[1]
load=lambda n:json.load(gzip.open(out/n))
def norm(r):return r['exit'],r['stdout'],r['stderr'].removesuffix('warning: ficheiro vazio: sem conteúdo\n\n')
counts=collections.Counter();changes=[];residual=[];rows=[]
for corpus in ['probe','context-probe']:
 a=load('before-'+corpus+'.json.gz');z=load('after-'+corpus+'.json.gz');assert a['binaries']['vanilla']['sha256']==z['binaries']['vanilla']['sha256'];assert all(z.get('fixtures',{}).get(p)==h for p,h in a.get('fixtures',{}).items());assert len(a['cases'])==len(z['cases'])
 for old,new in zip(a['cases'],z['cases']):
  assert {k:v for k,v in old.items() if k!='results'}=={k:v for k,v in new.items() if k!='results'}
  t=new['results']['tekt'];v=new['results']['vanilla'];o=old['results']['tekt'];ov=old['results']['vanilla'];label=new.get('label',str({k:v for k,v in new.items() if k!='results'}));assert t['exit'] in [0,1],(label,t)
  if new.get('target'):
   # Compare semantic HTML events, including element names/attributes and text.
   class Events(HTMLParser):
    def __init__(self):super().__init__();self.events=[]
    def handle_starttag(self,t,a):self.events.append(('start',t,sorted(a)))
    def handle_startendtag(self,t,a):self.events.append(('startend',t,sorted(a)))
    def handle_endtag(self,t):self.events.append(('end',t))
    def handle_data(self,t):self.events.append(('text',t))
    def handle_decl(self,t):self.events.append(('decl',t))
   def events(text):
    p=Events();p.feed(text);p.close();return p.events
   assert norm(t)==norm(v) and t['exit']==0 and set(t['files'])==set(v['files'])=={'child.html'},(t,v)
   for file in t['files']:assert events(t['files'][file]['text'])==events(v['files'][file]['text']),(t,v)
   assert v['files']==ov['files'] and norm(v)==norm(ov)
   category='bundle-html-semantic-parity';changed=norm(o)!=norm(t) or o['files']!=t['files']
  else:
   assert ov==v
   changed=norm(o)!=norm(t)
   if norm(t)==norm(v):
    disabled=(new.get('features')==[] and label in ['html/False','bundle/False']) or ('profile' in new and not new['profile']&{'html':1,'a11y':2,'bundle':4}[new['file']])
    category='disabled-feature-rejection-parity' if disabled else 'language-parity'
   else:
    category='explicit-residual';residual.append(dict(corpus=corpus,label=label,before=o,after=t,vanilla=v,unchanged_tekt_result=norm(o)==norm(t)))
  counts[category]+=1;rows.append(dict(corpus=corpus,label=label,category=category,tekt_changed=changed))
  if changed:changes.append(dict(corpus=corpus,label=label))
# The main controls and preexisting include morphology must remain identical.
main=[x for x in rows if x['corpus']=='probe' and x['label'].endswith('/main')];assert len(main)==3 and not any(x['tekt_changed'] for x in main)
for x in residual:
 if x['label'] in ['valid/include','valid/include-path']:assert x['unchanged_tekt_result']
assert not [x for x in rows if x['category']=='explicit-residual' and 'profile' in x['label']],residual
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),counts=dict(counts),changes=changes,residual=residual,rows=rows,normalization='Only literal preexisting empty-file warning removed for comparison; raw diagnostics retained; HTML compared as complete parser events, raw files retained')
(out/'observables-result.json').write_text(json.dumps(result,indent=2)+'\n');print(dict(counts),'changes',len(changes),'residual',len(residual))
