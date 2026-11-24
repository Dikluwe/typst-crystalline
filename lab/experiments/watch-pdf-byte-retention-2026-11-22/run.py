"""Isolate terminal-document reuse with one ELF, then compare fixed references."""
import json
import os
from pathlib import Path
from random import Random
import shutil
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(ROOT / 'lab/experiments/physical-order-2026-11-21'))
import watch_observe as observe

WORK = Path(os.environ['TEKT_LAB_WORK'])
OUT = Path(os.environ['TEKT_RESULT_DIR'])
EXE = HERE / 'candidate-binary'
observe.EXES = {
    'off': EXE,
    'doc': EXE,
    'bytes': EXE,
    'baseline': Path(os.environ['TEKT_BASELINE_EXE']),
    'vanilla': ROOT / 'lab/typst-original/target/release/typst',
}
observe.ARMS = tuple(observe.EXES)
old_env = observe.env


def environment(case):
    result = old_env(case)
    if case.name.endswith('-doc') or '/fresh/doc/' in str(case):
        result['TEKT_LAB_WATCH_RETAIN'] = '1'
        result['TEKT_LAB_WATCH_RETAIN_TRACE'] = '1'
    if case.name.endswith('-bytes') or '/fresh/bytes/' in str(case):
        result['TEKT_LAB_WATCH_RETAIN_BYTES'] = '1'
        result['TEKT_LAB_WATCH_RETAIN_TRACE'] = '1'
    return result


observe.env = environment


def collect_directed():
    original = observe.STATES
    visible = original[-1][1]
    leaf = observe.cycle.cycle.opening.LEAF
    leaf_edit = observe.cycle.cycle.opening.LEAF_EDIT
    cases = {
        'height': [('open', visible, None),
                   ('visible', visible.replace(b'height:600pt', b'height:440pt'), None)],
        'summary': [('open', visible, None),
                    ('visible', visible.replace(b'#outline()', b'#outline(title: [Probe contents])', 1), None)],
        'reference': [('open', visible, None),
                      ('visible', visible.replace(b'See @s0.', b'See @s13.', 1), None)],
        'suffix': [('open', b'#include "middle.typ"\n' + visible, leaf),
                   ('changed', b'#include "middle.typ"\n' + visible, leaf_edit),
                   ('visible', b'#include "middle.typ"\n' + visible, leaf)],
    }
    selected = os.environ.get('TEKT_CASES')
    if selected:
        cases = {name: states for name, states in cases.items() if name in selected.split(',')}
    result = json.loads((OUT / 'directed.json').read_text()) if (OUT / 'directed.json').exists() else {}
    for name, states in cases.items():
        observe.STATES = states
        result[name] = {}
        for arm in observe.ARMS:
            for attempt in range(3):
                area = f'directed-{name}' if attempt == 0 else f'directed-{name}-retry-{attempt}'
                directory = WORK / area / f'00-{arm}'
                if directory.exists():
                    shutil.rmtree(directory)
                try:
                    row = observe.run(0, arm, area=area, keep=True)
                    break
                except (TimeoutError, observe.IncompleteWatch):
                    if attempt == 2:
                        raise
            directory = WORK / row['directory']
            result[name][arm] = {
                state: observe.describe(directory / f'{state}.pdf')
                for state, _, _ in states
            }
        for state, _, _ in states:
            assert result[name]['off'][state] == result[name]['doc'][state] == result[name]['bytes'][state]
            assert result[name]['bytes'][state] == result[name]['baseline'][state]
        print('directed', name, flush=True)
    observe.STATES = original
    observe.dump('directed.json', result)


def measure():
    rng = Random(20261122)
    paired = []
    orders = [list(observe.ARMS)[shift:] + list(observe.ARMS)[:shift]
              for shift in range(len(observe.ARMS))]
    with (OUT / 'paired.jsonl').open('w') as log:
        for block in range(24):
            pair = {}
            for arm in orders[block % len(orders)]:
                row = observe.run(block, arm, area='paired')
                pair[arm] = row
                print(json.dumps(row), file=log, flush=True)
            paired.append(pair)
            print('pair', block + 1, '/ 24', flush=True)
    summary = {}
    for state, _, _ in observe.STATES:
        arms = {arm: [pair[arm]['states'][state]['cpu_ns'] / 1e6 for pair in paired]
                for arm in observe.ARMS}
        summary[state] = {'cpu_ms': {arm: observe.statistics(values) for arm, values in arms.items()}}
        for arm in ('off', 'doc', 'baseline', 'vanilla'):
            gaps = [a-b for a,b in zip(arms['bytes'], arms[arm])]
            summary[state][f'bytes_minus_{arm}_ms'] = observe.statistics(gaps)
    observe.dump('timing.json', summary)


if __name__ == '__main__':
    if not (OUT / 'validation.json').exists():
        observe.validate()
    doc_trace = (WORK / 'witness/00-doc/watch.log').read_text()
    byte_trace = (WORK / 'witness/00-bytes/watch.log').read_text()
    assert doc_trace.count('watch-retain hit') == 2, doc_trace
    assert doc_trace.count('watch-retain skip changed semantic prefix') == 1, doc_trace
    assert byte_trace.count('watch-byte hit') == 2, byte_trace
    assert byte_trace.count('watch-byte skip changed semantic prefix') == 1, byte_trace
    collect_directed()
    measure()
