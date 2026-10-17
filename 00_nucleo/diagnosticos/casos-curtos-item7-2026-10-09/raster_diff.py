"""Quantify 36-dpi raster differences for the qualified item-7 documents."""
import datetime,gzip,hashlib,json,os,subprocess,tempfile
from pathlib import Path
from PIL import Image,ImageChops
D=Path(__file__).resolve().parent;R=D.parents[2];F=D/'fixtures';FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
B={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):ENV.pop(k)
ENV.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R)
stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R)
(D/'raster-git-status.txt').write_bytes(status)
(D/'raster-git-diff-stat.txt').write_bytes(stat)
state={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),
       'working_tree':'uncommitted','git_status_sha256':hashlib.sha256(status).hexdigest(),
       'git_diff_stat_sha256':hashlib.sha256(stat).hexdigest(),
       'binaries':{e:sha(p) for e,p in B.items()},'fixtures':{p.name:sha(p) for p in F.glob('*.typ')},'cases':{}}
with tempfile.TemporaryDirectory(prefix='item7-raster-diff-') as d:
 out=Path(d)
 for fixture in sorted(F.glob('*.typ')):
  images={}
  for e,bin in B.items():
   pdf=out/(fixture.stem+'-'+e+'.pdf')
   cmd=[str(bin),'compile',str(fixture),str(pdf),'--root',str(F),'--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]+(['--jobs','1'] if e=='vanilla' else [])
   p=subprocess.run(cmd,cwd=R,env=ENV,capture_output=True,text=True,timeout=120);assert p.returncode==0,(cmd,p.stderr)
   prefix=out/(fixture.stem+'-'+e)
   subprocess.run(['pdftoppm','-r','36','-png',str(pdf),str(prefix)],check=True,capture_output=True)
   images[e]=sorted(out.glob(prefix.name+'-*.png'))
  assert len(images['tekt'])==len(images['vanilla'])
  pixels=0;different=0;sum_abs=0;max_abs=0
  for a,b in zip(images['tekt'],images['vanilla']):
   aa=Image.open(a).convert('RGB');bb=Image.open(b).convert('RGB');assert aa.size==bb.size
   data_a=aa.tobytes();data_b=bb.tobytes();pixels+=aa.width*aa.height
   for i in range(0,len(data_a),3):
    delta=[abs(data_a[i+j]-data_b[i+j]) for j in range(3)]
    if any(delta):different+=1
    sum_abs+=sum(delta)
    max_abs=max(max_abs,*delta)
  state['cases'][fixture.stem]={'pages':len(images['tekt']),'pixels':pixels,'different_pixels':different,'different_fraction':different/pixels,'mean_channel_abs_diff':sum_abs/(pixels*3),'max_channel_abs_diff':max_abs}
  print(fixture.stem,state['cases'][fixture.stem],flush=True)
state['utc_end']=utc()
assert state['binaries']=={e:sha(p) for e,p in B.items()}
assert state['fixtures']=={p.name:sha(p) for p in F.glob('*.typ')}
(D/'raster-diff.json').write_text(json.dumps(state,indent=2)+'\n')
