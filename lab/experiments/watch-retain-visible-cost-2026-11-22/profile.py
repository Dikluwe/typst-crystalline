"""Sample the visible-edit cost with one ELF and retention on/off."""
import json
import os
from pathlib import Path
from random import Random
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path.insert(0, str(ROOT / 'lab/experiments/watch-vanilla-reuse-2026-11-22'))
import watch_observe as observe

WORK = Path(os.environ['TEKT_LAB_WORK'])
OUT = Path(os.environ['TEKT_RESULT_DIR'])
EXE = ROOT / 'lab/experiments/watch-vanilla-reuse-2026-11-22/candidate-binary'
observe.EXES = {'off': EXE, 'retain': EXE}
old_env = observe.env


def environment(case):
    result = old_env(case)
    if case.name.endswith('-retain'):
        result['TEKT_LAB_WATCH_RETAIN'] = '1'
    return result


observe.env = environment
witness = json.loads((ROOT / 'lab/results/construction/diagnostics/watch-vanilla-reuse-2026-11-22/final/witness.json').read_text())
expected = {'off': witness['off'], 'retain': witness['off']}
periods = [347000, 937000] * 12
Random(582219).shuffle(periods)
observe.driver.TIMEOUT = 8
os.sched_setaffinity(0, {2})
with (OUT / 'profile.jsonl').open('w') as stream:
    for block, period in enumerate(periods):
        order = ('off', 'retain') if block % 2 == 0 else ('retain', 'off')
        for arm in order:
            for attempt in range(3):
                area = 'profile' if attempt == 0 else f'profile/retry-{attempt}'
                try:
                    row = observe.run(block, arm, period, area=area, expected=expected)
                    break
                except (observe.IncompleteWatch, TimeoutError):
                    if attempt == 2:
                        raise
            stream.write(json.dumps(row) + '\n')
            stream.flush()
        print(block + 1, '/ 24', flush=True)
