"""A.1 selector realization via metadata emitted by show rules; no product reads."""
import importlib.util
import json
import os
import subprocess
import sys
import time
from pathlib import Path

P=Path(__file__).with_name('p1339-full-probe.py')
spec=importlib.util.spec_from_file_location('probe',P)
m=importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

def put(path,text):
    assert not path.exists()
    subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+s+'\n' for s in text.splitlines())+'*** End Patch\n',text=True,capture_output=True,check=True,cwd=m.ROOT)

def main():
    stage=sys.argv[1]
    a0=json.loads((m.D/'p1339-a0.json').read_text())
    provenance=m.provenance()
    corpus=[]
    for elem,field,content in [('strong','body: [BASE]','strong[BASE]'),('emph','body: [BASE]','emph[BASE]'),('text','text: "BASE"','text("BASE")')]:
        for form in ['static','bound']:
            for mode,args in [('empty',''),('match',field),('miss',field.replace('BASE','OTHER'))]:
                id=elem+'-'+form+'-'+mode
                selector='function.where('+elem+(','+args if args else '')+')' if form=='static' else elem+'.where('+args+')'
                source='#show '+selector+': it => metadata("MATCH")\n#'+content+'\n'
                path=m.D/('p1339-full-show-fixture-'+id+'.typ')
                if not path.exists():
                    put(path,source)
                corpus.append(dict(id=id,source=source,path=str(path),sha256=m.sha(path),preclassification='Unknown',obligation='realized selector filters '+elem+' through show; field miss must not emit MATCH'))
    if stage=='show-focal':
        corpus=[c for c in corpus if c['id']=='strong-static-match']
    m.save(stage+'-manifest',dict(**provenance,runner_sha=m.sha(__file__),cases=corpus,profiles=m.PROFILES,observation='query metadata --field value triggers layout; metadata emission observes selector match',no_candidate=True))
    env=dict(os.environ,LC_ALL='C',NO_COLOR='1')
    for key in list(env):
        if key.startswith('TYPST_'):
            del env[key]
    for name,key in ([('vanilla','vanilla')] if stage=='show-focal' else [('vanilla','vanilla'),('crystalline-before','candidate')]):
        binary=a0['binaries'][key]
        assert m.sha(binary['path'])==binary['sha256']
        start=m.now(); rows=[]
        for order in (['normal'] if stage=='show-focal' else ['normal','reverse']):
            for case in (corpus if order=='normal' else list(reversed(corpus))):
                for profile,flags in (list(m.PROFILES.items())[:1] if stage=='show-focal' else m.PROFILES.items()):
                    argv=[binary['path'],'query',case['path'],'metadata','--field','value',*flags]
                    began=m.now(); tick=time.monotonic()
                    try:
                        p=subprocess.run(argv,cwd=m.ROOT,env=env,capture_output=True,timeout=15)
                        channels=dict(exit=p.returncode,stdout=p.stdout.decode(errors='replace'),stderr=p.stderr.decode(errors='replace'),execution='Observed')
                    except subprocess.TimeoutExpired as e:
                        channels=dict(exit=None,stdout=(e.stdout or b'').decode(errors='replace'),stderr=(e.stderr or b'').decode(errors='replace'),execution='Unknown')
                    rows.append(dict(id=case['id'],order=order,profile=profile,argv=argv,cwd=str(m.ROOT),source_sha256=case['sha256'],start=began,end=m.now(),seconds=time.monotonic()-tick,**channels))
        m.save(stage+'-'+name+'-runs',dict(start=start,end=m.now(),provenance=provenance,end_provenance=m.provenance(),runner_sha256=m.sha(__file__),manifest_sha256=m.sha(m.D/('p1339-full-'+stage+'-manifest.json')),binary=binary,rows=rows))

if __name__=='__main__':
    main()
