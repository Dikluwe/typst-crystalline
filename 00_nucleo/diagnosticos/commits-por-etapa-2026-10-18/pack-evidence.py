from pathlib import Path
import os,subprocess,json,gzip,hashlib,base64,datetime,re
root=Path.cwd();out=Path(__file__).resolve().parent
visible=set(subprocess.check_output(['git','ls-files','-z'],text=True).split('\0'))|set(subprocess.check_output(['git','ls-files','--others','--exclude-standard','-z'],text=True).split('\0'))
plan=json.loads(gzip.decompress((out/'commit-plan.json.gz').read_bytes()))
folders=['paridade-novo-vanilla-2026-10-17']+[s['diagnostic'] for s in plan['stages']]+['textual-realization-2026-10-18','show-returns-2026-10-18','inventario-organizacao-2026-10-18','limpeza-caches-2026-10-18']
skip={'bin','retencao','target','rustdoc','static.files','__pycache__','.git','smoke','scratch','checkpoint'}
allowed={'.json','.log','.typ','.bib','.csl','.toml','.yaml','.yml','.csv','.tsv','.txt','.html','.svg','.xml','.patch','.diff','.sha256'}
receipts=[]
for folder in folders:
 base=root/'00_nucleo/diagnosticos'/folder;packed=[];omitted=[]
 for here,dirs,files in os.walk(base,followlinks=False):
  dirs[:]=[d for d in dirs if d not in skip and not (Path(here)/d).is_symlink()]
  for name in sorted(files):
   path=Path(here)/name;relative=path.relative_to(root).as_posix()
   if path.is_symlink() or relative in visible or name.startswith('evidencias-commit'):continue
   eligible=path.suffix in allowed or name.endswith(('.patch.gz','.tsv.gz','.txt.gz','.log.gz'))
   if not eligible:continue
   assert path.stat().st_size<=64*1024**2,('oversized evidence requires explicit classification',relative)
   packed.append(path)
 dest=base/'evidencias-commit.json.gz';index=[];total=0
 with gzip.open(dest,'wt',encoding='utf-8',compresslevel=6) as output:
  output.write('{"format":"base64-files-v1","scope":'+json.dumps(str(base.relative_to(root)))+',"files":[')
  for number,path in enumerate(sorted(packed)):
   data=path.read_bytes();digest=hashlib.sha256(data).hexdigest();total+=len(data)
   # Detect credential material without ever printing the matched text.
   for label,pattern in [('private key',rb'-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----'),('GitHub token',rb'\bgh[pousr]_[A-Za-z0-9]{36,}\b')]:
    assert re.search(pattern,data) is None,('credential-like material',label,str(path))
   relative=path.relative_to(root).as_posix();mode=0o755 if path.stat().st_mode&0o111 else 0o644
   row=dict(path=relative,sha256=digest,bytes=len(data),mode=mode)
   index.append(row)
   if number:output.write(',')
   json.dump(dict(row,base64=base64.b64encode(data).decode()),output,ensure_ascii=False,separators=(',',':'))
  output.write(']}')
 digest=hashlib.sha256(dest.read_bytes()).hexdigest()
 record=dict(folder=folder,path=str(dest.relative_to(root)),sha256=digest,files=len(index),uncompressed_bytes=total,compressed_bytes=dest.stat().st_size,entries=index,policy='Ignored textual metadata, logs, patches and source fixtures; large rendered outputs, font assets and executables remain local with their original manifests.')
 (base/'evidencias-commit-indice.json.gz').write_bytes(gzip.compress(json.dumps(record,ensure_ascii=False,indent=2).encode()))
 receipts.append(record)
 print(folder,len(index),'files',round(dest.stat().st_size/1024**2,2),'MiB',flush=True)
(out/'evidence-packages.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),packages=receipts),ensure_ascii=False,indent=2).encode()))
