import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

experiment = Path(__file__).resolve().parent
root = experiment.parents[2]
source = Path(sys.argv[2])
output = root / 'lab/results/construction/diagnostics' / experiment.name
paths = {
    'metrics': 'font_metrics',
    'fallback': 'font_metrics/fallback',
    'session': 'shaper/complete_session',
}
mode = sys.argv[1]
if mode == 'red':
    previous = output.parent / 'reserve-locations-borrowed-2026-11-21/candidate-source.sha256'
    for line in previous.read_text().splitlines():
        digest, filename = line.split(maxsplit=1)
        assert hashlib.sha256((source / filename).read_bytes()).hexdigest() == digest, filename
    for name, path in paths.items():
        rust = source / f'03_infra/src/{path}.rs'
        l0 = source / f'00_nucleo/prompts/infra/{path}.md'
        (output / f'before-{name}.rs').write_bytes(rust.read_bytes())
        (output / f'before-{name}.md').write_bytes(l0.read_bytes())
        with l0.open('a') as file:
            file.write((experiment / f'{name}.md').read_text())
    proc = subprocess.run(['crystalline-lint', '--fix-hashes', '--dry-run', str(source)],
                          text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    (output / 'dry-seal.log').write_text(proc.stdout)
    for name, path in paths.items():
        rust = source / f'03_infra/src/{path}.rs'
        l0 = source / f'00_nucleo/prompts/infra/{path}.md'
        line = next(line for line in proc.stdout.splitlines()
                    if f'/{path}.rs prompt=00_nucleo/prompts/infra/{path}.md ' in line)
        seal, code = re.search(r'hash-a=(\w+) hash-b=(\w+)', line).groups()
        rust.write_text(re.sub(r'@prompt-hash \w+', f'@prompt-hash {seal}', rust.read_text(), count=1))
        l0.write_text(re.sub(r'Hash do Código: \w+', f'Hash do Código: {code}', l0.read_text(), count=1))
    path = source / '03_infra/src/shaper/complete_session.rs'
    path.write_text(path.read_text() + (experiment / 'tests.rs').read_text() + (experiment / 'stub.rs').read_text())
elif mode == 'green':
    session = source / '03_infra/src/shaper/complete_session.rs'
    s = session.read_text()
    stub = (experiment / 'stub.rs').read_text()
    assert s.endswith(stub)
    s = s[:-len(stub)]
    needle = '    memo: Arc<Mutex<Memo>>,'
    assert s.count(needle) == 1
    s = s.replace(needle, needle + '\n    paragraph_ascii: Arc<Mutex<VecDeque<ParagraphAscii>>>,')
    session.write_text(s + (experiment / 'coverage.rs').read_text())
    metrics = source / '03_infra/src/font_metrics.rs'
    s = metrics.read_text()
    s = s.replace("    world: &'a dyn World,", "    world: &'a dyn World,\n    paragraph_coverage: Option<crate::shaper::ShapeSession>,", 1)
    s = s.replace('let metrics = Self::with_paragraph_session(world, paragraphs);',
                  'let mut metrics = Self::with_paragraph_session(world, paragraphs);\n        metrics.paragraph_coverage = Some(products.clone());')
    s = s.replace('            paragraph_products: None,', '            paragraph_products: None,\n            paragraph_coverage: None,')
    s = s.replace('            paragraph_products: self.paragraph_products.clone(),',
                  '            paragraph_products: self.paragraph_products.clone(),\n            paragraph_coverage: self.paragraph_coverage.clone(),')
    metrics.write_text(s)
    fallback = source / '03_infra/src/font_metrics/fallback.rs'
    s = fallback.read_text()
    needle = '''        let face = cached.face();
        if !text.chars().all(|c| face.glyph_index(c).is_some_and(|g| g.0 != 0)) {
            return None;
        }'''
    assert s.count(needle) == 1
    s = s.replace(needle, '''
        if !paragraph_has_coverage(self.paragraph_coverage.as_ref(), &font, cached.face(), text) {
            return None;
        }'''.lstrip('\n'))
    # Exercise the shared store in the existing replacement tests.
    s = s.replace('let metrics = FallbackFontMetrics::with_paragraph_session(&world, &session);\n        let style',
                  'let products = crate::shaper::ShapeSession::new();\n        let metrics = FallbackFontMetrics::with_product_sessions(&world, &session, &products);\n        let style', 1)
    s = s.replace('let fresh = FallbackFontMetrics::with_paragraph_session(&world, &session);\n        assert!(',
                  'let fresh = FallbackFontMetrics::with_product_sessions(&world, &session, &products);\n        assert!(', 1)
    fallback.write_text(s + (experiment / 'fallback.rs').read_text())
    receipts = {}
    for name, path in paths.items():
        rust = source / f'03_infra/src/{path}.rs'
        l0 = source / f'00_nucleo/prompts/infra/{path}.md'
        for f in (rust, l0):
            receipts[str(f.relative_to(source))] = hashlib.sha256(f.read_bytes()).hexdigest()
        (output / f'candidate-{name}.rs').write_bytes(rust.read_bytes())
        (output / f'candidate-{name}.md').write_bytes(l0.read_bytes())
    (output / 'changed-files.json').write_text(json.dumps(receipts, indent=2))
else:
    raise SystemExit('choose red or green')
