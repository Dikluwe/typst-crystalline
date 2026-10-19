"""Read archived mutation/compiler evidence without running the compiler."""
from pathlib import Path
import collections, datetime, gzip, hashlib, json

root = Path.cwd()
base = root / 'lab/results/v16-corrections-2026-10-19/batch-e'
out = base / 'agents/reflection'
worker = base.parent / 'workers/batch_e_structural.py'
# The workers directory is a sibling of batch-e.
worker = base.parent / 'workers/batch_e_structural.py'
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
specs = json.loads((base / 'structural-targets.json').read_text())
result = {'review_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'review_method': 'Independently reconstruct mutations and target spans from archived source; parse raw compiler JSON; do not use selected_e0004 as proof.',
          'worker': {'path': str(worker.relative_to(root)), 'sha256': sha(worker)},
          'targets_sha256': sha(base / 'structural-targets.json'),
          'phases': {},
          'limits': ['Compiler witness proves enum-domain review coverage, not language correctness.',
                     'Shell witnesses remove positive arms in actual shell matches; they are not future-variant cross-crate compilation witnesses.',
                     'Core baseline is an intentionally failing extension build with unrelated E0004s; it is not a successful unmutated build.']}
for phase in ('baseline', 'green'):
    archive = base / ('structural-' + phase)
    if not (archive / 'results.json').exists():
        result['phases'][phase] = {'status': 'pending archived results'}
        continue
    records = json.loads((archive / 'results.json').read_text())
    original = json.load(gzip.open(archive / 'original-sources.json.gz', 'rt'))
    inputs = json.load(gzip.open(archive / 'inputs.json.gz', 'rt'))
    mismatches = [path for path, text in original.items()
                  if hashlib.sha256(text.encode()).hexdigest() != inputs['inputs'][path]]
    phase_result = {'status': 'complete' if len(records) == 2 else 'partial',
                    'source_archive_hash_mismatches': mismatches, 'groups': []}
    assert not mismatches
    for recorded in records:
        group = recorded['name']
        selected = [s for s in specs if s['group'] == group and phase in s]
        mutations = json.load(gzip.open(archive / (group + '-mutations.json.gz'), 'rt'))
        expected_mutations = {}
        if group == 'core-extension':
            path = '01_core/src/entities/content.rs'
            text = original[path]
            pos = text.index('\n}', text.index('pub enum Content {'))
            expected_mutations[path] = text[:pos] + '\n    TektV16EvolutionProbe,' + text[pos:]
        else:
            for target in selected:
                path = target['file']
                text = expected_mutations.get(path, original[path])
                assert text.count(target['remove_exact']) == 1, target['id']
                expected_mutations[path] = text.replace(target['remove_exact'], '')
        assert expected_mutations == mutations, (phase, group, 'mutation mismatch')
        errors, finished = [], []
        for line in (archive / (group + '.log')).read_text().splitlines():
            try:
                item = json.loads(line)
            except ValueError:
                continue
            if item.get('reason') == 'compiler-message' and item['message']['level'] == 'error':
                errors.append(item['message'])
            if item.get('reason') == 'build-finished':
                finished.append(item['success'])
        witnesses = []
        for target in selected:
            text = mutations.get(target['file'], original[target['file']])
            start = text.index(target[phase]['after'])
            pos = text.index(target[phase]['anchor'], start)
            line = text[:pos].count('\n') + 1
            byte = len(text[:pos].encode())
            end = byte + len(target[phase]['anchor'].encode())
            hits = []
            for error in errors:
                if (error.get('code') or {}).get('code') != 'E0004':
                    continue
                for span in error['spans']:
                    filename = span['file_name'].removeprefix(recorded['cwd'] + '/')
                    if span['is_primary'] and filename == target['file'] and span['line_start'] == line and byte <= span['byte_start'] < end:
                        hits.append({'message': error['message'], 'primary_span': span,
                                     'primary_bytes_text': text.encode()[span['byte_start']:span['byte_end']].decode(),
                                     'code': 'E0004'})
            witnesses.append({'id': target['id'], 'file': target['file'],
                              'match_anchor': {'line': line, 'byte_start': byte, 'byte_end': end},
                              'e0004_primary_diagnostics': hits,
                              'expected': 'no selected E0004' if phase == 'baseline' else 'one local E0004'})
            assert len(hits) == (0 if phase == 'baseline' else 1), (phase, target['id'], hits)
        codes = dict(collections.Counter((error.get('code') or {}).get('code') for error in errors))
        assert all(code == 'E0004' for code in codes)
        expected_success = phase == 'baseline' and group == 'shell-omissions'
        assert finished == [expected_success], (phase, group, finished)
        phase_result['groups'].append({'name': group, 'mutation_verified_exact': True,
                                      'mutated_paths': sorted(mutations), 'compiler_error_codes': codes,
                                      'compiler_build_finished': finished, 'recorded_exit': recorded['exit'],
                                      'log_sha256': sha(archive / (group + '.log')),
                                      'mutation_archive_sha256': sha(archive / (group + '-mutations.json.gz')),
                                      'witnesses': witnesses})
    if (archive / 'freeze.json').exists():
        phase_result['source_freeze'] = json.loads((archive / 'freeze.json').read_text())
        assert phase_result['source_freeze']['changed'] == []
    result['phases'][phase] = phase_result
(out / 'structural-independent-review.json').write_text(json.dumps(result, indent=2) + '\n')
lines = ['# Independent structural evidence review', '',
         'The review reconstructed every mutation from archived original sources and derived target locations independently. Raw compiler JSON, including primary spans, was inspected instead of trusting the worker\'s selected-diagnostic counter. No compiler was run during this review.', '']
for phase, data in result['phases'].items():
    lines += ['## ' + phase.title(), '', 'Status: ' + data['status'] + '.', '']
    for group in data.get('groups', []):
        lines.append(f"{group['name']}: exact mutation verified; raw compiler error codes {group['compiler_error_codes']}; build-finished {group['compiler_build_finished']}.")
        for witness in group['witnesses']:
            hits = witness['e0004_primary_diagnostics']
            span = hits[0]['primary_span'] if hits else None
            detail = ('E0004 primary ' + str(span['line_start']) + ':' + str(span['column_start']) + ' bytes ' + str(span['byte_start']) + '..' + str(span['byte_end']) + ' (`' + hits[0]['primary_bytes_text'] + '`)') if span else 'no primary E0004 at the match'
            lines.append(f"- {witness['id']}: {witness['file']}:{witness['match_anchor']['line']} — {detail}.")
        lines.append('')
lines += ['## Scope', '', *result['limits'], '', 'The worker restores every Rust source before each mutation group. The core group adds only the sentinel Content variant. The shell group removes only the selected positive arms, compiling against unchanged Content. Input hashes, exact mutated bytes and unchanged final freeze are verified in the JSON report.']
(out / 'structural-independent-review.md').write_text('\n'.join(lines) + '\n')
print(json.dumps({p: d['status'] for p, d in result['phases'].items()}))
