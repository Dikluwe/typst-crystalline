"""Recompute descriptive summaries from accepted raw records, without running compilers."""
import csv, hashlib, json, math, statistics
from collections import defaultdict
from pathlib import Path
OUT=Path(__file__).resolve().parent
EXPECTED=json.loads((OUT/'build-state.json').read_text())['sha256']

def save(name,obj): (OUT/name).write_text(json.dumps(obj,indent=2,ensure_ascii=False)+'\n')
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

def micro():
    cases=defaultdict(list)
    for path in sorted((OUT/'criterion').glob('r*/*/**/new/benchmark.json')):
        benchmark=json.loads(path.read_text());parts=path.relative_to(OUT/'criterion').parts;target=parts[1];round_name=parts[0]
        if target.endswith('_filtered'):
            target=target.removesuffix('_filtered');receipt=OUT/'receipts'/f'micro-{round_name}-{target}-{parts[2]}.json'
        else:receipt=OUT/'receipts'/f'micro-{round_name}-{target}.json'
        if not receipt.exists():continue
        admitted=json.loads(receipt.read_text())
        if not admitted['valid'] or admitted['fingerprint_before']!=EXPECTED or admitted['fingerprint_after']!=EXPECTED:continue
        estimates=json.loads((path.parent/'estimates.json').read_text());sample=json.loads((path.parent/'sample.json').read_text())
        selected='slope' if estimates.get('slope') is not None else 'mean'
        record={'round':round_name,'target':target,'criterion_path':str(path.parent.relative_to(OUT)),'estimator':selected,'point_ns':estimates[selected]['point_estimate'],'estimates':estimates,'sample_distribution_ns_per_iter':distribution([t/i for t,i in zip(sample['times'],sample['iters'])]),'sampling_mode':sample['sampling_mode'],'throughput':benchmark.get('throughput')}
        cases[benchmark['full_id']].append(record)
    summary={}
    for name,records in cases.items():
        points=[r['point_ns'] for r in records];med=statistics.median(points);mean=statistics.mean(points)
        summary[name]={'target':records[0]['target'],'rounds':records,'round_distribution_ns':distribution(points),'mean_median_discrepancy_percent':abs(mean-med)/med*100,'range_over_median_percent':(max(points)-min(points))/med*100,'three_rounds':len(points)==3,'adr0115_mean_median_pass':len(points)==3 and abs(mean-med)/med<.03,'strict_range_pass':len(points)==3 and (max(points)-min(points))/med<.03}
    for row in summary.values():
        throughput=row['rounds'][0]['throughput']
        if row['target']=='scanner_bench' and throughput and 'Bytes' in throughput:
            row['input_bytes']=throughput['Bytes']
            row['round_distribution_ns_per_byte']=distribution([r['point_ns']/row['input_bytes'] for r in row['rounds']])
    save('micro-summary.json',summary)
    rows=['# Accepted microbenchmark results','','Times are ns/op. Each point is Criterion slope where available, mean otherwise.','Round range is descriptive, not a confidence interval. Full per-round confidence','intervals and normalized sample distributions are in `micro-summary.json`; batch','iteration counts and total times remain in `criterion/`. Fixtures differ across','groups and cannot be added to obtain a document total.','', '| Case | Round median ns | Scanner ns/byte | Round min–max ns | Range/median | ADR mean/median <3% |','|---|---:|---:|---:|---:|---|']
    for name,row in sorted(summary.items()):
        d=row['round_distribution_ns'];normalized=f"{row['round_distribution_ns_per_byte']['median']:.3f}" if 'input_bytes' in row else '—';rows.append(f"| `{name}` | {d['median']:.3f} | {normalized} | {d['min']:.3f}–{d['max']:.3f} | {row['range_over_median_percent']:.2f}% | {'pass' if row['adr0115_mean_median_pass'] else 'noisy/incomplete'} |")
    (OUT/'micro-results.md').write_text('\n'.join(rows)+'\n')
    return summary

def e2e():
    groups=defaultdict(lambda:defaultdict(list));phases=[]
    for path in sorted((OUT/'e2e').glob('*/*/*/samples.json')):
        block=json.loads(path.read_text())
        if not block['valid'] or block['fingerprint_before']!=EXPECTED or block['fingerprint_after']!=EXPECTED:continue
        mode=path.relative_to(OUT/'e2e').parts[0];name=path.parent.name
        for sample in block['samples']:
            groups[(mode,name)][sample['engine']].append(sample)
            if mode=='instrumented':
                row={k:sample[k] for k in ['engine','fixture','round','pair','wall_ms','user_ms','system_ms']}
                if sample['engine']=='tekt':
                    t=sample['timings'];row['detail']=t
                    row['broad']={'eval_initial_ms':t['eval_ms'],'layout_stabilization_ms':t['introspect_ms']+t['expand_context_ms']+t['layout_ms']+t['shape_ms'],'pdf_export_ms':t['subset_ms']+t['render_ms'],'unattributed_ms':sample['wall_ms']-t['total_ms']}
                else:
                    t=trace_summary(path.parent/(sample['label']+'.json'));row['detail']=t
                    row['broad']={k:t[k] for k in ['eval_initial_ms','layout_stabilization_ms','pdf_export_ms']};row['broad']['unattributed_ms']=sample['wall_ms']-sum(row['broad'].values())
                assert abs(sum(row['broad'].values())-sample['wall_ms'])<1e-4
                phases.append(row)
    summary={}
    for (mode,name),data in groups.items():
        paired={}
        for engine,records in data.items():
            for row in records:paired.setdefault((row['round'],row['pair']),{})[engine]=row['wall_ms']
        complete=[p for p in paired.values() if set(p)=={'tekt','vanilla'}]
        summary[mode+'/'+name]={'engine':{engine:{'wall_ms':distribution([r['wall_ms'] for r in records]),'user_ms':distribution([r['user_ms'] for r in records]),'system_ms':distribution([r['system_ms'] for r in records]),'maxrss_kib':distribution([r['maxrss_kib'] for r in records]),'round_means_ms':{str(n):statistics.mean(r['wall_ms'] for r in records if r['round']==n) for n in sorted(set(r['round'] for r in records))}} for engine,records in data.items()},'paired_difference_ms':distribution([p['tekt']-p['vanilla'] for p in complete]),'paired_ratio':distribution([p['tekt']/p['vanilla'] for p in complete])}
    save('e2e-summary.json',summary);save('phase-samples.json',phases)
    phase_summary={}
    for name in sorted(set(p['fixture'] for p in phases)):
        data=[p for p in phases if p['fixture']==name]
        engines={}
        for engine in ['tekt','vanilla']:
            records=[p for p in data if p['engine']==engine]
            engines[engine]={'wall_ms':distribution([p['wall_ms'] for p in records]),'broad':{k:distribution([p['broad'][k] for p in records]) for k in records[0]['broad']},'detail':{k:distribution([p['detail'][k] for p in records]) for k,v in records[0]['detail'].items() if isinstance(v,(float,int))}}
        delta={k:engines['tekt']['broad'][k]['mean']-engines['vanilla']['broad'][k]['mean'] for k in engines['tekt']['broad']}
        phase_summary[name]={'engine':engines,'mean_phase_delta_ms':delta,'mean_wall_delta_ms':engines['tekt']['wall_ms']['mean']-engines['vanilla']['wall_ms']['mean']}
        assert abs(sum(delta.values())-phase_summary[name]['mean_wall_delta_ms'])<1e-4
    save('phase-summary.json',phase_summary)
    return summary,phase_summary

if __name__=='__main__':
    m=micro();e,p=e2e();print('accepted micro cases',len(m),'CLI comparisons',len(e),'phase comparisons',len(p))
