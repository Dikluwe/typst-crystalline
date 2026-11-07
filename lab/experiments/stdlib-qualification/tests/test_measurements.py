"""Independent nominal CSV and diagnostic fixtures; no driver import."""
import copy
import csv
import io
from pathlib import Path
import tempfile
import unittest
from check_measurements import (FIELDS, TASK_RAM, parse_measurements, check_snapshots,
    expected_marker, snapshot_name, classify_exit, compare_diagnostics, normalize_diagnostic)


def fixtures(cycles=2):
    rows = []
    for variant_index, variant in enumerate(('bas', 'neu', 'laz')):
        common = dict(round=0, repeat=0, variant=variant, fixture='cetz.typ', sample_epoch='1793910000',
                      wall_ns=20_000_000, process_user_ns=70, process_system_ns=30,
                      process_cpu_ns=100, process_peak_rss_bytes=4096, exit_code=0)
        rows.append(dict(common, mode='cli', process_id=100 + variant_index * 2,
                         case=f'{variant}-cli', cycle=0, cpu_ns=100))
        for cycle in range(cycles * 2 + 1):
            mode = 'startup' if cycle == 0 else ('same_bytes' if cycle % 2 else 'main_edit')
            rows.append(dict(common, mode=mode, process_id=101 + variant_index * 2,
                             case=f'{variant}-watch', cycle=cycle, cpu_ns=10, exit_code=-15))
    return rows


def csv_text(rows):
    stream = io.StringIO()
    writer = csv.DictWriter(stream, fieldnames=sorted(FIELDS)); writer.writeheader(); writer.writerows(rows)
    return stream.getvalue()


class NominalMeasurements(unittest.TestCase):
    def test_three_nominal_variants_with_two_warm_pairs(self):
        self.assertEqual(len(parse_measurements(csv_text(fixtures()), 2, ['bas', 'neu', 'laz'])), 6)

    def test_cycle_pid_mode_and_cpu_mutants_are_rejected(self):
        for field, value in [('cycle', 2), ('mode', 'main_edit'), ('process_id', 100),
                             ('cpu_ns', 100_000_000), ('process_cpu_ns', 101)]:
            rows = fixtures(); rows[2][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError): parse_measurements(csv_text(rows), 2)
        rows = fixtures(); del rows[3]
        with self.assertRaises(ValueError): parse_measurements(csv_text(rows), 2)
        rows = fixtures(); rows.insert(3, copy.deepcopy(rows[3]))
        with self.assertRaises(ValueError): parse_measurements(csv_text(rows), 2)
        rows = fixtures()
        for row in rows[1:6]: row['cpu_ns'] = 300_000
        with self.assertRaises(ValueError): parse_measurements(csv_text(rows), 2)

    def test_diagnostic_exit_is_classified_separately_and_not_accepted_as_timing_success(self):
        self.assertEqual(classify_exit('cli', 0), 'success')
        self.assertEqual(classify_exit('startup', -15), 'success')
        self.assertEqual(classify_exit('cli', 1), 'diagnostic')
        self.assertEqual(classify_exit('cli', -11), 'unexpected-termination')
        self.assertEqual(classify_exit('main_edit', 1), 'unexpected-termination')
        rows = fixtures(); rows[0]['exit_code'] = 1
        with self.assertRaises(ValueError): parse_measurements(csv_text(rows), 2)

    def test_saved_epoch_sequence_is_zero_zero_one_one_zero(self):
        TASK_RAM.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='qualification-checker-b-', dir=TASK_RAM) as directory:
            root = Path(directory)
            processes = parse_measurements(csv_text(fixtures()), 2)
            for rows in processes.values():
                for row in rows:
                    path = root / row['case'] / snapshot_name(row['mode'], row['cycle'])
                    path.parent.mkdir(exist_ok=True); path.write_text(expected_marker(row['cycle']))
            check_snapshots(processes, root, extractor=lambda path: path.read_text())
            changed = root / 'laz-watch/cycle-004-main_edit.pdf'
            changed.write_text('TEKT_EPOCH_1')
            with self.assertRaises(ValueError): check_snapshots(processes, root, extractor=lambda path: path.read_text())


class Diagnostics(unittest.TestCase):
    def test_only_absolute_case_roots_are_normalized(self):
        left = 'error: sample\n  at /dev/shm/A/inputs/main.typ:2:4\nhelp: keep this\ntrace: /dev/shm/A/inputs/child.typ:7\n'
        right = left.replace('/dev/shm/A', '/dev/shm/B')
        result = compare_diagnostics(left, right, '/dev/shm/A', '/dev/shm/B')
        self.assertIn('<CASE_ROOT>/inputs/main.typ:2:4', result)
        self.assertEqual(normalize_diagnostic('/dev/shm/AB/file /outside/cache', '/dev/shm/A'),
                         '/dev/shm/AB/file /outside/cache')

    def test_message_order_trace_labels_spans_and_external_paths_remain_significant(self):
        original = 'error: cannot load\n  at /dev/shm/A/main.typ:2:4\ntrace: imported /cache/package.typ:7\nhelp: first\nhelp: second\n'
        equivalent = original.replace('/dev/shm/A', '/dev/shm/B')
        mutations = [
            equivalent.replace('cannot load', 'cannot parse'),
            equivalent.replace(':2:4', ':2:5'),
            equivalent.replace('help: first\nhelp: second', 'help: second\nhelp: first'),
            equivalent.replace('trace:', 'note:'),
            equivalent.replace('/cache/package', '/other/package'),
            equivalent.replace('\nhelp: second', ''),
        ]
        for changed in mutations:
            with self.subTest(changed=changed), self.assertRaises(ValueError):
                compare_diagnostics(original, changed, '/dev/shm/A', '/dev/shm/B')


if __name__ == '__main__':
    unittest.main()
