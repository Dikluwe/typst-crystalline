from pathlib import Path
script=Path('00_nucleo/diagnosticos/raw-block-spacing-2026-10-07/measure.py').read_text();prefix=script[:script.index('cases=')];runner=script[script.index("d={'utc'"):]
prefix=prefix.replace("R/'target/release/typst'", "Path(os.environ.get('TEKT_MEASURE_BIN',str(R/'target/release/typst')))")
base='#set page(width:400pt,height:600pt,margin:24pt)\n#set text(font:"Libertinus Serif",size:10pt)\n'
cases={}
for name,count,local,before,after in [
 ('regression-100',100,'set table.cell(fill:red)','',''),
 ('two-cells',2,'set table.cell(fill:red)','',''),
 ('plain-cell',2,'','',''),
 ('outer-14pt',2,'set table.cell(fill:red)','#set text(size:14pt)\n',''),
 ('outer-bold-italic',2,'set table.cell(fill:red)','#set text(weight:"bold",style:"italic")\n',''),
 ('local-explicit-11pt',2,'set table.cell(fill:red);set text(size:11pt)','',''),
 ('local-explicit-18pt',2,'set table.cell(fill:red);set text(size:18pt)','',''),
 ('later-insertion-14pt',2,'set table.cell(fill:red)','','#set text(size:14pt)\n'),
 ('local-size-over-later',2,'set table.cell(fill:red);set text(size:11pt)','','#set text(size:14pt)\n'),
 ('local-show-identity',2,'set table.cell(fill:red);show table.cell: it=>it','',''),
 ('local-show-body',2,'set table.cell(fill:red);show table.cell: it=>it.body','',''),
 ('local-bold-off',2,'set table.cell(fill:red);set text(weight:"regular")','#set text(weight:"bold")\n','')]:
 body=(local+';' if local else '')+'table.cell[Local]';cases[name]=base+before+'#let c={'+body+'}\n'+after+f'#table(columns:{4 if count==100 else 2},..range({count}).map(_=>c))'
exec(prefix.replace("W=Path('/tmp/tekt-raw-block-spacing')","W=Path('/tmp/c2r1-geometry')")+runner,globals())
