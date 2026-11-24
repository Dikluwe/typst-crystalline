"""Add flexible navigation checks to PDFs retained by directed.py."""
import json
import os
from pathlib import Path

from navigation import inspect

work = Path(os.environ['TEKT_LAB_WORK'])
out = Path(os.environ['TEKT_RESULT_DIR'])
report = json.loads((out/'directed.json').read_text())
for case, arms in report.items():
    for arm, states in arms.items():
        for state, data in states.items():
            pdf = work/'directed'/case/f'00-{arm}'/f'{state}.pdf'
            data['navigation_flexible'] = inspect(pdf, data['description'])
(out/'directed.json').write_text(json.dumps(report, indent=2) + '\n')
for case, arms in report.items():
    print(case, {arm: {state: {key: len(value) for key,value in
               data['navigation_flexible']['wrong_pages'].items()}
               for state,data in states.items()} for arm,states in arms.items()})
