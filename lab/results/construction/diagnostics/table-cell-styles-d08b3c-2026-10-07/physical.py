from pathlib import Path
script=Path('00_nucleo/diagnosticos/raw-block-spacing-2026-10-07/measure.py').read_text();prefix=script[:script.index('cases=')];runner=script[script.index("d={'utc'"):]
cases={
 'fill-with-body-recipe':'#show table.cell: it=>it.body\n#set table.cell(fill:red)\n#table(columns:1,[abc])',
 'stroke-with-body-recipe':'#show table.cell: it=>it.body\n#set table.cell(stroke:red+2pt)\n#table(columns:1,[abc])',
 'fill-with-hidden-recipe':'#show table.cell: it=>[HIDDEN]\n#set table.cell(fill:red)\n#table(columns:1,[abc])',
 'default-with-body-recipe':'#show table.cell: it=>it.body\n#table(columns:1,[abc])',
}

runner=runner.replace("  row['runs'][k]=v", """  if r.returncode==0:
   svg=cw/'out.svg';svgcmd=['pdftocairo','-svg',str(cw/'out.pdf'),str(svg)];subprocess.run(svgcmd,check=True,capture_output=True)
   v['svg_command']=svgcmd;v['svg_sha256']=hashlib.sha256(svg.read_bytes()).hexdigest();v['fill_colors']=sorted(set(e.attrib['fill'] for e in ET.fromstring(svg.read_text()).iter() if e.attrib.get('fill') not in (None,'none')));v['stroke_styles']=sorted(set((e.attrib.get('stroke'),e.attrib.get('stroke-width'),e.attrib.get('transform')) for e in ET.fromstring(svg.read_text()).iter() if e.attrib.get('stroke') not in (None,'none')))
  row['runs'][k]=v""")
exec(prefix.replace("W=Path('/tmp/tekt-raw-block-spacing')","W=Path('/tmp/d08b3c-physical-before')")+runner,globals())
