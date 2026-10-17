from common import *
import gzip,signal,struct,resource
resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))
SO=HERE/'heap_probe.so'
def snapshot(p,path):
 for attempt in range(10):
  previous=path.read_text().count('\n')if path.exists()else 0;p.send_signal(signal.SIGUSR2);deadline=time.monotonic()+2
  while time.monotonic()<deadline:
   lines=path.read_text().splitlines()if path.exists()else[]
   if len(lines)>previous:break
   time.sleep(.002)
  assert len(lines)>previous,'missing signal checkpoint'
  result=json.loads(lines[-1])
  if result['consistent']:
   assert not result['dropped'];assert not result['untracked_frees'];break
 else:raise RuntimeError('no consistent heap checkpoint')
 result.update(mem(p.pid));result['retries']=attempt
 try:
  page=os.sysconf('SC_PAGE_SIZE');count=result['registry_bytes']//page
  with open(f'/proc/{p.pid}/pagemap','rb')as f:f.seek(result['registry_base']//page*8);raw=f.read(count*8)
  result['registry_resident_bytes']=sum((x[0]>>63)&1 for x in struct.iter_unpack('Q',raw))*page
 except OSError as e:result['registry_resident_error']=str(e)
 return result
state={'utc_start':now(),'before_sha256':sha(HERE/'before.json.gz'),'binaries':{k:sha(v)for k,v in BINS.items()},'script_sha256':sha(__file__),'probe_source_sha256':sha(HERE/'heap_probe.c.txt'),'probe_sha256':sha(SO),'common_sha256':sha(HERE/'common.py'),'cpu':CPU,'samples':[]}
with tempfile.TemporaryDirectory(prefix='watch-retention-')as temp:
 base=Path(temp)
 for e in BINS:
  folder=base/e;folder.mkdir();main=folder/'main.typ';pdf=folder/'main.pdf';log=folder/'watch.log';heap=folder/'heap.jsonl';original=(HERE/'fixtures/context300.typ').read_text();main.write_text(original);word='Alpha';rows=300
  env=ENV.copy();env.update(LD_PRELOAD=str(SO),HEAP_PHASE_OUT=str(heap));cmd=[str(BINS[e]),'watch',str(main),str(pdf),'--root',str(folder),'--pdf-tagged=false','--pretty=false']+(['--jobs','1']if e=='vanilla'else[])
  with log.open('w')as output:
   p=subprocess.Popen(cmd,cwd=ROOT,env=env,stdout=output,stderr=output,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}))
   try:
    assert wait_pdf(p,pdf,None,20);time.sleep(.25);samples=[{'stage':'initial','cycle':-1,'utc':now(),**snapshot(p,heap)}]
    for i in range(36):
     old=generation(pdf);kind=['same_bytes','small','structural'][i%3]
     if kind=='small':word='Bravo'if word=='Alpha'else'Alpha'
     if kind=='structural':rows=320 if rows==300 else 300
     tmp=folder/'main.new';tmp.write_text(original.replace('Alpha',word).replace('range(300)',f'range({rows})'));os.replace(tmp,main)
     assert wait_pdf(p,pdf,old,20);time.sleep(.15);text=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True);assert word in text and str(rows-1)+':1' in ''.join(text.split())
     samples.append({'stage':kind,'cycle':i,'utc':now(),'source_sha256':sha(main),'pdf_sha256':sha(pdf),'text':text,**snapshot(p,heap)})
    state['samples'].append({'engine':e,'command':cmd,'source':original,'samples':samples,'log':log.read_text(),'raw_heap':heap.read_text()});print(e,samples[0],samples[-1],flush=True)
   finally:p.terminate();p.wait(timeout=5)
state['utc_end']=now();assert state['binaries']=={k:sha(v)for k,v in BINS.items()}
with gzip.open(HERE/'retention.json.gz','wt')as f:json.dump(state,f)
