"""External release probe: time paged layout calls without changing L0/L1-L4.

Run from any directory with Python 3; requires GDB, nm, pdfinfo, pdftotext.
Outputs individual samples, logs, fixtures and provenance under /tmp.
"""
import hashlib
import json
import os
import resource
from pathlib import Path
import statistics
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = Path('/tmp/tekt-grid-pagination-cost-2026-10-05')
OUT.mkdir(exist_ok=True)
TEKT = ROOT / 'target/release/typst'
VANILLA = ROOT / 'lab/typst-original/target/release/typst'
ENV = dict(os.environ, SOURCE_DATE_EPOCH='0', RAYON_NUM_THREADS='1')
CPU = min(os.sched_getaffinity(0))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)


def provenance():
    return dict(utc=time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime()),
                head=git('rev-parse', 'HEAD').strip(),
                status=git('status', '--porcelain=v1', '--untracked-files=all'),
                diff_stat=git('diff', 'HEAD', '--stat'),
                tracked_diff_sha256=hashlib.sha256(git('diff', 'HEAD').encode()).hexdigest(),
                binary_sha256=dict(tekt=sha(TEKT), vanilla=sha(VANILLA)))


symbols = [line.split()[-1] for line in subprocess.check_output(
    ['nm', str(TEKT)], text=True).splitlines()
    if 'layout_with_realized_math_callbacks_from_with_metrics' in line]
assert len(symbols) == 1, symbols
GDB = OUT / 'layout.gdb'
GDB.write_text('''set pagination off
set confirm off
set print thread-events off
python
import gdb, time, json
spans = []
def cpu_ns():
    pid = gdb.selected_inferior().pid
    tid = gdb.selected_thread().ptid[1]
    with open(f'/proc/{pid}/task/{tid}/schedstat') as source:
        return int(source.read().split()[0])
class End(gdb.FinishBreakpoint):
    def __init__(self, index):
        super().__init__(internal=True)
        self.silent = True
        self.index = index
    def stop(self):
        spans[self.index]['end_ns'] = time.perf_counter_ns()
        spans[self.index]['end_cpu_ns'] = cpu_ns()
        return False
class Start(gdb.Breakpoint):
    def stop(self):
        index = len(spans)
        End(index)
        spans.append({'start_ns': time.perf_counter_ns(), 'start_cpu_ns': cpu_ns()})
        return False
Start(''' + repr(symbols[0]) + ''', internal=True)
def done(event):
    print('PAGED_SPANS ' + json.dumps(spans))
gdb.events.exited.connect(done)
end
run
quit
''')


def fixture(rows):
    path = OUT / f'grid{rows}.typ'
    cells = ',\n'.join(f'[C{r:02d}{c}]' for r in range(rows) for c in range(4))
    path.write_text('#set page(width: 500pt, height: 3500pt, margin: 30pt)\n'
                    '#set text(font: "Libertinus Serif", size: 10pt)\n'
                    f'#table(columns: 4, rows: {rows},\n{cells}\n)\n')
    return path


def run(source, mode, index):
    prefix = OUT / f'{source.stem}-{mode}-{index}'
    pdf, timing = prefix.with_suffix('.pdf'), prefix.with_suffix('.json')
    engine = VANILLA if mode == 'vanilla' else TEKT
    command = [str(engine), 'compile', str(source), str(pdf), '--root', str(OUT),
               '--pdf-tagged=false', '--pretty=false']
    if mode == 'vanilla':
        command += ['--jobs', '1']
    else:
        command += ['--timings-json', str(timing)]
    if mode == 'gdb':
        command = ['gdb', '-q', '-nx', '-batch', '-x', str(GDB), '--args', *command]
    started = time.perf_counter_ns()
    usage_before = resource.getrusage(resource.RUSAGE_CHILDREN)
    proc = subprocess.run(command, cwd=ROOT, env=ENV, text=True,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                          preexec_fn=lambda: os.sched_setaffinity(0, {CPU}), timeout=30)
    wall_ms = (time.perf_counter_ns() - started) / 1e6
    usage_after = resource.getrusage(resource.RUSAGE_CHILDREN)
    prefix.with_suffix('.log').write_text(proc.stdout + proc.stderr)
    assert proc.returncode == 0, proc.stderr
    assert pdf.exists(), proc.stdout + proc.stderr
    sample = dict(mode=mode, index=index, command=command, wall_ms=wall_ms,
                  child_cpu_ms=1000 * (usage_after.ru_utime + usage_after.ru_stime -
                                      usage_before.ru_utime - usage_before.ru_stime))
    sample['text_sha256'] = hashlib.sha256(subprocess.check_output(
        ['pdftotext', '-raw', str(pdf), '-'])).hexdigest()
    bbox = subprocess.check_output(['pdftotext', '-bbox', str(pdf), '-'], text=True)
    sample['geometry_sha256'] = hashlib.sha256(
        bbox[bbox.index('<body>'):].encode()).hexdigest()
    sample['pdfinfo'] = subprocess.check_output(['pdfinfo', str(pdf)], text=True)
    if mode != 'vanilla':
        sample['timings'] = json.loads(timing.read_text())
    if mode == 'gdb':
        lines = [s for s in proc.stdout.splitlines() if s.startswith('PAGED_SPANS ')]
        assert len(lines) == 1, proc.stdout
        spans = json.loads(lines[0].removeprefix('PAGED_SPANS '))
        assert len(spans) == 2 and all('end_ns' in s for s in spans), spans
        sample['spans'] = spans
        sample['first_layout_ms'], sample['second_layout_ms'] = [
            (s['end_ns'] - s['start_ns']) / 1e6 for s in spans]
        sample['between_layouts_ms'] = (spans[1]['start_ns'] - spans[0]['end_ns']) / 1e6
        sample['second_cycle_ms'] = sample['between_layouts_ms'] + sample['second_layout_ms']
        sample['first_layout_cpu_ms'], sample['second_layout_cpu_ms'] = [
            (s['end_cpu_ns'] - s['start_cpu_ns']) / 1e6 for s in spans]
        sample['between_layouts_cpu_ms'] = (spans[1]['start_cpu_ns'] - spans[0]['end_cpu_ns']) / 1e6
        sample['second_cycle_cpu_ms'] = sample['between_layouts_cpu_ms'] + sample['second_layout_cpu_ms']
    return sample


def main():
    before = provenance()
    sources = {rows: fixture(rows) for rows in (1, 20, 80)}
    samples = []
    summary = {}
    for rows, source in sources.items():
        for warm in range(2):
            for mode in ('tekt', 'vanilla', 'gdb'):
                run(source, mode, f'w{warm}')
        data = []
        for index in range(9):
            order = ('tekt', 'vanilla', 'gdb') if index % 2 == 0 else ('gdb', 'vanilla', 'tekt')
            for mode in order:
                sample = dict(rows=rows, **run(source, mode, index))
                data.append(sample)
                samples.append(sample)
        med = lambda mode, key: statistics.median(s[key] for s in data if s['mode'] == mode)
        summary[rows] = {key: med('gdb', key) for key in (
            'first_layout_ms', 'second_layout_ms', 'between_layouts_ms', 'second_cycle_ms',
            'first_layout_cpu_ms', 'second_layout_cpu_ms', 'between_layouts_cpu_ms', 'second_cycle_cpu_ms')}
        summary[rows]['wall_ms'] = {mode: med(mode, 'wall_ms') for mode in ('tekt', 'vanilla')}
        summary[rows]['child_cpu_ms'] = {mode: med(mode, 'child_cpu_ms') for mode in ('tekt', 'vanilla')}
        summary[rows]['paired_cli_delta_ms'] = statistics.median(
            next(s['wall_ms'] for s in data if s['index'] == i and s['mode'] == 'tekt') -
            next(s['wall_ms'] for s in data if s['index'] == i and s['mode'] == 'vanilla')
            for i in range(9))
        summary[rows]['timings_median'] = {mode: {
            key: statistics.median(s['timings'][key] for s in data if s['mode'] == mode)
            for key in data[0 if data[0]['mode'] != 'vanilla' else 1]['timings']
        } for mode in ('tekt', 'gdb')}
        summary[rows]['text_equal_all_modes'] = len({s['text_sha256'] for s in data}) == 1
        summary[rows]['gdb_geometry_equal_tekt'] = len({s['geometry_sha256'] for s in data
                                                      if s['mode'] != 'vanilla'}) == 1
        print(rows, json.dumps(summary[rows]), flush=True)
    result = dict(before=before, after=provenance(), cpu=CPU,
                  fixture_sha256={rows: sha(source) for rows, source in sources.items()},
                  probe_sha256=sha(Path(__file__)), gdb_sha256=sha(GDB), symbols=symbols,
                  summary=summary, samples=samples)
    (OUT / 'results.json').write_text(json.dumps(result, indent=2) + '\n')


if __name__ == '__main__':
    main()
