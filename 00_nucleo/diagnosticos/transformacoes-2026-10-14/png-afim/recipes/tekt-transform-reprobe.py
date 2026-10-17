from pathlib import Path
import json,gzip,subprocess,datetime,hashlib,sys
from PIL import Image, ImageChops
root=Path.cwd();base=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14';out=base/sys.argv[1];out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
record=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files,binary_sha256=sha('target/debug/typst'),rows=[])
old=json.load(gzip.open(base/'results.json.gz','rt'));groups={}
for r in old['rows']:groups.setdefault((r['kind'],r['case']),{})[r['binary']]=r
for (kind,name),pair in groups.items():
 orig=pair['tekt'];cmd=orig['command'].copy();expected=pair['vanilla'];row=dict(kind=kind,case=name,source=orig['source'],expected=expected)
 if kind=='render':cmd[-1]=str(out/(name+'.svg'))
 row['command']=cmd;row['start']=now();r=subprocess.run(cmd,capture_output=True,text=True,timeout=60);row.update(end=now(),exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
 if kind=='render' and r.returncode==0 and expected['exit']==0:
  svg=Path(cmd[-1]);png=svg.with_suffix('.png');rc=['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(png),str(svg)];subprocess.run(rc,check=True,capture_output=True)
  reference=base/'renders'/(name+'-vanilla.png');a=Image.open(png).convert('RGB');b=Image.open(reference).convert('RGB');assert a.size==b.size
  ds=[max(v) for v in ImageChops.difference(a,b).getdata()];bbox=lambda im:ImageChops.difference(im,Image.new('RGB',im.size,'white')).getbbox()
  row.update(max_channel=max(ds),different_pixels=sum(x>0 for x in ds),tekt_bbox=bbox(a),reference_bbox=bbox(b),svg_sha256=sha(svg),png_sha256=sha(png),reference_png_sha256=sha(reference))
  print(kind,name,row['different_pixels'],row['tekt_bbox'],row['reference_bbox'],flush=True)
 elif kind=='query':
  row['stdout_match']=r.stdout==expected['stdout'];row['error_match']=r.returncode==expected['exit'] and ((r.stderr.splitlines()[0] if r.stderr else '') == (expected['stderr'].splitlines()[0] if expected['stderr'] else ''))
  if not row['stdout_match'] or not row['error_match']:print(kind,name,'DIFF',r.stdout.strip(),r.stderr.splitlines()[:1],flush=True)
 else:print(kind,name,'EXIT',r.returncode,flush=True)
 record['rows'].append(row)
for group in ['controls','context-controls','style-contract']:
 old=json.loads((base/group/'results.json').read_text())
 for r in old['rows']:
  if r.get('binary')=='tekt':continue
  cmd=r['command'].copy();cmd[0]=str(root/'target/debug/typst')
  if 'compile' in cmd:continue
  q=subprocess.run(cmd,capture_output=True,text=True,timeout=60)
  row=dict(group=group,case=r['case'],command=cmd,source=r.get('source'),expected=r,exit=q.returncode,stdout=q.stdout,stderr=q.stderr)
  row['stdout_match']=q.stdout==r['stdout'];row['error_match']=q.returncode==r['exit'] and ((q.stderr.splitlines()[0] if q.stderr else '') == (r['stderr'].splitlines()[0] if r['stderr'] else ''));record['rows'].append(row)
  if not row['stdout_match'] or not row['error_match']:print(group,r['case'],'DIFF',q.stdout.strip(),q.stderr.splitlines()[:1],flush=True)
record['end']=now();assert all(sha(p)==h for p,h in files.items());assert sha('target/debug/typst')==record['binary_sha256'];record['source_guard']=True
with gzip.open(out/'results.json.gz','wt') as f:json.dump(record,f)
print('saved',out,flush=True)
