#!/usr/bin/env python3
"""Sequential, fingerprint-gated performance baseline; writes only this diagnostic directory.
Build products go to Cargo's existing ignored target directory. No source edits.
"""
import argparse, datetime, gzip, hashlib, json, os, pathlib, platform, shutil, signal, subprocess, tarfile, time
ROOT = pathlib.Path(__file__).resolve().parents[3]
OUT = pathlib.Path(__file__).resolve().parent
PIN = '9f2b6e8715237cb086899a42873660fe744622e8'
VANILLA_SHA = '7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
TARGETS = ['scanner_bench', 'content_bench', 'layout_bench', 'pdf_builder_bench', 'stream_bench', 'pipeline_bench', 'observation_bench', 'stdlib_bench']
ENV = os.environ.copy()
ENV.update(CARGO_BUILD_JOBS='1', CARGO_ENCODED_RUSTFLAGS='', RUSTFLAGS='', RAYON_NUM_THREADS='1', PYTHONDONTWRITEBYTECODE='1', SOURCE_DATE_EPOCH='0', TZ='UTC', LC_ALL='C.UTF-8')
for key in list(ENV):
    if key.startswith(('TYPST_', 'CRYSTALLINE_', 'CARGO_PROFILE_')): ENV.pop(key)
CPU = 4
FONT_CACHE_PATH = OUT/'font-digest-cache.json'
FONT_CACHE = json.loads(FONT_CACHE_PATH.read_text()) if FONT_CACHE_PATH.exists() else {}
SIBLINGS = set(map(int, pathlib.Path(f'/sys/devices/system/cpu/cpu{CPU}/topology/thread_siblings_list').read_text().strip().replace('-',',').split(',')))
def digest(p):
    if p.is_relative_to(ROOT): return sha(p)
    st=p.stat(); signature=[st.st_dev,st.st_ino,st.st_size,st.st_mtime_ns,st.st_ctime_ns]
    old=FONT_CACHE.get(str(p))
    if old and old['stat']==signature: return old['sha256']
    value=sha(p)
    assert signature==[p.stat().st_dev,p.stat().st_ino,p.stat().st_size,p.stat().st_mtime_ns,p.stat().st_ctime_ns], 'font changed during hashing'
    FONT_CACHE[str(p)]={'stat':signature,'sha256':value}
    return value

def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p):
    h=hashlib.sha256()
    with open(p,'rb') as f:
        for block in iter(lambda:f.read(1048576),b''): h.update(block)
    return h.hexdigest()
def save(p,obj):
    p=OUT/p; p.parent.mkdir(parents=True,exist_ok=True)
    p.write_text(json.dumps(obj,indent=2,ensure_ascii=False)+'\n')
def call(cmd): return subprocess.check_output(cmd,cwd=ROOT,env=ENV,text=True,stderr=subprocess.STDOUT)
def processes():
    rows=[]
    for p in pathlib.Path('/proc').glob('[0-9]*'):
        try:
            s=(p/'stat').read_text(); tail=s[s.rfind(')')+2:].split()
            argv=(p/'cmdline').read_bytes().split(b'\0')
            script=next((x.decode(errors='replace') for x in argv[1:] if x.endswith(b'.py')),None)
            rows.append({'pid':int(p.name),'ppid':int(tail[1]),'comm':(p/'comm').read_text().strip(),'exe':os.readlink(p/'exe'),'script':script,'ticks':int(tail[11])+int(tail[12])})
        except (OSError,ValueError,IndexError): pass
    return rows

def blockers(rows,own):
    allowed={os.getpid()}
    if own: allowed.add(own)
    for _ in range(8): allowed.update(r['pid'] for r in rows if r['ppid'] in allowed)
    result=[]
    for r in rows:
        if r['pid'] in allowed: continue
        name=pathlib.Path(r['exe']).name
        build_tools={'cargo','rustc','rustdoc','cmake','ninja','make','gmake','gcc','g++','cc','c++','cc1','cc1plus','collect2','ld','ld.lld','rust-lld','clang','clang++','go','javac','gradle','mvn','pytest','ctest','hyperfine','typst','crystalline-lint'}
        if (r.get('script') and any(x in r['script'] for x in ['reproduction','probe','benchmark','test_'])) or name in build_tools or name.endswith('.test') or 'bench' in name.lower() or name.lower().endswith(('_test','-test','tests')) or ('/target/' in r['exe'] and '/deps/' in r['exe']): result.append(r)
    return result

def source_files():
    paths=set()
    for base in ['01_core','02_shell','03_infra','04_wiring','benches','.cargo']:
        for p in (ROOT/base).rglob('*'):
            if p.is_file() and not any(x in {'target','__pycache__','.git'} for x in p.parts): paths.add(p)
    for name in ['Cargo.toml','Cargo.lock','rust-toolchain','rust-toolchain.toml','build.rs']:
        p=ROOT/name
        if p.is_file(): paths.add(p)
    for p in (OUT/'fixtures').glob('*'):
        if p.is_file(): paths.add(p)
    for p in (OUT/'fonts').glob('*'):
        if p.is_file(): paths.add(p)
    paths.update(OUT/name for name in ['run.py','e2e.py','prepare_fonts.py'])
    if (OUT/'vanilla-provenance.json').exists():
        for name in json.loads((OUT/'vanilla-provenance.json').read_text())['source_files']:
            paths.add(ROOT/'lab/typst-original'/name)
    # Full file identities of accessible system fonts and fontconfig configuration.
    for name in set(call(['fc-list','--format','%{file}\n']).splitlines()):
        p=pathlib.Path(name)
        if p.is_file():paths.add(p)
    for root in [pathlib.Path('/usr/share/fonts'),pathlib.Path('/usr/local/share/fonts'),pathlib.Path('/home/dikluwe/.local/share/fonts'),pathlib.Path('/usr/share/texmf/fonts')]:
        if root.exists():
            for p in root.rglob('*'):
                if p.is_file() and p.suffix.lower() in {'.ttf','.otf','.ttc','.pfb','.pfa'}:paths.add(p)
    if (OUT/'embedded-fonts.json').exists():
        for row in json.loads((OUT/'embedded-fonts.json').read_text()):
            for name in ['tekt_path','vanilla_path']:
                p=pathlib.Path(row[name])
                if p.is_file():paths.add(p)
    for p in pathlib.Path('/etc/fonts').rglob('*'):
        if p.is_file(): paths.add(p)
    return sorted(paths)

def fingerprint():
    files={str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p):digest(p) for p in source_files()}
    FONT_CACHE_PATH.write_text(json.dumps(FONT_CACHE,sort_keys=True)+'\n')
    h=hashlib.sha256(json.dumps(files,sort_keys=True).encode()).hexdigest()
    return {'utc':utc(),'head':call(['git','rev-parse','HEAD']).strip(),'sha256':h,'files':files}

def snapshot(name):
    fp=fingerprint(); save(pathlib.Path('state')/(name+'.json'),fp)
    for suffix,cmd in [('status.txt',['git','status','--porcelain=v1','--untracked-files=all']),('diff-stat.txt',['git','diff','HEAD','--stat'])]:
        (OUT/'state'/f'{name}.{suffix}').write_text(call(cmd))
    patch=call(['git','diff','HEAD','--binary']).encode()
    digest_value=hashlib.sha256(patch).hexdigest(); directory=OUT/'state'/'patches';directory.mkdir(exist_ok=True)
    target=directory/(digest_value+'.patch.gz')
    if not target.exists(): target.write_bytes(gzip.compress(patch,mtime=0))
    save(pathlib.Path('state')/(name+'.patch.json'),{'patch_sha256':digest_value,'file':str(target.relative_to(OUT))})
    return fp

def quiet():
    a=fingerprint(); seen=[]
    for _ in range(30):
        bad=blockers(processes(),None)
        if bad: seen.extend(bad)
        time.sleep(1)
    b=fingerprint(); save(pathlib.Path('state')/('quiet-'+str(time.time_ns())+'.json'),{'before':a,'after':b,'blockers':seen})
    if a['sha256']!=b['sha256'] or a['head']!=b['head'] or seen: raise RuntimeError('No stable/idle 30-second gate; try again after other work stops')
    expected=json.loads((OUT/'build-state.json').read_text()) if (OUT/'build-state.json').exists() else None
    if expected and (expected['sha256']!=b['sha256'] or expected['head']!=b['head']): raise RuntimeError('Sources changed after build: rebuild before measuring')

def require_built_state(fp):
    expected=json.loads((OUT/'build-state.json').read_text())
    assert (fp['sha256'],fp['head'])==(expected['sha256'],expected['head']), 'Sources changed after build: rebuild before measuring'

def resume_or_retire(label,directory):
    receipt=OUT/'receipts'/(label+'.json')
    if not directory.exists(): return False
    expected=json.loads((OUT/'build-state.json').read_text())
    if receipt.exists():
        record=json.loads(receipt.read_text())
        if record['valid'] and record['fingerprint_before']==expected['sha256'] and record['fingerprint_after']==expected['sha256']:
            print(label,'already accepted; retained',flush=True); return True
    destination=OUT/'rejected'/(label+'-'+str(time.time_ns()));destination.mkdir(parents=True)
    shutil.move(str(directory),str(destination/'data'))
    for p in [receipt,OUT/'logs'/(label+'.log'),*list((OUT/'state').glob(label+'-*'))]:
        if p.is_file():shutil.move(str(p),str(destination/p.name))
    return False

def monitored(cmd,label,affinity=True,extra=None):
    before=snapshot(label+'-before')
    if not label.startswith('build-'): require_built_state(before)
    rows=processes(); bad=blockers(rows,None)
    if bad and not label.startswith('build-'): raise RuntimeError('Concurrent workload before '+label+': '+str(bad))
    env=ENV.copy(); env.update(extra or {})
    log=OUT/'logs'/(label+'.log'); log.parent.mkdir(exist_ok=True)
    clock=time.perf_counter_ns(); start=utc(); telemetry=[]; contamination=[]
    with open(log,'w') as f:
        p=subprocess.Popen(cmd,cwd=ROOT,env=env,stdout=f,stderr=subprocess.STDOUT,preexec_fn=(lambda:os.sched_setaffinity(0,{CPU})) if affinity else None,start_new_session=True)
        while p.poll() is None:
            current=processes(); hit=blockers(current,p.pid)
            if hit:
                contamination.extend(hit)
                if not label.startswith('build-'):
                    try:os.killpg(p.pid,signal.SIGTERM)
                    except ProcessLookupError:pass
            telemetry.append({'utc':utc(),'loadavg':pathlib.Path('/proc/loadavg').read_text().strip(),'blockers':hit,'cpu_stat':[x for x in pathlib.Path('/proc/stat').read_text().splitlines() if any(x.startswith(f'cpu{i} ') for i in SIBLINGS)]})
            time.sleep(.25)
        code=p.returncode
    duration=(time.perf_counter_ns()-clock)/1e6
    after=snapshot(label+'-after')
    stable=before['sha256']==after['sha256'] and before['head']==after['head']
    result={'command':cmd,'cwd':str(ROOT),'start_utc':start,'end_utc':utc(),'launcher_elapsed_ms_not_a_benchmark':duration,'exit_code':code,'valid':code==0 and stable and (not contamination or label.startswith('build-')),'measurement':not label.startswith('build-'),'concurrent_preparation':bool(contamination) and label.startswith('build-'),'source_stable':stable,'contamination':contamination,'telemetry':telemetry,'fingerprint_before':before['sha256'],'fingerprint_after':after['sha256'],'environment_overrides':{k:env[k] for k in ['CARGO_BUILD_JOBS','CARGO_ENCODED_RUSTFLAGS','RUSTFLAGS','RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','TZ','LC_ALL']},'affinity':CPU if affinity else None}
    save(pathlib.Path('receipts')/(label+'.json'),result)
    print(label, 'VALID' if result['valid'] else 'DISCARDED',flush=True)
    if not result['valid']: raise RuntimeError('Invalid result; raw log retained: '+label)
    return result

def prepare():
    (OUT/'fixtures').mkdir(exist_ok=True)
    fixtures={'pipeline-single-font':'#set page(width: 64pt, height: 64pt)\nHello',
      'text-dense':'#set page(width: 400pt, height: 600pt, margin: 24pt)\n#set text(font: "Libertinus Serif", size: 10pt)\n'+('\n\nPerformance baseline uses identical text, fonts, and options. Repeated words exercise text layout and shaping without generated dates or external packages.'*32),
      'shapes':'#set page(width: 400pt, height: 600pt, margin: 24pt)\n#for i in range(128) { place(top + left, dx: calc.rem(i, 8) * 30pt, dy: calc.floor(i / 8) * 14pt, rect(width: 24pt, height: 8pt, fill: blue)) }',
      'context':'#set page(width: 128pt, height: 64pt, margin: 8pt)\n#set text(font: "Libertinus Serif", size: 10pt)\n#context [Page #counter(page).get().first()]'}
    for name,text in fixtures.items(): (OUT/'fixtures'/(name+'.typ')).write_text(text)
    (OUT/'fixtures'/'tiny.jpg').write_bytes((ROOT/'03_infra/fixtures/p307b/sources/tiny.jpg').read_bytes())
    (OUT/'fixtures'/'image.typ').write_text('#set page(width: 128pt, height: 128pt, margin: 8pt)\n#image("tiny.jpg", width: 80pt, height: 80pt)')
    fonts=sorted(set(call(['fc-list','--format','%{file}\n']).splitlines()))
    save(pathlib.Path('system-font-paths.json'),fonts)
    host={'utc':utc(),'machine':platform.uname()._asdict(),'cpu':call(['lscpu']),'rustc':call(['rustc','-vV']),'cargo':call(['cargo','-V']),'python':platform.python_version(),'affinity':sorted(os.sched_getaffinity(0)),'selected_cpu':CPU,'siblings':pathlib.Path(f'/sys/devices/system/cpu/cpu{CPU}/topology/thread_siblings_list').read_text().strip(),'governor':pathlib.Path(f'/sys/devices/system/cpu/cpu{CPU}/cpufreq/scaling_governor').read_text().strip(),'memory':pathlib.Path('/proc/meminfo').read_text(),'boot_id':pathlib.Path('/proc/sys/kernel/random/boot_id').read_text().strip(),'environment_overrides':{k:ENV[k] for k in ['CARGO_BUILD_JOBS','CARGO_ENCODED_RUSTFLAGS','RUSTFLAGS','RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','TZ','LC_ALL']}}
    save(pathlib.Path('host.json'),host)
    original=ROOT/'lab/typst-original/target/release/typst'
    assert sha(original)==VANILLA_SHA,'Vanilla binary differs from ratified P1489 executable'
    entries=call(['git','ls-tree','-r',PIN]).splitlines(); differences=[]; files={}
    for row in entries:
        meta,name=row.split('\t',1); mode,kind,expected=meta.split()
        if kind!='blob': continue
        p=ROOT/'lab/typst-original'/name
        if not p.exists(): differences.append({'path':name,'missing':True});continue
        data=p.read_bytes(); observed=hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()
        files[name]=hashlib.sha256(data).hexdigest()
        if observed!=expected: differences.append({'path':name,'expected_git_blob':expected,'observed_git_blob':observed})
    save(pathlib.Path('vanilla-provenance.json'),{'utc':utc(),'pin':PIN,'tree':call(['git','rev-parse',PIN+'^{tree}']).strip(),'executable':str(original),'sha256':sha(original),'version_auxiliary_only':call([str(original),'--version']).strip(),'source_differences':differences,'source_files':files,'recipe_authority':'00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md','recipe':'CARGO_BUILD_JOBS=1 TYPST_COMMIT_SHA='+PIN+' cargo build --release --locked --manifest-path lab/typst-original/Cargo.toml -p typst-cli -p p1140-inventory --quiet'})
    snapshot('initial')
    print('Protocol prepared; vanilla binary verified; source differences:',[x['path'] for x in differences],flush=True)

def build():
    # Builds are preparation, never timing samples. Verify stable sources around
    # each build and require a quiet gate again before actual measurements.
    pass
    monitored(['cargo','bench','--manifest-path','benches/Cargo.toml','--no-run','--offline','--locked','--message-format=json'],'build-benches',False)
    monitored(['cargo','build','--release','-p','typst-wiring','--offline','--locked','--message-format=json'],'build-cli',False)
    monitored(['cargo','test','--offline','-p','typst-core','--release','--lib','--no-run','--message-format=json'],'build-private',False)
    artifacts={}
    for label in ['build-benches','build-cli','build-private']:
        for row in (OUT/'logs'/(label+'.log')).read_text().splitlines():
            try: m=json.loads(row)
            except ValueError: continue
            if m.get('reason')!='compiler-artifact' or not m.get('executable'): continue
            name=m['target']['name']; profile=m['profile']; exe=pathlib.Path(m['executable'])
            if name in TARGETS or name in {'typst','typst_core'}:
                assert profile['opt_level'] in {'1','2','3','s','z'} and not profile['debug_assertions']
                artifacts[name]={'path':str(exe),'sha256':sha(exe),'profile':profile,'features':m.get('features'),'fresh':m.get('fresh')}
    assert all(x in artifacts for x in TARGETS+['typst','typst_core'])
    fp=snapshot('built')
    for label in ['build-benches','build-cli','build-private']:
        receipt=json.loads((OUT/'receipts'/(label+'.json')).read_text())
        assert receipt['fingerprint_before']==fp['sha256']==receipt['fingerprint_after'], 'Preparation products came from different source states'
    save(pathlib.Path('artifacts.json'),artifacts);save(pathlib.Path('build-state.json'),fp)
    with tarfile.open(OUT/'consumers-inputs.tar.gz','w:gz') as tar:
        for p in source_files():
            if p.is_relative_to(ROOT): tar.add(p,arcname=str(p.relative_to(ROOT)))
    print('All optimized artifacts recorded; source archive created',flush=True)

def identity_check():
    artifacts=json.loads((OUT/'artifacts.json').read_text())
    for name,x in artifacts.items(): assert sha(x['path'])==x['sha256'],name+' artifact changed'
    assert sha(ROOT/'lab/typst-original/target/release/typst')==VANILLA_SHA
    return artifacts

def micro(rounds):
    quiet(); artifacts=identity_check()
    for run in rounds:
        for name in TARGETS:
            identity_check()
            destination=OUT/'criterion'/f'r{run}'/name
            if resume_or_retire(f'micro-r{run}-{name}',destination):continue
            destination.mkdir(parents=True,exist_ok=False)
            cmd=[artifacts[name]['path'],'--bench']
            if name not in {'observation_bench','stdlib_bench'}: cmd+=['--sample-size','30','--warm-up-time','0.5','--measurement-time','1.5','--nresamples','10000','--noplot']
            monitored(cmd,f'micro-r{run}-{name}',True,{'CRITERION_HOME':str(destination)})
            identity_check()
    print('Microbenchmark rounds finished',flush=True)

def e2e(rounds,instrumented):
    quiet(); identity_check()
    from e2e import run
    run(rounds,instrumented)

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('action',choices=['prepare','build','micro','e2e']);parser.add_argument('--rounds',nargs='+',type=int,default=[1,2,3]);parser.add_argument('--instrumented',action='store_true');a=parser.parse_args()
    if a.action=='prepare':prepare()
    elif a.action=='build':build()
    elif a.action=='micro':micro(a.rounds)
    else:e2e(a.rounds,a.instrumented)
