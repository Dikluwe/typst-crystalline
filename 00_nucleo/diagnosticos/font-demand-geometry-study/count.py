from pathlib import Path
import subprocess,json,hashlib,datetime,os
ROOT=Path(__file__).resolve().parents[3];R=Path(__file__).resolve().parent;C=R/'catalog-counts';exe=ROOT/'target/debug/infra-font-demand-counts'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
rows=[]
for n in range(3):
 for case in ['image-default','shape-default','placed_shape-default','image_then_text-default']:
  source=R/'after/inputs'/(case+'.typ');out=C/f'{case}-r{n}.pdf';cmd=[str(exe),str(source),str(ROOT/'00_nucleo/diagnosticos/infra-performance-followup/fonts'),str(out)];env=os.environ.copy();env['RAYON_NUM_THREADS']='1'
  for key in list(env):
   if key.startswith(('TYPST_','CRYSTALLINE_')):env.pop(key)
  result=subprocess.run(cmd,cwd=ROOT,env=env,text=True,capture_output=True,check=True);(C/f'{case}-r{n}.stderr').write_text(result.stderr);rows.append(json.loads(result.stdout)|{'round':n,'command':cmd,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'input_sha256':sha(source),'output_sha256':sha(out)})
(C/'results.json').write_text(json.dumps(rows,indent=2)+'\n');profiles=[json.loads(line) for line in (C/'build.jsonl').read_text().splitlines() if line.startswith('{')];artifact=[x for x in profiles if x.get('executable')==str(exe)][-1];(C/'artifact.json').write_text(json.dumps({'sha256':sha(exe),'cargo_artifact':artifact,'functional_counts_only':True},indent=2)+'\n')
print([(r['input'].split('/')[-1],r['round'],r['book_calls']) for r in rows])
