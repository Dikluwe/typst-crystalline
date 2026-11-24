from pathlib import Path
import json
import subprocess
from bench import D, OUT, WORK, EXPLICIT, STATES, cycle, overrides

expected=json.loads((OUT/'golden.json').read_text())
results={}
for kind,states in [('explicit',EXPLICIT),('references',STATES)]:
    results[kind]={}
    for arm in ('control','candidate'):
        raw=subprocess.check_output(['nm','-n','-S','-C','--defined-only',str(WORK/arm)],text=True)
        names={'wait':'::watch::wait_for_change_since','line_commit':'::flush_line_with_outer_singleton'}
        addresses={key:[] for key in names}
        for line in raw.splitlines():
            items=line.split(maxsplit=3)
            if len(items)!=4 or items[2] not in ('t','T'):
                continue
            symbol=items[3].split('.llvm.',1)[0]
            for key,suffix in names.items():
                if symbol.endswith(suffix):
                    addresses[key].append(int(items[0],16))
        assert all(addresses.values()),addresses
        case=WORK/'census'/kind/arm
        (case/'edits').mkdir(parents=True)
        for i,(_,data,leaf) in enumerate(states):
            assert leaf is None
            (case/('main.typ' if i==0 else f'edits/{i}.typ')).write_bytes(data)
        (case/'addresses.json').write_text(json.dumps(addresses))
        env=cycle.environment(case,'candidate',False)
        env.update(overrides(arm,True))
        with (case/'gdb.log').open('wb') as log:
            subprocess.run(['taskset','-c','1','gdb','-q','-nx','-batch','-x',str(D/'census.gdb'),'--args',str(WORK/arm),
                            'watch','main.typ','out.pdf','--root',str(case),'--pdf-tagged=true'],
                           cwd=case,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=180,check=True)
        for i,(state,_,_) in enumerate(states):
            actual=cycle.cycle.key('candidate',case/f'publication-{i}.pdf')
            if kind=='explicit':
                assert actual==expected[arm][state],(kind,arm,state)
            else:
                from watch import golden
                assert actual==golden('candidate',state),(kind,arm,state)
        result={'counts':json.loads((case/'counts.json').read_text()),
                'trace':[line for line in (case/'gdb.log').read_text().splitlines() if 'lab-page-replay' in line]}
        results[kind][arm]=result
        print(kind,arm,result,flush=True)
        (OUT/'census.json').write_text(json.dumps(results,indent=2))
