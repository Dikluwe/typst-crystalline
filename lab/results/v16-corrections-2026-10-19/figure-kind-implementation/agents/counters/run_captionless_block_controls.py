#!/usr/bin/env python3
"""Run frozen real-language extra controls; no Cargo or productive writes."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess


def utc():
    return datetime.now(timezone.utc).isoformat()


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def mismatches(inputs):
    return {p: {'expected': h, 'actual': sha(p) if Path(p).is_file() else None}
            for p, h in inputs.items() if not Path(p).is_file() or sha(p) != h}


p = argparse.ArgumentParser()
p.add_argument('--build-receipt', required=True)
p.add_argument('--binary', default='lab/target/product/debug/typst')
p.add_argument('--output', required=True, help='New evidence directory; never overwrites')
a = p.parse_args()
manifest_path = Path(__file__).with_name('captionless-block-manifest.json')
manifest = json.loads(manifest_path.read_text())
build = json.loads(Path(a.build_receipt).read_text())
assert build['exit'] == 0 and not build['changed_inputs'], 'Build must be successful with frozen inputs'
assert sha(a.binary) == build['product_sha256'], 'Executable differs from supplied build'
assert sha(manifest['oracle_binary']) == manifest['oracle_sha256'], 'Pinned oracle bytes changed'
assert not mismatches(build['inputs']), 'Current full inputs differ from frozen build'
corpus = dict(manifest['assets'])
for case in manifest['cases']:
    corpus[case['source']] = case['source_sha256']
    corpus[case['oracle_receipt']] = case['oracle_receipt_sha256']
assert not mismatches(corpus), 'Frozen source/asset/oracle receipt changed'
output = Path(a.output)
output.mkdir(parents=True, exist_ok=False)
receipt = {'utc_start': utc(), 'head': subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
           'state': 'uncommitted working tree; full supplied build input hashes verified before/after; evidence-only controls',
           'diff_head_stat': subprocess.check_output(['git','diff','HEAD','--stat'],text=True),
           'build_receipt': a.build_receipt, 'build_receipt_sha256': sha(a.build_receipt),
           'build_input_hashes': build['inputs'], 'binary': a.binary, 'binary_sha256': sha(a.binary),
           'oracle_commit': manifest['oracle_commit'], 'oracle_tree': manifest['oracle_tree'],
           'oracle_binary': manifest['oracle_binary'], 'oracle_sha256': manifest['oracle_sha256'],
           'manifest_path': str(manifest_path), 'manifest_sha256': sha(manifest_path),
           'source_asset_and_oracle_receipt_hashes': corpus, 'cases': []}
for case in manifest['cases']:
    pdf = output / (case['name']+'.pdf')
    argv = [a.binary, 'compile', str(Path(case['source']).resolve()), str(pdf.resolve())]
    start = utc()
    proc = subprocess.run(argv, capture_output=True, text=True, timeout=60)
    text_argv = ['pdftotext','-layout',str(pdf),'-']
    text_result = subprocess.run(text_argv,capture_output=True,text=True) if proc.returncode==0 else None
    normalized = ' '.join(text_result.stdout.split()) if text_result and text_result.returncode==0 else None
    result = {'name':case['name'], 'source_sha256':case['source_sha256'], 'argv':argv,
              'utc_start':start, 'utc_end':utc(), 'returncode':proc.returncode,'stdout':proc.stdout,'stderr':proc.stderr,
              'normalized_text':normalized,'text_extraction_argv':text_argv if text_result else None,
              'text_extraction_status':text_result.returncode if text_result else None,
              'pdf_sha256':sha(pdf) if pdf.exists() else None,
              'expected_returncode':case['expected_returncode'], 'expected_normalized_text':case['expected_normalized_text'],
              'expected_stdout':case['expected_stdout'], 'expected_stderr':case['expected_stderr']}
    result['passed'] = all([proc.returncode==case['expected_returncode'], proc.stdout==case['expected_stdout'],
                            proc.stderr==case['expected_stderr'], normalized==case['expected_normalized_text']])
    if case.get('expected_query'):
        expected_query = case['expected_query']
        query_argv = [a.binary, 'query', str(Path(case['source']).resolve()), expected_query['selector'], '--field', expected_query['field']]
        query_start = utc()
        query = subprocess.run(query_argv, capture_output=True, text=True, timeout=60)
        result['query'] = {'argv':query_argv, 'utc_start':query_start, 'utc_end':utc(),
                           'returncode':query.returncode, 'stdout':query.stdout, 'stderr':query.stderr,
                           'expected':expected_query}
        result['passed'] &= all([query.returncode==expected_query['returncode'], query.stdout==expected_query['stdout'], query.stderr==expected_query['stderr']])
    receipt['cases'].append(result)
receipt.update(utc_end=utc(), changed_build_inputs_after=mismatches(build['inputs']), changed_corpus_after=mismatches(corpus),
               binary_unchanged=sha(a.binary)==build['product_sha256'], oracle_unchanged=sha(manifest['oracle_binary'])==manifest['oracle_sha256'])
receipt['passed'] = all(c['passed'] for c in receipt['cases']) and not receipt['changed_build_inputs_after'] and not receipt['changed_corpus_after'] and receipt['binary_unchanged'] and receipt['oracle_unchanged']
(output/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'passed':receipt['passed'],'cases':[{k:c[k] for k in ['name','returncode','normalized_text','passed']} for c in receipt['cases']],'receipt':str(output/'receipt.json')}))
raise SystemExit(0 if receipt['passed'] else 1)
