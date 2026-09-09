#!/usr/bin/env python3
"""P1330 independent literal CLI oracle. Never normalize candidate observations."""
import argparse
import datetime
import hashlib
import importlib.util
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
HISTORY_RUNNER = D / "p1329-ab-cli-r2.py"
HISTORY = D / "p1329-ab-cli-expected-r2.json"
spec = importlib.util.spec_from_file_location("p1329_oracle", HISTORY_RUNNER)
history = importlib.util.module_from_spec(spec)
spec.loader.exec_module(history)
BASE = "/tmp/p1329-target.bg3p5A/release/typst"
VANILLA = "/usr/local/bin/typst"
MANIFEST = D / "p1330-manifest-r2.json"
FROZEN = D / "p1330-ab-cli-expected.json"
NORM = "9a5d734e086297d49d80d567bdb5527919049de19b7aea11f43b4467a972d2db"
M = "-9223372036854775807 - 1"
TRACE_DEBT = {"overflow-with-bound", "overflow-with-nested", "overflow-arguments"}
CASES = [(n, "overflow-correction" if n == "overflow" else c, e) for n, c, e in history.CASES]
CASES += [
    ("overflow-alias", "overflow-correction", "{let ab=calc.abs; ab(" + M + ")}"),
    ("overflow-with-empty", "overflow-correction", "{let ab=calc.abs.with(); ab(" + M + ")}"),
    ("overflow-with-bound", "overflow-correction", "{let ab=calc.abs.with(" + M + "); ab()}"),
    ("overflow-with-nested", "overflow-correction", "{let ab=calc.abs.with(" + M + ").with(); ab()}"),
    ("overflow-array", "overflow-correction", "calc.abs(..(" + M + ",))"),
    ("overflow-arguments", "overflow-correction", "{let aa=arguments(" + M + "); calc.abs(..aa)}"),
    ("overflow-distinct-origins", "overflow-correction", "{let x=" + M + "; let y=" + M + "; calc.abs(y)}"),
    ("overflow-utf8-lines", "overflow-correction", "{let café=" + M + ";\n calc.abs(\n café\n)}"),
    ("overflow-warning", "overflow-correction", "{import std; calc.abs(" + M + ")}"),
    ("overflow-markup-route", "overflow-correction", "[#calc.abs(" + M + ")]"),
    ("integer-limits", "parity", "(calc.abs(-9223372036854775807), calc.abs(-9223372036854775807 + 1), calc.abs(9223372036854775807), calc.abs(-9007199254740993), calc.abs(-1), calc.abs(-0), calc.abs(1))"),
    ("integer-types", "parity", "(type(calc.abs(-9223372036854775807)), type(calc.abs(-9007199254740993)), type(calc.abs(-0)))"),
    ("integer-positive-routes", "parity", "{let ab=calc.abs.with(-19).with(); let aa=arguments(-19); (ab(), calc.abs(..aa))}"),
    ("integer-warning-success", "parity", "{import std; calc.abs(-9223372036854775807)}"),
    ("overflow-math-qualified-content", "preserved-control", "$std.calc.abs(" + M + ")$"),
    ("overflow-math-alias-content", "preserved-control", "{let ab=calc.abs; $ab(" + M + ")$}"),
    ("parser-direct-minimum-debt", "preserved-debt", "calc.abs(-9223372036854775808)"),
    ("overflow-named-guard-debt", "preserved-debt", "calc.abs(" + M + ", bad: 1)"),
    ("overflow-arity-guard-debt", "preserved-debt", "calc.abs(" + M + ", 0)"),
    ("overflow-named-before-arity-debt", "preserved-debt", "calc.abs(bad: 1, " + M + ", 0)"),
]
PROFILES = history.PROFILES
digest, command, observed, save, normhash = history.digest, history.command, history.observed, history.save, history.normhash

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--candidate")
    ap.add_argument("--order", choices=["normal", "reverse"], default="normal")
    ap.add_argument("--output", required=True)
    ap.add_argument("--freeze-from")
    args = ap.parse_args()
    assert normhash() == NORM
    assert digest(MANIFEST) == "fa90e9d05855842467eb7baac85d81a157af03bb285b9c1d40df0faf1f660acf"
    assert digest(BASE) == "9f347f742a5cdb5c4c36a4af985b4ff122ac1bb118a1e018b960e7f5d105c2ec"
    assert digest(VANILLA) == "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
    if args.freeze_from:
        assert not args.candidate
        baseline = json.loads((ROOT / args.freeze_from).read_text())
        historical = {(r["case"], r["profile"]): r for r in json.loads(HISTORY.read_text())["expectations"]}
        expectations = []
        for row in baseline["cases"]:
            key = (row["case"], row["profile"])
            classification = row["classification"]
            if key in historical and row["case"] != "overflow":
                prior = historical[key]
                oracle = prior["expected"]
                classification = prior["classification"]
                assert observed(row["results"]["BASE"]) == oracle, key
            elif classification == "preserved-debt":
                oracle = observed(row["results"]["BASE"])
            else:
                oracle = observed(row["results"]["VANILLA"])
                if classification != "overflow-correction":
                    assert observed(row["results"]["BASE"]) == oracle, key
            if row["case"] in TRACE_DEBT:
                # Author-time construction from the pre-C vanilla observation and
                # previously established dispatcher-name debt. Stored literals are
                # compared verbatim; no observed output is ever normalized.
                assert oracle["stderr"].count("while calling `abs`") == 1, key
                oracle["stderr"] = oracle["stderr"].replace("while calling `abs`", "while calling `calc.abs`")
                classification = "overflow-correction-with-preserved-dispatcher-name-debt"
            expectations.append({"case": row["case"], "profile": row["profile"], "classification": classification, "expected": oracle})
        save(args.output, {
            "frozen_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
            "manifest_sha256": digest(MANIFEST), "l0_norm_sha256": normhash(),
            "baseline_sha256": digest(ROOT / args.freeze_from), "historical_expectations_sha256": digest(HISTORY),
            "policy": "Exact complete exit/stdout/stderr literals. All 252 historical cells inherited except four overflow cells. Three new bound-origin routes preserve literal calc.abs dispatcher-name debt. Parser/guard debt remains BASE. No candidate normalization. Unknown blocks.",
            "expectations": expectations,
        })
        return
    binaries = {"BASE": BASE, "VANILLA": VANILLA}
    if args.candidate:
        binaries["CANDIDATE"] = args.candidate
    frozen = {(r["case"], r["profile"]): r for r in json.loads(FROZEN.read_text())["expectations"]} if args.candidate else {}
    receipt = {
        "utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "head": command(["git", "rev-parse", "HEAD"])["stdout"].strip(),
        "working_tree": "noncommitted; public input records initial exact diff/stat, current names/stat below; no runtime content inspected",
        "diff_stat": command(["git", "diff", "HEAD", "--stat"])["stdout"],
        "l0_norm_sha256": normhash(), "manifest_sha256": digest(MANIFEST),
        "public_baseline_sha256": digest(D / "p1330-baseline-public.json"),
        "runner_sha256": digest(__file__), "historical_runner_sha256": digest(HISTORY_RUNNER),
        "expected_sha256": digest(FROZEN) if args.candidate else None,
        "order": args.order, "binaries": {k: {"path": v, "sha256": digest(v)} for k, v in binaries.items()}, "cases": [],
    }
    cases = CASES if args.order == "normal" else list(reversed(CASES))
    for name, classification, expr in cases:
        for profile, flags in PROFILES:
            results = {k: command([v, "--color", "never", "eval", *flags, expr]) for k, v in binaries.items()}
            row = {"case": name, "classification": classification, "expression": expr, "profile": profile, "results": results,
                   "baseline_equals_vanilla": observed(results["BASE"]) == observed(results["VANILLA"])}
            if args.candidate:
                expectation = frozen[(name, profile)]
                row.update(classification=expectation["classification"], expected=expectation["expected"],
                           candidate_matches_frozen_policy=observed(results["CANDIDATE"]) == expectation["expected"])
            receipt["cases"].append(row)
    save(args.output, receipt)
    print(json.dumps({"observations": len(receipt["cases"]), "baseline_parity": sum(r["baseline_equals_vanilla"] for r in receipt["cases"]),
                      "candidate_failures": [r["case"] + "/" + r["profile"] for r in receipt["cases"] if r.get("candidate_matches_frozen_policy") is False]}))

if __name__ == "__main__":
    main()
