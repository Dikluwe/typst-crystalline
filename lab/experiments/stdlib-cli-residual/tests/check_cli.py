#!/usr/bin/env python3
"""Independent CLI-only paired CPU/wall evidence and snapshot validation."""
import argparse
import csv
from collections import Counter, defaultdict
import io
import math
from pathlib import Path, PurePosixPath
import random
import re
import statistics
import subprocess

TASK_RAM = Path('/dev/shm/tekt-stdlib-cli-residual-20261106')
FIELDS = ['round', 'repeat', 'variant', 'fixture', 'mode', 'cycle', 'process_id',
          'case', 'sample_epoch', 'wall_ns', 'cpu_ns', 'process_cpu_ns',
          'process_user_ns', 'process_system_ns', 'process_peak_rss_bytes', 'exit_code']


def require(value, message):
    if not value:
        raise ValueError(message)


def parse(text, expected_variants=None, expected_fixtures=None, check_order=True):
    reader = csv.DictReader(io.StringIO(text))
    require(reader.fieldnames and len(reader.fieldnames) == len(FIELDS)
            and set(reader.fieldnames) == set(FIELDS), 'wrong CSV schema')
    rows = []
    pids = set()
    cases = set()
    identities = set()
    groups = defaultdict(list)
    for raw in reader:
        require(None not in raw and all(v is not None for v in raw.values()), 'malformed CSV row')
        row = dict(raw)
        for key in ['round', 'repeat', 'cycle', 'process_id', 'wall_ns', 'cpu_ns',
                    'process_cpu_ns', 'process_user_ns', 'process_system_ns',
                    'process_peak_rss_bytes', 'exit_code']:
            require(raw[key].isascii() and raw[key].isdecimal(), f'invalid integer {key}')
            row[key] = int(raw[key])
        for key in ['variant', 'case']:
            require(re.fullmatch(r'[A-Za-z0-9_.-]+', row[key]) and row[key] not in ['.', '..'], f'invalid {key}')
        fixture = PurePosixPath(row['fixture'])
        require(row['fixture'] and not fixture.is_absolute() and '..' not in fixture.parts
                and fixture != PurePosixPath('.'), 'fixture escapes root')
        try:
            row['sample_epoch'] = float(row['sample_epoch'])
        except ValueError as error:
            raise ValueError('invalid epoch') from error
        require(math.isfinite(row['sample_epoch']) and row['sample_epoch'] > 0, 'invalid epoch')
        require(not rows or row['sample_epoch'] > rows[-1]['sample_epoch'], 'CSV is not in execution-time order')
        require(row['mode'] == 'cli' and row['cycle'] == 0 and row['exit_code'] == 0, 'not successful CLI cycle zero')
        require(row['process_id'] > 0 and row['wall_ns'] > 0
                and row['process_peak_rss_bytes'] > 0, 'nonpositive process quantity')
        require(row['process_id'] not in pids and row['case'] not in cases, 'PID/case reused')
        require(row['cpu_ns'] == row['process_cpu_ns']
                == row['process_user_ns'] + row['process_system_ns'], 'wait4 CPU mismatch')
        require(row['cpu_ns'] <= row['wall_ns'] + 1_000_000, 'single-CPU exceeds wall by >1ms')
        identity = tuple(row[k] for k in ['round', 'repeat', 'variant', 'fixture'])
        require(identity not in identities, 'duplicate paired observation')
        pids.add(row['process_id']); cases.add(row['case']); identities.add(identity)
        groups[identity[:2] + identity[3:]].append(row)
        rows.append(row)
    require(rows, 'empty CSV')
    rounds = {r['round'] for r in rows}; repeats = {r['repeat'] for r in rows}
    variants = {r['variant'] for r in rows}; fixtures = {r['fixture'] for r in rows}
    require(rounds == set(range(max(rounds) + 1))
            and repeats == set(range(max(repeats) + 1)), 'noncontiguous round/repeat')
    if expected_variants is not None:
        require(variants == set(expected_variants), 'wrong variants')
    if expected_fixtures is not None:
        require(fixtures == set(expected_fixtures), 'wrong fixtures')
    require(len(rows) == len(rounds) * len(repeats) * len(fixtures) * len(variants), 'incomplete rectangular batch')
    require(all({r['variant'] for r in group} == variants and len(group) == len(variants)
                for group in groups.values()), 'incomplete variant pair')
    if check_order:
        # Rotation need not match any particular driver algorithm. For each
        # fixture/repeat, every variant must cover every ordinal position.
        positions = defaultdict(Counter)
        for group in groups.values():
            for position, row in enumerate(group):
                positions[row['fixture'], row['repeat'], row['variant']][position] += 1
        require(len(rounds) >= len(variants), 'too few rounds for ordinal controls')
        for counts in positions.values():
            all_counts = [counts[position] for position in range(len(variants))]
            require(min(all_counts) > 0 and max(all_counts) - min(all_counts) <= 1,
                    'variant order is not balanced across ordinal positions')
    return rows


def check_snapshots(rows, ram_dir, extractor=None):
    root = Path(ram_dir)
    require(root.is_absolute() and root.resolve().is_relative_to(TASK_RAM.resolve())
            and not root.is_symlink(), 'snapshot root escapes task RAM')
    if extractor is None:
        def extractor(path):
            return subprocess.check_output(['pdftotext', '-enc', 'UTF-8', str(path), '-'], timeout=30).decode('utf-8')
    for row in rows:
        pdf = root / row['case'] / 'cli.pdf'
        require(pdf.is_file() and not pdf.is_symlink()
                and pdf.resolve().is_relative_to(root.resolve()), 'missing/escaped CLI snapshot')
        content = pdf.read_bytes()
        require(content.startswith(b'%PDF-') and content.rstrip().endswith(b'%%EOF'), 'truncated/non-PDF snapshot')
        # copy2 retains output mtime; a same-marker PDF from an earlier process
        # cannot be accepted merely because text extraction still succeeds.
        stamp = pdf.stat().st_mtime_ns
        start = round(row['sample_epoch'] * 1e9)
        require(start <= stamp <= start + row['wall_ns'] + 1_000_000_000,
                'snapshot predates process or exceeds its publication interval')
        require(set(re.findall(r'TEKT_EPOCH_[A-Za-z0-9_]+', extractor(pdf))) == {'TEKT_EPOCH_0'},
                'wrong/mixed CLI epoch marker')


def percentile(values, probability):
    ordered = sorted(values)
    at = (len(ordered) - 1) * probability
    lo = math.floor(at); hi = math.ceil(at)
    return ordered[lo] + (ordered[hi] - ordered[lo]) * (at - lo)


def summarize(rows, reference, metric='cpu_ns', draws=10000):
    require(metric in ['cpu_ns', 'wall_ns'], 'unknown metric')
    require(isinstance(draws, int) and draws >= 100, 'too few bootstrap draws')
    require(reference in {r['variant'] for r in rows}, 'reference absent')
    by_round = defaultdict(list)
    for row in rows:
        by_round[row['fixture'], row['variant'], row['round']].append(row[metric])
    result = []
    for fixture in sorted({r['fixture'] for r in rows}):
        for variant in sorted({r['variant'] for r in rows} - {reference}):
            rounds = sorted({r['round'] for r in rows if r['fixture'] == fixture})
            require(len(rounds) >= 2, 'too few paired rounds')
            a = [statistics.mean(by_round[fixture, reference, n]) for n in rounds]
            b = [statistics.mean(by_round[fixture, variant, n]) for n in rounds]
            require(all(x > 0 for x in a), 'zero reference interval')
            differences = [(y - x) / 1e6 for x, y in zip(a, b)]
            percentages = [(y / x - 1) * 100 for x, y in zip(a, b)]
            randomizer = random.Random(0)
            delta_boot = []; percent_boot = []
            for _ in range(draws):
                indices = [randomizer.randrange(len(rounds)) for _ in rounds]
                delta_boot.append(statistics.median(differences[i] for i in indices))
                percent_boot.append(statistics.median(percentages[i] for i in indices))
            result.append(dict(fixture=fixture, reference=reference, variant=variant,
                               metric=metric, rounds=len(rounds),
                               reference_ms=statistics.median(a) / 1e6,
                               variant_ms=statistics.median(b) / 1e6,
                               paired_delta_ms=statistics.median(differences),
                               delta_ci_low_ms=percentile(delta_boot, .025),
                               delta_ci_high_ms=percentile(delta_boot, .975),
                               paired_percent=statistics.median(percentages),
                               ci_low=percentile(percent_boot, .025),
                               ci_high=percentile(percent_boot, .975)))
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('csv', type=Path)
    parser.add_argument('--variant', action='append')
    parser.add_argument('--fixture', action='append')
    parser.add_argument('--ram-dir', type=Path)
    parser.add_argument('--reference')
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    rows = parse(args.csv.read_text(), args.variant, args.fixture)
    if args.ram_dir:
        check_snapshots(rows, args.ram_dir)
    if args.reference:
        require(args.output is not None, 'summary output is required')
        summary = summarize(rows, args.reference) + summarize(rows, args.reference, 'wall_ns')
        require(summary, 'summary has no comparison')
        with args.output.open('x', newline='') as handle:
            writer = csv.DictWriter(handle, fieldnames=list(summary[0]))
            writer.writeheader(); writer.writerows(summary)
    print(f'validated {len(rows)} CLI processes; snapshots={args.ram_dir is not None}')


if __name__ == '__main__':
    main()
