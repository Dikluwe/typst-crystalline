"""Sample evaluation CPU in paired watch sessions of two Tekt binaries."""
import json
import os
from pathlib import Path
from random import Random

import sys
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'physical-order-2026-11-21'))
import observe

out = Path(os.environ['TEKT_RESULT_DIR'])
observe.EXES['published'] = Path(os.environ['TEKT_PUBLISHED_EXE'])
witness = json.loads((out / 'witness.json').read_text())['tekt']
expected = {'tekt': witness, 'published': witness}
periods = [347000, 937000] * 12
Random(482911).shuffle(periods)
observe.driver.TIMEOUT = 8
os.sched_setaffinity(0, {2})
with (out / 'profile-paired.jsonl').open('w') as stream:
    for block, period in enumerate(periods):
        arms = ('tekt', 'published') if block % 2 == 0 else ('published', 'tekt')
        for arm in arms:
            for retry in range(3):
                try:
                    area = 'profile-paired' if retry == 0 else f'profile-paired/retry-{retry}'
                    row = observe.run(block, arm, period, area=area, expected=expected)
                    break
                except (observe.IncompleteWatch, TimeoutError):
                    if retry == 2:
                        raise
            stream.write(json.dumps(row) + '\n')
            stream.flush()
        print(block + 1, '/', len(periods), flush=True)
