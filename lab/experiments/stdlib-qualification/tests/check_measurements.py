#!/usr/bin/env python3
"""Nominal package qualification: independent CSV/PDF and diagnostic checks."""
import argparse
import importlib.util
import os
from pathlib import Path
import re

ORACLE = Path(os.environ.get('TEKT_QUAL_MEASUREMENT_ORACLE', str(
    Path(__file__).resolve().parents[2] / 'outline-warm-residual/tests/check_measurements.py')))
spec = importlib.util.spec_from_file_location('qualification_prior_measurement_oracle', ORACLE)
prior = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prior)
# Only the artifact namespace changes; all pairing/CPU/cycle rules remain.
TASK_RAM = Path('/dev/shm/tekt-stdlib-qualification-20261106')
prior.TASK_RAM = TASK_RAM
FIELDS = prior.FIELDS
parse_measurements = prior.parse_measurements
check_snapshots = prior.check_snapshots
expected_marker = prior.expected_marker
snapshot_name = prior.snapshot_name


def classify_exit(mode, code):
    """Classification only; successful timing CSVs still reject diagnostics."""
    if (mode == 'cli' and code == 0) or (mode in ('startup', 'same_bytes', 'main_edit') and code in (0, -15)):
        return 'success'
    if mode == 'cli' and code > 0:
        return 'diagnostic'
    return 'unexpected-termination'


def normalize_diagnostic(text, case_root):
    # Do not strip ANSI, sort traces, collapse whitespace, rewrite messages,
    # change labels/spans, or normalize a separate package-cache location.
    root = Path(case_root)
    prior.require(root.is_absolute() and root != Path('/'), 'diagnostic case root must be absolute and specific')
    prefix = str(root).rstrip('/')
    boundary = r'(?=$|[/\s:\"\'\)\]\}])'
    return re.sub(re.escape(prefix) + boundary, '<CASE_ROOT>', text)


def compare_diagnostics(left, right, left_root, right_root):
    normalized_left = normalize_diagnostic(left, left_root)
    normalized_right = normalize_diagnostic(right, right_root)
    prior.require(normalized_left == normalized_right, 'diagnostics differ after replacing only absolute case roots')
    return normalized_left


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('csv', nargs='?', type=Path)
    parser.add_argument('--cycles', type=int, default=2)
    parser.add_argument('--variant', action='append')
    parser.add_argument('--ram-dir', type=Path)
    parser.add_argument('--diagnostic-left', type=Path)
    parser.add_argument('--diagnostic-right', type=Path)
    parser.add_argument('--root-left', type=Path)
    parser.add_argument('--root-right', type=Path)
    args = parser.parse_args()
    diagnostic_args = [args.diagnostic_left, args.diagnostic_right, args.root_left, args.root_right]
    if any(diagnostic_args) and not all(diagnostic_args):
        parser.error('diagnostic comparison requires both files and both absolute case roots')
    if not args.csv and not all(diagnostic_args):
        parser.error('provide CSV and/or complete diagnostic comparison arguments')
    if args.csv:
        groups = parse_measurements(args.csv.read_text(), args.cycles, args.variant)
        if args.ram_dir: check_snapshots(groups, args.ram_dir)
        print(f'validated {len(groups)} nominal processes; markers checked={args.ram_dir is not None}')
    if all(diagnostic_args):
        compare_diagnostics(args.diagnostic_left.read_text(), args.diagnostic_right.read_text(), args.root_left, args.root_right)
        print('diagnostics agree with only absolute case roots normalized')


if __name__ == '__main__':
    main()
