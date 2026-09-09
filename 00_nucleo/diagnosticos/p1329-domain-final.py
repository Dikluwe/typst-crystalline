"""Report-only public witnesses for the NaN construction boundary; not a parity gate."""
import json
import subprocess
from datetime import datetime, timezone
expressions = [
    '(calc.inf - calc.inf) * 1deg',
    '(calc.inf - calc.inf) * 1%',
    'calc.abs((calc.inf - calc.inf) * 1deg)',
    'calc.abs((calc.inf - calc.inf) * 1%)',
    'calc.abs(float.nan * 1deg)',
    'calc.abs(-calc.inf * 1fr)',
    'calc.abs(-calc.inf * 1pt)',
]
rows = []
for expression in expressions:
    for role, binary in [
        ('BASE', '/tmp/p1328-target.T7Tg57/release/typst'),
        ('VANILLA', '/usr/local/bin/typst'),
        ('CANDIDATE', '/tmp/p1329-target.bg3p5A/release/typst'),
    ]:
        argv = [binary, '--color', 'never', 'eval', expression]
        at = datetime.now(timezone.utc).isoformat()
        result = subprocess.run(argv, capture_output=True, text=True, timeout=30)
        rows.append(dict(expression=expression, role=role, argv=argv, at=at,
                         exit=result.returncode, stdout=result.stdout, stderr=result.stderr))
print(json.dumps(dict(rows=rows, policy='Report-only explicit construction debts; not a general parity verdict'), indent=2))
