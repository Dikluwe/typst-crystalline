from pathlib import Path
import json,gzip,subprocess,hashlib,datetime,xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;r=Path.cwd();rows=[]
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
for kind in ['underline','strike','overline']:
 for label,body in [('empty',''),('metadata','#metadata(0)'),('par','#par()[]')]:
  source='#set page(width:160pt,height:auto,margin:8pt)\n#set text(size:12pt)\n#set par(hanging-indent:-10pt)\nPREFIX\\\n'+f'#{kind}[{body}]';name=f'empty-deco-{kind}-{label}';f=p/'outputs'/(name+'.typ');f.write_text(source);row=dict(label=name,source=source,source_sha256=sha(f),results={})
  for actor,bin in [('tekt',r/'target/debug/typst'),('vanilla',r/'lab/typst-original/target/release/typst')]:
   out=f.with_name(name+'-'+actor+'.svg');cmd=[str(bin),'compile',str(f),str(out)];z=subprocess.run(cmd,capture_output=True,text=True);assert z.returncode==0,(actor,z.stderr);root=ET.parse(out).getroot();strokes=[dict(e.attrib) for e in root.iter() if e.tag.rsplit('}',1)[-1] in ['line','path'] and 'stroke' in e.attrib];assert not strokes,(name,actor,strokes)
   row['results'][actor]=dict(command=cmd,binary_sha256=sha(bin),artifact_sha256=sha(out),stroke_count=len(strokes),exit=z.returncode)
  rows.append(row);print(name,'passed',flush=True)
(p/'deco-empty-after.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),scope='No painted decoration after empty body under negative hanging, not full document size parity.',cases=rows),indent=2).encode()))
