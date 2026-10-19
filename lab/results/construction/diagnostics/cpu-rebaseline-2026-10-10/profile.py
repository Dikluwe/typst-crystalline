from common import *
import tempfile,random,collections
x=state(__file__);x['sampler_source']={'text':(D/'cpu_probe.c.txt').read_text(),'sha256':sha(D/'cpu_probe.c.txt')};x['sampler_sha256']=sha(D/'cpu_probe.so');x['runs']=[]
x['scope']='ITIMER_PROF process CPU, interrupted user PC plus inclusive backtrace. Constructor-start, 64 frames, 8192 samples. backtrace in handler is diagnostic, not async-signal-safe; process timeout and samples/caps checked. Instrumented duration is not benchmark. CPU kernel ticks may deliver at user return; no exclusive kernel/I/O attribution.'
with tempfile.TemporaryDirectory(prefix='cpu-rebaseline-')as temp:
 T=Path(temp)
 for rep in range(24):
  cases=F[:];random.Random(20261020+rep).shuffle(cases)
  for f in cases:
   engines=list(B);random.Random(20261021+rep+len(f.stem)).shuffle(engines)
   for e in engines:
    path=T/(e+'-'+f.stem+'.profile');pdf=T/(e+'-'+f.stem+'.pdf');env=ENV.copy();env.update(LD_PRELOAD=str(D/'cpu_probe.so'),CPU_PROFILE_OUT=str(path),CPU_PROFILE_INTERVAL_US='1000');cmd=command(e,f,pdf);p=run(cmd,env);raw=path.read_text();data=parse_profile(raw);assert data['samples'],(cmd,data['header'])
    x['runs'].append({'engine':e,'fixture':f.stem,'rep':rep,'utc':utc(),'command':cmd,'stderr':p.stderr,'pdf_sha256':sha(pdf),'raw':raw,**data})
  print('rep',rep,'done',flush=True)
# Symbolize separately per mapped module, recording ELF identity and address recipe.
modules=collections.defaultdict(set)
for r in x['runs']:
 for s in r['samples']:
  for idx,addr in s['addresses']:
   if idx>=0:
    m=r['modules'][idx];modules[m['path']].add(addr)
x['symbols']={};x['modules']={}
for path,addresses in modules.items():
 p=Path(path);info={'path':path,'addresses':sorted(addresses)}
 if not p.is_file():x['modules'][path]=info;continue
 info['sha256']=sha(p);elf=subprocess.check_output(['readelf','-h',str(p)],text=True);info['elf_header']=elf;is_exec='EXEC ('in elf
 base=next(m['base']for r in x['runs']for m in r['modules'].values()if m['path']==path);offset=base if is_exec else 0
 pcs=sorted(addresses);cmd=['eu-addr2line','-f','-C','-a','-e',path];proc=subprocess.run(cmd,input=''.join(hex(a+offset)+'\n'for a in pcs),capture_output=True,text=True,check=True);lines=proc.stdout.splitlines();assert len(lines)==len(pcs)*3,(path,len(lines),len(pcs));x['symbols'][path]={format(a,'x'):{'function':lines[i*3+1],'location':lines[i*3+2]}for i,a in enumerate(pcs)};info['symbolize_command']=cmd;info['address_adjustment']=offset;x['modules'][path]=info
assert sha(D/'cpu_probe.so')==x['sampler_sha256'];save('profiles.json.gz',x);print('runs',len(x['runs']),'samples',sum(len(r['samples'])for r in x['runs']),flush=True)
