from pathlib import Path
from PIL import Image, ImageChops
import hashlib, json, subprocess, datetime
root = Path(__file__).resolve().parents[3]
out = Path(__file__).parent
old = root / '00_nucleo/diagnosticos/typs-teste-2026-09-30/integracao/after'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
result = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'head': subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(), 'binary_sha256':sha(root/'target/debug/typst'), 'cases':{}}
for n in ('09','10','12','22'):
 name=f'sec_{n}'
 source=root/'.typ'/f'{name}.typ'
 pdf=out/f'{name}.pdf'
 subprocess.run([str(root/'target/debug/typst'),'compile',str(source),str(pdf)],check=True,cwd=root)
 oldpdf=old/f'{name}-tekt.pdf'
 def extract(path, mode):
  return subprocess.check_output(['pdftotext',mode,str(path),'-'])
 def raster(path, prefix):
  subprocess.run(['pdftoppm','-f','1','-singlefile','-r','120','-png',str(path),str(prefix)],check=True,stdout=subprocess.DEVNULL)
  return Image.open(str(prefix)+'.png').convert('RGB')
 fresh=raster(pdf,out/name)
 previous=raster(oldpdf,out/(name+'-before'))
 diff=ImageChops.difference(fresh,previous)
 result['cases'][name]={'source_sha256':sha(source),'pdf_sha256':sha(pdf),'old_pdf_sha256':sha(oldpdf),'text_raw_equal':extract(pdf,'-raw')==extract(oldpdf,'-raw'),'text_layout_equal':extract(pdf,'-layout')==extract(oldpdf,'-layout'),'raster_equal':diff.getbbox() is None,'size_equal':fresh.size==previous.size}
(out/'comparison.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
