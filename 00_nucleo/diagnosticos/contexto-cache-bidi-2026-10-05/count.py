import argparse,subprocess,json,gzip,re,datetime,os,hashlib
from pathlib import Path
H=Path(__file__).resolve().parent;T=Path('/tmp/tekt-context-bidi');R=H.parents[2]
p=argparse.ArgumentParser();p.add_argument('engine');a=p.parse_args();binary=T/'baseline-typst' if a.engine=='baseline' else R/'target/release/typst'
symbols=subprocess.check_output(['nm',str(binary)],text=True).splitlines()
def symbol(*need):
 names=[l.split()[-1] for l in symbols if all(n in l for n in need)]
 assert len(names)==1,names
 return names[0]
breaks=[('queries',symbol('font_metrics','cached_shaped_width')),('misses',symbol('font_metrics','fallback','advance_shaped','closure')),('layouts',symbol('paged53layout_with_realized_math_callbacks_from_with_metrics')),('metrics_new',symbol('font_metrics','FallbackFontMetrics3new'))]
script=T/(a.engine+'-counts.gdb');text='set pagination off\nset confirm off\nset $bidi=0\nset $bidi_queries=0\nset $bidi_misses=0\nset $recon=0\nset $recon_queries=0\nset $recon_misses=0\n'
for i,(key,sym) in enumerate(breaks,1):
 text+=f'set ${key}=0\nbreak {sym}\ncommands {i}\nsilent\nset ${key}=${key}+1\n'
 if key in ['queries','misses']:text+=f'if $bidi\nset $bidi_{key}=$bidi_{key}+1\nend\nif $recon\nset $recon_{key}=$recon_{key}+1\nend\n'
 text+='continue\nend\n'
text+=f'break {symbol("reconciliation","fix_line_positions_with_metrics")}\ncommands 5\nsilent\nset $bidi=0\nset $recon=1\ncontinue\nend\n'
text+=f'break {symbol("layout_bidi","reorder_bidi_document")}\ncommands 6\nsilent\nset $bidi=1\ncontinue\nend\n'
text+=f'break {symbol("shaper","shape_document")}\ncommands 7\nsilent\nset $bidi=0\ncontinue\nend\n'
text+='run\nprintf "\\nCOUNTS queries=%d misses=%d layouts=%d metrics_new=%d recon_queries=%d recon_misses=%d bidi_queries=%d bidi_misses=%d\\n",$queries,$misses,$layouts,$metrics_new,$recon_queries,$recon_misses,$bidi_queries,$bidi_misses\n';script.write_text(text)
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'script':text,'cases':{}}
for n,source in json.loads((H/'fixtures.json').read_text()).items():
 cmd=['gdb','--batch','-x',str(script),'--args',str(binary),'compile',str(T/(n+'.typ')),str(T/(n+'-'+a.engine+'.pdf')),'--pdf-tagged=false','--pretty=false'];r=subprocess.run(cmd,capture_output=True,text=True,env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1'),timeout=90);match=re.search(r'COUNTS queries=(\d+) misses=(\d+) layouts=(\d+) metrics_new=(\d+) recon_queries=(\d+) recon_misses=(\d+) bidi_queries=(\d+) bidi_misses=(\d+)',r.stdout);d['cases'][n]={'source':source,'command':cmd,'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr,'counts':dict(zip([k for k,_ in breaks]+["recon_queries","recon_misses","bidi_queries","bidi_misses"],map(int,match.groups()))) if match else None};print(n,d['cases'][n]['counts'],r.stderr[-120:],flush=True)
gzip.open(H/('count-'+a.engine+'.json.gz'),'wt').write(json.dumps(d,ensure_ascii=False))
assert all(x['returncode']==0 and x['counts'] and x['counts']['layouts'] for x in d['cases'].values())
