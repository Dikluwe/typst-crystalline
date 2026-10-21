#!/usr/bin/env python3
"""Requalify recorded observations without executing products or changing receipts.

No aggregate acceptance PASS is emitted. Oracle refusals, strict diagnostic differences,
invalid observers and source-valid successful observations remain separate. This is a
read-only downstream interpretation of the immutable bilateral worker evidence.
"""
import argparse
from datetime import datetime, timezone
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[5]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def success(command):
    return type(command.get('exit')) is int and command['exit'] == 0 and command.get('timed_out') is False


def errors(command):
    return re.findall(r'^error: (.*)$', command.get('stderr', ''), re.M)


def command_class(command):
    if command.get('timed_out'):
        return 'timeout-not-language-refusal'
    if success(command):
        return 'success'
    if type(command.get('exit')) is int and command['exit'] > 0 and errors(command):
        return 'sourced-refusal-needs-language-classification'
    return 'invalid-command-or-abort'


def typed_equal(left, right):
    """Preserve JSON type and container shape; Python bool is not JSON number."""
    if type(left) is not type(right):
        return False
    if isinstance(left, dict):
        return left.keys() == right.keys() and all(typed_equal(left[k], right[k]) for k in left)
    if isinstance(left, list):
        return len(left) == len(right) and all(typed_equal(a, b) for a, b in zip(left, right))
    return left == right


def json_value(command):
    if not success(command):
        return {'valid': False, 'reason': 'query command failed or timed out'}
    try:
        value = json.loads(command['stdout'], parse_constant=lambda token: (_ for _ in ()).throw(ValueError('non-JSON constant ' + token)))
        return {'valid': True, 'value': value}
    except (ValueError, TypeError, KeyError) as error:
        return {'valid': False, 'reason': str(error)}


def query_comparison(oracle, product):
    a, b = json_value(oracle), json_value(product)
    valid = a['valid'] and b['valid']
    return {'oracle': a, 'product': b, 'both_valid': valid,
            'typed_equal': typed_equal(a['value'], b['value']) if valid else False}


def raw_text_equal(oracle, product):
    return success(oracle) and success(product) and oracle['stdout'] == product['stdout']


def check_command(command, stem):
    """Validate duplicate receipt payloads against the retained raw byte streams."""
    for channel in ('stdout', 'stderr'):
        data = stem.with_suffix('.' + channel).read_bytes()
        assert hashlib.sha256(data).hexdigest() == command[channel + '_sha256'], (stem, channel, 'hash')
        assert data.decode('utf-8', errors='replace') == command[channel], (stem, channel, 'text')
    saved = json.loads(stem.with_suffix('.json').read_text())
    assert typed_equal(saved, command), (stem, 'saved command receipt')


class DocumentTree(HTMLParser):
    VOID = {'area', 'base', 'br', 'col', 'embed', 'hr', 'img', 'input', 'link', 'meta', 'param', 'source', 'track', 'wbr'}

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.root = {'tag': '#document', 'children': []}
        self.stack = [self.root]
        self.invalid = []

    def handle_decl(self, decl):
        self.stack[-1]['children'].append({'tag': '#declaration', 'value': decl.lower()})

    def handle_starttag(self, tag, attrs):
        node = {'tag': tag, 'attrs': sorted(attrs, key=lambda a: (a[0], a[1] or '')), 'children': []}
        self.stack[-1]['children'].append(node)
        if tag not in self.VOID:
            self.stack.append(node)

    def handle_startendtag(self, tag, attrs):
        self.handle_starttag(tag, attrs)
        if tag not in self.VOID:
            self.handle_endtag(tag)

    def handle_endtag(self, tag):
        if len(self.stack) == 1 or self.stack[-1]['tag'] != tag:
            self.invalid.append('unbalanced close ' + tag)
        else:
            self.stack.pop()

    def handle_data(self, data):
        if data:
            self.stack[-1]['children'].append(data)


def nodes(tree, tag):
    if isinstance(tree, dict):
        if tree.get('tag') == tag:
            yield tree
        for child in tree.get('children', []):
            yield from nodes(child, tag)


def visible_text(tree):
    if isinstance(tree, str):
        return tree
    return ''.join(visible_text(c) for c in tree.get('children', []))


def html_observation(path):
    parser = DocumentTree()
    parser.feed(path.read_text())
    parser.close()
    bodies = list(nodes(parser.root, 'body'))
    valid = not parser.invalid and len(parser.stack) == 1 and len(bodies) == 1
    return {'valid': valid, 'dom': parser.root, 'body_text': visible_text(bodies[0]) if len(bodies) == 1 else None,
            'parse_issues': parser.invalid}


def selfchecks():
    ok = {'exit': 0, 'timed_out': False, 'stdout': '[]', 'stderr': ''}
    tests = []
    def check(name, actual):
        assert actual is True, name
        tests.append({'name': name, 'pass': True})
    check('True differs from integer1 recursively', not typed_equal({'is18': [True]}, {'is18': [1]}))
    check('valid JSON null equals null', query_comparison({**ok, 'stdout': 'null'}, {**ok, 'stdout': 'null'})['typed_equal'])
    check('null differs from missing or invalid JSON', not query_comparison({**ok, 'stdout': 'null'}, {**ok, 'stdout': ''})['typed_equal'])
    check('two invalid JSON payloads never compare equal', not query_comparison({**ok, 'stdout': 'bad'}, {**ok, 'stdout': 'bad'})['typed_equal'])
    failed = {**ok, 'exit': 1, 'stdout': ''}
    check('two failed empty observers never compare equal', not raw_text_equal(failed, failed))
    timeout = {**ok, 'exit': None, 'timed_out': True, 'stdout': ''}
    check('dual timeouts never count as successful equal commands', not raw_text_equal(timeout, timeout) and command_class(timeout) != 'success')
    check('nonzero query with valid-looking JSON remains invalid', not query_comparison({**ok, 'exit': 1}, ok)['typed_equal'])
    check('container cardinality remains exact', not typed_equal([1], [1, 2]))
    return tests


def review(receipt_path):
    receipt = json.loads(receipt_path.read_text())
    matrix_path = ROOT / 'lab/results/set-page-header-2026-10-21/agents/parity/matrix.json'
    matrix = json.loads(matrix_path.read_text())
    assert sha(matrix_path) == receipt['matrix_sha256']
    assert receipt['changed_inputs'] == receipt['changed_measurement_inputs'] == []
    assert receipt['product_unchanged'] and receipt['oracle_unchanged'] and receipt['build_receipt_unchanged']
    assert len(receipt['cases']) == len(matrix['cases'])
    assert [x['name'] for x in receipt['cases']] == [x['name'] for x in matrix['cases']]
    rows = []
    for row, specification in zip(receipt['cases'], matrix['cases']):
        name = row['name']
        assert re.fullmatch(r'[a-z0-9-]+', name)
        dest = receipt_path.parent / name
        assert sha(dest / 'source.typ') == row['source_sha256'] == specification['source_sha256']
        assert (dest / 'source.typ').read_text() == specification['source']
        states = {}
        for who in ('oracle', 'product'):
            data = row[who]
            base = dest / who
            check_command(data['compile'], base / 'compile')
            state = {'compile_class': command_class(data['compile']), 'error_messages': errors(data['compile']),
                     'artifact_verified': False}
            artifact = base / ('document.pdf' if row['target'] == 'pdf' else 'document.html')
            if success(data['compile']):
                assert artifact.is_file() and sha(artifact) == data['artifact_sha256'], (name, who, 'artifact')
                state['artifact_verified'] = True
            assert len(data['queries']) == len(specification['queries'])
            for index, (query, selector) in enumerate(zip(data['queries'], specification['queries'])):
                assert query['selector'] == selector
                check_command(query['command'], base / f'query-{index}')
            if success(data['compile']) and row['target'] == 'pdf':
                check_command(data['pdfinfo'], base / 'pdfinfo')
                info = data['pdfinfo']
                match = re.search(r'^Pages:\s+(\d+)\s*$', info['stdout'], re.M)
                count = int(match.group(1)) if success(info) and match else None
                pages = data.get('pages', [])
                state['pdf_observers_valid'] = type(count) is int and count > 0 and count == data.get('page_count') == len(pages)
                for index, page in enumerate(pages, 1):
                    assert page['page'] == index
                    check_command(page['text'], base / f'page-{index}-text')
                    check_command(page['bbox'], base / f'page-{index}-bbox')
                    state['pdf_observers_valid'] &= success(page['text']) and success(page['bbox']) and page.get('bbox_valid') is True
            if success(data['compile']) and row['target'] == 'html':
                state['html'] = html_observation(artifact)
            states[who] = state
        a, b = row['oracle'], row['product']
        qualified_success = all(success(x['compile']) for x in (a, b))
        comparison = {'both_compiled_successfully': qualified_success,
                      'compile_exit_equal_is_acceptance': False,
                      'error_messages_equal': errors(a['compile']) == errors(b['compile']),
                      'complete_stderr_exact_equal': a['compile']['stderr_sha256'] == b['compile']['stderr_sha256'],
                      'queries': [{'selector': qa['selector'], **query_comparison(qa['command'], qb['command'])}
                                  for qa, qb in zip(a['queries'], b['queries'])]}
        if row['target'] == 'pdf':
            observers_valid = qualified_success and all(states[w].get('pdf_observers_valid', False) for w in ('oracle', 'product'))
            comparison['both_pdf_observers_valid'] = observers_valid
            comparison['physical_page_count_equal'] = a.get('page_count') == b.get('page_count') if observers_valid else False
            comparison['per_page_raw_text_equal'] = [raw_text_equal(pa['text'], pb['text']) for pa, pb in zip(a.get('pages', []), b.get('pages', []))] if observers_valid else None
            comparison['per_page_words_equal'] = [[w['text'] for w in pa['words_in_pdf_observer_order']] == [w['text'] for w in pb['words_in_pdf_observer_order']] for pa, pb in zip(a.get('pages', []), b.get('pages', []))] if observers_valid else None
            comparison['observer_comparison_limit'] = 'PDF extraction order and whitespace are retained observations, not semantic counter/query order or geometric acceptance.'
        else:
            html_valid = qualified_success and all(states[w].get('html', {}).get('valid', False) for w in ('oracle', 'product'))
            comparison['both_html_observers_valid'] = html_valid
            comparison['semantic_dom_equal'] = typed_equal(states['oracle']['html']['dom'], states['product']['html']['dom']) if html_valid else False
            if name == 'html-header-context-not-admitted':
                comparison['BODY_preserved_and_panic_inert'] = html_valid and all(states[w]['html']['body_text'] == 'BODY' and 'HTML_HEADER_MUST_NOT_RUN' not in visible_text(states[w]['html']['dom']) for w in ('oracle', 'product'))
        rows.append({'name': name, 'target': row['target'], 'classification': 'oracle-success-source' if success(a['compile']) else 'oracle-refusal-or-invalid-needs-explicit-review', 'states': states, 'checks': comparison})
    return {'utc': datetime.now(timezone.utc).isoformat(), 'mode': 'Downstream raw-evidence qualification only; no products/Cargo/receipt mutation',
            'receipt': str(receipt_path.relative_to(ROOT)), 'receipt_sha256': sha(receipt_path), 'matrix_sha256': sha(matrix_path),
            'verifier_sha256': sha(__file__), 'head': receipt['head'], 'product_sha256': receipt['product_sha256'],
            'oracle_commit': receipt['oracle_commit'], 'oracle_tree': receipt['oracle_tree'], 'oracle_sha256': receipt['oracle_sha256'],
            'selfchecks': selfchecks(), 'all_retained_command_and_source_artifact_hashes_checked': True, 'cases': rows,
            'limits': ['No overall PASS or refusal acceptance is inferred from matching exits, missing observers or invalid query JSON.',
                       'JSON comparisons preserve exact JSON type/shape; they do not substitute for unrecorded Typst morphology or arithmetic semantics.',
                       'Strict diagnostics remain unwaived, including source spans/hints in retained raw stderr. Equal error exit alone is insufficient.',
                       'DOM parsing preserves tags/attributes/text/declaration without browser repair; it is not visual browser layout.',
                       'The input receipt proves its own frozen before/after run; this verifier does not rebuild a binary or claim later source state identity.']}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--receipt', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--selfchecks-only', action='store_true')
    args = parser.parse_args()
    assert not args.output.exists(), 'never overwrite prior verification evidence'
    if args.selfchecks_only:
        result = {'utc': datetime.now(timezone.utc).isoformat(), 'mode': 'Pure verifier selfchecks; no product runs', 'verifier_sha256': sha(__file__), 'selfchecks': selfchecks()}
    else:
        assert args.receipt is not None
        result = review(args.receipt.resolve())
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({'output': str(args.output), 'sha256': sha(args.output), 'cases': len(result.get('cases', [])), 'selfchecks': len(result['selfchecks'])}))


if __name__ == '__main__':
    main()
