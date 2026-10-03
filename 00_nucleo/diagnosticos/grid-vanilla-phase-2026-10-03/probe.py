"""Compare pinned vanilla trace spans with Tekt's public timing fields.

Generated PDFs, fixtures and traces are kept under /tmp.
"""

import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = Path('/tmp/tekt-grid-vanilla-phase')
OUT.mkdir(exist_ok=True)
VANILLA = ROOT / 'lab/typst-original/target/release/typst'
TEKT = ROOT / 'target/release/typst'
ENV = dict(os.environ, SOURCE_DATE_EPOCH='0', RAYON_NUM_THREADS='1')
CPU = min(os.sched_getaffinity(0))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)


def fixture(rows):
    path = OUT / f'grid{rows}.typ'
    cells = ',\n'.join(f'[C{r:02d}{c}]' for r in range(rows) for c in range(4))
    path.write_text(
        '#set page(width: 500pt, height: 3500pt, margin: 30pt)\n'
        '#set text(font: "Libertinus Serif", size: 10pt)\n'
        f'#table(columns: 4, rows: {rows},\n{cells}\n)\n'
    )
    return path


def run(rows, iteration, engine, source):
    pdf = OUT / f'{engine}-{rows}-{iteration}.pdf'
    timing = OUT / f'{engine}-{rows}-{iteration}.json'
    executable = VANILLA if engine == 'vanilla' else TEKT
    command = [str(executable), 'compile', str(source), str(pdf),
               '--root', str(OUT), '--pdf-tagged=false', '--pretty=false']
    if engine == 'vanilla':
        command += ['--jobs', '1', '--timings', str(timing)]
    else:
        command += ['--timings-json', str(timing)]
    subprocess.run(command, cwd=ROOT, env=ENV, check=True,
                   stdout=subprocess.DEVNULL,
                   preexec_fn=lambda: os.sched_setaffinity(0, {CPU}))
    payload = json.loads(timing.read_text())
    if engine == 'tekt':
        return {'layout_ms': payload['layout_ms'],
                'shape_ms': payload['shape_ms'],
                'total_ms': payload['total_ms']}

    stack = []
    spans = []
    for event in payload:
        phase = event.get('ph')
        if phase == 'B':
            stack.append((event['name'], event['ts']))
        elif phase == 'E':
            name, start = stack.pop()
            assert name == event['name']
            spans.append((name, start, event['ts'] - start))
    assert not stack
    table = [span for span in spans if span[0] == 'layout_table']
    pads = sorted((span for span in spans if span[0] == 'layout_pad'),
                  key=lambda span: span[1])
    assert len(table) == 1
    expected = rows * 4
    assert len(pads) == expected * 3
    groups = [pads[i * expected:(i + 1) * expected] for i in range(3)]
    return {
        'layout_table_ms': table[0][2] / 1000,
        'layout_table_calls': len(table),
        'layout_pad_calls': len(pads),
        'phase_pad_sum_ms': [sum(span[2] for span in group) / 1000
                             for group in groups],
        'phase_span_ms': [(group[-1][1] - group[0][1]) / 1000
                          for group in groups],
    }


def main():
    sources = {rows: fixture(rows) for rows in (1, 20, 80)}
    samples = []
    for rows, source in sources.items():
        for iteration in range(5):
            for engine in ('vanilla', 'tekt'):
                samples.append({'rows': rows, 'iteration': iteration,
                                'engine': engine,
                                **run(rows, iteration, engine, source)})
    summary = {}
    for rows in sources:
        summary[rows] = {}
        for engine in ('vanilla', 'tekt'):
            data = [sample for sample in samples if sample['rows'] == rows
                    and sample['engine'] == engine]
            fields = ('layout_table_ms', 'layout_pad_calls',
                      'layout_table_calls', 'layout_ms', 'shape_ms', 'total_ms')
            summary[rows][engine] = {
                field: statistics.median(sample[field] for sample in data)
                for field in fields if field in data[0]
            }
            if engine == 'vanilla':
                summary[rows][engine]['phase_pad_sum_ms'] = [
                    statistics.median(sample['phase_pad_sum_ms'][i]
                                      for sample in data) for i in range(3)
                ]
                summary[rows][engine]['phase_span_ms'] = [
                    statistics.median(sample['phase_span_ms'][i]
                                      for sample in data) for i in range(3)
                ]
    result = {
        'utc': time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime()),
        'head': git('rev-parse', 'HEAD').strip(),
        'status': git('status', '--porcelain=v1', '--untracked-files=all'),
        'diff_stat': git('diff', 'HEAD', '--stat'),
        'cpu': CPU,
        'binary_sha256': {'vanilla': sha(VANILLA), 'tekt': sha(TEKT)},
        'fixture_sha256': {str(rows): sha(path)
                           for rows, path in sources.items()},
        'summary': summary,
        'samples': samples,
    }
    (OUT / 'results.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(summary, indent=2))


if __name__ == '__main__':
    main()
