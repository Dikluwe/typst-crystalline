"""Measure edit-to-published-PDF latency and live process thread CPU.

Only starts/stops its own local watch processes and edits generated fixtures.
Wait/polling costs belong to latency. CPU excludes sleeping and observer work.
"""
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = Path('/tmp/tekt-macro-micro-watch-2026-10-05')
OUT.mkdir(exist_ok=True)
BIN = dict(tekt=ROOT / 'target/release/typst',
           vanilla=ROOT / 'lab/typst-original/target/release/typst')
ENV = dict(os.environ, SOURCE_DATE_EPOCH='0', RAYON_NUM_THREADS='1', NO_COLOR='1')
CPU = min(os.sched_getaffinity(0))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def provenance():
    git = lambda *a: subprocess.check_output(['git', *a], cwd=ROOT, text=True)
    return dict(utc=time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime()),
                head=git('rev-parse', 'HEAD').strip(),
                status=git('status', '--porcelain=v1', '--untracked-files=all'),
                diff_stat=git('diff', 'HEAD', '--stat'),
                binaries={key: sha(value) for key, value in BIN.items()})


def cpu_runtime(pid):
    return {int(path.parent.name): int(path.read_text().split()[0])
            for path in Path(f'/proc/{pid}/task').glob('*/schedstat')}


def generation(path):
    if not path.exists():
        return None
    stat = path.stat()
    return stat.st_ino, stat.st_mtime_ns, stat.st_size


def wait_pdf(proc, pdf, previous):
    deadline = time.monotonic() + 15
    while time.monotonic() < deadline:
        assert proc.poll() is None, proc.returncode
        if generation(pdf) != previous and pdf.exists():
            with pdf.open('rb') as stream:
                stream.seek(max(0, pdf.stat().st_size - 32))
                if b'%%EOF' in stream.read():
                    return
        time.sleep(0.001)
    raise RuntimeError(f'No new complete PDF: {pdf}')


def source(name, revision):
    prefix = ('#set page(width: 500pt, height: 3500pt, margin: 30pt)\n'
              '#set text(font: "Libertinus Serif", size: 10pt)\n')
    if name == 'hello':
        return prefix + f'Hello World revision {revision}\n'
    return (prefix + '#table(columns: 4, rows: 80,\n' +
            ',\n'.join(f'[C{r:02d}{c}]' for r in range(80) for c in range(4)) +
            f'\n)\n// revision {revision}\n')


def run(name, engine):
    fixture, pdf, timing = (OUT / f'{name}-{engine}{ext}' for ext in ('.typ', '.pdf', '.json'))
    fixture.write_text(source(name, 0))
    if pdf.exists():
        pdf.unlink()
    command = [str(BIN[engine]), 'watch', str(fixture), str(pdf), '--root', str(OUT),
               '--pdf-tagged=false', '--pretty=false']
    if engine == 'tekt':
        command += ['--timings-json', str(timing)]
    else:
        command += ['--jobs', '1']
    samples = []
    with (OUT / f'{name}-{engine}.log').open('w') as log:
        started = time.perf_counter_ns()
        proc = subprocess.Popen(command, cwd=ROOT, env=ENV, stdout=log, stderr=log,
                                preexec_fn=lambda: os.sched_setaffinity(0, {CPU}))
        try:
            wait_pdf(proc, pdf, None)
            initial = dict(wall_ms=(time.perf_counter_ns() - started) / 1e6,
                           cpu_ms=sum(cpu_runtime(proc.pid).values()) / 1e6)
            for revision in range(1, 6):
                # Let both watchers arm after their previous publication.
                time.sleep(0.15)
                previous = generation(pdf)
                before = cpu_runtime(proc.pid)
                started = time.perf_counter_ns()
                fixture.write_text(source(name, revision))
                wait_pdf(proc, pdf, previous)
                elapsed = (time.perf_counter_ns() - started) / 1e6
                after = cpu_runtime(proc.pid)
                assert set(before) == set(after), (before, after)
                payload = json.loads(timing.read_text()) if engine == 'tekt' else None
                row = dict(fixture=name, engine=engine, revision=revision,
                           latency_ms=elapsed,
                           cpu_ms=sum(after[tid] - before[tid] for tid in after) / 1e6,
                           thread_ids=list(after), source_sha256=sha(fixture),
                           source_text=source(name, revision), timing=payload)
                row['text'] = subprocess.check_output(['pdftotext', '-raw', str(pdf), '-'], text=True)
                expected = (f'Hello World revision {revision}' if name == 'hello' else 'C000')
                assert expected in row['text'], row['text']
                samples.append(row)
        finally:
            proc.terminate()
            try:
                proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                proc.kill()
                proc.wait()
    return dict(command=command, initial=initial, samples=samples)


def main():
    before = provenance()
    runs = {name: {engine: run(name, engine) for engine in ('tekt', 'vanilla')}
            for name in ('hello', 'grid320')}
    summary = {name: {engine: {key: statistics.median(row[key] for row in run['samples'])
                              for key in ('latency_ms', 'cpu_ms')}
                     for engine, run in engines.items()} for name, engines in runs.items()}
    assert all(t['text'] == v['text'] for engines in runs.values()
               for t, v in zip(engines['tekt']['samples'], engines['vanilla']['samples']))
    result = dict(before=before, after=provenance(), cpu=CPU,
                  probe_sha256=sha(Path(__file__)), summary=summary, runs=runs)
    (OUT / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(summary, indent=2))


if __name__ == '__main__':
    main()
