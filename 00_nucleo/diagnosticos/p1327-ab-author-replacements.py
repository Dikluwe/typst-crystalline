"""Build exact legacy replacements from allowed test-only preimage, before C."""
import hashlib
import importlib.util
import pathlib

root = pathlib.Path('/repos/Antigravity/typst-crystalline')
runner_path = root / '00_nucleo/diagnosticos/p1327-ab-runner.py'
spec = importlib.util.spec_from_file_location('runner', runner_path)
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
source = (root / '01_core/src/compiler/eval/tests.rs').read_text()
replacements = []
for name, ranges in [('p1305_global_bare', '9..12'), ('p1305_global_alias_bare', '28..35')]:
    start = source.index('        #[test]\n        fn ' + name + '()')
    end = source.index('        #[test]', start + 20)
    old = source[start:end]
    new = old.replace('observe(', 'p1327_oracles::observe(', 1)
    new = new.replace('                        features\n', '                        features,\n                        &[' + ranges + ']\n', 1)
    replacements.append({'observations': [name], 'old': old, 'new': new})

name = 'p1305_import_binding_negatives_and_dynamic_spans'
start = source.index('        #[test]\n        fn ' + name + '()')
end = source.index('        #[test]', start + 20)
old = source[start:end]
needle = '                    assert!(side.is_empty(), "{profile}/{expression}: {side:?}");'
replacement = '''                    match *expression {
                        "{ import std; global }" =>
                            p1327_oracles::assert_warnings(expression, &side, &[9..12]),
                        "{ let renamed = std; import renamed; global }" =>
                            p1327_oracles::assert_warnings(expression, &side, &[28..35]),
                        _ => assert!(side.is_empty(), "{profile}/{expression}: {side:?}"),
                    }'''
assert old.count(needle) == 1
replacements.append({'observations': ['negative-global-bare', 'negative-global-alias-bare'],
                     'old': old, 'new': old.replace(needle, replacement)})

name = 'p1306_existing_lookup_and_repr_all_profiles'
start = source.index('        #[test]\n        fn ' + name + '()')
end = source.index('        #[test]', start + 20)
old = source[start:end]
needle = '                        assert!(side.is_empty(), "{profile}/{id}: {side:?}");'
replacement = '''                        match *id {
                            "global-bare-positive" =>
                                p1327_oracles::assert_warnings(expression, &side, &[9..12]),
                            "global-bare-alias-positive" =>
                                p1327_oracles::assert_warnings(expression, &side, &[26..31]),
                            _ => assert!(side.is_empty(), "{profile}/{id}: {side:?}"),
                        }'''
assert old.count(needle) == 1
replacements.append({'observations': ['global-bare-positive', 'global-bare-alias-positive'],
                     'old': old, 'new': old.replace(needle, replacement)})
for item in replacements:
    assert source.count(item['old']) == 1
runner.save(root / '00_nucleo/diagnosticos/p1327-ab-legacy-replacements.json', {
    'source_sha256': hashlib.sha256(source.encode()).hexdigest(),
    'body_sha256': hashlib.sha256(source[source.index('use super::*;'):].encode()).hexdigest(),
    'append_before_unique_anchor': '    // P1305-r2: independent snapshots of pinned vanilla before candidate.\n',
    'snippet': '00_nucleo/diagnosticos/p1327-ab-tests.rs',
    'replacements': replacements,
})
