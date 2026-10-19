"""Compare fresh-process workloads and externally inspect current Tekt.

No compiler edits. GDB probes use inferior-thread schedstat CPU, not GDB CPU.
Requires nm, c++filt, GDB, pdfinfo and pdftotext; ptrace must be permitted.
"""
import hashlib
import json
import os
from pathlib import Path
import resource
import statistics
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = Path('/tmp/tekt-macro-micro-2026-10-05')
OUT.mkdir(exist_ok=True)
BIN = {'tekt': ROOT / 'target/release/typst',
       'vanilla': ROOT / 'lab/typst-original/target/release/typst'}
ENV = dict(os.environ, SOURCE_DATE_EPOCH='0', RAYON_NUM_THREADS='1')
CPU = min(os.sched_getaffinity(0))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def provenance():
    git = lambda *args: subprocess.check_output(['git', *args], cwd=ROOT, text=True)
    return dict(utc=time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime()),
                head=git('rev-parse', 'HEAD').strip(),
                status=git('status', '--porcelain=v1', '--untracked-files=all'),
                diff_stat=git('diff', 'HEAD', '--stat'),
                binaries={name: sha(path) for name, path in BIN.items()})


def sources():
    page = '#set page(width: 500pt, height: 3500pt, margin: 30pt)\n'
    text = '#set text(font: "Libertinus Serif", size: 10pt)\n'
    fixtures = dict(
        shape=page + '#rect(width: 100pt, height: 40pt, fill: blue)\n',
        hello=page + text + 'Hello World\n',
        context=page + text + '= Title\n#context counter(page).display()\nHello World\n',
        paragraphs=page + text + '#set par(linebreaks: "simple")\n' +
        '\n\n'.join('Alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu. ' * 3
                      for _ in range(60)) + '\n',
        grid320=page + text + '#table(columns: 4, rows: 80,\n' +
        ',\n'.join(f'[C{r:02d}{c}]' for r in range(80) for c in range(4)) + '\n)\n',
    )
    for count in (1000, 10000, 30000):
        fixtures[f'eval{count}'] = (page + '#let total = 0\n' +
            f'#for i in range({count}) {{ total += i }}\n' +
            f'#assert(total == {count * (count - 1) // 2})\n' +
            '#rect(width: 100pt, height: 40pt, fill: blue)\n')
    result = {}
    for name, content in fixtures.items():
        path = OUT / f'{name}.typ'
        path.write_text(content)
        result[name] = path
    return result


def gdb_script():
    raw = [line.split()[-1] for line in subprocess.check_output(
        ['nm', str(BIN['tekt'])], text=True).splitlines() if len(line.split()) == 3]
    names = subprocess.check_output(['c++filt'], input='\n'.join(raw), text=True).splitlines()
    needles = dict(
        catalog='typst_infra::world::fonts::<impl typst_infra::world::SystemWorld>::with_fonts_and_system::{{closure}}::',
        system_fonts='typst_infra::fontdb::load_system_fonts::',
        embedded_fonts='typst_infra::embedded_fonts::load_embedded_fonts::',
        eval_pass='typst_core::compiler::eval::eval_with_full_error_target_and_features::{{closure}}::',
        stdlib='typst_core::compiler::stdlib::registry::make_stdlib_with_features::',
        layout='typst_infra::pipeline::paged::layout_with_realized_math_callbacks_from_with_metrics::',
        shape='typst_infra::shaper::traversal::shape_document::',
        reconcile='typst_infra::shaper::reconciliation::fix_line_positions_with_metrics::',
        pdf='typst_infra::export::export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard::',
    )
    symbols = {}
    for key, needle in needles.items():
        matches = [sym for sym, name in zip(raw, names) if name.startswith(needle)]
        assert len(matches) == 1, (key, matches)
        symbols[key] = matches[0]
    script = '''set pagination off
set confirm off
set print thread-events off
python
import gdb, time, json
spans = []
stack = []
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
        assert stack.pop() == self.index
        return False
class Start(gdb.Breakpoint):
    def __init__(self, symbol, label):
        super().__init__(symbol, internal=True)
        self.silent = True
        self.label = label
    def stop(self):
        index = len(spans)
        End(index)
        spans.append(dict(label=self.label, parent=stack[-1] if stack else None,
                          start_ns=time.perf_counter_ns(), start_cpu_ns=cpu_ns()))
        stack.append(index)
        return False
'''
    script += '\n'.join(f'Start({symbol!r}, {key!r})' for key, symbol in symbols.items())
    script += '''
def done(event):
    print('MACRO_SPANS ' + json.dumps(spans))
gdb.events.exited.connect(done)
end
run
quit
'''
    path = OUT / 'probe.gdb'
    path.write_text(script)
    return path, symbols


def run(name, source, mode, index, gdb):
    prefix = OUT / f'{name}-{mode}-{index}'
    pdf, timing = prefix.with_suffix('.pdf'), prefix.with_suffix('.json')
    engine = 'tekt' if mode == 'gdb' else mode
    command = [str(BIN[engine]), 'compile', str(source), str(pdf), '--root', str(OUT),
               '--pdf-tagged=false', '--pretty=false']
    if engine == 'tekt':
        command += ['--timings-json', str(timing)]
    else:
        command += ['--jobs', '1']
    if mode == 'gdb':
        command = ['gdb', '-q', '-nx', '-batch', '-x', str(gdb), '--args', *command]
    if pdf.exists():
        pdf.unlink()
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    started = time.perf_counter_ns()
    proc = subprocess.run(command, cwd=ROOT, env=ENV, text=True, stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE, timeout=60,
                          preexec_fn=lambda: os.sched_setaffinity(0, {CPU}))
    wall_ms = (time.perf_counter_ns() - started) / 1e6
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    prefix.with_suffix('.log').write_text(proc.stdout + proc.stderr)
    assert proc.returncode == 0 and pdf.exists(), (name, mode, proc.stdout, proc.stderr)
    info = subprocess.check_output(['pdfinfo', str(pdf)], text=True)
    bbox = subprocess.check_output(['pdftotext', '-bbox', str(pdf), '-'], text=True)
    data = dict(fixture=name, mode=mode, index=index, command=command, wall_ms=wall_ms,
                cpu_ms=1000 * (after.ru_utime + after.ru_stime - before.ru_utime - before.ru_stime),
                text_sha256=hashlib.sha256(subprocess.check_output(
                    ['pdftotext', '-raw', str(pdf), '-'])).hexdigest(),
                geometry_sha256=hashlib.sha256(bbox[bbox.index('<body>'):].encode()).hexdigest(),
                pdfinfo=info, diagnostics=proc.stderr)
    if engine == 'tekt':
        data['timings'] = json.loads(timing.read_text())
    if mode == 'gdb':
        lines = [s for s in proc.stdout.splitlines() if s.startswith('MACRO_SPANS ')]
        assert len(lines) == 1, proc.stdout
        data['spans'] = json.loads(lines[0].removeprefix('MACRO_SPANS '))
        for span in data['spans']:
            assert 'end_cpu_ns' in span, (name, span)
            span['cpu_ms'] = (span['end_cpu_ns'] - span['start_cpu_ns']) / 1e6
        assert sum(s['label'] == 'eval_pass' for s in data['spans']) == 2
        assert sum(s['label'] == 'stdlib' for s in data['spans']) == 2
    return data


def main():
    initial = provenance()
    fixtures = sources()
    gdb, symbols = gdb_script()
    samples, summary = [], {}
    for name, source in fixtures.items():
        for mode in ('tekt', 'vanilla', 'gdb'):
            run(name, source, mode, 'warm', gdb)
        cohort = []
        for i in range(6):
            modes = ('tekt', 'vanilla') if i % 2 == 0 else ('vanilla', 'tekt')
            for mode in modes:
                cohort.append(run(name, source, mode, i, gdb))
            if i % 2 == 0:
                cohort.append(run(name, source, 'gdb', i, gdb))
        samples.extend(cohort)
        summary[name] = {}
        for mode in ('tekt', 'vanilla'):
            data = [s for s in cohort if s['mode'] == mode]
            summary[name][mode] = {key: statistics.median(s[key] for s in data)
                                   for key in ('wall_ms', 'cpu_ms')}
        probes = [s for s in cohort if s['mode'] == 'gdb']
        summary[name]['probe'] = {label: {
            'calls': [sum(s['label'] == label for s in p['spans']) for p in probes],
            'cpu_sum_median_ms': statistics.median(sum(s['cpu_ms'] for s in p['spans']
                                                     if s['label'] == label) for p in probes)
        } for label in symbols}
        summary[name]['text_equal_engines'] = len({s['text_sha256'] for s in cohort}) == 1
        summary[name]['gdb_geometry_equal_tekt'] = len({s['geometry_sha256'] for s in cohort
                                                      if s['mode'] != 'vanilla'}) == 1
        summary[name]['geometry_equal_engines'] = len({s['geometry_sha256'] for s in cohort}) == 1
        print(name, json.dumps(summary[name]), flush=True)
        result = dict(before=initial, after=provenance(), cpu=CPU, symbols=symbols,
                      probe_sha256=sha(Path(__file__)),
                      fixture_sha256={name: sha(path) for name, path in fixtures.items()},
                      samples=samples, summary=summary)
        (OUT / 'results.json').write_text(json.dumps(result, indent=2) + '\n')


if __name__ == '__main__':
    main()
