from pathlib import Path
import json,gzip,datetime
p=Path(__file__).resolve().parent
read=lambda n:json.loads(gzip.decompress((p/n).read_bytes()))
pdf=read('occurrences-final.json.gz');html=read('occurrences-html-final.json.gz');before=read('occurrences-html-before.json.gz')
remaining={'equation-alias','equation-grow','equation-shrink','equation-grow-numbered','equation-shrink-numbered'}
same=lambda row:row['results']['tekt']['exit']==row['results']['vanilla']['exit']==0 and row['results']['tekt']['text']==row['results']['vanilla']['text']
assert all(same(row) for row in pdf['cases'] if row['label'] not in remaining)
assert all(not same(row) for row in pdf['cases'] if row['label'] in remaining)
assert all(same(row) for row in html['cases'] if row['label'].startswith('text-'))
def obs(d):return [(x['label'],x['results']['tekt']['exit'],x['results']['tekt']['text'],x['results']['tekt']['stderr'].splitlines()[0] if x['results']['tekt']['exit'] else '') for x in d['cases']]
assert obs(before)==obs(html)
d=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source='occurrences-final-sources.json.gz',pdf=dict(passed=sum(same(x) for x in pdf['cases']),pending=sorted(remaining)),html=dict(passed=sum(same(x) for x in html['cases']),pending=[x['label'] for x in html['cases'] if not same(x)],unchanged_from_before=len(obs(html))))
(p/'probe-audit-final.json').write_text(json.dumps(d,indent=2)+'\n');print(json.dumps(d,indent=2))
