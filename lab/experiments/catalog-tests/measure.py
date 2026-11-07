#!/usr/bin/env python3
"""C-only terminating CLI/startup calibration driver; all transient output stays in task RAM.

Example: measure.py --mode timing --rounds 5 --label joint -- BINARY compile INPUT OUTPUT
This driver is CLI-only; use an external watcher driver for watch.
Timing mode pairs CAT_PROBE off/on in shuffled order for the *same executable*.
Run a native original separately with --mode baseline. Run detail separately with
--mode detail: its durations are deliberately omitted. JSON goes to stdout only.
Wall includes process startup, output capture, and the probe's end-of-cycle flush;
phase ns exclude that flush. Child CPU includes the full process. Inclusive phase
clocks must never be summed as disjoint components.
"""
import argparse
import json
import os
from pathlib import Path
import random
import resource
import subprocess
import time

RAM = Path('/dev/shm/tekt-catalog-tests-20261105')


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--mode', choices=('baseline', 'timing', 'detail'), required=True)
    p.add_argument('--rounds', type=int, default=5)
    p.add_argument('--label', required=True)
    p.add_argument('command', nargs=argparse.REMAINDER)
    args = p.parse_args()
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    if not command or not 1 <= args.rounds <= 1000:
        p.error('Supply a command and 1..1000 rounds')
    if not args.label or any(c not in 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_' for c in args.label):
        p.error('Label must contain only letters/digits/-/_')
    if not RAM.is_dir():
        p.error('Task RAM directory does not exist; no disk fallback')
    output = RAM / ('measure-' + args.label)
    output.mkdir(exist_ok=False)
    rng = random.Random(20261105)
    rows = []
    for round_id in range(args.rounds):
        states = ['off', 'on'] if args.mode == 'timing' else ['on' if args.mode == 'detail' else 'off']
        rng.shuffle(states)
        for state in states:
            stem = output / f'{round_id:03d}-{state}'
            env = os.environ.copy()
            env['CAT_PROBE'] = state
            env['CAT_PROBE_OUT'] = str(stem.with_suffix('.tsv'))
            before = resource.getrusage(resource.RUSAGE_CHILDREN)
            start = time.perf_counter_ns()
            result = subprocess.run(command, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            wall = time.perf_counter_ns() - start
            after = resource.getrusage(resource.RUSAGE_CHILDREN)
            stem.with_suffix('.stdout').write_bytes(result.stdout)
            stem.with_suffix('.stderr').write_bytes(result.stderr)
            row = dict(round=round_id, state=state, returncode=result.returncode)
            if args.mode != 'detail':
                row.update(wall_ns=wall,
                           child_user_s=after.ru_utime - before.ru_utime,
                           child_system_s=after.ru_stime - before.ru_stime)
            probe_file = stem.with_suffix('.tsv')
            row['aggregate'] = str(probe_file) if probe_file.exists() else None
            rows.append(row)
            if result.returncode:
                print(json.dumps(dict(command=command, mode=args.mode, rows=rows)))
                raise SystemExit(result.returncode)
    print(json.dumps(dict(command=command, mode=args.mode, rows=rows)))


if __name__ == '__main__':
    main()
