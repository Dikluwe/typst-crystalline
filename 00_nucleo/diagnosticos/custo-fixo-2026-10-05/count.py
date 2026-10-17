"""Release catalog/metadata/cmap entry counts, outside time samples."""
import datetime,gzip,hashlib,json,os,subprocess
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];O=Path('/tmp/tekt-fixed-cost-2026-10-05');exe=O/'typst-baseline'
raw=[x.split()[-1] for x in subprocess.check_output(['nm',str(exe)],text=True).splitlines() if len(x.split())==3]
names=subprocess.check_output(['c++filt'],input='\n'.join(raw),text=True).splitlines()
labels={'catalog':'typst_infra::world::fonts::<impl typst_infra::world::SystemWorld>::with_fonts_and_system::{{closure}}::','system':'typst_infra::fontdb::load_system_fonts::','embedded':'typst_infra::embedded_fonts::load_embedded_fonts::','metadata':'typst_infra::fonts::font_info_from_bytes::','coverage':'typst_infra::fonts::extract_coverage::'}
script='set pagination off\nset confirm off\nset print thread-events off\n'
for label,needle in labels.items():
 symbols=[r for r,n in zip(raw,names) if n.startswith(needle)];assert len(symbols)==1,(label,symbols)
 script+=f'set ${label}_calls = 0\nbreak \'{symbols[0]}\'\ncommands\nsilent\nset ${label}_calls = ${label}_calls + 1\ncontinue\nend\n'
script+='run\n'
for label in labels:script+=f'printf "COUNT {label} %d\\n", ${label}_calls\n'
path=O/'fixed-count.gdb';path.write_text(script)
r={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'binary_sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'source_snapshot':'before-state.json.gz','script':script,'cases':[]}
for n in ['empty','hello','shapes','unicode']:
 cmd=['gdb','-q','-nx','--batch','-x',str(path),'--args',str(exe),'compile',str(O/(n+'.typ')),str(O/(n+'-gdb.pdf')),'--pdf-tagged=false','--pretty=false']
 p=subprocess.run(cmd,text=True,capture_output=True,timeout=60,env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1'),preexec_fn=lambda:os.sched_setaffinity(0,{min(os.sched_getaffinity(0))}));assert p.returncode==0,p.stderr
 counts={line.split()[1]:int(line.split()[2]) for line in p.stdout.splitlines() if line.startswith('COUNT ')}
 r['cases'].append({'fixture':n,'command':cmd,'counts':counts,'stdout':p.stdout,'stderr':p.stderr});print(n,counts)
r['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(H/'count.json.gz','wt') as f:json.dump(r,f,indent=2)
