"""Verify the frozen diagnostic; does not execute the compiler or change sources."""
from pathlib import Path
import ast
import gzip
import hashlib
import json

folder = Path(__file__).resolve().parent
root = folder.parents[2]


def sha(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


before = json.loads(gzip.decompress((folder / "before.json.gz").read_bytes()))
for name, expected in before["source_hashes"].items():
    assert sha(root / name) == expected, name
for binary in before["binaries"].values():
    assert sha(Path(binary["path"])) == binary["sha256"], binary["path"]
cases = []
for name in ("measurement", "supplement", "focal", "last", "edges", "include"):
    cases.extend(json.loads(gzip.decompress((folder / (name + ".json.gz")).read_bytes()))["cases"])
assert len(cases) == 703
assert sum(len(case["results"]) for case in cases) == 1498
for script in folder.glob("*.py"):
    ast.parse(script.read_text(), filename=str(script))
for line in (folder / "manifest.sha256").read_text().splitlines():
    expected, name = line.split("  ", 1)
    assert sha(folder / name) == expected, name
print(f"OK: {len(before['source_hashes'])} protected files, 3 binaries, 703 rows, 1498 invocations, manifest and scripts")
