"""Pin the approved preimplementation inputs, without judging them."""
import datetime
import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"

def pin(path):
    p = ROOT / path
    return {"path": str(p), "sha256": hashlib.sha256(p.read_bytes()).hexdigest()}

def save(name, value):
    with (D / name).open("x") as f:
        json.dump(value, f, ensure_ascii=False, indent=2)
        f.write("\n")

base = json.loads((D / "p1307-r6-baseline.json").read_text())
owner = "00_nucleo/prompts/compiler/eval/bindings.md"
receipt_name = "00_nucleo/diagnosticos/p1307-r6-preseal-amendment.json"
amendment = {
    "path": owner,
    "before_sha256": base["source_inventory"][owner],
    "after_sha256": pin(owner)["sha256"],
}
save(Path(receipt_name).name, {
    "at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "baseline": pin("00_nucleo/diagnosticos/p1307-r6-baseline.json"),
    "change": amendment,
    "reason": "The existing crate-internal bindings hub must reexport the snapshot method helper already approved in R5; no new public API or hub logic.",
    "source": "01_core/src/compiler/eval/bindings/mod.rs:27",
    "reviewer": "/root/p1306_oracle reviewed the exact delta before implementation",
    "candidate_exists": False,
})
amendment["receipt"] = pin(receipt_name)
contracts = sorted(set(p for p in base["source_inventory"] if p.startswith("00_nucleo/prompts/")))
save("p1307-r6-manifest.json", {
    "protocol": "P1307-R6 preimplementation v1",
    "at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "inputs": [pin("00_nucleo/diagnosticos/" + n) for n in [
        "p1307-r6-baseline.json", "p1307-r5-review.json", "p1307-r5-l0-freeze-v5.json",
        "p1307-r6-oracle-measurement.json", "p1307-r6-oracle-note.md",
        "p1307-r6-tests-note.md"]],
    "current_contracts": [pin(p) for p in contracts],
    "suites": [{"oracle": pin("00_nucleo/diagnosticos/p1307-r6-oracle.json"),
                "runner": pin("00_nucleo/diagnosticos/p1307-r6-oracle.py")}],
    "independent_tests": [pin("00_nucleo/diagnosticos/p1307-r6-tests.patch")],
    "preseal_amendments": [amendment],
    "authorization": base["authorization"],
    "unknown_policy": "Unknown is never acceptance; only deliberate opacity controls require Unknown. Historical excluded probes remain recorded, not silently promoted.",
    "observables": "All predecessor R4 obligations except the explicitly approved R5 LocatedContent repr replacement, math provenance, approved R5 snapshot fields/equality/serialization/repr and CBOR fallback; complete public envelopes and four feature profiles.",
    "roles": {**base["roles"],
              "p1307_args_impl": "Implementation only: approved Args and writer owners; no oracle, tests, L0 or verdict writes; fresh context, must wait for seal and semantic RED.",
              "p1307_encoders_impl": "Implementation only: approved loading, namespace registration and repr owners; no oracle, tests, L0 or verdict writes; fresh context, must wait for seal and semantic RED."},
    "regime": base["regime"],
    "candidate_exists": False,
})
print(json.dumps(pin("00_nucleo/diagnosticos/p1307-r6-manifest.json")))
