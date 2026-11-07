#!/usr/bin/env python3
"""Install a joint-witness diagnostic in a separate RAM pipeline copy.

Usage: apply_overlay.py PIPELINE_RAM [ENGINE_RAM] [--mutation NAME]. No build,
lock reconciliation, configuration/flag changes or productive edits are done.
"""
import argparse
from pathlib import Path
import shutil
from _overlay import Edits, HERE, RAM, checked, prepared_workspace


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('pipeline')
    parser.add_argument('engine', nargs='?')
    parser.add_argument('--mutation', default='none', choices=(
        'none', 'ignore-ctime', 'omit-witness', 'omit-selfcheck', 'omit-g2', 'omit-g3', 'omit-policy',
    ))
    args = parser.parse_args()
    pipeline = prepared_workspace(args.pipeline)
    # A complete config is mandatory; its bytes/flags are never rewritten.
    config = checked(pipeline, '.cargo/config.toml')
    if not config.is_file():
        parser.error('Copy the complete received .cargo/config.toml before applying')
    engine_argument = Path(args.engine) if args.engine else RAM / 'engine'
    if engine_argument.absolute() != engine_argument.resolve():
        parser.error('Engine path may not use symlinks or lexical escapes')
    engine = engine_argument.resolve()
    if (not engine.is_relative_to(RAM.resolve()) or engine == RAM.resolve()
            or engine.is_relative_to(pipeline) or pipeline.is_relative_to(engine)
            or engine.relative_to(RAM.resolve()).parts[0].lower().startswith(('frozen', 'baseline'))
            or engine.relative_to(RAM.resolve()).parts[0] == 'R'):
        parser.error('Engine must be a separate non-reference RAM child')
    if args.mutation != 'none' and engine.exists():
        parser.error('A mutant must use a fresh private engine path')
    if not engine.exists():
        # No logs, builds, lockfiles, tests or compiled caches are synthesized.
        shutil.copytree(HERE, engine, ignore=shutil.ignore_patterns('target', '__pycache__', 'Cargo.lock'))
    engine = prepared_workspace(engine)
    checked(engine, 'Cargo.toml')
    checked(engine, 'vendor/fontdb/Cargo.toml')
    edits = Edits(pipeline)
    for relative in ('03_infra/src/fontdb/generation.rs', '03_infra/src/world/font_session.rs'):
        target = checked(pipeline, relative)
        edits.contents[target] = (HERE / 'overlays' / relative).read_text()
    manifest = checked(pipeline, '03_infra/Cargo.toml').read_text()
    if 'joint_font_witness' in manifest or 'joint_font_discovery' in manifest:
        parser.error('Pipeline already contains a joint discovery experiment')
    edits.replace('03_infra/Cargo.toml', '[dependencies]\n',
        '[dependencies]\njoint_font_witness = { path = "' + str(engine) + '" }\n')
    root_manifest = checked(pipeline, 'Cargo.toml')
    text = root_manifest.read_text()
    patch = 'fontdb = { path = "' + str(engine / 'vendor/fontdb') + '" }\n'
    if '[patch.crates-io]' in text:
        section = text.split('[patch.crates-io]', 1)[1].split('\n[', 1)[0]
        if 'fontdb' in section:
            parser.error('Existing fontdb patch requires review')
        text = text.replace('[patch.crates-io]', '[patch.crates-io]\n' + patch, 1)
    else:
        text += '\n[patch.crates-io]\n' + patch
    edits.contents[root_manifest] = text
    if args.mutation != 'none':
        flag = args.mutation.replace('-', '_').upper()
        engine_edits = Edits(engine)
        engine_edits.replace('src/lib.rs', f'const {flag}: bool = false;', f'const {flag}: bool = true;')
        engine_edits.publish()
    edits.publish()
    print(f'Applied LAB joint witness to {pipeline}; engine={engine}; mutation={args.mutation}')
    print('C reconciles Cargo.lock paths/new lab crate only; reject dependency upgrades.')
    print('TEKT_JOINT_MODE=0 original, 1 Legacy, 2 SingleRegular. CLI discovery remains original.')


if __name__ == '__main__':
    main()
