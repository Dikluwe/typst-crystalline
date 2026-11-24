#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
export TEKT_EXE="$d/typst"
export TEKT_LAB_WORK="$(mktemp -d "$root/lab/target/watch-reference.XXXXXXXX")"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
export PYTHONDONTWRITEBYTECODE=1
export PYTHONPATH="$root/lab/experiments/references-500-visible-eval-residual-2026-11-21"
printf '%s  %s\n' 53ca01500b641bcf12aa3db002d4b075e8031aa84b4232c4c34ffc2cbafa796f "$TEKT_EXE" | sha256sum --check >&2
python3 - <<'PY'
import json
import os
from pathlib import Path
from watch import EXES, run

EXES['candidate'] = Path(os.environ['TEKT_EXE'])
run_id = run(0, 'candidate', 0, overrides={
    'TEKT_LAB_MEMO_PROTECT': '1',
    'TEKT_LAB_MEMO_BYTES': str(8 * 1024 * 1024),
})
print(json.dumps(run_id, indent=2))
PY
