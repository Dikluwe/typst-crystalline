#!/usr/bin/env python3
"""Build the pinned upstream CLI and retain source/build/binary provenance."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tomllib

PIN = 'bd5ebec285505889e8ff1d4f68804ab147a699f3'
TREE = 'cac836c07e4dec2a89219370428292393a3216b5'


def verify_source(source: Path, pin: str = PIN, tree: str = TREE) -> None:
    def git(*args):
        return subprocess.check_output(['git', '-C', str(source), *args], text=True).strip()
    if git('rev-parse', '--show-toplevel') != str(source.resolve()):
        raise ValueError('oracle must have its own upstream checkout')
    if git('rev-parse', 'HEAD') != pin or git('rev-parse', 'HEAD^{tree}') != tree:
        raise ValueError('oracle commit/tree differs from the ratified pin')
    if git('status', '--porcelain', '--untracked-files=all'):
        raise ValueError('oracle source is dirty')


def build(source: Path, receipt: Path) -> None:
    source = source.resolve()
    receipt = receipt.resolve()
    # A failed attempt must never leave a prior success receipt in place.
    receipt.unlink(missing_ok=True)
    verify_source(source)
    env = os.environ.copy()
    env['TYPST_COMMIT_SHA'] = PIN
    env['CARGO_TARGET_DIR'] = str(source / 'target')
    env['CARGO_BUILD_JOBS'] = '1'
    toolchain_file = Path(__file__).resolve().parents[3] / 'rust-toolchain.toml'
    env['RUSTUP_TOOLCHAIN'] = tomllib.loads(toolchain_file.read_text())['toolchain']['channel']
    command = ['cargo', 'build', '--release', '--locked', '-p', 'typst-cli']
    rustc = subprocess.check_output(['rustc', '-Vv'], env=env, text=True).strip()
    subprocess.run(command, cwd=source, env=env, check=True)
    verify_source(source)
    binary = source / 'target/release/typst'
    sha256 = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    metadata = {
        'pin': PIN, 'upstream_tree': TREE,
        'completed_at_utc': datetime.now(timezone.utc).isoformat(),
        'recipe': command, 'rustc': rustc,
        'environment': {key: env[key] for key in ('TYPST_COMMIT_SHA', 'CARGO_TARGET_DIR', 'CARGO_BUILD_JOBS', 'RUSTUP_TOOLCHAIN')},
        'cargo_lock_sha256': sha256(source / 'Cargo.lock'),
        'binary_sha256': sha256(binary),
        'version_auxiliary': subprocess.check_output([str(binary), '--version'], text=True).strip(),
    }
    receipt.parent.mkdir(parents=True, exist_ok=True)
    receipt.write_text(json.dumps(metadata, indent=2) + '\n', encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', required=True, type=Path)
    parser.add_argument('--receipt', required=True, type=Path)
    args = parser.parse_args()
    build(args.source, args.receipt)
