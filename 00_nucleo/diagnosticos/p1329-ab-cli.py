#!/usr/bin/env python3
"""Independent P1329 oracle catalog. Output is exact, never normalized."""
import argparse
import datetime
import hashlib
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
BASE = "/tmp/p1328-target.T7Tg57/release/typst"
VANILLA = "/usr/local/bin/typst"
FROZEN = ROOT / "00_nucleo/diagnosticos/p1329-ab-cli-expected-r1.json"
TRACE_DEBT = {"mixed-with", "mixed-with-nested", "mixed-spread-args"}
DIMENSIONS = {"length", "angle", "ratio", "fraction"}
HISTORY = ROOT / "00_nucleo/diagnosticos/p1328-ab-cli-expected-r1.json"
CASES = [
    ("content-empty", "correction", "calc.abs([])"),
    ("content-markup", "correction", "calc.abs([*bold*])"),
    ("content-math-qualified", "correction", "$std.calc.abs(-1)$"),
    ("content-math-bare", "correction", "{let ab = calc.abs; $ab(-1)$}"),
    ("content-alias", "correction", "{let ab = calc.abs; ab([x])}"),
    ("content-with-empty", "correction", "{let ab = calc.abs.with(); ab([x])}"),
    ("content-with-bound", "correction", "{let ab = calc.abs.with([x]); ab()}"),
    ("content-with-nested", "correction", "{let ab = calc.abs.with([x]).with(); ab()}"),
    ("content-spread-array", "correction", "calc.abs(..([x],))"),
    ("content-spread-args", "correction", "{let aa = arguments([x]); calc.abs(..aa)}"),
    ("content-utf8-lines", "correction", "{let café = [á];\n calc.abs(\n café\n)}"),
    ("content-distinct-origins", "correction", "{let x = [x]; let y = [x]; calc.abs(y)}"),
    ("content-markup-route", "correction", "[#calc.abs([x])]"),
    ("warning-before-content", "correction", "{import std; calc.abs([x])}"),
    ("integer", "parity", "calc.abs(-19)"),
    ("float", "parity", "calc.abs(-1.25)"),
    ("decimal", "parity", "calc.abs(decimal(\"-1.25\"))"),
    ("named-before-arity", "preserved-debt", "calc.abs([] , [], unexpected: 1)"),
    ("multiple", "preserved-debt", "calc.abs([], [])"),
    ("zero", "preserved-debt", "calc.abs()"),
    ("string", "preserved-debt", "calc.abs(\"x\")"),
    ("symbol", "preserved-debt", "calc.abs(sym.alpha)"),
    ("length", "preserved-debt", "calc.abs(-2pt)"),
    ("angle", "preserved-debt", "calc.abs(-2deg)"),
    ("ratio", "preserved-debt", "calc.abs(-2%)"),
    ("fraction", "preserved-debt", "calc.abs(-2fr)"),
    ("overflow", "preserved-debt", "calc.abs(-9223372036854775807 - 1)"),
    ("sqrt", "preserved-debt", "calc.sqrt([x])"),
]
CASES = [(n, "correction" if n in DIMENSIONS else c, e) for n, c, e in CASES]
CASES += [
    ("length-em", "correction", "calc.abs(-2em)"),
    ("length-cm", "correction", "calc.abs(-2cm)"),
    ("length-mm", "correction", "calc.abs(-2mm)"),
    ("length-in", "correction", "calc.abs(-2in)"),
    ("angle-rad", "correction", "calc.abs(-2rad)"),
    ("angle-overturn", "correction", "calc.abs(-810deg)"),
    ("ratio-unbounded", "correction", "calc.abs(-250%)"),
    ("fraction-literal", "correction", "calc.abs(-2.5fr)"),
    ("zero-pt", "correction", "calc.abs(0pt)"),
    ("zero-em", "correction", "calc.abs(-0em)"),
    ("zero-angle", "correction", "calc.abs(-0deg)"),
    ("zero-ratio", "correction", "calc.abs(-0%)"),
    ("zero-fraction", "correction", "calc.abs(-0fr)"),
    ("positive-all", "correction", "(calc.abs(2pt), calc.abs(2em), calc.abs(810deg), calc.abs(250%), calc.abs(2.5fr))"),
    ("zero-abs-component", "correction", "calc.abs(0pt - 3em)"),
    ("zero-em-component", "correction", "calc.abs(-2pt + 0em)"),
    ("mixed-pp", "correction", "calc.abs(2pt + 3em)"),
    ("mixed-nn", "correction", "calc.abs(-2pt - 3em)"),
    ("mixed-pn", "correction", "calc.abs(2pt - 3em)"),
    ("mixed-np", "correction", "calc.abs(-2pt + 3em)"),
    ("mixed-alias", "correction", "{let ab=calc.abs; ab(2pt + 3em)}"),
    ("mixed-with", "correction", "{let ab=calc.abs.with(2pt + 3em); ab()}"),
    ("mixed-with-nested", "correction", "{let ab=calc.abs.with(2pt + 3em).with(); ab()}"),
    ("mixed-spread-args", "correction", "{let aa=arguments(2pt + 3em); calc.abs(..aa)}"),
    ("mixed-spread-array", "correction", "calc.abs(..(2pt + 3em,))"),
    ("mixed-utf8", "correction", "{let café = 2pt + 3em;\n calc.abs(\n café\n)}"),
    ("dimensional-alias", "correction", "{let ab=calc.abs; ab(-2em)}"),
    ("dimensional-with", "correction", "{let ab=calc.abs.with(-2rad).with(); ab()}"),
    ("dimensional-spread", "correction", "{let aa=arguments(-2.5fr); calc.abs(..aa)}"),
    ("warning-mixed", "correction", "{import std; calc.abs(2pt + 3em)}"),
    ("warning-success", "correction", "{import std; calc.abs(-2em)}"),
    ("math-dimension-content", "preserved-control", "$std.calc.abs(-2pt)$"),
    ("math-alias-content", "preserved-control", "{let ab=calc.abs; $ab(-2deg)$}"),
    ("prior-float-fraction", "preserved-debt", "calc.abs(-calc.inf * 1fr)"),
    ("prior-zero-division", "preserved-control", "calc.abs((0.0/0.0) * 1deg)"),
]
PROFILES = [("default", []), ("html", ["--features", "html"]),
            ("a11y", ["--features", "a11y-extras"]),
            ("html+a11y", ["--features", "html,a11y-extras"])]

def digest(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def command(argv):
    p = subprocess.run(argv, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return {"argv": argv, "exit": p.returncode,
            "stdout": p.stdout.decode("utf-8"), "stderr": p.stderr.decode("utf-8")}

def observed(result):
    return {k: result[k] for k in ("exit", "stdout", "stderr")}

def save(path, data):
    text = json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    target = ROOT / path
    if target.exists():
        raise SystemExit("Refusing to overwrite receipt: " + str(target))
    patch = "*** Begin Patch\n*** Add File: " + str(target) + "\n"
    patch += "".join("+" + line + "\n" for line in text.splitlines())
    patch += "*** End Patch\n"
    subprocess.run(["apply_patch"], input=patch.encode(), check=True, cwd=ROOT)

def normhash():
    lines = (ROOT / "00_nucleo/prompts/compiler/stdlib/calc.md").read_text().splitlines(keepends=True)
    return hashlib.sha256("".join(x for x in lines if not x.startswith("Hash do Código:")).encode()).hexdigest()

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--candidate")
    ap.add_argument("--order", choices=["normal", "reverse"], default="normal")
    ap.add_argument("--output", required=True)
    ap.add_argument("--freeze-from")
    args = ap.parse_args()
    if args.freeze_from:
        baseline = json.loads((ROOT / args.freeze_from).read_text())
        history = {(r["case"], r["profile"]): r for r in json.loads(HISTORY.read_text())["expectations"]}
        expectations = []
        for row in baseline["cases"]:
            classification = row["classification"]
            oracle = observed(row["results"]["BASE" if classification == "preserved-debt" else "VANILLA"])
            key = (row["case"], row["profile"])
            if key in history and row["case"] not in DIMENSIONS:
                oracle = history[key]["expected"]
                classification = history[key]["classification"]
                assert observed(row["results"]["BASE"]) == oracle, key
            if row["case"] in TRACE_DEBT:
                # Author-time literal construction; the runtime comparator does no substitutions.
                assert oracle["stderr"].count("while calling `abs`") == 1
                assert row["results"]["BASE"]["stderr"].count("while calling `calc.abs`") == 1
                oracle["stderr"] = oracle["stderr"].replace("while calling `abs`", "while calling `calc.abs`")
                classification = "correction-with-preserved-dispatcher-name-debt"
            expectations.append({"case": row["case"], "profile": row["profile"],
                                 "classification": classification, "expected": oracle})
        save(args.output, {"frozen_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
                           "baseline_sha256": digest(ROOT / args.freeze_from),
                           "l0_norm_sha256": normhash(),
                           "manifest_sha256": digest(ROOT / "00_nucleo/diagnosticos/p1329-manifest.json"),
                           "historical_expectations_sha256": digest(HISTORY),
                           "policy": "Exact complete exit/stdout/stderr literals; only three explicitly named routes retain the baseline external dispatcher name calc.abs. No candidate normalization.",
                           "expectations": expectations})
        return
    assert normhash() == "3756c7998c5a81cfdc54119b6fc152ca1f186f0f74df654e01877b73bdadbe0d"
    binaries = {"BASE": BASE, "VANILLA": VANILLA}
    if args.candidate:
        binaries["CANDIDATE"] = args.candidate
    frozen = {(r["case"], r["profile"]): r for r in json.loads(FROZEN.read_text())["expectations"]} if args.candidate else {}
    receipt = {"utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
               "head": command(["git", "rev-parse", "HEAD"])["stdout"].strip(),
               "state": "working tree noncommitted; A/B test authority cannot inspect product diffs",
               "l0_norm_sha256": normhash(),
               "manifest_sha256": digest(ROOT / "00_nucleo/diagnosticos/p1329-manifest.json"),
               "public_baseline_sha256": digest(ROOT / "00_nucleo/diagnosticos/p1329-baseline-public.json"),
               "runner_sha256": digest(__file__), "order": args.order,
               "binaries": {k: {"path": v, "sha256": digest(v)} for k, v in binaries.items()},
               "cases": []}
    cases = CASES if args.order == "normal" else list(reversed(CASES))
    for name, classification, expr in cases:
        for profile, flags in PROFILES:
            rows = {k: command([v, "--color", "never", "eval", *flags, expr])
                    for k, v in binaries.items()}
            row = {"case": name, "classification": classification, "expression": expr,
                   "profile": profile, "results": rows,
                   "baseline_equals_vanilla": observed(rows["BASE"]) == observed(rows["VANILLA"])}
            if args.candidate:
                expectation = frozen[(name, profile)]
                row["classification"] = expectation["classification"]
                row["expected"] = expectation["expected"]
                row["candidate_matches_frozen_policy"] = observed(rows["CANDIDATE"]) == expectation["expected"]
            receipt["cases"].append(row)
    save(args.output, receipt)
    print(json.dumps({"observations": len(receipt["cases"]),
                      "baseline_parity": sum(c["baseline_equals_vanilla"] for c in receipt["cases"]),
                      "candidate_failures": [c["case"] + "/" + c["profile"] for c in receipt["cases"]
                                             if c.get("candidate_matches_frozen_policy") is False]}))

if __name__ == "__main__":
    main()
