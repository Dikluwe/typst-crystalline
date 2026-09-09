#!/usr/bin/env python3
"""P1331 independent literal CLI oracle. Never normalize candidate observations."""
import argparse
import datetime
import hashlib
import importlib.util
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
HISTORY_RUNNER = D / "p1330-ab-cli.py"
HISTORY = D / "p1330-ab-cli-expected.json"
spec = importlib.util.spec_from_file_location("p1329_oracle", HISTORY_RUNNER)
history = importlib.util.module_from_spec(spec)
spec.loader.exec_module(history)
BASE = "/tmp/p1330-target.f0lmDu/release/typst"
VANILLA = "/usr/local/bin/typst"
MANIFEST = D / "p1331-manifest-r2.json"
FROZEN = D / "p1331-ab-cli-expected-r2.json"
NORM = "ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a"
TRACE_DEBT = {"fallback-with-bound", "fallback-with-nested", "fallback-arguments-spread"}
CASES = [(n, "fallback-correction" if n in {"string", "symbol"} else c, e) for n, c, e in history.CASES]
CASES += [
  [
    "fallback-bool-true",
    "fallback-correction",
    "calc.abs(true)"
  ],
  [
    "fallback-bool-false",
    "fallback-correction",
    "calc.abs(false)"
  ],
  [
    "fallback-none",
    "fallback-correction",
    "calc.abs(none)"
  ],
  [
    "fallback-auto",
    "fallback-correction",
    "calc.abs(auto)"
  ],
  [
    "fallback-array",
    "fallback-correction",
    "calc.abs((1, 2))"
  ],
  [
    "fallback-dictionary",
    "fallback-correction",
    "calc.abs((a: 1))"
  ],
  [
    "fallback-module",
    "fallback-correction",
    "calc.abs(calc)"
  ],
  [
    "fallback-function",
    "fallback-correction",
    "calc.abs((x => x))"
  ],
  [
    "fallback-datetime",
    "fallback-correction",
    "calc.abs(datetime(year: 2026, month: 1, day: 2))"
  ],
  [
    "fallback-relative-zero",
    "fallback-correction",
    "calc.abs(0% + 0pt)"
  ],
  [
    "fallback-relative-nonzero",
    "fallback-correction",
    "calc.abs(50% + 2pt)"
  ],
  [
    "fallback-color",
    "fallback-correction",
    "calc.abs(red)"
  ],
  [
    "fallback-stroke",
    "fallback-correction",
    "calc.abs(1pt + red)"
  ],
  [
    "fallback-alignment",
    "fallback-correction",
    "calc.abs(left + top)"
  ],
  [
    "fallback-gradient",
    "fallback-correction",
    "calc.abs(gradient.linear(red, blue))"
  ],
  [
    "fallback-regex",
    "fallback-correction",
    "calc.abs(regex(\"a\"))"
  ],
  [
    "fallback-tiling",
    "fallback-correction",
    "calc.abs(tiling(size: (2pt, 2pt))[x])"
  ],
  [
    "fallback-bytes",
    "fallback-correction",
    "calc.abs(bytes((1,2)))"
  ],
  [
    "fallback-duration",
    "fallback-correction",
    "calc.abs(duration(seconds: 1))"
  ],
  [
    "fallback-version",
    "fallback-correction",
    "calc.abs(version(1,2))"
  ],
  [
    "fallback-selector",
    "fallback-correction",
    "calc.abs(selector(heading))"
  ],
  [
    "fallback-arguments-value",
    "fallback-correction",
    "calc.abs(arguments(1))"
  ],
  [
    "fallback-state",
    "fallback-correction",
    "calc.abs(state(\"p1331\", 1))"
  ],
  [
    "fallback-counter",
    "fallback-correction",
    "calc.abs(counter(heading))"
  ],
  [
    "fallback-label",
    "fallback-correction",
    "calc.abs(<x>)"
  ],
  [
    "fallback-direction",
    "fallback-correction",
    "calc.abs(ltr)"
  ],
  [
    "fallback-path",
    "fallback-correction",
    "calc.abs(path(\"p1331.typ\"))"
  ],
  [
    "fallback-type",
    "fallback-correction",
    "calc.abs(location)"
  ],
  [
    "fallback-numeric-string",
    "fallback-correction",
    "calc.abs(\"-1.25\")"
  ],
  [
    "fallback-alias",
    "fallback-correction",
    "{let ab=calc.abs; ab(false)}"
  ],
  [
    "fallback-with-empty",
    "fallback-correction",
    "{let ab=calc.abs.with(); ab(\"x\")}"
  ],
  [
    "fallback-with-bound",
    "fallback-correction",
    "{let ab=calc.abs.with(\"x\"); ab()}"
  ],
  [
    "fallback-with-nested",
    "fallback-correction",
    "{let ab=calc.abs.with(false).with(); ab()}"
  ],
  [
    "fallback-array-spread",
    "fallback-correction",
    "calc.abs(..(false,))"
  ],
  [
    "fallback-arguments-spread",
    "fallback-correction",
    "{let aa=arguments(\"x\"); calc.abs(..aa)}"
  ],
  [
    "fallback-distinct-origins",
    "fallback-correction",
    "{let x=\"x\"; let y=\"x\"; calc.abs(y)}"
  ],
  [
    "fallback-utf8-lines",
    "fallback-correction",
    "{let café=\"á\";\n calc.abs(\n café\n)}"
  ],
  [
    "fallback-warning",
    "fallback-correction",
    "{import std; calc.abs(false)}"
  ],
  [
    "fallback-math-content",
    "preserved-control",
    "$std.calc.abs(\"-1\")$"
  ],
  [
    "fallback-named-guard",
    "preserved-debt",
    "calc.abs(false, bad: 1)"
  ],
  [
    "fallback-arity-guard",
    "preserved-debt",
    "calc.abs(\"x\", false)"
  ],
  [
    "fallback-named-before-arity",
    "preserved-debt",
    "calc.abs(bad: 1, false, \"x\")"
  ],
  [
    "fallback-path-construction-debt",
    "preserved-debt",
    "calc.abs(path())"
  ]
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
    assert digest(MANIFEST) == "6383d89ef0fccf78290182c1180fccaba290c1a11f36f4fe44ba75df53de81c9"
    assert digest(BASE) == "6f1db621bc0b2a7fe4fc9d05925fb96636f33232b8527b83c040b793970fbda0"
    assert digest(VANILLA) == "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
    if args.freeze_from:
        assert not args.candidate
        baseline = json.loads((ROOT / args.freeze_from).read_text())
        historical = {(r["case"], r["profile"]): r for r in json.loads(HISTORY.read_text())["expectations"]}
        expectations = []
        for row in baseline["cases"]:
            key = (row["case"], row["profile"])
            classification = next(c for n, c, _ in CASES if n == row["case"])
            # Pre-C measured explicit strings in math remain strings. The raw
            # baseline preserves the initial author label; this correction is
            # classification only, not a transformation of an observation.
            if row["case"] == "fallback-math-content":
                classification = "fallback-correction"
            if key in historical and row["case"] not in {"string", "symbol"}:
                prior = historical[key]
                oracle = prior["expected"]
                classification = prior["classification"]
                assert observed(row["results"]["BASE"]) == oracle, key
            elif classification == "preserved-debt":
                oracle = observed(row["results"]["BASE"])
            else:
                oracle = observed(row["results"]["VANILLA"])
                if classification != "fallback-correction":
                    assert observed(row["results"]["BASE"]) == oracle, key
            if row["case"] in TRACE_DEBT:
                # Author-time construction from the pre-C vanilla observation and
                # previously established dispatcher-name debt. Stored literals are
                # compared verbatim; no observed output is ever normalized.
                assert oracle["stderr"].count("while calling `abs`") == 1, key
                oracle["stderr"] = oracle["stderr"].replace("while calling `abs`", "while calling `calc.abs`")
                classification = "fallback-correction-with-preserved-dispatcher-name-debt"
            expectations.append({"case": row["case"], "profile": row["profile"], "classification": classification, "expected": oracle})
        save(args.output, {
            "frozen_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
            "manifest_sha256": digest(MANIFEST), "l0_norm_sha256": normhash(),
            "baseline_sha256": digest(ROOT / args.freeze_from), "historical_expectations_sha256": digest(HISTORY),
            "policy": "Exact complete exit/stdout/stderr literals. All 332 historical cells inherited except eight string/symbol cells. Three bound-origin fallback routes preserve literal calc.abs trace debt. Constructor and guards remain BASE. No candidate normalization. Mandatory Unknown blocks.",
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
        "public_baseline_sha256": digest(D / "p1331-baseline-public.json"),
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
