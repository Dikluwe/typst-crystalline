"""Compare the previous and corrected Tekt watch binaries on identical cycles."""
import datetime, difflib, hashlib, json, os, statistics, subprocess, tempfile, time, xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
BINS = {'tekt': Path('/tmp/tekt-watch-context/typst'), 'vanilla': ROOT/'lab/typst-original/target/release/typst'}
CPU = min(os.sched_getaffinity(0))
ENV = os.environ.copy()
for key in list(ENV):
    if key.startswith(('TYPST_', 'CRYSTALLINE_', 'CARGO_PROFILE_')) or key in ('LD_PRELOAD','CPU_PROFILE_OUT'): ENV.pop(key)
ENV.update(RAYON_NUM_THREADS='1', SOURCE_DATE_EPOCH='0', TZ='UTC', NO_COLOR='1', LC_ALL='C')

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

