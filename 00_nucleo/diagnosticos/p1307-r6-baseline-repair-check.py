"""Check the isolated baseline copy, then run the repaired independent tests."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
COPY = Path("/tmp/p1307-r6-baseline-source.s6jhEk")
base = json.loads((ROOT / "00_nucleo/diagnosticos/p1307-r6-baseline.json").read_text())
checked = {}
for path, expected in base["source_inventory"].items():
    if not path.endswith(".rs") or not (COPY / path).exists():
        continue
    data = (COPY / path).read_bytes()
    if path == "01_core/src/compiler/eval/tests.rs":
        marker = b"    // P1307-R6 independent tests:"
        assert data.count(marker) == 1
        data = data.split(marker)[0] + b"}\n"
    actual = hashlib.sha256(data).hexdigest()
    assert actual == expected, (path, expected, actual)
    checked[path] = actual
print(json.dumps({"baseline_copy": str(COPY), "original_source_hashes": checked,
                  "repaired_tests_sha256": hashlib.sha256((COPY / "01_core/src/compiler/eval/tests.rs").read_bytes()).hexdigest()}, ensure_ascii=False), flush=True)
raise SystemExit(subprocess.call([
    "cargo", "test", "--manifest-path", str(COPY / "Cargo.toml"),
    "-p", "typst-core", "--release", "p1307_independent", "--", "--nocapture",
], cwd=COPY))
