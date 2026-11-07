#!/usr/bin/env python3
"""Check completed paired measurement CSVs and independently inspect saved PDFs.

No driver import. Marker checks run after all timed child intervals.
"""
import argparse
import csv
import math
from collections import defaultdict
from pathlib import Path, PurePosixPath
import re
import subprocess

FIELDS = {'round', 'repeat', 'variant', 'fixture', 'mode', 'process_id', 'case', 'cycle',
          'sample_epoch', 'wall_ns', 'cpu_ns', 'process_cpu_ns', 'process_user_ns',
          'process_system_ns', 'process_peak_rss_bytes', 'exit_code'}
CPU_TOLERANCE_NS = 1_000_000
TASK_RAM = Path('/dev/shm/tekt-outline-warm-20261106')


def require(condition, message):
    if not condition: raise ValueError(message)


def natural(raw, label, positive=False):
    require(raw.isascii() and raw.isdecimal(), f'invalid {label}: {raw!r}')
    value = int(raw)
    require(not positive or value > 0, f'{label} must be positive')
    return value


def expected_marker(cycle):
    # startup0, same0, edit1, same1, edit0, same0, edit1, ...
    return f'TEKT_EPOCH_{(cycle // 2) % 2}'


def snapshot_name(mode, cycle):
    return f'{mode}.pdf' if cycle == 0 else f'cycle-{cycle:03d}-{mode}.pdf'


def parse_measurements(text, cycles=6, expected_variants=None):
    require(isinstance(cycles, int) and cycles > 0, 'cycles must be positive')
    reader = csv.DictReader(text.splitlines())
    require(reader.fieldnames is not None and len(reader.fieldnames) == len(FIELDS)
            and set(reader.fieldnames) == FIELDS, f'CSV must have exactly columns {sorted(FIELDS)}')
    processes = defaultdict(list)
    used_cases = {}
    for number, raw in enumerate(reader, 2):
        require(None not in raw and all(value is not None for value in raw.values()), f'CSV row {number} malformed')
        row = dict(raw)
        for label in ('round', 'repeat', 'process_id', 'cycle', 'wall_ns', 'cpu_ns', 'process_cpu_ns',
                      'process_user_ns', 'process_system_ns', 'process_peak_rss_bytes'):
            row[label] = natural(row[label], label, label in ('process_id', 'wall_ns', 'process_peak_rss_bytes'))
        for label in ('variant', 'case'):
            require(re.fullmatch(r'[A-Za-z0-9_.-]+', row[label]) is not None and row[label] not in ('.', '..'), f'invalid {label}')
        fixture = PurePosixPath(row['fixture'])
        require(row['fixture'] and not fixture.is_absolute() and '..' not in fixture.parts and fixture != PurePosixPath('.'), 'fixture path escapes its tree')
        require(row['mode'] in ('cli', 'startup', 'same_bytes', 'main_edit'), f'unknown mode {row["mode"]}')
        try:
            epoch = float(row['sample_epoch'])
        except ValueError as error:
            raise ValueError('invalid sample_epoch') from error
        require(math.isfinite(epoch) and epoch > 0, 'sample_epoch must be a finite positive timestamp')
        row['sample_epoch'] = epoch
        require(row['exit_code'] in ('0', '-15'), 'unexpected child termination')
        row['exit_code'] = int(row['exit_code'])
        require(row['process_cpu_ns'] == row['process_user_ns'] + row['process_system_ns'], 'wait4 CPU components disagree')
        require(row['cpu_ns'] <= row['wall_ns'] + CPU_TOLERANCE_NS, 'single-CPU interval exceeds wall time by more than 1 ms')
        require(row['cpu_ns'] <= row['process_cpu_ns'] + CPU_TOLERANCE_NS, 'interval CPU exceeds final process total')
        pid = row['process_id']
        require(row['case'] not in used_cases or used_cases[row['case']] == pid, 'case belongs to more than one process')
        used_cases[row['case']] = pid
        processes[pid].append(row)
    require(processes, 'empty CSV evidence')
    pairs = defaultdict(dict)
    rounds = set(); repeats = set(); fixtures = set(); variants = set()
    for pid, rows in processes.items():
        fixed = ['round', 'repeat', 'variant', 'fixture', 'case', 'process_cpu_ns', 'process_user_ns',
                 'process_system_ns', 'process_peak_rss_bytes', 'exit_code']
        require(all(all(row[key] == rows[0][key] for key in fixed) for row in rows), f'pid={pid}: process metadata changed')
        first = rows[0]
        cli = first['mode'] == 'cli'
        expected = [(0, 'cli')] if cli else [(0, 'startup')] + [
            (index, 'same_bytes' if index % 2 else 'main_edit') for index in range(1, 2 * cycles + 1)]
        require([(row['cycle'], row['mode']) for row in rows] == expected,
                f'pid={pid}: missing/reordered/duplicated cycle or wrong mode')
        if cli:
            require(first['exit_code'] == 0, 'CLI failed')
            require(first['cpu_ns'] == first['process_cpu_ns'], 'CLI interval must equal final wait4 CPU')
        else:
            require(sum(row['cpu_ns'] for row in rows) <= first['process_cpu_ns'] + CPU_TOLERANCE_NS,
                    f'pid={pid}: cycle CPU sum exceeds final wait4 total by more than 1 ms')
        kind = 'cli' if cli else 'watch'
        identity = tuple(first[key] for key in ('round', 'repeat', 'variant', 'fixture'))
        require(kind not in pairs[identity], f'duplicate {kind} process for {identity}')
        pairs[identity][kind] = rows
        rounds.add(first['round']); repeats.add(first['repeat']); variants.add(first['variant']); fixtures.add(first['fixture'])
    require(rounds == set(range(max(rounds) + 1)) and repeats == set(range(max(repeats) + 1)), 'round/repeat indices must be contiguous from zero')
    if expected_variants is not None:
        require(variants == set(expected_variants), 'variant set does not match requested batch')
    require(len(pairs) == len(rounds) * len(repeats) * len(variants) * len(fixtures), 'batch is not a complete rectangular pairing')
    for identity, pair in pairs.items():
        require(set(pair) == {'cli', 'watch'}, f'{identity}: missing CLI/watch paired process')
    return processes


def check_snapshots(processes, ram_dir, extractor=None):
    ram_dir = Path(ram_dir)
    require(ram_dir.is_absolute() and ram_dir.is_relative_to(TASK_RAM), 'snapshot root must be under task RAM')
    require(ram_dir.resolve().is_relative_to(TASK_RAM), 'snapshot root escapes task RAM through symlink')
    if extractor is None:
        def extractor(path):
            result = subprocess.run(['pdftotext', '-enc', 'UTF-8', str(path), '-'], check=True,
                                    stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=30)
            return result.stdout.decode('utf-8')
    for rows in processes.values():
        for row in rows:
            path = ram_dir / row['case'] / snapshot_name(row['mode'], row['cycle'])
            require(path.is_file() and not path.is_symlink(), f'missing/non-regular snapshot: {path}')
            require(path.resolve().is_relative_to(ram_dir.resolve()), f'snapshot escaped case area: {path}')
            markers = set(re.findall(r'TEKT_EPOCH_[A-Za-z0-9_]+', extractor(path)))
            require(markers == {expected_marker(row['cycle'])},
                    f'{path}: wrong/stale/mixed epoch marker: {markers}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('csv', type=Path)
    parser.add_argument('--cycles', type=int, default=6)
    parser.add_argument('--variant', action='append', help='expected variant name, repeat for complete batch')
    parser.add_argument('--ram-dir', type=Path, help='also independently extract saved PDFs outside timed intervals')
    args = parser.parse_args()
    processes = parse_measurements(args.csv.read_text(), args.cycles, args.variant)
    if args.ram_dir: check_snapshots(processes, args.ram_dir)
    print(f'validated {len(processes)} completed processes; markers checked={args.ram_dir is not None}')


if __name__ == '__main__':
    main()
