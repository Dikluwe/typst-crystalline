"""Execute the unchanged P1335 universe; report observations, never a verdict."""
import argparse
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
import datetime
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[2]
D=ROOT/'00_nucleo/diagnosticos'
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def state():
    def git(*args):
        return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
    return dict(at=now(),head=git('rev-parse','HEAD').strip(),status=git('status','--short'),diff_stat=git('diff','HEAD','--stat'),
                modified_sha256={p:sha(ROOT/p) for p in git('diff','HEAD','--name-only').splitlines()},
                untracked_product_sha256={p:sha(ROOT/p) for p in git('ls-files','--others','--exclude-standard','--','01_core','02_shell','03_infra','04_wiring','tests').splitlines()})
def save(path,data):
    assert not path.exists(), 'Immutable output; choose a successor receipt'
    body=json.dumps(data,ensure_ascii=False,indent=2)
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in body.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],cwd=ROOT,input=patch,text=True,check=True)
def load_module(name,path):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('phase',choices=['freeze','normal','repeat','reverse'])
    parser.add_argument('--candidate',type=Path)
    parser.add_argument('--seal',type=Path,default=D/'p1339-seal.json')
    parser.add_argument('--prefix',default='p1339-rebaseline')
    parser.add_argument('--workers',type=int,default=8)
    args=parser.parse_args()
    assert args.prefix.startswith('p1339-rebaseline') and Path(args.prefix).name==args.prefix
    frozen_path=D/(args.prefix+'-freeze.json')
    catalog_path=D/'p1335-probe-catalog.json'
    old_runner=D/'p1309-matrix.py'
    assert sha(catalog_path)=='988bab4e5a88e400db2533f1db8a428328f1a5fcec51d4415ea171d20d5832d8'
    assert sha(old_runner)=='7bf0b67dd52d640237a7667eca19d23c7b6d831624e373e1361982299bccc079'
    catalog=json.loads(catalog_path.read_text())
    old=load_module('p1339_readonly_p1309_matrix',old_runner)
    assert len(catalog['probes'])==4718 and catalog['profiles']==old.PROFILES
    if args.phase=='freeze':
        assert args.candidate and args.candidate.is_absolute() and args.candidate.is_file()
        assert args.seal.is_file(), 'Only prepare actual runtime freeze after independent seal exists'
        candidate=args.candidate.resolve()
        assert candidate not in [Path('/usr/local/bin/typst'),Path('/tmp/p1338-target.vlNAmp/release/typst')]
        inputs=[catalog_path,old_runner,D/'p1309-record-r2.py',Path(__file__).resolve(),args.seal.resolve(),D/'p1339-contract-r3.json',D/'p1339-authority-manifest-r2.json']
        save(frozen_path,dict(at=now(),state=state(),manifest_sha256=sha(D/'p1339-authority-manifest-r2.json'),
             inputs={str(p):sha(p) for p in inputs},
             binaries={side:dict(path=str(p),sha256=sha(p)) for side,p in [('vanilla',Path('/usr/local/bin/typst')),('crystalline',candidate)]},
             profiles=old.PROFILES,workers=args.workers,orders=['normal','repeat','reverse'],
             projection='Unmodified p1309 observe/classify: exact raw stdout/stderr; successful JSON must parse; no normalization',
             command=sys.argv))
        return
    frozen=json.loads(frozen_path.read_text())
    def verify():
        for path,digest in frozen['inputs'].items(): assert sha(path)==digest,path
        for binary in frozen['binaries'].values(): assert sha(binary['path'])==binary['sha256']
    verify()
    before=state();start=now();tick=time.monotonic()
    jobs=[(p,profile) for p in sorted(catalog['probes'],key=lambda p:p['id']) for profile in old.PROFILES]
    if args.phase=='reverse':jobs.reverse()
    def pair(job):
        probe,profile=job
        row=dict(id=probe['id'],path=probe['path'],expression=probe['expression'],profile=profile,phase=args.phase,universe='principal')
        for side in (('crystalline','vanilla') if args.phase=='reverse' else ('vanilla','crystalline')):
            row[side]=old.observe(frozen['binaries'][side],probe,profile,side,args.phase)
        row['runtime_class']=old.classify(row['vanilla'],row['crystalline'])
        return row
    rows=[]
    with ThreadPoolExecutor(max_workers=frozen['workers']) as pool:
        for row in pool.map(pair,jobs):
            rows.append(row)
            if len(rows)%1000==0:print(args.phase,len(rows),len(jobs),flush=True)
    verify();after=state()
    assert before['head']==after['head'] and before['modified_sha256']==after['modified_sha256'] and before['untracked_product_sha256']==after['untracked_product_sha256'], 'Source changed during measurement'
    counts=dict(Counter(row['runtime_class'] for row in rows))
    save(D/(args.prefix+'-'+args.phase+'.json'),dict(schema='p1339-rebaseline-observations-v1',start=start,end=now(),seconds=time.monotonic()-tick,
         manifest_sha256=frozen['manifest_sha256'],freeze_sha256=sha(frozen_path),runner_sha256=sha(__file__),
         before=before,after=after,binaries=frozen['binaries'],profiles=old.PROFILES,workers=frozen['workers'],
         probes=len(catalog['probes']),pairs=len(rows),counts=counts,results=rows,command=sys.argv,
         verdict='NOT_ASSIGNED; independent verification required'))
    print(json.dumps(counts),flush=True)

if __name__=='__main__':main()
