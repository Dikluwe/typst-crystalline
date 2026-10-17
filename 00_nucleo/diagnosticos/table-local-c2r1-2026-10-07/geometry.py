from pathlib import Path
script=Path('00_nucleo/diagnosticos/raw-block-spacing-2026-10-07/measure.py').read_text();prefix=script[:script.index('cases=')];runner=script[script.index("d={'utc'"):]
prefix=prefix.replace("R/'target/release/typst'", "Path(os.environ.get('TEKT_MEASURE_BIN',str(R/'target/release/typst')))")
cases={
 'nested-local-text':'#let c={set table.cell(fill:red);show table.cell: it=>[LOCAL #it];table.cell[a]}\n#show table.cell: it=>[OUTER #it]\n#table(columns:1,c)',
 'nested-zero-inset':'#let c={set table.cell(fill:red);show table.cell: it=>[LOCAL #it];table.cell[a]}\n#show table.cell: it=>[OUTER #it]\n#table(columns:1,inset:0pt,c)',
 'prior-local-body':'#let c={show table.cell: it=>[LOCAL #it.body];table.cell([a])}\n#show table.cell: it=>[FUNC#repr(it.body.func())]\n#table(columns:1,c)',
 'local-default-body':'#let c={set table.cell(fill:red);table.cell[a]}\n#set table.cell(fill:blue)\n#table(columns:1,c)',
}
runner=runner.replace("  row['runs'][k]=v", """  if r.returncode==0:
   svg=cw/'out.svg';svgcmd=['pdftocairo','-svg',str(cw/'out.pdf'),str(svg)];subprocess.run(svgcmd,check=True,capture_output=True)
   v['svg_command']=svgcmd;v['svg_sha256']=hashlib.sha256(svg.read_bytes()).hexdigest();v['fill_colors']=sorted(set(e.attrib['fill'] for e in ET.fromstring(svg.read_text()).iter() if e.attrib.get('fill') not in (None,'none')));v['stroke_styles']=sorted(set((e.attrib.get('stroke'),e.attrib.get('stroke-width'),e.attrib.get('transform')) for e in ET.fromstring(svg.read_text()).iter() if e.attrib.get('stroke') not in (None,'none')))
  row['runs'][k]=v""")
exec(prefix.replace("W=Path('/tmp/tekt-raw-block-spacing')","W=Path('/tmp/d08b3c2-geometry')")+runner,globals())
