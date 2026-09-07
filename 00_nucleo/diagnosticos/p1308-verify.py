#!/usr/bin/env python3
"""P1308 independent read-only pre-candidate preparation; emits JSON to stdout.

No source or judged-artifact writes. Do not invoke prepare after candidate edits:
it deliberately rejects every productive delta except the exact lineage header.
No result here is a product certificate or an executed source-mutant score.
"""
import argparse
import datetime as dt
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = ROOT / '00_nucleo/diagnosticos'
BASELINE = HERE / 'p1308-baseline.json'
BASELINE_SHA = '62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394'
STEP = ROOT / '00_nucleo/materialization/typst-passo-1308.md'
OWNERS = [
    'compiler/eval.md', 'compiler/eval/operators/arithmetic.md',
    'compiler/eval/operators/join.md', 'compiler/eval/call_dispatch.md',
    'compiler/eval/math.md', 'compiler/eval/closures.md', 'compiler/eval/repr.md',
    'compiler/eval/tests.md', 'entities/func.md', 'compiler/stdlib/collections.md',
    'compiler/stdlib/panic.md', 'wiring/tests/p1293_contract.md',
]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def pin(path):
    p = Path(path).resolve()
    return {'path': str(p.relative_to(ROOT)), 'sha256': sha(p.read_bytes())}


def read(path):
    return json.loads(Path(path).read_text())


def apply_block(raw, block, path):
    """Apply one exact text diff in memory only, checking every old line."""
    lines = raw.decode().splitlines(keepends=True)
    result, cursor = [], 0
    hunks = re.split(r'(?m)(?=^@@ )', block)[1:]
    for hunk in hunks:
        header, *body = hunk.splitlines(keepends=True)
        m = re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@', header)
        assert m, header
        pos = int(m[1]) - 1
        old = [s[1:] for s in body if s.startswith((' ', '-'))]
        new = [s[1:] for s in body if s.startswith((' ', '+'))]
        if lines[pos:pos + len(old)] != old:
            matches = [i for i in range(len(lines) - len(old) + 1)
                       if lines[i:i + len(old)] == old]
            assert len(matches) == 1, (path, 'non-unique exact hunk', matches)
            pos = matches[0]
        assert pos >= cursor
        result.extend(lines[cursor:pos]); result.extend(new); cursor = pos + len(old)
    result.extend(lines[cursor:])
    data = ''.join(result).encode() if hunks else raw
    return data


def baseline_bytes(base, path):
    """Rebuild only the requested baseline file from pinned HEAD plus captured diff."""
    raw = subprocess.check_output(['git', 'show', base['state']['head'].strip() + ':' + path], cwd=ROOT)
    parts = re.split(r'(?m)(?=^diff --git )', base['state']['diff'])
    block = next((s for s in parts if s.startswith(f'diff --git a/{path} b/{path}\n')), '')
    data = apply_block(raw, block, path)
    assert sha(data) == base['inventory'][path], 'Cannot reconstruct frozen baseline: ' + path
    return data


def code_semantic(data):
    out, n = re.subn(rb'(?m)^//! @prompt-hash [^\n]*\n', b'', data)
    assert n == 1, 'Missing/ambiguous lineage header'
    return sha(out)


def L0_semantic(data):
    out, n = re.subn(rb'(?m)^Hash do C\xc3\xb3digo: [^\n]*$', b'Hash do Codigo: <metadata>', data)
    assert n <= 1
    return sha(out)


def formatted_semantic(data):
    p = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'],
                       input=data, capture_output=True, check=True)
    return code_semantic(p.stdout)


def prepare(extra_patches=()):
    assert sha(BASELINE.read_bytes()) == BASELINE_SHA
    base = read(BASELINE)
    expected_owners = {'00_nucleo/prompts/' + p for p in OWNERS}
    predecessor_failures = [n for n, digest in base['predecessors'].items()
                            if sha((HERE / n).read_bytes()) != digest]
    assert not predecessor_failures, predecessor_failures
    test_patches = [HERE / 'p1308-tests.patch', *map(Path, extra_patches)]
    test_blocks = {}
    for patch in test_patches:
        for block in re.split(r'(?m)(?=^--- a/)', patch.read_text()):
            match = re.match(r'--- a/(.+)\n\+\+\+ b/(.+)\n', block)
            if match:
                assert match[1] == match[2]
                assert match[1] in ['01_core/src/compiler/eval/tests.rs',
                                    '01_core/src/compiler/eval/repr.rs',
                                    '04_wiring/tests/p1293_contract.rs']
                test_blocks.setdefault(match[1], []).append(block)
    metadata_sources, test_sources, changed_owners, failures = [], [], [], []
    for name, digest in base['inventory'].items():
        p = ROOT / name
        if not p.is_file():
            failures.append({'path': name, 'reason': 'missing'}); continue
        current = p.read_bytes()
        if sha(current) == digest:
            continue
        if name in expected_owners:
            changed_owners.append(name)
        elif name.endswith('.rs'):
            old = baseline_bytes(base, name)
            if name in test_blocks:
                for block in test_blocks[name]:
                    old = apply_block(old, block, name)
                if formatted_semantic(old) == formatted_semantic(current):
                    test_sources.append(name)
                else:
                    failures.append({'path': name, 'reason': 'independent test integration differs'})
            elif code_semantic(old) == code_semantic(current):
                metadata_sources.append(name)
            else:
                failures.append({'path': name, 'reason': 'productive source changed before freeze'})
        else:
            failures.append({'path': name, 'reason': 'outside authorized twelve owners'})
    assert not failures, failures
    contracts = []
    for name in sorted(expected_owners):
        p = ROOT / name
        contracts.append({**pin(p), 'semantic_sha256': L0_semantic(p.read_bytes())})
    candidate = subprocess.check_output(['git', 'diff', '--cached', '--binary'], cwd=ROOT).decode()
    assert candidate == base['state']['staged'], 'Index changed'
    assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT).decode().strip() == base['state']['head'].strip()
    return {
        'schema': 'p1308-independent-preparation-v1',
        'at': dt.datetime.now(dt.timezone.utc).isoformat(),
        'executor': '/root/p1306_oracle',
        'regime': 'executado sem atestacao de isolamento tecnico',
        'status': 'PRE_CANDIDATE_INTEGRITY_ONLY_AWAITING_SUITE',
        'baseline': pin(BASELINE), 'step': pin(STEP), 'verifier': pin(__file__),
        'contracts': contracts, 'changed_authorized_L0': sorted(changed_owners),
        'independent_test_patches': [pin(p) for p in test_patches],
        'canonical_independent_test_integrations': sorted(test_sources),
        'source_changes_only_exact_prompt_hash_header': sorted(metadata_sources),
        'protected_predecessor_failures': predecessor_failures,
        'source_integrity_failures': failures,
        'current_HEAD': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT).decode().strip(),
        'index_unchanged': True, 'candidate_source_exists': False,
        'actual_Rust_mutants': 0, 'actual_Rust_mutation_score': None,
        'full_discriminatory_seal': False,
        'limitations': ['No candidate or source mutants have been tested.',
                        'Suite/tests pins and focal baseline discrimination must be added before a preimplementation receipt.',
                        'The future six real P1308 mutants do not discharge the37 pending P1307 families.'],
    }


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=['prepare'])
    parser.add_argument('--extra-test-patch', action='append', default=[])
    args = parser.parse_args()
    print(json.dumps(prepare(args.extra_test_patch), ensure_ascii=False, indent=2))
