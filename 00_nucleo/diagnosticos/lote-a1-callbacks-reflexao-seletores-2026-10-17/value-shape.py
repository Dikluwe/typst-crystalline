from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,sys
p=Path(__file__).resolve().parent;root=p.parents[2];phase=sys.argv[1]
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for b in iter(lambda:stream.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
source=p/'outputs'/'value-shape.rs';source.write_text('''use typst_core::entities::value::{Value, IntrospectedContent};
fn main() { println!("{} {} {} {}", std::mem::size_of::<Value>(), std::mem::align_of::<Value>(), std::mem::size_of::<IntrospectedContent>(), std::mem::align_of::<IntrospectedContent>()); }
''')
rlib=max((root/'target/debug/deps').glob('libtypst_core-*.rlib'),key=lambda f:f.stat().st_mtime_ns);binary=p/'outputs'/('value-shape-'+phase)
cmd=['rustc',str(source),'--edition=2021','-L','dependency='+str(root/'target/debug/deps'),'--extern','typst_core='+str(rlib),'-o',str(binary)]
r=subprocess.run(cmd,text=True,capture_output=True);assert r.returncode==0,r.stderr
output=subprocess.check_output([str(binary)],text=True).strip();fields=['value','value_align','snapshot','snapshot_align'];sizes=dict(zip(fields,map(int,output.split())))
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'phase':phase,'command':cmd,'rlib_sha256':sha(rlib),'source_sha256':sha(source),'value_source_sha256':sha(root/'01_core/src/entities/value.rs'),'binary_sha256':sha(binary),'sizes':sizes,'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True)}
(p/('value-shape-'+phase+'.json.gz')).write_bytes(gzip.compress(json.dumps(record,indent=2).encode()));print(sizes)
if phase=='after':
 before=json.loads(gzip.decompress((p/'value-shape-before.json.gz').read_bytes()));assert sizes==before['sizes'],(before['sizes'],sizes)
