from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];plan=json.load(open(out/'projection-plan.json'));b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in list(b['hashes'])+scope['nuclei'] if Path(p).exists()};variants=plan['variants'];asts={p:json.loads(subprocess.check_output(['/tmp/tekt-math-layout-latent',p,'matches'],text=True)) for p in scope['sources']};enum=[];constructors=[]
for v in variants:
 n=v['name'];kind=v['kind'];enum.append(n+('(())' if kind=='(' else '{'+','.join(f+':()' for f in v['fields'])+'}' if kind=='{' else ''))
 constructors.append('("'+n+'",Content::'+n+('(())' if kind=='(' else '{'+','.join(f+':()' for f in v['fields'])+'}' if kind=='{' else '')+')')
functions=[]
for t in plan['targets']:
 arms=asts[t['source']][t['match_index']]['arms'];baseline=t['arms'];expected=[x['pattern'] for x in baseline];rows=[]
 for a in arms:
  p=a['pattern'];split=re.split(r'\s+if\s+',p,maxsplit=1);pat=split[0].replace('Self::','Content::');guard=' if gate' if len(split)>1 else ''
  if p=='_' or (phase=='after' and split[0].replace(' ','').replace('\n','')==t['replacement'].replace('Self::','Content::').replace(' ','').replace('\n','')):label=9999
  elif p in expected:label=expected.index(p)
  else:
   assert t['function']=='styled_equation' and any('Content::'+v+'(' in p for v in ['Source','Located','Realized','ShowRecipe']),p;label=20000
  rows.append(pat+guard+'=>'+str(label)+',')
 scrutinee='(content,field)' if t['function']=='get_field' else 'content';functions.append('fn p'+str(t['id'])+'(content:&Content,field:&str,gate:bool)->usize{match '+scrutinee+'{'+''.join(rows)+'}}')
fields=['body','block','numbering','number-align','supplement','alt','double','enabled','alternative','quotes','value','tekt-unknown-field'];calls=''.join('println!("'+str(t['id'])+'/{name}/{field}/{gate}: {}",p'+str(t['id'])+'(&c,field,gate));' for t in plan['targets']);main='fn main(){for(name,c)in ['+','.join(constructors)+']{for field in '+json.dumps(fields)+' {for gate in [false,true]{'+calls+'}}}}';results=[];start=datetime.datetime.now(datetime.timezone.utc).isoformat()
for future in [False,True]:
 tag=phase+('-future' if future else '-control');source=out/(tag+'.rs');binary=out/(tag+'.bin');program='#![allow(dead_code,unused_variables,unreachable_patterns)]\nenum Content{'+','.join(enum+(['TektContentFuture'] if future else []))+'}\n'+'\n'.join(functions)+'\n'+main;source.write_text(program);cmd=['rustc','--edition=2021','--crate-name','content_projection_probe','--error-format=json',str(source),'-o',str(binary)];r=subprocess.run(cmd,capture_output=True,text=True);messages=[]
 for line in r.stderr.splitlines():
  try:d=json.loads(line)
  except ValueError:continue
  if d.get('code'):messages.append(d)
 codes=[d['code']['code'] for d in messages];item=dict(future=future,command=cmd,exit=r.returncode,stderr=r.stderr,codes=codes)
 if not future:
  assert r.returncode==0 and not messages,item;run=subprocess.run([str(binary)],capture_output=True,text=True);assert run.returncode==0;item.update(stdout=run.stdout,cases=len(run.stdout.splitlines()));assert item['cases']==17280
 binary.unlink(missing_ok=True);results.append(item)
assert all(sha(p)==h for p,h in hashes.items());record=dict(start=start,end=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,method='Live match patterns/guard presence, enum names/shapes; opaque payloads replaced by unit and guarded predicate by a boolean. Symbolic dispatch and future-enum guard only, not payload semantics or full parity.',results=results,sources_unchanged=True)
if phase=='before':assert results[1]['exit']==0 and not results[1]['codes'];record['structural_red']=True
else:
 assert results[1]['exit']==1 and results[1]['codes']==['E0004']*6,results[1];old=json.load(open(out/'before-probe-result.json'));a=old['results'][0]['stdout'].splitlines();z=results[0]['stdout'].splitlines();changes=[];hits={t['id']:set() for t in plan['targets']}
 for x,y in zip(a,z):
  key,val=y.rsplit(': ',1);i,n,field,gate=key.split('/');i=int(i)
  if val=='9999':hits[i].add(n)
  if x!=y:assert plan['targets'][i]['function']=='styled_equation' and n in ['Source','Located','Realized','ShowRecipe'];changes.append(key)
 assert len(changes)==96
 for t in plan['targets']:assert hits[t['id']]==set(t['complement']),(t['function'],hits[t['id']],t['complement'])
 record.update(symbolic_changes=changes,all_complement_variants_exercised={str(i):sorted(names) for i,names in hits.items()})
(out/(phase+'-probe-result.json')).write_text(json.dumps(record,indent=2)+'\n');print('future codes',results[1]['codes'],'symbolic cases',results[0]['cases'])
