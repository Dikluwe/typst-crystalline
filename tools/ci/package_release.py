#!/usr/bin/env python3
"""Package a previously built Tekt CLI from a clean checkout; never publish."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
import zipfile


def package(root: Path, target: str, builder: str, *, rustc: str | None = None) -> Path:
    if not re.fullmatch(r'[a-zA-Z0-9_-]+', target) or builder not in ('cargo', 'cross'):
        raise ValueError('invalid target or builder')
    root = root.resolve()
    def git(*args):
        return subprocess.check_output(['git', '-C', str(root), *args], text=True).strip()
    if git('status', '--porcelain', '--untracked-files=all'):
        raise ValueError('refusing release packaging from a dirty source tree, including untracked files')
    commit = git('rev-parse', 'HEAD')
    if os.environ.get('TYPST_COMMIT_SHA', commit) != commit:
        raise ValueError('TYPST_COMMIT_SHA does not match the source commit')
    binary = root / 'target' / target / 'release' / ('typst.exe' if 'windows' in target else 'typst')
    digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
    metadata = {
        'package': 'typst-wiring', 'binary': binary.name, 'target': target,
        'commit': commit, 'tree': git('rev-parse', 'HEAD^{tree}'),
        'source_date_epoch': int(git('show', '-s', '--format=%ct', 'HEAD')),
        'binary_sha256': digest(binary), 'cargo_lock_sha256': digest(root / 'Cargo.lock'),
        'toolchain_sha256': digest(root / 'rust-toolchain.toml'),
        'rustc': rustc if rustc is not None else subprocess.check_output(['rustc', '-Vv'], cwd=root, text=True).strip(),
        'build_command': [builder, 'build', '--locked', '-p', 'typst-wiring', '--bin', 'typst', '--release', '--target', target],
    }
    dist = root / 'dist'
    dist.mkdir(exist_ok=True)
    name = f'typst-{target}'
    extension = '.zip' if 'windows' in target else '.tar.xz'
    archive = dist / (name + extension)
    with tempfile.TemporaryDirectory() as temporary:
        bundle = Path(temporary) / name
        bundle.mkdir()
        for path in [binary, *(root / x for x in ('README.md', 'LICENSE', 'NOTICE'))]:
            shutil.copy2(path, bundle / path.name)
        (bundle / 'BUILD.json').write_text(json.dumps(metadata, indent=2) + '\n', encoding='utf-8')
        if extension == '.zip':
            with zipfile.ZipFile(archive, 'w', compression=zipfile.ZIP_DEFLATED) as output:
                for path in sorted(bundle.iterdir()):
                    output.write(path, f'{name}/{path.name}')
        else:
            with tarfile.open(archive, 'w:xz') as output:
                output.add(bundle, arcname=name)
    archive.with_suffix(archive.suffix + '.sha256').write_text(f'{digest(archive)}  {archive.name}\n', encoding='ascii')
    return archive


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--target', required=True)
    parser.add_argument('--builder', choices=['cargo', 'cross'], required=True)
    args = parser.parse_args()
    print(package(Path(__file__).resolve().parents[2], args.target, args.builder))
