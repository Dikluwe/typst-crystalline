from pathlib import Path
import json,gzip,datetime
p=Path(__file__).resolve().parent
read=lambda n:json.loads(gzip.decompress((p/n).read_bytes()))
pdf=read('occurrences-verified.json.gz');html=read('occurrences-html-verified.json.gz');before=read('occurrences-html-before.json.gz')
assert pdf['binaries']['vanilla']['sha256']==html['binaries']['vanilla']['sha256']=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
assert pdf['binaries']['tekt']['sha256']==html['binaries']['tekt']['sha256']
remaining={'equation-alias','equation-grow','equation-shrink','equation-grow-numbered','equation-shrink-numbered'}
same=lambda row:row['results']['tekt']['exit']==row['results']['vanilla']['exit']==0 and row['results']['tekt']['text']==row['results']['vanilla']['text']
assert all(same(row) for row in pdf['cases'] if row['label'] not in remaining)
assert all(not same(row) for row in pdf['cases'] if row['label'] in remaining)
assert all(same(row) for row in html['cases'] if row['label'].startswith('text-'))
def obs(d):return [(x['label'],x['results']['tekt']['exit'],x['results']['tekt']['text'],x['results']['tekt']['stderr'].splitlines()[0] if x['results']['tekt']['exit'] else '') for x in d['cases']]
assert obs(before)==obs(html)
d=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source='occurrences-verified-sources.json.gz',pdf=dict(passed=sum(same(x) for x in pdf['cases']),pending=sorted(remaining)),html=dict(passed=sum(same(x) for x in html['cases']),pending=[x['label'] for x in html['cases'] if not same(x)],unchanged_from_before=len(obs(html))))
(p/'probe-audit-verified.json').write_text(json.dumps(d,indent=2)+'\n');print(json.dumps(d,indent=2))
