from pathlib import Path
from collections import Counter
import copy, datetime, difflib, gzip, hashlib, json, subprocess
root=Path.cwd();out=root/'lab/results/v16-corrections-2026-10-19/batch-e';scope=json.loads((out/'scope.json').read_text());lint=root/'lab/target/linter/debug/crystalline-lint'
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
state={'utc_start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'linter_sha256':hashlib.sha256(lint.read_bytes()).hexdigest(),'inputs':{str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in (root/top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}}
r=subprocess.run([str(lint),'.','--format','sarif'],capture_output=True,text=True);(out/'after.sarif').write_text(r.stdout);(out/'after.stderr.log').write_text(r.stderr)
state.update(utc_end=now(),exit=r.returncode,command=[str(lint),'.','--format','sarif'])
with gzip.open(out/'lint-state.json.gz','wt') as f:json.dump(state,f)
baseline_bytes=(out/'baseline.sarif').read_bytes() if (out/'baseline.sarif').exists() else gzip.decompress((out/'baseline.sarif.gz').read_bytes())
before=json.loads(baseline_bytes)['runs'][0]['results'];after=json.loads(r.stdout)['runs'][0]['results']
def path(x):return x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')
def key(x):return json.dumps(x,sort_keys=True)
cases=json.loads((out/'cases.json').read_text())
selected_locations={(r['analysis']['file'],r['analysis']['line']) for r in cases}
selected=[x for x in before if x['ruleId']=='V16' and (path(x),x['locations'][0]['physicalLocation']['region']['startLine']) in selected_locations]
assert len(selected)==7
adjusted_before=copy.deepcopy(before)
movement=[]
for row in adjusted_before:
 if row['level'] not in ['error','warning'] or row['ruleId']!='V16':continue
 location=row['locations'][0]['physicalLocation'];file=path(row);old=location['region']['startLine']
 if (file,old) in selected_locations or file not in scope['sources']:continue
 original=subprocess.check_output(['git','show','HEAD:'+file],text=True).splitlines()
 current=(root/file).read_text().splitlines()
 matches=difflib.SequenceMatcher(a=original,b=current,autojunk=False).get_matching_blocks()
 mapped=[b.b+(old-1-b.a)+1 for b in matches if b.a<=old-1<b.a+b.size]
 assert len(mapped)==1,(file,old,'retained diagnostic source changed')
 new=mapped[0]
 if old!=new:
  movement.append({'file':file,'old_line':old,'new_line':new,'rule':row['ruleId'],'message':row['message'],'level':row['level'],'unchanged_source_line':original[old-1]})
  location['region']['startLine']=new
be=Counter(key(x) for x in adjusted_before if x['level'] in ['error','warning']);ae=Counter(key(x) for x in after if x['level'] in ['error','warning'])
assert be-ae==Counter(map(key,selected)),list((be-ae).elements())
assert not ae-be,list((ae-be).elements())
assert all(x['ruleId']=='V16' and x['level']=='error' for x in after if x['level'] in ['error','warning'])
assert sum(x['ruleId']=='V16' for x in after)==37
# Outside the selected consumers even informational messages and locations stay exact.
assert Counter(key(x) for x in before if path(x) not in scope['sources'])==Counter(key(x) for x in after if path(x) not in scope['sources'])
notes_before=Counter(key(x) for x in before if x['level']=='note');notes_after=Counter(key(x) for x in after if x['level']=='note')
report={'utc':now(),'closed_ids':scope['ids'],'remaining_v16':37,'counts_before':dict(Counter(x['ruleId']+':'+x['level'] for x in before)),'counts_after':dict(Counter(x['ruleId']+':'+x['level'] for x in after)),'removed_notes':[json.loads(x) for x in (notes_before-notes_after).elements()],'added_notes':[json.loads(x) for x in (notes_after-notes_before).elements()],'new_errors_or_warnings':0,'outside_scope_exact':True,'preserved_finding_movements':movement}
(out/'lint-delta.json').write_text(json.dumps(report,indent=2)+'\n')
for name in ['baseline.sarif','after.sarif']:
 with gzip.open(out/(name+'.gz'),'wb') as f:f.write(baseline_bytes if name=='baseline.sarif' else (out/name).read_bytes())
print(json.dumps({k:v for k,v in report.items() if k not in ['removed_notes','added_notes']},indent=2))
print('Notes to review:',len(report['removed_notes']),'removed/moved,',len(report['added_notes']),'added/moved')
