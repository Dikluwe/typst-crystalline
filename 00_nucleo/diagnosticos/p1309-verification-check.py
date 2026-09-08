#!/usr/bin/env python3
"""Role E read-only checks. JSON stdout only; never changes judged artifacts."""
import sys
sys.dont_write_bytecode = True
import csv
import hashlib
import json
import subprocess
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
checks = []
pins = {}

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def read(name):
    p = D / name
    pins[name] = digest(p)
    return json.loads(p.read_text())

def check(name, ok, detail=None):
    checks.append({"name": name, "ok": bool(ok), "detail": detail})

def pin(ref):
    p = Path(ref["path"])
    if not p.is_absolute():
        p = ROOT / p
    try:
        got = digest(p)
        check("hash:" + ref["path"], got == ref["sha256"], got)
    except OSError as e:
        check("hash:" + ref["path"], False, str(e))

def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True)

baseline = read("p1309-baseline-r2.json")
manifest = read("p1309-manifest-r2.json")
incident = read("p1309-inventory-incident.json")
build = read("p1309-build-r2.json")
catalog = read("p1309-probe-catalog.json")
reconciliation = read("p1309-inventory-reconciliation.json")
author = read("p1309-inventory-author-receipt-r2.json")
historical = read("p1304-probe-catalog.json")
check("baseline_pin", pins["p1309-baseline-r2.json"] == "68dd669aba8d94ef80eca8a2bdf7f35ed27c60cf36f2a829ae03b1017d555bce")
check("manifest_pin", pins["p1309-manifest-r2.json"] == "d7d2f8ebfe7a5453c4fec7417248654a066d9859dd8d29abf259544c888cf9c3")
check("build_pin", pins["p1309-build-r2.json"] == "3553d0aa43ee277db4bbba9821f971466e6ad1283c952d0fe44359f7f9641a80")
check("catalog_pin", pins["p1309-probe-catalog.json"] == "20a5bdec9a848d24ebe1fcf451f143ce65499184e57984e4e5389aef5aecde3a")
check("incident_preserved", pins["p1309-inventory-incident.json"] == baseline["incident_sha256"] and incident["status"] == "ATTEMPT_INVALIDATED_ALLOWLIST_WRITE")
pin({"path": incident["recovery"]["recoverable_path"], "sha256": incident["sha256"]})
check("incident_original_cache_absent", not (ROOT / incident["out_of_allowlist_path"]).exists())
check("successor_baseline_chain", digest(D / "p1309-baseline.json") == baseline["supersedes_invalid_attempt"])
check("successor_manifest_chain", digest(D / "p1309-manifest.json") == manifest["supersedes_invalid_attempt"])
check("fresh_build_after_refreeze", baseline["at"] < manifest["at"] < build["at"] < build["end"])
check("build_success_and_target", build["exit"] == 0 and build["env"]["CARGO_TARGET_DIR"] == baseline["fresh_target"] and build["argv"] == ["cargo", "build", "--workspace", "--release"])
check("build_link", build["manifest_sha256"] == pins["p1309-manifest-r2.json"])
pin(build["candidate"])
pin(baseline["vanilla"])
pin(manifest["step"])
for phase in (baseline["state"], build["before"], build["after"]):
    check("recorded_tracked_staged_clean", not phase["diff"] and not phase["staged"] and phase["head"] == baseline["state"]["head"])
for name, value in baseline["inputs"].items():
    pin({"path": str(D / name), "sha256": value})
product_changed = [p for p, sha in baseline["product_inventory"].items() if not (ROOT / p).is_file() or digest(ROOT / p) != sha]
check("product_inventory_unchanged", not product_changed, {"count": len(baseline["product_inventory"]), "changed": product_changed})
status = git("status", "--short")
outside = [line for line in status.splitlines() if not line[3:].startswith("00_nucleo/diagnosticos/p1309-") and line[3:] not in baseline["initial_user_untracked"]]
check("current_write_allowlist", not outside, outside)
check("current_tracked_staged_clean", not git("diff", "HEAD", "--stat") and not git("diff", "--cached", "--stat"))
check("current_head", git("rev-parse", "HEAD").strip() == baseline["state"]["head"])
for ref in author["inputs"] + author["scripts"] + author["outputs"] + [author["baseline_receipt"], author["manifest"]]:
    pin(ref)
probes = catalog["probes"]
probe_ids = {p["id"] for p in probes}
probe_paths = {p["path"] for p in probes}
check("historical627_verbatim_ordered", len(historical["probes"]) == 627 and probes[:627] == historical["probes"])
check("catalog_count_unique_ids", len(probe_ids) == len(probes) == catalog["counts"]["probes"])
check("profile_definitions", catalog["profiles"] == {"default": [], "html": ["html"], "a11y": ["a11y-extras"], "html+a11y": ["html", "a11y-extras"]})
check("every_probe_four_profiles", all(set(p["profiles"]) == set(catalog["profiles"]) for p in probes))
union = set()
measurements = []
for source in catalog["inventory_inputs"]:
    pin(source)
    pin(source["language_observations"])
    inv = json.loads((ROOT / source["path"]).read_text())
    obs = json.loads((ROOT / source["language_observations"]["path"]).read_text())
    union.update(inv["entries"])
    side, profile = source["side"], source["profile"]
    check("inventory_side_profile:" + side + ":" + profile, inv["side"] == obs["side"] == side and inv["profile"] == obs["profile"] == profile)
    check("inventory_observation_keys:" + side + ":" + profile, set(inv["entries"]) == set(obs["entries"]))
    check("inventory_observation_source_pin:" + side + ":" + profile, obs["structural_input"]["sha256"] == source["sha256"])
    check("inventory_observation_unknowns:" + side + ":" + profile, not obs["unknowns"] and all(e["observation"] != "EXECUTION_UNKNOWN" and e["execution"]["unknown"] is None for e in obs["entries"].values()))
    expected_product = {k: baseline["vanilla"][k] for k in ("path", "sha256")} if side == "vanilla" else build["candidate"]
    check("inventory_observation_binary:" + side + ":" + profile, obs["product"] == expected_product)
    measurements.append({"side": side, "profile": profile, "structural_entries": len(inv["entries"]), "observed_entries": len(obs["entries"])})
check("fresh_union_covered", union <= probe_paths, {"union_count": len(union), "missing": sorted(union - probe_paths)})
check("reconciliation_added_exact", {p["id"] for p in probes[627:]} == {p["probe_id"] for p in reconciliation["added"]})
check("reconciliation_no_silent_removal", all(not reconciliation[k] for k in ("removed", "renamed", "split", "merged")))
with (D / "p1304-owner-ledger.tsv").open() as f:
    old_rows = list(csv.DictReader(f, delimiter="\t"))
check("historical167_paths_reconciled", len(old_rows) == len(reconciliation["historical_paths"]) == 167 and {r["path"] for r in old_rows} == {r["historical_path"] for r in reconciliation["historical_paths"]})
check("blocked_ancestors_executable", all(r["executable_probe_id"] in probe_ids for r in reconciliation["blocked_by_ancestor"]))
check("catalog_no_unknowns", not catalog["unknowns"] and not reconciliation["unknowns"])
print(json.dumps({"schema": "p1309-verification-preliminary-v1", "verifier": "/root/p1309_verifier", "at": datetime.now(timezone.utc).isoformat(), "head": git("rev-parse", "HEAD").strip(), "source_diff_stat": git("diff", "HEAD", "--stat"), "manifest_sha256": pins["p1309-manifest-r2.json"], "pins": pins, "checks": checks, "measurements": measurements, "failed": [c for c in checks if not c["ok"]], "pending": ["three complete matrix orders", "sentinel combined receipt", "semantic ledger and transitions", "deterministic selection", "18 valid auditor mutations rejected", "complete gates and final byte identity"], "verdict": "PRELIMINARY_ONLY_NO_CERTIFICATE", "regime": "executado sem atestação de isolamento técnico"}, ensure_ascii=False, indent=2))
