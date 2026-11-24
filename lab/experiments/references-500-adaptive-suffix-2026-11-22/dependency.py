"""Verify external suffix changes against the same ELF with reuse disabled."""
import json
import os
from pathlib import Path
import sys

here = Path(__file__).resolve().parent
sys.path.insert(0, str(here.parent / 'references-500-visible-eval-residual-2026-11-21'))
from watch import EXES, STATES, WORK, cycle, run

sys.path.insert(0, str(here.parent / 'references-500-visible-stable-cids-2026-11-21'))
from pdf_check import render, structure, text_layout

EXES['candidate'] = here / 'candidate.elf'
main = STATES[-1][1] + b'\n#include "middle.typ"\n'
leaf = b'= Original included suffix\nOriginal dependency.\n'
edit = b'= Changed included suffix\nUpdated dependency.\n'
states = (('open', main, leaf), ('changed', main, edit), ('restored', main, leaf))
out = Path(os.environ['TEKT_LAB_TIMING_OUT'])
proof = {}
for sequence in (states[:1], states[:2], states):
    name = sequence[-1][0]
    rows = {}
    for i, mode in enumerate(('off', 'adaptive')):
        rows[mode] = run(i, 'candidate', 0, states=sequence, check=False,
                         overrides={'TEKT_LAB_SUFFIX_MODE': mode, 'TEKT_LAB_MEMO_PROTECT': '1',
                                    'TEKT_LAB_MEMO_BYTES': str(8 * 1024 * 1024)},
                         directory=f'dependency-semantic-{name}')
    a, b = [WORK / rows[mode]['directory'] / 'out.pdf' for mode in ('off', 'adaptive')]
    describe = cycle.cycle.original.compare.describe
    assert describe(a) == describe(b), name
    assert structure(a) == structure(b), name
    assert text_layout(a) == text_layout(b), name
    rasters = render(a, a.parent, 'off')
    assert rasters == render(b, b.parent, 'adaptive'), name
    if name == 'open':
        opened = (describe(a), structure(a), text_layout(a), rasters)
    elif name == 'changed':
        assert text_layout(a) != opened[2], name
    else:
        assert (describe(a), structure(a), text_layout(a), rasters) == opened, name
    proof[name] = {'equal': True, 'rasters': rasters, 'watch': rows}
    print(name, 'ok', flush=True)
(out / 'dependency.json').write_text(json.dumps(proof, indent=2) + '\n')
