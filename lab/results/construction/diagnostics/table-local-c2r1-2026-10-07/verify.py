from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,re,difflib,collections
R=Path.cwd();D=Path(__file__).parent
read=lambda p:json.loads(gzip.decompress(p.read_bytes())) if p.suffix=='.gz' else json.loads(p.read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
productive=lambda p:p.startswith(('01_core/','02_shell/','03_infra/','04_wiring/','00_nucleo/prompts/'))
a=read(D/'tested-final.json.gz');b=read(D/'before-code.json.gz');owners=read(D/'owners.json');allowed={p for pair in owners for p in pair}
def original(p):
 if p in b['files']:return b['files'][p]
 r=subprocess.run(['git','show',b['head']+':'+p],capture_output=True,text=True);return r.stdout if r.returncode==0 else ''
def original_hash(p):
 if p in b['sha256']:return b['sha256'][p]
 r=subprocess.run(['git','show',b['head']+':'+p],capture_output=True);return hashlib.sha256(r.stdout).hexdigest() if r.returncode==0 else None
changed=sorted(p for p in set(a['sha256'])|set(b['sha256']) if productive(p) and a['sha256'].get(p)!=original_hash(p));outside=[p for p in changed if p not in allowed];assert not outside,outside
for p,v in a['sha256'].items():
 if productive(p):assert sha(R/p)==v,p
for p,v in read(D/'seals.json').items():assert sha(R/p)==v['source_sha256'];assert sha(R/v['owner'])==v['l0_sha256']
patch=''.join(''.join(difflib.unified_diff(original(p).splitlines(True),a['files'].get(p,'').splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in changed);(D/'implementation.patch').write_text(patch)
out={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':a['head'],'snapshot':'tested-final.json.gz','preservation':{'owners':len(owners),'changed':changed,'outside_scope':outside},'patch_sha256':sha(D/'implementation.patch'),'collections':{},'geometry':{},'physical':{},'regressions':{}}
keys=['status','stdout','stderr','pdf_text','html']
for name in ['origins','additional','styles','isolated','controls-additional','depth']:
 d=read(D/(name+'.json.gz'));bad={n:[k for k in keys if c['runs']['tekt'].get(k)!=c['runs']['vanilla'].get(k)] for n,c in d['cases'].items()};bad={n:v for n,v in bad.items() if v};assert set(bad)==({'raw-fields','raw-x'} if name=='isolated' else set()),(name,bad)
 out['collections'][name]={'utc':d['utc'],'end_utc':d['finished_utc'],'cases':len(d['cases']),'complete_outputs_equal':len(d['cases'])-len(bad),'differences':bad,'bin_sha256':d['bin_sha256']}
 if name=='origins':out['bin_sha256']=d['bin_sha256'];out['vanilla_pin']=d['vanilla_pin'];assert sha(R/'target/release/typst')==d['bin_sha256']['tekt']
 elif name=='depth':out['debug_sha256']=d['bin_sha256']['tekt'];assert sha(R/'target/debug/typst')==out['debug_sha256']
 else:assert d['bin_sha256']==out['bin_sha256'],name
font=lambda r:sorted(set((x['size'],x['family'].split('+')[-1],x['color']) for x in r.get('fonts',[])))
def metrics(t,v):
 w1,w2=t.get('words',[]),v.get('words',[]);same=[x['text'] for x in w1]==[x['text'] for x in w2]
 delta=lambda ks:max((abs(float(x['box'][k])-float(y['box'][k])) for x,y in zip(w1,w2) for k in ks),default=0) if same else None
 return {'status_matches':t['status']==v['status'],'text_matches':t.get('text')==v.get('text'),'same_words_and_count':same,'font_size_family_color_matches':font(t)==font(v),'max_horizontal_delta_pt':delta(['xMin','xMax']),'max_vertical_delta_pt':delta(['yMin','yMax'])}
def rgb8(s):
 if not s:return None
 vals=re.findall(r'[\d.]+%',s)
 return tuple(round(float(x[:-1])*255/100) for x in vals) if len(vals)==3 else s
stroke=lambda r:sorted(set((rgb8(color),float(width)) for color,width,transform in r.get('stroke_styles',[])))
for label,field in [('geometry-c','geometry'),('geometry-c2','local_geometry'),('physical-c','physical')]:
 d=read(D/(label+'.json.gz'));assert d['bin_sha256']==out['bin_sha256'];out.setdefault(field,{})
 for n,c in d['cases'].items():
  t,v=c['runs']['tekt'],c['runs']['vanilla'];row=metrics(t,v);out[field][n]=row
  assert all(row[k] for k in ['status_matches','text_matches','same_words_and_count']),(label,n,row)
  if n!='show-set-color-positional':assert row['font_size_family_color_matches'],(label,n,row)
  if n!='nested-local-text':assert row['max_vertical_delta_pt']<0.0001,(label,n,row)
  else:assert abs(row['max_vertical_delta_pt']-6.04996)<0.0001,(label,n,row)
  if field=='physical':
   row.update(stroke_matches=stroke(t)==stroke(v),fill_colors_rgb8_matches=sorted(set(map(rgb8,t['fill_colors'])),key=str)==sorted(set(map(rgb8,v['fill_colors'])),key=str),stroke_rgb8_and_width=stroke(t))
   assert row['stroke_matches'] and row['fill_colors_rgb8_matches'],(n,row)
index=read(D/'c2r1-final-regressions.json');old=read(R/'00_nucleo/diagnosticos/table-origens-d08b3c2-2026-10-07/d08b3c2-terminal-regressions.json')
for group,rows in index['groups'].items():
 for i,item in enumerate(rows):
  d=read(Path(item['collection']));prev=read(Path(old['groups'][group][i]['collection']));assert d['bin_sha256']==out['bin_sha256'];ks=['status','stdout','stderr','text'] if group.startswith('raw') else keys
  def differences(doc):return {n:diff for n,c in doc['cases'].items() if (diff:=[k for k in ks if c['runs']['tekt'].get(k)!=c['runs']['vanilla'].get(k)])}
  bad=differences(d);old_bad=differences(prev);new={n:v for n,v in bad.items() if n not in old_bad or not set(v).issubset(old_bad[n])};assert not new,(group,new)
  row={'collection':str(Path(item['collection']).relative_to(R)),'cases':len(d['cases']),'differences':bad,'new_parity_differences':new}
  if group.startswith('raw'):row['metrics']={n:metrics(c['runs']['tekt'],c['runs']['vanilla']) for n,c in d['cases'].items()}
  out['regressions'][group+str(i)]=row
lint=read(D/'final-lint.json');results=lint['runs'][0]['results'];out['lint']=dict(collections.Counter(x['level'] for x in results));assert out['lint'].get('error',0)==0
critical=[]
for x in results:
 p=x.get('locations',[{}])[0].get('physicalLocation',{}).get('artifactLocation',{}).get('uri','').removeprefix('./')
 if p in allowed and x['ruleId'] in ['V5','V6','V15','V26']:critical.append((p,x['ruleId']))
assert not critical,critical;out['lint_owner_critical']=critical
validation=read(D/'validation.json');assert len(validation)==5 and all(r['status']==0 for r in validation);out['validation']=validation
out['tests']={n:re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',(D/('final-'+n+'.log')).read_text()) for n in ['workspace','cli-query']}
final_state=read(D/'final-state.json.gz')
assert all(a['sha256'].get(p)==digest for p,digest in final_state['sha256'].items() if productive(p))
out['final_state']='final-state.json.gz'

r1=read(D/'physical-r1.json.gz');assert r1['bin_sha256']==out['bin_sha256'];out['r1_geometry']={}
for n,c in r1['cases'].items():
 row=metrics(c['runs']['tekt'],c['runs']['vanilla']);out['r1_geometry'][n]=row
 assert all(row[k] for k in ['status_matches','same_words_and_count','font_size_family_color_matches']),(n,row)
 if n in ['regression-100','two-cells','plain-cell','local-show-identity','local-show-body']:
  assert row['max_horizontal_delta_pt']<0.0001 and row['max_vertical_delta_pt']<0.0001,(n,row)
assert len(out['r1_geometry'])==12
(D/'final-results.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n');print(json.dumps({k:out[k] for k in ['bin_sha256','debug_sha256','lint','tests']},indent=2));print('owners',len(owners),'changed',len(changed),'outside',outside)
P=D/'performance';perf=read(P/'summary.json');initial=read(P/'state/start.json');end=read(P/'state/end.json');assert initial['sha256']==end['sha256'] and initial['head']==end['head']
assert all(sha(Path(p))==perf['identity']['bin_sha256'][k] for k,p in perf['identity']['binaries'].items());assert sha(R/'target/release/typst')==perf['identity']['bin_sha256']['current']==out['bin_sha256']['tekt']
blocks=[read(P/p) for p in perf['blocks']];assert len(blocks)==9 and all(b['valid'] for b in blocks)
legacy={name:read(D/(name+'.json.gz')) for name in ['before-r1','before-c2']}
assert legacy['before-r1']['bin_sha256']['tekt']=='ac889581d6423d2fcfdd77d2f7e5cf57562dbf9adb40e73aebc25f959864c04f'
assert legacy['before-c2']['bin_sha256']['tekt']=='23eb4227d17b332af08414506b81c685c63cfa440d18b18e0c2ec69fd23c675c'
out['extended_geometry_comparison']={n:{'current':metrics(c['runs']['tekt'],c['runs']['vanilla']),**{key:metrics(old['cases'][n]['runs']['tekt'],old['cases'][n]['runs']['vanilla']) for key,old in legacy.items()}} for n,c in r1['cases'].items()}
out['performance']={'summary':'performance/summary.json','summary_sha256':sha(P/'summary.json'),'utc_start':initial['utc'],'utc_end':end['utc'],'blocks':9,'measured_invocations':324,'warmups':54,'all_sources_and_binaries_stable':True,'notes':'GNU time/taskset wrapper wall; RSS target. Difference against prior wrong output is descriptive, not speedup for equivalent work.'}
out['documentation']={p:sha(R/p) for p in ['00_nucleo/diagnosticos/table-local-c2r1-execucao-2026-10-07.md','00_nucleo/diagnosticos/plano-debitos-2026-10-06.md']}
(D/'final-results.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n')
print('Extended controls, performance and documentation verified')
