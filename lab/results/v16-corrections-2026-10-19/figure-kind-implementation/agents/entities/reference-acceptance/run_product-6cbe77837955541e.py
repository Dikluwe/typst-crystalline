"""Replay frozen language probes against a provenance-verified product build."""
from pathlib import Path
import argparse
import datetime
import gzip
import hashlib
import json
import re
import subprocess

ROOT = Path.cwd()
BASE = Path(__file__).resolve().parent
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
parser = argparse.ArgumentParser()
parser.add_argument('--product', required=True)
parser.add_argument('--build-receipt', required=True)
parser.add_argument('--out', required=True)
parser.add_argument('--matrix', default='matrix.json')
parser.add_argument('--oracle-receipt', default='oracle/receipt.json')
parser.add_argument('--policies', default='policies.json')
parser.add_argument('--baseline', action='store_true', help='Historical pre-Ref receipt: validate frozen build provenance instead of current modified source identity')
args = parser.parse_args()
binary = Path(args.product).resolve()
build_path = Path(args.build_receipt).resolve()
build = json.loads(build_path.read_text())
assert sha(binary) == build['product_sha256']
if args.baseline:
    assert build['build_job']['exit'] == 0
    state = build_path.parent / build['source_state']
    assert sha(state) == build['source_state_sha256']
    assert (build_path.parent / build['unchanged_build_through_stop']).is_file()
else:
    assert build['exit'] == 0 and build['changed_inputs'] == []
    assert build['inputs']
    for name, expected in build['inputs'].items():
        assert sha(ROOT / name) == expected, ('changed build input', name)
oracle_path = BASE / args.oracle_receipt
oracle = json.loads(oracle_path.read_text())
assert oracle['binary_sha256'] == '9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
assert sha(BASE / args.matrix) == oracle['matrix_sha256']
matrix = json.loads((BASE / args.matrix).read_text())
policy = json.loads((BASE / args.policies).read_text())
out = Path(args.out).resolve()
out.mkdir(exist_ok=False)
receipt = {'utc_start': utc(), 'head': subprocess.check_output(['git','rev-parse','HEAD'], text=True).strip(),
           'product': str(binary), 'product_sha256': sha(binary), 'build_receipt': str(build_path),
           'build_receipt_sha256': sha(build_path), 'historical_baseline': args.baseline,
           'oracle_receipt_sha256': sha(oracle_path), 'matrix_sha256': sha(BASE/args.matrix),
           'policies_sha256': sha(BASE/args.policies), 'runner_sha256': sha(Path(__file__)), 'cases': []}
for case, expected in zip(matrix['cases'], oracle['cases']):
    assert case['name'] == expected['name']
    source = BASE / (case['name'] + '.typ')
    assert sha(source) == expected['source_sha256']
    assert all(sha(BASE/name)==digest for name,digest in expected['extra_sha256'].items())
    output = out / (case['name'] + '.' + case['target'])
    commands = [['compile',str(source),str(output)]]
    if case['target']=='html': commands[0] += ['--features','html']
    if case['query']: commands.append(['query',str(source),case.get('query_selector','<probe>'),'--field','value'])
    record = {'name':case['name'], 'policy':policy['cases'][case['name']], 'runs':[]}
    for command in commands:
        start = utc()
        argv = [str(binary),*command]
        try:
            run = subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=30)
            result = {'exit':run.returncode,'stdout':run.stdout,'stderr':run.stderr}
        except subprocess.TimeoutExpired as error:
            result = {'timeout':True,'stdout':str(error.stdout),'stderr':str(error.stderr)}
        record['runs'].append({'utc_start':start,'utc_end':utc(),'argv':argv,**result})
    if output.exists():
        record['output_sha256'] = sha(output)
        if case['target']=='pdf':
            record['rendered_text'] = subprocess.run(['pdftotext','-layout',str(output),'-'],capture_output=True,text=True).stdout
        else: record['rendered_html'] = output.read_text()
    actual = record['runs']
    checks = {}
    if record['policy']['class']=='required-language':
        checks['compile_success'] = actual[0].get('exit')==0
        if case['query']:
            try: checks['language_values'] = json.loads(actual[1]['stdout']) == json.loads(expected['runs'][1]['stdout'])
            except (ValueError,KeyError):checks['language_values']=False
        text = record.get('rendered_text','')
        for token,count in record['policy'].get('render_token_counts',{}).items():
            checks['render_'+token] = len(re.findall(r'(?<![A-Za-z_-])'+re.escape(token)+r'(?![A-Za-z_-])',text))==count
    elif record['policy']['class']=='required-error':
        checks['terminates_with_error'] = actual[0].get('exit')==1 and not actual[0].get('timeout')
        checks['error_message'] = record['policy']['message'] in actual[0].get('stderr','')
        origin = re.search(re.escape(case['name']+'.typ')+r':(\d+):(\d+)', actual[0].get('stderr',''))
        observed_origin = list(map(int, origin.groups())) if origin else None
        checks['error_origin'] = observed_origin == record['policy']['origin']
        record['observed_error_origin'] = observed_origin
        record['expected_error_origin'] = record['policy']['origin']
    elif record['policy']['class']=='required-termination':
        checks['terminates_with_error'] = actual[0].get('exit')==1 and not actual[0].get('timeout')
    else:
        checks['oracle_success_class'] = (actual[0].get('exit')==0) == (expected['runs'][0].get('exit')==0)
        if expected['runs'][0].get('exit'):
            oracle_message = expected['runs'][0]['stderr'].splitlines()[0].removeprefix('error: ')
            checks['oracle_error_message'] = oracle_message in actual[0].get('stderr','')
        record['comparison_note'] = 'Measured control/debt; no green certification from this row. See explicit policy and immutable oracle result.'
    record['checks'] = checks
    record['pass'] = all(checks.values()) if checks else None
    receipt['cases'].append(record)
    print(case['name'], record['pass'], flush=True)
receipt['utc_end']=utc()
receipt['oracle_comparison_failures']=[c['name'] for c in receipt['cases'] if c['pass'] is False]
receipt['required_failures']=[c['name'] for c in receipt['cases'] if c['pass'] is False and c['policy']['class'].startswith('required-')]
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('required_failures',receipt['required_failures'])
