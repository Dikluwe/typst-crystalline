"""Balanced paired CPU measurements on independent document fixtures."""
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
from statistics import median
import subprocess
import sys
import time

here = Path(__file__).resolve().parent
sys.path.insert(0, str(here.parent / 'references-500-visible-eval-residual-2026-11-21'))
from watch import EXES, WORK, cycle, driver, measure
from fixtures import DOCUMENTS

OUT = Path(os.environ['TEKT_LAB_TIMING_OUT'])
EXES['candidate'] = here / 'candidate.elf'
EXES['vanilla'] = here.parents[2] / 'lab/typst-original/target/release/typst'
witnesses = json.loads((OUT / 'census.json').read_text())
ARMS = ('off', 'documents', 'vanilla')
rng = Random(20261122)

def estimate(values):
    draws = sorted(median(rng.choices(values, k=len(values))) for _ in range(10000))
    return {'median': median(values), 'ci95': [draws[249], draws[9749]]}

def vanilla_witness(name, states):
    products = {}
    for state, data, leaf in states:
        case = WORK / 'fresh' / name / state
        case.mkdir(parents=True)
        cycle.base.runner.setup_sources(case, data, leaf)
        environment = cycle.environment(case, 'vanilla', False)
        subprocess.run(['taskset', '-c', '1', str(EXES['vanilla']), 'compile',
                        'main.typ', 'out.pdf', '--root', str(case), '--pdf-tagged=true'],
                       cwd=case, env=environment, check=True, timeout=60,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        products[state] = cycle.cycle.key('vanilla', case / 'out.pdf')
    return products

def run_document(name, serial, arm, states, options, checks):
    engine = 'vanilla' if arm == 'vanilla' else 'candidate'
    for attempt in range(3):
        case = WORK / name / f'{serial:03d}-{arm}-{attempt}'
        case.mkdir(parents=True)
        source, pdf = case / 'main.typ', case / 'out.pdf'
        tmp = case / 'tmp'
        tmp.mkdir()
        cycle.base.runner.setup_sources(case, *states[0][1:])
        env = cycle.environment(tmp, engine, False)
        env.update(options or {})
        command = [str(EXES[engine]), 'watch', str(source), str(pdf),
                   '--root', str(case), '--pdf-tagged=true']
        hashes = {}
        windows = {}
        logfile = case / 'watch.log'
        try:
            with logfile.open('wb') as log:
                child = driver.Child(command, case, env, 1, log)
                try:
                    publication = None
                    for i, (state, data, leaf) in enumerate(states):
                        if i:
                            # Separate consecutive events and let the watcher return to its loop.
                            # This wait is outside the CPU interval of the next publication.
                            time.sleep(0.20)
                        before = child.cpu() if i else 0
                        if i:
                            cycle.base.runner.setup_sources(case, data, leaf)
                        publication, _, _ = driver.await_pdf(child, pdf, publication)
                        windows[state] = measure.settle(child) - before
                        hashes[state] = cycle.cycle.key(engine, pdf)
                    until = time.monotonic() + 1
                    while logfile.read_text().count('compiled successfully') < len(states):
                        if time.monotonic() >= until:
                            raise TimeoutError('publication log did not settle')
                        time.sleep(0.01)
                finally:
                    child.stop()
                    child.close()
            if logfile.read_text().count('compiled successfully') == len(states) and hashes == checks:
                return {'cpu_ns': windows['visible'], 'states_cpu_ns': windows, 'all_pdfs': hashes,
                        'directory': str(case.relative_to(WORK)), 'exe': str(EXES[engine]),
                        'utc': measure.utc(), 'attempt': attempt}
        except TimeoutError:
            pass
        print('retry', name, arm, serial, attempt + 1, flush=True)
    raise AssertionError((name, serial, arm, hashes, checks))


os.sched_setaffinity(0, {2})
summary = {}
with (OUT / 'measurements.jsonl').open('w') as log:
    for name, states in DOCUMENTS.items():
        checks = witnesses[name]['normalized_pdf_hashes'] | {'vanilla': vanilla_witness(name, states)}
        rows = []
        for group in range(2):
            orders = list(permutations(ARMS))
            rng.shuffle(orders)
            for order in orders:
                block = len(rows)
                paired = {}
                for i, arm in enumerate(order):
                    engine = 'vanilla' if arm == 'vanilla' else 'candidate'
                    options = None if arm == 'vanilla' else {
                        'TEKT_LAB_SUFFIX_MODE': arm, 'TEKT_LAB_MEMO_PROTECT': '1',
                        'TEKT_LAB_MEMO_BYTES': str(8 * 1024 * 1024),
                    }
                    row = run_document(name, 3 * block + i, arm, states, options, checks[arm])
                    paired[arm] = row['cpu_ns'] / 1e6
                    print(json.dumps(row | {'document': name, 'block': block, 'mode': arm}),
                          file=log, flush=True)
                rows.append(paired)
                print(name, block + 1, '/ 12', flush=True)
        summary[name] = {
            'cpu_ms': {arm: estimate([row[arm] for row in rows]) for arm in ARMS},
            'documents_minus_ms': {
                arm: estimate([row['documents'] - row[arm] for row in rows])
                for arm in ARMS if arm != 'documents'
            },
            'saving_vs_off_percent': estimate([100 * (1 - row['documents'] / row['off']) for row in rows]),
            'paired_ratio_over_vanilla': estimate([row['documents'] / row['vanilla'] for row in rows]),
        }
        (OUT / 'timing.json').write_text(json.dumps(summary, indent=2) + '\n')
