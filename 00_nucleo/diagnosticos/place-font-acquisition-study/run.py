"""Collect a new dataset; do not rerun against a sealed evidence directory."""
from pathlib import Path
import datetime, hashlib, json, os, shutil, statistics, subprocess, threading, time

ROOT = Path(__file__).resolve().parents[3]
R = Path(__file__).resolve().parent
FONT = ROOT / '00_nucleo/diagnosticos/infra-performance-followup/fonts'
CASES = ['empty', 'shape', 'placed_shape', 'image', 'placed_image', 'text', 'placed_text']
ENV = os.environ.copy()
for key in list(ENV):
    if key.startswith(('TYPST_', 'CRYSTALLINE_')):
        ENV.pop(key)
ENV['RAYON_NUM_THREADS'] = '1'

def sha(path):
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def fingerprints():
    inventory = [str(p.relative_to(ROOT)) for base in ['01_core', '02_shell', '03_infra', '04_wiring', 'benches', '00_nucleo/prompts', 'lab/typst-original/crates'] for p in (ROOT / base).rglob('*') if p.is_file() and not set(p.parts) & {'target', '__pycache__'}]
    inventory += [p for p in ['Cargo.toml', 'Cargo.lock', '.cargo/config.toml'] if (ROOT / p).is_file()]
    inputs = list((R / 'inputs').rglob('*')) + list((R / 'probe').rglob('*'))
    inputs += list(FONT.rglob('*')) + list((R / 'bin').iterdir())
    inputs += [Path(p) for p in subprocess.check_output(['fc-list', '--format', '%{file}\n'], text=True).splitlines()]
    for base in ['/etc/fonts', '/usr/share/fontconfig', str(Path.home() / '.cache/fontconfig')]:
        if Path(base).exists():
            inputs += list(Path(base).rglob('*'))
    files = {p: sha(ROOT / p) if (ROOT / p).is_file() else None for p in inventory}
    for p in inputs:
        if p.is_file() and not set(p.parts) & {'target', '__pycache__'}:
            files[str(p)] = sha(p)
    return files

active = set()
lock = threading.Lock()
hits = []
stopped = threading.Event()

def competitors():
    found = []
    for p in Path('/proc').iterdir():
        if not p.name.isdigit():
            continue
        try:
            comm = (p / 'comm').read_text().strip()
            if int(p.name) not in active and (comm in {'cargo', 'rustc', 'rustdoc', 'typst'} or comm.endswith('_bench') or comm.startswith(('infra-font-', 'place-font-'))):
                found.append({'pid': int(p.name), 'comm': comm, 'cmd': (p / 'cmdline').read_bytes().replace(b'\0', b' ').decode(errors='replace')})
        except (FileNotFoundError, PermissionError, ProcessLookupError):
            pass
    return found

def monitor():
    while not stopped.is_set():
        with lock:
            found = competitors()
        if found:
            hits.append({'utc': utc(), 'processes': found})
        stopped.wait(.1)

def execute(cmd, stem, timed=False):
    with lock:
        assert not competitors(), 'Concurrent build/test/benchmark: stop measurement'
        start = time.perf_counter_ns()
        proc = subprocess.Popen(cmd, cwd=ROOT, env=ENV, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        active.add(proc.pid)
    out, err = proc.communicate()
    elapsed = (time.perf_counter_ns() - start) / 1e6
    with lock:
        active.remove(proc.pid)
    stem.with_suffix('.stdout').write_bytes(out)
    stem.with_suffix('.stderr').write_bytes(err)
    row = {'utc': utc(), 'command': cmd, 'returncode': proc.returncode, 'launcher_ms': elapsed, 'timed_launcher': timed}
    save(stem.with_suffix('.receipt.json'), row)
    assert proc.returncode == 0, (cmd, err.decode(errors='replace'))
    return row, out

def cli(engine, case, output, trace=None):
    cmd = [str(R / 'bin' / engine), 'compile', str(R / 'inputs' / (case + '.typ')), str(output), '--root', str(R / 'inputs'), '--font-path', str(FONT)]
    if engine == 'vanilla':
        cmd += ['--jobs', '1']
    if trace:
        cmd += ['--timings' if engine == 'vanilla' else '--timings-json', str(trace)]
    return cmd

def probe(case, output, preload=False, stack=False):
    cmd = [str(R / 'bin/probe'), str(R / 'inputs' / (case + '.typ')), str(FONT), str(output), '3']
    if preload:
        cmd += ['--preload']
    if stack:
        cmd += ['--stack']
    return cmd

def distribution(values):
    mean = statistics.mean(values)
    return {'n': len(values), 'mean_ms': mean, 'median_ms': statistics.median(values), 'min_ms': min(values), 'max_ms': max(values), 'stdev_ms': statistics.stdev(values), 'cv': statistics.stdev(values) / mean if mean else 0}

def main():
    assert not (R / 'samples.json').exists(), 'Preserve existing evidence'
    for folder in ['bin', 'raw', 'controls']:
        (R / folder).mkdir(exist_ok=True)
    for source, dest in [(ROOT / 'target/release/place-font-acquisition-probe', 'probe'), (ROOT / 'target/release/typst', 'tekt'), (ROOT / 'lab/typst-original/target/release/typst', 'vanilla')]:
        shutil.copy2(source, R / 'bin' / dest)
    assert sha(R / 'bin/vanilla') == '7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
    save(R / 'executables.json', {p.name: sha(p) for p in (R / 'bin').iterdir()})
    save(R / 'run-environment.json', {'utc': utc(), 'rayon_threads': 1, 'affinity': sorted(os.sched_getaffinity(0)), 'loadavg': os.getloadavg(), 'rust_flags': {k: os.environ.get(k) for k in ['RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS']}, 'caches': 'warm OS caches; no cache drop; new world per probe process; repeats reuse world and process caches'})
    initial = fingerprints()
    save(R / 'initial-fingerprints.json', initial)
    thread = threading.Thread(target=monitor, daemon=True)
    thread.start()
    samples, receipts = [], []
    try:
        # Stack and trace controls do not contribute timing distributions.
        for case in CASES:
            stem = R / 'controls' / (case + '-stack')
            _, out = execute(probe(case, stem.with_suffix('.pdf'), stack=True), stem)
            save(stem.with_suffix('.json'), json.loads(out))
            for engine in ['vanilla', 'tekt']:
                stem = R / 'controls' / (case + '-' + engine)
                execute(cli(engine, case, stem.with_suffix('.pdf'), stem.with_suffix('.trace.json')), stem)
                for warmup in range(2):
                    warm = R / 'controls' / f'{case}-{engine}-warm{warmup}'
                    execute(cli(engine, case, warm.with_suffix('.pdf')), warm)
        for round_no in range(3):
            before = fingerprints()
            hit_start = len(hits)
            block = []
            for sample in range(4):
                for case in CASES:
                    for preload in ([False, True] if (round_no + sample) % 2 == 0 else [True, False]):
                        stem = R / 'raw' / f'r{round_no}-s{sample}-{case}-preload{preload}'
                        receipt, out = execute(probe(case, stem.with_suffix('.pdf'), preload), stem)
                        block.append({'kind': 'probe', 'round': round_no, 'sample': sample, 'case': case, 'preload': preload, 'receipt': receipt, 'result': json.loads(out)})
                    for engine in (['tekt', 'vanilla'] if (round_no + sample) % 2 == 0 else ['vanilla', 'tekt']):
                        stem = R / 'raw' / f'r{round_no}-s{sample}-{case}-{engine}'
                        receipt, _ = execute(cli(engine, case, stem.with_suffix('.pdf')), stem, timed=True)
                        block.append({'kind': 'cli', 'round': round_no, 'sample': sample, 'case': case, 'engine': engine, 'receipt': receipt})
            after = fingerprints()
            valid = before == after == initial and not hits[hit_start:]
            save(R / f'round{round_no}-fingerprints.json', {'before': before, 'after': after})
            receipts.append({'round': round_no, 'utc': utc(), 'accepted': valid, 'contamination': hits[hit_start:], 'loadavg': os.getloadavg()})
            if valid:
                samples.extend(block)
            save(R / 'samples.json', samples)
            save(R / 'round-receipts.json', receipts)
            print(f'round {round_no}: {"accepted" if valid else "discarded"}', flush=True)
    finally:
        stopped.set()
        thread.join()
        save(R / 'process-monitor.json', hits)
    assert all(x['accepted'] for x in receipts), 'Discarded rounds: retain raw data, do not claim baseline'
    summaries = []
    for case in CASES:
        for kind, variants in [('cli', ['tekt', 'vanilla']), ('probe', [False, True])]:
            for variant in variants:
                rows = [s for s in samples if s['kind'] == kind and s['case'] == case and s['engine' if kind == 'cli' else 'preload'] == variant]
                for iteration in ([None] if kind == 'cli' else range(3)):
                    value = lambda s: s['receipt']['launcher_ms'] if kind == 'cli' else s['result']['rows'][iteration]['pipeline_ms']
                    stat = distribution([value(s) for s in rows])
                    rounds = [statistics.median(value(s) for s in rows if s['round'] == n) for n in range(3)]
                    stable = stat['cv'] <= .2 and all(abs(m / stat['median_ms'] - 1) <= .1 for m in rounds)
                    summaries.append({'case': case, 'kind': kind, 'variant': variant, 'iteration': iteration, 'distribution': stat, 'round_medians_ms': rounds, 'stable': stable})
    save(R / 'summary.json', summaries)
    final = fingerprints()
    save(R / 'final-fingerprints.json', final)
    assert final == initial
    print('collection complete; fingerprints preserved', flush=True)

if __name__ == '__main__':
    main()
