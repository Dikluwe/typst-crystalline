"""Overlay the archived blobs of the measured fixed-cut experiment."""
from hashlib import sha256
from pathlib import Path
import sys
import tarfile

here = Path(__file__).resolve().parent
lab = here.parents[1]
reports = lab / 'results/construction/diagnostics'
source = here / 'source'
manifest = reports / 'page-three-suffix-memo-replay-2026-11-21/candidate-source.sha256'
expected = {name.removeprefix('./'): digest for digest, name in
            (line.split(maxsplit=1) for line in manifest.read_text().splitlines())}
needed = {}
for name, digest in expected.items():
    dest = source / name
    if not dest.is_file() or sha256(dest.read_bytes()).hexdigest() != digest:
        needed.setdefault(digest, []).append(dest)
paths = sorted(p for ext in ('rs', 'md', 'toml') for p in reports.glob(f'**/*.{ext}'))
for path in paths:
    if not needed:
        break
    data = path.read_bytes()
    for dest in needed.pop(sha256(data).hexdigest(), []):
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_bytes(data)
for name,digest in expected.items():
    candidate = lab.parent / name
    if digest in needed and candidate.is_file() and sha256(data := candidate.read_bytes()).hexdigest() == digest:
        for dest in needed.pop(digest):
            dest.write_bytes(data)
for archive in sorted(lab.glob('experiments/**/*.tar.gz')) + sorted(reports.glob('**/*source*.tar.gz')):
    if not needed:
        break
    if not any(fragment in archive.name for fragment in ("source", "consumer", "product")):
        continue
    names = {dest.name for dests in needed.values() for dest in dests}
    with tarfile.open(archive, "r|gz") as bundle:
        for member in bundle:
            if member.isfile() and Path(member.name).name in names:
                data = bundle.extractfile(member).read()
                for dest in needed.pop(sha256(data).hexdigest(), []):
                    dest.parent.mkdir(parents=True, exist_ok=True)
                    dest.write_bytes(data)
for digest,dests in needed.items():
    for dest in dests:
        print(digest,dest.relative_to(source))
print('missing',sum(map(len,needed.values())))
sys.exit(bool(needed))
