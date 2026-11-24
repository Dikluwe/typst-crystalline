"""Watch transitions for geometry, outline, reference targets and suffix inputs."""
import json
import os
import shutil
from pathlib import Path

import observe

WORK = Path(os.environ['TEKT_LAB_WORK'])
OUT = Path(os.environ['TEKT_RESULT_DIR'])
observe.EXES['published'] = Path(os.environ['TEKT_PUBLISHED_EXE'])
visible = observe.STATES[-1][1]
opening = observe.cycle.cycle.opening
leaf = opening.LEAF
leaf_edit = opening.LEAF_EDIT

cases = {
    'watch': observe.STATES,
    'height': [('open', visible, None),
               ('visible', visible.replace(b'height:600pt', b'height:440pt'), None)],
    'summary': [('open', visible, None),
                ('visible', visible.replace(b'#outline()',
                                            b'#outline(title: [Probe contents])', 1), None)],
    'reference': [('open', visible, None),
                  ('visible', visible.replace(b'See @s0.', b'See @s13.', 1), None)],
    'suffix': [('open', b'#include "middle.typ"\n' + visible, leaf),
               ('changed', b'#include "middle.typ"\n' + visible, leaf_edit),
               ('visible', b'#include "middle.typ"\n' + visible, leaf)],
}
selected = os.environ.get('TEKT_CASES')
if selected:
    wanted = set(selected.split(','))
    cases = {name: states for name, states in cases.items() if name in wanted}
original = observe.STATES
report = json.loads((OUT/'directed.json').read_text()) if selected and (OUT/'directed.json').exists() else {}
for case_name, states in cases.items():
    observe.STATES = states
    report[case_name] = {}
    for arm in ('tekt', 'published', 'vanilla'):
        previous = WORK / f'directed/{case_name}/00-{arm}'
        if previous.exists():
            shutil.rmtree(previous)
        row = observe.run(0, arm, area=f'directed/{case_name}', keep=True)
        directory = WORK / row['directory']
        publications = {}
        for name, _, _ in states:
            pdf = directory / f'{name}.pdf'
            description = observe.describe(pdf)
            try:
                navigation = observe.cycle.cycle.original.navigation(
                    pdf, description, name == 'visible')
            except AssertionError:
                navigation = None
            publications[name] = {
                'cpu_ms': row['states'][name]['cpu_ns'] / 1e6,
                'description': description,
                'navigation': navigation,
            }
        report[case_name][arm] = publications
    for name, _, _ in states:
        report[case_name]['tekt'][name]['same_as_published'] = (
            report[case_name]['tekt'][name]['description'] ==
            report[case_name]['published'][name]['description'])
    print(case_name, 'done', flush=True)
observe.STATES = original
(OUT / 'directed.json').write_text(json.dumps(report, indent=2) + '\n')
