#!/usr/bin/env python3
"""P1332 independent CLI oracle: frozen complete observations, no candidate rewriting."""
import argparse
import copy
import datetime
import importlib.util
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
HISTORY_RUNNER = D / "p1331-ab-cli-r2.py"
HISTORY = D / "p1331-ab-cli-expected-r2.json"
spec = importlib.util.spec_from_file_location("p1331_history", HISTORY_RUNNER)
history = importlib.util.module_from_spec(spec)
spec.loader.exec_module(history)
BASE = "/tmp/p1331-target.rtY0la/release/typst"
VANILLA = "/usr/local/bin/typst"
MANIFEST = D / "p1332-manifest.json"
FROZEN = D / "p1332-ab-cli-expected.json"
NORM = "ac26a9a0492f250e4a59175256b8b1b7a4a173546dc483131a704f01262b84ad"
digest, command, observed, normhash = history.digest, history.command, history.observed, history.normhash
PROFILES = history.PROFILES
CASES = list(history.CASES)
CASES += [
    ("name-lookup", "preserved-parity", "calc.abs"),
    ("name-repr", "preserved-parity", "repr(calc.abs)"),
    ("name-alias-repr", "preserved-parity", "{let a=calc.abs; repr(a)}"),
    ("name-import-repr", "preserved-parity", "{import calc: abs; repr(abs)}"),
    ("name-bound-repr", "preserved-parity", "repr(calc.abs.with(-1))"),
    ("name-nested-repr", "preserved-parity", "repr(calc.abs.with().with(-1))"),
    ("name-identity", "preserved-parity", "{import calc: abs; let a=abs; (a == abs, abs == calc.abs, calc.abs == std.calc.abs, a == calc.sqrt, a == calc.pow, a == (x => x))}"),
    ("name-identity-with-alias", "preserved-parity", "{let a=calc.abs.with(-19); let b=a; (a == b, b())}"),
    ("name-nested-success", "preserved-parity", "{import calc: abs; let a=abs.with().with(-9007199254740993); a()}"),
    ("name-import-content", "trace-name", "{import calc: abs; let a=abs.with([x]); a()}"),
    ("name-import-mixed", "trace-name", "{import calc: abs; let a=abs.with(2pt + 3em).with(); a()}"),
    ("name-import-overflow", "trace-name", "{import calc: abs; let a=abs.with(-9223372036854775807 - 1); a()}"),
    ("name-import-fallback", "trace-name", "{import calc: abs; let a=abs.with(false).with(); a()}"),
    ("name-import-arguments", "trace-name", "{import calc: abs; let aa=arguments(\"x\"); abs(..aa)}"),
    ("name-warning-trace", "trace-name", "{import std; let a=calc.abs.with([x]); a()}"),
    ("name-utf8-lines-trace", "trace-name", "{let café=calc.abs.with([á]);\n café()}"),
    ("name-math-import", "preserved-debt-imported-math-resolution", "{import calc: abs; $abs(-1)$}"),
    ("name-empty-guard", "preserved-debt", "calc.abs()"),
    ("name-with-empty-guard", "preserved-debt", "{let a=calc.abs.with(); a()}"),
    ("name-with-named-guard", "preserved-debt", "{let a=calc.abs.with(false, bad: 1); a()}"),
    ("name-with-arity-guard", "preserved-debt", "{let a=calc.abs.with(false, 1); a()}"),
    ("name-other-sqrt", "preserved-debt", "{let a=calc.sqrt.with(\"x\"); a()}"),
    ("name-other-repr", "preserved-parity", "(repr(calc.sqrt), repr(calc.pow))"),
    ("name-gradient-linear", "transitive-name-debt", "gradient.linear(red,blue,space:calc.abs)"),
    ("name-gradient-radial", "transitive-name-debt", "gradient.radial(red,blue,space:calc.abs)"),
    ("name-gradient-conic", "transitive-name-debt", "gradient.conic(red,blue,space:calc.abs)"),
    ("name-show", "transitive-name-debt", "{show calc.abs: it => it; [x]}"),
    ("name-where", "preserved-debt", "calc.abs.where()"),
]

def save(path, value):
    target = pathlib.Path(path).resolve()
    assert target.name.startswith("p1332-ab-")
    assert not target.exists(), "immutable evidence: choose a new output"
    content = json.dumps(value, indent=2, ensure_ascii=False) + "\n"
    patch = "*** Begin Patch\n*** Add File: " + str(target) + "\n" + "".join("+" + line + "\n" for line in content.splitlines()) + "*** End Patch\n"
    subprocess.run(["apply_patch"], input=patch, text=True, check=True, capture_output=True)

def expected_name_changes(before, transitive=False):
    after = copy.deepcopy(before)
    # Author-time transformation only. Source lines and primary arity strings
    # cannot match this complete trace prefix. Candidate bytes remain untouched.
    after["stderr"] = after["stderr"].replace("  while calling `calc.abs` at ", "  while calling `abs` at ")
    if transitive:
        first, separator, remainder = after["stderr"].partition("\n")
        first = first.replace("Some(\"calc.abs\")", "Some(\"abs\")").replace("função 'calc.abs'", "função 'abs'")
        after["stderr"] = first + separator + remainder
    return after

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--candidate")
    ap.add_argument("--output", required=True)
    ap.add_argument("--order", choices=["normal", "reverse"], default="normal")
    ap.add_argument("--freeze-from")
    args = ap.parse_args()
    assert normhash() == NORM
    assert digest(MANIFEST) == "b32fa0ac39b79d7a1b7f23f62540011b58ac68e20f0c850009f688b68b412084"
    assert digest(BASE) == "a8d6e2f4472fefc9e63a123783feac852445191dcb82ac269d366a6419ae1a47"
    assert digest(VANILLA) == "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
    if args.freeze_from:
        assert not args.candidate
        baseline = json.loads(pathlib.Path(args.freeze_from).read_text())
        historical = {(r["case"], r["profile"]): r for r in json.loads(HISTORY.read_text())["expectations"]}
        expectations = []
        for row in baseline["cases"]:
            key = (row["case"], row["profile"])
            before = observed(row["results"]["BASE"])
            cls = next(c for n,c,e in CASES if n == row["case"])
            if key in historical:
                assert before == historical[key]["expected"], key
                cls = historical[key]["classification"]
            oracle = expected_name_changes(before, cls == "transitive-name-debt")
            if cls in {"preserved-parity", "trace-name"}:
                assert oracle == observed(row["results"]["VANILLA"]), key
            if oracle != before:
                cls += "; intrinsic-name-correction"
            expectations.append({"case": row["case"], "profile": row["profile"], "classification": cls, "expected": oracle})
        assert len(historical) == 504
        assert len(expectations) == len(CASES) * len(PROFILES)
        save(args.output, {"utc": datetime.datetime.now(datetime.timezone.utc).isoformat(), "manifest_sha256": digest(MANIFEST), "l0_norm_sha256": normhash(), "baseline_sha256": digest(args.freeze_from), "historical_expectations_sha256": digest(HISTORY), "policy": "Historical 504 cells unchanged except exact trace-name field. New gradient/show primary name interpolation only; all remaining diagnostics including where/guards remain measured debt. Exact full exit/stdout/stderr; no candidate normalization; mandatory Unknown blocks.", "expectations": expectations})
        return
    binaries = {"BASE": BASE, "VANILLA": VANILLA}
    if args.candidate:
        binaries["CANDIDATE"] = args.candidate
    frozen = {(r["case"],r["profile"]):r for r in json.loads(FROZEN.read_text())["expectations"]} if args.candidate else {}
    receipt = {"utc": datetime.datetime.now(datetime.timezone.utc).isoformat(), "head": command(["git", "rev-parse", "HEAD"])["stdout"].strip(), "diff_stat": command(["git", "diff", "HEAD", "--stat"])["stdout"], "working_tree": "uncommitted; no runtime content inspected", "manifest_sha256": digest(MANIFEST), "l0_norm_sha256": normhash(), "public_baseline_sha256": digest(D / "p1332-baseline-public.json"), "name_consumers_sha256": digest(D / "p1332-name-consumers-public.json"), "runner_sha256": digest(__file__), "history_runner_sha256": digest(HISTORY_RUNNER), "expected_sha256": digest(FROZEN) if args.candidate else None, "order":args.order, "binaries":{k:{"path":v,"sha256":digest(v)} for k,v in binaries.items()}, "cases":[]}
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
