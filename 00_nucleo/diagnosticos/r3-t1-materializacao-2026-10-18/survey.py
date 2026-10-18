from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys,xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):
 with Path(f).open('rb') as s:return hashlib.file_digest(s,'sha256').hexdigest()
mode=sys.argv[1];outdir=p/'outputs'/mode;outdir.mkdir(parents=True,exist_ok=True)
cases=json.loads((r/'00_nucleo/materialization/textual-realization-2026-10-18/acceptance.json').read_text())
bins={'tekt':p/'bin/typst-before' if mode=='before' else r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),pin='bd5ebec285505889e8ff1d4f68804ab147a699f3',binaries={n:dict(path=str(b),sha256=sha(b)) for n,b in bins.items()},source_state='before-sources.json.gz' if mode=='before' else 'candidate-sources.json.gz',cases=[])
for case in cases:
 f=outdir/(case['id']+'-'+case['label']+'.typ');f.write_text(case['source']);assert sha(f)==case['source_sha256']
 row={k:case[k] for k in ['id','label','source','source_sha256']};row['results']={}
 for name,binary in bins.items():
  out=f.with_name(f.stem+'-'+name+'.pdf');cmd=[str(binary),'compile',str(f),str(out)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=60)
  result=dict(command=cmd,exit=z.returncode,stderr=z.stderr,text='')
  if z.returncode==0:
   result['sha256']=sha(out);result['text']=' '.join(subprocess.check_output(['pdftotext',str(out),'-'],text=True).split())
   bbox=out.with_suffix('.bbox');subprocess.run(['pdftotext','-bbox',str(out),str(bbox)],check=True,capture_output=True);root=ET.parse(bbox).getroot();ns={'x':'http://www.w3.org/1999/xhtml'};result['pages']=[dict(width=float(pg.attrib['width']),height=float(pg.attrib['height']),words=[dict(text=w.text,bounds=[float(w.attrib[k]) for k in ['xMin','yMin','xMax','yMax']]) for w in pg.findall('.//x:word',ns)]) for pg in root.findall('.//x:page',ns)]
  row['results'][name]=result
 expected=case['expected_oracle'];actual=row['results']['vanilla'];assert actual['exit']==expected['exit'],case['id']
 if not actual['exit']:assert actual['text']==expected['text'],(case['id'],actual['text'],expected['text'])
 row['frozen_oracle_consistent']=True
 record['cases'].append(row);print(case['id'],case['label'],[(n,v['exit'],v['text'] if not v['exit'] else v['stderr'].splitlines()[0]) for n,v in row['results'].items()],flush=True)
assert all(sha(b)==record['binaries'][n]['sha256'] for n,b in bins.items())
(p/(mode+'.json.gz')).write_bytes(gzip.compress(json.dumps(record,indent=2,ensure_ascii=False).encode()))
