"""Paired fresh-process CLI measurements for scaled item-7 cases."""
import datetime, gzip, hashlib, json, os, platform, statistics, subprocess, tempfile, time
from pathlib import Path
D = Path(__file__).resolve().parent
R = D.parents[2]
F = D / 'fixtures'
FONT = R / '00_nucleo/diagnosticos/performance-baseline/fonts'
BINS = {'tekt': R / 'target/release/typst', 'vanilla': R / 'lab/typst-original/target/release/typst'}
CPU = min(os.sched_getaffinity(0))
ENV = os.environ.copy()
for key in list(ENV):
    if key.startswith(('TYPST_', 'CRYSTALLINE_', 'CARGO_PROFILE_')): ENV.pop(key)
ENV.update(RAYON_NUM_THREADS='1', SOURCE_DATE_EPOCH='0', NO_COLOR='1', TZ='UTC', LC_ALL='C')
def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def capture(args): return subprocess.check_output(args, cwd=R)
def run(bin_name, fixture, out, label):
    pdf = out / f'{label}.pdf'
    usage = out / f'{label}.usage'
    cmd = ['/usr/bin/time', '-f', '%M %U %S', '-o', str(usage), str(BINS[bin_name]),
           'compile', str(fixture), str(pdf), '--root', str(F), '--pdf-tagged=false',
           '--pretty=false', '--font-path', str(FONT)]
    if bin_name == 'vanilla': cmd += ['--jobs', '1']
    start = time.perf_counter_ns()
    p = subprocess.run(cmd, cwd=R, env=ENV, capture_output=True, text=True, timeout=120,
                       preexec_fn=lambda: os.sched_setaffinity(0, {CPU}))
    wall = (time.perf_counter_ns() - start) / 1e6
    if p.returncode: raise RuntimeError((label, cmd, p.returncode, p.stderr))
    rss, user, system = usage.read_text().split()
    return dict(engine=bin_name, fixture=fixture.stem, utc=utc(), command=cmd,
                wall_ms=wall, peak_rss_kib=int(rss), user_s=float(user), system_s=float(system),
                stderr=p.stderr, pdf_sha256=sha(pdf))

def main():
    fixtures = sorted(F.glob('*.typ'))
    status = capture(['git', 'status', '--short', '--untracked-files=all'])
    stat = capture(['git', 'diff', 'HEAD', '--stat'])
    (D / 'measurement-git-status.txt').write_bytes(status)
    (D / 'measurement-git-diff-stat.txt').write_bytes(stat)
    binary_sha = {k: sha(v) for k, v in BINS.items()}
    assert binary_sha['vanilla'] == '7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
    inputs = {str(p.relative_to(R)): sha(p) for p in [*fixtures, *sorted(FONT.glob('*')),
                                                       D/'measure.py', D/'generate.py'] if p.is_file()}
    result = dict(utc_start=utc(), head=capture(['git', 'rev-parse', 'HEAD']).decode().strip(),
                  working_tree='uncommitted', git_status_sha256=hashlib.sha256(status).hexdigest(),
                  git_diff_stat_sha256=hashlib.sha256(stat).hexdigest(), binaries=binary_sha,
                  inputs=inputs, environment={k: ENV[k] for k in ('RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','NO_COLOR','TZ','LC_ALL')},
                  cpu=CPU, platform=platform.platform(), rounds=3, pairs_per_round=6, samples=[])
    with tempfile.TemporaryDirectory(prefix='item7-scaled-') as d:
        out = Path(d)
        for round_no in range(3):
            for fixture in fixtures:
                for pair in range(-1, 6):
                    order = ('tekt','vanilla') if (round_no + pair) % 2 == 0 else ('vanilla','tekt')
                    for engine in order:
                        row = run(engine, fixture, out, f'{round_no}-{fixture.stem}-{pair}-{engine}')
                        if pair >= 0: result['samples'].append(dict(round=round_no, pair=pair, **row))
                print(round_no, fixture.stem, 'done', flush=True)
    assert binary_sha == {k: sha(v) for k,v in BINS.items()}
    assert inputs == {str(p.relative_to(R)): sha(p) for p in [*fixtures, *sorted(FONT.glob('*')),
                                                                 D/'measure.py', D/'generate.py'] if p.is_file()}
    result['utc_end'] = utc()
    result['summary'] = {}
    for fixture in fixtures:
        name = fixture.stem
        case = {}
        for engine in BINS:
            vals = [s for s in result['samples'] if s['fixture'] == name and s['engine'] == engine]
            case[engine] = dict(n=len(vals), wall_median_ms=statistics.median(s['wall_ms'] for s in vals),
                                rss_median_kib=statistics.median(s['peak_rss_kib'] for s in vals),
                                rss_range_kib=[min(s['peak_rss_kib'] for s in vals), max(s['peak_rss_kib'] for s in vals)])
        pairs = []
        for round_no in range(3):
            for pair in range(6):
                p = {s['engine']:s for s in result['samples'] if s['fixture']==name and s['round']==round_no and s['pair']==pair}
                pairs.append(dict(round=round_no, pair=pair,
                                  wall_ratio=p['tekt']['wall_ms']/p['vanilla']['wall_ms']-1,
                                  rss_ratio=p['tekt']['peak_rss_kib']/p['vanilla']['peak_rss_kib']-1,
                                  wall_delta_ms=p['tekt']['wall_ms']-p['vanilla']['wall_ms'],
                                  rss_delta_kib=p['tekt']['peak_rss_kib']-p['vanilla']['peak_rss_kib']))
        case['paired'] = dict(wall_median_ratio=statistics.median(p['wall_ratio'] for p in pairs),
                              rss_median_ratio=statistics.median(p['rss_ratio'] for p in pairs),
                              wall_round_median_ratio=[statistics.median(p['wall_ratio'] for p in pairs if p['round']==r) for r in range(3)],
                              rss_round_median_ratio=[statistics.median(p['rss_ratio'] for p in pairs if p['round']==r) for r in range(3)],
                              wall_delta_range_ms=[min(p['wall_delta_ms'] for p in pairs),max(p['wall_delta_ms'] for p in pairs)],
                              rss_delta_range_kib=[min(p['rss_delta_kib'] for p in pairs),max(p['rss_delta_kib'] for p in pairs)])
        result['summary'][name] = case
    with gzip.open(D/'measurements.json.gz','wt') as f: json.dump(result,f,ensure_ascii=False,indent=2)
    for name,case in result['summary'].items():
        print(name,case['tekt']['wall_median_ms'],case['vanilla']['wall_median_ms'],
              case['paired']['wall_median_ratio'],case['tekt']['rss_median_kib'],
              case['vanilla']['rss_median_kib'],case['paired']['rss_median_ratio'],flush=True)
if __name__ == '__main__': main()
