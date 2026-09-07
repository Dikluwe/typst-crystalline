"""Recibos documentais R5; nunca altera L0, Rust ou artefatos predecessores."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
OWNERS = [
    "entities/value.md", "entities/introspector.md",
    "compiler/introspect.md", "compiler/introspect/heading.md",
    "compiler/introspect/from_tags.md", "compiler/stdlib/foundations/query.md",
    "compiler/eval/bindings/field_access.md", "compiler/eval/call_dispatch.md",
    "compiler/eval/operators/equality.md", "compiler/eval/repr.md",
    "compiler/stdlib/loading.md", "infra/query-helpers.md", "shell/cli.md",
]
NUCLEUS = "00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml"

def sha(data):
    return hashlib.sha256(data).hexdigest()

def command(*args):
    p = subprocess.run(args, cwd=ROOT, capture_output=True, text=True)
    return {"argv": list(args), "exit": p.returncode, "stdout": p.stdout, "stderr": p.stderr}

def write_new(name, obj):
    with (D / name).open("x") as out:
        json.dump(obj, out, ensure_ascii=False, indent=2)
        out.write("\n")

mode = sys.argv[1]
result = {"mode": mode, "timestamp": datetime.now(timezone.utc).isoformat(),
          "baseline_sha256": sha((D / "p1307-r5-baseline.json").read_bytes()),
          "state": {key: command(*args) for key, args in {
              "head": ["git", "rev-parse", "HEAD"],
              "status": ["git", "status", "--porcelain=v1"],
              "diff_stat": ["git", "diff", "HEAD", "--stat"],
              "diff": ["git", "diff", "HEAD"],
              "staged": ["git", "diff", "--cached"],
          }.items()}}
paths = ["00_nucleo/prompts/" + name for name in OWNERS]
if mode == "before":
    assert not (ROOT / NUCLEUS).exists()
    result["files"] = {p: {"sha256": sha((ROOT / p).read_bytes()), "text": (ROOT / p).read_text()} for p in paths}
    result["new_nucleus"] = NUCLEUS
    write_new("p1307-r5-l0-before.json", result)
elif mode.startswith("freeze"):
    paths.append(NUCLEUS)
    result["files"] = {p: {"sha256": sha((ROOT / p).read_bytes()), "text": (ROOT / p).read_text()} for p in paths}
    result["checks"] = [command("git", "diff", "--check"),
                        command("crystalline-lint", "--checks", "v15,v26", "--fail-on", "warning", ".")]
    baseline = json.loads((D / "p1307-r5-baseline.json").read_text())
    result["baseline_keys"] = list(baseline)
    result["changed_inventory"] = [p for p, digest in baseline["source_inventory"].items()
                                   if not (ROOT / p).is_file() or sha((ROOT / p).read_bytes()) != digest]
    result["changed_predecessors"] = [p for p, digest in baseline["protected_predecessors"].items()
                                      if not (ROOT / p).is_file() or sha((ROOT / p).read_bytes()) != digest]
    result["unexpected_inventory_changes"] = [p for p in result["changed_inventory"] if p not in paths]
    result["predecessor_count"] = len(baseline["protected_predecessors"])
    result["current_r5_evidence"] = {str(p.relative_to(ROOT)): sha(p.read_bytes())
                                     for p in sorted(D.glob("p1307-r5-*.json"))
                                     if "l0-freeze" not in p.name}
    write_new("p1307-r5-l0-" + mode + ".json", result)
elif mode == "gates":
    before = {p: sha((ROOT / p).read_bytes()) for p in paths + [NUCLEUS]}
    result["checks"] = [command("git", "diff", "--check"), command("crystalline-lint", ".")]
    result["l0_before"] = before
    result["l0_after"] = {p: sha((ROOT / p).read_bytes()) for p in before}
    baseline = json.loads((D / "p1307-r5-baseline.json").read_text())
    result["changed_rust"] = [p for p, digest in baseline["source_inventory"].items()
                               if p.endswith(".rs") and sha((ROOT / p).read_bytes()) != digest]
    result["changed_predecessors"] = [p for p, digest in baseline["protected_predecessors"].items()
                                      if not (ROOT / p).is_file() or sha((ROOT / p).read_bytes()) != digest]
    result["note"] = "Full linter exit is not zero violations; V5 and existing warnings remain. No hashes were repaired."
    write_new("p1307-r5-final-gates.json", result)
else:
    raise SystemExit("mode must be before or freeze")
print(json.dumps({"mode": mode, "files": len(paths), "timestamp": result["timestamp"]}))
