from pathlib import Path
import json,gzip,hashlib,difflib,datetime,re,collections,html5lib
R=Path.cwd();D=Path(__file__).parent
read=lambda p:json.loads(gzip.decompress(p.read_bytes())) if p.suffix=='.gz' else json.loads(p.read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
a=read(D/'tested-fifth.json.gz');b=read(D/'before-state.json.gz');owners=read(D/'owners.json');nuclei=['00_nucleo/prompts/_nuclei/table/cell-occurrence.toml','00_nucleo/prompts/_nuclei/layout/typc-atom-display.toml'];allowed={p for pair in owners for p in pair}|set(nuclei)
changed=sorted(p for p in set(a['sha256'])|set(b['sha256']) if a['sha256'].get(p)!=b['sha256'].get(p));outside=[p for p in changed if p not in allowed];assert not outside,outside
for p in allowed:assert sha(R/p)==a['sha256'][p],p
for f,row in read(D/'seals.json').items():assert sha(R/f)==row['source_sha256'];assert sha(R/row['owner'])==row['l0_sha256']
patch=''.join(''.join(difflib.unified_diff(b['files'].get(p,'').splitlines(True),a['files'].get(p,'').splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in changed);(D/'implementation.patch').write_text(patch)
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':a['head'],'snapshot':'tested-fifth.json.gz','manifest_sha256':'3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df','preservation':{'owners':len(owners),'changed':changed,'outside_scope':outside},'patch_sha256':sha(D/'implementation.patch'),'collections':{},'geometry':{},'regressions':{}}
keys=['status','stdout','stderr','pdf_text','html']
for name in ['after-fifth','additional-fifth','debug-depth-fifth']:
 d=read(D/(name+'.json.gz'));rows={n:[k for k in keys if c['runs']['tekt'].get(k)!=c['runs']['vanilla'].get(k)] for n,c in d['cases'].items()};bad={n:v for n,v in rows.items() if v}
 result['collections'][name]={'cases':len(rows),'complete_outputs_equal':sum(not v for v in rows.values()),'differences':bad,'bin_sha256':d['bin_sha256']}
 if name=='after-fifth':
  result['bin_sha256']=d['bin_sha256'];result['vanilla_pin']=d['vanilla_pin'];assert set(bad)=={'raw-fields','raw-x','cell-set'}
 else:assert not bad,bad
assert sha(R/'target/release/typst')==result['bin_sha256']['tekt']
g=read(D/'geometry-fifth.json.gz')
for n,c in g['cases'].items():
 t,v=c['runs']['tekt'],c['runs']['vanilla'];w1,w2=t.get('words',[]),v.get('words',[]);same=[x['text'] for x in w1]==[x['text'] for x in w2]
 def delta(keys):return max((abs(float(x['box'][k])-float(y['box'][k])) for x,y in zip(w1,w2) for k in keys),default=0) if same else None
 def fonts(r):return sorted(set((f['size'],f['family'].split('+')[-1],f['color']) for f in r.get('fonts',[])))
 row={'status_matches':t['status']==v['status'],'text_matches':t.get('text')==v.get('text'),'same_words_and_count':same,'font_size_family_color_matches':fonts(t)==fonts(v),'max_vertical_delta_pt':delta(['yMin','yMax']),'max_bbox_delta_pt':delta(['xMin','xMax','yMin','yMax'])};result['geometry'][n]=row
 if n!='show-set-color-positional':assert all(row[k] for k in ['status_matches','text_matches','same_words_and_count','font_size_family_color_matches']) and row['max_vertical_delta_pt']<0.0001,(n,row)
index=read(D/'verified-d08b3b-fifth-regressions.json')
for group,rows in index['groups'].items():
 for item in rows:
  p=Path(item['collection']);d=read(p);assert d['bin_sha256']==result['bin_sha256'],(p,d['bin_sha256']);out={'collection':str(p.relative_to(R)),'cases':len(d['cases']),'differences':{},'html5_dom_equal':{}}
  for n,c in d['cases'].items():
   t,v=c['runs']['tekt'],c['runs']['vanilla'];ks=['status','stdout','stderr','text'] if group.startswith('raw') else keys;dif=[k for k in ks if t.get(k)!=v.get(k)]
   if dif:out['differences'][n]=dif
   if t.get('html') is not None and v.get('html') is not None:
    dom=lambda s:html5lib.serialize(html5lib.parse(s,treebuilder='etree'),tree='etree',omit_optional_tags=False,quote_attr_values='always')
    out['html5_dom_equal'][n]=dom(t['html'])==dom(v['html'])
   if group.startswith('raw') and 'words' in t:
    wt,wv=t['words'],v['words'];assert [x['text'] for x in wt]==[x['text'] for x in wv]
    md=max((abs(float(x['box'][k])-float(y['box'][k])) for x,y in zip(wt,wv) for k in ['yMin','yMax']),default=0)
    out.setdefault('vertical_deltas_pt',{})[n]=md
  result['regressions'][group+'/'+p.name]=out
levels=collections.Counter(x.get('level') for run in read(D/'lint-fifth.sarif')['runs'] for x in run['results']);assert levels['error']==0;result['lint']=dict(levels)
result['tests']={name:re.findall(r'test result:.*',(D/(name+'.log')).read_text()) for name in ['suites-fifth','query-final']};result['builds']={name:(D/(name+'.log')).read_text().splitlines()[-1] for name in ['build-fifth','release-fifth']}
result['limits']=['set table.cell (D08B3-C)','empty-file eval warnings','positional set text(red) ignored in existing set/text owner','generic Table defaults/equality','existing math root render and Raw inline metric residuals','Raw HTML beyond inline typc auto/true/false','rich styles/casts/lines/gutters/spans/PDF-UA and global nonlocatable termination']
(D/'final-results.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'collections':result['collections'],'lint':result['lint'],'preservation':{'owners':len(owners),'outside_scope':outside}},ensure_ascii=False,indent=2))
