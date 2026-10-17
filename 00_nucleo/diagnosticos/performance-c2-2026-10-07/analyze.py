from pathlib import Path
import json,statistics as st,random,itertools
D=Path(__file__).parent
read=lambda p:json.loads(p.read_text())
def pct(values,q):
 s=sorted(values);x=(len(s)-1)*q;i=int(x);return s[i]+(s[min(i+1,len(s)-1)]-s[i])*(x-i)
def stats(xs):
 mean=st.mean(xs);med=st.median(xs);return {'n':len(xs),'median':med,'mean':mean,'p05':pct(xs,.05),'p95':pct(xs,.95),'mean_median_discrepancy_pct':abs(mean-med)/med*100 if med else 0}
def bootstrap(xs):
 rng=random.Random(127);bs=[st.median(rng.choices(xs,k=len(xs))) for _ in range(10000)];return [pct(bs,.025),pct(bs,.975)]
def compare(t,v):
 samewords=[x['text'] for x in t['words']]==[x['text'] for x in v['words']]
 delta=lambda keys:max((abs(float(x['box'][k])-float(y['box'][k])) for x,y in zip(t['words'],v['words']) for k in keys),default=0) if samewords else None
 font=lambda r:sorted(set((x['size'],x['family'].split('+')[-1],x['color']) for x in r['fonts']))
 row={'text_matches':t['text']==v['text'],'same_words_and_count':samewords,'pages_and_dimensions_match':t['pages']==v['pages'],'fonts_family_size_color_match':font(t)==font(v),'max_x_delta_pt':delta(['xMin','xMax']),'max_y_delta_pt':delta(['yMin','yMax'])}
 row['qualified_geometry']=samewords and row['pages_and_dimensions_match'] and row['fonts_family_size_color_match'] and row['max_x_delta_pt']<.0001 and row['max_y_delta_pt']<.0001
 return row
index=read(D/'index.json');blocks=[read(D/p) for p in index['blocks']];assert len(blocks)==21 and all(b['valid'] for b in blocks)
fixtures=sorted({r['fixture'] for b in blocks for r in b['samples']});out={'identity':read(D/'identity.json'),'blocks':index['blocks'],'fixtures':{},'memory_note':'wait4 maxrss includes forked Python launcher; excluded. Separate GNU time population measures target RSS.'}
for f in fixtures:
 group=[b for b in blocks if b['samples'][0]['fixture']==f];rows=[r for b in group for r in b['samples']];assert len(rows)==108
 by={(r['round'],r['pair'],r['engine']):r for r in rows};result={'engines':{},'comparisons':{},'qualification':{}}
 for engine in ['current','previous','vanilla']:
  times=[r['wall_ms'] for r in rows if r['engine']==engine];rounds=[stats([r['wall_ms'] for r in rows if r['engine']==engine and r['round']==rnd]) for rnd in [1,2,3]];meds=[r['median'] for r in rounds]
  result['engines'][engine]={'wall_ms':stats(times),'round_wall_ms':rounds,'round_median_range_pct':(max(meds)-min(meds))/st.median(meds)*100,'cpu_ms':stats([r['user_ms']+r['system_ms'] for r in rows if r['engine']==engine]),'passes_repeatability':all(r['mean_median_discrepancy_pct']<3 for r in rounds) and (max(meds)-min(meds))/st.median(meds)*100<3}
 for base in ['previous','vanilla']:
  ratios=[by[(rnd,pair,'current')]['wall_ms']/by[(rnd,pair,base)]['wall_ms'] for rnd in [1,2,3] for pair in range(12)];deltas=[by[(rnd,pair,'current')]['wall_ms']-by[(rnd,pair,base)]['wall_ms'] for rnd in [1,2,3] for pair in range(12)]
  result['comparisons'][base]={'paired_ratio':stats(ratios),'paired_delta_ms':stats(deltas),'ratio_bootstrap95':bootstrap(ratios),'delta_bootstrap95_ms':bootstrap(deltas),'round_paired_ratio_median':[st.median(ratios[i*12:(i+1)*12]) for i in range(3)]}
  result['qualification'][base]=[compare(b['qualification']['current'],b['qualification'][base]) for b in group]
 out['fixtures'][f]=result
(D/'summary.json').write_text(json.dumps(out,indent=2,ensure_ascii=False)+'\n')
for f,r in out['fixtures'].items():
 print(f,'wall',[round(r['engines'][e]['wall_ms']['median'],2) for e in ['previous','current','vanilla']],'change%',round((r['comparisons']['previous']['paired_ratio']['median']-1)*100,2),'CI',r['comparisons']['previous']['ratio_bootstrap95'],'qualified old/vanilla',[all(x['qualified_geometry'] for x in r['qualification'][e]) for e in ['previous','vanilla']],'repeat',[r['engines'][e]['passes_repeatability'] for e in ['current','previous','vanilla']])
