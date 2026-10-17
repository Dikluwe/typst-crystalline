from pathlib import Path
import subprocess,json,hashlib,datetime
root=Path.cwd();out=root/'00_nucleo/diagnosticos/vanilla-resync-2026-10-17'
results=[]
for profile in ['default','html','a11y-extras','html+a11y','bundle','html+bundle']:
 path=out/('vanilla-catalog-'+profile.replace('+','-')+'.json')
 cmd=[str(root/'lab/typst-original/target/release/p1140-inventory'),str(path),profile,str(root/'lab/typst-original/target/release/typst')]
 r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,r.stderr
 data=json.loads(path.read_text());assert data['vanilla_revision']=='bd5ebec2';assert data['product_sha256']==json.loads((out/'oracle-build.json').read_text())['binary_sha256']
 results.append({'profile':profile,'command':cmd,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'revision':data['vanilla_revision'],'route_count':data['route_count'],'catalog_sha256':data['catalog_sha256'],'output_sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
(out/'catalog-summary.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps(results,indent=2))
