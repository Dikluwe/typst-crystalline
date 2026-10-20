"""Run a fixed sequence of recorded Figure acceptance workers."""
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
stage = sys.argv[1]
if stage == 'red-four':
    commands = [
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'integration-tests-4'), '--filter', 'figure_', '--red'],
        ['python3', str(HERE / 'build.py'), '--output', str(HERE / 'candidate-build-1')],
    ]
elif stage == 'candidate-one':
    commands = [
        ['python3', str(HERE / 'replay.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'candidate-replay-1'), '--phase', 'candidate', '--build-receipt', str(HERE / 'candidate-build-1/receipt.json')],
        ['python3', str(HERE / 'target_controls.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'candidate-target-controls-1'), '--build-receipt', str(HERE / 'candidate-build-1/receipt.json')],
        ['python3', str(HERE / 'ownership_controls.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'ownership-candidate-1')],
    ]
elif stage == 'green-seven':
    commands = [
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'integration-tests-7'), '--filter', 'figure_'],
        ['python3', str(HERE / 'build.py'), '--output', str(HERE / 'candidate-build-2')],
        ['python3', str(HERE / 'replay.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'candidate-replay-2'), '--phase', 'candidate', '--build-receipt', str(HERE / 'candidate-build-2/receipt.json')],
        ['python3', str(HERE / 'target_controls.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'candidate-target-controls-2'), '--build-receipt', str(HERE / 'candidate-build-2/receipt.json')],
        ['python3', str(HERE / 'ownership_controls.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'ownership-candidate-2')],
    ]
elif stage == 'red-eight':
    commands = [
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'integration-tests-8-core'), '--filter', 'figure_', '--red'],
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'integration-tests-8-infra'), '--filter', 'figure_', '--package', 'typst-infra', '--red'],
    ]
elif stage == 'green-nine':
    commands = [
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'integration-tests-9-core'), '--filter', 'figure_'],
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'integration-tests-9-infra'), '--filter', 'figure_', '--package', 'typst-infra'],
        ['python3', str(HERE / 'build.py'), '--output', str(HERE / 'candidate-build-3')],
        ['python3', str(HERE / 'replay.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'candidate-replay-3'), '--phase', 'candidate', '--build-receipt', str(HERE / 'candidate-build-3/receipt.json')],
        ['python3', str(HERE / 'target_controls.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'candidate-target-controls-3'), '--build-receipt', str(HERE / 'candidate-build-3/receipt.json')],
        ['python3', str(HERE / 'ownership_controls.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'ownership-candidate-3')],
    ]
elif stage in ('reference-one', 'reference-two', 'reference-three', 'reference-four', 'reference-five', 'reference-six', 'reference-seven', 'reference-eight', 'reference-nine'):
    commands = [
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'reference-tests-1-core'), '--filter', 'figure_'],
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'reference-tests-1-infra'), '--filter', 'figure_', '--package', 'typst-infra'],
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'migration-tests-1-language'), '--filter', 'p792_text_lang'],
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'migration-tests-1-numeric-ref'), '--filter', 'p462_ref_numeric'],
        ['python3', str(HERE / 'run_tests.py'), '--output', str(HERE / 'migration-tests-1-infra'), '--filter', 'pipeline_figura_numerada_prefixo_no_pdf', '--package', 'typst-infra'],
        ['python3', str(HERE / 'build.py'), '--output', str(HERE / 'reference-build-1')],
        ['python3', str(HERE / 'replay.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'reference-replay-1'), '--phase', 'candidate', '--build-receipt', str(HERE / 'reference-build-1/receipt.json')],
        ['python3', str(HERE / 'target_controls.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'reference-target-controls-1'), '--build-receipt', str(HERE / 'reference-build-1/receipt.json')],
        ['python3', str(HERE / 'ownership_controls.py'), '--product', 'lab/target/product/debug/typst', '--output', str(HERE / 'reference-ownership-1')],
    ]
else:
    raise ValueError(stage)
if stage in ('reference-two', 'reference-three', 'reference-four', 'reference-five', 'reference-six', 'reference-seven', 'reference-eight', 'reference-nine'):
    number = {'reference-two': '2', 'reference-three': '3', 'reference-four': '4', 'reference-five': '5', 'reference-six': '6', 'reference-seven': '7', 'reference-eight': '8', 'reference-nine': '9'}[stage]
    prefixes = ('reference-tests-', 'migration-tests-', 'reference-build-', 'reference-replay-', 'reference-target-controls-', 'reference-ownership-')
    for command in commands:
        for i, part in enumerate(command):
            for prefix in prefixes:
                part = part.replace(prefix + '1', prefix + number)
            command[i] = part
results = []
for command in commands:
    result = subprocess.run(command).returncode
    results.append(result)
    if stage.startswith('reference-') and result:
        output = Path(command[command.index('--output') + 1])
        log = output / 'tests.log'
        if (log.exists() and 'error[E' in log.read_text()) or command[1].endswith('build.py'):
            print('Stop after recorded compile/build failure; later workers not run', flush=True)
            break
print('Stage exits:', results, flush=True)
raise SystemExit(int(any(results)))
