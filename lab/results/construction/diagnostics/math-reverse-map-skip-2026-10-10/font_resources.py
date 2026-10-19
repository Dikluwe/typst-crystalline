from common import *
import tempfile,collections
x=state(__file__);x['cases']={}
with tempfile.TemporaryDirectory(prefix='cpu-font-resources-')as temp:
 T=Path(temp)
 for f in [p for p in F if p.stem in ['plain','variants-1600']]:
  rows={}
  for e in B:
   pdf=T/(e+'.pdf');cmd=command(e,f,pdf);p=run(cmd);tool=['pdffonts',str(pdf)];text=subprocess.check_output(tool,text=True);entries=[l.split()for l in text.splitlines()[2:]if l.strip()];names=collections.Counter(row[0]for row in entries)
   rows[e]={'command':cmd,'stderr':p.stderr,'tool_command':tool,'tool_output':text,'font_resources':len(entries),'names':dict(names),'pdf_size_bytes':pdf.stat().st_size,'pdf_sha256':sha(pdf)}
  x['cases'][f.stem]=rows;print(f.stem,{e:{k:r[k]for k in ['font_resources','names','pdf_size_bytes']}for e,r in rows.items()},flush=True)
save('font-resources.json.gz',x)
