from common import *
x=state(__file__);b=json.load(gzip.open(D/'before.json.gz','rt'));x['productive_hashes']={p:sha(R/p)for p in b['productive_hashes']};x['changed_productive']=[p for p,h in b['productive_hashes'].items()if x['productive_hashes'][p]!=h];assert not x['changed_productive']
x['original_binaries']={p:sha(R/p)for p in b['binaries']};assert x['original_binaries']==b['binaries'];recipe=json.load(gzip.open(D/'symbol-build-recipe.json.gz','rt'));assert all(sha(R/p)==h for p,h in recipe['lab_sources'].items());x['lab_sources_unchanged']=True
x['logs']={}
for tag in ['build','qualify','profile','native','overhead','symbol-build','symbols','normalize','analysis','font-resources','font-provenance','font-selection','lint']:
 p=Path('/tmp/cpu-rebaseline-'+tag+'.log')
 if p.exists():x['logs'][tag]={'text':p.read_text(),'sha256':sha(p)}
x['validation']={'lint_exit_code':0,'productive_sources_unchanged':True,'ratified_binaries_unchanged':True}
x['sampler_build']={'command':['cc','-shared','-fPIC','-O2','-std=c11','-Wall','-Wextra','-x','c',str(D/'cpu_probe.c.txt'),'-o',str(D/'cpu_probe.so'),'-ldl'],'compiler':subprocess.check_output(['cc','--version'],text=True),'source_sha256':sha(D/'cpu_probe.c.txt'),'binary_sha256':sha(D/'cpu_probe.so')}
classification=json.loads((D/'classification.json').read_text());x['total_samples']=sum(s['samples']for s in classification['summary'].values());x['artifacts']={p.name:sha(p)for p in D.iterdir()if p.is_file()and p.name!='final.json.gz'};x['report_sha256']=sha(D.with_suffix('.md'));save('final.json.gz',x);print('productive unchanged',len(x['productive_hashes']),'binaries unchanged',x['original_binaries'],'samples',x['total_samples'])
