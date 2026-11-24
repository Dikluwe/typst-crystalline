import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

experiment = Path(__file__).resolve().parent
repo = experiment.parents[2]
out = repo / 'lab/results/construction/diagnostics' / experiment.name
base = out.parent / 'page-three-sequence-continuation-2026-11-21'
source = Path(sys.argv[2])
mode = sys.argv[1]
owners = {
    'trace': 'compiler/layout/page_running_requests',
    'memo': 'compiler/layout/physical_memo',
    'positions': 'entities/sealed_positions',
}
paths = {k: (source/f'01_core/src/{v}.rs', source/f'00_nucleo/prompts/core/{v}.md')
         for k, v in owners.items()}

def change(s, old, new):
    assert s.count(old) == 1, old
    return s.replace(old, new, 1)

manifest = dict((p.removeprefix('./'), h) for h, p in
                map(lambda r: r.split(maxsplit=1), (base/'candidate-source.sha256').read_text().splitlines()))
if mode == 'red':
    for name, digest in manifest.items():
        assert hashlib.sha256((source/name).read_bytes()).hexdigest() == digest, name
    for key, (rust, prompt) in paths.items():
        (out/f'before-{key}.rs').write_bytes(rust.read_bytes())
        (out/f'before-{key}.md').write_bytes(prompt.read_bytes())
        with prompt.open('a') as f:
            f.write((experiment/f'contract-{key}.md').read_text())
    seal = subprocess.run(['crystalline-lint','--fix-hashes','--dry-run',str(source)],
                          stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    (out/'dry-seal.log').write_text(seal.stdout)
    for rust, prompt in paths.values():
        owner = prompt.relative_to(source)
        row = next(row for row in seal.stdout.splitlines() if f'{rust} prompt={owner} ' in row)
        header, code = re.search(r'hash-a=(\w+) hash-b=(\w+)', row).groups()
        rust.write_text(re.sub(r'@prompt-hash \w+', f'@prompt-hash {header}', rust.read_text(), count=1))
        prompt.write_text(re.sub(r'Hash do Código: \w+', f'Hash do Código: {code}', prompt.read_text(), count=1))
    with paths['positions'][0].open('a') as f:
        f.write((experiment/'positions-tests.rs').read_text())
        f.write('\n#[cfg(test)]\nimpl SealedPositions {\n    fn lab_position_overlay(&self, _: &[(Location, Position)]) -> Option<Self> { None }\n}\n')
elif mode == 'green':
    rust = paths['positions'][0]
    s = rust.read_text().rsplit('\n#[cfg(test)]\nimpl SealedPositions {', 1)[0]
    rust.write_text(s + (experiment/'positions.rs').read_text())
    rust = paths['trace'][0]
    with rust.open('a') as f:
        f.write((experiment/'trace.rs').read_text())
    rust = paths['memo'][0]
    s = rust.read_text()
    old = '    let compacted = product.trace.data.lab_geometry_lines;'
    assert s.count(old) == 1
    s = change(s, '''    let product = super::convergence::run_planned_lab(body, input, recorded, images, 11.0,
        inputs, Some(&witness.journal), false, true, Some(&mut witness.checkpoint)).into_result()?;
    Ok((product, witness))''', '''    let previous = recorded.inner.paragraph_session()
        .and_then(|session| session.lab_previous_physical.lock().unwrap().clone());
    let mut product = super::convergence::run_planned_lab(body, input, recorded, images, 11.0,
        inputs, Some(&witness.journal), false, true, Some(&mut witness.checkpoint)).into_result()?;
    if let Some(previous) = previous {
        let (positions, receipt) = product.trace.lab_remap_suffix_positions(&previous.trace);
        if let Some(positions) = positions {
            product.document.extracted_positions = positions;
        }
        lab_position_remap_receipt(&receipt);
    }
    Ok((product, witness))''')
    rust.write_text(s + '''
#[inline(never)]
fn lab_position_remap_receipt(receipt: &[u64; 14]) {
    std::hint::black_box(receipt);
}
''')
    changed = {p: h for p in manifest if (h := hashlib.sha256((source/p).read_bytes()).hexdigest()) != manifest[p]}
    assert set(changed) == {str(p.relative_to(source)) for pair in paths.values() for p in pair}, changed
    (out/'freeze.json').write_text(json.dumps(changed,indent=2)+'\n')
    patch = []
    for k, pair in paths.items():
        for ext, p in zip(('rs', 'md'), pair):
            frozen = out/f'candidate-{k}.{ext}'
            frozen.write_bytes(p.read_bytes())
            rel = str(p.relative_to(source))
            diff = subprocess.run(['diff','-u','--label','a/'+rel,'--label','b/'+rel,
                                   str(out/f'before-{k}.{ext}'),str(frozen)],capture_output=True)
            assert diff.returncode == 1
            patch.append(diff.stdout)
    (out/'candidate.patch').write_bytes(b''.join(patch))
else:
    raise SystemExit('use red or green')
