from pathlib import Path
import json,re,hashlib,tomllib,subprocess,datetime
ROOT=Path('/repos/Antigravity/typst-crystalline'); OUT=ROOT/'lab/results/prompt-consolidation-2026-10-18'; ARC=ROOT/'lab/history/prompt-consolidation-2026-10-18'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
coverage={}
def add(path,receipt,entry):
 if isinstance(path,str) and path.startswith('00_nucleo/prompts/') and (ROOT/path).is_file():coverage.setdefault(path,[]).append({'receipt':str(receipt.relative_to(ROOT)),'review':entry})
for f in (OUT/'compiler').glob('*.json'):
 data=json.loads(f.read_text()); entries=data if isinstance(data,list) else []
 for e in entries:
  if isinstance(e,dict) and e.get('status','').startswith(('reviewed','consolidated-full-review','translated-full')):add(e.get('path'),f,e)
for f in (OUT/'layout').rglob('*.json'):
 data=json.loads(f.read_text())
 if isinstance(data,dict) and data.get('status')=='complete':add(data.get('file'),f,data)
for sub,names in [('boundaries',['coverage.jsonl','eval-assistance.jsonl','entity-assistance.jsonl']),('nuclei',['coverage.jsonl'])]:
 for name in names:
  f=OUT/sub/name
  if f.exists():
   for line in f.read_text().splitlines():
    e=json.loads(line)
    if e.get('read')=='full':add(e.get('path'),f,e)
f=OUT/'entities/ledger.json'
for path,e in json.loads(f.read_text()).items():
 if e.get('status')=='reviewed':add(path,f,e)
manifest=json.loads((ARC/'manifest.json').read_text()); issues=[];results=[]
for entry in manifest['files']:
 rel=entry['path']; old=ARC/'records'/rel;p=ROOT/rel
 if sha(old)!=entry['sha256'] or old.stat().st_size!=entry['bytes']:issues.append([rel,'archive differs'])
 if p.suffix=='.md':
  for pat in [r'^Hash do Código:.*$',r'^- .* sha256:[0-9a-f]+$']:
   if re.findall(pat,p.read_text(),re.M)!=re.findall(pat,old.read_text(),re.M):issues.append([rel,'metadata differs before repin',pat])
  if len(re.findall(r'^```',p.read_text(),re.M))%2:issues.append([rel,'unbalanced fences'])
  for target in re.findall(r'(?<![\w/])(00_nucleo/prompts/[\w/.-]+\.md)',p.read_text()):
   if not(ROOT/target).is_file():issues.append([rel,'missing canonical owner reference',target])
 else:
  def strip(x):
   if isinstance(x,dict):return {k:strip(v) for k,v in x.items() if k not in ['title','statement']}
   if isinstance(x,list):return [strip(v) for v in x]
   return x
  if strip(tomllib.loads(p.read_text()))!=strip(tomllib.loads(old.read_text())):issues.append([rel,'nucleus structure changed'])
 results.append({'path':rel,'original_sha256':entry['sha256'],'reviewed_sha256':sha(p),'changed':sha(p)!=entry['sha256'],'reviews':coverage.get(rel,[])})
missing=[e['path'] for e in results if not e['reviews']]
summary={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checkpoint':manifest['source_commit'],'state':'uncommitted working tree; agents still editing until explicit final freeze','inventory':len(results),'reviewed':len(results)-len(missing),'missing_reviews':missing,'archive_integrity':'pass' if not any('archive' in str(i) for i in issues) else 'fail','issues':issues}
(OUT/'coverage-progress.json').write_text(json.dumps(summary,indent=2)+'\n');(OUT/'reviewed-progress.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps(summary,indent=2))
