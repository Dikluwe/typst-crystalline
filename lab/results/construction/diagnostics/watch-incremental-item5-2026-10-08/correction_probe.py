"""Local phase probe; sequential watch runs, no productive source changes."""
import datetime, hashlib, json, os, statistics, subprocess, tempfile, time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
HERE=Path(__file__).resolve().parent
BIN=ROOT/'target/release/typst'
CPU=min(os.sched_getaffinity(0))
ENV=os.environ.copy()
for key in list(ENV):
    if key.startswith(('TYPST_','CRYSTALLINE_')): ENV.pop(key)
ENV.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',CRYSTALLINE_PDF_FIXED_EPOCH='0',TZ='UTC',NO_COLOR='1')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def gen(p):
    if not p.exists():return None
    s=p.stat();return (s.st_ino,s.st_mtime_ns,s.st_size)
def wait(p,old,proc):
    end=time.monotonic()+10
    while time.monotonic()<end:
        if proc.poll() is not None:raise RuntimeError(f'watch exited {proc.returncode}')
        v=gen(p)
        if v is not None and v!=old:return
        time.sleep(.002)
    raise TimeoutError(str(p))
prefix='#set page(width: 500pt, height: 700pt, margin: 30pt)\n'
loop='#for i in range(8) [Item #i ]\n'
variants={
 'empty':'#let x = 1\n',
 'plain':'Hello\n',
 'set_size':'#set text(size: 10pt)\nHello\n',
 'set_font':'#set text(font: "Libertinus Serif")\nHello\n',
 'no_import_loop':prefix+'Label Alpha. Value 1999.\n'+loop,
 'minimum_import_loop':prefix+'#import "lib.typ": value\nLabel Alpha. Value #value.\n'+loop,
 'heavy_import_loop':prefix+'#import "lib.typ": value\nLabel Alpha. Value #value.\n'+loop,
}
stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT)
status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=ROOT)
(HERE/'correction-git-diff-stat.txt').write_bytes(stat)
(HERE/'correction-git-status.txt').write_bytes(status)
receipt={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
 'binary_sha256':sha(BIN),'script_sha256':sha(Path(__file__)),'git_diff_stat_sha256':hashlib.sha256(stat).hexdigest(),
 'git_status_sha256':hashlib.sha256(status).hexdigest(),'cpu':CPU,'variants':{}}
with tempfile.TemporaryDirectory(prefix='item5-correction-') as d:
    base=Path(d)
    for name,src in variants.items():
        p=base/name;p.mkdir();main=p/'main.typ';pdf=p/'out.pdf';tim=p/'timing.json';main.write_text(src)
        if name.endswith('import_loop'):
            lib='#let value = 1999\n' if name.startswith('minimum') else ''.join(f'#let v{i} = {i}\n' for i in range(2000))+'#let value = v1999\n'
            (p/'lib.typ').write_text(lib)
        cmd=[str(BIN),'watch',str(main),str(pdf),'--root',str(p),'--pdf-tagged=false','--pretty=false','--timings-json',str(tim)]
        rows=[]
        with (p/'log').open('w') as log:
            proc=subprocess.Popen(cmd,cwd=ROOT,env=ENV,stdout=log,stderr=log,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}))
            try:
                for cycle in range(4):
                    old=gen(pdf)
                    if cycle:
                        t=p/'main.new';t.write_text(src);os.replace(t,main)
                    wait(pdf,old,proc)
                    end=time.monotonic()+5
                    while True:
                        try:values=json.loads(tim.read_text());break
                        except (FileNotFoundError,json.JSONDecodeError):
                            if time.monotonic()>end:raise TimeoutError('timings')
                            time.sleep(.002)
                    rows.append(values);time.sleep(.15)
            finally:
                proc.terminate();proc.wait(timeout=5)
        receipt['variants'][name]={'source_sha256':hashlib.sha256(src.encode()).hexdigest(),
            'import_sha256':sha(p/'lib.typ') if (p/'lib.typ').exists() else None,
            'command':cmd,'cycles':rows,
            'median_warm_eval_ms':statistics.median(x['eval_ms'] for x in rows[1:]),
            'median_warm_total_ms':statistics.median(x['total_ms'] for x in rows[1:])}
        print(name,receipt['variants'][name]['median_warm_eval_ms'],receipt['variants'][name]['median_warm_total_ms'],flush=True)
receipt['utc_end']=utc()
(HERE/'correction-probe.json').write_text(json.dumps(receipt,indent=2)+'\n')
