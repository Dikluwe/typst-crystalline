#!/usr/bin/env python3
"""Prepare laboratory probes in a pre-existing, separate RAM pipeline copy.

C alone runs this script. Native productive files and received experiments are
never modified. See --help. No compilation or test execution occurs here.
"""
import argparse
from pathlib import Path
import re
import shutil

RAM = Path('/dev/shm/tekt-catalog-tests-20261105')
HERE = Path(__file__).resolve().parent


def ram_child(path):
    p = Path(path).resolve()
    base = RAM.resolve()
    if not p.is_relative_to(base) or p in (base, base / 'workspace'):
        raise ValueError('Destination must be a separate child of the task RAM directory')
    return p


def dependency(manifest, name, target):
    text = manifest.read_text()
    if re.search(r'^' + re.escape(name) + r'\s*=', text, re.M):
        raise ValueError(f'{name} already linked in {manifest}')
    line = f'{name} = {{ path = "{target}" }}\n'
    if '[dependencies]\n' not in text:
        text += '\n[dependencies]\n'
    manifest.write_text(text.replace('[dependencies]\n', '[dependencies]\n' + line, 1))


def scopes(text, mapping):
    # Every selected signature below has a plain function body (no const braces
    # in generic arguments). Insert from the end to preserve source offsets.
    edits = []
    for match in re.finditer(r'\bfn\s+(\w+)\s*[<(]', text):
        name = match.group(1)
        if name not in mapping:
            continue
        start = text.find('{', match.end())
        semi = text.find(';', match.end())
        if start < 0 or (semi >= 0 and semi < start):
            continue
        edits.append((start + 1, '\n        let _catalog_scope = catalog_probe::scope("' + mapping[name] + '");'))
    for offset, edit in reversed(edits):
        text = text[:offset] + edit + text[offset:]
    return text


def fs_wrappers(text):
    for name in ('metadata', 'symlink_metadata', 'canonicalize', 'read', 'read_dir'):
        text = text.replace('std::fs::' + name + '(', 'catalog_probe::' + name + '(')
    return text.replace('std::fs::File::open(', 'catalog_probe::open(')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('destination', help='Existing pristine RAM pipeline copy, distinct from workspace')
    parser.add_argument('--source', default=str(RAM / 'workspace'), help='Frozen initial workspace')
    parser.add_argument('--candidate', choices=('original', 'joint'), required=True)
    parser.add_argument('--probe', choices=('none', 'timing', 'detail'), required=True)
    args = parser.parse_args()
    source = Path(args.source).resolve()
    dest = ram_child(args.destination)
    if source == dest:
        raise ValueError('Source and destination must differ')
    files = ('03_infra/src/world/font_session.rs', '03_infra/src/fontdb/generation.rs',
             '03_infra/Cargo.toml', '03_infra/src/world/fonts.rs', 'Cargo.toml')
    for name in files:
        if (dest / name).read_bytes() != (source / name).read_bytes():
            raise ValueError(f'Destination is not pristine relative to frozen source: {name}')
    experiment = None
    if args.candidate == 'joint':
        experiment = ram_child(RAM / ('joint-' + dest.name))
        if experiment.exists():
            raise ValueError(f'Refusing to overwrite {experiment}')
        shutil.copytree(source / 'lab/experiments/joint-font-discovery', experiment,
                        ignore=shutil.ignore_patterns('target', '__pycache__'))
        for src in (experiment / 'overlays').rglob('*.rs'):
            shutil.copyfile(src, dest / src.relative_to(experiment / 'overlays'))
        dependency(dest / '03_infra/Cargo.toml', 'joint_font_discovery', experiment)
        manifest = dest / 'Cargo.toml'
        text = manifest.read_text()
        patch = f'fontdb = {{ path = "{experiment / "vendor/fontdb"}" }}\n'
        if '[patch.crates-io]' in text:
            section = text.split('[patch.crates-io]', 1)[1].split('\n[', 1)[0]
            if re.search(r'^fontdb\s*=', section, re.M):
                raise ValueError('Existing fontdb patch needs explicit integration')
            text = text.replace('[patch.crates-io]', '[patch.crates-io]\n' + patch, 1)
        else:
            text += '\n[patch.crates-io]\n' + patch
        manifest.write_text(text)
    if args.probe == 'none':
        print('Prepared native sources without probes:', dest)
        return
    probe = ram_child(RAM / ('probe-' + dest.name))
    if probe.exists():
        raise ValueError(f'Refusing to overwrite {probe}')
    shutil.copytree(HERE / 'probe', probe)
    dependency(dest / '03_infra/Cargo.toml', 'catalog_probe', probe)
    generation = dest / '03_infra/src/fontdb/generation.rs'
    text = generation.read_text()
    text = scopes(text, {
        'config_key': 'configuration.reparse', 'fast_key': 'native.fast_key',
        'fast_key_unchanged': 'native.final_policy_proof',
        'probe_roots': 'native.lexical_traversal', 'prepare': 'native.prepare',
        'prepare_from': 'generation.prepare_from', 'prepare_joint': 'joint.prepare',
        'unchanged': 'generation.identity_recheck',
    })
    # G1 is the prepared-key check inside prepare_from; G2/G3 stay in session.
    text = text.replace('if !key.unchanged() {',
        'if !catalog_probe::measured("G1", || key.unchanged()) {')
    if args.probe == 'detail':
        text = fs_wrappers(text)
    generation.write_text(text)
    session = dest / '03_infra/src/world/font_session.rs'
    text = session.read_text()
    # Outer cycle outlives every timing guard; flush is outside scope clocks.
    text = text.replace('pub(super) fn catalog(&self, paths: &[PathBuf]) -> Arc<Catalog> {',
        'pub(super) fn catalog(&self, paths: &[PathBuf]) -> Arc<Catalog> {\n'
        '        let _catalog_cycle = catalog_probe::cycle();')
    text = scopes(text, {'catalog_joint': 'joint.session',
        'catalog_standard': 'native.session', 'catalog_with_fast': 'native.session_fast'})
    text = text.replace('if prepared.unchanged() {',
        'if catalog_probe::measured("G2", || prepared.unchanged()) {')
    text = text.replace('!prepared.unchanged()',
        '!catalog_probe::measured("G2", || prepared.unchanged())')
    text = text.replace('!key.unchanged()',
        '!catalog_probe::measured("G3", || key.unchanged())')
    text = text.replace('let _catalog_cycle = catalog_probe::cycle();',
        'let _catalog_cycle = catalog_probe::cycle();\n'
        '        let _catalog_scope = catalog_probe::scope("catalog.total");')
    if '_catalog_cycle' not in text:
        raise ValueError('Catalog signature changed; review instrumentation')
    session.write_text(text)
    fonts = dest / '03_infra/src/world/fonts.rs'
    text = fonts.read_text()
    signature = 'pub(super) fn fresh_catalog(paths: &[PathBuf]) -> (Vec<FontSlot>, FontBook) {'
    if signature not in text:
        raise ValueError('Fresh catalog signature changed')
    text = text.replace(signature, signature + '\n    let _catalog_cycle = catalog_probe::cycle();\n'
        '    let _catalog_scope = catalog_probe::scope("catalog.fresh");')
    text = scopes(text, {'prepared_catalog': 'catalog.materialize', 'combine_catalog': 'catalog.compose'})
    text = text.replace('let sets = crate::embedded_fonts::load_embedded_fonts();',
        'let sets = catalog_probe::measured("acquire.embedded", || crate::embedded_fonts::load_embedded_fonts());')
    text = text.replace('let system = crate::fontdb::load_system_fonts();',
        'let system = catalog_probe::measured("acquire.system", || crate::fontdb::load_system_fonts());')
    text = text.replace('let project = crate::fonts::discover_fonts(paths);',
        'let project = catalog_probe::measured("acquire.project", || crate::fonts::discover_fonts(paths));')
    text = text.replace('let system = crate::fontdb::catalog_from_database(&prepared.db);',
        'let system = catalog_probe::measured("acquire.prepared_system", || crate::fontdb::catalog_from_database(&prepared.db));')
    fonts.write_text(text)
    if experiment:
        for manifest in (experiment / 'Cargo.toml', experiment / 'vendor/fontdb/Cargo.toml'):
            dependency(manifest, 'catalog_probe', probe)
        lib = experiment / 'src/lib.rs'
        text = scopes(lib.read_text(), {
            'capture_system': 'joint.discovery', 'validate': 'joint.final_evidence',
            'entry': 'joint.observer.lexical', 'root': 'joint.observer.root',
            'directory': 'joint.observer.directory', 'resolved': 'joint.observer.canonical',
            'file_before': 'joint.observer.file_before', 'file_after': 'joint.observer.file_after',
            'config': 'joint.observer.configuration', 'finish': 'joint.finish',
        }) if args.probe == 'detail' else scopes(lib.read_text(), {
            'capture_system': 'joint.discovery', 'validate': 'joint.final_evidence',
            'finish': 'joint.finish',
        })
        lib.write_text(fs_wrappers(text) if args.probe == 'detail' else text)
        lib = experiment / 'vendor/fontdb/src/lib.rs'
        text = scopes(lib.read_text(), {
            'load_system_fonts': 'fontdb.native.system',
            'load_system_fonts_observed': 'fontdb.observed.system',
            'load_config_file_observed': 'fontdb.observed.configuration',
        })
        if args.probe == 'detail':
            text = scopes(text, {'canonicalize': 'fontdb.native.canonical',
                'canonicalize_observed': 'fontdb.observed.canonical',
                'load_fonts_dir_impl': 'fontdb.native.directory',
                'load_fonts_dir_observed': 'fontdb.observed.directory'})
            text = fs_wrappers(text)
        lib.write_text(text)
    print('Prepared', args.candidate, args.probe, 'only in', dest)
    print('CAT_PROBE=on CAT_PROBE_OUT=/dev/shm/tekt-catalog-tests-20261105/NAME.tsv')
    print('CAT_PROBE=off is same-binary calibration; E_JOINT_DISCOVERY=1 selects joint.')
    print('All phase clocks inclusive; detail build is counts evidence only.')


if __name__ == '__main__':
    main()
