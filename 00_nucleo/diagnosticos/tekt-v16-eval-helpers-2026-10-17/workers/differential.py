from pathlib import Path
import json,collections,difflib,gzip,re
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));b=json.load(gzip.open(out/'before.json.gz'));source=scope['sources'][0];old=b['files'][source];new=Path(source).read_text();mapping={i+1:j+1 for block in difflib.SequenceMatcher(None,old.splitlines(),new.splitlines(),autojunk=False).get_matching_blocks() for i,j in zip(range(block.a,block.a+block.size),range(block.b,block.b+block.size))}
def rows(file):return json.load(open(file))['runs'][0]['results']
def loc(r):
 l=r['locations'][0]['physicalLocation'];return l['artifactLocation']['uri'].removeprefix('./'),l['region']['startLine']
def key(r,line=None):
 p,n=loc(r);frozen=json.loads(json.dumps(r));frozen['locations'][0]['physicalLocation']['region']['startLine']=line or n;return (r['ruleId'],r.get('level'),r['message']['text'],p,line or n,json.dumps(frozen,sort_keys=True,ensure_ascii=False))
selected={r['line'] for r in scope['targets']};before=rows(out/'baseline-architecture.log');after=rows(out/'validation-architecture.log');expected=[];removed=[]
for r in before:
 p,n=loc(r)
 if p==source and r['ruleId']=='V16' and n in selected:removed.append(r);continue
 assert p!=source or n in mapping,(r['ruleId'],n,r['message']['text'])
 expected.append(key(r,mapping[n] if p==source else n))
oldcounts=collections.Counter(expected);newcounts=collections.Counter(key(r) for r in after);missing=oldcounts-newcounts;added=newcounts-oldcounts;assert not missing,missing
assert len(removed)==11 and sum(added.values())==11
additions=[]
for k,count in added.items():
 rule,level,message,p,n,full=k;assert count==1 and rule=='V19' and level=='note' and p==source,(k,count)
 additions.append({'rule':rule,'level':level,'message':message,'line':n})
# V19 points to the first alternative in the replacement, at the old fallback line plus prior insertions.
for target in scope['targets']:
 expected_names=set(target['complement']);start=sum(len(line)+1 for line in new.splitlines()[:mapping[target['match_line']]-1]);spelling=scope['variants'][target['enum']][target['complement'][0]];n=new[:new.index(spelling,start)].count('\n')+1
 matched=[a for a in additions if a['line']==n and all(re.search(r'\b'+re.escape(name)+r'\b',a['message']) for name in expected_names)]
 assert len(matched)==1,(target['id'],n,len(matched))
counts=lambda rs:dict(collections.Counter(r['ruleId']+':'+r.get('level','') for r in rs));record={'before':counts(before),'after':counts(after),'removed_V16':len(removed),'new_V19':additions,'all_other_diagnostics_preserved':True};(out/'architecture-delta.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
remaining=[r for r in after if r['ruleId']=='V16'];(out/'remaining-v16.json').write_text(json.dumps(remaining,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'V16_before':233,'V16_after':len(remaining),'new_V19':len(additions),'all_other_diagnostics_preserved':True}),flush=True);assert len(remaining)==222
