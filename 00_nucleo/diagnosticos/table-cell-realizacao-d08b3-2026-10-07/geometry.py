from pathlib import Path
import json,gzip,subprocess,hashlib,datetime,xml.etree.ElementTree as ET,os,sys
R=Path.cwd();D=R/'00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07';W=Path('/tmp/d08b3-geometry-'+sys.argv[1]);W.mkdir(exist_ok=True)
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'build_snapshot':'tested-fifth.json.gz','diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--short'],text=True),'cases':{}}
main_name=sys.argv[2] if len(sys.argv)>2 else 'after-fifth'; extra_name=sys.argv[3] if len(sys.argv)>3 else 'refuters-fifth'
main=json.loads(gzip.decompress((D/(main_name+'.json.gz')).read_bytes()));extra=json.loads(gzip.decompress((D/(extra_name+'.json.gz')).read_bytes()));d['build_snapshot']='tested-'+main_name.removeprefix('after-')+'.json.gz'
for name,row in [(n,main['cases'][n]) for n in ['nested','show-cell-filter-position','show-cell-no-default-escape']]+[(n,extra['cases']['aliases/'+n]) for n in ['local-cell-alias','local-over-outer-cell']]:
 result={'source':row['source'],'runs':{}}
 for key,old in row['runs'].items():
  p=Path(old['command'][0]);cw=W/name/key;cw.mkdir(parents=True,exist_ok=True);(cw/'main.typ').write_text(row['source']);cmd=[str(p),'compile','main.typ','out.pdf'];r=subprocess.run(cmd,cwd=cw,text=True,capture_output=True,env={**os.environ,'NO_COLOR':'1','SOURCE_DATE_EPOCH':'0','RAYON_NUM_THREADS':'1'},timeout=45)
  box=subprocess.check_output(['pdftotext','-bbox',str(cw/'out.pdf'),'-'],text=True)
  words=[{'text':e.text,'box':e.attrib} for e in ET.fromstring(box).iter() if e.tag.endswith('word')]
  result['runs'][key]={'command':cmd,'cwd':str(cw),'bin_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'pdf_sha256':hashlib.sha256((cw/'out.pdf').read_bytes()).hexdigest(),'status':r.returncode,'stderr':r.stderr,'stdout':r.stdout,'words':words};print(name,key,words,flush=True)
 d['cases'][name]=result
d['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(D/(sys.argv[1]+'.json')).write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n')
