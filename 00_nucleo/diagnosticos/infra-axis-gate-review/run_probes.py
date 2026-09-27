"""Correctness probes through the existing CLI; never interpret debug durations as performance."""
from pathlib import Path
import os,subprocess,hashlib,json,datetime
ROOT=Path(__file__).resolve().parents[3];R=Path(__file__).resolve().parent
exe=ROOT/'target/debug/typst';font=ROOT/'03_infra/fixtures/fonts/UbuntuSans-Variable.ttf'
missing=str(R/'absent-python');assert not Path(missing).exists()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
source=sorted(p for p in (ROOT/'03_infra/src').rglob('*.rs'))
before={str(p.relative_to(ROOT)):sha(p) for p in source}
base=(R/'control.typ').read_text();(R/'matching.typ').write_text(base.replace('size: 12pt','size: 12pt, variations: (wght: 700)'))
(R/'bundle-unknown.typ').write_text('#document("one.pdf")[\n'+(R/'unknown.typ').read_text()+'\n]\n')
runs=[]
for name in ['control','unknown','matching']:
 for fmt in ['svg','pdf']:
  cmd=[str(exe),'compile','--font-path',str(font),str(R/(name+'.typ')),str(R/(name+'-green.'+fmt))]
  result=subprocess.run(cmd,cwd=ROOT,env=os.environ|{'TYPST_CRYSTALLINE_PYTHON':missing},capture_output=True,text=True,timeout=60)
  (R/(name+'-'+fmt+'-green.stderr')).write_text(result.stderr)
  expected=1 if name=='matching' else 0;assert result.returncode==expected,(cmd,result.stderr)
  runs.append({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd,'environment':{'TYPST_CRYSTALLINE_PYTHON':missing},'exit_code':result.returncode})
cmd=[str(exe),'compile','--features','bundle','--format','bundle','--font-path',str(font),str(R/'bundle-unknown.typ'),str(R/'bundle-output')]
result=subprocess.run(cmd,cwd=ROOT,env=os.environ|{'TYPST_CRYSTALLINE_PYTHON':missing},capture_output=True,text=True,timeout=60);(R/'bundle-green.stderr').write_text(result.stderr);assert result.returncode==0,result.stderr
runs.append({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd,'environment':{'TYPST_CRYSTALLINE_PYTHON':missing},'exit_code':result.returncode})
after={str(p.relative_to(ROOT)):sha(p) for p in source};assert before==after
(R/'probe-green.json').write_text(json.dumps({'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'executable_sha256':sha(exe),'font_sha256':sha(font),'source_before':before,'source_after':after,'inputs':{p.name:sha(p) for p in R.glob('*.typ')},'runs':runs},indent=2)+'\n')
print('SVG/PDF controls and unknown axes pass; matching axes fail closed; bundle unknown axes pass.')
