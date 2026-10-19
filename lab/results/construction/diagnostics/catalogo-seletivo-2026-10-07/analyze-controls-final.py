from pathlib import Path
import json,subprocess,statistics as st,hashlib,collections
D=Path(__file__).parent;P=D/'performance-final';idx=json.loads((P/'index.json').read_text());blocks=[json.loads((P/p).read_text()) for p in idx['blocks']];assert len(blocks)==24 and all(b['valid'] for b in blocks)
summary=json.loads((P/'summary.json').read_text());assert all(all(q['qualified_geometry'] and q['text_matches'] for q in r['qualification']['previous']) for r in summary['fixtures'].values())
messages=[]
for b in blocks:
 rows={(r['pair'],r['engine']):r for r in b['samples']}
 for i in range(12):
  a=rows[i,'current'];c=rows[i,'previous'];assert a['stderr']==c['stderr'] and a['stdout']==c['stdout'];messages.append({'fixture':a['fixture'],'round':a['round'],'pair':i,'messages_equal':True})
V=D/'visual-final';V.mkdir(exist_ok=True);visual={}
for fixture in ['image','shapes','multiscript']:
 b=next(b for b in blocks if b['samples'][0]['fixture']==fixture and b['samples'][0]['round']==1);row={}
 for engine in ['current','previous','vanilla']:
  r=next(r for r in b['samples'] if r['engine']==engine and r['pair']==0);assert len(b['qualification'][engine]['pages'])==1
  prefix=V/(fixture+'-'+engine);cmd=['pdftoppm','-r','144','-png','-singlefile',r['pdf'],str(prefix)];subprocess.run(cmd,check=True,capture_output=True);png=prefix.with_suffix('.png');row[engine]={'command':cmd,'png':str(png),'sha256':hashlib.sha256(png.read_bytes()).hexdigest()}
 row['identical_to_previous']=row['current']['sha256']==row['previous']['sha256'];row['identical_to_vanilla']=row['current']['sha256']==row['vanilla']['sha256'];assert row['identical_to_previous'];visual[fixture]=row
probe={};C=D/'probe-results-final'
faces=json.loads((C/'validate.json').read_text());probe['faces']=dict(collections.Counter(r['kind'] for r in faces['faces']));probe['all_metadata_coverage_equal']=faces['all_equal'];probe['fast_accepted_faces']=sum(r['fast_accepted'] for r in faces['faces']);probe['selectable_faces']=sum(r['selectable'] for r in faces['faces']);probe['system_requested_bytes']=sum(r['projected_bytes_and_requested'][1] for r in faces['faces'] if r['kind']=='system' and r['projected_bytes_and_requested'])
for mode in ['acquire','catalog']:
 j=json.loads((C/(mode+'.json')).read_text());samples=j['samples'];rounds=[]
 for rnd in [1,2,3]:
  rs=[r for r in samples if r['round']==rnd];rounds.append({'round':rnd,'legacy_median_ms':st.median(r['legacy_ns']/1e6 for r in rs),'current_median_ms':st.median(r['current_ns']/1e6 for r in rs),'paired_ratio_median':st.median(r['current_ns']/r['legacy_ns'] for r in rs)})
 med=lambda key:st.median(r[key]/1e6 for r in samples)
 ratio=st.median(r['current_ns']/r['legacy_ns'] for r in samples)
 spread=lambda key:(max(r[key] for r in rounds)-min(r[key] for r in rounds))/st.median(r[key] for r in rounds)*100
 probe[mode]={'n_pairs':len(samples),'legacy_median_ms':med('legacy_ns'),'current_median_ms':med('current_ns'),'paired_reduction_pct':(1-ratio)*100,'all_pairs_improved':all(r['current_ns']<r['legacy_ns'] for r in samples),'rounds':rounds,'round_median_range_pct':{'legacy':spread('legacy_median_ms'),'current':spread('current_median_ms'),'paired_ratio':spread('paired_ratio_median')},'precise_absolute_time_claim_allowed':spread('current_median_ms')<3 and spread('legacy_median_ms')<3}
result={'probe':probe,'cli':{'samples':sum(len(b['samples']) for b in blocks),'warmups':sum(len(b['warmups']) for b in blocks),'all24_blocks_valid':True,'all_outputs_equivalent_to_previous':True,'messages_pairs':len(messages)},'visual':visual,'note':'Aquisição inclui leitura/metadata/coverage por intervalos vigente; catálogo inclui descoberta. RSS do probe não é memória exclusiva de aquisição; CLI é medida separada. Pixel igualdade é controle contra binário anterior; negativas vanilla ficam preservadas.'}
(D/'controls-final.json').write_text(json.dumps(result,indent=2,ensure_ascii=False)+'\n');print(json.dumps(result['probe'],indent=2));print('visual', {f:(r['identical_to_previous'],r['identical_to_vanilla']) for f,r in visual.items()})
