#!/usr/bin/env python3
"""E independently reconstructs anchors and ledger arithmetic; stdout only."""
import sys
sys.dont_write_bytecode = True
import csv
import hashlib
import json
import re
from collections import Counter, defaultdict
from datetime import datetime, timezone
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
pins, failures, checks = {}, [], []
def sha(b): return hashlib.sha256(b).hexdigest()
def check(name, ok, witness=None):
    checks.append(name)
    if not ok: failures.append({"name": name, "witness": witness})
def load(name, expected=None):
    data = (D / name).read_bytes()
    pins[name] = sha(data)
    if expected: check("pin " + name, pins[name] == expected)
    if name.endswith(".tsv"):
        return list(csv.DictReader(data.decode().splitlines(), delimiter="\t"))
    return json.loads(data)
ledger = load("p1309-owner-ledger.tsv", "36deab222babd8edbf90b030a56977482d28c13c6b15a55b01f9ea546c7f23f4")
transitions = load("p1309-transition-ledger.tsv", "8922aab7c85d3dfe69cf5914c5fff29b24a1ac8d82883e71b5949d171d55ce2b")
selection = load("p1309-selection.json", "72e0505ebe6106dd458fa89555f0667c50a10eca8ef1135fd7008559d89aa808")
summary = load("p1309-classification-summary.json", "0b5751799e50f07190322754582779b2ece69704cc79b28e394bc7459d543a1f")
sources = load("p1309-classification-sources.json", "982fc7f73f978554f3b42ae4617e7f9f64a4432591b60027340f1b91f47b11a9")
anchors = load("p1309-audit-anchors.json", "881fa34090aaad636eced997bfa383d595dc665aaddbc2750989c1c9a18a9d15")
bundle = load("p1309-audit-bundle.json", "eeaa851c41ce226436ccb6ef729aacac61e59bdaed444927d7d894ca4dd3e4b2")
catalog = load("p1309-probe-catalog.json")
historical = load("p1304-probe-catalog.json")
old_matrix = load("p1304-feature-matrix.json")
old_ledger = load("p1304-owner-ledger.tsv")
sentinels = load("p1309-sentinels.json")
baseline = load("p1309-baseline-r2.json")
check("embedded anchors exact external frozen anchors", bundle["anchors"] == anchors)
check("bundle ledger exact sealed TSV", bundle["ledger"] == ledger)
check("bundle selection exact sealed selection", bundle["selection"] == selection)
check("bundle catalog exact sealed catalog", bundle["catalog"] == catalog)
check("historical anchors verbatim", anchors["historical_probes"] == historical["probes"])
check("principal anchors verbatim", anchors["principal_probes"] == catalog["probes"])
check("product baseline anchors exact", anchors["product_inventory"] == baseline["product_inventory"])
for ref in anchors["source_artifacts"]:
    check("anchor source " + ref["path"], sha((ROOT / ref["path"]).read_bytes()) == ref["sha256"])
raw, runtime = {}, defaultdict(dict)
for phase in ("normal", "repeat", "reverse"):
    m = load("p1309-matrix-" + phase + ".json")
    for row in m["results"]:
        for side in ("vanilla", "crystalline"):
            s = row[side]
            raw["|".join((phase, row["id"], row["profile"], side))] = [s["exit_code"], sha(s["stdout"].encode()), sha(s["stderr"].encode())]
        if phase == "normal": runtime[row["id"]][row["profile"]] = row["runtime_class"]
    if phase == "normal": normal = m
check("anchors raw all channels reconstructed", anchors["raw_channels"] == raw)
check("anchors runtime from matrix", anchors["runtime_by_probe"] == runtime)
old_classes = {r["id"] + "|" + r["profile"]: r["runtime_class"] for r in old_matrix["results"]}
check("anchors historical classes", anchors["historical_classes"] == old_classes)
sentinel_channels = {}
for row in sentinels["rows"]:
    for side in ("vanilla", "crystalline"):
        s = row[side]
        sentinel_channels["|".join((row["id"], row["profile"], side))] = [s.get("exit_code", s.get("returncode")), sha(s["stdout"].encode()), sha(s["stderr"].encode())]
check("anchors supplemental raw reconstructed", anchors["sentinel_raw_channels"] == sentinel_channels)
inventory_union = set()
for inv in catalog["inventory_inputs"]:
    inventory_union.update(json.loads((ROOT / inv["path"]).read_text())["entries"])
check("anchors inventory completeness", set(anchors["inventory_paths"]) == inventory_union)
owners = defaultdict(list)
for p in baseline["product_inventory"]:
    if p.endswith(".rs") and p.startswith(("01_core/", "02_shell/", "03_infra/", "04_wiring/")):
        for match in re.finditer(r"@prompt\s+(00_nucleo/prompts/\S+)", (ROOT / p).read_text()[:12000]):
            owners[match.group(1)].append(p)
check("anchor ownership reproduced from product headers", dict(owners) == anchors["ownership"], {"missing": sorted(set(anchors["ownership"]) - set(owners)), "extra": sorted(set(owners) - set(anchors["ownership"]))})
check("ownership all unique", all(len(v) == 1 for v in owners.values()))
for prompt in sources["prompts"]:
    check("C prompt sealed hash " + prompt["path"], sha((ROOT / prompt["path"]).read_bytes()) == prompt["sha256"])
    check("C prompt owner " + prompt["path"], owners[prompt["path"]] == prompt["consumers"])
    check("C consumer hash " + prompt["path"], sha((ROOT / prompt["consumers"][0]).read_bytes()) == prompt["consumer_sha256"])
for path, nucleus in anchors["nuclei"].items():
    check("nucleus raw seal " + path, sha((ROOT / path).read_bytes()) == nucleus["raw_sha256"])
for pin in anchors["nucleus_pins"]:
    check("nucleus pin " + pin["owner"] + ":" + pin["path"], pin["path"] in anchors["nuclei"] and anchors["nuclei"][pin["path"]]["sha256"] == pin["sha256"] and pin["path"] + " sha256:" + pin["sha256"] in (ROOT / pin["owner"]).read_text())
principal = [r for r in ledger if r["universe"].startswith("principal")]
principal_by_id = {r["probe_id"]:r for r in principal}
check("ledger principal bijection", len(principal) == len(principal_by_id) == len(catalog["probes"]) and set(principal_by_id) == set(runtime))
row_errors = []
for r in principal:
    recorded = dict(p.split(":",1) for p in r["runtime_class_by_profile"].split(";"))
    if recorded != runtime[r["probe_id"]]: row_errors.append(r["probe_id"])
check("ledger runtime exact fresh matrix", not row_errors, row_errors)
principal_counts = Counter(r["current_language_class"] for r in principal)
historical_paths = {r["path"] for r in old_ledger}
hist = [r for r in principal if r["path"] in historical_paths]
hist_counts = Counter(r["current_language_class"] for r in hist)
check("principal semantic recount", principal_counts == summary["decision"]["principal_unique_path_counts"])
check("historical semantic recount", len(hist) == 167 and hist_counts == summary["decision"]["historical_semantic_counts"])
trans_errors = []
for r in transitions:
    prior = old_classes.get(r["probe_id"] + "|" + r["profile"], "NOT_IN_P1304")
    now = runtime[r["probe_id"]][r["profile"]]
    if now != r["current_runtime_class"] or (prior != "NOT_IN_P1304" and prior != r["previous_runtime_class"]): trans_errors.append(r["probe_id"])
check("transition reconstruction", len(transitions) == 8728 and not trans_errors, trans_errors[:10])
extension_rows = [r for r in principal if r["current_language_class"] == "INTENTIONAL_PRODUCT_EXTENSION"]
check("extensions documented not inferred from execution", len(extension_rows) == 39 and all(r["path"] in sources["extension_normative_evidence"] and anchors["normative_facts"].get(r["path"],{}).get("INTENTIONAL_PRODUCT_EXTENSION") for r in extension_rows))
unresolved = sorted(r["path"] for r in principal if r["current_language_class"] == "UNRESOLVED")
check("unresolved unexcluded", unresolved == ["calc.deg","calc.log10","calc.rad"])
matches = sum(r["runtime_class"].startswith("MATCH_") for r in normal["results"])
excluded_ids = {r["probe_id"] for r in extension_rows}
adjusted = [r for r in normal["results"] if r["id"] not in excluded_ids]
metrics = {"raw": {"numerator":matches,"denominator":len(normal["results"]),"percentage":100*matches/len(normal["results"])},"adjusted": {"numerator":sum(r["runtime_class"].startswith("MATCH_") for r in adjusted),"denominator":len(adjusted),"percentage":100*sum(r["runtime_class"].startswith("MATCH_") for r in adjusted)/len(adjusted)}}
for name, value in metrics.items():
    reported = summary["metrics"][name]
    check("metric arithmetic " + name, all(abs(value[k]-reported[k]) < 1e-10 for k in value))
ranked = []
priority_by_class = {"NEW_REGRESSION":1,"DIAGNOSTIC_DIVERGENCE":3,"WRONG_PUBLIC_VALUE":4,"WRONG_PUBLIC_REPR":4,"WRONG_PUBLIC_KIND_OR_IDENTITY":4}
for cohort in selection["cohorts"]:
    facts = anchors["cohort_facts"][cohort["id"]]
    associated = [r for r in ledger if r["cohort_id"] == cohort["id"]]
    check("cohort semantic facts from ledger " + cohort["id"], sorted({r["current_language_class"] for r in associated}) == facts["semantic_classes"])
    classes = facts["semantic_classes"]
    priority = min(priority_by_class.get(c, 2 if c == "L0_CONTRADICTION" and facts["canonical_route_demonstrated"] else 7 if c == "L0_CONTRADICTION" else 5 if c == "MISSING_LANGUAGE_MEMBER" and facts["carriers_existing"] else 6 if c == "MISSING_LANGUAGE_MEMBER" else 99) for c in classes)
    expected_rank = [priority,len(set(cohort["owners"])), -len(set(cohort["paths"])), cohort["regression_surface"]["rank"],cohort["id"]]
    check("cohort priority and tie break " + cohort["id"], cohort["rank_key"] == expected_rank and cohort["owners"] == facts["owners"] and cohort["paths"] == facts["paths"])
    if cohort["eligible"]: ranked.append(expected_rank)
check("deterministic winner", min(ranked)[-1] == selection["selected_cohort"] == "loader-data-source-cast")
debt = load("p1309-certification-debt.json")
print(json.dumps({"schema":"p1309-verification-semantic-v1","role":"E","at":datetime.now(timezone.utc).isoformat(),"head":baseline["state"]["head"],"tracked_diff_stat":"","manifest_sha256":anchors["manifest"]["sha256"],"pins":pins,"check_count":len(checks),"failures":failures,"principal_semantic_counts":dict(principal_counts),"historical_semantic_counts":dict(hist_counts),"metrics":metrics,"normative_unresolved":unresolved,"selected_cohort":selection["selected_cohort"],"minimum_rank":min(ranked),"eligible_cohorts":len(ranked),"ownership_count":len(owners),"nuclei_count":len(anchors["nuclei"]),"nucleus_pin_count":len(anchors["nucleus_pins"]),"limitations":["C independently supplied semantic interpretation; E checks sealed source hashes, ownership, raw measurements, class grounding, priority arithmetic and explicit unresolved exceptions.","Nucleus effective hash algorithm validated by sealed crystalline-lint V26 gate; E independently verifies raw nucleus identities and all declared pins.","No certificate until D valid 18/18 attacks are received."],"regime":"executado sem atestação de isolamento técnico","verdict":"SEMANTIC_CHECKS_ONLY_NO_CERTIFICATE"},ensure_ascii=False,indent=2))
