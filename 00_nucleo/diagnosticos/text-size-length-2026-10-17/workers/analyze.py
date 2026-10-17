from pathlib import Path
import datetime,gzip,json,math,re
out=Path(__file__).resolve().parents[1];rows=[]
for corpus in ['measurement','extended']:
 before=json.load(gzip.open(out/('before-'+corpus+'.json.gz')));after=json.load(gzip.open(out/('after-'+corpus+'.json.gz')))
 for name in dict.fromkeys(x['name'] for x in after['cases']):
  b={x['binary']:x for x in before['cases'] if x['name']==name};
  if not b and (out/'refuters-before-extended.json.gz').exists():b={x['binary']:x for x in json.load(gzip.open(out/'refuters-before-extended.json.gz'))['cases'] if x['name']==name}
  a={x['binary']:x for x in after['cases'] if x['name']==name};tekt=a['tekt'];vanilla=a['vanilla'];errors=lambda x:[l.strip() for l in x['stderr'].splitlines() if l.startswith('error:') or 'hint:' in l];row=dict(corpus=corpus,name=name,before_exits={k:v['exit'] for k,v in b.items()},after_exits={k:v['exit'] for k,v in a.items()},same_exit=tekt['exit']==vanilla['exit'],before_errors={k:errors(v) for k,v in b.items()},after_errors={k:errors(v) for k,v in a.items()})
  if tekt['exit']==vanilla['exit']==0:
   tw=[w for p in tekt['pages'] for w in p['words']];vw=[w for p in vanilla['pages'] for w in p['words']];row['tekt_words']=tw;row['vanilla_words']=vw;row['same_words']=[w['text'] for w in tw]==[w['text'] for w in vw];row['same_extraction_order']=row['same_words'];row['same_word_multiset']=sorted(w['text'] for w in tw)==sorted(w['text'] for w in vw);
   if not row['same_words'] and row['same_word_multiset'] and len(set(w['text'] for w in tw))==len(tw):
    tw=sorted(tw,key=lambda w:w['text']);vw=sorted(vw,key=lambda w:w['text']);row['aligned_by_unique_word']=True;row['same_words']=True
   row['same_pages']=len(tekt['pages'])==len(vanilla['pages'])
   if row['same_words']:
    row['deltas']={k:max([abs(x[k]-y[k]) for x,y in zip(tw,vw)],default=0) for k in ['x0','x1','y0','y1']};row['font_height_deltas']=[dict(word=x['text'],tekt=x['y1']-x['y0'],vanilla=y['y1']-y['y0'],delta=abs((x['y1']-x['y0'])-(y['y1']-y['y0']))) for x,y in zip(tw,vw)]
  else:row['same_primary_errors_hints']=errors(tekt)==errors(vanilla)
  rows.append(row)
(out/'analysis.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(rows),rows=rows),indent=2)+'\n')
for r in rows:
 print(json.dumps({k:r[k] for k in ['corpus','name','before_exits','after_exits','same_exit','same_primary_errors_hints','same_words','deltas','font_height_deltas'] if k in r}))
