#!/usr/bin/env python3
"""Recompute paired populations and compatible phase boundaries from admitted raw records."""
import json, math, statistics
from collections import defaultdict
from pathlib import Path
OUT=Path(__file__).resolve().parent
EXPECTED=json.loads((OUT/'build-state.json').read_text())['sha256']
def save(name,obj):(OUT/name).write_text(json.dumps(obj,indent=2)+'\n')
def percentile(values,p):
    a=sorted(values);n=(len(a)-1)*p;lo=math.floor(n);hi=math.ceil(n)
    return a[lo]+(a[hi]-a[lo])*(n-lo)
def distribution(a):
    return {'n':len(a),'mean':statistics.mean(a),'median':statistics.median(a),'sd':statistics.stdev(a) if len(a)>1 else 0,'min':min(a),'p05':percentile(a,.05),'p25':percentile(a,.25),'p75':percentile(a,.75),'p95':percentile(a,.95),'max':max(a)}
def union(intervals):
    a=sorted(intervals); result=[]
    for start,end in a:
        if result and start<=result[-1][1]: result[-1][1]=max(end,result[-1][1])
        else: result.append([start,end])
    return result
def span_time(intervals):return sum(b-a for a,b in union(intervals))/1000

def trace_summary(path):
    events=json.loads(path.read_text());events=events.get('traceEvents',[]) if isinstance(events,dict) else events
    stacks=defaultdict(list); spans=[]
    for event in events:
        key=event.get('tid',0)
        if event['ph']=='B':stacks[key].append(event)
        elif event['ph']=='E':
            begin=stacks[key].pop();assert begin['name']==event['name']
            spans.append({'name':begin['name'],'tid':key,'start_us':begin['ts'],'end_us':event['ts'],'depth':len(stacks[key])})
        elif event['ph']=='X':spans.append({'name':event['name'],'tid':key,'start_us':event['ts'],'end_us':event['ts']+event['dur'],'depth':0})
    assert not any(stacks.values()),'Unbalanced trace'
    names=defaultdict(list)
    for s in spans:names[s['name']].append((s['start_us'],s['end_us']))
    root=names['compile once'];assert len(root)==1,'Unexpected compile_once trace shape'
    lo,hi=root[0]
    iterations=union([(s['start_us'],s['end_us']) for s in spans if s['name'].startswith('iter (')])
    def outside_iterations(s):return not any(a<=s['start_us'] and s['end_us']<=b for a,b in iterations)
    evals=union([(s['start_us'],s['end_us']) for s in spans if s['name']=='eval' and outside_iterations(s)])
    pdf=union(names['pdf'])
    # Broad boundaries are disjoint: initial eval, complete stabilization iterations, PDF export.
    categories={'eval_initial_ms':span_time(evals),'layout_stabilization_ms':span_time(iterations),'pdf_export_ms':span_time(pdf)}
    total=(hi-lo)/1000
    assert abs(sum(categories.values())-span_time(evals+iterations+pdf))<1e-5,'Broad spans overlap'
    categories.update(compile_once_ms=total,compile_other_ms=total-sum(categories.values()),iteration_count=len(iterations),nested_eval_inside_layout_ms=span_time([(s['start_us'],s['end_us']) for s in spans if s['name']=='eval' and not outside_iterations(s)]),span_unions_ms={name:span_time(value) for name,value in names.items()},threads=sorted(set(s['tid'] for s in spans)))
    return categories

groups=defaultdict(lambda:defaultdict(list));phase_rows=[]
for p in sorted((OUT/'samples').glob('*/samples.json')):
    receipt=json.loads((OUT/'receipts'/(p.parent.name+'.json')).read_text())
    assert receipt['valid'] and receipt['before']==receipt['after']==EXPECTED
    for row in json.loads(p.read_text())['samples']:
        mode='traced' if row['instrumented'] else 'plain';groups[(mode,row['fixture'])][row['variant']].append(row)
        if mode=='traced':
            if row['variant']=='vanilla':
                detail=trace_summary(p.parent/(row['label']+'.json'))
                broad={k:detail[k] for k in ['eval_initial_ms','layout_stabilization_ms','pdf_export_ms']}
            else:
                detail=json.loads((p.parent/(row['label']+'.json')).read_text())
                broad={'eval_initial_ms':detail['eval_ms'],'layout_stabilization_ms':sum(detail[k] for k in ['introspect_ms','expand_context_ms','layout_ms','shape_ms']),'pdf_export_ms':detail['subset_ms']+detail['render_ms']}
            broad['residual_ms']=row['wall_ms']-sum(broad.values())
            assert abs(sum(broad.values())-row['wall_ms'])<1e-6
            phase_rows.append({'fixture':row['fixture'],'variant':row['variant'],'round':row['round'],'iteration':row['iteration'],'wall_ms':row['wall_ms'],'broad':broad,'detail':detail})
summary={}
for (mode,fixture),values in groups.items():
    engines={};paired=defaultdict(dict)
    for name,rows in values.items():
        assert len(rows)==(18 if mode=='traced' else 36)
        engines[name]={metric:distribution([r[metric] for r in rows]) for metric in ['wall_ms','user_ms','system_ms','maxrss_kib']}
        engines[name]['round_means_ms']={str(n):statistics.mean(r['wall_ms'] for r in rows if r['round']==n) for n in [1,2,3]}
        engines[name]['round_medians_ms']={str(n):statistics.median(r['wall_ms'] for r in rows if r['round']==n) for n in [1,2,3]}
        points=list(engines[name]['round_means_ms'].values());mean=statistics.mean(points);median=statistics.median(points)
        engines[name]['round_mean_median_difference_pct']=abs(mean-median)/median*100
        engines[name]['round_range_over_median_pct']=(max(points)-min(points))/median*100
        for r in rows:paired[(r['round'],r['iteration'])][name]=r
    comparisons={}
    for a,b in [('before','lazy'),('lazy','interval'),('before','interval'),('interval','vanilla')]:
        comparisons[a+'-'+b]={'paired_difference_ms':distribution([p[a]['wall_ms']-p[b]['wall_ms'] for p in paired.values()]),'paired_ratio':distribution([p[a]['wall_ms']/p[b]['wall_ms'] for p in paired.values()])}
    summary[mode+'/'+fixture]={'variant':engines,'comparisons':comparisons}
save('e2e-summary.json',summary);save('phase-samples.json',phase_rows)
phases={}
for fixture in sorted({r['fixture'] for r in phase_rows}):
    engines={}
    for name in ['before','lazy','interval','vanilla']:
        rows=[r for r in phase_rows if r['fixture']==fixture and r['variant']==name]
        engines[name]={'wall_ms':distribution([r['wall_ms'] for r in rows]),'broad':{k:distribution([r['broad'][k] for r in rows]) for k in rows[0]['broad']}}
        if name=='vanilla':engines[name]['font_discovery_ms']=distribution([r['detail']['span_unions_ms'].get('discover fonts',0) for r in rows])
    differences={}
    for a,b in [('before','lazy'),('lazy','interval'),('before','interval'),('interval','vanilla')]:
        delta={k:engines[a]['broad'][k]['mean']-engines[b]['broad'][k]['mean'] for k in engines[a]['broad']}
        wall=engines[a]['wall_ms']['mean']-engines[b]['wall_ms']['mean'];assert abs(sum(delta.values())-wall)<1e-5
        differences[a+'-'+b]={'broad_mean_delta_ms':delta,'wall_mean_delta_ms':wall}
    phases[fixture]={'variant':engines,'differences':differences}
save('phase-summary.json',phases)
rows=[]
for p in sorted((OUT/'catalog').glob('*/samples.json')):
    receipt=json.loads((OUT/'receipts'/(p.parent.name+'.json')).read_text());assert receipt['valid'] and receipt['before']==receipt['after']==EXPECTED
    rows.extend(json.loads(p.read_text())['samples'])
by=defaultdict(dict)
for r in rows:by[(r['round'],r['iteration'])][r['variant']]=r
if rows:
    catalog={'variant':{n:{k:distribution([r[k] for r in rows if r['variant']==n]) for k in ['discover_ms','drop_ms','wall_ms']} for n in ['lazy','interval']},'metadata_controls':sorted({(r['faces'],r['metadata_control']) for r in rows}),'paired_discovery_difference_ms':distribution([r['lazy']['discover_ms']-r['interval']['discover_ms'] for r in by.values()])}
    save('catalog-summary.json',catalog)
print(json.dumps({name:{n:round(r['wall_ms']['median'],3) for n,r in group['variant'].items()} for name,group in summary.items() if name.startswith('plain')},indent=2))
