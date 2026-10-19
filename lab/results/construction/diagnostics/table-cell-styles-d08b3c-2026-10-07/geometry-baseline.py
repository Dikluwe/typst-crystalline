from pathlib import Path
script=Path('00_nucleo/diagnosticos/raw-block-spacing-2026-10-07/measure.py').read_text();prefix=script[:script.index('cases=')].replace("('tekt',R/'target/release/typst')","('tekt',Path('/tmp/d08b3c-styles/tekt-721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3'))");runner=script[script.index("d={'utc'"):]
cases={
 'cell-align-ctor-control':'#table(columns:1,rows:(30pt,),align:right,table.cell(align:bottom)[abc])',
 'cell-inset-ctor-control':'#table(columns:2,table.cell(inset:3pt)[abc],table.cell(inset:3pt)[def])',
}

runner=runner.replace("  row['runs'][k]=v", """  if r.returncode==0:
   svg=cw/'out.svg';svgcmd=['pdftocairo','-svg',str(cw/'out.pdf'),str(svg)];subprocess.run(svgcmd,check=True,capture_output=True)
   v['svg_command']=svgcmd;v['svg_sha256']=hashlib.sha256(svg.read_bytes()).hexdigest();v['stroke_styles']=sorted(set((e.attrib.get('stroke'),e.attrib.get('stroke-width'),e.attrib.get('transform')) for e in ET.fromstring(svg.read_text()).iter() if e.attrib.get('stroke') not in (None,'none')))
  row['runs'][k]=v""")
exec(prefix.replace("W=Path('/tmp/tekt-raw-block-spacing')","W=Path('/tmp/d08b3c-geometry-baseline')")+runner,globals())
