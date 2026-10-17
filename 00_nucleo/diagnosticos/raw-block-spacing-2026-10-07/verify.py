from pathlib import Path
import json,gzip,collections,difflib,hashlib,datetime,re
D=Path(__file__).parent
read=lambda p:json.loads(gzip.decompress(p.read_bytes())) if p.suffix=='.gz' else json.loads(p.read_text())
a=read(D/'tested-fourth.json.gz');b=read(D/'before-state.json.gz');allowed={p for pair in read(D/'owners.json') for p in pair}|{'00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml'}
changed=sorted(p for p in set(a['sha256'])|set(b['sha256']) if a['sha256'].get(p)!=b['sha256'].get(p));outside=[p for p in changed if p not in allowed];assert not outside,outside
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==a['sha256'][p] for p in allowed)
patch=''.join(''.join(difflib.unified_diff(b['files'].get(p,'').splitlines(True),a['files'].get(p,'').splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in changed);(D/'implementation.patch').write_text(patch)
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':a['head'],'snapshot':'tested-fourth.json.gz','preservation':{'owners':len(read(D/'owners.json')),'changed':changed,'outside_scope':outside},'patch_sha256':hashlib.sha256(patch.encode()).hexdigest(),'cases':{}}
for group,label in [('main','after-fourth'),('additional','after-fourth-additional'),('aliases','aliases-fourth'),('refuters','refuters-fourth'),('edges','edges-fourth')]:
 data=read(D/(label+'.json.gz'));result['bin_sha256']=data['bin_sha256'];result['vanilla_pin']=data['vanilla_pin']
 for n,c in data['cases'].items():
  t,v=c['runs']['tekt'],c['runs']['vanilla'];w1,w2=t.get('words',[]),v.get('words',[]);same=[x['text'] for x in w1]==[x['text'] for x in w2]
  def delta(keys):return max((abs(float(x['box'][k])-float(y['box'][k])) for x,y in zip(w1,w2) for k in keys),default=0) if same else None
  def fonts(r):return sorted(set((f['size'],f['family'].split('+')[-1],f['color']) for f in r.get('fonts',[])))
  result['cases'][group+'/'+n]={'collection':label+'.json.gz','status_matches':t['status']==v['status'],'text_matches':t.get('text')==v.get('text'),'font_size_family_color_matches':fonts(t)==fonts(v),'same_words_and_count':same,'max_bbox_delta_pt':delta(['xMin','xMax','yMin','yMax']),'max_vertical_delta_pt':delta(['yMin','yMax'])}
levels=collections.Counter(r.get('level','warning') for run in read(D/'lint-fourth.sarif')['runs'] for r in run['results']);result['lint']={'errors':levels['error'],'warnings':levels['warning'],'notes':levels['note'],'collection':'lint-fourth.sarif'}
result['tests']={n:re.findall(r'test result:.*',(D/(n+'.log')).read_text()) for n in ['green-second-fixed','suites-fourth','query-fourth']}
result['builds']={n:(D/(n+'.log')).read_text().splitlines()[-1] for n in ['build-fourth','release-fourth']}
p=D/'final-results.json'
existing=read(p) if p.exists() else {}
for k in ['regressions','limits']: 
 if k in existing:result[k]=existing[k]
p.write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps(result['cases'],ensure_ascii=False,indent=2))
