from pathlib import Path
script=Path('00_nucleo/diagnosticos/raw-block-spacing-2026-10-07/measure.py').read_text();prefix=script[:script.index('cases=')];runner=script[script.index("d={'utc'"):]
prefix=prefix.replace("R/'target/release/typst'", "Path(os.environ.get('TEKT_MEASURE_BIN',str(R/'target/release/typst')))")
cases={
 'auto':'Before #auto After',
 'cell-align-ctor-control':'#table(columns:1,rows:(30pt,),align:right,table.cell(align:bottom)[abc])',
 'cell-inset-ctor-control':'#table(columns:2,table.cell(inset:3pt)[abc],table.cell(inset:3pt)[def])',
 'cell-stroke-two-sets':'#set table.cell(stroke:red)\n#set table.cell(stroke:2pt)\n#table(columns:1,[abc])',
 'cell-stroke-constructor':'#set table.cell(stroke:red)\n#table(columns:1,stroke:3pt,table.cell(stroke:2pt)[abc])',
 'cell-set-inset':'#set table.cell(inset:3pt)\n#table(columns:2,[abc],[def])',
 'cell-set-span':'#set table.cell(colspan:2)\n#table(columns:3,[abc])',
 'cell-self-set-span':'#show table.cell: set table.cell(colspan:2)\n#table(columns:3,[abc])',
 'cell-partial-inset':'#set table.cell(inset:(left:3pt,top:2pt))\n#table(columns:1,inset:7pt,[abc])',
 'cell-align-fold':'#set table.cell(align:bottom)\n#table(columns:1,rows:(30pt,),align:right,[abc])',
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
runner=runner.replace("  row['runs'][k]=v", """  if r.returncode==0:
   svg=cw/'out.svg';svgcmd=['pdftocairo','-svg',str(cw/'out.pdf'),str(svg)];subprocess.run(svgcmd,check=True,capture_output=True)
   v['svg_command']=svgcmd;v['svg_sha256']=hashlib.sha256(svg.read_bytes()).hexdigest();v['fill_colors']=sorted(set(e.attrib['fill'] for e in ET.fromstring(svg.read_text()).iter() if e.attrib.get('fill') not in (None,'none')));v['stroke_styles']=sorted(set((e.attrib.get('stroke'),e.attrib.get('stroke-width'),e.attrib.get('transform')) for e in ET.fromstring(svg.read_text()).iter() if e.attrib.get('stroke') not in (None,'none')))
  row['runs'][k]=v""")
exec(prefix.replace("W=Path('/tmp/tekt-raw-block-spacing')","W=Path('/tmp/d08b3c2-geometry')")+runner,globals())
