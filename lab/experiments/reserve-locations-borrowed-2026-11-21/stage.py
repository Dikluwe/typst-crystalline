import hashlib
from pathlib import Path
import re
import sys

experiment = Path(__file__).resolve().parent
root = experiment.parents[2]
source = Path(sys.argv[2])
walk = source / '01_core/src/compiler/introspect/walk.rs'
owner = source / '00_nucleo/prompts/core/compiler/introspect/walk.md'
output = root / 'lab/results/construction/diagnostics' / experiment.name
if sys.argv[1] == 'red':
    baseline = walk.read_text()
    assert baseline == (root / walk.relative_to(source)).read_text()
    assert owner.read_bytes() == (root / owner.relative_to(source)).read_bytes()
    (output / 'baseline-walk.rs').write_text(baseline)
    (output / 'baseline-walk.md').write_bytes(owner.read_bytes())
    with owner.open('a') as file:
        file.write((experiment / 'contract.md').read_text())
    digest = hashlib.sha256(owner.read_bytes()).hexdigest()
    (output / 'l0.sha256').write_text(f'{digest}  {owner.relative_to(source)}\n')
    # The linter hashes normalized content. Obtain the seal using its own command.
    import subprocess
    copied = output / 'lint-red.log'
    with copied.open('w') as log:
        subprocess.run(['crystalline-lint', '--fix-hashes', str(source)], stdout=log,
                       stderr=subprocess.STDOUT, check=False)
    sealed_header = re.search(r'@prompt-hash \w+', walk.read_text()).group()
    baseline = re.sub(r'@prompt-hash \w+', sealed_header, baseline, count=1)
    walk.write_text(baseline + '\nuse crate::entities::source_result::SourceResult;\n' + (experiment / 'tests.rs').read_text() +
                    '\nfn try_reserve_mapped(_: &Content, _: &mut Locator, _: bool) -> SourceResult<bool> { Ok(false) }\n')
elif sys.argv[1] == 'green':
    s = walk.read_text()
    stub = '\nfn try_reserve_mapped(_: &Content, _: &mut Locator, _: bool) -> SourceResult<bool> { Ok(false) }\n'
    assert s.endswith(stub)
    needle = '    let _ = content.map_content(&mut |node| {'
    assert needle in s
    s = s[:-len(stub)].replace(needle,
        '    let _ = reserve_mapped_locations(content, locator);\n}\n\n'
        'fn reserve_mapped_locations(content: &Content, locator: &mut Locator) -> SourceResult<()> {\n'
        '    if try_reserve_mapped(content, locator, true)? {\n'
        '        return Ok(());\n'
        '    }\n' + needle.replace('let _ = ', '', 1), 1)
    end = '        Ok(None)\n    });\n}'
    assert end in s
    s = s.replace(end, '        Ok(None)\n    }).map(|_| ())\n}', 1)
    imported = '\nuse crate::entities::source_result::SourceResult;\n'
    s = s.replace(imported, '', 1).replace('use super::*;', 'use super::*;' + imported, 1)
    boundary = s.index('#[expect(\n    clippy::too_many_arguments,')
    walk.write_text(s[:boundary] + (experiment / 'borrowed.rs').read_text() + '\n' + s[boundary:])
else:
    raise SystemExit('choose red or green')
