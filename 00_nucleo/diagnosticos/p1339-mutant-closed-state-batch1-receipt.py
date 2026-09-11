"""Generate preparation metadata only; never import/run an adapter or predicate."""
import ast
import datetime
import hashlib
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'

def pin(path):
    path = pathlib.Path(path)
    return {'path': str(path.resolve()), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}

def command(argv):
    result = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True, check=True)
    return {'argv': argv, 'exit': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr}

runner = D / 'p1339-mutant-closed-state-final-runner.py'
tree = ast.parse(runner.read_text())
pins = next(ast.literal_eval(node.value) for node in tree.body
            if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == 'PINS' for t in node.targets))
for name, expected in pins.items():
    assert pin(D / name)['sha256'] == expected, name
extras = [
    'p1339-mutant-closed-state-final-runner.py',
    'p1339-mutant-closed-state-batch1-integration.md',
    'p1339-mutant-closed-state-batch1-receipt.py',
    'p1339-mutant-closed-state-inventory.json',
    'p1339-mutant-closed-state-inventory-supplement.json',
    'p1339-mutant-closed-state-inventory-closure.json',
    'p1339-mutant-closed-state-inventory-resolution.json',
    'p1339-mutant-closed-state-style-signatures.md',
    'p1339-ab-batch1-coverage-map.json',
    'p1339-mutation-registry.json',
    'p1339-negative-oracles.json',
    'p1339-mutant-focal-final-r1.json',
    'p1339-closed-harness-authority.json',
]
receipt = {
    'schema': 'p1339-mechanical-batch1-preparation-v1',
    'at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'author': '/root/p1336_tests',
    'regime': 'executado sem atestacao de isolamento',
    'status': 'PreparedForIndependentReview_NOT_EXECUTED_NOT_SEALED',
    'batch': 1,
    'authority_manifest_sha256': pins['p1339-authority-manifest-r2.json'],
    'runner_input_pins': pins,
    'component_and_ancestry_pins': [pin(D / name) for name in extras],
    'repository': {
        'head': command(['git', 'rev-parse', 'HEAD']),
        'working_tree_diff_stat': command(['git', 'diff', 'HEAD', '--stat', '--', '.', ':!00_nucleo/materialization', ':!00_nucleo/context']),
        'status': command(['git', 'status', '--porcelain=v1', '--untracked-files=all', '--', '.', ':!00_nucleo/materialization', ':!00_nucleo/context']),
    },
    'derived_future_cells_not_observed': {'API': 312, 'relation': 852, 'opaque': 12, 'style': 12, 'projection': 96, 'lifecycle': 144},
    'preparation_ledger': {
        'semantic_processes_this_batch': 0,
        'new_focal_batches_this_role': 0,
        'full_preseal_executions_this_role': 0,
        'future_api_compile_and_runtime_credit': 0,
        'allowed_operations': ['read-only source/declaration inspection', 'deterministic inventory generation', 'mechanical fixture-to-adapter translation', 'rustfmt syntax only', 'Python ast.parse syntax only', 'hash and provenance capture'],
        'known_preparation_corrections': [
            'Preserved original inventory; successor removes only variant-label syntax so payload homonyms remain nominal dependencies.',
            'Preserved first opaque adapter; successor translates independently authored total-channel veto and typed predicates.',
            'Style hook trait-object Sized adjustment before published hash; no compile attempt and no expectation change.',
            'Transport proposal preserved but superseded by independently authored typed-r2 fixtures/checker.',
        ],
        'historical_cost_source': 'p1339-mutation-registry.json retains multiplex r1/r2 compile failures, r3 signum miss, r4 valid build, M12 rebuild lineage, and focal histories including final81. No reset and no future runtime credit.',
        'preparation_read_tool_failures': ['jq unavailable', 'one read-only Python expression SyntaxError corrected', 'read-only nonexistent-path rg/wc calls'],
        'duration_limit': 'No invented cumulative wall time; original tool/receipt timestamps remain authoritative. This receipt timestamps preparation capture, not earlier builds.',
    },
    'open_preconditions': [
        'Aggregate author manifest and independent audit before any new focal execution.',
        'Actual future APIs and private bindings only after seal/RED and authorized implementation phase.',
        'Cross-crate test-only observation exposure explicitly resolved and audited; infra cfg(test) does not propagate to core dependency.',
        'Real source/configuration/binary binding audit; no fixture-driven logs, substitute cycle or new productive API.',
        'Typing/lifetimes not checked: syntax-only preparation is not successful compilation.',
        'Coverage-map structural witnesses independently judged; no inventory-only discharge.',
    ],
}
output = D / 'p1339-mutant-closed-state-batch1-receipt.json'
with output.open('x') as stream:
    json.dump(receipt, stream, ensure_ascii=False, indent=2)
    stream.write('\n')
print(json.dumps({'receipt': pin(output), 'status': receipt['status']}))
