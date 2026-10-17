"""Scale the original item-7 Typst fixtures without changing their content."""
from pathlib import Path
D = Path(__file__).resolve().parent
R = D.parents[2]
base = R / '00_nucleo/diagnosticos/conjunto-prioridades-vanilla-2026-10-08/fixtures'
out = D / 'fixtures'
text = (base / 'text-dense.typ').read_text()
head, body = text.split('\n\n\n', 1)
paragraph = 'Performance baseline uses identical text, fonts, and options. Repeated words exercise text layout and shaping without generated dates or external packages.'
assert body.split('\n\n') == [paragraph] * 32
shapes = (base / 'shapes.typ').read_text()
shape_head, shape_body = shapes.split('\n', 1)
assert shape_body.startswith('#for i in range(128)')
for scale in (1, 4, 16):
    (out / f'text-dense-{scale}x.typ').write_text(head + '\n\n\n' + '\n\n'.join([paragraph] * (32 * scale)) + '\n')
    (out / f'table-local-{scale}x.typ').write_text(
        '#set page(width: 400pt, height: 600pt, margin: 24pt)\n'
        '#set text(font: "Libertinus Serif", size: 10pt)\n'
        '#let c = { set table.cell(fill: red); table.cell[Local] }\n'
        f'#table(columns: 4, ..range({100 * scale}).map(_ => c))\n'
    )
    (out / f'shapes-{scale}x.typ').write_text(
        shape_head + '\n' + ('\n#pagebreak()\n'.join([shape_body] * scale)) + '\n'
    )
