from common import *
import gzip,resource,sys
resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))
F=HERE/'fixtures'
NAMES=['context300']if '--context-only'in sys.argv else['plain','import','context300']
def source(name,word,rows):
 t=(F/(name+'.typ')).read_text().replace('Alpha',word)
 return t.replace('range(300)'if name=='context300'else'range(8)',f'range({rows})')
def run_case(engine,name,round_no,base):
 folder=base/f'{name}-{round_no}-{engine}';folder.mkdir();lib=folder/'lib.typ';lib.write_bytes((F/'lib.typ').read_bytes());main=folder/'main.typ';pdf=folder/'main.pdf';log=folder/'watch.log';word='Alpha';rows=300 if name=='context300'else 8;main.write_text(source(name,word,rows))
 cmd=[str(BINS[engine]),'watch',str(main),str(pdf),'--root',str(folder),'--pdf-tagged=false','--pretty=false']+(['--jobs','1']if engine=='vanilla'else[])
 samples=[]
 with log.open('w')as output:
  start=time.perf_counter_ns();p=subprocess.Popen(cmd,cwd=ROOT,env=ENV,stdout=output,stderr=output,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}))
  try:
   assert wait_pdf(p,pdf,None,20),'missing initial PDF';initial={'wall_ms':(time.perf_counter_ns()-start)/1e6,'cpu_ms':cpu_ns(p.pid)/1e6,**mem(p.pid)}
   time.sleep(.15);old=generation(pdf);c=cpu_ns(p.pid);time.sleep(.8);idle={'cpu_ms':(cpu_ns(p.pid)-c)/1e6,'publication':generation(pdf)!=old,**mem(p.pid)};assert not idle['publication']
   for cycle in range(16):
    kind=('metadata_touch','same_bytes','small','structural')[cycle%4];time.sleep(.18);old=generation(pdf);c=cpu_ns(p.pid);before=mem(p.pid);logsize=log.stat().st_size;t=time.perf_counter_ns()
    if kind=='metadata_touch':
     st=main.stat();os.utime(main,ns=(st.st_atime_ns,st.st_mtime_ns+1_000_000_000))
    else:
     if kind=='small':word='Bravo'if word=='Alpha'else'Alpha'
     if kind=='structural':rows=(320 if rows==300 else 300)if name=='context300'else(12 if rows==8 else 8)
     tmp=folder/'main.new';tmp.write_text(source(name,word,rows));os.replace(tmp,main)
    published=wait_pdf(p,pdf,old,.8 if kind=='metadata_touch'else 20);latency=(time.perf_counter_ns()-t)/1e6 if published else None
    time.sleep(.12);aftercpu=cpu_ns(p.pid);after=mem(p.pid);text=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True);assert word in text,(name,engine,cycle,text)
    if name!='context300':assert text.count('Item')==rows,(name,engine,cycle,text)
    else:assert str(rows-1)+':1' in ''.join(text.split()),(name,engine,cycle,text)
    if kind!='metadata_touch':assert published,(name,engine,kind)
    sample={'cycle':cycle,'kind':kind,'word':word,'rows':rows,'published':published,'latency_ms':latency,'cpu_ms':(aftercpu-c)/1e6,'before_mem_kib':before,'after_mem_kib':after,'source_sha256':sha(main),'source':main.read_text(),'pdf_sha256':sha(pdf),'text':text,'words':word_boxes(pdf),'log_delta':log.stat().st_size-logsize,'utc':now()}
    assert sample['cpu_ms']>=0
    if cycle>=4:samples.append(sample)
   print(round_no,name,engine,'cpu',round(statistics.median(s['cpu_ms']for s in samples),2),'rss',samples[-1]['after_mem_kib']['VmRSS'],flush=True)
  finally:p.terminate();p.wait(timeout=5)
 return {'fixture':name,'round':round_no,'engine':engine,'command':cmd,'initial':initial,'idle':idle,'samples':samples,'log':log.read_text()}
state={'utc_start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'working_tree':'uncommitted','before_sha256':sha(HERE/'before.json.gz'),'binaries':{k:sha(v)for k,v in BINS.items()},'cpu':CPU,'stack_limit':resource.getrlimit(resource.RLIMIT_STACK)[0],'env':{k:ENV[k]for k in ('RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','TZ','NO_COLOR','LC_ALL')},'fixtures':{p.name:{'text':p.read_text(),'sha256':sha(p)}for p in F.iterdir()},'script_sha256':sha(__file__),'common_sha256':sha(HERE/'common.py'),'runs':[]}
with tempfile.TemporaryDirectory(prefix='tekt-watch-context-')as temp:
 for r in range(3):
  for n in NAMES:
   for e in (['tekt','vanilla']if r%2==0 else['vanilla','tekt']):state['runs'].append(run_case(e,n,r,Path(temp)))
state['comparison']=[]
for r in range(3):
 for n in NAMES:
  pair={a['engine']:a for a in state['runs']if a['round']==r and a['fixture']==n}
  for a,b in zip(pair['tekt']['samples'],pair['vanilla']['samples']):
   assert a['source_sha256']==b['source_sha256'];wa,wb=a['words'],b['words'];same=[w['text']for w in wa]==[w['text']for w in wb];delta=max((abs(i[k]-j[k])for i,j in zip(wa,wb)for k in ('xMin','xMax','yMin','yMax')),default=0)if same else None
   state['comparison'].append({'round':r,'fixture':n,'cycle':a['cycle'],'kind':a['kind'],'same_text':a['text']==b['text'],'same_words':same,'bbox_max_delta_pt':delta})
state['summary']={}
for n in NAMES:
 state['summary'][n]={}
 for k in ['metadata_touch','same_bytes','small','structural']:
  state['summary'][n][k]={}
  for e in BINS:
   rows=[s for r in state['runs']if r['fixture']==n and r['engine']==e for s in r['samples']if s['kind']==k];lat=[s['latency_ms']for s in rows if s['published']]
   state['summary'][n][k][e]={'cpu_median_ms':statistics.median(s['cpu_ms']for s in rows),'latency_median_ms':statistics.median(lat)if lat else None,'rss_median_kib':statistics.median(s['after_mem_kib']['VmRSS']for s in rows),'published':sum(s['published']for s in rows),'samples':len(rows)}
state['utc_end']=now();assert state['binaries']=={k:sha(v)for k,v in BINS.items()};assert state['script_sha256']==sha(__file__);assert state['common_sha256']==sha(HERE/'common.py')
with gzip.open(HERE/'measurements.json.gz','wt')as f:json.dump(state,f)
print(json.dumps(state['summary'],indent=2),flush=True)
