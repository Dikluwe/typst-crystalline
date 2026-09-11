"""Record the approved observation interface and partial owner integration."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"


def sha(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True)


start = now()
target = DIAG / "p1339-observation-integration-receipt.json"
assert not target.exists(), "Receipt is immutable; use a successor for another run"
prior = DIAG / "p1339-stabilization-receipt.json"
assert sha(prior) == "cdc7e134cbc42acc748857d8711eaa4157aca29c1e13abb53d2686b236fcc289"
previous = json.loads(prior.read_text())
assert all(sha(DIAG / p) == h for p, h in previous["artifact_sha256"].items())
approval_path = DIAG / "p1339-observation-interface-approval.json"
approval = json.loads(approval_path.read_text())
assert approval["predecessor_sha256"] == sha(prior)
assert approval["approved_l0_sha256"] == previous["l0_sha256"]
assert sha(ROOT / approval["gate_path"]) == approval["gate_sha256"]
historical = json.loads((DIAG / "p1339-retification-check.json").read_text())
assert all(sha(ROOT / p) == h for p, h in historical["artifact_hashes"].items())

prompts = git("diff", "HEAD", "--name-only").splitlines()
expected = set(previous["l0_sha256"]) | {"00_nucleo/prompts/compiler/stdlib/state.md"}
assert set(prompts) == expected, prompts
a0 = json.loads((DIAG / "p1339-a0.json").read_text())
changes = [p for p, h in a0["product_inventory"].items() if sha(ROOT / p) != h]
assert sorted(changes) == sorted(prompts)
assert not git("ls-files", "--others", "--exclude-standard",
               "01_core", "02_shell", "03_infra", "04_wiring").strip()

sources = [
    "01_core/src/compiler/eval/mod.rs",
    "01_core/src/compiler/eval/call_dispatch.rs",
    "01_core/src/compiler/eval/bindings/value_methods.rs",
    "01_core/src/compiler/eval/bindings/field_access.rs",
    "01_core/src/compiler/stdlib/counter.rs",
    "01_core/src/compiler/stdlib/state.rs",
    "01_core/src/compiler/stdlib/foundations/query.rs",
    "01_core/src/entities/engine.rs",
    "03_infra/src/pipeline.rs",
]
checks = []
for argv, expected_exit in [
    (["crystalline-lint", "--checks", "v15,v26", "--fail-on", "warning", "."], 0),
    (["crystalline-lint", "--checks", "v5", "--fail-on", "warning", "."], 1),
    (["git", "diff", "--check"], 0),
]:
    begin = now()
    proc = subprocess.run(argv, cwd=ROOT, text=True, capture_output=True, timeout=60)
    checks.append(dict(argv=argv, cwd=str(ROOT), start=begin, end=now(),
                       exit=proc.returncode, stdout=proc.stdout, stderr=proc.stderr))
    assert proc.returncode == expected_exit, checks[-1]
assert (checks[1]["stdout"] + checks[1]["stderr"]).count("[V5]") == len(prompts)

artifacts = ["p1339-observation-interface-approval.json", "p1339-observation-integration.md"]
result = dict(
    start=start, end=now(), head=git("rev-parse", "HEAD").strip(),
    branch=git("branch", "--show-current").strip(), status=git("status", "--short"),
    diff_stat=git("diff", "HEAD", "--stat"),
    state="L0_INTEGRATION_PARTIAL; PUBLIC_GATE_APPROVED; NO_IMPLEMENTATION",
    regime="executado sem atestação de isolamento",
    predecessor_sha256=sha(prior), approval_sha256=sha(approval_path),
    checks=checks, artifact_sha256={p: sha(DIAG / p) for p in artifacts},
    l0_sha256={p: sha(ROOT / p) for p in prompts},
    source_sha256={p: sha(ROOT / p) for p in sources},
    l0_changed_since_predecessor=[p for p in prompts
        if p not in previous["l0_sha256"] or sha(ROOT / p) != previous["l0_sha256"][p]],
    a0_inventory_differences=changes, product_source_unchanged=True,
    historical_artifacts_preserved=True, recorder_sha256=sha(Path(__file__)),
    command="python3 00_nucleo/diagnosticos/p1339-observation-integration-recorder.py",
    remaining=["Observation comparator coverage and external-input preconditions",
               "Snapshot/output correspondence and diagnostics",
               "Other P1339 routes L0 and feature observability",
               "Independent contract, mutation calibration, seal and RED"],
    no_claims=["No implementation", "No sealed contract", "No phase-D independent RED",
               "No functional GREEN", "No general parity", "No source rehash", "No commit"],
)
body = json.dumps(result, ensure_ascii=False, indent=2)
patch = "*** Begin Patch\n*** Add File: " + str(target) + "\n" + "".join(
    "+" + line + "\n" for line in body.splitlines()) + "*** End Patch\n"
subprocess.run(["apply_patch"], cwd=ROOT, input=patch, text=True, check=True)
print(json.dumps(dict(receipt_sha256=sha(target), v15_v26="clean",
                      v5_pending=len(prompts), source_unchanged=True,
                      l0_changed_since_predecessor=result["l0_changed_since_predecessor"])))
