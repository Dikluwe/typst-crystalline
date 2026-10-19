from pathlib import Path
import subprocess,os,datetime,gzip,json,hashlib
p=Path(__file__).resolve().parent;r=p.parents[2];now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
cmd=['cargo','build','--offline','--locked','--release','-p','typst-wiring','--bin','typst'];start=now();env=dict(os.environ,CARGO_PROFILE_RELEASE_DEBUG='0');z=subprocess.run(cmd,env=env);record={'start':start,'end':now(),'build_argv':cmd,'environment':{'CARGO_PROFILE_RELEASE_DEBUG':'0','CARGO_BUILD_JOBS':os.environ.get('CARGO_BUILD_JOBS'),'CARGO_INCREMENTAL':os.environ.get('CARGO_INCREMENTAL')},'build_exit':z.returncode,'cases':[]}
if z.returncode:
 (p/'release-result.json.gz').write_bytes(gzip.compress(json.dumps(record,indent=2).encode()));raise SystemExit(z.returncode)
binary=r/'target/release/typst';record['binary_sha256']=sha(binary)
for label,expected in [('original-64-2048','A'),('strong-64','X'),('strong-256','X')]:
 source=p/('outputs/'+label+'.typ');out=p/('outputs/release-'+label+'.pdf');rss=out.with_suffix('.rss');command=['/usr/bin/time','-f','%M','-o',str(rss),str(binary),'compile',str(source),str(out)];start=now();z=subprocess.run(command,text=True,capture_output=True,timeout=90);text=' '.join(subprocess.run(['pdftotext',str(out),'-'],text=True,capture_output=True).stdout.split()) if z.returncode==0 else '';row={'label':label,'source':source.read_text(),'source_sha256':sha(source),'argv':command,'start':start,'end':now(),'exit':z.returncode,'stderr':z.stderr,'text':text,'rss_kib':rss.read_text()};record['cases'].append(row);print(row,flush=True)
 (p/'release-result.json.gz').write_bytes(gzip.compress(json.dumps(record,indent=2).encode()))
 assert z.returncode==0 and text==expected,row
