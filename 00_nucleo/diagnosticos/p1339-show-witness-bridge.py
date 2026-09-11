"""Describe the causal bridge without rewriting prior Unknown or issuing verdicts."""
from collections import Counter
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
PREFIX = "p1339-show-witness-"

def read(name):
    return json.loads((BASE / name).read_text())

def sha(path):
    with open(path, "rb") as handle:
        return hashlib.file_digest(handle, "sha256").hexdigest()

def save(name, data):
    path = BASE / name
    assert name.startswith(PREFIX) and not path.exists()
    text = json.dumps(data, ensure_ascii=False, indent=2)
    patch = "*** Begin Patch\n*** Add File: " + str(path) + "\n" + "".join("+" + line + "\n" for line in text.splitlines()) + "*** End Patch\n"
    subprocess.run(["apply_patch"], input=patch, text=True, capture_output=True, check=True)

old_manifest_name = "p1339-full-show-final-manifest.json"
old_runs_name = "p1339-full-show-final-vanilla-runs.json"
manifest_name = PREFIX + "final-manifest.json"
observation_name = PREFIX + "final-observation.json"
old_manifest, old_runs = read(old_manifest_name), read(old_runs_name)
manifest, observation = read(manifest_name), read(observation_name)
results = {p: read(PREFIX + "final-" + p + "-runs.json") for p in ("vanilla", "baseline")}
assert observation["manifest_sha256"] == sha(BASE / manifest_name)
for path, digest in manifest["inputs"].items():
    assert sha(ROOT / path) == digest
for product in results:
    assert observation["runs_sha256"][product] == sha(BASE / (PREFIX + "final-" + product + "-runs.json"))
bridge = []
for case in manifest["cases"]:
    if case["kind"] != "selector":
        continue
    original, = [c for c in old_manifest["cases"] if c["id"] == case["id"]]
    reconstructed = original["source"].replace('metadata("MATCH")', 'panic("' + case["marker"] + '")')
    assert reconstructed == case["source"]
    for prior in [r for r in old_runs["rows"] if r["id"] == case["id"]]:
        actual = {}
        for product in results:
            index, row = next((i, r) for i, r in enumerate(results[product]["rows"]) if (r["id"], r["profile"], r["order"]) == (prior["id"], prior["profile"], prior["order"]))
            actual[product] = {"receipt": PREFIX + "final-" + product + "-runs.json", "row_index_zero_based": index, "exit": row["exit"], "effect": row["observed_effect"], "primary_diagnostic": row["primary_diagnostic"]}
        bridge.append({"id": case["id"], "profile": prior["profile"], "order": prior["order"], "prior_source_sha256": original["sha256"], "new_source_sha256": case["sha256"], "prior_query_stdout": prior["stdout"], "prior_cell_reported_transport_Unknown": prior["profile"] != "default", "prior_status_mutated": False, "replacement_body_only_verified": True, "new_expected_effect": case["expected_effect"], "observed": actual, "scope": "Successor compile observes the same selector and subject but a sentinel body; prior metadata output itself is not reproduced. Baseline selector rejection is witnessed before callback; baseline matching is not demonstrated."})
controls = [{"id": row["id"], "profile": row["profile"], "order": row["order"], "effect": row["observed_effect"], "expected_effect": row["expected_effect"], "primary_diagnostic": row["primary_diagnostic"]} for row in results["baseline"]["rows"] if row["kind"] == "direct-control"]
names = [old_manifest_name, old_runs_name, manifest_name, observation_name, PREFIX + "probe.py", PREFIX + "probe-r1.py", PREFIX + "focal-manifest.json", PREFIX + "focal-vanilla-runs.json", PREFIX + "focal-r1-manifest.json", PREFIX + "focal-r1-vanilla-runs.json", PREFIX + "focal-r1-baseline-runs.json", PREFIX + "focal-r1-observation.json", PREFIX + "final-vanilla-runs.json", PREFIX + "final-baseline-runs.json"]
data = {"schema": "p1339-show-witness-causal-bridge-v1", "utc": datetime.datetime.now(datetime.timezone.utc).isoformat(), "author": "/root/p1312_tests", "regime": "Exploratory independent observation without technical isolation attestation", "hashes": {name: sha(BASE / name) for name in names}, "bridge_script_sha256": sha(__file__), "bridge": bridge, "baseline_direct_controls": controls, "summary": {"original_selector_cells": len(bridge), "formerly_transport_Unknown_cells_with_successor_semantic_observation": sum(b["prior_cell_reported_transport_Unknown"] and b["observed"]["baseline"]["effect"] == "OtherDiagnosticBeforeCallback" for b in bridge), "original_Unknown_cells_rewritten": 0, "new_transport_Unknown": len(observation["unknown"]), "order_instability": len(observation["instability"]), "text_direct_control_counterexamples": sum(c["id"] == "text-direct-control" and c["effect"] != c["expected_effect"] for c in controls), "vanilla_prediction_mismatches": sum(r["observed_effect"] != r["expected_effect"] for r in results["vanilla"]["rows"]), "baseline_primaries": dict(Counter(r["primary_diagnostic"] for r in results["baseline"]["rows"]))}, "timing_and_git": {p: {"start": d["start"], "end": d["end"], "process_seconds": sum(r["seconds"] for r in d["rows"])} for p, d in results.items()}, "gates_not_claimed": ["No contract or seal", "No productive implementation", "No final PASS", "No technical isolation attestation", "No successful baseline where matching", "No repaired text direct show control", "No original metadata output equality", "No Angle NaN construction", "No HTML target observation"]}
save(PREFIX + "bridge.json", data)
print(json.dumps({"path": PREFIX + "bridge.json", "sha256": sha(BASE / (PREFIX + "bridge.json")), "summary": data["summary"]}, ensure_ascii=False, indent=2))
