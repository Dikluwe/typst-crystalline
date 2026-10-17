from common import *
import tempfile,random,time,statistics
x=state(__file__);x['runs']=[];selected=[f for f in F if f.stem in ['plain','inline-set-1600','variants-1600']]
with tempfile.TemporaryDirectory(prefix='cpu-overhead-')as temp:
 T=Path(temp)
 for rnd in range(3):
  for f in selected:
   for e in B:
    for pair in range(3):
     modes=['native','sampled'];random.Random(20261027+rnd+pair).shuffle(modes)
     for mode in modes:
      pdf=T/'out.pdf';cmd=command(e,f,pdf);env=ENV.copy()
      if mode=='sampled':env.update(LD_PRELOAD=str(D/'cpu_probe.so'),CPU_PROFILE_OUT=str(T/'profile'))
      start=time.perf_counter_ns();p=run(cmd,env);wall=(time.perf_counter_ns()-start)/1e6;x['runs'].append({'round':rnd,'pair':pair,'engine':e,'fixture':f.stem,'mode':mode,'utc':utc(),'command':cmd,'wall_ms':wall,'stderr':p.stderr,'samples':parse_profile((T/'profile').read_text())['header']if mode=='sampled'else None})
x['summary']={}
for f in selected:
 for e in B:
  ratios=[]
  for rnd in range(3):
   for pair in range(3):
    rs={r['mode']:r for r in x['runs']if r['fixture']==f.stem and r['engine']==e and r['round']==rnd and r['pair']==pair};ratios.append(rs['sampled']['wall_ms']/rs['native']['wall_ms']-1)
  x['summary'][f.stem+'-'+e]={'wall_delta_paired_median':statistics.median(ratios),'round_medians':[statistics.median(ratios[r*3:r*3+3])for r in range(3)]}
save('overhead.json.gz',x);print(x['summary'])
