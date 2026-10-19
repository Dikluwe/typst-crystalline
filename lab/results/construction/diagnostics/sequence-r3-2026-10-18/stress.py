from pathlib import Path
import subprocess,json,gzip,datetime,resource,hashlib,time,sys
p=Path(__file__).resolve().parent;r=p.parents[2];phase=sys.argv[1];rows=[]
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as s:
  for b in iter(lambda:s.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
cases={
 'original-64-2048':'#let values=range(2048)\n#let s=state("k",none)\n#for i in range(64) {s.update(values)}\n#context assert.eq(s.final().len(),2048)\nA',
 'larger-256-128':'#let values=range(128)\n#let s=state("k",none)\n#for i in range(256) {s.update(values)}\n#context assert.eq(s.final().len(),128)\nA',
 'while-64':'#let s=state("k",0)\n#let n=0\n#while n < 64 {n += 1;s.update(n)}\n#context assert.eq(s.final(),64)\nA',
 'shape-for-256':'#let c=for i in range(256) {[X]}\n#assert.eq(repr(c),"sequence("+("[X]",)*256 .join(", ")+")")\nA',
 'strong-16':'#let body=[X]\n#for i in range(16) {body=strong(body)}\n#body',
 'strong-256':'#let body=[X]\n#for i in range(256) {body=strong(body)}\n#body',
 'strong-64':'#let body=[X]\n#for i in range(64) {body=strong(body)}\n#body',
}
# Construct the expected representation independently of the parser's precedence.
cases['shape-for-256']='#let c=for i in range(256) {[X]}\n#assert.eq(repr(c).split("sequence(").len(),2)\n#assert.eq(repr(c).split("[X]").len(),257)\nA'
bins={'candidate':r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
for label,src in cases.items():
 f=p/('outputs/'+label+'.typ');f.write_text(src+'\n');row={'case':label,'source':src,'sha256':sha(f),'results':{}}
 for name,binary in bins.items():
  out=p/('outputs/'+phase+'-'+label+'-'+name+'.pdf');rss=out.with_suffix('.rss');cmd=['/usr/bin/time','-f','%M','-o',str(rss),str(binary),'compile',str(f),str(out)];start=datetime.datetime.now(datetime.timezone.utc).isoformat();t=time.monotonic()
  try:z=subprocess.run(cmd,text=True,capture_output=True,timeout=90);result={'exit':z.returncode,'stderr':z.stderr,'stdout':z.stdout}
  except subprocess.TimeoutExpired as e:result={'exit':'timeout','stderr':str(e),'stdout':''}
  result.update(argv=cmd,start=start,end=datetime.datetime.now(datetime.timezone.utc).isoformat(),elapsed=time.monotonic()-t,rss=rss.read_text() if rss.exists() else None,binary_sha256=sha(binary));result['text']=' '.join(subprocess.run(['pdftotext',str(out),'-'],text=True,capture_output=True).stdout.split()) if result['exit']==0 else '';row['results'][name]=result
 rows.append(row);print(label,[(k,v['exit'],v['text'],v['rss']) for k,v in row['results'].items()],flush=True)
(p/('stress-'+phase+'.json.gz')).write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'stack_limit':resource.getrlimit(resource.RLIMIT_STACK),'cases':rows},indent=2).encode()))
for c in rows:
 candidate=c['results']['candidate'];vanilla=c['results']['vanilla']
 if c['case']=='strong-256':
  assert vanilla['exit']==1 and 'maximum show rule depth exceeded' in vanilla['stderr']
  assert candidate['exit'] in [0,1],candidate
  assert candidate['text']=='X' if candidate['exit']==0 else 'depth exceeded' in candidate['stderr']
 else:
  assert candidate['exit']==vanilla['exit']==0 and candidate['text']==vanilla['text'],c['case']
