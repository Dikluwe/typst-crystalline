from pathlib import Path
out=Path(__file__).resolve().parents[1];s=(out/'workers/cli_measure.py').read_text();a=s.index('cases={');z=s.index('class Table',a);cases={
'footer-hidden':'#show table.cell: it => panic("hidden-cell")\n#show table: none\n#table(columns: 2, table.footer([a#strong[b]]))',
'header-hidden':'#let h = table.cell(colspan: 2)[H]\n#show table.cell: it => panic("hidden-cell")\n#show table: none\n#table(columns: 2, table.header(h))',
'footer-late-rule':'#let f = table.footer([a#strong[b]])\n#show table.cell: it => [C#it.body]\n#table(columns: 2, [x], [y], f)',
}
s=s[:a]+'cases='+repr(cases)+'\n'+s[z:];s=s.replace("'cli-'","'cli-extra-'");(out/'workers/cli_extra_measure.py').write_text(s)
