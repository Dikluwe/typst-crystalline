"""Record first-label ownership and hidden-ancestor language observations."""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import subprocess

HERE = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument('--product', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
assert not args.output.exists()
args.output.mkdir(parents=True)
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
cases = [
    ('ancestor-hidden', '#show block:none\n#block(figure([A],kind:raw,supplement:[Listing]))\n#context [#metadata((query(figure).len(),counter(figure).final(),counter(figure.where(kind:raw)).final())) <probe>]\n'),
    ('own-hidden', '#show figure:none\n#figure([A],kind:raw,supplement:[Listing])\n#context [#metadata((query(figure).len(),counter(figure).final(),counter(figure.where(kind:raw)).final())) <probe>]\n'),
    ('duplicate-kind-first', '#figure([A],kind:table,caption:[ONE]) <same>\n#figure([B],kind:raw,caption:[TWO]) <same>\n#context [#metadata((query(<same>).first().kind,counter(figure.where(kind:table)).final(),counter(figure.where(kind:raw)).final())) <probe>]\n'),
    ('duplicate-numbering-first', '#figure([A],kind:table,caption:[ONE],numbering:none) <same>\n#figure([B],kind:raw,caption:[TWO]) <same>\n#context [#metadata((query(<same>).first().kind,counter(figure.where(kind:table)).final(),counter(figure.where(kind:raw)).final())) <probe>]\n')
]
record = {'utc_start': utc(), 'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
          'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
          'product': str(args.product), 'product_sha256': sha(args.product), 'cases': []}
for name, source in cases:
    path = args.output / (name + '.typ')
    path.write_text(source)
    command = [str(args.product), 'query', str(path), '<probe>', '--field', 'value', '--one']
    start = utc()
    run = subprocess.run(command, capture_output=True, text=True, timeout=60)
    record['cases'].append({'name': name, 'source': source, 'source_sha256': sha(path),
                            'argv': command, 'utc_start': start, 'utc_end': utc(),
                            'returncode': run.returncode, 'stdout': run.stdout, 'stderr': run.stderr})
    print(name, run.returncode, run.stdout.strip(), run.stderr[:500])
record['utc_end'] = utc()
(args.output / 'receipt.json').write_text(json.dumps(record, indent=2) + '\n')
