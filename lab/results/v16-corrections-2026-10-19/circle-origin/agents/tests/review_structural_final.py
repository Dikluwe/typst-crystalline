#!/usr/bin/env python3
"""Review actual Rust compiler witnesses independently of the worker's verdict."""
import datetime
import gzip
import hashlib
import json
import pathlib
import re

ROOT = pathlib.Path('/repos/Antigravity/typst-crystalline')
HERE = pathlib.Path(__file__).resolve().parent
EVIDENCE = HERE.parent.parent

def digest(data):
    return hashlib.sha256(data).hexdigest()

def read(path):
    raw = path.read_bytes()
    return json.loads(gzip.decompress(raw) if path.suffix == '.gz' else raw)

def code_tokens(source):
    # Keep string contents exact; rustfmt only changes whitespace/trailing call commas.
    tokens = re.findall(r'//[^\n]*|"(?:\\.|[^"\\])*"|[^\s]', source)
    tokens = [token for token in tokens if not token.startswith('//')]
    return [token for index, token in enumerate(tokens)
            if not (token == ',' and index + 1 < len(tokens) and tokens[index + 1] == ')')]

def inspect(folder_name, stage, expected_count):
    folder = EVIDENCE / folder_name
    frozen = read(folder / 'freeze.json')
    assert frozen['changed'] == []
    originals = read(folder / 'original-sources.json.gz')
    modified = read(folder / 'all-domains-mutations.json.gz')
    state = read(folder / 'inputs.json.gz')
    records = read(folder / 'results.json')
    assert len(records) == 1
    record = records[0]
    assert record['exit'] == 101
    assert record['command'] == ['cargo', 'test', '--offline', '--locked', '-p', 'typst-core', '--lib', '--no-run', '--message-format=json']
    assert len(originals) > 900
    declarations = {'01_core/src/entities/value.rs': 'pub enum Value {'}
    if stage == 'green':
        declarations.update({'01_core/src/entities/elements/shape.rs': 'pub enum ShapeOrigin {',
                             '01_core/src/entities/layout_types.rs': 'pub enum FrameItem {'})
    assert set(modified) == set(declarations)
    mutations = []
    for path, declaration in declarations.items():
        original = originals[path]
        declaration_start = original.index(declaration)
        closing = original.index('\n}', declaration_start)
        reconstructed = original[:closing] + '\n    TektV16EvolutionProbe,' + original[closing:]
        assert reconstructed == modified[path], path
        assert digest(original.encode()) == state['inputs'][path]
        mutations.append({'source': path, 'declaration': declaration, 'before_sha256': digest(original.encode()),
                          'mutated_sha256': digest(reconstructed.encode()), 'insert_byte': len(original[:closing].encode())})
    errors = []
    for line in (folder / 'all-domains.log').read_text().splitlines():
        try:
            entry = json.loads(line)
        except ValueError:
            continue
        if entry.get('reason') == 'compiler-message' and entry['message'].get('level') == 'error':
            errors.append(entry['message'])
    assert errors
    assert all((error.get('code') or {}).get('code') == 'E0004' for error in errors)
    witnesses = []
    for target in record['targets']:
        source = modified.get(target['file'], originals[target['file']])
        selector = target[stage]
        begin = source.index(selector['after']) if selector.get('after') else 0
        start = source.index(selector['anchor'], begin)
        line_number = source[:start].count('\n') + 1
        byte_start = len(source[:start].encode())
        byte_end = len(source[:start + len(selector['anchor'])].encode())
        assert (line_number, byte_start, byte_end) == (target['line'], target['byte_start'], target['byte_end'])
        primary = []
        for error in errors:
            for span in error['spans']:
                path = span['file_name'].removeprefix(record['cwd'] + '/')
                if span['is_primary'] and path == target['file'] and span['line_start'] == line_number and byte_start <= span['byte_start'] < byte_end:
                    assert 'TektV16EvolutionProbe' in error['message']
                    primary.append({'message': error['message'], 'span': span})
        assert bool(primary) == (stage == 'green'), target['id']
        witnesses.append({'id': target['id'], 'source': target['file'], 'line': line_number,
                          'byte_start': byte_start, 'byte_end': byte_end, 'primary_e0004': primary})
    assert sum(bool(item['primary_e0004']) for item in witnesses) == expected_count
    if stage == 'green':
        assert {item['id'] for item in witnesses} == {'V16-065', 'ShapeOrigin-self', 'ShapeOrigin-geometry-other', 'ShapeOrigin-circle-other', 'Circle-test-observer-FrameItem'}
        mismatches = [path for path, text in originals.items() if (ROOT / path).read_bytes() != text.encode()]
        assert mismatches == [], mismatches
        input_mismatches = [path for path, expected in state['inputs'].items() if digest((ROOT / path).read_bytes()) != expected]
        assert input_mismatches == [], input_mismatches
        current = (ROOT / '01_core/src/compiler/introspect/queries.rs').read_text()
        label_module = current[current.index('#[cfg(test)]\nmod circle_origin_label_tests'):]
        staged = (HERE / 'label-query-tests.rs').read_text()
        assert code_tokens(label_module) == code_tokens(staged), 'label acceptance changed beyond formatting'
        label_control = {'current_module_sha256': digest(label_module.encode()), 'original_staged_sha256': digest(staged.encode()),
                         'original_staged_path': str(HERE / 'label-query-tests.rs'),
                         'token_sha256': digest(json.dumps(code_tokens(staged)).encode()),
                         'method': 'exact Rust tokens and quoted string contents, ignoring comments/whitespace and rustfmt trailing call commas'}
    else:
        label_control = None
    assert state['memory']['memory.max'] == str(12 * 1024 ** 3)
    assert state['memory']['memory.swap.max'] == '0'
    files = ['freeze.json', 'original-sources.json.gz', 'all-domains-mutations.json.gz', 'inputs.json.gz', 'results.json', 'all-domains.log']
    return {'folder': folder_name, 'stage': stage, 'source_count': len(originals), 'input_count': len(state['inputs']),
            'selected_witness_count': expected_count, 'raw_error_count': len(errors), 'all_raw_errors_are_e0004': True,
            'mutations': mutations, 'witnesses': witnesses, 'final_sources_and_inputs_equal_current': stage == 'green',
            'label_tests_unchanged': label_control, 'sha256': {name: digest((folder / name).read_bytes()) for name in files}}

out = HERE / 'structural-review.json'
assert not out.exists(), 'append-only review'
phases = [inspect('structural-baseline', 'baseline', 0), inspect('structural-green-final', 'green', 5)]
report = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'passed': True,
          'method': 'Reconstructed exact variant mutations; reparsed raw compiler JSON; matched target primary error byte intervals. Nonselected E0004 are retained but not credited.',
          'runner_sha256': digest(pathlib.Path(__file__).read_bytes()), 'phases': phases}
out.write_text(json.dumps(report, indent=2) + '\n')
print('PASS: baseline0 / final5; exact source/current input bindings; original label acceptance unchanged')
