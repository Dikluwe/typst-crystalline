#!/usr/bin/env python3
"""P1333 frozen literal CLI oracle: historical 616 cells plus bounded boundary cases."""
import argparse
import datetime
import importlib.util
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
HISTORY_RUNNER = D / "p1332-ab-cli.py"
HISTORY = D / "p1332-ab-cli-expected.json"
spec = importlib.util.spec_from_file_location("p1332_history", HISTORY_RUNNER)
history = importlib.util.module_from_spec(spec)
spec.loader.exec_module(history)
BASE = "/tmp/p1332-target.tTIpWy/release/typst"
VANILLA = "/usr/local/bin/typst"
MANIFEST = D / "p1333-manifest.json"
FROZEN = D / "p1333-ab-cli-expected.json"
NORM = "517e1544fbd419a5b1c13e524c02570d3c22606a7b97d25562723c278bdd2e83"
digest, command, observed, normhash = history.digest, history.command, history.observed, history.normhash
PROFILES = history.PROFILES
MIGRATE = {
    "named-before-arity", "multiple", "overflow-named-guard-debt",
    "overflow-arity-guard-debt", "overflow-named-before-arity-debt",
    "fallback-named-guard", "fallback-arity-guard", "fallback-named-before-arity",
    "name-with-named-guard", "name-with-arity-guard",
}
CASES = list(history.CASES) + [
    ("precedence-content-named-before", "first-invalid-parity", "calc.abs(bad: 1, [x], 2)"),
    ("precedence-content-named-after", "first-invalid-parity", "calc.abs([x], bad: 1)"),
    ("precedence-mixed-positive-extra", "first-invalid-parity", "calc.abs(2pt + 3em, 2)"),
    ("precedence-mixed-negative-named", "first-invalid-parity", "calc.abs(bad: 1, -2pt - 3em, 2)"),
    ("precedence-overflow-import-with", "first-invalid-parity", "{import calc: abs; let a=abs.with(bad: 1).with(-9223372036854775807 - 1); a(2)}"),
    ("precedence-fallback-alias", "first-invalid-parity", "{let a=calc.abs; a(false, bad: 1, 2)}"),
    ("precedence-fallback-array-spread", "first-invalid-parity", "calc.abs(..(false, 2), bad: 1)"),
    ("precedence-mixed-arguments-spread", "first-invalid-parity", "{let aa=arguments(bad: 1, 2pt + 3em, 2); calc.abs(..aa)}"),
    ("precedence-warning-utf8", "first-invalid-parity", "{import std; let café=[á];\n calc.abs(bad: 1, café, 2)}"),
    ("precedence-valid-integer-invalid-second", "preserved-debt", "calc.abs(-19, false)"),
    ("precedence-valid-float-named-before", "preserved-debt", "calc.abs(bad: 1, -1.25, false)"),
    ("precedence-valid-decimal-named", "preserved-debt", "calc.abs(decimal(\"-1.25\"), bad: 1)"),
    ("precedence-valid-length-mixed-second", "preserved-debt", "calc.abs(-2pt, 2pt + 3em)"),
    ("precedence-valid-angle-named", "preserved-debt", "calc.abs(-810deg, bad: 1)"),
    ("precedence-valid-ratio-named", "preserved-debt", "calc.abs(-250%, bad: 1)"),
    ("precedence-valid-fraction-overflow-second", "preserved-debt", "calc.abs(-2fr, -9223372036854775807 - 1)"),
    ("precedence-valid-infinity-named", "preserved-debt", "calc.abs(-calc.inf, bad: 1)"),
    ("precedence-no-positional-named-value", "preserved-debt", "calc.abs(value: false)"),
    ("precedence-no-positional-named", "preserved-debt", "calc.abs(bad: 1)"),
    ("precedence-empty-spread", "preserved-debt", "calc.abs(..())"),
    ("precedence-eager-extra", "preserved-parity", "calc.abs(false, panic(\"p1333-eager\"))"),
    ("precedence-eager-named-after", "preserved-parity", "calc.abs(false, bad: panic(\"p1333-eager\"))"),
    ("precedence-eager-named-before", "preserved-parity", "calc.abs(bad: panic(\"p1333-eager\"), false)"),
    ("precedence-math-import-known-debt", "preserved-debt-imported-math-resolution", "{import calc: abs; $abs(-1, 2)$}"),
]

def save(path, value):
    target = pathlib.Path(path).resolve()
    assert target.parent == D and target.name.startswith("p1333-ab-")
    assert not target.exists(), "immutable evidence: choose new output"
    content = json.dumps(value, indent=2, ensure_ascii=False) + "\n"
    patch = "*** Begin Patch\n*** Add File: " + str(target) + "\n" + "".join("+" + line + "\n" for line in content.splitlines()) + "*** End Patch\n"
    subprocess.run(["apply_patch"], input=patch, text=True, check=True, capture_output=True)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--candidate")
    ap.add_argument("--output", required=True)
    ap.add_argument("--order", choices=["normal", "reverse"], default="normal")
    ap.add_argument("--freeze-from")
    args = ap.parse_args()
    assert normhash() == NORM
    assert digest(MANIFEST) == "75eea7c2a28cd8c75098d4d8fc5b2c733c30c09b7eca52a2ce7a772a24e0e99b"
    assert digest(BASE) == "dfe7c3ab89eaa145d79e5b56ac72c657e49c6c994f9a88a4338915808ec0e8ea"
    assert digest(VANILLA) == "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
    if args.freeze_from:
        assert not args.candidate
        baseline = json.loads(pathlib.Path(args.freeze_from).read_text())
        historical = {(r["case"], r["profile"]): r for r in json.loads(HISTORY.read_text())["expectations"]}
        expectations = []
        migrated = []
        for row in baseline["cases"]:
            key = (row["case"], row["profile"])
            before = observed(row["results"]["BASE"])
            cls = next(c for n,c,e in CASES if n == row["case"])
            if key in historical:
                assert before == historical[key]["expected"], key
                cls = historical[key]["classification"]
            if row["case"] in MIGRATE or cls == "first-invalid-parity":
                oracle = observed(row["results"]["VANILLA"])
                assert oracle["exit"] != 0, key
                cls = "first-invalid-parity"
                migrated.append({"case": row["case"], "profile": row["profile"], "historical": key in historical, "before": before, "after": oracle})
            else:
                oracle = before
                if cls == "preserved-parity":
                    assert oracle == observed(row["results"]["VANILLA"]), key
            expectations.append({"case": row["case"], "profile": row["profile"], "classification": cls, "expected": oracle})
        assert len(historical) == 616
        assert len(expectations) == len(CASES) * len(PROFILES)
        save(args.output, {"utc": datetime.datetime.now(datetime.timezone.utc).isoformat(), "manifest_sha256": digest(MANIFEST), "l0_norm_sha256": normhash(), "baseline_sha256": digest(args.freeze_from), "historical_expectations_sha256": digest(HISTORY), "policy": "All 616 historical cells verified against BASE before migration. Only listed first-invalid expectations migrate to literal VANILLA; all other baseline observations remain exact debt. Full exit/stdout/stderr, no candidate normalization, mandatory Unknown blocks.", "migration_ledger": migrated, "expectations": expectations})
        return
    binaries = {"BASE": BASE, "VANILLA": VANILLA}
    if args.candidate:
        binaries["CANDIDATE"] = args.candidate
    frozen = {(r["case"],r["profile"]):r for r in json.loads(FROZEN.read_text())["expectations"]} if args.candidate else {}
    receipt = {"utc": datetime.datetime.now(datetime.timezone.utc).isoformat(), "head": command(["git", "rev-parse", "HEAD"])["stdout"].strip(), "diff_stat": command(["git", "diff", "HEAD", "--stat"])["stdout"], "working_tree": "uncommitted; no runtime content inspected", "manifest_sha256": digest(MANIFEST), "l0_norm_sha256": normhash(), "public_baseline_sha256": digest(D / "p1333-baseline-public.json"), "runner_sha256": digest(__file__), "history_runner_sha256": digest(HISTORY_RUNNER), "expected_sha256": digest(FROZEN) if args.candidate else None, "order":args.order, "binaries":{k:{"path":v,"sha256":digest(v)} for k,v in binaries.items()}, "cases":[]}
    for name, cls, expr in CASES if args.order == "normal" else reversed(CASES):
        for profile, flags in PROFILES:
            results = {k:command([v,"--color","never","eval",*flags,expr]) for k,v in binaries.items()}
            row = {"case":name,"classification":cls,"expression":expr,"profile":profile,"results":results}
            if args.candidate:
                expectation = frozen[(name,profile)]
                row.update(classification=expectation["classification"], expected=expectation["expected"], candidate_matches_frozen_policy=observed(results["CANDIDATE"]) == expectation["expected"])
            receipt["cases"].append(row)
    save(args.output, receipt)
    failures = [r["case"]+"/"+r["profile"] for r in receipt["cases"] if r.get("candidate_matches_frozen_policy") is False]
    print(json.dumps({"observations":len(receipt["cases"]),"candidate_failures":failures}))
    if failures:
        raise SystemExit(1)

if __name__ == "__main__":
    main()
