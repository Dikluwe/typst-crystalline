"""Record exact P1308 baselines and command provenance, without judging results."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def state():
    return {key: subprocess.check_output(["git", *args], cwd=ROOT, text=True)
            for key, args in {"head": ["rev-parse", "HEAD"],
                              "status": ["status", "--short"],
                              "diff_stat": ["diff", "HEAD", "--stat"],
                              "diff": ["diff", "HEAD", "--binary"],
                              "staged": ["diff", "--cached", "--binary"]}.items()}

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def save(name, value):
    path = D / ("p1308-" + name + ".json")
    with path.open("x") as f:
        json.dump(value, f, ensure_ascii=False, indent=2)
        f.write("\n")
    print(json.dumps({"path": str(path), "sha256": sha(path)}))

if sys.argv[1] == "baseline":
    inventory = {str(p.relative_to(ROOT)): sha(p)
                 for directory in ["00_nucleo/prompts", "01_core", "02_shell", "03_infra", "04_wiring"]
                 for p in (ROOT / directory).rglob("*")
                 if p.is_file() and p.suffix in (".rs", ".md", ".toml")}
    save("baseline", {"at": now(), "state": state(), "inventory": inventory,
        "authorization": "Pode escrever o passo e implementar autorizo — four R6 boundaries, L0 first, no commit/stage/push",
        "regime": "executado sem atestacao de isolamento tecnico",
        "baseline_binary": {"path": "/dev/shm/p1308-baseline.XzjFgA/typst", "sha256": sha("/dev/shm/p1308-baseline.XzjFgA/typst")},
        "vanilla": {"path": "/usr/local/bin/typst", "sha256": sha("/usr/local/bin/typst"), "upstream": "a51e02804"},
        "predecessors": {p.name: sha(p) for p in D.glob("p1307-r6-*") if p.is_file()},
        "runner_sha256": sha(__file__)})
else:
    before = state(); start = now(); tick = time.monotonic()
    command = sys.argv[2:]
    result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
    save(sys.argv[1], {"argv": command, "cwd": str(ROOT), "at": start, "end": now(),
        "seconds": time.monotonic() - tick, "exit": result.returncode,
        "stdout": result.stdout, "stderr": result.stderr, "before": before, "after": state(),
        "baseline_sha256": sha(D / "p1308-baseline.json"),
        "manifest_sha256": sha(D / "p1308-manifest.json") if (D / "p1308-manifest.json").exists() else None,
        "successor_sha256": sha(D / "p1308-manifest-successor-1.json") if (D / "p1308-manifest-successor-1.json").exists() else None})
    print(result.stdout[-2000:]); print(result.stderr[-2000:])
    raise SystemExit(result.returncode)
