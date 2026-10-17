"""Paired fresh-process RSS and wall growth for numeric arrays and dictionaries."""
import datetime, hashlib, itertools, json, os, statistics, subprocess, tempfile, time
from pathlib import Path

D = Path(__file__).resolve().parent
R = D.parents[2]
F = D / 'fixtures'
BINS = {'before': Path('/tmp/tekt-item6-experiment/tekt-before'), 'after': R/'target/release/typst'}
FONT = R/'00_nucleo/diagnosticos/performance-baseline/fonts'
CPU = min(os.sched_getaffinity(0))
ENV = os.environ.copy()
for key in list(ENV):
    if key.startswith(('TYPST_', 'CRYSTALLINE_', 'CARGO_PROFILE_')): ENV.pop(key)
ENV.update(RAYON_NUM_THREADS='1', SOURCE_DATE_EPOCH='0', NO_COLOR='1', TZ='UTC', LC_ALL='C')

def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()

def run(engine, fixture, output_dir, label):
    pdf = output_dir / f'{label}.pdf'
    usage = output_dir / f'{label}.usage'
    cmd = ['/usr/bin/time','-f','%M %U %S','-o',str(usage),str(BINS[engine]),
           'compile',str(fixture),str(pdf),'--root',str(F),
           '--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]
    
    start = time.perf_counter_ns()
    result = subprocess.run(cmd, cwd=R, env=ENV, capture_output=True, text=True,
                            timeout=60, preexec_fn=lambda: os.sched_setaffinity(0,{CPU}))
    wall = (time.perf_counter_ns()-start)/1e6
    if result.returncode: raise RuntimeError((cmd,result.stderr))
    rss,user,system = usage.read_text().split()
    extracted = subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True).strip()
    return dict(engine=engine, fixture=fixture.stem, utc=utc(), command=cmd,
                wall_ms=wall, peak_rss_kib=int(rss), user_s=float(user), system_s=float(system),
                text=extracted, stderr=result.stderr, pdf_sha256=sha(pdf))

def main():
    fixtures = sorted(F.glob('*.typ'))
    stat = subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R)
    status = subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R)
    (D/'dispatch-experiment-git-diff-head-stat.txt').write_bytes(stat)
    (D/'dispatch-experiment-git-status-short.txt').write_bytes(status)
    state = dict(utc_start=utc(), head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),
                 working_tree='uncommitted', git_diff_stat_sha256=hashlib.sha256(stat).hexdigest(),
                 git_status_sha256=hashlib.sha256(status).hexdigest(),
                 binaries={k:sha(v) for k,v in BINS.items()},
                 inputs={str(p.relative_to(R)):sha(p) for p in fixtures},
                 fonts={str(p.relative_to(R)):sha(p) for p in FONT.iterdir() if p.is_file()},
                 environment={k:ENV[k] for k in ('RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','NO_COLOR','TZ','LC_ALL')},
                 cpu=CPU, samples=[])
    with tempfile.TemporaryDirectory(prefix='item6-collections-') as tmp:
        out = Path(tmp)
        for round_no in range(3):
            for fixture in fixtures:
                for pair in range(-1,4):
                    order = ('before','after') if (round_no+pair)%2 == 0 else ('after','before')
                    for engine in order:
                        sample = run(engine,fixture,out,f'{round_no}-{fixture.stem}-{pair}-{engine}')
                        if pair >= 0: state['samples'].append(dict(round=round_no,pair=pair,**sample))
                    print(round_no, fixture.stem, pair, 'done', flush=True)
    assert {k:sha(v) for k,v in BINS.items()} == state['binaries']
    assert {str(p.relative_to(R)):sha(p) for p in fixtures} == state['inputs']
    state['utc_end'] = utc()
    state['text_mismatches'] = []
    for r in range(3):
        for fixture in fixtures:
            for pair in range(4):
                samples=[s for s in state['samples'] if s['round']==r and s['fixture']==fixture.stem and s['pair']==pair]
                if samples[0]['text'] != samples[1]['text']:
                    state['text_mismatches'].append([r,fixture.stem,pair,samples[0]['text'],samples[1]['text']])
    state['summary']={}
    for fixture in fixtures:
        state['summary'][fixture.stem]={}
        for engine in BINS:
            rows=[s for s in state['samples'] if s['fixture']==fixture.stem and s['engine']==engine]
            state['summary'][fixture.stem][engine]={
                'rss_median_kib':statistics.median(s['peak_rss_kib'] for s in rows),
                'rss_min_kib':min(s['peak_rss_kib'] for s in rows),
                'rss_max_kib':max(s['peak_rss_kib'] for s in rows),
                'wall_median_ms':statistics.median(s['wall_ms'] for s in rows),
                'samples':len(rows)}
    (D/'dispatch-experiment-measurements.json').write_text(json.dumps(state,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps(state['summary'],indent=2), flush=True)
if __name__ == '__main__': main()
