from pathlib import Path
import hashlib
import json
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

def execute(arm, case, states):
    expected={name:digest for digest,name in map(str.split,(OUT/'binaries.sha256').read_text().splitlines())}
    exe=WORK/arm
    assert hashlib.sha256(exe.read_bytes()).hexdigest()==expected[arm]
    names={'wait':'::watch::wait_for_change_since',
           'line_commit':'::flush_line_with_outer_singleton',
           'resume':'::lab_resume_sequence',
           'cut':'::lab_three_cut_receipt',
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
    assert addresses['wait'] and addresses['line_commit'] and addresses['region']
    assert addresses['resume'] and bool(addresses['cut'])==(arm=='candidate'),addresses
    addresses['node-compare']=[0x20dc408,0x20dc42c,0x20dc439,0x20dc44d,0x20dc470,0x20dc485,
                              0x20dc49f,0x20dc4c7,0x20dc4fb,0x20dc505,0x20dc521,0x20dc52b,
                              0x20dc540,0x20dc55c,0x20dc566,0x20dc57b,0x20dc590,0x20dc5b6]
    (case/'edits').mkdir(parents=True)
    for i,(_,data,leaf) in enumerate(states):
        assert leaf is None
        (case/('main.typ' if i==0 else f'edits/{i}.typ')).write_bytes(data)
    (case/'addresses.json').write_text(json.dumps(addresses,indent=2))
    env=cycle.environment(case,'candidate',False)
    env.update(options)
    with (case/'gdb.log').open('wb') as log:
        subprocess.run(['taskset','-c','1','gdb','-q','-nx','-batch','-x',str(D/'rebind.gdb'),
                        '--args',str(exe),'watch','main.typ','out.pdf','--root',str(case),'--pdf-tagged=true'],
                       cwd=case,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=180,check=True)
    counts=json.loads((case/'counts.json').read_text())
    events=json.loads((case/'resume.json').read_text())
    return counts,events

case=WORK/'rebind-terminal-diagnostic'
execute('candidate',case,STATES)
(OUT/'rebind-terminal.json').write_bytes((case/'trials.json').read_bytes())
