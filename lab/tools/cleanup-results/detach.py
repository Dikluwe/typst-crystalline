#!/usr/bin/env python3
"""Give historical result paths independent storage before reusing/editing them."""
from pathlib import Path
import argparse,os,shutil,tempfile,hashlib
p=argparse.ArgumentParser(description=__doc__);p.add_argument('paths',type=Path,nargs='+');args=p.parse_args();count=0
for root in args.paths:
 paths=root.rglob('*') if root.is_dir() else [root]
 for path in paths:
  if path.is_symlink() or not path.is_file() or path.stat().st_nlink<=1:continue
  fd,name=tempfile.mkstemp(prefix='.detach-',dir=path.parent);os.close(fd);tmp=Path(name)
  try:
   shutil.copy2(path,tmp)
   with path.open('rb') as a,tmp.open('rb') as b:assert hashlib.file_digest(a,'sha256').hexdigest()==hashlib.file_digest(b,'sha256').hexdigest()
   os.replace(tmp,path);count+=1
  finally:tmp.unlink(missing_ok=True)
print('Independent copies:',count)
