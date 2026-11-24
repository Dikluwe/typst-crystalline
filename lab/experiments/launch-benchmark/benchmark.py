#!/usr/bin/env python3
"""Serial paired Linux CLI benchmark. See README.md for scope and acceptance."""
from pathlib import Path
from contextlib import contextmanager
import argparse, datetime, fcntl, hashlib, itertools, json, os, random, shutil, statistics, subprocess, time, select, signal

def digest(path):
    with open(path,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def verify(path, expected):
    actual=digest(path)
    if actual != expected:raise ValueError(f'hash mismatch: {path}: {actual} != {expected}')
    return actual
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def environment(source):
    forbidden=[k for k in source if k.startswith('MALLOC_') or k in ('LD_PRELOAD','LD_AUDIT','GLIBC_TUNABLES')]
    if forbidden:raise ValueError(f'non-nominal environment: {forbidden}')
    return dict(sorted(source.items()))
def orders(arms, rounds, seed):
    if not 2<=len(arms)<=3:raise ValueError('two or three arms required')
    base=list(itertools.permutations(arms))
    if rounds<=0 or rounds%len(base):raise ValueError('rounds must be a positive multiple of arm permutations')
    result=base*(rounds//len(base));random.Random(seed).shuffle(result);return result
@contextmanager
def lock(root):
    with (root/'runner.lock').open('a') as f:
        fcntl.flock(f,fcntl.LOCK_EX|fcntl.LOCK_NB)
        try:yield
        finally:fcntl.flock(f,fcntl.LOCK_UN)
def argv(root):return [str(root/'typst'),'compile','--color','never',str(root/'input.typ'),str(root/'output.pdf')]
def measure(args, root, env, mode):
    cmd=(['/usr/bin/time','-f','%M','-o',str(root/'rss.txt')]+args) if mode=='memory' else args
    with (root/'stdout.txt').open('wb') as out,(root/'stderr.txt').open('wb') as err:
        start=utc();t=time.perf_counter_ns()
        p=subprocess.Popen(cmd,cwd=root,env=env,stdout=out,stderr=err,start_new_session=True)
        try:
            fd=os.pidfd_open(p.pid)
            try:
                if not select.select([fd],[],[],120)[0]:
                    raise subprocess.TimeoutExpired(cmd,120)
            finally:os.close(fd)
            p.wait()
        except BaseException:
            # Kill the whole group: GNU time must not leave its compiler alive.
            try:os.killpg(p.pid,signal.SIGKILL)
            except ProcessLookupError:pass
            p.wait()
            raise
        elapsed=time.perf_counter_ns()-t
    if p.returncode:raise RuntimeError(f'child exit {p.returncode}; see {root}/stderr.txt')
    result={'utc':start,'end':utc(),'argv':args,'measurement_argv':cmd,'cwd':str(root),'exit':p.returncode,
            'stdout_sha256':digest(root/'stdout.txt'),'stderr_sha256':digest(root/'stderr.txt')}
    result.update({'rss_kib':int((root/'rss.txt').read_text())} if mode=='memory' else {'wall_ns':elapsed})
    return result

def campaign(manifest_path, output, root, rounds=12, seed=20261114):
    manifest_path=manifest_path.resolve();output=output.absolute();root=root.absolute()
    m=json.loads(manifest_path.read_text())
    if m.get('self_contained_fixtures') is not True:raise ValueError('self-contained fixture declaration required; imports/assets unsupported')
    if root.is_symlink() or root.resolve()!=root:raise ValueError('root must be canonical, without symlinks')
    # Fresh directory prevents accidental overwrite and stale summaries. It is intentionally retained.
    root.mkdir(mode=0o700);output.mkdir(parents=True,exist_ok=False)
    env=environment(os.environ);env['PWD']=str(root);env=environment(env)
    env_hash=hashlib.sha256(json.dumps(env,sort_keys=True).encode()).hexdigest()
    sequence=orders(list(m['arms']),rounds,seed)
    state={'status':'running','start':utc(),'manifest':m,'manifest_sha256':digest(manifest_path),'runner_sha256':digest(__file__),
           'root':str(root),'environment_sha256':env_hash,'environment_keys':list(env),'orders':sequence,'seed':seed,
           'platform':os.uname()._asdict() if hasattr(os.uname(),'_asdict') else list(os.uname()),
           'aslr':Path('/proc/sys/kernel/randomize_va_space').read_text().strip(),
           'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
           'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True)}
    def save(): (output/'state.json').write_text(json.dumps(state,indent=2))
    save()
    try:
      with lock(root):
        (root/'bins').mkdir();(root/'fixtures').mkdir()
        for group,folder in [('arms','bins'),('fixtures','fixtures')]:
            for i,(name,item) in enumerate(m[group].items()):
                src=Path(item['path']).resolve();verify(src,item['sha256'])
                dest=root/folder/str(i);shutil.copyfile(src,dest);verify(dest,item['sha256']);verify(src,item['sha256'])
                dest.chmod(0o555 if group=='arms' else 0o444);item['staged']=str(dest)
        save();observables={}
        with (output/'samples.jsonl').open('x') as log:
          for mode in ['memory','time']:
            for r,order in enumerate([sequence[0]]+sequence):
              cases=list(m['fixtures']);random.Random(seed+r).shuffle(cases)
              for case in cases:
                item=m['fixtures'][case];verify(item['staged'],item['sha256'])
                shutil.copyfile(item['staged'],root/'input.typ')
                for arm in order:
                    a=m['arms'][arm];verify(a['staged'],a['sha256'])
                    # rename is a no-op when both names already refer to the same inode.
                    (root/'typst.next').unlink(missing_ok=True)
                    os.link(a['staged'],root/'typst.next');os.replace(root/'typst.next',root/'typst')
                    verify(root/'typst',a['sha256']);verify(root/'input.typ',item['sha256'])
                    identity=(os.stat(root/'typst').st_dev,os.stat(root/'typst').st_ino)
                    (root/'output.pdf').unlink(missing_ok=True)
                    row=measure(argv(root),root,env,mode)
                    verify(root/'typst',a['sha256']);verify(root/'input.typ',item['sha256'])
                    if identity != (os.stat(root/'typst').st_dev,os.stat(root/'typst').st_ino):raise ValueError('executable replaced during sample')
                    geom=subprocess.check_output(['pdftotext','-bbox',str(root/'output.pdf'),'-'])
                    geom=geom[geom.index(b'<body>'):]
                    observable=(hashlib.sha256(geom).hexdigest(),row['stdout_sha256'],row['stderr_sha256'])
                    if case in observables and observables[case]!=observable:raise ValueError(f'output mismatch: {case}/{arm}')
                    observables[case]=observable
                    row.update(mode=mode,round=r,warmup=r==0,fixture=case,arm=arm,order=order,elf_sha256=a['sha256'],input_sha256=item['sha256'],environment_sha256=env_hash,geometry_sha256=observable[0],inode=identity)
                    log.write(json.dumps(row)+'\n');log.flush()
              print(mode,r,flush=True)
        rows=[json.loads(s) for s in (output/'samples.jsonl').read_text().splitlines()]
        expected=2*(rounds+1)*len(m['fixtures'])*len(m['arms'])
        if len(rows)!=expected:raise ValueError('incomplete sample count')
        summary=[];baseline=list(m['arms'])[0]
        for mode,metric in [('memory','rss_kib'),('time','wall_ns')]:
          for case in m['fixtures']:
            for arm in list(m['arms'])[1:]:
                rs=[x for x in rows if x['mode']==mode and x['fixture']==case and not x['warmup']]
                a=[next(x[metric] for x in rs if x['round']==r and x['arm']==arm) for r in range(1,rounds+1)]
                b=[next(x[metric] for x in rs if x['round']==r and x['arm']==baseline) for r in range(1,rounds+1)]
                delta=[x-y for x,y in zip(a,b)]
                summary.append(dict(mode=mode,fixture=case,comparison=arm+' - '+baseline,pairs=rounds,metric=metric,median_delta=statistics.median(delta),median_percent=100*statistics.median(x/y-1 for x,y in zip(a,b)),positive=sum(x>0 for x in delta),min_delta=min(delta),max_delta=max(delta)))
        # Preserve evidence of staged inputs; no implicit cleanup.
        for group in ['arms','fixtures']:
            for item in m[group].values():verify(item['path'],item['sha256']);verify(item['staged'],item['sha256'])
        (output/'summary.json').write_text(json.dumps(summary,indent=2))
        state.update(status='complete',end=utc(),samples=len(rows));save()
    except BaseException as e:
        state.update(status='failed',end=utc(),error=repr(e));save();raise

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('manifest',type=Path);p.add_argument('output',type=Path);p.add_argument('root',type=Path);p.add_argument('--rounds',type=int,default=12);p.add_argument('--seed',type=int,default=20261114)
    a=p.parse_args();campaign(a.manifest,a.output,a.root,a.rounds,a.seed)
