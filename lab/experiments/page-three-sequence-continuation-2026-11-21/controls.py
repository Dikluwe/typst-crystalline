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
           'trial':'::lab_suffix_trial_receipt',
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
    assert bool(addresses['resume'])==bool(addresses['trial'])==(arm=='candidate'),addresses
    (case/'edits').mkdir(parents=True)
    for i,(_,data,leaf) in enumerate(states):
        assert leaf is None
        (case/('main.typ' if i==0 else f'edits/{i}.typ')).write_bytes(data)
    (case/'addresses.json').write_text(json.dumps(addresses,indent=2))
    env=cycle.environment(case,'candidate',False)
    env.update(options)
    with (case/'gdb.log').open('wb') as log:
        subprocess.run(['taskset','-c','1','gdb','-q','-nx','-batch','-x',str(D/'census.gdb'),
                        '--args',str(exe),'watch','main.typ','out.pdf','--root',str(case),'--pdf-tagged=true'],
                       cwd=case,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=180,check=True)
    counts=json.loads((case/'counts.json').read_text())
    events=json.loads((case/'resume.json').read_text())
    return counts,events

results={}
for arm in ('control','candidate'):
    case=WORK/f'census-{arm}'
    counts,events=execute(arm,case,STATES)
    for n,(name,_,_) in enumerate(STATES):
        assert cycle.cycle.key('candidate',case/f'publication-{n}.pdf')==golden('candidate',name)
    results[arm]={'counts':counts,'events':events}
    print(arm,counts,flush=True)
(OUT/'census.json').write_text(json.dumps(results,indent=2)+'\n')
assert results['candidate']['counts']['0:resume']==results['candidate']['counts']['3:resume']==1
for event in results['candidate']['events']:
    assert event['line_commits_so_far']==57
    assert event['region_transitions_so_far']==2
    assert 'sequence::lab_resume_sequence' in event['stack']
    assert 'sequence::layout_parts' in event['stack']
    for scope in ('paragraph_emit::emit_impl','paragraph_plan::try_parts_impl','sequence::try_source_segment'):
        assert scope not in event['stack'], scope
trial=json.loads((WORK/'census-candidate'/'trials.json').read_text())
assert len(trial)==2 and trial[0]['publication']==0 and trial[1]['publication']==3
assert trial[1]['prior_pages']==23,trial
(OUT/'trials.json').write_text(json.dumps(trial,indent=2)+'\n')
print('suffix trials',trial,flush=True)

def fresh(case,data):
    case=case/'fresh'
    case.mkdir()
    (case/'main.typ').write_bytes(data)
    env=cycle.environment(case,'candidate',False)
    output=case/'cold.pdf'
    with (case/'compile.log').open('wb') as log:
        subprocess.run(['taskset','-c','1',str(WORK/'control'),'compile',str(case/'main.typ'),
                        str(output),'--root',str(case),'--pdf-tagged=true'],
                       env=env,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=60)
    return output

directed={}
before=describe(WORK/'census-candidate'/'publication-0.pdf')
for label, data in (
    ('visible',STATES[3][1]),
    ('height',STATES[3][1].replace(b'height:600pt',b'height:440pt',1)),
    ('reference',STATES[3][1].replace(b'See @s0.',b'See @s13.',1)),
):
    if label=='visible':
        case=WORK/'census-candidate'
        counts=results['candidate']['counts']
    else:
        case=WORK/'directed-census'/label
        counts,_=execute('candidate',case,[*STATES[:3],(label,data,None)])
    warm=case/'publication-3.pdf'
    cold=fresh(case,data)
    a,b=describe(warm),describe(cold)
    assert a==b and structure(warm)==structure(cold) and text_layout(warm)==text_layout(cold),label
    refs=lambda result:[[x.get('/Dest') or x.get('/A',{}).get('/D') for x in page] for page in result['links']]
    same_targets=refs(a)==refs(before)
    expected_pages=33 if label=='height' else 23
    assert a['page_count']==expected_pages,label
    if label=='reference':
        assert not same_targets
    directed[label]={'cold_equal':True,'pages':expected_pages,'links':sum(map(len,a['links'])),
                     'same_targets_as_open':same_targets,'counts':counts,
                     'trials':json.loads((case/'trials.json').read_text())}
    (OUT/'directed.json').write_text(json.dumps(directed,indent=2)+'\n')
    print(label,directed[label],flush=True)
