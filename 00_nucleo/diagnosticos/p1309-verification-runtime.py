#!/usr/bin/env python3
"""Independent role E arithmetic and byte checks; stdout only."""
import sys
sys.dont_write_bytecode = True
import base64
import hashlib
import json
import re
import subprocess
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
pins, failures, checks = {}, [], []
def sha(b):
    return hashlib.sha256(b).hexdigest()
def read(name, expected=None):
    data = (D / name).read_bytes()
    pins[name] = sha(data)
    if expected:
        require("sealed hash " + name, pins[name] == expected)
    return json.loads(data)
def require(name, condition, witness=None):
    checks.append(name)
    if not condition:
        failures.append({"check": name, "witness": witness})
def verify_ref(ref):
    p = Path(ref["path"])
    p = p if p.is_absolute() else ROOT / p
    require("reference " + ref["path"], sha(p.read_bytes()) == ref["sha256"])
def key(row):
    return row["id"], row["profile"]
def raw_class(row):
    v, c = row["vanilla"], row["crystalline"]
    if any(not s["complete"] or s["reason_code"] is not None or s["exit_code"] not in (0, 1) for s in (v, c)):
        return "EXECUTION_UNKNOWN"
    if v["exit_code"] == c["exit_code"] == 0:
        if v["stdout_base64"] != c["stdout_base64"]:
            return "DIFFERENT_VALUE"
        return "MATCH_VALUE" if v["stderr_base64"] == c["stderr_base64"] else "DIFFERENT_DIAGNOSTIC"
    if v["exit_code"] == c["exit_code"]:
        same = all(v[k] == c[k] for k in ("stdout_base64", "stderr_base64"))
        return ("MATCH_" if same else "DIFFERENT_") + ("VALUE" if v["exit_code"] == 0 else "DIAGNOSTIC")
    return "VANILLA_ONLY" if v["exit_code"] == 0 else "CRYSTALLINE_ONLY"
def projection(row):
    names = ("exit_code", "stdout_base64", "stderr_base64", "complete", "reason_code", "source_sha256", "binary_sha256", "features")
    return {side: {n: row[side][n] for n in names} for side in ("vanilla", "crystalline")}

catalog = read("p1309-probe-catalog.json", "20a5bdec9a848d24ebe1fcf451f143ce65499184e57984e4e5389aef5aecde3a")
probes = {p["id"]: p for p in catalog["probes"]}
profiles = ["default", "html", "a11y", "html+a11y"]
expected_keys = [(pid, profile) for pid in sorted(probes) for profile in profiles]
matrix_hashes = {"normal": "ed6b603578ada1a3961bbb564c272652e52b4652231b5d324c8b9afa57889e51", "repeat": "af86b508546934b60511502d3e3768bbe9449cd8c30c4cd573c81e9f2d35c51d", "reverse": "26021665ea875e67f61be8d8f93bc97ecd9f80def748d58fa812da24720207bd"}
maps, phase_counts = {}, {}
for phase, expected in matrix_hashes.items():
    m = read("p1309-matrix-" + phase + ".json", expected)
    require("matrix manifest " + phase, m["manifest_sha256"] == "d7d2f8ebfe7a5453c4fec7417248654a066d9859dd8d29abf259544c888cf9c3")
    require("matrix catalog " + phase, m["catalog_sha256"] == pins["p1309-probe-catalog.json"])
    require("matrix build " + phase, m["build_receipt_sha256"] == "3553d0aa43ee277db4bbba9821f971466e6ad1283c952d0fe44359f7f9641a80")
    require("matrix runner " + phase, sha((D / "p1309-matrix.py").read_bytes()) == m["runner_sha256"])
    require("full ordered keys " + phase, list(map(key, m["results"])) == (list(reversed(expected_keys)) if phase == "reverse" else expected_keys))
    row_issues = []
    for r in m["results"]:
        if r["universe"] != "principal" or r["expression"] != probes[r["id"]]["expression"] or r["runtime_class"] != raw_class(r):
            row_issues.append([*key(r), "row metadata/class"])
        for side in ("vanilla", "crystalline"):
            s = r[side]
            for channel in ("stdout", "stderr"):
                data = base64.b64decode(s[channel + "_base64"], validate=True)
                if sha(data) != s[channel + "_sha256"] or data.decode("utf-8") != s[channel]:
                    row_issues.append([*key(r), side, channel])
            args = [m["binaries"][side]["path"], "eval", r["expression"], "--format", "json"]
            if catalog["profiles"][r["profile"]]:
                args += ["--features", ",".join(catalog["profiles"][r["profile"]])]
            if s["argv"] != args or s["features"] != catalog["profiles"][r["profile"]] or s["source_sha256"] != sha(r["expression"].encode()) or s["binary_sha256"] != m["binaries"][side]["sha256"]:
                row_issues.append([*key(r), side, "argv/feature/source/binary"])
            if s["exit_code"] == 0:
                try:
                    if json.loads(s["stdout"]) != s["parsed_value"]:
                        row_issues.append([*key(r), side, "JSON parse"])
                except Exception:
                    row_issues.append([*key(r), side, "JSON failure"])
    require("every row raw envelope " + phase, not row_issues, row_issues[:20])
    counts = Counter(raw_class(r) for r in m["results"])
    require("counts " + phase, counts == m["counts"] and counts["EXECUTION_UNKNOWN"] == 0)
    maps[phase] = {key(r): projection(r) for r in m["results"]}
    phase_counts[phase] = dict(counts)
    if phase == "normal":
        normal = m
require("exact independent three order stability", maps["normal"] == maps["repeat"] == maps["reverse"])
stability = read("p1309-stability.json", "3479b62231013a313493db0966109e3409d142dc0bb49d99107a73d0210df3df")
historical = read("p1304-feature-matrix.json")
current = {key(r): r for r in normal["results"]}
transitions = Counter()
regressions = []
for r in historical["results"]:
    now = current[key(r)]
    require("historical expression identity " + str(key(r)), r["expression"] == now["expression"])
    transitions[r["runtime_class"] + " -> " + now["runtime_class"]] += 1
    if r["runtime_class"].startswith("MATCH_") and not now["runtime_class"].startswith("MATCH_"):
        regressions.append(key(r))

sentinel = read("p1309-sentinels.json", "2af42cf65e4d59f0d199528a37a7bc7efb994d1c39bbda914b977a194618ce65")
for ref in sentinel["inputs"]:
    verify_ref(ref)
require("supplement denominator disjoint", len(sentinel["rows"]) == sentinel["pairs"] == 2170 and all(r["universe"] == "supplement" for r in sentinel["rows"]) and sentinel["principal_probe_contribution"] == 0)
require("supplement counts", Counter(r["bilateral_class"] for r in sentinel["rows"]) == sentinel["bilateral_counts"])
oracle = read("p1308-r2-oracle.json")
expected_oracle = {(c["id"], p): o["future_expected"] for c in oracle["cases"] for p, o in c["observations"].items()}
replay = json.loads(read("p1309-sentinels-p1308-crystalline.json")["stdout"])
oracle_mismatches = [ [r["case"], r["profile"]] for r in replay["rows"] if r["observable"] != expected_oracle[(r["case"], r["profile"])]]
require("P1308 envelopes independently equal frozen expected", len(replay["rows"]) == len(expected_oracle) == 1982 and not oracle_mismatches, oracle_mismatches[:20])
require("module independent envelope preservation", len(sentinel["module_preservation"]) == 148 and all(r["actual"] == r["expected"] for r in sentinel["module_preservation"]))
require("full array data independent", len(sentinel["array_integrity"]) == 48 and all(r["values"] == list(range(r["length"])) for r in sentinel["array_integrity"]))
extra = read("p1309-sentinels-extra.json")
extra_maps = {p: {key(r): projection(r) for r in extra["rows"] if r["phase"] == p} for p in matrix_hashes}
require("extra three orders independent", all(len(m) == 188 for m in extra_maps.values()) and extra_maps["normal"] == extra_maps["repeat"] == extra_maps["reverse"])
require("extra raw class recalculated", all(raw_class(r) == r["runtime_class"] for r in extra["rows"]))

gates = read("p1309-gates.json", "3bfa71d097287c8948e824987c6f60ff257c80d0994b66d228339043b18feb80")
for name, ref in gates["commands"].items():
    verify_ref(ref)
    receipt = json.loads((ROOT / ref["path"]).read_text())
    require("gate receipt exit and manifest " + name, receipt["exit"] == 0 and receipt["manifest_sha256"] == normal["manifest_sha256"])
tests = read("p1309-workspace-tests-r2.json")
test_summaries = re.findall(r"test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;", tests["stdout"])
test_counts = dict(zip(("passed", "failed", "ignored"), [sum(int(r[i]) for r in test_summaries) for i in range(3)]))
require("workspace independent counts", test_counts == gates["workspace_tests"])
ignored_names = [line for line in tests["stdout"].splitlines() if re.match(r"test .* \.\.\. ignored", line)]
require("ignored names retained", ignored_names == gates["ignored_test_names"])
lint = read("p1309-lint-r2.json")
severity_counts = {s: len(re.findall(r"^" + s + ":", lint["stdout"] + lint["stderr"], re.M)) for s in ("error", "warning", "info")}
require("lint independent findings", severity_counts == gates["lint_severities"])
baseline = read("p1309-baseline-r2.json")
changes = [p for p, h in baseline["product_inventory"].items() if sha((ROOT / p).read_bytes()) != h]
require("final product bytes", not changes, changes)
source = subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=ROOT, text=True)
require("current tracked tree clean", not source)
print(json.dumps({"schema": "p1309-verification-runtime-v1", "role": "E", "at": datetime.now(timezone.utc).isoformat(), "head": baseline["state"]["head"], "tracked_diff_stat": source, "manifest_sha256": normal["manifest_sha256"], "pins": pins, "check_count": len(checks), "failures": failures, "principal_pairs_each_order": len(expected_keys), "principal_counts_each_order": phase_counts, "historical_transitions": dict(transitions), "historical_regressions": regressions, "supplement_pairs": sentinel["pairs"], "supplement_bilateral_counts": sentinel["bilateral_counts"], "oracle_preserved": len(replay["rows"]) - len(oracle_mismatches), "module_preserved": len(sentinel["module_preservation"]), "array_integrity_entries": len(sentinel["array_integrity"]), "workspace_tests": test_counts, "ignored_names": ignored_names, "lint_findings": severity_counts, "regime": "executado sem atestação de isolamento técnico", "pending": ["sealed semantic ledger and selection", "adversarial 18/18 gate", "certificate"], "limitation": "P1308 preservation compares frozen normalized full language envelopes; raw side outputs retained. This is not raw bilateral equality or complete language parity.", "verdict": "RUNTIME_CHECKS_ONLY_NO_CERTIFICATE"}, ensure_ascii=False, indent=2))
