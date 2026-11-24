"""Compare probe and previous ELF on height, outline, reference, suffix."""
import json
import os
from pathlib import Path
import shutil
import sys

here = Path(__file__).resolve().parent
root = here.parents[2]
sys.path.insert(0, str(root / 'lab/experiments/watch-pdf-byte-retention-2026-11-22'))
import run

observe = run.observe
work = Path(os.environ['TEKT_LAB_WORK'])
out = Path(os.environ['TEKT_RESULT_DIR'])
observe.EXES = {'old': root / 'lab/experiments/watch-pdf-byte-retention-2026-11-22/candidate-binary',
                'probe': here / 'candidate-binary'}
observe.ARMS = tuple(observe.EXES)
def environment(case):
    env = run.old_env(case)
    env['TEKT_LAB_WATCH_RETAIN_BYTES'] = '1'
    return env
observe.env = environment
original = observe.STATES
visible = original[-1][1]
leaf = observe.cycle.cycle.opening.LEAF
leaf_edit = observe.cycle.cycle.opening.LEAF_EDIT
cases = {
    'height': [('open', visible, None), ('visible', visible.replace(b'height:600pt', b'height:440pt'), None)],
    'summary': [('open', visible, None), ('visible', visible.replace(b'#outline()', b'#outline(title: [Probe contents])', 1), None)],
    'reference': [('open', visible, None), ('visible', visible.replace(b'See @s0.', b'See @s13.', 1), None)],
    'suffix': [('open', b'#include "middle.typ"\n' + visible, leaf),
               ('changed', b'#include "middle.typ"\n' + visible, leaf_edit),
               ('visible', b'#include "middle.typ"\n' + visible, leaf)],
}
result = {}
for name, states in cases.items():
    observe.STATES = states
    result[name] = {}
    for arm in observe.ARMS:
        directory = work / f'directed-{name}/00-{arm}'
        if directory.exists():
            shutil.rmtree(directory)
        row = observe.run(0, arm, area=f'directed-{name}', keep=True)
        result[name][arm] = {
            state: observe.describe(directory / f'{state}.pdf')
            for state, _, _ in states
        }
    assert result[name]['old'] == result[name]['probe'], name
    print(name, flush=True)
observe.STATES = original
(out / 'directed.json').write_text(json.dumps(result, indent=2) + '\n')
