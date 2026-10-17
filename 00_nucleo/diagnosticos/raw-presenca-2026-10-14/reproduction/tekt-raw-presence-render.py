from pathlib import Path
import subprocess,datetime,hashlib,json,gzip,sys
out=Path('00_nucleo/diagnosticos/raw-presenca-2026-10-14')/('render-'+sys.argv[1]);out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
cases={'inline':'#raw("x",block:false,lang:none)','block':'#raw("x\\ny",block:true,lang:"typc")','markup':'`x`\n\n```typc\nx\ny\n```','display':'#false #auto','show':'#show raw: it=>[R#it];#raw("x")'}
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binary_sha256=sha('target/debug/typst'),rows=[])
for name,body in cases.items():
 source=out/(name+'.typ');source.write_text('#set page(width:140pt,height:100pt,margin:10pt)\n'+body);pdf=source.with_suffix('.pdf');cmd=['target/debug/typst','compile',str(source),str(pdf)];r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,r.stderr
 q=subprocess.run(['qpdf','--check',str(pdf)],capture_output=True,text=True);assert q.returncode==0,q.stderr
 subprocess.run(['pdftoppm','-singlefile','-scale-to','560','-png',str(pdf),str(pdf.with_suffix(''))],check=True,capture_output=True)
 text=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True)
 record['rows'].append(dict(name=name,source=source.read_text(),command=cmd,pdf_sha256=sha(pdf),png_sha256=sha(pdf.with_suffix('.png')),text=text,qpdf=q.stdout,stderr=r.stderr))
with gzip.open(out/'results.json.gz','wt') as f:json.dump(record,f)
print('render',sys.argv[1],'complete')
