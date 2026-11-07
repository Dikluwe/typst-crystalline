#!/usr/bin/env python3
"""Install the unchanged nominal stdlib candidate/neutral control in task RAM.

No tests, builds, Cargo, lock/dependency/config changes or productive edits.
The previous patcher is explicitly imported with a process-local RAM constant.
"""
import argparse
import importlib.util
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
PREVIOUS = HERE.parent / 'watch-startup-neutral'
RAM = Path('/dev/shm/tekt-stdlib-qualification-20261106')


def load(name, path):
    specification = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(specification)
    saved = sys.dont_write_bytecode
    sys.dont_write_bytecode = True
    try:
        specification.loader.exec_module(module)
    finally:
        sys.dont_write_bytecode = saved
    return module


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('workspace')
    parser.add_argument('--variant', required=True, choices=('bas', 'neu', 'laz'))
    parser.add_argument('--mutation', default='none', choices=(
        'none', 'share-identities', 'ignore-inputs', 'ignore-features',
    ))
    args = parser.parse_args()
    if args.mutation != 'none' and args.variant != 'laz':
        parser.error('Mutations require a fresh nominal laz workspace')
    guard = load('qualification_previous_guard', PREVIOUS / '_overlay.py')
    guard.RAM = RAM
    workspace = guard.prepared_workspace(args.workspace)
    first = workspace.relative_to(RAM.resolve()).parts[0].lower()
    if first.startswith('reference'):
        parser.error('Reference workspace is immutable')
    if not guard.checked(workspace, '.cargo/config.toml').is_file():
        parser.error('Copy the complete received .cargo/config.toml first')
    helper = guard.checked(workspace, '01_core/src/compiler/eval/lab_stdlib_snapshot.rs')
    if helper.exists():
        parser.error('Workspace already has a stdlib overlay')
    if (HERE / 'snapshot.rs').read_bytes() != (PREVIOUS / 'snapshot.rs').read_bytes():
        parser.error('Qualification helper must remain byte-identical to the previous candidate')
    saved_module = sys.modules.get('_overlay')
    sys.modules['_overlay'] = guard
    try:
        previous = load('qualification_previous_apply', PREVIOUS / 'apply.py')
    finally:
        if saved_module is None:
            del sys.modules['_overlay']
        else:
            sys.modules['_overlay'] = saved_module
    saved_arguments = sys.argv
    try:
        sys.argv = [str(PREVIOUS / 'apply.py'), str(workspace), '--variant', args.variant]
        previous.main()
    finally:
        sys.argv = saved_arguments
    if args.mutation != 'none':
        flag = args.mutation.replace('-', '_').upper()
        edits = guard.Edits(workspace)
        edits.replace('01_core/src/compiler/eval/lab_stdlib_snapshot.rs',
            f'const {flag}: bool = false;', f'const {flag}: bool = true;')
        edits.publish()
    print(f'Qualification LAB variant={args.variant} mutation={args.mutation}; no build executed')


if __name__ == '__main__':
    main()
