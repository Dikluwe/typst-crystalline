from pathlib import Path
import gzip,json,datetime
p=Path(__file__).resolve().parent
read=lambda n:json.loads(gzip.decompress((p/n).read_bytes()))
rows=read('after-candidate8.json.gz')['cases'];pdf=[]
for row in rows:
 a,b=[row['results'][n] for n in ['tekt','vanilla']]
 same=a['exit']==b['exit'] and (a['text']==b['text'] if a['exit']==0 else a['stderr'].splitlines()[0]==b['stderr'].splitlines()[0])
 pdf.append(dict(id=row['id'],same=same,tekt=a['text'],vanilla=b['text']))
svg=[]
for row in read('style-after-candidate8.json.gz')['cases']:
 a,b=[row['results'][n] for n in ['tekt','vanilla']]
 fills=lambda v:[g['fill'] for g in v.get('glyphs',[])]
 svg.append(dict(label=row['label'],same=a['exit']==b['exit']==0 and fills(a)==fills(b),tekt=fills(a),vanilla=fills(b)))
html=[dict(id=row['id'],same=row['same']) for row in read('html-after-candidate8.json.gz')['cases']]
report=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pdf=pdf,svg=svg,html=html)
(p/'exports-check-candidate8.json').write_text(json.dumps(report,indent=2)+'\n')
for name,records in [('PDF',pdf),('SVG colors',svg),('HTML',html)]:
 print(name,sum(x['same'] for x in records),'/',len(records))
 assert all(row['same'] for row in records),(name,[r for r in records if not r['same']])
