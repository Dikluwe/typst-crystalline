"""Executed only by C: independent adverse evidence for the phase checker."""
import unittest
from check_phases import parse_phases


def render(labels):
    return ''.join(f'61\t0\t{label}\t{calls}\t{0 if label.startswith("event.") else 1000}\n'
                   for label, calls in labels.items())


def base():
    return {label: 1 for label in 'event.cycle catalog.access catalog.initialize'.split()}


def joint_cold():
    labels = base()
    labels.update({label: 1 for label in '''joint.session joint.lock_wait joint.establish
joint.establish_discover joint.capture joint.config_context joint.discovery joint.evidence_selfcheck
joint.g1_prepare joint.g2 joint.build joint.g3 joint.policy catalog.prepared_build
event.joint.cold event.joint.established'''.split()})
    labels['event.joint.capture_lstat'] = 2
    return labels


def standard_cold():
    return {label: 1 for label in '''session.total session.lock_wait session.probe_before
session.prepare session.prepared_check session.build session.final_key_check
event.session.cold event.session.published_new'''.split()}


class Phases(unittest.TestCase):
    def test_standard_labels_and_invariants_are_preserved(self):
        rows = base(); rows.update(standard_cold())
        parse_phases(render(rows))

    def test_single_regular_zero_target_stats_and_legacy_positive_are_both_valid(self):
        rows = joint_cold(); parse_phases(render(rows))
        rows['event.joint.capture_stat'] = 2
        parse_phases(render(rows))

    def test_decline_and_standard_fallback_form_two_inclusive_scopes(self):
        rows = joint_cold()
        for label in ['joint.build', 'joint.g3', 'joint.policy', 'catalog.prepared_build', 'event.joint.established']:
            del rows[label]
        rows.update(standard_cold())
        rows['event.joint.decline'] = rows['event.joint.fallback_standard'] = 1
        parse_phases(render(rows))

    def test_warm_joint_retains_standard_short_circuit_path(self):
        rows = base()
        rows.update({label: 1 for label in '''joint.session joint.lock_wait event.joint.warm
event.joint.fallback_standard session.total session.lock_wait session.initial_fast_verify
session.initial_key_check event.session.warm event.session.reuse_fast'''.split()})
        parse_phases(render(rows))

    def test_rejects_omitted_gates_and_inconsistent_counters(self):
        for label in ['joint.g2', 'joint.g3', 'joint.policy', 'event.cycle', 'event.joint.established']:
            rows = joint_cold(); del rows[label]
            with self.subTest(label=label), self.assertRaises(ValueError):
                parse_phases(render(rows))
        rows = joint_cold(); rows['event.joint.capture_stat'] = 3
        with self.assertRaises(ValueError):
            parse_phases(render(rows))
        rows = joint_cold(); rows['event.joint.fallback_standard'] = 1
        with self.assertRaises(ValueError):
            parse_phases(render(rows))

    def test_rejects_bad_labels_ids_duplicates_and_nonzero_event_duration(self):
        valid = render(joint_cold())
        for text in [valid.replace('joint.g2\t', 'joint.g22\t'),
                     valid.replace('61\t0\t', '0\t0\t', 1),
                     valid + '61\t0\tjoint.g2\t1\t1000\n',
                     valid.replace('event.cycle\t1\t0', 'event.cycle\t1\t1')]:
            with self.subTest(text=text), self.assertRaises(ValueError):
                parse_phases(text)


if __name__ == '__main__':
    unittest.main()
