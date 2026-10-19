#!/usr/bin/env python3
"""Strict preservation replay of frozen inputs; no oracle/parity equivalence claim."""
import datetime, gzip, hashlib, json, pathlib, subprocess, sys
ROOT = pathlib.Path(__file__).resolve().parents[6]
HERE = pathlib.Path(__file__).resolve().parent
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
assert len(sys.argv) == 5, 'replay.py OUTPUT VALIDATION_STATE GREEN_BUILD_RECEIPT EXPECTED_PRODUCT_SHA256'
out = HERE / sys.argv[1]
statepath = pathlib.Path(sys.argv[2]).resolve()
build = pathlib.Path(sys.argv[3]).resolve()
expected_product = sys.argv[4]
state = json.loads(gzip.decompress(statepath.read_bytes()) if statepath.suffix == '.gz' else statepath.read_bytes())
inputs = state['inputs']
assert inputs and all((ROOT/p).is_file() and sha(ROOT/p) == h for p,h in inputs.items())
state_sha = sha(statepath)
build_sha = sha(build)
canonical = lambda value: json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(',', ':'), allow_nan=False)
assert not out.exists()
product = ROOT / 'lab/target/product/debug/typst'
oracle = ROOT / 'lab/typst-original/target/release/typst'
assert sha(product) == expected_product
assert sha(oracle) == '9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
def observable(result):
    # Diagnostics and exit status are observable; only JSON transport formatting
    # is decoded. YAML/JSON encoder result strings remain exact string values.
    stdout = result['stdout']
    if result['returncode'] == 0:
        stdout = json.loads(stdout)
    return {'returncode': result['returncode'], 'stdout': stdout, 'stderr': result['stderr']}
r = {'utc_start': utc(), 'head': subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(), 'diff_head_stat': subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True), 'validation_state': {'path': str(statepath), 'sha256': state_sha}, 'build_receipt_path': str(build), 'build_receipt_sha256': sha(build), 'build_receipt': json.loads(build.read_text()), 'runner_sha256': sha(pathlib.Path(__file__)), 'comparison': 'Each product and oracle output compared to its own frozen baseline; no product/oracle byte-parity claim.', 'binaries': {'product': sha(product), 'oracle': sha(oracle)}, 'cases': [], 'failures': []}
for name in ['baseline-enabled-probes.json', 'baseline-html-projection.json']:
    baseline_path = HERE / name
    baseline = json.loads(baseline_path.read_text())
    for case in baseline['cases']:
        item = {'name': case['name'], 'expression': case['expression'], 'baseline_file': name, 'baseline_sha256': sha(baseline_path), 'results': {}}
        for role, binary in [('product',product), ('oracle',oracle)]:
            old = case['results'][role]
            argv = [str(binary), *old['argv'][1:]]
            begin = utc()
            proc = subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=20)
            got = {'argv':argv,'returncode':proc.returncode,'stdout':proc.stdout,'stderr':proc.stderr,'utc_start':begin,'utc_end':utc()}
            got['preserved'] = canonical(observable(got)) == canonical(observable(old))
            if not got['preserved']:
                r['failures'].append({'baseline':name,'name':case['name'],'role':role,'expected':observable(old),'actual':observable(got)})
            item['results'][role] = got
        r['cases'].append(item)
r['utc_end'] = utc()
r['binaries_after'] = {'product':sha(product),'oracle':sha(oracle)}
assert r['binaries_after'] == r['binaries']
r['head_after'] = subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
r['changed_build_inputs'] = [p for p,h in inputs.items() if not (ROOT/p).is_file() or sha(ROOT/p) != h]
r['validation_state_sha256_after'] = sha(statepath)
r['build_receipt_sha256_after'] = sha(build)
r['preserved_cases'] = sum(all(x['preserved'] for x in c['results'].values()) for c in r['cases'])
r['passed'] = not r['failures'] and not r['changed_build_inputs'] and r['head'] == r['head_after'] and sha(statepath) == state_sha and sha(build) == build_sha
out.write_text(json.dumps(r,indent=2)+'\n')
print(json.dumps({'output':str(out),'cases':len(r['cases']),'preserved':r['preserved_cases'],'failures':len(r['failures'])}))
sys.exit(not r['passed'])
