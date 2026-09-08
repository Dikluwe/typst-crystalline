"""Recount final command gates and changes, without mutating judged inputs."""
import collections
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import sys

sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('p1310_record', D/'p1310-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)

def read(name):
    return json.loads((D/('p1310-'+name+'.json')).read_text())

baseline = read('baseline')
receipts = {name: read(name) for name in (
    'unit-red', 'unit-green', 'build', 'workspace-tests', 'lint', 'fmt', 'diff-check')}
output = receipts['workspace-tests']['stdout']
results = re.findall(r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored;', output)
tests = dict(zip(('passed','failed','ignored'), (sum(int(x[i]) for x in results) for i in range(3))))
lint = receipts['lint']['stdout'] + '\n' + receipts['lint']['stderr']
severities = collections.Counter(re.findall(r'^(error|warning|info):', lint, re.M))
changes = [p for p, h in baseline['files'].items()
           if not (r.ROOT/p).is_file() or r.sha(r.ROOT/p) != h]
prior_changes = [p for p, h in baseline['p1309_artifacts'].items()
                 if not (r.ROOT/p).is_file() or r.sha(r.ROOT/p) != h]
allowed = {'01_core/src/compiler/stdlib/loading.rs', '00_nucleo/prompts/compiler/stdlib/loading.md'}
before_untracked = {line[3:] for line in baseline['state']['status'].splitlines() if line.startswith('?? ')}
state = r.state()
outside = [line for line in state['status'].splitlines()
           if line[3:] not in allowed | before_untracked
           and not line[3:].startswith('00_nucleo/diagnosticos/p1310-')
           and line[3:] != '00_nucleo/materialization/typst-passo-1310.md']
code = (r.ROOT/'01_core/src/compiler/stdlib/loading.rs').read_bytes()
hash_b = hashlib.sha256(b''.join(line for line in code.splitlines(keepends=True)
                                if not line.startswith(b'//! @prompt-hash '))).hexdigest()[:8]
l0 = (r.ROOT/'00_nucleo/prompts/compiler/stdlib/loading.md').read_text()
declared_b = re.search(r'^Hash do Código: ([0-9a-f]{8})$', l0, re.M).group(1)
ignored = [line for line in output.splitlines() if line.startswith('test ') and line.endswith(' ... ignored')]
checks = {
    'red_real': receipts['unit-red']['exit'] == 101 and '3 failed' in receipts['unit-red']['stdout'],
    'all_green_commands': all(v['exit'] == 0 for k,v in receipts.items() if k != 'unit-red'),
    'workspace_results_present': bool(results),
    'no_failed_test': tests['failed'] == 0,
    'no_lint_error': severities['error'] == 0,
    'only_authorized_product_and_l0': set(changes) == allowed,
    'prior_artifacts_preserved': not prior_changes,
    'allowlist': not outside,
    'no_stage_or_commit': not state['staged'] and state['head'] == baseline['state']['head'],
    'code_hash_metadata': hash_b == declared_b,
    'p1308_deltas_expected': read('p1308-delta')['pass'],
    'baseline_binary_intact': r.sha(baseline['baseline_binary']['path']) == baseline['baseline_binary']['sha256'],
    'candidate_matches_build': r.sha(receipts['build']['candidate']['path']) == receipts['build']['candidate']['sha256'],
}
r.save('gates', {'at': r.now(), 'state': state, 'checks': checks, 'pass': all(checks.values()),
    'baseline_sha256': r.sha(D/'p1310-baseline.json'), 'runner_sha256': r.sha(__file__),
    'inputs': {'p1310-'+name+'.json': r.sha(D/('p1310-'+name+'.json')) for name in receipts},
    'workspace_tests': tests, 'ignored_tests': ignored,
    'lint': {k: severities[k] for k in ('error','warning','info')},
    'lint_by_rule': dict(collections.Counter(re.findall(r'\[(V[0-9]+)\]', lint))),
    'files_compared': len(baseline['files']), 'changed_files': changes,
    'p1309_changes': prior_changes, 'outside_allowlist': outside,
    'hash_b': hash_b, 'declared_hash_b': declared_b,
    'claim': 'Command/identity/preservation gates only. Independent A/B verdict issued separately; no product mutation score.'})
print(json.dumps({'checks': checks, 'tests': tests, 'lint': dict(severities)}, indent=2))
assert all(checks.values())
