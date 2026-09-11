"""Mechanical candidate runs using the frozen reference transport and literals.
No new oracle, no semantic verdict; r1/r2 Node prototypes remain historical.
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess

D = Path(__file__).resolve().parent
ROOT = D.parents[1]
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def pin(path):
    return dict(path=str(Path(path).resolve()), sha256=sha(path))
def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('mode', choices=['focal', 'expanded'])
    parser.add_argument('binary')
    parser.add_argument('seal')
    parser.add_argument('go')
    args = parser.parse_args()
    output = D / ('p1339-array-candidate-public-' + args.mode + '-r1.json')
    assert not output.exists(), 'Never overwrite evidence'
    rawpath = D / 'p1339-contract-array-expanded-raw-r1.json'
    oraclepath = D / 'p1339-contract-array-expanded-oracles-r1.json'
    transportpath = D / 'p1339-contract-array-expanded-recorder-r2.py'
    statepath = D / 'p1339-array-product-gates.py'
    for path, expected in [
        (rawpath, '53b9ab9f4b5c3ee0bcffa2ea248a101aa85dc9a03539217cbea9b15202fdee8e'),
        (oraclepath, '35c043a82bbfd56b1d9e334355a0acacaed57039bb0c47b02826bdd888b183c0'),
        (transportpath, '07ff1270ad4609db44cf39ef343bfa94308069da0af76a21c1806923d15cc145'),
        (statepath, 'd4dd934008030b8e9f5c87452f9fb3a60fa60e6e9171162a3d23df1818953304'),
    ]:
        assert sha(path) == expected, ('Protected input drift', str(path))
    transport = module('frozen_array_reference_transport', transportpath)
    gates = module('array_candidate_state_capture', statepath)
    raw = json.loads(rawpath.read_text())
    oracles = json.loads(oraclepath.read_text())
    key = lambda cell: (cell['id'], cell['profile'], cell['order'])
    bykey = {key(cell): cell for cell in oracles['cases']}
    references = [r for r in raw['runs'] if r['binary'] == 'vanilla' and
                  (args.mode == 'expanded' or (r['profile'] == 'default' and r['order'] == 'normal'))]
    assert len(references) == len(set(map(key, references))) == (336 if args.mode == 'expanded' else 28)
    paths = [args.binary, args.seal, args.go, __file__, rawpath, oraclepath, transportpath, statepath]
    before = gates.state()
    inputs_before = [pin(path) for path in paths]
    rows = []
    for ref in references:
        cell = bykey[key(ref)]
        assert cell['source'] == ref['source'] == ref['argv'][-1]
        assert hashlib.sha256(cell['source'].encode()).hexdigest() == cell['source_sha256'] == ref['source_sha256']
        result = transport.run([str(Path(args.binary).resolve()), *ref['argv'][1:]])
        observed = {name: result[name] for name in ['exit', 'stdout', 'stderr']}
        valid = result['exit'] in (0, 1) and all(isinstance(result[name], str) for name in ['stdout', 'stderr'])
        expected = cell['expected']
        rows.append(dict(result, id=cell['id'], profile=cell['profile'], order=cell['order'],
                         source=cell['source'], source_sha256=cell['source_sha256'],
                         transport_valid=valid, expected=expected,
                         literal_match=None if expected is None else valid and observed == expected))
    after = gates.state()
    inputs_after = [pin(path) for path in paths]
    unchanged = inputs_before == inputs_after and all(before[field] == after[field] for field in
        ['head', 'modified_sha256', 'untracked_product_sha256', 'protected_test_inputs'])
    receipt = dict(schema='p1339-array-candidate-public-engineering-v3',
        role='Implementer execution and literal comparison only; independent verdict required',
        mode=args.mode, before=before, after=after, inputs_before=inputs_before, inputs_after=inputs_after,
        all_inputs_unchanged=unchanged, cwd=str(ROOT), stdin='', execution_order='Exact measured vanilla reference order',
        environment_overrides={'NO_COLOR': '1', 'PYTHONDONTWRITEBYTECODE': '1'},
        relevant_environment={k:v for k,v in transport.ENV.items() if k in
            ['PATH','LANG','LC_ALL','LC_CTYPE','TZ','HOME','NO_COLOR','PYTHONDONTWRITEBYTECODE','SOURCE_DATE_EPOCH']
            or k.startswith(('TYPST_','FONTCONFIG_','XDG_'))},
        rows=rows, processes=len(rows), literal_matches=sum(r['literal_match'] is True for r in rows),
        literal_failures=sum(r['literal_match'] is False for r in rows),
        transport_failures=sum(not r['transport_valid'] for r in rows),
        exploratory_unknown=sum(r['expected'] is None for r in rows), global_P1339_PASS=False)
    text = json.dumps(receipt, ensure_ascii=True, indent=2)
    patch = '*** Begin Patch\n*** Add File: '+str(output)+'\n'+''.join('+'+line+'\n' for line in text.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'], cwd=ROOT, input=patch, text=True, capture_output=True, check=True)
    print(json.dumps(dict(receipt=pin(output), matches=receipt['literal_matches'], failures=receipt['literal_failures'],
        transport_failures=receipt['transport_failures'], exploratory_unknown=receipt['exploratory_unknown'], unchanged=unchanged)))
if __name__ == '__main__':
    main()
