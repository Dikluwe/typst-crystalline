"""Strict successor projection; original R3 observations remain immutable."""
import copy, importlib.util, json, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('receipt', D/'p1335-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
RAW = D/'p1335-transversal-r3.json'
CASES = D/'p1335-transversal-cases.json'
FREEZE = D/'p1335-transversal-freeze-r3.json'
POLICY = D/'p1335-review-cli-capability-policy.md'

def classify(row, data, freeze, outroot):
    ident, phase = row['id'], row['phase']
    if ident not in ('html-legacy', 'preserve-pdf') or phase not in ('normal', 'repeat', 'reverse'):
        return 'Unknown'
    case = next(c for c in data['warning_cases'] if c['id'] == ident)
    o = row['observations']['vanilla']
    binary = freeze['binaries']['vanilla']
    output = str(Path(outroot)/'warning'/f'{phase}-vanilla-{ident}.{case["format"]}')
    args = [a.replace(str(D/'p1323-fixtures'), data['html']).replace('{output}', output) for a in case['argv']]
    source = Path(data['html'])/'hello.typ'
    if r.sha(source) != freeze['inputs'].get(str(source)) or r.sha(binary['path']) != binary['sha256']:
        return 'Unknown'
    if row['case'] != case or o['command'] != [binary['path'], *args] or o['binary_sha256'] != binary['sha256']:
        return 'Unknown'
    if o['cwd'] != str(r.ROOT) or o['stdin_base64'] != case['stdin_base64'] or o['stdout'] != '' or o['exit_code'] != 2 or o['artifact'] is not None:
        return 'Unknown'
    if ident == 'html-legacy':
        expected = f"error: unrecognized subcommand '{source}'\n\nUsage: typst [OPTIONS] <COMMAND>\n\nFor more information, try '--help'.\n"
    else:
        expected = "error: unexpected argument '--document-id' found\n\n  tip: to pass '--document-id' as a value, use '-- --document-id'\n\nUsage: typst compile --format <FORMAT> <--jobs <JOBS>|--features <FEATURES>|--diagnostic-format <DIAGNOSTIC_FORMAT>> <INPUT> <OUTPUT>\n\nFor more information, try '--help'.\n"
    return 'PUBLIC_CLI_ABSENT' if o['stderr'] == expected else 'Unknown'

def run():
    before = r.state(); r.verify(before)
    data = json.loads(CASES.read_text()); freeze = json.loads(FREEZE.read_text()); raw = json.loads(RAW.read_text())
    for p, h in freeze['inputs'].items():
        assert r.sha(p) == h, p
    rows, attacks = [], []
    for row in raw['warnings']:
        if row['id'] not in ('html-legacy', 'preserve-pdf'):
            continue
        state = classify(row, data, freeze, raw['output_root']); assert state == 'PUBLIC_CLI_ABSENT'
        rows.append(dict(id=row['id'], phase=row['phase'], side='vanilla', original_complete=row['observations']['vanilla']['complete'], successor_state=state,
                         interpretation='CLI capability absent; no export/warning parity claim; C warning contract separate'))
        if row['phase'] != 'normal':
            continue
        for attack in ('missing-stderr', 'truncated', 'generic-error', 'crash', 'stdout', 'wrong-command', 'wrong-binary', 'fake-artifact'):
            altered = copy.deepcopy(row); o = altered['observations']['vanilla']
            if attack == 'missing-stderr': o['stderr'] = ''
            elif attack == 'truncated': o['stderr'] = o['stderr'].splitlines()[0]+'\n'
            elif attack == 'generic-error': o['stderr'] = 'error: unknown failure\n'
            elif attack == 'crash': o['exit_code'] = -11
            elif attack == 'stdout': o['stdout'] = 'unexplained output'
            elif attack == 'wrong-command': o['command'][1] = '--version'
            elif attack == 'wrong-binary': o['binary_sha256'] = '0'*64
            elif attack == 'fake-artifact': o['artifact'] = {'path': 'fabricated', 'sha256': '0'*64}
            result = classify(altered, data, freeze, raw['output_root']); assert result == 'Unknown'
            attacks.append(dict(id=row['id'], attack=attack, state=result))
    assert len(rows) == 6
    after = r.state(); r.verify(after)
    r.save('cli-capability-overlay', dict(at=r.now(), before=before, after=after,
        inputs={str(p):r.sha(p) for p in [RAW, CASES, FREEZE, POLICY, Path(__file__), D/'p1335-manifest.json']},
        rows=rows, controls=attacks, unknown=0,
        scope='Successor interpretation only of six retained exit-2 receipts; raw R3 and prior Unknown review retained; no product rerun or production mutation score'))

if __name__ == '__main__': run()
