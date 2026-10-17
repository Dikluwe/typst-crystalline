from pathlib import Path
import json,datetime,subprocess,re
out=Path('00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16');scope=json.loads((out/'proposal-scope.json').read_text());tests=json.loads((out/'tests-materialization.json').read_text());record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before={r['source']:Path(r['source']).read_text() for r in scope},edits=[],unformatted={})
assert json.loads((out/'red-result.json').read_text())['exit']==101
for p,t in tests['after'].items():assert record['before'][p]==t

def replace(p,old,new):
 s=Path(p).read_text();assert s.count(old)==1,(p,old);start=s.index(old);s=s[:start]+new+s[start+len(old):];Path(p).write_text(s);record['edits'].append(dict(path=p,start=start,old=old,new=new))
# Constructor bounds are obtained by balanced delimiters, outside comments/strings.
def spans(text):
 i=0;quoted=False;block=0
 while i<len(text):
  if block:
   if text.startswith('/*',i):block+=1;i+=2
   elif text.startswith('*/',i):block-=1;i+=2
   else:i+=1
   continue
  if text.startswith('//',i):i=text.find('\n',i);i=len(text) if i<0 else i;continue
  if text.startswith('/*',i):block=1;i+=2;continue
  if text[i]=='"':
   i+=1
   while i<len(text):
    if text[i]=='\\':i+=2
    elif text[i]=='"':i+=1;break
    else:i+=1
   continue
  if text.startswith('Tag::Start(',i):
   start=i;at=i+len('Tag::Start(');j=at;depth=0;commas=[]
   while j<len(text):
    if text[j]=='"':
     j+=1
     while j<len(text):
      if text[j]=='\\':j+=2
      elif text[j]=='"':j+=1;break
      else:j+=1
     continue
    c=text[j]
    if c in '([{':depth+=1
    elif c in ')]}':
     if depth==0:break
     depth-=1
    elif c==',' and depth==0:commas.append(j)
    j+=1
   assert depth==0 and commas,(text[start:j+1],depth)
   argstart=commas[0]+1;argend=commas[1] if len(commas)>1 else j
   assert len(commas)<=2
   arg=text[argstart:argend];trim=arg.strip()
   if trim == 'info' or trim == 'info()' or trim.startswith(('ElementInfo::new(', 'ElementInfo::with_label(')):
    lead=arg[:len(arg)-len(arg.lstrip())];tail=arg[len(arg.rstrip()):]
    new=text[start:argstart]+lead+'Box::new('+trim+')'+tail+text[argend:j+1]
    yield start,j+1,new
   i=j+1
  else:i+=1
counts={}
for row in scope[:6]:
 p=row['source'];s=Path(p).read_text();items=list(spans(s));counts[p]=len(items);expected={'tag.rs':4,'convergence.rs':7,'from_tags.rs':19,'heading.rs':3,'labelled.rs':1,'walk.rs':1};assert len(items)==expected[Path(p).name],(p,len(items))
 for start,end,new in reversed(items):
  old=s[start:end];assert Path(p).read_text()[start:end]==old
  current=Path(p).read_text();Path(p).write_text(current[:start]+new+current[end:]);record['edits'].append(dict(path=p,start=start,old=old,new=new))
replace('01_core/src/entities/tag.rs','    Start(Location, ElementInfo),','    Start(Location, Box<ElementInfo>),')
replace('01_core/src/entities/show.rs','use ecow::EcoString;','use ecow::EcoString;\nuse std::sync::Arc;')
replace('01_core/src/entities/show.rs','    Style(Styles),','    Style(Arc<Styles>),')
replace('01_core/src/compiler/eval/rules/show.rs','Transformation::Style(capture_set_styles(set, scopes, ctx, engine)?)','Transformation::Style(Arc::new(capture_set_styles(set, scopes, ctx, engine)?))')
for p in record['before']:record['unformatted'][p]=Path(p).read_text()
record['tag_constructor_counts']=counts
(out/'materialization.json').write_text(json.dumps(record,indent=2)+'\n')
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*record['before']],check=True)
print(json.dumps(dict(tag_constructor_counts=counts,edits=len(record['edits']))))
