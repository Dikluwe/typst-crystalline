#!/usr/bin/env python3
"""Checks every complete observable against frozen pre-candidate expectations."""
import collections, hashlib, json, pathlib, re, sys

def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def main():
    freeze = json.loads(pathlib.Path('00_nucleo/diagnosticos/p1325-ab-freeze.json').read_text())
    changed = [p for p, h in freeze['protected'].items() if sha(p) != h]
    prompt = pathlib.Path(freeze['prompt']['path']).read_bytes()
    norm = re.sub(rb'^Hash do C\xc3\xb3digo: [0-9a-f]{8}\n', b'', prompt, flags=re.M)
    if hashlib.sha256(norm).hexdigest() != freeze['prompt']['normalized_sha256']:
        changed.append(freeze['prompt']['path'])
    expectations = json.loads(pathlib.Path('00_nucleo/diagnosticos/p1325-ab-expectations.json').read_text())['expected']
    receipt = json.loads(pathlib.Path(sys.argv[1]).read_text())
    by_key = {(x['case'],x['profile']):x for x in expectations}
    seen = collections.Counter()
    failures = []
    for row in receipt['rows']:
        key = (row['case'], row['profile'])
        seen[(row['order'],row['product'],key)] += 1
        expected = by_key[key][row['product']]
        actual = {k:row[k] for k in ['exit','stdout','stderr']}
        if actual != expected:
            failures.append(dict(case=row['case'], profile=row['profile'], product=row['product'], order=row['order'], expected=expected, actual=actual))
    for order in ['normal','repeat','reverse']:
        for product in ['baseline','vanilla','candidate']:
            for key in by_key:
                if seen[(order,product,key)] != 1:
                    failures.append(dict(reason='missing_or_duplicate', order=order,product=product,key=key,count=seen[(order,product,key)]))
    verdict = dict(status='PASS' if not changed and not failures else 'FAIL', receipt=sys.argv[1], receipt_sha256=sha(sys.argv[1]), freeze_sha256=sha('00_nucleo/diagnosticos/p1325-ab-freeze.json'), changed_protected=changed, compared_rows=len(receipt['rows']), classifications=dict(collections.Counter(x['classification'] for x in expectations)), failures=failures)
    print(json.dumps(verdict, ensure_ascii=False, indent=2))
    return bool(changed or failures)

if __name__ == '__main__': sys.exit(main())
