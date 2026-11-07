import csv
import importlib.util
import io
import os
from pathlib import Path
import tempfile
import unittest

spec = importlib.util.spec_from_file_location('independent_cli_checker', Path(__file__).with_name('check_cli.py'))
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)


def fixture_rows():
    result = []
    for round_id in range(6):
        names = ['bas', 'laz', 'dup']
        names = names[round_id % 3:] + names[:round_id % 3]
        for variant in names:
            i = len(result)
            cpu = (100 + round_id + (variant == 'laz')) * 1_000_000
            result.append(dict(round=round_id, repeat=0, variant=variant,
                               fixture='no_import.typ', mode='cli', cycle=0,
                               process_id=100 + i, case=f'case-{i}', sample_epoch=1000.0 + i,
                               wall_ns=200_000_000, cpu_ns=cpu, process_cpu_ns=cpu,
                               process_user_ns=cpu - 2_000_000, process_system_ns=2_000_000,
                               process_peak_rss_bytes=32 * 1024 * 1024, exit_code=0))
    return result


def encode(rows):
    out = io.StringIO()
    writer = csv.DictWriter(out, fieldnames=checker.FIELDS)
    writer.writeheader(); writer.writerows(rows)
    return out.getvalue()


class EvidenceChecks(unittest.TestCase):
    def parse(self, rows):
        return checker.parse(encode(rows), ['bas', 'laz', 'dup'], ['no_import.typ'])

    def test_complete_balanced_pairing_and_exact_schema(self):
        rows = fixture_rows()
        self.assertEqual(len(self.parse(rows)), 18)
        with self.assertRaises(ValueError):
            checker.parse(encode(rows).replace('process_id', 'unused_column'))
        for key, bad in [('mode', 'startup'), ('exit_code', -15), ('cycle', 1),
                         ('fixture', '../escape.typ'), ('sample_epoch', 'nan')]:
            changed = fixture_rows(); changed[0][key] = bad
            with self.subTest(key=key), self.assertRaises(ValueError):
                self.parse(changed)

    def test_cpu_components_interval_and_single_cpu_bounds(self):
        for edits in [dict(cpu_ns=1), dict(process_cpu_ns=1), dict(process_user_ns=1),
                      dict(wall_ns=1)]:
            changed = fixture_rows(); changed[0].update(edits)
            with self.subTest(edits=edits), self.assertRaises(ValueError):
                self.parse(changed)

    def test_missing_pair_reused_pid_and_duplicate_observation_rejected(self):
        with self.assertRaises(ValueError):
            self.parse(fixture_rows()[:-1])
        for key in ['process_id', 'case']:
            changed = fixture_rows(); changed[1][key] = changed[0][key]
            with self.subTest(key=key), self.assertRaises(ValueError):
                self.parse(changed)
        changed = fixture_rows(); changed[1]['variant'] = changed[0]['variant']
        with self.assertRaises(ValueError):
            self.parse(changed)

    def test_fixed_ordinal_bias_is_not_accepted_as_balanced(self):
        changed = sorted(fixture_rows(), key=lambda r: (r['round'], r['variant']))
        for index, row in enumerate(changed):
            row['sample_epoch'] = 1000.0 + index
        with self.assertRaises(ValueError):
            self.parse(changed)

    def test_summary_reports_ms_ratio_and_identical_elf_control_without_budget(self):
        summary = checker.summarize(self.parse(fixture_rows()), 'bas', draws=500)
        observed = {r['variant']: r for r in summary}
        self.assertEqual(observed['dup']['paired_delta_ms'], 0)
        self.assertEqual(observed['dup']['delta_ci_low_ms'], 0)
        self.assertEqual(observed['dup']['delta_ci_high_ms'], 0)
        self.assertEqual(observed['dup']['paired_percent'], 0)
        self.assertEqual(observed['laz']['paired_delta_ms'], 1)
        self.assertEqual(observed['laz']['delta_ci_low_ms'], 1)
        self.assertEqual(observed['laz']['delta_ci_high_ms'], 1)
        self.assertGreater(observed['laz']['ci_low'], 0)
        self.assertNotIn('acceptable', observed['laz'])
        self.assertEqual(summary, checker.summarize(self.parse(fixture_rows()), 'bas', draws=500))

    def test_summary_averages_repeats_inside_each_round(self):
        rows = self.parse(fixture_rows())
        extra = [dict(r, repeat=1, cpu_ns=r['cpu_ns'] + 4_000_000) for r in rows]
        summary = checker.summarize(rows + extra, 'bas', draws=500)
        laz = next(r for r in summary if r['variant'] == 'laz')
        self.assertEqual(laz['rounds'], 6)
        self.assertEqual(laz['paired_delta_ms'], 1)

    def test_fresh_pdf_valid_but_same_marker_old_pdf_rejected(self):
        row = self.parse(fixture_rows())[0]
        with tempfile.TemporaryDirectory() as temporary:
            old = checker.TASK_RAM
            checker.TASK_RAM = Path(temporary)
            try:
                path = Path(temporary) / 'batch' / row['case'] / 'cli.pdf'
                path.parent.mkdir(parents=True)
                path.write_bytes(b'%PDF-1.7\ncontent\n%%EOF\n')
                epoch_ns = round(row['sample_epoch'] * 1e9)
                os.utime(path, ns=(epoch_ns + 10_000_000, epoch_ns + 10_000_000))
                checker.check_snapshots([row], Path(temporary) / 'batch', lambda _: 'TEKT_EPOCH_0')
                os.utime(path, ns=(epoch_ns - 1, epoch_ns - 1))
                with self.assertRaises(ValueError):
                    checker.check_snapshots([row], Path(temporary) / 'batch', lambda _: 'TEKT_EPOCH_0')
            finally:
                checker.TASK_RAM = old

    def test_truncated_pdf_and_mixed_markers_rejected(self):
        row = self.parse(fixture_rows())[0]
        with tempfile.TemporaryDirectory() as temporary:
            old = checker.TASK_RAM
            checker.TASK_RAM = Path(temporary)
            try:
                path = Path(temporary) / 'batch' / row['case'] / 'cli.pdf'
                path.parent.mkdir(parents=True)
                epoch_ns = round(row['sample_epoch'] * 1e9)
                for data, text in [(b'%PDF-1.7\nmissing eof', 'TEKT_EPOCH_0'),
                                   (b'%PDF-1.7\n%%EOF', 'TEKT_EPOCH_0 TEKT_EPOCH_1')]:
                    path.write_bytes(data)
                    os.utime(path, ns=(epoch_ns + 10_000_000, epoch_ns + 10_000_000))
                    with self.subTest(data=data), self.assertRaises(ValueError):
                        checker.check_snapshots([row], Path(temporary) / 'batch', lambda _: text)
            finally:
                checker.TASK_RAM = old


if __name__ == '__main__':
    unittest.main()
