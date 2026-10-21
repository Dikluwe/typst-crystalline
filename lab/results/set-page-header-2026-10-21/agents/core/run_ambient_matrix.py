#!/usr/bin/env python3
"""Serial bilateral observations of frozen running-content sources; no Cargo.

This worker records observations, including product failures and oracle refusals.
It does not ratify architecture, replace source validity review, normalize errors,
or claim PDF bytes/geometric layout are the language acceptance contract.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import subprocess
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[5]
HERE = Path(__file__).resolve().parent
ORACLE = ROOT / 'lab/typst-original/target/release/typst'
ORACLE_SHA = '9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def utc():
    return datetime.now(timezone.utc).isoformat()


def command(argv, dest, timeout):
    started = utc()
    try:
        p = subprocess.run([str(x) for x in argv], cwd=ROOT, capture_output=True,
                           timeout=timeout)
        out, err, code, timed_out = p.stdout, p.stderr, p.returncode, False
    except subprocess.TimeoutExpired as e:
        out, err, code, timed_out = e.stdout or b'', e.stderr or b'', None, True
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.with_suffix('.stdout').write_bytes(out)
    dest.with_suffix('.stderr').write_bytes(err)
    record = dict(argv=[str(x) for x in argv], utc_start=started, utc_end=utc(),
                  exit=code, timed_out=timed_out,
                  stdout=out.decode('utf-8', errors='replace'),
                  stderr=err.decode('utf-8', errors='replace'),
                  stdout_sha256=hashlib.sha256(out).hexdigest(),
                  stderr_sha256=hashlib.sha256(err).hexdigest())
    dest.with_suffix('.json').write_text(json.dumps(record, indent=2) + '\n')
    return record


def json_observation(record):
    if record['exit'] != 0 or record['timed_out']:
        return {'valid': False, 'reason': 'command did not succeed'}
    try:
        return {'valid': True, 'value': json.loads(record['stdout'])}
    except (ValueError, TypeError) as error:
        return {'valid': False, 'reason': str(error)}


def observe(binary, case, source, dest, timeout):
    dest.mkdir(parents=True)
    target = case['target']
    artifact = dest / ('document.pdf' if target == 'pdf' else 'document.html')
    argv = [binary, 'compile', source, artifact]
    if target == 'html':
        argv += ['--features', 'html', '--format', 'html']
    result = {'compile': command(argv, dest / 'compile', timeout), 'queries': []}
    if artifact.exists():
        result['artifact_sha256'] = sha(artifact)
    if result['compile']['exit'] == 0 and target == 'pdf' and artifact.exists():
        info = command(['pdfinfo', artifact], dest / 'pdfinfo', timeout)
        result['pdfinfo'] = info
        match = re.search(r'^Pages:\s+(\d+)\s*$', info['stdout'], re.M)
        result['page_count'] = int(match.group(1)) if match and info['exit'] == 0 else None
        result['pages'] = []
        for page in range(1, (result['page_count'] or 0) + 1):
            text = command(['pdftotext', '-layout', '-f', str(page), '-l', str(page),
                            artifact, '-'], dest / f'page-{page}-text', timeout)
            bbox = command(['pdftotext', '-bbox', '-f', str(page), '-l', str(page),
                            artifact, '-'], dest / f'page-{page}-bbox', timeout)
            observed = {'page': page, 'text': text, 'bbox': bbox}
            try:
                tree = ET.fromstring(bbox['stdout'])
                observed['words_in_pdf_observer_order'] = [
                    {'text': e.text or '', 'bounds': dict(e.attrib)}
                    for e in tree.iter() if e.tag.split('}')[-1] == 'word']
                observed['bbox_valid'] = bbox['exit'] == 0
            except ET.ParseError as error:
                observed.update(bbox_valid=False, bbox_error=str(error))
            result['pages'].append(observed)
    for i, selector in enumerate(case['queries']):
        run = command([binary, 'query', source, selector, '--field', 'value'],
                      dest / f'query-{i}', timeout)
        result['queries'].append({'selector': selector, 'command': run,
                                  'json': json_observation(run)})
    return result


def compare(oracle, product):
    oq, pq = oracle['queries'], product['queries']
    queries = [dict(selector=a['selector'], both_valid=a['json']['valid'] and b['json']['valid'],
                    equal=(a['json']['value'] == b['json']['value'])
                    if a['json']['valid'] and b['json']['valid'] else False)
               for a, b in zip(oq, pq)]
    op, pp = oracle.get('pages', []), product.get('pages', [])
    return {'compile_exit_equal': oracle['compile']['exit'] == product['compile']['exit'],
            'compile_stderr_exact_equal': oracle['compile']['stderr'] == product['compile']['stderr'],
            'page_count_equal': (oracle.get('page_count') == product.get('page_count'))
                if 'page_count' in oracle and 'page_count' in product else None,
            'queries': queries,
            'per_page_raw_text_equal': [a['text']['stdout'] == b['text']['stdout']
                                       for a, b in zip(op, pp)] if len(op) == len(pp) else None,
            'per_page_words_equal': [[w['text'] for w in a.get('words_in_pdf_observer_order', [])]
                                     == [w['text'] for w in b.get('words_in_pdf_observer_order', [])]
                                     if a.get('bbox_valid') and b.get('bbox_valid') else False
                                     for a, b in zip(op, pp)] if len(op) == len(pp) else None,
            'qualification': 'Raw PDF extraction order/geometric bounds are observer output, not semantic query order. Invalid/missing query JSON never compares equal.'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build-receipt', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--product', type=Path, default=ROOT / 'lab/target/product/debug/typst')
    parser.add_argument('--timeout', type=int, default=120)
    args = parser.parse_args()
    build_path, product = args.build_receipt.resolve(), args.product.resolve()
    matrix_path = HERE / 'ambient-matrix.json'
    matrix = json.loads(matrix_path.read_text())
    build = json.loads(build_path.read_text())
    assert build['exit'] == 0 and build['changed_inputs'] == []
    assert sha(product) == build['product_sha256']
    assert sha(ORACLE) == ORACLE_SHA == matrix['oracle_sha256']
    # Freeze the build's complete input set, plus the worker and exact measurement sources.
    inputs = {p: sha(ROOT / p) for p in build['inputs']}
    assert inputs == build['inputs'], 'current productive inputs differ from build receipt'
    additional = {str(matrix_path.relative_to(ROOT)): sha(matrix_path),
                  str(Path(__file__).resolve().relative_to(ROOT)): sha(__file__)}
    for case in matrix['cases']:
        path = ROOT / case['source_file']
        assert path.read_text() == case['source'] and sha(path) == case['source_sha256']
        additional[case['source_file']] = sha(path)
    assert not args.output.exists(), 'never overwrite an earlier measurement'
    output = args.output.resolve()
    output.mkdir(parents=True)
    receipt = {'utc_start': utc(), 'head': subprocess.check_output(
        ['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
        'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], cwd=ROOT, text=True),
        'matrix_sha256': sha(matrix_path), 'worker_sha256': sha(__file__),
        'build_receipt': str(build_path.relative_to(ROOT)), 'build_receipt_sha256': sha(build_path),
        'product': str(product), 'product_sha256': sha(product), 'oracle': str(ORACLE),
        'oracle_sha256': sha(ORACLE), 'oracle_commit': matrix['oracle_commit'],
        'oracle_tree': matrix['oracle_tree'], 'inputs': inputs,
        'measurement_inputs': additional, 'cases': []}
    try:
        for case in matrix['cases']:
            source = ROOT / case['source_file']
            dest = output / case['name']
            dest.mkdir()
            (dest / 'source.typ').write_bytes(source.read_bytes())
            row = {'name': case['name'], 'source_file': case['source_file'],
                   'source_sha256': sha(source), 'target': case['target'],
                   'classification': case['classification'],
                   'language_properties_to_measure': case['language_properties_to_measure']}
            row['oracle'] = observe(ORACLE, case, source, dest / 'oracle', args.timeout)
            row['product'] = observe(product, case, source, dest / 'product', args.timeout)
            row['observations'] = compare(row['oracle'], row['product'])
            receipt['cases'].append(row)
            (output / 'receipt.partial.json').write_text(json.dumps(receipt, indent=2) + '\n')
            print(case['name'], 'oracle=', row['oracle']['compile']['exit'],
                  'product=', row['product']['compile']['exit'], flush=True)
    finally:
        receipt.update(utc_end=utc(), changed_inputs=[p for p, h in inputs.items()
                       if not (ROOT / p).is_file() or sha(ROOT / p) != h],
                       changed_measurement_inputs=[p for p, h in additional.items()
                       if not (ROOT / p).is_file() or sha(ROOT / p) != h],
                       product_unchanged=sha(product) == receipt['product_sha256'],
                       oracle_unchanged=sha(ORACLE) == ORACLE_SHA,
                       build_receipt_unchanged=sha(build_path) == receipt['build_receipt_sha256'])
        (output / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    assert receipt['changed_inputs'] == [] and receipt['changed_measurement_inputs'] == []
    assert receipt['product_unchanged'] and receipt['oracle_unchanged'] and receipt['build_receipt_unchanged']
    assert len(receipt['cases']) == len(matrix['cases'])


if __name__ == '__main__':
    main()
