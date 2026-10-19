#!/usr/bin/env python3
import collections,datetime,difflib,gzip,hashlib,json,pathlib,subprocess
R=pathlib.Path('/repos/Antigravity/typst-crystalline');H=pathlib.Path(__file__).resolve().parent;C=H.parent.parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
read=lambda p:json.loads(gzip.decompress(p.read_bytes()) if p.suffix=='.gz' else p.read_bytes())
def data(p):return read(p)['runs'][0]['results']
def row(v):
 loc=v['locations'][0]['physicalLocation'];return {'source':loc['artifactLocation']['uri'].removeprefix('./'),'line':loc['region']['startLine'],'column':loc['region'].get('startColumn'), 'rule':v['ruleId'],'level':v['level'],'message':v['message']['text']}
def key(v):return json.dumps(v,sort_keys=True)
def counter(items):return collections.Counter(map(key,items))
def expand(counter):return [json.loads(k) for k,n in counter.items() for _ in range(n)]
def main():
 out=H/'lint-review.json';assert not out.exists(),'preserve audit attempts'
 bpath=C/'baseline.sarif.gz';apath=C/'after.sarif.gz';before=list(map(row,data(bpath)));after=list(map(row,data(apath)));scope=read(C/'scope.json');sources=set(scope['sources'])
 outside_before=counter([v for v in before if v['source'] not in sources]);outside_after=counter([v for v in after if v['source'] not in sources]);assert outside_before==outside_after,'out-of-scope diagnostics changed'
 errorsbefore=[v for v in before if v['level'] in {'error','warning'}];errorsafter=[v for v in after if v['level'] in {'error','warning'}]
 def no_pos(v):return {k:x for k,x in v.items() if k not in {'line','column'}}
 removed=expand(counter(map(no_pos,errorsbefore))-counter(map(no_pos,errorsafter)));added=expand(counter(map(no_pos,errorsafter))-counter(map(no_pos,errorsbefore)))
 assert not added and len(removed)==1 and removed[0]['source']=='01_core/src/compiler/stdlib/shapes/ellipse.rs' and removed[0]['rule']=='V16'
 assert sum(v['rule']=='V16' for v in before)==21 and sum(v['rule']=='V16' for v in after)==20
 maps={};sourcehash={}
 for source in sources:
  old=subprocess.check_output(['git','show',scope['base_head']+':'+source],cwd=R,text=True).splitlines();new=(R/source).read_text().splitlines();maps[source]={};sourcehash[source]=sha(R/source)
  for match in difflib.SequenceMatcher(None,old,new,autojunk=False).get_matching_blocks():
   maps[source].update({match.a+i+1:match.b+i+1 for i in range(match.size)})
 note_before=[v for v in before if v['source'] in sources and v['level'] not in {'error','warning'}];note_after=[v for v in after if v['source'] in sources and v['level'] not in {'error','warning'}];remaining=counter(note_after);preserved=[];lost=[]
 for v in note_before:
  mapped=dict(v);mapped['line']=maps[v['source']].get(v['line'])
  if remaining[key(mapped)]:remaining[key(mapped)]-=1;preserved.append({'before':v,'after':mapped})
  else:lost.append(v)
 added_notes=expand(remaining)
 for v in lost:v['source_line']=subprocess.check_output(['git','show',scope['base_head']+':'+v['source']],cwd=R,text=True).splitlines()[v['line']-1]
 for v in added_notes:v['source_line']=(R/v['source']).read_text().splitlines()[v['line']-1]
 report={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'runner_sha256':sha(pathlib.Path(__file__)),'baseline_sarif_sha256':sha(bpath),'after_sarif_sha256':sha(apath),'source_sha256':sourcehash,'outside_scope_identical':True,'removed_errors':removed,'new_errors':added,'counts_before':dict(collections.Counter(v['rule'] for v in before)),'counts_after':dict(collections.Counter(v['rule'] for v in after)),'notes_mapped_without_message_change':preserved,'notes_unmapped_removed':lost,'notes_unmapped_added':added_notes,'note_review_status':'pending independent source review'};out.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k] for k in ['counts_before','counts_after']}));print('notes need review:',len(lost),len(added_notes))
if __name__=='__main__':main()
