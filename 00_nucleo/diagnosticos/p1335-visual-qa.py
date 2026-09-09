"""Receipt of read-only visual QA, not a document-generation workflow."""
import importlib.util, json, subprocess, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('receipt', D/'p1335-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
before = r.state(); r.verify(before)
raw = D/'p1335-transversal-r3.json'
data = json.loads(raw.read_text())
directory = Path(data['output_root'])/'normal/default/P1138-L-001'
qa = Path('/tmp/p1335-visual-qa-llbfc0fs')
observations = {}
for side in ('oracle', 'crystalline'):
    pdf, png = directory/(side+'.pdf'), qa/(side+'.png')
    assert r.sha(pdf) == data['artifacts'][str(pdf)]
    commands = [['pdfinfo', str(pdf)], ['pdftotext', '-layout', str(pdf), '-']]
    outputs = []
    for argv in commands:
        started = r.now(); p = subprocess.run(argv, capture_output=True, text=True, check=True)
        outputs.append(dict(argv=argv, started_at=started, ended_at=r.now(), exit=p.returncode, stdout=p.stdout, stderr=p.stderr))
    observations[side] = dict(pdf=dict(path=str(pdf), sha256=r.sha(pdf)), png=dict(path=str(png), sha256=r.sha(png)),
        rendering_argv=['pdftoppm', '-f', '1', '-singlefile', '-scale-to', '1400', '-png', str(pdf), str(qa/side)], commands=outputs)
assert observations['oracle']['png']['sha256'] == observations['crystalline']['png']['sha256']
after = r.state(); r.verify(after)
r.save('visual-qa', dict(at=r.now(), before=before, after=after,
    inputs={str(p):r.sha(p) for p in [raw, Path(__file__), D/'p1335-manifest.json']}, observations=observations,
    visual_inspection=dict(executor='/root', tool='view_image', scope='Only both first pages of normal/default/P1138-L-001',
        finding='The one-page plain fixture shows Hello, parity. near the upper left text margin on an otherwise blank white page. No visible clipping, overlap or missing glyph. Both 990x1400 Poppler renders have the same SHA-256; the shared rendered page was inspected.',
        limitation='This sample neither certifies general layout/PDF/a11y nor treats PDF byte equality as language parity. Different PDF metadata/bytes are recorded in the source receipts.'),
    document_authoring=False, original_pdf_edited=False))
