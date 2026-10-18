from pathlib import Path
import subprocess,hashlib,json,datetime,sys,gzip
import xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;r=Path.cwd();rows=[]
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
for kind in ['underline','strike','overline']:
 for amount in [0,10]:
  for forced in [False,True]:
   label=f'deco-{kind}-{amount}-{forced}'
   source='#set page(width:160pt,height:auto,margin:8pt)\n#set text(size:12pt,hyphenate:false)\n'+f'#set par(hanging-indent:{amount}pt)\n'+('PREFIX\\\n' if forced else '')+f'#{kind}[FIRST alpha beta gamma delta epsilon zeta eta theta iota kappa lambda LAST]'
   f=p/'outputs'/(label+'.typ');f.write_text(source);row=dict(label=label,source=source,source_sha256=sha(f),results={})
   for name,bin in [('tekt',r/'target/debug/typst'),('vanilla',r/'lab/typst-original/target/release/typst')]:
    out=f.with_name(label+'-'+name+'.svg');cmd=[str(bin),'compile',str(f),str(out)];z=subprocess.run(cmd,capture_output=True,text=True);assert z.returncode==0,(cmd,z.stderr)
    root=ET.parse(out).getroot();lines=[]
    for el in root.iter():
     if el.tag.endswith('path') and 'stroke' in el.attrib:lines.append(dict(el.attrib))
    row['results'][name]=dict(command=cmd,binary_sha256=sha(bin),exit=z.returncode,artifact_sha256=sha(out),lines=lines)
   rows.append(row);print(label,[(k,v['lines']) for k,v in row['results'].items()],flush=True)
(p/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),cases=rows),indent=2).encode()))
