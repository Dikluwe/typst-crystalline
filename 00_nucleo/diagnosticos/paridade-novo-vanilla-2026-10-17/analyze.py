from pathlib import Path
import gzip,json,collections
out=Path(__file__).resolve().parent;data=json.load(gzip.open(out/'measurement.json.gz'));rows=[]
def compare(a,b,mode):
 reasons=[]
 if a['exit']!=b['exit']:reasons.append('exit')
 if a['errors']!=b['errors']:reasons.append('error-message')
 if a['hints']!=b['hints']:reasons.append('hints')
 if a['anchors']!=b['anchors']:reasons.append('anchor')
 if a['exit']==b['exit']==0:
  if mode=='eval':
   if a.get('value',a.get('value_raw'))!=b.get('value',b.get('value_raw')):reasons.append('value')
  elif 'table_rows' in a or 'table_rows' in b:
   key='table_rows' if a.get('table_rows') or b.get('table_rows') else 'body_text'
   if a.get(key)!=b.get(key):reasons.append(key)
  elif 'pages' in a and 'pages' in b:
   pa,pb=a['pages'],b['pages']
   if len(pa)!=len(pb):reasons.append('page-count')
   else:
    for x,y in zip(pa,pb):
     if any(abs(x[k]-y[k])>0.002 for k in ['width','height']):reasons.append('page-dimensions')
     if [w['text'] for w in x['words']]!=[w['text'] for w in y['words']]:reasons.append('words')
     else:
      if any(abs(wx['bounds'][k]-wy['bounds'][k])>0.002 for wx,wy in zip(x['words'],y['words']) for k in [0,1,3]):reasons.append('word-geometry')
  if a.get('xml_valid')!=b.get('xml_valid'):reasons.append('xml-validity')
  if a.get('image_size')!=b.get('image_size'):reasons.append('png-dimensions')
  if a.get('bundle_pngs')!=b.get('bundle_pngs'):reasons.append('bundle-png-dimensions')
  if a.get('files')!=b.get('files'):reasons.append('output-files')
 return sorted(set(reasons))
for c in data['cases']:
 a,b=c['results']['tekt'],c['results']['vanilla'];r={'group':c['group'],'label':c['label'],'differences':compare(a,b,c['mode'])}
 if 'vanilla_old' in c['results']:r['new_upstream_delta']=compare(c['results']['vanilla_old'],b,c['mode'])
 rows.append(r)
(out/'analysis.json').write_text(json.dumps(rows,indent=2)+'\n')
for group in dict.fromkeys(r['group'] for r in rows):
 selected=[r for r in rows if r['group']==group];diff=[r for r in selected if r['differences']]
 print(group,'pairs',len(selected),'with preliminary differences',len(diff));print([(r['label'],r['differences'],r.get('new_upstream_delta')) for r in diff][:12])
