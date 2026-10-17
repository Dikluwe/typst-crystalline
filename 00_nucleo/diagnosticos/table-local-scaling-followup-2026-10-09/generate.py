"""Diagnostic controls for the scaled local table style fixture."""
from pathlib import Path
D=Path(__file__).resolve().parent
F=D/'fixtures';F.mkdir(exist_ok=True)
head='#set page(width: 400pt, height: 600pt, margin: 24pt)\n#set text(font: "Libertinus Serif", size: 10pt)\n'
variants={
 'local-set':'#let c = { set table.cell(fill: red); table.cell[Local] }\n',
 'explicit-fill':'#let c = table.cell(fill: red)[Local]\n',
 'no-fill':'#let c = table.cell[Local]\n',
 'alias':'#let c = [Local]\n',
 'inline-set':'',
}
for n in (100,400,1600):
 for name,decl in variants.items():
  if name=='inline-set':body=f'#table(columns: 4, ..range({n}).map(_ => {{ set table.cell(fill: red); table.cell[Local] }}))\n'
  else:body=f'#table(columns: 4, ..range({n}).map(_ => c))\n'
  (F/f'{name}-{n}.typ').write_text(head+decl+body)
