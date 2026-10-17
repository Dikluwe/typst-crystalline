from common import *
import re
x=state(__file__);source=D/'profiles-symbolized.json.gz';data=json.load(gzip.open(source,'rt'));raw=sorted({v['function']for m in data['symbols'].values()for v in m.values()});targets=[n for n in raw if n.startswith(('_ZN','_R'))];names=[re.sub(r'\.llvm\..*$','',n)for n in targets];cmd=['c++filt','-n','-s','rust'];p=subprocess.run(cmd,input='\n'.join(names)+'\n',capture_output=True,text=True,check=True);out=p.stdout.splitlines();assert len(out)==len(names);mapping=dict(zip(targets,out))
for m in data['symbols'].values():
 for v in m.values():
  if v['function']in mapping:v['raw_function']=v['function'];v['function']=mapping[v['function']]
data['symbol_normalization']={'command':cmd,'driver_sha256':sha(__file__),'input_sha256':sha(source),'llvm_suffix_removed_only_for_demangling':True,'function_names':len(mapping)}
with gzip.open(D/'profiles-normalized.json.gz','wt')as f:json.dump(data,f,ensure_ascii=False)
x.update(input_sha256=sha(source),command=cmd,normalized_functions=len(mapping),mapping=mapping);save('normalization.json.gz',x);print('normalized',len(mapping))
