"""Sample the byte-cache visible fallback against the same ELF disabled."""
import json
import os
from pathlib import Path
from random import Random

import run

observe = run.observe
out = Path(os.environ['TEKT_RESULT_DIR'])
witness = json.loads((out / 'witness.json').read_text())
expected = {'off': witness['off'], 'bytes': witness['off']}
periods = [347000, 937000] * 12
Random(582220).shuffle(periods)
observe.driver.TIMEOUT = 8
os.sched_setaffinity(0, {2})
with (out / 'profile-visible.jsonl').open('w') as stream:
    for block, period in enumerate(periods):
        order = ('off', 'bytes') if block % 2 == 0 else ('bytes', 'off')
        for arm in order:
            for attempt in range(3):
                area = 'profile-visible' if attempt == 0 else f'profile-visible/retry-{attempt}'
                try:
                    row = observe.run(block, arm, period, area=area, expected=expected)
                    break
                except (observe.IncompleteWatch, TimeoutError):
                    if attempt == 2:
                        raise
            stream.write(json.dumps(row) + '\n')
            stream.flush()
        print(block + 1, '/ 24', flush=True)
