"""Recheck directed PDFs saved by the validated watch runs."""
import json
import os
from pathlib import Path

import watch_observe as observe

work = Path(os.environ['TEKT_LAB_WORK'])
out = Path(os.environ['TEKT_RESULT_DIR'])
cases = {
    'height': ('open', 'visible'),
    'summary': ('open', 'visible'),
    'reference': ('open', 'visible'),
    'suffix': ('open', 'changed', 'visible'),
}
result = {}
for case, states in cases.items():
    result[case] = {}
    for arm in ('off', 'retain', 'baseline', 'vanilla'):
        directory = work / f'directed-{case}' / f'00-{arm}'
        result[case][arm] = {}
        for state in states:
            pdf = directory / f'{state}.pdf'
            description = observe.describe(pdf)
            try:
                navigation = observe.cycle.cycle.original.navigation(
                    pdf, description, state == 'visible')
            except AssertionError:
                navigation = None
            result[case][arm][state] = {
                'description': description,
                'navigation': navigation,
            }
    for state in states:
        assert result[case]['off'][state]['description'] == result[case]['retain'][state]['description']
        assert result[case]['retain'][state]['description'] == result[case]['baseline'][state]['description']
    print(case, {arm: {state: (None if item['navigation'] is None else
                             {key: len(value) for key, value in item['navigation']['wrong_pages'].items()})
                       for state, item in states_map.items()} for arm, states_map in result[case].items()})
(out / 'directed.json').write_text(json.dumps(result, indent=2) + '\n')
