"""Independent ordinary/adversarial fixtures. C executes; no driver import."""
import copy
import csv
import io
from pathlib import Path
import tempfile
import unittest
from check_phases import parse_phases
from check_measurements import (FIELDS, TASK_RAM, expected_marker, parse_measurements,
                                check_snapshots, snapshot_name)


def phases_fixture():
    return {
        'event.cycle': (1, 0), 'pipeline.paged': (1, 1000),
        'pipeline.eval': (1, 100), 'pipeline.introspect': (1, 100),
        'pipeline.context': (1, 200), 'pipeline.layout': (1, 200), 'pipeline.finish': (1, 100),
        'catalog.access': (2, 80), 'catalog.initialize': (2, 50),
        'font.eval': (1, 20), 'font.outside': (1, 30),
        'layout.round': (2, 300), 'layout.pass.1': (2, 100), 'layout.pass.2': (1, 80),
        'event.layout.complete': (2, 0), 'event.layout.pending': (1, 0),
        'context.attempt.1': (1, 50), 'context.attempt.2': (1, 50),
        'event.context.attempt': (2, 0), 'source.construct': (3, 5000),
    }


def tsv(rows, cycle=0):
    return ''.join(f'17\t{cycle}\t{label}\t{calls}\t{ns}\n' for label, (calls, ns) in rows.items())


def measurement_fixture(cycles=2, variants=('base', 'joint')):
    rows = []
    for index, variant in enumerate(variants):
        fixed = dict(round=0, repeat=0, variant=variant, fixture='outline.typ', sample_epoch='1793910000.0',
                     wall_ns=10_000_000, process_user_ns=60, process_system_ns=40,
                     process_cpu_ns=100, process_peak_rss_bytes=4096, exit_code=0)
        rows.append(dict(fixed, process_id=100 + 2 * index, case=f'{variant}-cli', mode='cli', cycle=0, cpu_ns=100))
        for cycle in range(2 * cycles + 1):
            mode = 'startup' if cycle == 0 else ('same_bytes' if cycle % 2 else 'main_edit')
            rows.append(dict(fixed, process_id=101 + 2 * index, case=f'{variant}-watch', mode=mode,
                             cycle=cycle, cpu_ns=10, exit_code=-15))
    return rows


def csv_text(rows):
    output = io.StringIO()
    writer = csv.DictWriter(output, fieldnames=sorted(FIELDS)); writer.writeheader(); writer.writerows(rows)
    return output.getvalue()


class PhaseChecker(unittest.TestCase):
    def test_multiple_helpers_are_aggregated_without_single_global_budget(self):
        rows = phases_fixture()
        # Two helpers can each reach pass eight. The aggregate permits 16,
        # rather than falsely enforcing eight globally.
        rows['layout.round'] = (2, 300)
        for index in range(1, 9): rows[f'layout.pass.{index}'] = (2, 20)
        rows['event.layout.complete'] = (2, 0); rows['event.layout.pending'] = (14, 0)
        parse_phases(tsv(rows))

    def test_physical_roles_are_aggregated_without_an_eight_call_budget(self):
        rows = phases_fixture()
        rows.update({
            'physical.candidate': (1, 180),
            'physical.step.discovery': (2, 90), 'physical.step.replay': (1, 80),
            'physical.layout.discovery': (20, 70), 'physical.layout.replay': (10, 60),
            'event.physical.complete': (2, 0), 'event.physical.pending': (28, 0),
        })
        parse_phases(tsv(rows))

    def test_physical_outcomes_role_parents_and_unique_labels_are_required(self):
        original = phases_fixture()
        original.update({
            'physical.candidate': (1, 180),
            'physical.step.discovery': (1, 90), 'physical.step.replay': (1, 80),
            'physical.layout.discovery': (1, 70), 'physical.layout.replay': (1, 60),
            'event.physical.complete': (1, 0), 'event.physical.pending': (1, 0),
        })
        for change in ('missing-outcome', 'missing-candidate', 'missing-role-parent', 'role-duration', 'duplicate-role'):
            rows = copy.deepcopy(original)
            if change == 'missing-outcome': del rows['event.physical.pending']
            elif change == 'missing-candidate': del rows['physical.candidate']
            elif change == 'missing-role-parent': del rows['physical.step.replay']
            elif change == 'role-duration': rows['physical.layout.replay'] = (1, 81)
            text = tsv(rows)
            if change == 'duplicate-role': text += '17\t0\tphysical.layout.discovery\t1\t1\n'
            with self.subTest(change=change), self.assertRaises(ValueError): parse_phases(text)

    def test_source_construction_may_precede_paged_and_is_not_parse_only(self):
        parse_phases(tsv(phases_fixture()))  # construct ns intentionally > paged
        parse_phases(tsv({'event.cycle': (1, 0), 'source.construct': (1, 50)}))

    def test_font_outside_and_qualified_stage_form_exact_attribution(self):
        parse_phases(tsv(phases_fixture()))
        for change in ('missing', 'duplicate', 'duration', 'above-stage'):
            rows = phases_fixture()
            if change == 'missing': del rows['font.eval']
            elif change == 'duplicate': rows['font.context'] = (1, 20)
            elif change == 'duration': rows['font.eval'] = (1, 21)
            else:
                rows['font.eval'] = (1, 101)
                rows['catalog.initialize'] = (2, 131)
                rows['catalog.access'] = (2, 150)
            with self.subTest(change=change), self.assertRaises(ValueError): parse_phases(tsv(rows))

    def test_missing_pass_outcome_wrong_index_or_overbudget_is_rejected(self):
        for change in ('missing-pass', 'missing-outcome', 'pass9', 'attempt6', 'overbudget', 'wrong-order', 'context-count'):
            rows = phases_fixture()
            if change == 'missing-pass': del rows['layout.pass.2']
            elif change == 'missing-outcome': del rows['event.layout.pending']
            elif change == 'pass9': rows['layout.pass.9'] = rows.pop('layout.pass.2')
            elif change == 'attempt6': rows['context.attempt.6'] = rows.pop('context.attempt.2')
            elif change == 'overbudget':
                rows['layout.round'] = (1, 300)
                rows['layout.pass.1'] = (9, 100)
                rows['event.layout.complete'] = (9, 0)
            elif change == 'wrong-order':
                rows['layout.pass.2'] = (3, 80)
                rows['event.layout.pending'] = (3, 0)
            else: rows['event.context.attempt'] = (3, 0)
            with self.subTest(change=change), self.assertRaises(ValueError): parse_phases(tsv(rows))

    def test_inclusive_nested_labels_are_not_added_to_coarse_phases(self):
        rows = phases_fixture()
        parse_phases(tsv(rows))
        rows['pipeline.finish'] = (1, 401)  # gross sum becomes 1001 > paged1000
        with self.assertRaises(ValueError): parse_phases(tsv(rows))
        rows = phases_fixture(); rows['layout.pass.1'] = (2, 301)
        with self.assertRaises(ValueError): parse_phases(tsv(rows))

    def test_prior_schema_label_and_identity_guards_are_retained(self):
        text = tsv(phases_fixture())
        for mutant in [text + '17\t0\tpipeline.eval\t1\t1\n',
                       text.replace('pipeline.layout\t', 'pipeline.layouut\t'),
                       text.replace('17\t0\t', '0\t0\t', 1),
                       text.replace('event.cycle\t1\t0', 'event.cycle\t1\t1')]:
            with self.assertRaises(ValueError): parse_phases(mutant)


class MeasurementChecker(unittest.TestCase):
    def test_complete_batch_and_epoch_alternation(self):
        self.assertEqual(len(parse_measurements(csv_text(measurement_fixture()), cycles=2)), 4)
        self.assertEqual([expected_marker(cycle) for cycle in range(9)],
                         ['TEKT_EPOCH_0', 'TEKT_EPOCH_0', 'TEKT_EPOCH_1', 'TEKT_EPOCH_1',
                          'TEKT_EPOCH_0', 'TEKT_EPOCH_0', 'TEKT_EPOCH_1', 'TEKT_EPOCH_1', 'TEKT_EPOCH_0'])

    def test_missing_duplicate_reordered_or_mixed_cycles_are_rejected(self):
        for change in ('missing', 'duplicate', 'reordered', 'wrong-mode', 'mixed-pid', 'wrong-limit', 'incomplete-pair'):
            rows = measurement_fixture()
            if change == 'missing': del rows[3]
            elif change == 'duplicate': rows.insert(3, copy.deepcopy(rows[3]))
            elif change == 'reordered': rows[2], rows[3] = rows[3], rows[2]
            elif change == 'wrong-mode': rows[2]['mode'] = 'main_edit'
            elif change == 'mixed-pid': rows[2]['process_id'] = rows[0]['process_id']
            elif change == 'incomplete-pair': rows = rows[1:]
            with self.subTest(change=change), self.assertRaises(ValueError):
                parse_measurements(csv_text(rows), cycles=3 if change == 'wrong-limit' else 2)

    def test_cpu_components_intervals_totals_and_metadata_are_checked(self):
        for field, value in [('process_cpu_ns', 101), ('cpu_ns', -1), ('wall_ns', 0),
                             ('exit_code', 1), ('sample_epoch', 'NaN'), ('process_peak_rss_bytes', 0)]:
            rows = measurement_fixture(); rows[2][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError): parse_measurements(csv_text(rows), 2)
        rows = measurement_fixture()
        for row in rows[1:6]:
            row['cpu_ns'] = 500_000
        with self.assertRaises(ValueError): parse_measurements(csv_text(rows), 2)
        rows = measurement_fixture(); rows[2]['cpu_ns'] = 20_000_000
        with self.assertRaises(ValueError): parse_measurements(csv_text(rows), 2)

    def test_variant_and_rectangular_pairing_are_not_silently_mixed(self):
        rows = measurement_fixture()
        with self.assertRaises(ValueError): parse_measurements(csv_text(rows), 2, ['base', 'other'])
        rows[0]['round'] = 1
        with self.assertRaises(ValueError): parse_measurements(csv_text(rows), 2)

    def test_saved_snapshots_reject_stale_or_inverted_marker_offline(self):
        TASK_RAM.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='checker-b-', dir=TASK_RAM) as directory:
            root = Path(directory)
            processes = parse_measurements(csv_text(measurement_fixture()), 2)
            for rows in processes.values():
                for row in rows:
                    path = root / row['case'] / snapshot_name(row['mode'], row['cycle'])
                    path.parent.mkdir(exist_ok=True)
                    path.write_text(expected_marker(row['cycle']))
            check_snapshots(processes, root, extractor=lambda path: path.read_text())
            # These are controlled text stand-ins for extractor output, not PDF
            # conformance fixtures. The real CLI validator invokes pdftotext.
            changed = root / 'base-watch' / 'cycle-002-main_edit.pdf'
            changed.write_text('TEKT_EPOCH_0')
            with self.assertRaises(ValueError): check_snapshots(processes, root, extractor=lambda path: path.read_text())
            changed.write_text('TEKT_EPOCH_0 TEKT_EPOCH_1')
            with self.assertRaises(ValueError): check_snapshots(processes, root, extractor=lambda path: path.read_text())
            changed.unlink()
            with self.assertRaises(ValueError): check_snapshots(processes, root, extractor=lambda path: path.read_text())


if __name__ == '__main__':
    unittest.main()
