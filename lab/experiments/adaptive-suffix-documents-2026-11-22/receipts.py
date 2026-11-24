"""Record exact fixture hashes and the established references-500 navigation audit."""
from hashlib import sha256
import json
import os
from pathlib import Path
import sys

here = Path(__file__).resolve().parent
sys.path.insert(0, str(here.parent / 'references-500-visible-eval-residual-2026-11-21'))
from watch import STATES, WORK, cycle
from fixtures import DIRECTED

out = Path(os.environ['TEKT_LAB_TIMING_OUT'])
fixtures = {'references-500': STATES, **DIRECTED}
hashes = {case: {state: sha256(content).hexdigest() for state, content, _ in states}
          for case, states in fixtures.items()}
(out / 'fixture-hashes.json').write_text(json.dumps(hashes, indent=2) + '\n')
data = json.loads((out / 'census.json').read_text())['references-500']
old = cycle.cycle.original
nav = {}
for mode in ('off', 'documents'):
    pdf = WORK / data[mode]['pdf']
    nav[mode] = old.navigation(pdf, old.compare.describe(pdf), True)
(out / 'references-navigation.json').write_text(json.dumps(nav, indent=2) + '\n')
