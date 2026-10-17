from common import *
import gzip,resource
resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))
x={'utc_start':now(),'before_sha256':sha(HERE/'before.json.gz'),'binary_sha256':sha(BINS['tekt']),'script_sha256':sha(__file__),'common_sha256':sha(HERE/'common.py'),'cases':[]}
with tempfile.TemporaryDirectory(prefix='watch-phases-')as temp:
 base=Path(temp)
 for name in ['plain','import','context300']:
  folder=base/name;folder.mkdir();main=folder/'main.typ';pdf=folder/'main.pdf';timing=folder/'timing.json';log=folder/'watch.log';body=(HERE/'fixtures'/(name+'.typ')).read_text();main.write_text(body);(folder/'lib.typ').write_bytes((HERE/'fixtures/lib.typ').read_bytes());cmd=[str(BINS['tekt']),'watch',str(main),str(pdf),'--root',str(folder),'--pdf-tagged=false','--pretty=false','--timings-json',str(timing)];samples=[]
  with log.open('w')as output:
   p=subprocess.Popen(cmd,cwd=ROOT,env=ENV,stdout=output,stderr=output,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}))
   try:
    assert wait_pdf(p,pdf,None,20)
    for i in range(7):
     time.sleep(.18);old=generation(pdf);text=body.replace('Alpha','Bravo'if i%2 else'Alpha');tmp=folder/'main.new';tmp.write_text(text);os.replace(tmp,main);assert wait_pdf(p,pdf,old,20);time.sleep(.15)
     if i:samples.append({'cycle':i,'utc':now(),'source_sha256':sha(main),'timings':json.loads(timing.read_text())})
   finally:p.terminate();p.wait(timeout=5)
  x['cases'].append({'name':name,'command':cmd,'source':body,'samples':samples,'log':log.read_text()});print(name,samples[-1]['timings'],flush=True)
x['utc_end']=now();assert x['binary_sha256']==sha(BINS['tekt'])
with gzip.open(HERE/'phases.json.gz','wt')as f:json.dump(x,f)
