from pathlib import Path
import re,json,hashlib,subprocess,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-math-value-2026-10-16');scope=json.load(open(out/'scope.json'));baseline=json.load(open(out/'baseline-result.json'));probe=json.load(open(out/'probe-baseline-result.json'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert baseline.get('structural_red') and probe['selected_e0004']==0
assert all(sha(p)==h for p,h in baseline['source_hashes'].items())
record={'start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'changes':[],'files':{}}
for row in scope['targets']:
 p=Path(row['source']);text=p.read_text();matches=list(re.finditer(r'^([ \t]*)(?:pub(?:\([^)]*\))? )?fn '+row['function']+r'\(',text,re.M));assert len(matches)==1,row
 start=matches[0].start();opening=text.index('{',matches[0].end());balance=1;end=opening+1
 while balance:
  balance+=(text[end]=='{')-(text[end]=='}');end+=1
 body=text[start:end];wildcards=list(re.finditer(r'^([ \t]+)_(?=\s*=>)',body,re.M));assert len(wildcards)==1,row
 wildcard=wildcards[0]
 def spelling(name):return 'Value::'+name+('' if name in ['None','Auto'] else '(_, _)' if name=='LocatedContent' else '(_)')
 group=('\n'+wildcard[1]+'| ').join(map(spelling,row['complement']))
 after=body[:wildcard.start()]+wildcard[1]+group+body[wildcard.end():]
 p.write_text(text[:start]+after+text[end:])
 record['changes'].append({'id':row['id'],'source':row['source'],'function':row['function'],'before':body,'after':after,'complement':row['complement']})
for source in scope['sources']:
 p=Path(source);p.write_text(re.sub(r'^//! @updated [0-9-]+$','//! @updated 2026-10-16',p.read_text(),count=1,flags=re.M))
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*scope['sources']],check=True)
for source in scope['sources']:record['files'][source]={'expected_after':Path(source).read_text(),'baseline':baseline['files'][source]}
record['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();(out/'materialization.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'decisions':4,'sources':3,'pattern_only':True}),flush=True)
