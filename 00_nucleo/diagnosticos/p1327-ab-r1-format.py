"""Formatting-only successor from integrated test source; never reads product runtime."""
import datetime
import difflib
import hashlib
import importlib.util
import json
import pathlib
import re
import subprocess

root = pathlib.Path('/repos/Antigravity/typst-crystalline')
prefix = root / '00_nucleo/diagnosticos'
spec = importlib.util.spec_from_file_location('runner', prefix / 'p1327-ab-runner.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
source = (root / '01_core/src/compiler/eval/tests.rs').read_text()
argv = ['rustfmt', '--edition', '2021', '--emit', 'stdout']
formatted = subprocess.run(argv, input=source, capture_output=True, text=True, cwd=root, check=True).stdout
lines_before = source.splitlines(keepends=True)
lines_after = formatted.splitlines(keepends=True)
matcher = difflib.SequenceMatcher(a=lines_before, b=lines_after, autojunk=False)
replacements = []
for group in matcher.get_grouped_opcodes(4):
    a0, a1 = group[0][1], group[-1][2]
    b0, b1 = group[0][3], group[-1][4]
    old, new = ''.join(lines_before[a0:a1]), ''.join(lines_after[b0:b1])
    assert source.count(old) == 1
    replacements.append({'old': old, 'new': new})
replayed = source
for item in replacements:
    replayed = replayed.replace(item['old'], item['new'], 1)
assert replayed == formatted
legacy = json.loads((prefix / 'p1327-ab-legacy-replacements.json').read_text())
successors = []
for item in legacy['replacements']:
    name = re.search(r'fn (\w+)\(', item['old']).group(1)
    marker = '        #[test]\n        fn ' + name + '()'
    start = formatted.index(marker)
    end = formatted.index('        #[test]', start + len(marker))
    final = formatted[start:end]
    # rustfmt can add braces/commas around match arms, but no literal or expectation changes.
    before_literals = re.findall(r'"(?:\\.|[^"\\])*"', item['new'])
    after_literals = re.findall(r'"(?:\\.|[^"\\])*"', final)
    assert before_literals == after_literals, name
    before_ranges = re.findall(r'\b\d+\.\.\d+\b', item['new'])
    after_ranges = re.findall(r'\b\d+\.\.\d+\b', final)
    assert before_ranges == after_ranges, name
    successors.append({'function': name, 'observations': item['observations'],
                       'old': item['old'], 'new': final})
sha = lambda text: hashlib.sha256(text.encode()).hexdigest()
receipt = {
    'created_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'regime': 'A/B without technical isolation attestation',
    'pre_C': True,
    'cause': 'Rustfmt check of exactly integrated frozen legacy successors; no semantic failure',
    'command': argv,
    'input': '01_core/src/compiler/eval/tests.rs (integrated TEST-ONLY source)',
    'input_sha256': sha(source),
    'formatted_sha256': sha(formatted),
    'input_body_sha256': sha(source[source.index('use super::*;'):]),
    'formatted_body_sha256': sha(formatted[formatted.index('use super::*;'):]),
    'parent_frozen_replacements_sha256': runner.sha(prefix / 'p1327-ab-legacy-replacements.json'),
    'frozen_snippet_sha256': runner.sha(prefix / 'p1327-ab-tests.rs'),
    'semantic_argument': 'Only deterministic rustfmt output in full integrated test context. String literals and numeric ranges remain byte-identical and ordered in every legacy successor. Rustfmt may introduce match-arm braces or trailing commas. No expected value, assertion, helper or case is revised.',
    'replacements': replacements,
    'canonical_legacy_successors': successors,
    'full_diff': ''.join(difflib.unified_diff(lines_before, lines_after, fromfile='integrated-tests.rs', tofile='formatted-tests.rs')),
}
runner.save(prefix / 'p1327-ab-r1-format.json', receipt)
print(json.dumps({'delta_groups': len(replacements), 'receipt_sha256': runner.sha(prefix / 'p1327-ab-r1-format.json'), 'diff': receipt['full_diff']}, indent=2))
