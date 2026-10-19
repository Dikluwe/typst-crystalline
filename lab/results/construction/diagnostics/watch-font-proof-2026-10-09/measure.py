"""Compare the previous and corrected Tekt watch binaries on identical cycles."""
import datetime, difflib, hashlib, json, os, statistics, subprocess, tempfile, time, xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
BINS = {'after': ROOT/'target/release/typst', 'before': Path('/tmp/tekt-item5-before-font-proof')}
CPU = min(os.sched_getaffinity(0))
ENV = os.environ.copy()
for key in list(ENV):
    if key.startswith(('TYPST_', 'CRYSTALLINE_')): ENV.pop(key)
ENV.update(RAYON_NUM_THREADS='1', SOURCE_DATE_EPOCH='0', CRYSTALLINE_PDF_FIXED_EPOCH='0', TZ='UTC', NO_COLOR='1')

def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def cpu_ns(pid):
    return sum(int(p.read_text().split()[0]) for p in Path(f'/proc/{pid}/task').glob('*/schedstat'))
def mem(pid):
    data = dict(s.split(':',1) for s in Path(f'/proc/{pid}/status').read_text().splitlines() if ':' in s)
    return {k: int(data[k].split()[0]) for k in ('VmRSS','VmHWM')}
def generation(path):
    if not path.exists(): return None
    s=path.stat(); return (s.st_ino,s.st_mtime_ns,s.st_size)
def wait_pdf(proc,path,old,seconds):
    end=time.monotonic()+seconds
    while time.monotonic()<end:
        if proc.poll() is not None: raise RuntimeError(f'watch exited: {proc.returncode}')
        current=generation(path)
        if current != old and current is not None:
            with path.open('rb') as f:
                f.seek(max(0,path.stat().st_size-32))
                if b'%%EOF' in f.read(): return True
        time.sleep(.002)
    return False

def word_boxes(pdf):
    root=ET.fromstring(subprocess.check_output(['pdftotext','-bbox',str(pdf),'-']))
    ns={'x':'http://www.w3.org/1999/xhtml'}
    return [{'text':w.text,**{k:float(v) for k,v in w.attrib.items()}} for w in root.findall('.//x:word',ns)]

def main_source(word,rows):
    return ('#set page(width: 500pt, height: 700pt, margin: 30pt)\n'
            '#set text(font: "Libertinus Serif", size: 10pt)\n'
            '#import "lib.typ": value\n'
            f'Label {word}. Value #value.\n'
            f'#for i in range({rows}) [Item #i ]\n')

def run(engine,round_no,base):
    folder=base/f'{round_no}-{engine}';folder.mkdir()
    lib=folder/'lib.typ';lib.write_text(''.join(f'#let v{i} = {i}\n' for i in range(2000))+'#let value = v1999\n')
    main=folder/'main.typ';pdf=folder/'main.pdf';log=folder/'watch.log'
    word='Alpha';rows=8;main.write_text(main_source(word,rows))
    cmd=[str(BINS[engine]),'watch',str(main),str(pdf),'--root',str(folder),'--pdf-tagged=false','--pretty=false']
    samples=[]
    with log.open('w') as output:
        start=time.perf_counter_ns()
        proc=subprocess.Popen(cmd,cwd=ROOT,env=ENV,stdout=output,stderr=output,preexec_fn=lambda: os.sched_setaffinity(0,{CPU}))
        try:
            if not wait_pdf(proc,pdf,None,15): raise RuntimeError('initial PDF not published')
            time.sleep(.15)
            initial={'startup_wall_ms':(time.perf_counter_ns()-start)/1e6,'cpu_ms':cpu_ns(proc.pid)/1e6,**mem(proc.pid),'pdf_sha256':sha(pdf)}
            for cycle in range(12):
                kind=('metadata_touch','same_bytes','small','structural')[cycle%4]
                time.sleep(.18)
                old=generation(pdf);before_cpu=cpu_ns(proc.pid);before_mem=mem(proc.pid)
                output.flush();before_log=log.stat().st_size
                started=time.perf_counter_ns()
                if kind=='metadata_touch':
                    old_stat=main.stat();os.utime(main,ns=(old_stat.st_atime_ns,old_stat.st_mtime_ns+1_000_000_000))
                elif kind=='same_bytes':
                    tmp=folder/'main.new';tmp.write_bytes(main.read_bytes());os.replace(tmp,main)
                else:
                    if kind=='small':word='Bravo' if word=='Alpha' else 'Alpha'
                    else:rows=12 if rows==8 else 8
                    tmp=folder/'main.new';tmp.write_text(main_source(word,rows));os.replace(tmp,main)
                changed=wait_pdf(proc,pdf,old,.8 if kind=='metadata_touch' else 15)
                latency_ms=(time.perf_counter_ns()-started)/1e6 if changed else None
                time.sleep(.12)
                text=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True)
                assert word in text and text.count('Item') == rows,(engine,cycle,text)
                sample={'cycle':cycle,'kind':kind,'published':changed,'latency_ms':latency_ms,
                        'cpu_ms':(cpu_ns(proc.pid)-before_cpu)/1e6,'before_mem_kib':before_mem,
                        'after_mem_kib':mem(proc.pid),'source_sha256':sha(main),
                        'pdf_sha256':sha(pdf),'text':text,'words':word_boxes(pdf),'log_delta':log.stat().st_size-before_log}
                samples.append(sample)
                print(round_no,engine,cycle,kind,changed,round(sample['cpu_ms'],2),sample['after_mem_kib']['VmRSS'],flush=True)
        finally:
            proc.terminate()
            try:proc.wait(timeout=5)
            except subprocess.TimeoutExpired:proc.kill();proc.wait()
    return {'command':cmd,'initial':initial,'samples':samples,'log':log.read_text()}


def main():
    state={'utc_start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
           'binaries':{k:sha(v) for k,v in BINS.items()},'cpu':CPU,'runs':[]}
    stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT)
    status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=ROOT)
    (HERE/'git-diff-head-stat.txt').write_bytes(stat);(HERE/'git-status-short.txt').write_bytes(status)
    state['git_diff_stat_sha256']=hashlib.sha256(stat).hexdigest();state['git_status_sha256']=hashlib.sha256(status).hexdigest()
    with tempfile.TemporaryDirectory(prefix='item5-watch-') as d:
        base=Path(d)
        for round_no in range(3):
            engines=('after','before') if round_no%2==0 else ('before','after')
            for engine in engines: state['runs'].append({'round':round_no,'engine':engine,**run(engine,round_no,base)})
    state['text_differences']=[]
    for round_no in range(3):
        a,b=[r for r in state['runs'] if r['round']==round_no]
        for i,(x,y) in enumerate(zip(a['samples'],b['samples'])):
            if x['text']!=y['text']:
                state['text_differences'].append({'round':round_no,'cycle':i,'diff':''.join(difflib.unified_diff(x['text'].splitlines(True),y['text'].splitlines(True)))})
    state['word_comparison']=[]
    for round_no in range(3):
        pair={r['engine']:r for r in state['runs'] if r['round']==round_no}
        for i,(x,y) in enumerate(zip(pair['after']['samples'],pair['before']['samples'])):
            a,b=x['words'],y['words'];same=[w['text'] for w in a]==[w['text'] for w in b]
            delta=max((abs(u[k]-v[k]) for u,v in zip(a,b) for k in ('xMin','xMax','yMin','yMax')),default=0) if same else None
            state['word_comparison'].append({'round':round_no,'cycle':i,'same_words':same,'max_bbox_delta_pt':delta})
    summary={}
    for kind in ('metadata_touch','same_bytes','small','structural'):
        summary[kind]={}
        for engine in BINS:
            rows=[s for r in state['runs'] if r['engine']==engine for s in r['samples'] if s['kind']==kind]
            summary[kind][engine]={'published_count':sum(s['published'] for s in rows),'cycles':len(rows),
                'cpu_median_ms':statistics.median(s['cpu_ms'] for s in rows),
                'latency_median_ms':statistics.median(s['latency_ms'] for s in rows if s['latency_ms'] is not None) if any(s['published'] for s in rows) else None,
                'rss_median_kib':statistics.median(s['after_mem_kib']['VmRSS'] for s in rows),
                'rss_final_kib':next(r for r in reversed(state['runs']) if r['engine']==engine)['samples'][-1]['after_mem_kib']['VmRSS']}
    state['summary']=summary;state['utc_end']=now()
    assert {k:sha(v) for k,v in BINS.items()}==state['binaries']
    (HERE/'measurements.json').write_text(json.dumps(state,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
