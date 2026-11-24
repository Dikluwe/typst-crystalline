import unittest,tempfile,tarfile,subprocess,json,gzip,hashlib,io,os
from pathlib import Path
import payload_store as p
class StoreTest(unittest.TestCase):
 def test_alias_gzip_independence_and_refusal(self):
  with tempfile.TemporaryDirectory() as temp:
   root=Path(temp);oldroot,oldout=p.ROOT,p.OUT;p.ROOT=root;p.OUT=root/'store'
   try:
    data=b'preserved *source*';h=hashlib.sha256(data).hexdigest();specs={}
    for kind in ['one','two']:
     rows=[];tar=root/(kind+'.tar')
     with tarfile.open(tar,'w') as t:
      for i in range(2):
       name=f'files/{kind}-{i}';item=tarfile.TarInfo(name);item.mode=0o644
       if i:item.type=tarfile.LNKTYPE;item.linkname=f'files/{kind}-0';t.addfile(item)
       else:item.size=len(data);t.addfile(item,io.BytesIO(data))
       rows.append({'path':name,'sha256':h,'mode':0o644,'mtime_ns':1000000000})
     subprocess.run(['zstd','-q',str(tar),'-o',str(root/(kind+'.zst'))],check=True)
     with gzip.open(root/(kind+'.json.gz'),'wt') as f:
      for r in rows:f.write(json.dumps(r)+'\n')
     specs[kind]={'archive':kind+'.zst','index':kind+'.json.gz','hash_field':'sha256','encoding':'gzip' if kind=='two' else 'identity'}
    catalog=p.build(specs,p.OUT);self.assertEqual(catalog['unique_payloads'],1);(p.OUT/'catalog.json').write_text(json.dumps(catalog))
    p.restore('one',['files']);p.restore('two',['files'])
    a=root/'files/one-0';b=root/'files/one-1';self.assertFalse(os.path.samefile(a,b));self.assertEqual(a.read_bytes(),data)
    with gzip.open(root/'files/two-0','rb') as f:self.assertEqual(f.read(),data)
    b.write_bytes(b'local edit')
    with self.assertRaises(ValueError):p.restore('one',['files/one-1'])
    with (p.OUT/'payloads.tar.zst').open('ab') as f:f.write(b'corruption')
    with self.assertRaisesRegex(ValueError,'payload store changed'):p.restore('two',['files/two-0'])
   finally:p.ROOT,p.OUT=oldroot,oldout
if __name__=='__main__':unittest.main()
