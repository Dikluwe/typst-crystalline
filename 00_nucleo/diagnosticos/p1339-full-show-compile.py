"""Second/last transport correction on identical show fixtures.

Compile observes acceptance/diagnosis only; vanilla query already measured metadata.
Never infer matching semantics from compilation success alone.
"""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time

sys.dont_write_bytecode=True
spec=importlib.util.spec_from_file_location('probe',Path(__file__).with_name('p1339-full-probe.py'))
m=importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

def main():
    stage=sys.argv[1]
    source=json.loads((m.D/'p1339-full-show-final-manifest.json').read_text())
    a0=json.loads((m.D/'p1339-a0.json').read_text())
    cases=source['cases']
    focal=stage=='show-compile-focal'
    if focal:
        cases=[c for c in cases if c['id']=='strong-static-match']
    tmp=Path(tempfile.mkdtemp(prefix='p1339-full-show-compile-'))
    start_prov=m.provenance()
    m.save(stage+'-manifest',dict(**start_prov,entrypoint_sha256=m.sha(__file__),cases=cases,profiles=m.PROFILES,temp_output=str(tmp),predecessor_sha256=m.sha(m.D/'p1339-full-show-final-manifest.json'),observation='compile acceptance/diagnostic, not matching effect; matching already measured by vanilla query',correction=2))
    env=dict(os.environ,LC_ALL='C',NO_COLOR='1')
    for key in list(env):
        if key.startswith('TYPST_'):
            del env[key]
    for name,key in [('vanilla','vanilla'),('crystalline-before','candidate')]:
        binary=a0['binaries'][key]; assert m.sha(binary['path'])==binary['sha256']
        start=m.now(); rows=[]
        for order in (['normal'] if focal else ['normal','reverse']):
            for case in (cases if order=='normal' else list(reversed(cases))):
                assert m.sha(case['path'])==case['sha256']
                for profile,flags in m.PROFILES.items():
                    output=tmp/(case['id']+'-'+profile+'-'+order+'-'+name+'.pdf')
                    argv=[binary['path'],'compile',*flags,case['path'],str(output)]
                    began=m.now(); tick=time.monotonic()
                    try:
                        p=subprocess.run(argv,cwd=m.ROOT,env=env,capture_output=True,timeout=15)
                        row=dict(exit=p.returncode,stdout=p.stdout.decode(errors='replace'),stderr=p.stderr.decode(errors='replace'),execution='Observed')
                    except subprocess.TimeoutExpired as e:
                        row=dict(exit=None,stdout=(e.stdout or b'').decode(errors='replace'),stderr=(e.stderr or b'').decode(errors='replace'),execution='Unknown')
                    rows.append(dict(id=case['id'],order=order,profile=profile,argv=argv,cwd=str(m.ROOT),start=began,end=m.now(),seconds=time.monotonic()-tick,source_sha256=case['sha256'],output_sha256=m.sha(output) if output.exists() else None,**row))
        m.save(stage+'-'+name+'-runs',dict(start=start,end=m.now(),provenance=start_prov,end_provenance=m.provenance(),manifest_sha256=m.sha(m.D/('p1339-full-'+stage+'-manifest.json')),runner_sha256=m.sha(__file__),binary=binary,rows=rows))

if __name__=='__main__':
    main()
