from pathlib import Path
import gzip,json,hashlib,collections,datetime,difflib,re
D=Path(__file__).parent
read=lambda p:json.loads(gzip.decompress(p.read_bytes())) if p.suffix=='.gz' else json.loads(p.read_text())
b=read(D/'before-state.json.gz');a=read(D/'tested-final.json.gz');allowed={p for pair in read(D/'owners.json') for p in pair}|{'00_nucleo/prompts/_nuclei/layout/raw-defaults.toml'}
changed=sorted(p for p in set(b['sha256'])|set(a['sha256']) if b['sha256'].get(p)!=a['sha256'].get(p));outside=[p for p in changed if p not in allowed];assert not outside,outside
patch=''.join(''.join(difflib.unified_diff(b['files'].get(p,'').splitlines(True),a['files'].get(p,'').splitlines(True),fromfile='before/'+p,tofile='after/'+p)) for p in changed);(D/'implementation.patch').write_text(patch)
d=read(D/'after.json.gz');result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':a['head'],'snapshot':'tested-final.json.gz','bin_sha256':d['bin_sha256'],'vanilla_pin':d['vanilla_pin'],'preservation':{'changed':changed,'outside_scope':outside},'cases':{}}
for n,c in d['cases'].items():
 t,v=c['runs']['tekt'],c['runs']['vanilla'];w1,w2=t.get('words',[]),v.get('words',[]);same=[x['text'] for x in w1]==[x['text'] for x in w2]
 def fonts(r):return sorted(set((f['size'],f['family'].split('+')[-1],f['color']) for f in r.get('fonts',[])))
 result['cases'][n]={'status_matches':t['status']==v['status'],'text_matches':t.get('text')==v.get('text'),'font_size_family_color_matches':fonts(t)==fonts(v),'same_words_and_count':same,'max_bbox_delta_pt':max((abs(float(x['box'][k])-float(y['box'][k])) for x,y in zip(w1,w2) for k in ['xMin','yMin','xMax','yMax']),default=0) if same else None}
levels=collections.Counter(r.get('level','warning') for run in read(D/'lint-final-sorted.sarif')['runs'] for r in run['results']);result['lint']={'errors':levels['error'],'warnings':levels['warning'],'notes':levels['note']}
result['tests']={name:re.findall(r'test result:.*',(D/(name+'.log')).read_text()) for name in ['green-second','suites-final','query-final']}

existing=read(D/'final-results.json') if (D/'final-results.json').exists() else {}
for key in ['regressions','table_geometry','limits','binary_revalidation','final_utc','patch_sha256','builds']:
 if key in existing:result[key]=existing[key]
(D/'final-results.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps(result['cases'],ensure_ascii=False,indent=2))
