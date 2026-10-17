from pathlib import Path
import json,gzip,re,itertools,hashlib,subprocess,datetime,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];plan=json.load(open(out/'projection-plan.json'));b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};decl={};constructors={};fields={}
for en,key in [('Content','content'),('ElementPayload','payload')]:
 declarations=[];cases=[]
 for v in plan[key]:
  name=v['name'];kind=v['kind'];fs=v['fields'];declarations.append(name+('(())' if kind=='(' else '{'+','.join(f['name']+':'+('bool' if f['bool'] else '()') for f in fs)+'}' if kind=='{' else ''));bools=[f['name'] for f in fs if f['bool']]
  for values in itertools.product([False,True],repeat=len(bools)):
   flags=dict(zip(bools,values));label=name+''.join(';'+k+'='+str(int(value)) for k,value in flags.items());payload='(())' if kind=='(' else '{'+','.join(f['name']+':'+(str(flags[f['name']]).lower() if f['bool'] else '()') for f in fs)+'}' if kind=='{' else '';cases.append('("'+label+'",'+en+'::'+name+payload+')')
 decl[en]=declarations;constructors[en]=cases
functions=[]
canonical=lambda x:re.sub(r',\)',')',re.sub(r'\s+','',x))
for t in plan['targets']:
 asts=json.loads(subprocess.check_output(['/tmp/tekt-math-layout-latent',t['source'],'matches'],text=True));arms=asts[t['match_index']]['arms'];old=[x['pattern'] for x in t['arms']];code=[]
 for a in arms:
  p=a['pattern'];split=re.split(r'\s+if\s+',p,maxsplit=1);pat=split[0];guard=' if gate' if len(split)>1 else ''
  if p=='_' or canonical(pat)==canonical(t['replacement']):label=9999
  elif p in old:label=old.index(p)
  else:assert t['enum']=='Content' and pat.startswith('Content::Source('),p;label=20000
  code.append(pat+guard+'=>'+str(label)+',')
 functions.append('fn p'+str(t['id'])+'(input:&'+t['enum']+',gate:bool)->usize{match input{'+''.join(code)+'}}')
main='fn main(){'+''.join('for(name,c)in ['+','.join(constructors[t['enum']])+']{for gate in [false,true]{println!("'+str(t['id'])+'/{name}/{gate}: {}",p'+str(t['id'])+'(&c,gate));}}' for t in plan['targets'])+'}';results=[];start=datetime.datetime.now(datetime.timezone.utc).isoformat()
for future in [False,True]:
 name=phase+('-future' if future else '-control');source=out/(name+'.rs');binary=out/(name+'.bin');program='#![allow(dead_code,unused_variables,unreachable_patterns)]\n'+'\n'.join('enum '+en+'{'+','.join(rows+(['TektIntrospectFuture'] if future else []))+'}' for en,rows in decl.items())+'\n'+'\n'.join(functions)+'\n'+main;source.write_text(program);cmd=['rustc','--edition=2021','--crate-name','introspect_probe','--error-format=json',str(source),'-o',str(binary)];r=subprocess.run(cmd,capture_output=True,text=True);messages=[]
 for line in r.stderr.splitlines():
  try:m=json.loads(line)
  except ValueError:continue
  if m.get('code'):messages.append(m)
 item=dict(future=future,command=cmd,exit=r.returncode,stderr=r.stderr,codes=[m['code']['code'] for m in messages])
 if not future:
  assert r.returncode==0 and not messages,item;run=subprocess.run([str(binary)],capture_output=True,text=True);assert run.returncode==0;item['stdout']=run.stdout;item['cases']=len(run.stdout.splitlines())
 binary.unlink(missing_ok=True);results.append(item)
assert all(sha(p)==h for p,h in hashes.items());record=dict(start=start,end=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,results=results,method='Live patterns and guards; enum names/shapes with boolean fields retained and other payloads opaque. Symbolic classification/future guard, not payload semantics.',sources_unchanged=True)
if phase=='before':assert results[1]['exit']==0 and not results[1]['codes']
else:
 assert results[1]['exit']==1 and results[1]['codes']==['E0004']*2;old=json.load(open(out/'before-probe-result.json'));a=old['results'][0]['stdout'].splitlines();z=results[0]['stdout'].splitlines();assert len(a)==len(z);changes=[]
 for x,y in zip(a,z):
  if x!=y:assert y.startswith('0/Source/');changes.append(y)
 assert len(changes)==2;record['symbolic_changes']=changes
(out/(phase+'-probe-result.json')).write_text(json.dumps(record,indent=2)+'\n');print('Future codes',results[1]['codes'],'controls',results[0]['cases'])
