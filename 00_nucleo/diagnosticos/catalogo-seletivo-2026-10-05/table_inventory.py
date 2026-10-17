import collections,datetime,gzip,hashlib,json,struct,subprocess
from pathlib import Path
H=Path(__file__).resolve().parent;d=json.load(gzip.open(H/'measurement.json.gz'));s=collections.Counter()
for p,h in d['fonts'].items():
 assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h
 with open(p,'rb') as f:
  b=f.read(12);offs=[0]
  if b[:4]==b'ttcf':n=struct.unpack('>I',b[8:12])[0];offs=struct.unpack('>'+str(n)+'I',f.read(4*n))
  for off in offs:
   f.seek(off);b=f.read(12);n=struct.unpack('>H',b[4:6])[0];b=f.read(16*n)
   for i in range(n):tag,_,_,length=struct.unpack('>4sIII',b[i*16:i*16+16]);s[tag.decode('ascii','replace')]+=length
r={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':d['head'],'status':subprocess.check_output(['git','status','--short'],text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'font_sha256':d['fonts'],'table_lengths_all_collection_faces':dict(s),'script':Path(__file__).read_text()}
with gzip.open(H/'table-inventory.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
print({k:s[k] for k in ['post','name','cmap']})
