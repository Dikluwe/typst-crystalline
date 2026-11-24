from pathlib import Path
import hashlib
import json
import os
import subprocess
import sys

D=Path(__file__).resolve().parent
OUT=D.parents[1]/'results/construction/diagnostics'/D.name
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK, STATES, cycle, golden
sys.path.insert(0,str(D.parent/'references-500-visible-stable-cids-2026-11-21'))
from pdf_check import structure, text_layout

options={'TEKT_LAB_MEMO_PROTECT':'1','TEKT_LAB_MEMO_BYTES':str(8*1024*1024)}
describe=cycle.cycle.original.compare.describe

def execute(arm, case, states, reduced=False):
    expected={name:digest for digest,name in map(str.split,(OUT/'binaries.sha256').read_text().splitlines())}
    exe=WORK/arm
    assert hashlib.sha256(exe.read_bytes()).hexdigest()==expected[arm]
    names={'wait':'::watch::wait_for_change_since',
           'line_commit':'::flush_line_with_outer_singleton',
           'resume':'::lab_resume_sequence',
           'cut':'::lab_three_cut_receipt',
           'plan':'::PlanBuilder::build',
           'index':'::introspect::page_running::introspect_with_page_running',
           'apply':'::introspect::page_running::Construction::apply',
           'plan_reuse':'::lab_plan_reuse_receipt',
           'index_reuse':'::lab_index_reuse_receipt',
           'introspect':'::introspect::entrypoints::introspect_with_introspector',
           'physical_copy':'::physical_memo::clone_document',
           'region':'::region_progress::next_region'}
    addresses={k:[] for k in names}
    for line in (OUT/f'{arm}.symbols').read_text().splitlines():
        parts=line.split(maxsplit=3)
        if len(parts)==4 and parts[2] in ('t','T'):
            symbol=parts[3].split('.llvm.',1)[0]
            for key,name in names.items():
                if symbol.endswith(name):
                    addresses[key].append(int(parts[0],16))
    # LTO merges both identical receipt bodies in the measured release.
    if addresses['plan_reuse'] and not addresses['index_reuse']:
        addresses['shared_reuse'] = addresses.pop('plan_reuse')
        addresses['plan_reuse'] = []
    assert addresses['wait'] and addresses['line_commit'] and addresses['region']
    assert addresses['resume'] and addresses['cut'] and addresses['introspect'],addresses
    if reduced:
        addresses = {name: addresses[name] for name in ('wait', 'cut', 'plan_reuse', 'index_reuse', 'shared_reuse') if name in addresses}
    (case/'edits').mkdir(parents=True)
    for i,(_,data,leaf) in enumerate(states):
        assert leaf is None
        (case/('main.typ' if i==0 else f'edits/{i}.typ')).write_bytes(data)
    (case/'addresses.json').write_text(json.dumps(addresses,indent=2))
    env=cycle.environment(case,'candidate',True)
    env.update(options)
    if arm=='candidate':
        env['TEKT_LAB_TOPOLOGY_INDEX_TRACE']=os.environ.get('TEKT_LAB_TOPOLOGY_VALIDATE','trace')
    with (case/'gdb.log').open('wb') as log:
        subprocess.run(['taskset','-c','1','gdb','-q','-nx','-batch','-x',str(D/'topology.gdb'),
                        '--args',str(exe),'watch','main.typ','out.pdf','--root',str(case),'--pdf-tagged=true'],
                       cwd=case,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=180,check=True)
    counts=json.loads((case/'counts.json').read_text())
    events=json.loads((case/'resume.json').read_text())
    return counts,events

if __name__ == '__main__':
    arm=sys.argv[1]
    mode=os.environ.get('TEKT_LAB_TOPOLOGY_VALIDATE','trace')
    tag=f'{arm}-verify' if mode=='verify' else arm
    case=WORK/os.environ.get('TEKT_LAB_TOPOLOGY_CASE',f'topology-{tag}')
    counts,events=execute(arm,case,STATES)
    (OUT/f'topology-{tag}.json').write_text(json.dumps({'counts': counts, 'events': events},indent=2)+'\n')
    (OUT/f'topology-{tag}.log').write_bytes((case/'gdb.log').read_bytes())
    print(arm, counts)
    (OUT/f'topology-{tag}-trials.json').write_bytes((case/'trials.json').read_bytes())
    for n, (name, _, _) in enumerate(STATES):
        assert cycle.cycle.key('candidate', case/f'publication-{n}.pdf') == golden('candidate', name), name
