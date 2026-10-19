from pathlib import Path
import datetime, hashlib, json, subprocess
r=Path.cwd();d=r/'00_nucleo/diagnosticos/correcao-bundle-pdf-validacoes-2026-09-30';binary=r/'target/release/typst';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
cases={
 'single_missing':('#set format.pdf(standard: "ua-1")\nHello\n','pdf'),
 'bundle_missing':('#document("one.pdf")[#set format.pdf(standard: "ua-1")\nHello]\n','bundle'),
 'bundle_titled':('#document("one.pdf", title: [Okay])[#set format.pdf(standard: "ua-1")\nHello]\n','bundle'),
}
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','binary_sha256':sha(binary),'cases':{}}
for name,(text,fmt) in cases.items():
 src=d/(name+'.typ');src.write_text(text);dst=d/(name+'.pdf' if fmt=='pdf' else name+'-out');cmd=[str(binary),'compile',str(src),str(dst),'--format',fmt,*(['--features','bundle'] if fmt=='bundle' else [])];run=subprocess.run(cmd,capture_output=True,text=True);row={'source_sha256':sha(src),'command':cmd,'exit':run.returncode,'stdout':run.stdout,'stderr':run.stderr};pdf=dst if fmt=='pdf' else dst/'one.pdf'
 if pdf.is_file():
  info=subprocess.run(['pdfinfo','-meta',str(pdf)],capture_output=True,text=True,check=True);row.update(pdf_sha256=sha(pdf),pdf_bytes=pdf.stat().st_size,xmp_pdfua1='<pdfuaid:part>1</pdfuaid:part>' in info.stdout,xmp_title='<dc:title>' in info.stdout)
 record['cases'][name]=row
(d/'probe-final.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
for name,row in record['cases'].items():print(name,{k:v for k,v in row.items() if k not in ('command','stdout','stderr','source_sha256','pdf_sha256')},row['stderr'].splitlines()[:1])
