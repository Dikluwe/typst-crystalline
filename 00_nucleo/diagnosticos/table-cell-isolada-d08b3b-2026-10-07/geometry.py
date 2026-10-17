from pathlib import Path
script=Path('00_nucleo/diagnosticos/raw-block-spacing-2026-10-07/measure.py').read_text();prefix=script[:script.index('cases=')];runner=script[script.index("d={'utc'"):]
cases={
 'auto':'Before #auto After',
 'isolated-getters':'#show table.cell: it=>[X #it.x Y #it.y C #it.colspan R #it.rowspan]\n#table.cell[a]',
 'isolated-body':'Before #table.cell[a] After',
 'isolated-identity':'#show table.cell: it=>it\nBefore #table.cell[a] After',
 'isolated-replaced':'#show table.cell: it=>[CELL #it.body]\nBefore #table.cell[a] After',
 'multi-return':'#show table.cell: it=>[A #it.body]\n#show table.cell: it=>[B #it]\n#table.cell[a]',
 'show-set-color-positional':'#show table.cell: set text(red)\n#show table.cell: it=>it\n#table.cell[a]',
 'show-set-color-named':'#show table.cell: set text(fill:red)\n#show table.cell: it=>it\n#table.cell[a]',
 'show-set-size':'#show table.cell: set text(size:15pt)\n#show table.cell: it=>[CELL #it.body]\n#table.cell[a]',
 'table-two-rules':'#show table.cell: it=>[A #it.body]\n#show table.cell: it=>[B #it]\n#table(columns:1,[a])',
 'isolated-raw':'Before #table.cell[`abc`] After',
}
exec(prefix.replace("W=Path('/tmp/tekt-raw-block-spacing')","W=Path('/tmp/d08b3b-geometry')")+runner,globals())
