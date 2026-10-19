from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,struct,collections
R=Path.cwd();D=R/'00_nucleo/diagnosticos/catalogo-seletivo-preparacao-2026-10-07';sha=lambda b:hashlib.sha256(b).hexdigest();now=datetime.datetime.now(datetime.timezone.utc).isoformat()
files=subprocess.check_output(['git','ls-files','--modified'],text=True).splitlines()+subprocess.check_output(['git','ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'],text=True).splitlines()
state={'utc':now,'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'status':subprocess.check_output(['git','status','--short'],text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'sha256':{p:sha((R/p).read_bytes()) for p in files if (R/p).is_file()},'files':{p:(R/p).read_text() for p in files if p.endswith(('.rs','.md','.toml'))}}
(D/'before.json.gz').write_bytes(gzip.compress(json.dumps(state,ensure_ascii=False).encode()))
old=json.load(gzip.open(R/'00_nucleo/diagnosticos/catalogo-seletivo-2026-10-05/measurement.json.gz'))
inputs=json.loads((R/'00_nucleo/diagnosticos/coverage-ranges-2026-10-07/probe-results/validate.json').read_text())['faces'];assets={};rows=[]
for face in inputs:
 p=Path(face['path']);index=face['index'];assets.setdefault(str(p),sha(p.read_bytes()))
 with p.open('rb') as f:
  size=p.stat().st_size;b=f.read(12);offset=0
  if b[:4]==b'ttcf':f.seek(12+4*index);offset=struct.unpack('>I',f.read(4))[0]
  f.seek(offset);h=f.read(12);n=struct.unpack('>H',h[4:6])[0];directory=f.read(n*16);total=12+n*16;requested=total;valid=True
  for i in range(n):
   tag,_,off,length=struct.unpack('>4sIII',directory[i*16:i*16+16])
   if tag not in [b'head',b'hhea',b'maxp',b'name',b'OS/2',b'post',b'cmap']:continue
   if off+length>size:valid=False;break
   take=length
   if tag==b'post':
    f.seek(off);prefix=f.read(min(length,34));requested+=len(prefix)
    if length>=34 and struct.unpack('>I',prefix[:4])[0]==0x20000:take=min(length,34+2*struct.unpack('>H',prefix[32:34])[0])
    else:take=min(length,32)
   total+=take;requested+=take
  rows.append({'path':str(p),'index':index,'kind':face['kind'],'tables':n,'projected_bytes':total,'requested_bytes_excluding_ttc_index':requested,'valid_wanted_bounds':valid,'eligible_caps':valid and n<=4096 and total<=8*1024*1024 and requested<=16*1024*1024})
oldhash=old.get('fonts',{});unchanged=all(assets.get(p)==h for p,h in oldhash.items());audit={'utc':now,'head':state['head'],'snapshot':'before.json.gz','release_sha256':sha((R/'target/release/typst').read_bytes()),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','historical_font_assets_still_identical':unchanged,'historical_asset_count':len(oldhash),'font_sha256':assets,'faces':rows,'counts':dict(collections.Counter(r['kind'] for r in rows)),'max_tables':max(r['tables'] for r in rows),'max_projected_bytes':max(r['projected_bytes'] for r in rows),'max_requested_bytes':max(r['requested_bytes_excluding_ttc_index'] for r in rows),'caps_eligible_faces':sum(r['eligible_caps'] for r in rows),'owners_before':{p:sha((R/p).read_bytes()) for p in ['03_infra/src/fontdb.rs','00_nucleo/prompts/infra/fontdb.md','03_infra/src/fonts.rs','00_nucleo/prompts/infra/fonts.md']}}
(D/'audit.json').write_text(json.dumps(audit,indent=2,ensure_ascii=False)+'\n');print({k:v for k,v in audit.items() if k not in ['font_sha256','faces','owners_before']})
