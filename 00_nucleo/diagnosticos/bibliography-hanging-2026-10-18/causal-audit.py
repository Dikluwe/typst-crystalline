from pathlib import Path
import gzip,json,datetime,collections
p=Path(__file__).resolve().parent
read=lambda n:json.loads(gzip.decompress((p/(n+'.json.gz')).read_bytes()))
allrows={c['label']:c for n in ['after','extra-after','boundary-after','repeat-after'] for c in read(n)['cases']}
oldrows={c['label']:c for n in ['before','extra-before','boundary-before','repeat-before'] for c in read(n)['cases']}
results=read('audit');classifications=[]
def line_ids(r):
 out=[]
 for pi,page in enumerate(r['pages']):
  ys=[]
  for y in sorted(w['bounds'][1] for w in page['words']):
   if not ys or y-ys[-1]>1:ys.append(y)
  for w in page['words']:out.append((pi,next(i for i,y in enumerate(ys) if abs(w['bounds'][1]-y)<=1)))
 return out
for row in results['rows']:
 a=row['after'];label=row['label'];cur=allrows[label];old=oldrows[label];t=cur['results']['tekt'];v=cur['results']['vanilla'];b=old['results']['tekt']
 if a['full_geometry_equal']:kind='full_geometry'
 elif a.get('bbox_x_max_error',1000)<=.001 and a.get('bbox_max_error',1000)<=.157 and a['size_max_error']<=.001 and a['words_equal'] and a['pages_equal']:
  assert line_ids(t)==line_ids(v),label;kind='hanging_geometry_and_lines_equal_mixed_metrics_open'
 elif label.startswith('bib-optimized-custom.csl'):
  def bib_body(r):
   words=[w for page in r['pages'] for w in page['words']]
   first=next(i for i,w in enumerate(words) if w['text']=='CUSTOM-BIB:');words=words[first:];origin=words[0]['bounds'][1]
   return [(w['text'],*[round(v-origin if k in [1,3] else v,6) for k,v in enumerate(w['bounds'])]) for w in words if w['text']!='OUTSIDE']
  assert bib_body(t)==bib_body(b),label
  kind='preexisting_plain_optimized_body_line_choice_unchanged' 
 elif label=='bib-content-alias':kind='stored_content_realization_pending_R3_T1'
 else:
  assert label.startswith('context-'),label
  kind='container_residual'
 classifications.append(dict(label=label,classification=kind,tekt_pages_unchanged_from_before=t['pages']==b['pages'],word_lines_equal=line_ids(t)==line_ids(v) if a['words_equal'] else False))
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),criterion='Full geometry remains strict 0.001pt. Separate partial evidence requires same ordered words, per-word x, page and line assignment and page dimensions; mixed vertical metrics stays open. Line membership groups bbox yMin by gaps above1pt (mixed drift <=0.156pt vs observed line gap >=15pt). No relaxation of full metric.',counts=dict(collections.Counter(x['classification'] for x in classifications)),rows=classifications)
(p/'causal-audit.json').write_text(json.dumps(result,indent=2)+'\n');print(result['counts'])
