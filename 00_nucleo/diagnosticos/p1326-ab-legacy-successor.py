from pathlib import Path
import subprocess

base = Path('00_nucleo/diagnosticos')
source = (base / 'p1326-legacy-tests-original.rs').read_text()
old = 'for f in [closure, element] {'
assert source.count(old) == 1
source = source.replace(old, '''for depth in [0, 1, 3] {
            for span in anchors() {
                expect_error(
                    wrapped(closure.clone(), depth),
                    "missing",
                    span,
                    "cannot access fields on user-defined functions",
                );
            }
        }
        for f in [element] {''')
old = 'for f in [closure, element, plugin] {'
assert source.count(old) == 1
source = source.replace(old, '''assert_error(closure.clone(), "cannot access fields on user-defined functions");
        assert_error(
            closure.with(Args::positional(vec![])),
            "cannot access fields on user-defined functions",
        );
        for f in [element, plugin] {''')
target = base / 'p1326-ab-legacy-successor.rs'
patch = f'*** Begin Patch\n*** Add File: {target}\n' + ''.join('+' + line + '\n' for line in source.splitlines()) + '*** End Patch\n'
subprocess.run(['apply_patch', patch], check=True)
