"""Read-only documentary review of P1307; never certifies implementation."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(name):
    return json.loads((DIAG / name).read_text())


def pin(path):
    path = Path(path)
    return {"path": str(path), "sha256": sha(path), "bytes": path.stat().st_size}


def save(name, payload):
    with (DIAG / name).open("x") as stream:
        json.dump(payload, stream, ensure_ascii=False, indent=2)
        stream.write("\n")
    print(json.dumps(pin(DIAG / name)))


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True)


def freeze():
    preflight = read("p1307-preflight.json")
    excluded = {"p1307-manifest.json", "p1307-pre-gate-verification.json", "p1307-pre-gate-seal.json", "p1307-pre-gate-report.md"}
    inputs = {name: pin(DIAG / name) for name in preflight["outputs_exact"] if name not in excluded}
    save("p1307-manifest.json", {
        "protocol": "p1307-blocked-documentary-manifest-v1", "at": now(),
        "status": "P1307_REDESIGN_REQUIRED", "candidate_exists": False,
        "implementation_authorized": False, "ready_for_ADR0127_confirmation": False,
        "regime": "executado sem atestacao de isolamento tecnico",
        "inputs": inputs, "step": pin(ROOT / preflight["step"]),
        "roles": preflight["roles"], "owners_unchanged": preflight["owners"],
        "unknown_policy": preflight["unknown"],
        "causality": "Preflight and baseline precede independent measurements. This retrospective inventory is not a pre-contract seal and creates no implementation authority. Author receipts pin their actual causal inputs; no final attested protocol conformity is claimed.",
        "scope_stop": "Step section 3 requires reopening for other owners; mandatory contextual observability also remains a blocker. L0, Rust and headers stay byte-identical to P0. P3 reseal and all P4-P9 implementation phases are not executed.",
    })


def verify():
    manifest = read("p1307-manifest.json")
    baseline = read("p1307-baseline.json")
    checks = {}
    checks["manifest_inputs_unchanged"] = all(sha(row["path"]) == row["sha256"] for row in manifest["inputs"].values())
    checks["step_unchanged"] = sha(manifest["step"]["path"]) == manifest["step"]["sha256"]
    checks["head_unchanged"] = git("rev-parse", "HEAD") == baseline["source_before"]["head"]
    checks["tracked_diff_unchanged"] = hashlib.sha256(subprocess.check_output(["git", "diff", "HEAD", "--binary"], cwd=ROOT)).hexdigest() == baseline["source_before"]["diff_sha256"]
    changed = [path for path, digest in baseline["source_hashes"].items() if sha(ROOT / path) != digest]
    checks["all_product_L0_Cargo_sources_unchanged"] = not changed
    changed_closure = [path for path, row in baseline["p1306_closure"].items() if sha(ROOT / path) != row["sha256"]]
    checks["P1306_closure_unchanged"] = not changed_closure
    checks["binaries_match_P0"] = all(sha(row["path"]) == row["sha256"] for row in baseline["binaries"].values())
    checks["fresh_baseline_build_success"] = baseline["build"]["exit_code"] == 0
    gates = read("p1307-gates.json")
    checks["read_only_gates_success"] = all(row["exit_code"] == 0 for row in gates)
    checks["no_implementation_or_ready_claim"] = manifest["candidate_exists"] is False and manifest["implementation_authorized"] is False and manifest["ready_for_ADR0127_confirmation"] is False
    allowed = set(read("p1307-preflight.json")["outputs_exact"])
    unexpected = [p.name for p in DIAG.glob("p1307-*") if p.is_file() and p.name not in allowed]
    checks["declared_outputs_only"] = not unexpected
    initial_untracked = {line[3:] for line in baseline["source_before"]["status"].splitlines() if line.startswith("?? ")}
    current_untracked = set(git("ls-files", "--others", "--exclude-standard").splitlines())
    allowed_new = {"00_nucleo/diagnosticos/" + name for name in allowed}
    unexpected_new = sorted(current_untracked - initial_untracked - allowed_new)
    checks["no_undeclared_new_files"] = not unexpected_new
    checks["index_remains_unstaged"] = not git("diff", "--cached", "--name-only")
    contract = read("p1307-contract-measurement.json")
    checks["contract_pins_match"] = contract["baseline_artifact_sha256"] == sha(DIAG / "p1307-baseline.json") and contract["script_sha256"] == sha(DIAG / "p1307-contract-measure.py")
    contract_keys = {(r["case"], r["profile"], r["product"], r["order"]) for r in contract["runs"]}
    checks["contract_matrix_cardinality"] = len(contract_keys) == len(contract["runs"]) == len(contract["cases"]) * len(contract["profiles"]) * 2 * 3
    oracle = read("p1307-oracle.json")
    measurement = read("p1307-oracle-measurement.json")
    receipt = read("p1307-oracle-receipt.json")
    plan = read("p1307-mutant-plan.json")
    checks["oracle_measurement_pin_matches"] = oracle["measurement_sha256"] == sha(DIAG / "p1307-oracle-measurement.json")
    checks["oracle_receipt_pins_match"] = all(sha(DIAG / name) == digest for name, digest in receipt["inputs"].items()) and receipt["script_sha256"] == sha(DIAG / "p1307-oracle.py") and receipt["mutant_plan_sha256"] == sha(DIAG / "p1307-mutant-plan.json")
    checks["mutant_plan_inputs_match"] = all(sha(DIAG / name) == digest for name, digest in plan["inputs"].items())
    checks["mutants_not_misrepresented_as_executed"] = plan["actual_source_mutants_executed"] == 0 and plan["mutation_score"] is None
    unknown_rows = [r for rows in measurement["results"].values() for r in rows if r["observable"]["kind"] == "Unknown"]
    checks["oracle_unknowns_retained"] = len(unknown_rows) == len(measurement["unknowns"]) == len(oracle["unknowns"]) and len(unknown_rows) > 0
    findings = {
        "schema": "p1307-documentary-review-v1", "at": now(),
        "executor": "/root coordinator, not a distinct final implementation verifier",
        "manifest": pin(DIAG / "p1307-manifest.json"), "checks": checks,
        "changed_source_paths": changed, "changed_P1306_closure": changed_closure,
        "unexpected_outputs": unexpected,
        "unexpected_new_files": unexpected_new,
        "source_files_pinned": len(baseline["source_hashes"]),
        "P1306_closure_files_pinned": len(baseline["p1306_closure"]),
        "contract_runs": len(contract["runs"]),
        "contract_unknown_rows": sum(r["classification"] == "Unknown" for r in contract["runs"]),
        "oracle_normal_runs": sum(len(rows) for rows in measurement["results"].values()),
        "oracle_normal_unknown_rows": len(unknown_rows),
        "status": "P1307_REDESIGN_REQUIRED",
        "documentary_integrity": "CONSISTENT" if all(checks.values()) else "FAILED",
        "implementation_verdict": "NOT_EXECUTED",
        "mutation_score": None, "ready_seal": False,
        "limitations": ["No technical isolation attestation", "Separate final verifier unavailable due agent thread limit", "No L0 approval requested before scope and observability blockers are resolved", "This check authenticates bytes and scope, not general encoder parity or full diagnostic adapter correctness"],
    }
    save("p1307-pre-gate-verification.json", findings)
    if not all(checks.values()):
        raise SystemExit(1)


def refusal():
    review = read("p1307-pre-gate-verification.json")
    assert review["documentary_integrity"] == "CONSISTENT"
    save("p1307-pre-gate-seal.json", {
        "schema": "p1307-pre-gate-seal-refusal-v1", "at": now(),
        "status": "P1307_REDESIGN_REQUIRED", "sealed": False,
        "ready_for_implementation": False, "ready_for_ADR0127_confirmation": False,
        "reason": "Scope and mandatory observability require reopening before L0 can legitimately authorize the complete contract.",
        "manifest": pin(DIAG / "p1307-manifest.json"),
        "review": pin(DIAG / "p1307-pre-gate-verification.json"),
        "report": pin(DIAG / "p1307-pre-gate-report.md"),
        "mutation_score": None, "implementation_certificate": None,
        "regime": "executado sem atestacao de isolamento tecnico",
    })


if __name__ == "__main__":
    {"freeze": freeze, "verify": verify, "refusal": refusal}[sys.argv[1]]()
