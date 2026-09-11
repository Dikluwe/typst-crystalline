"""Static full-byte check of the independent R5 oracle; no product execution."""
import argparse
import hashlib
import json
from pathlib import Path


def digest(data):
    return hashlib.sha256(data).hexdigest()


def check(oracle_path):
    directory = Path(__file__).resolve().parent
    contract_path = directory / 'p1340-terminal-contract-r5.json'
    contract = json.loads(contract_path.read_text())
    original_path = directory / 'p1340-contract-terminal-harness-r2.rs'
    original = original_path.read_bytes()
    assert digest(original) == contract['byte_preservation']['R2_harness_sha256'], 'R2 changed'
    expected = original
    for row in contract['sources']:
        assert digest(row['original_source'].encode()) == row['original_sha256']
        assert digest(row['successor_source'].encode()) == row['successor_sha256']
        old = json.dumps(row['original_source'], ensure_ascii=False).encode()
        new = json.dumps(row['successor_source'], ensure_ascii=False).encode()
        assert expected.count(old) == 1, (row['case'], 'unique original literal')
        expected = expected.replace(old, new, 1)
    actual = oracle_path.read_bytes()
    assert actual == expected, 'Oracle differs outside exact three authorized source replacements'
    restored = actual
    for row in reversed(contract['sources']):
        old = json.dumps(row['original_source'], ensure_ascii=False).encode()
        new = json.dumps(row['successor_source'], ensure_ascii=False).encode()
        assert restored.count(new) == 1, (row['case'], 'unique successor literal')
        restored = restored.replace(new, old, 1)
    assert restored == original, 'Inverse transformation does not restore R2 bytes'
    return {'check': 'static-source-transformation-only', 'oracle': str(oracle_path),
            'oracle_sha256': digest(actual), 'contract_sha256': digest(contract_path.read_bytes()),
            'R2_restored_sha256': digest(restored), 'product_runs': 0,
            'seal': None, 'terminal_verdict': None}


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('oracle', type=Path)
    args = parser.parse_args()
    print(json.dumps(check(args.oracle), indent=2))
