"""Witness the actual dynamic cut and compare directed watch products."""
import json
import os
from pathlib import Path
import subprocess
import sys

here = Path(__file__).resolve().parent
sys.path.insert(0, str(here.parent / 'references-500-visible-eval-residual-2026-11-21'))
from watch import STATES, WORK, cycle

sys.path.insert(0, str(here.parent / 'references-500-visible-stable-cids-2026-11-21'))
from pdf_check import render, structure, text_layout

out = Path(os.environ['TEKT_LAB_TIMING_OUT'])
exe = here / 'candidate.elf'
symbols = {'wait': '::watch::wait_for_change_since',
           'line_commit': '::flush_line_with_outer_singleton',
           'cut': '::lab_three_cut_receipt',
           'planned': '::convergence::run_planned_lab',
           'index': '::Session>::index_physical_steps'}
addresses = {key: [] for key in symbols}
for row in subprocess.check_output(['nm', '-n', '-S', '-C', '--defined-only', exe], text=True).splitlines():
    cols = row.split(maxsplit=3)
    if len(cols) == 4 and cols[2] in ('t', 'T'):
        symbol = cols[3].split('.llvm.', 1)[0]
        for key, tail in symbols.items():
            if symbol.endswith(tail):
                addresses[key].append(int(cols[0], 16))
assert all(addresses.values()), addresses

def edit_section(section):
    data = STATES[0][1]
    start = data.index(f'= Section {section} <s{section}>'.encode())
    before, word, after = data[start:].partition(b'Paragraph')
    assert word
    return data[:start] + before + b'Visible paragraph' + after

cases = {
    'original': STATES[-1][1],
    'section-5': edit_section(5),
    'section-12': edit_section(12),
    'height': STATES[-1][1].replace(b'height:600pt', b'height:440pt', 1),
    'outline': STATES[-1][1].replace(b'#outline()', b'#outline(title: [Probe contents])', 1),
    'reference': STATES[-1][1].replace(b'See @s19.', b'See @s13.', 1),
}

def witness(name, mode, data):
    case = WORK / f'{name}-{mode}'
    (case / 'edits').mkdir(parents=True)
    (case / 'addresses.json').write_text(json.dumps(addresses))
    for idx, (_, source, leaf) in enumerate([*STATES[:3], ('edit', data, None)]):
        assert leaf is None
        (case / ('main.typ' if not idx else f'edits/{idx}.typ')).write_bytes(source)
    env = cycle.environment(case, 'candidate', False)
    env.update(TEKT_LAB_SUFFIX_MODE=mode, TEKT_LAB_MEMO_PROTECT='1',
               TEKT_LAB_MEMO_BYTES=str(8 * 1024 * 1024))
    with (case / 'gdb.log').open('wb') as log:
        subprocess.run(['taskset', '-c', '1', 'gdb', '-q', '-nx', '-batch', '-x', str(here/'census.gdb'),
                        '--args', str(exe), 'watch', 'main.typ', 'out.pdf', '--root', str(case),
                        '--pdf-tagged=true'], cwd=case, env=env, stdout=log,
                       stderr=subprocess.STDOUT, timeout=120, check=True)
    return case, {'counts': json.loads((case / 'counts.json').read_text()),
                  'trials': json.loads((case / 'trials.json').read_text())}

describe = cycle.cycle.original.compare.describe
results = {}
for name, data in cases.items():
    results[name] = {}
    for mode in ('off', 'adaptive', 'fixed') if name == 'original' else ('off', 'adaptive'):
        directory, row = witness(name, mode, data)
        results[name][mode] = row
        row['pdf'] = str((directory / 'publication-3.pdf').relative_to(WORK))
        print(name, mode, row['counts'], [t for t in row['trials'] if t['publication'] == 3 and t['reason'] == 0], flush=True)
    (out / 'census.json').write_text(json.dumps(results, indent=2) + '\n')
    control = WORK / results[name]['off']['pdf']
    candidate = WORK / results[name]['adaptive']['pdf']
    assert describe(control) == describe(candidate), (name, 'describe')
    assert structure(control) == structure(candidate), (name, 'structure')
    assert text_layout(control) == text_layout(candidate), (name, 'layout')
    directory = candidate.parent
    assert render(control, directory, 'off') == render(candidate, directory, 'adaptive'), (name, 'raster')
    results[name]['equal'] = True
    (out / 'census.json').write_text(json.dumps(results, indent=2) + '\n')
