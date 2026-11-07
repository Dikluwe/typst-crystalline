#!/usr/bin/env python3
"""Independent extended TSV validation. Inclusive phases are never exclusive costs."""
import argparse
import importlib.util
import os
from pathlib import Path

# An explicit read-only oracle for the already-exercised standard/joint labels.
# Load a private module so extending its allowed namespace does not mutate any
# other checker invocation or the previous experiment's files.
ORACLE = Path(os.environ.get('TEKT_OUTLINE_PHASE_ORACLE',
    str(Path(__file__).resolve().parents[2] / 'joint-font-witness/tests/check_phases.py')))
spec = importlib.util.spec_from_file_location('outline_prior_phase_oracle', ORACLE)
prior = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prior)

GROSS = ['pipeline.eval', 'pipeline.introspect', 'pipeline.context', 'pipeline.layout', 'pipeline.finish']
NEW_TIMERS = set(GROSS[1:] + ['source.construct', 'layout.round'])
NEW_TIMERS.update(f'layout.pass.{index}' for index in range(1, 9))
NEW_TIMERS.update(f'context.attempt.{index}' for index in range(1, 6))
FONT_STAGES = ['eval', 'introspect', 'context', 'layout', 'finish', 'outside']
NEW_TIMERS.update(f'font.{stage}' for stage in FONT_STAGES)
NEW_TIMERS.update(['physical.candidate', 'physical.step.discovery', 'physical.step.replay',
                   'physical.layout.discovery', 'physical.layout.replay'])
NEW_EVENTS = {'event.layout.complete', 'event.layout.pending', 'event.context.attempt',
              'event.physical.complete', 'event.physical.pending'}
prior.TIMERS = prior.TIMERS | NEW_TIMERS
prior.EVENTS = prior.EVENTS | NEW_EVENTS
prior.PARENTS = {label: parents[:] for label, parents in prior.PARENTS.items()}
for label in GROSS[1:]:
    prior.PARENTS[label] = ['pipeline.paged']
prior.PARENTS['layout.round'] = ['pipeline.context', 'pipeline.layout']
prior.PARENTS['physical.candidate'] = ['pipeline.context']
for role in ('discovery', 'replay'):
    prior.PARENTS[f'physical.step.{role}'] = ['physical.candidate']
    prior.PARENTS[f'physical.layout.{role}'] = [f'physical.step.{role}']
for index in range(1, 9):
    prior.PARENTS[f'layout.pass.{index}'] = ['layout.round']
for index in range(1, 6):
    prior.PARENTS[f'context.attempt.{index}'] = ['pipeline.context']
for stage in FONT_STAGES[:-1]:
    prior.PARENTS[f'font.{stage}'] = [f'pipeline.{stage}']
# font.outside intentionally has no recorded pipeline parent.
# source.construct includes actual Source construction/reparse at L3 anchors.
# It may precede paged, sit inside source.access, or have no recorded parent.
# Do not invent a parse-only timing or fixed call count for cache hits.


def parse_phases(text):
    groups = prior.parse_phases(text)
    for identity, rows in groups.items():
        def calls(label): return rows.get(label, (0, 0))[0]
        def ns(label): return rows.get(label, (0, 0))[1]
        prior.require(sum(calls(f'font.{stage}') for stage in FONT_STAGES) == calls('catalog.initialize'),
                      f'{identity}: each font initialization needs exactly one stage attribution')
        prior.require(sum(ns(f'font.{stage}') for stage in FONT_STAGES) == ns('catalog.initialize'),
                      f'{identity}: font attribution duration must equal initialize duration exactly')
        physical_calls = sum(calls(f'physical.layout.{role}') for role in ('discovery', 'replay'))
        prior.require(calls('event.physical.complete') + calls('event.physical.pending') == physical_calls,
                      f'{identity}: every physical layout call needs exactly one outcome')
        # Physical replay/running requests are not indexed by callbacks.passes.
        # Do not impose an eight-pass budget or a call ratio to physical.step.
        passes = [calls(f'layout.pass.{index}') for index in range(1, 9)]
        outcomes = calls('event.layout.complete') + calls('event.layout.pending')
        prior.require(sum(passes) == outcomes,
                      f'{identity}: every real layout pass needs exactly one outcome')
        prior.require(sum(passes) <= 8 * calls('layout.round'),
                      f'{identity}: more than eight passes per aggregate helper invocation')
        prior.require(all(later <= earlier for earlier, later in zip(passes, passes[1:])),
                      f'{identity}: later pass count exceeds earlier pass count')
        attempts = [calls(f'context.attempt.{index}') for index in range(1, 6)]
        prior.require(all(later <= earlier for earlier, later in zip(attempts, attempts[1:])),
                      f'{identity}: later context attempt count exceeds earlier attempt count')
        prior.require(sum(attempts) <= 5 * calls('pipeline.context'),
                      f'{identity}: more than five attempts per context phase')
        if 'event.context.attempt' in rows:
            prior.require(calls('event.context.attempt') == sum(attempts),
                          f'{identity}: context event disagrees with observed attempt timers')
        prior.require(sum(ns(label) for label in GROSS) <= ns('pipeline.paged'),
                      f'{identity}: sequential coarse phases exceed paged inclusive duration')
        # layout.pass durations sit within layout.round, itself inside context
        # and/or layout. These nested labels must never be added to GROSS.
        # catalog.initialize may occur across stages; no subtraction/attribution.
    return groups


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('phases', type=Path)
    args = parser.parse_args()
    groups = parse_phases(args.phases.read_text())
    print(f'validated {len(groups)} process/cycle groups; nested durations inclusive')


if __name__ == '__main__':
    main()
