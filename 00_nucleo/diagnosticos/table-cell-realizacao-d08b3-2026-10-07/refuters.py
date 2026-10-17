from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys,os,shutil
R=Path.cwd();D=R/'00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07';W=Path('/tmp/d08b3-refuters-final');W.mkdir(exist_ok=True)
B={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for k,p in list(B.items()):
 frozen=W/(k+'-'+sha(p))
 if not frozen.exists():shutil.copyfile(p,frozen);frozen.chmod(0o755)
 B[k]=frozen
files=subprocess.check_output(['git','ls-files','--modified'],text=True).splitlines()+subprocess.check_output(['git','ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'],text=True).splitlines()
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--short'],text=True),'source_hashes':{p:sha(R/p) for p in files if (R/p).is_file()},'bin_sha256':{k:sha(p) for k,p in B.items()},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','environment_overrides':{'NO_COLOR':'1','SOURCE_DATE_EPOCH':'0','RAYON_NUM_THREADS':'1'},'cases':{}}
for name in ['additional','order','aliases','placement-refuters-red']:
 p=D/(name+('.json' if name.endswith('red') else '.json.gz'));raw=p.read_bytes();before=json.loads(raw if p.suffix=='.json' else gzip.decompress(raw))
 for case,row in before['cases'].items():
  (W/'main.typ').write_text(row['source']);result={'source':row['source'],'cwd':str(W),'runs':{}}
  for key,bin in B.items():
   (W/'out.pdf').unlink(missing_ok=True);(W/'out.html').unlink(missing_ok=True)
   cmd=[str(bin),*row['runs'][key]['command'][1:]]
   r=subprocess.run(cmd,cwd=W,capture_output=True,text=True,timeout=45,env={**os.environ,**d['environment_overrides']})
   v={'command':cmd,'status':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
   if (W/'out.pdf').exists():v['pdf_text']=subprocess.check_output(['pdftotext',str(W/'out.pdf'),'-'],text=True)
   if (W/'out.html').exists():v['html']=(W/'out.html').read_text()
   result['runs'][key]=v
  d['cases'][name+'/'+case]=result;print(name+'/'+case,{k:(v['status'],v['stdout'],v.get('pdf_text')) for k,v in result['runs'].items()},flush=True)
d['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(D/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(d,ensure_ascii=False,indent=2).encode()))
