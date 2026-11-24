from pathlib import Path
import hashlib
import json
import subprocess
import sys

D=Path(__file__).resolve().parent
OUT=D.parents[1]/'results/construction/diagnostics'/D.name
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK, STATES, cycle, golden

arm=WORK/'control'
assert hashlib.sha256(arm.read_bytes()).hexdigest()=='80681d67c5b8c3cbb4b3b64cec04f422e68272239513fdf82a6a5d3e36fa202d'
table=subprocess.check_output(['nm','-n','-S','-C','--defined-only',str(arm)],text=True)
wanted={
    'wait':'::watch::wait_for_change_since',
    'region':'::region_progress::next_region',
    'line_commit':'::flush_line_with_outer_singleton',
}
targets={k:[] for k in wanted}
for line in table.splitlines():
    words=line.split(maxsplit=3)
    if len(words)==4 and words[2] in ('t','T'):
        for k,s in wanted.items():
            if words[3].split('.llvm.',1)[0].endswith(s):
                targets[k].append(int(words[0],16))
assert all(targets.values()),targets
case_name=sys.argv[1] if len(sys.argv)>1 else 'original'
assert case_name in ('original','height','reference')
schedules={
 'original':STATES,
 'height':[*STATES[:3],('height',STATES[3][1].replace(b'height:600pt',b'height:440pt',1),None)],
 'reference':[*STATES[:3],('reference',STATES[3][1].replace(b'See @s0.',b'See @s13.',1),None)],
}
states=schedules[case_name]
case=WORK/f'checked-{case_name}'
(case/'edits').mkdir(parents=True)
for n,(_,data,leaf) in enumerate(states):
    assert leaf is None
    p=case/('main.typ' if n==0 else f'edits/{n}.typ')
    p.write_bytes(data)
(case/'addresses.json').write_text(json.dumps(targets))
env=cycle.environment(case,'candidate',False)
env.update(TEKT_LAB_MEMO_PROTECT='1',TEKT_LAB_MEMO_BYTES=str(8*1024*1024))
with (case/'gdb.log').open('wb') as log:
    subprocess.run(['taskset','-c','1','gdb','-q','-nx','-batch','-x',str(D/'checkpoint.gdb'),
                    '--args',str(arm),'watch','main.typ','out.pdf','--root',str(case),'--pdf-tagged=true'],
                   cwd=case,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=120,check=True)
for n,(name,_,_) in enumerate(states):
    if n<3 or case_name=='original':
        assert cycle.cycle.key('candidate',case/f'publication-{n}.pdf')==golden('candidate',name)
for name in ('counts.json','frontiers.json','addresses.json','snapshots.json','gdb.log'):
    (OUT/f'{case_name}-{name}').write_bytes((case/name).read_bytes())
