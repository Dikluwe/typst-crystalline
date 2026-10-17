from common import *
import struct
x=state(__file__);source=D/'profiles.json.gz';data=json.load(gzip.open(source,'rt'));original=B['orig'];donor=Path('/tmp/tekt-cpu-rebaseline/symbols/typst')
def sections(path):
 b=path.read_bytes();h=struct.unpack_from('<16sHHIQQQIHHHHHH',b);assert h[0][:6]==b'\x7fELF\x02\x01';ss=[struct.unpack_from('<IIQQQQIIQQ',b,h[6]+i*h[11])for i in range(h[12])];strings=b[ss[h[13]][4]:ss[h[13]][4]+ss[h[13]][5]];out={}
 for s in ss:
  name=strings[s[0]:].split(b'\0',1)[0].decode();out[name]={'type':s[1],'flags':s[2],'address':s[3],'offset':s[4],'size':s[5],'sha256':hashlib.sha256(b[s[4]:s[4]+s[5]]).hexdigest()if s[1]!=8 else None}
 return out
old,new=sections(original),sections(donor);matches={}
for name,a in old.items():
 if not a['flags']&2:continue
 b=new[name];same=all(a[k]==b[k]for k in ['type','flags','address','size','sha256']);matches[name]={'original':a,'donor':b,'same':same}
bad=[n for n,v in matches.items()if not v['same']];assert not bad or bad==['.note.gnu.build-id'],bad
assert all(matches[n]['same']for n in ['.text','.init','.fini','.plt']);assert set(n for n,v in new.items()if v['flags']&2)==set(matches)
x.update(original_sha256=sha(original),donor_sha256=sha(donor),sections=matches,source_receipt_sha256=sha(source),build_recipe_sha256=sha(D/'symbol-build-recipe.json.gz'),build_log=Path('/tmp/cpu-rebaseline-symbol-build.log').read_text(),accepted=True)
path=str(original);pcs=sorted(int(a,16)for a in data['symbols'][path]);cmd=['eu-addr2line','-f','-C','-a','-e',str(donor)];p=subprocess.run(cmd,input=''.join(hex(a)+'\n'for a in pcs),capture_output=True,text=True,check=True);lines=p.stdout.splitlines();assert len(lines)==3*len(pcs)
data['symbols'][path]={format(a,'x'):{'function':lines[i*3+1],'location':lines[i*3+2]}for i,a in enumerate(pcs)};x['symbolize_command']=cmd;x['addresses']=pcs;x['unknown_after']=sum(v['function']=='??'for v in data['symbols'][path].values());data['symbol_donor']={'original':str(original),'original_sha256':sha(original),'donor':str(donor),'donor_sha256':sha(donor),'text':matches['.text'],'all_alloc_sections_equal_except':bad,'command':cmd,'recipe':'symbol-build-recipe.json.gz'}
# The original profiled ELF is unchanged; donor supplies names only for identical runtime code/addresses.
with gzip.open(D/'profiles-symbolized.json.gz','wt')as f:json.dump(data,f,ensure_ascii=False)
save('symbol-donor.json.gz',x);print('accepted donor',x['donor_sha256'],'alloc differences',bad,'unknown',x['unknown_after'])
