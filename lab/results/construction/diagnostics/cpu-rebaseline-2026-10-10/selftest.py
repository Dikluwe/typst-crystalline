from common import *
import tempfile
x=state(__file__)
source='''__attribute__((noinline)) void busy(unsigned long n){volatile unsigned long x=1;while(n--)x=x*1664525+1013904223;}int main(){busy(100000000);return 0;}'''
with tempfile.TemporaryDirectory(prefix='cpu-probe-self-')as temp:
 T=Path(temp);(T/'main.c').write_text(source);build=['cc','-O2','-fPIE','-pie',str(T/'main.c'),'-o',str(T/'test')];run(build);env=ENV.copy();env.update(LD_PRELOAD=str(D/'cpu_probe.so'),CPU_PROFILE_OUT=str(T/'profile'))
 cmd=[str(T/'test')];run(cmd,env);raw=(T/'profile').read_text();data=parse_profile(raw)
 pcs=[r['addresses'][0][1]for r in data['samples']if data['modules'].get(r['addresses'][0][0],{}).get('path')==str(T/'test')]
 p=subprocess.run(['eu-addr2line','-a','-f','-C','-e',str(T/'test')],input=''.join(hex(a)+'\n'for a in pcs),capture_output=True,text=True,check=True);lines=p.stdout.splitlines();functions=lines[1::3];busy=sum(n=='busy'for n in functions);assert len(data['samples'])>50;assert busy/len(data['samples'])>.9,(busy,len(data['samples']))
 x.update(source=source,build=build,command=cmd,binary_sha256=sha(T/'test'),interposer_sha256=sha(D/'cpu_probe.so'),interposer_source={'text':(D/'cpu_probe.c.txt').read_text(),'sha256':sha(D/'cpu_probe.c.txt')},raw=raw,symbols=functions,passed=True,busy=busy,samples=len(data['samples']))
save('selftest.json.gz',x);print('selftest',busy,len(data['samples']),'passed')
