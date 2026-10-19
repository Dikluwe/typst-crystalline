from common import *
import tempfile,random,time,statistics
x=state(__file__);x['runs']=[];x['load_start']=os.getloadavg()
with tempfile.TemporaryDirectory(prefix='cpu-native-')as temp:
 T=Path(temp)
 for rnd in range(3):
  cases=F[:];random.Random(20261025+rnd).shuffle(cases)
  for f in cases:
   for pair in range(-1,6):
    engines=list(B);random.Random(20261026+rnd*100+pair+len(f.stem)).shuffle(engines)
    for e in engines:
     usage=T/'usage';pdf=T/'out.pdf';cmd=['/usr/bin/time','-f','%M %U %S','-o',str(usage)]+command(e,f,pdf);start=time.perf_counter_ns();p=run(cmd);wall=(time.perf_counter_ns()-start)/1e6;rss,user,system=usage.read_text().split()
     if pair>=0:x['runs'].append({'round':rnd,'pair':pair,'engine':e,'fixture':f.stem,'utc':utc(),'command':cmd,'stderr':p.stderr,'wall_ms':wall,'user_s':float(user),'system_s':float(system),'rss_kib':int(rss),'pdf_sha256':sha(pdf)})
   print(rnd,f.stem,'done',flush=True)
x['summary']={}
for f in F:
 c={}
 for e in B:
  rs=[r for r in x['runs']if r['fixture']==f.stem and r['engine']==e];c[e]={'wall_median_ms':statistics.median(r['wall_ms']for r in rs),'cpu_median_ms':1000*statistics.median(r['user_s']+r['system_s']for r in rs),'user_median_ms':1000*statistics.median(r['user_s']for r in rs),'system_median_ms':1000*statistics.median(r['system_s']for r in rs),'rss_median_kib':statistics.median(r['rss_kib']for r in rs),'wall_round_median_ms':[statistics.median(r['wall_ms']for r in rs if r['round']==rnd)for rnd in range(3)]}
 ratios=[]
 for rnd in range(3):
  for pair in range(6):
   rs={r['engine']:r for r in x['runs']if r['fixture']==f.stem and r['round']==rnd and r['pair']==pair};ratios.append(rs['tekt']['wall_ms']/rs['orig']['wall_ms'])
 c['paired_wall_ratio_median']=statistics.median(ratios);x['summary'][f.stem]=c
x['load_end']=os.getloadavg();save('native-benchmark.json.gz',x)
for n,s in x['summary'].items():print(n,s,flush=True)
