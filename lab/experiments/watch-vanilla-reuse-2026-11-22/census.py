"""Count actual physical and index entries for on/off watch runs."""
import json
import os
from pathlib import Path
import subprocess

import watch_observe as observe

here = Path(__file__).resolve().parent
work = Path(os.environ['TEKT_LAB_WORK'])
out = Path(os.environ['TEKT_RESULT_DIR'])
exe = here / 'candidate-binary'
patterns = {
    'wait': '::watch::wait_for_change_since',
    'paged': '::compile_paged_in_session',
    'physical_candidate': '::Session>::physical_candidate',
    'physical_index': '::Session>::index_physical_steps',
    'planned_layout': '::compiler::layout::convergence::run_planned',
    'line_commit': '::flush_line_with_outer_singleton',
}
addresses = {name: [] for name in patterns}
for row in (out / 'candidate.symbols').read_text().splitlines():
    fields = row.split(maxsplit=3)
    if len(fields) != 4:
        continue
    symbol = fields[3].split('.llvm.', 1)[0]
    for name, pattern in patterns.items():
        if symbol.endswith(pattern):
            addresses[name].append(int(fields[0], 16))
assert len(addresses['wait']) == 1, addresses['wait']
for arm in ('off', 'retain'):
    case = work / 'census' / arm
    (case / 'edits').mkdir(parents=True, exist_ok=True)
    (case / 'addresses.json').write_text(json.dumps(addresses, indent=2))
    for index, (_, data, leaf) in enumerate(observe.STATES):
        assert leaf is None
        (case / ('main.typ' if index == 0 else f'edits/{index}.typ')).write_bytes(data)
    env = observe.env(case)
    if arm == 'retain':
        env['TEKT_LAB_WATCH_RETAIN'] = '1'
        env['TEKT_LAB_WATCH_RETAIN_TRACE'] = '1'
    with (out / f'census-{arm}.log').open('wb') as log:
        subprocess.run(['taskset', '-c', '1', 'gdb', '-q', '-nx', '-batch', '-x', str(here / 'census.gdb'),
                        '--args', str(exe), 'watch', 'main.typ', 'out.pdf', '--root', str(case),
                        '--pdf-tagged=true'], cwd=case, env=env, stdout=log,
                       stderr=subprocess.STDOUT, timeout=120, check=True)
    print(arm, (case / 'counts.json').read_text(), flush=True)
