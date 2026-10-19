"""Untimed output controls and separate vanilla trace cohort."""
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import time
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[3]
OUT = Path('/tmp/tekt-macro-micro-2026-10-05')
VANILLA = ROOT / 'lab/typst-original/target/release/typst'
ENV = dict(os.environ, SOURCE_DATE_EPOCH='0', RAYON_NUM_THREADS='1')
CPU = min(os.sched_getaffinity(0))


def words(pdf):
    tree = ET.fromstring(subprocess.check_output(['pdftotext', '-bbox', str(pdf), '-']))
    return [(node.text or '', *[float(node.attrib[key])
                               for key in ('xMin', 'yMin', 'xMax', 'yMax')])
            for node in tree.iter() if node.tag.endswith('}word')]


def traces():
    samples = []
    for name in ('hello', 'paragraphs', 'grid320', 'eval30000', 'context'):
        for index in range(3):
            timing, pdf = (OUT / f'{name}-vanilla-trace-{index}{ext}'
                           for ext in ('.json', '.pdf'))
            command = [str(VANILLA), 'compile', str(OUT / f'{name}.typ'), str(pdf),
                       '--root', str(OUT), '--pdf-tagged=false', '--pretty=false',
                       '--jobs', '1', '--timings', str(timing)]
            subprocess.run(command, cwd=ROOT, env=ENV, check=True,
                           stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
                           preexec_fn=lambda: os.sched_setaffinity(0, {CPU}))
            events, stack, spans = json.loads(timing.read_text()), [], []
            for event in events:
                if event.get('ph') == 'B':
                    stack.append(event)
                elif event.get('ph') == 'E':
                    start = stack.pop()
                    assert start['name'] == event['name']
                    spans.append(dict(name=start['name'], ms=(event['ts'] - start['ts']) / 1000))
            assert not stack
            labels = ('discover fonts', 'scan system fonts', 'eval', 'layout document', 'pdf')
            samples.append(dict(fixture=name, index=index, command=command,
                                spans={label: [s['ms'] for s in spans if s['name'] == label]
                                       for label in labels}))
    return samples


def main():
    result = dict(utc=time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime()),
                  head=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
                  diff_stat=subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], cwd=ROOT, text=True),
                  status=subprocess.check_output(['git', 'status', '--porcelain=v1', '--untracked-files=all'],
                                                 cwd=ROOT, text=True),
                  vanilla_sha256=hashlib.sha256(VANILLA.read_bytes()).hexdigest(),
                  probe_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  controls={})
    for name in ('hello', 'paragraphs', 'grid320'):
        t, v = [words(OUT / f'{name}-{engine}-0.pdf') for engine in ('tekt', 'vanilla')]
        assert len(t) == len(v) and all(a[0] == b[0] for a, b in zip(t, v))
        result['controls'][name] = dict(word_count=len(t),
            max_coordinate_delta_pt={key: max(abs(a[i] - b[i]) for a, b in zip(t, v))
                                     for i, key in enumerate(('xMin', 'yMin', 'xMax', 'yMax'), 1)})
    # Identical rectangle output validates the calculations-only fixtures.
    rasters = {}
    for name in ('shape', 'eval1000', 'eval10000', 'eval30000'):
        rasters[name] = {}
        for engine in ('tekt', 'vanilla'):
            prefix = OUT / f'{name}-{engine}-raster'
            subprocess.run(['pdftoppm', '-r', '72', '-scale-to', '900', '-singlefile',
                            str(OUT / f'{name}-{engine}-0.pdf'), str(prefix)], check=True)
            rasters[name][engine] = hashlib.sha256(prefix.with_suffix('.ppm').read_bytes()).hexdigest()
    result['raster_sha256'] = rasters
    result['trace_samples'] = traces()
    result['trace_summary'] = {}
    for name in ('hello', 'paragraphs', 'grid320', 'eval30000', 'context'):
        data = [s for s in result['trace_samples'] if s['fixture'] == name]
        result['trace_summary'][name] = {label: {
            'calls': [len(s['spans'][label]) for s in data],
            'inclusive_wall_median_ms': statistics.median(sum(s['spans'][label]) for s in data)
        } for label in data[0]['spans']}
    (OUT / 'qualification.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({key: result[key] for key in ('controls', 'raster_sha256', 'trace_summary')}, indent=2))


if __name__ == '__main__':
    main()
