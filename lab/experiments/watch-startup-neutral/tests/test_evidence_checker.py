"""Adversarial inputs for the independent checker; no benchmark execution."""
import csv
import io
import unittest
from check_evidence import COUNTS_COLUMNS, parse_counts, parse_phases


def phase_row(pid, cycle, label, calls=1, ns=None):
    return '\t'.join(map(str, (pid, cycle, label, calls, 0 if label.startswith('event.') else (1000 if ns is None else ns))))


def cold_cycle(pid=17, cycle=0):
    labels = '''event.cycle catalog.access catalog.initialize session.total session.lock_wait
session.probe_before session.prepare session.prepared_check session.build session.final_key_check
prepare.total prepare.from_database catalog.prepared_build
event.session.cold event.session.published_new'''.split()
    return '\n'.join(phase_row(pid, cycle, label) for label in labels) + '\n'


def warm_cycle(pid=17, cycle=1):
    labels = '''event.cycle catalog.access catalog.initialize session.total session.lock_wait
session.initial_fast_verify session.initial_key_check
event.session.warm event.session.reuse_fast'''.split()
    return '\n'.join(phase_row(pid, cycle, label) for label in labels) + '\n'


def counts_rows(imports=0, recipe='aligned'):
    return [dict(zip(COUNTS_COLUMNS, [
        'batch0', recipe, variant, 'fixture0' if imports == 0 else 'fixture16', imports,
        4, 4 * (imports + 1), 800, 100000, 2976 if variant == 'bas' else 2992,
        4 * ((1 if imports == 0 else 2) if variant == 'laz' else imports + 1),
    ])) for variant in ('bas', 'neu', 'laz')]


def count_csv(rows):
    buffer = io.StringIO()
    writer = csv.DictWriter(buffer, fieldnames=COUNTS_COLUMNS)
    writer.writeheader()
    writer.writerows(rows)
    return buffer.getvalue()


class PhaseEvidence(unittest.TestCase):
    def test_cold_then_fast_warm(self):
        self.assertEqual(len(parse_phases(cold_cycle() + warm_cycle())), 2)

    def test_prepared_path_is_valid_when_host_not_fast_eligible(self):
        text = cold_cycle().replace('event.session.cold', 'event.session.warm')
        text += phase_row(17, 0, 'session.initial_fast_verify') + '\n'
        parse_phases(text)

    def test_empty_session_set_is_valid_for_fresh_cli_catalog(self):
        parse_phases(phase_row(42, 0, 'event.cycle') + '\n')

    def test_rejects_wrong_labels_ids_duplicates_and_event_time(self):
        valid = cold_cycle()
        corruptions = [
            valid.replace('session.prepare\t', 'session.preprare\t'),
            valid.replace('17\t0\t', '0\t0\t', 1),
            valid + phase_row(17, 0, 'session.prepare') + '\n',
            valid.replace('event.cycle\t1\t0', 'event.cycle\t2\t0'),
            valid.replace('event.cycle\t1\t0', 'event.cycle\t1\t1'),
            valid.replace('event.cycle\t1\t0', 'event.cycle\t0\t0'),
            valid.replace(phase_row(17, 0, 'event.cycle') + '\n', ''),
            valid + warm_cycle(cycle=2) + warm_cycle(cycle=1),
            valid.replace('17\t0\t', 'different\t0\t', 1),
        ]
        for text in corruptions:
            with self.subTest(text=text), self.assertRaises(ValueError):
                parse_phases(text)

    def test_rejects_count_mutant_and_short_circuit_violation(self):
        for text in [
            cold_cycle().replace('session.lock_wait\t1\t', 'session.lock_wait\t2\t'),
            cold_cycle().replace('event.session.published_new\t1\t0', 'event.session.reuse_fast\t1\t0'),
            cold_cycle() + phase_row(17, 0, 'session.initial_key_check') + '\n',
            cold_cycle().replace('session.prepare\t1\t1000', 'session.prepare\t1\t1001'),
        ]:
            with self.subTest(text=text), self.assertRaises(ValueError):
                parse_phases(text)


class CounterEvidence(unittest.TestCase):
    def test_matched_triples_with_separate_recipes_and_imports(self):
        groups = parse_counts(count_csv(counts_rows() + counts_rows(recipe='historical') + counts_rows(16)))
        self.assertEqual(len(groups), 3)

    def test_separate_allocation_and_builder_benches(self):
        rows = counts_rows()
        for row in rows:
            row['builders'] = ''
        parse_counts(count_csv(rows))
        rows = counts_rows()
        for row in rows:
            row['alloc_calls'] = row['alloc_bytes'] = ''
        parse_counts(count_csv(rows))
        rows[0]['alloc_calls'] = 800
        with self.assertRaises(ValueError):
            parse_counts(count_csv(rows))

    def test_neutral_allocation_mutant_is_rejected(self):
        rows = counts_rows()
        rows[1]['alloc_calls'] += 4
        rows[1]['alloc_bytes'] += 128
        with self.assertRaises(ValueError):
            parse_counts(count_csv(rows))

    def test_wrong_builder_input_size_variant_or_pair_is_rejected(self):
        for field, value in [('builders', 8), ('inputs', 8), ('context_bytes', 2976),
                             ('variant', 'neutral'), ('roots', 0)]:
            rows = counts_rows()
            rows[1][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                parse_counts(count_csv(rows))
        for rows in [counts_rows()[:-1], counts_rows() + counts_rows()[:1]]:
            with self.assertRaises(ValueError):
                parse_counts(count_csv(rows))

    def test_recipe_mix_is_not_silently_paired(self):
        rows = counts_rows()
        rows[1]['recipe'] = 'different'
        with self.assertRaises(ValueError):
            parse_counts(count_csv(rows))


if __name__ == '__main__':
    unittest.main()
