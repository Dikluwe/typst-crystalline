from pathlib import Path
import json,gzip,hashlib,difflib,subprocess,datetime,re,math,collections
R=Path.cwd();D=Path(__file__).parent
read=lambda p:json.loads(gzip.decompress(p.read_bytes())) if p.suffix=='.gz' else json.loads(p.read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
a=read(D/'tested-final.json.gz');b=read(D/'before-state.json.gz');owners=read(D/'owners.json');nucleus='00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml';allowed={p for pair in owners for p in pair}|{nucleus}
def productive(p):return p.startswith(('01_core/','02_shell/','03_infra/','04_wiring/','00_nucleo/prompts/'))
changed=sorted(p for p in set(a['sha256'])|set(b['sha256']) if productive(p) and a['sha256'].get(p)!=b['sha256'].get(p));outside=[p for p in changed if p not in allowed];assert not outside,outside
for p in allowed:assert sha(R/p)==a['sha256'][p],p
for p,v in read(D/'seals.json').items():assert sha(R/p)==v['source_sha256'];assert sha(R/v['owner'])==v['l0_sha256']
def original(p):
 if p in b['files']:return b['files'][p]
 r=subprocess.run(['git','show',b['head']+':'+p],capture_output=True,text=True);return r.stdout if r.returncode==0 else ''
patch=''.join(''.join(difflib.unified_diff(original(p).splitlines(True),a['files'].get(p,'').splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in changed);(D/'implementation.patch').write_text(patch)
out={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':a['head'],'snapshot':'tested-final.json.gz','preservation':{'owners':len(owners),'changed':changed,'outside_scope':outside},'patch_sha256':sha(D/'implementation.patch'),'collections':{},'geometry':{},'physical':{},'regressions':{}}
keys=['status','stdout','stderr','pdf_text','html']
expected={'refuters-verified':{'header-y-style-conflict','local-prepared-inside-table'},'isolated-verified':{'raw-fields','raw-x'}}
for name in ['after-verified','refuters-verified','extra-verified','show-fold-verified','additional-verified','isolated-verified','debug-depth-verified']:
 d=read(D/(name+'.json.gz'));bad={n:[k for k in keys if c['runs']['tekt'].get(k)!=c['runs']['vanilla'].get(k)] for n,c in d['cases'].items()};bad={n:ks for n,ks in bad.items() if ks};assert set(bad)==expected.get(name,set()),(name,bad)
 out['collections'][name]={'utc':d['utc'],'end_utc':d['finished_utc'],'cases':len(d['cases']),'complete_outputs_equal':len(d['cases'])-len(bad),'differences':bad,'bin_sha256':d['bin_sha256']}
 if name=='after-verified':out['bin_sha256']=d['bin_sha256'];out['vanilla_pin']=d['vanilla_pin'];assert sha(R/'target/release/typst')==d['bin_sha256']['tekt']
 if name=='debug-depth-verified':assert sha(R/'target/debug/typst')==d['bin_sha256']['tekt'];out['debug_sha256']=d['bin_sha256']['tekt']
 elif name!='after-verified':assert d['bin_sha256']==out['bin_sha256']
def font(r):return sorted(set((x['size'],x['family'].split('+')[-1],x['color']) for x in r.get('fonts',[])))
def metrics(t,v):
 w1,w2=t.get('words',[]),v.get('words',[]);same=[x['text'] for x in w1]==[x['text'] for x in w2]
 delta=lambda ks:max((abs(float(x['box'][k])-float(y['box'][k])) for x,y in zip(w1,w2) for k in ks),default=0) if same else None
 return {'status_matches':t['status']==v['status'],'text_matches':t.get('text')==v.get('text'),'same_words_and_count':same,'font_size_family_color_matches':font(t)==font(v),'max_horizontal_delta_pt':delta(['xMin','xMax']),'max_vertical_delta_pt':delta(['yMin','yMax'])}
def rgb8(s):
 if not s:return None
 vals=re.findall(r'[\d.]+%',s)
 if len(vals)==3:return tuple(round(float(x[:-1])*255/100) for x in vals)
 return s
red=(255,65,54)
def stroke_styles(r):return sorted(set((rgb8(color),float(width)) for color,width,transform in r.get('stroke_styles',[])))
g=read(D/'geometry-verified.json.gz');assert g['bin_sha256']==out['bin_sha256']
for n,c in g['cases'].items():
 t,v=c['runs']['tekt'],c['runs']['vanilla'];row=metrics(t,v);out['geometry'][n]=row
 assert row['status_matches'] and row['text_matches'] and row['same_words_and_count'],(n,row)
 assert row['max_vertical_delta_pt']<0.0001,(n,row)
 if n!='show-set-color-positional':assert row['font_size_family_color_matches'],(n,row)
 if n.startswith('cell-stroke'):
  row['stroke_rgb8_and_width']=stroke_styles(t);row['vanilla_stroke_rgb8_and_width']=stroke_styles(v)
  assert stroke_styles(t)==stroke_styles(v)==[(red,2.0)],(n,stroke_styles(t),stroke_styles(v))
for style,ctor in [('cell-align-fold','cell-align-ctor-control'),('cell-set-inset','cell-inset-ctor-control')]:
 t=g['cases'][style]['runs']['tekt'];v=g['cases'][ctor]['runs']['tekt'];row=metrics(t,v);assert row['max_horizontal_delta_pt']==0 and row['max_vertical_delta_pt']==0,(style,row)
 out['geometry'][style]['same_as_explicit_constructor']=True
p=read(D/'physical-verified.json.gz');assert p['bin_sha256']==out['bin_sha256']
for n,c in p['cases'].items():
 t,v=c['runs']['tekt'],c['runs']['vanilla'];row=metrics(t,v);row['stroke_matches']=stroke_styles(t)==stroke_styles(v);row['stroke_rgb8_and_width']=stroke_styles(t);row['fill_colors_rgb8_matches']=sorted(set(map(rgb8,t['fill_colors'])),key=str)==sorted(set(map(rgb8,v['fill_colors'])),key=str);out['physical'][n]=row
 assert all(row[k] for k in ['status_matches','text_matches','same_words_and_count','font_size_family_color_matches','stroke_matches','fill_colors_rgb8_matches']),(n,row)
 if n.startswith('fill-'):assert red in set(map(rgb8,t['fill_colors']))
 if n.startswith('stroke-'):assert stroke_styles(t)==[(red,2.0)]
index=read(D/'verified-d08b3c-verified-regressions.json');old=read(R/'00_nucleo/diagnosticos/table-cell-isolada-d08b3b-2026-10-07/verified-d08b3b-fifth-regressions.json')
for group,rows in index['groups'].items():
 for i,item in enumerate(rows):
  d=read(Path(item['collection']));prev=read(Path(old['groups'][group][i]['collection']));assert d['bin_sha256']==out['bin_sha256'];bad={};old_bad={};ks=['status','stdout','stderr','text'] if group.startswith('raw') else keys
  for n,c in d['cases'].items():
   t,v=c['runs']['tekt'],c['runs']['vanilla'];diff=[k for k in ks if t.get(k)!=v.get(k)]
   if diff:bad[n]=diff
  for n,c in prev['cases'].items():
   t,v=c['runs']['tekt'],c['runs']['vanilla'];diff=[k for k in ks if t.get(k)!=v.get(k)]
   if diff:old_bad[n]=diff
  new={n:ks for n,ks in bad.items() if n not in old_bad or not set(ks).issubset(old_bad[n])};assert not new,(group,new)
  row={'collection':str(Path(item['collection']).relative_to(R)),'cases':len(d['cases']),'differences':bad,'new_parity_differences':new}
  if group.startswith('raw'):row['metrics']={n:metrics(c['runs']['tekt'],c['runs']['vanilla']) for n,c in d['cases'].items()}
  out['regressions'][group+str(i)]=row
lint=read(D/'lint-final.json');out['lint']=dict(collections.Counter(x['level'] for x in lint['runs'][0]['results']));assert out['lint'].get('error',0)==0
validation=read(D/'validation.json');assert all(r['status']==0 for r in validation);out['validation']=validation
out['tests']={n:re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored', (D/(n+'.log')).read_text()) for n in ['workspace','cli-query']}
final_state=read(D/'final-state.json.gz')
assert all(a['sha256'].get(p)==digest for p,digest in final_state['sha256'].items() if productive(p)), 'productive source changed after validation'
out['final_state']='final-state.json.gz'
out['documentation']={p:sha(R/p) for p in ['00_nucleo/diagnosticos/table-cell-styles-d08b3c-execucao-2026-10-07.md','00_nucleo/diagnosticos/plano-debitos-2026-10-06.md']}
(D/'final-results.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n');print(json.dumps({k:out[k] for k in ['bin_sha256','debug_sha256','lint','tests','preservation']},ensure_ascii=False,indent=2))
