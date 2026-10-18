from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,re
p=Path(__file__).resolve().parent;root=p.parents[2];work=p/'outputs';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for chunk in iter(lambda:stream.read(1024*1024),b''):h.update(chunk)
 return h.hexdigest()
inputs={
 'minimal':'#set page(width:100pt,height:100pt,margin:5pt)\n#set text(font:"DejaVu Sans",size:10pt)\nA',
 'plain':'#set page(width:300pt,height:500pt,margin:8pt)\n#set text(font:"DejaVu Sans",size:10pt)\n#for i in range(20) [Linha #i texto de controle.#linebreak()]',
 'state-set-heavy':'#let values=range(512)\n#let s=state("k",none)\n#for i in range(8) {s.update(values)}\n#context assert.eq(s.final().len(),512)\nA',
 'heading':'#set page(width:300pt,height:500pt,margin:8pt)\n#set text(font:"DejaVu Sans",size:10pt)\n#set heading(numbering:"1")\n= Titulo\nCorpo de controle.',
}
bins={'tekt-before':work/'typst-before','tekt-after':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
rows=[]
for index,(label,source) in enumerate(inputs.items()):
 f=work/('memory-'+label+'.typ');f.write_text(source+'\n')
 names=list(bins);names=names[index:]+names[:index]
 for name in names:
  output=work/('memory-'+label+'-'+name+'.pdf');rss=work/('memory-'+label+'-'+name+'.rss');cmd=['/usr/bin/time','-f','%M','-o',str(rss),str(bins[name]),'compile',str(f),str(output),'--font-path','/usr/share/fonts/truetype/dejavu'];start=now();r=subprocess.run(cmd,capture_output=True,text=True,timeout=90)
  text=subprocess.run(['pdftotext',str(output),'-'],text=True,capture_output=True).stdout if r.returncode==0 else ''
  row={'case':label,'backend':name,'source':source,'source_sha256':sha(f),'binary_sha256':sha(bins[name]),'command':cmd,'start':start,'end':now(),'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr,'maximum_rss_kib':int(rss.read_text().strip()) if rss.read_text().strip().isdigit() else None,'normalized_text':' '.join(text.split()),'page_breaks':text.count('\f')};rows.append(row);print(label,name,row['maximum_rss_kib'],r.returncode,flush=True)
d={'utc':now(),'rows':rows,'method':'One process per input/backend, order rotated across cases; maximum RSS from GNU time in KiB; debug baseline/debug0 candidate versus release oracle, descriptive samples only, not a general performance benchmark.'};(p/'runtime-memory.json.gz').write_bytes(gzip.compress(json.dumps(d,ensure_ascii=False,indent=2).encode()))
assert all(r['exit']==0 for r in rows)
for case in inputs:
 group=[r for r in rows if r['case']==case];assert len({r['normalized_text'] for r in group})==1,case
