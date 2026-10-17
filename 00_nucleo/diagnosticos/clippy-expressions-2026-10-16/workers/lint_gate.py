from pathlib import Path
import gzip,json,subprocess,sys
b=json.load(gzip.open('00_nucleo/diagnosticos/clippy-expressions-2026-10-16/before.json.gz'))
cmd=['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-A','warnings']
for rule in b['rules']:cmd.extend(['-D',rule])
cmd.extend(['-D','unfulfilled_lint_expectations'])
sys.exit(subprocess.run(cmd).returncode)
