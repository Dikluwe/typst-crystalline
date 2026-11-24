from pathlib import Path
import json
from random import Random
from statistics import median

D=Path(__file__).resolve().parent
out=D.parents[1]/'results/construction/diagnostics'/D.name
rng=Random(542)
def stats(xs):
    samples=sorted(median(rng.choices(xs,k=len(xs))) for _ in range(10000))
    return {'median':median(xs),'ci95':[samples[249],samples[9749]]}
result={}
for kind in ('explicit','references'):
    blocks={}
    for line in (out/f'{kind}-measurements.jsonl').read_text().splitlines():
        row=json.loads(line)
        assert row['period_ns']==0
        blocks.setdefault(row['block'],{})[row['arm']]=row['cpu_ns']/1e6
    arms=('candidate','control','vanilla')
    assert len(blocks)==24 and all(set(b)==set(arms) for b in blocks.values())
    cpu={arm:stats([b[arm] for b in blocks.values()]) for arm in arms}
    pair=lambda before:[b['candidate']-b[before] for b in blocks.values()]
    ratio=cpu['candidate']['median']/cpu['vanilla']['median']
    result[kind]={
        'cpu_ms':cpu,'candidate_control_ms':stats(pair('control')),
        'candidate_vanilla_ms':stats(pair('vanilla')),
        'candidate_vanilla_ratio':ratio,'remaining_percent':100*(1-1/ratio),
        'favorable_pairs':sum(n<0 for n in pair('control')),
    }
raw=json.loads((out/'census.json').read_text())
for kind in result:
    c={arm:row['counts'].get('3:line_commit',0) for arm,row in raw[kind].items()}
    result[kind]['line_commit']=c|{'avoided':c['control']-c['candidate']}
(out/'summary.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
