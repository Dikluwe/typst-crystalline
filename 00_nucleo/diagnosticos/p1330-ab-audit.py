#!/usr/bin/env python3
"""Read-only independent audit of public P1330 CLI receipts and frozen inputs."""
import hashlib
import importlib.util
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
PINS = {
    "p1330-manifest-r2.json": "fa90e9d05855842467eb7baac85d81a157af03bb285b9c1d40df0faf1f660acf",
    "p1330-baseline-public.json": "c2f6500ed2f98fe2e37875b476fdb8570483ad516662cb804c4960c18a0781c2",
    "p1329-ab-tests-r1.rs": "567f8d4eda2fa5313af8449323248d814621cb47e5122a4ab19842dbf93fa838",
    "p1329-ab-p1328-successor.rs": "1fb3d1c0b6ddfd486aa1cfd4cbdaffbdf2ecd774021b5512d217b769e92c3a7c",
    "p1329-ab-cli-r2.py": "ee0d6a5da2283d50b0591a24db58f93a8300091729b6d6f2ad48e3457b393ab2",
    "p1329-ab-cli-expected-r2.json": "9501ae018ada17565bfc54227c244bea9af99e3988c9a212b262e92a0ee7c239",
    "p1330-ab-tests.rs": "6ed8f3af58855a88c9ea15113b38c2dad6029639448b052671b4effed4f5ca59",
    "p1330-ab-p1328-successor.rs": "56edc0c1b8c92d4e25bcfb73d1c3972526ea968f0647ea9462b659c2f517b47f",
    "p1330-ab-cli.py": "d405464d3d0961f5c23d0d34648e8361281cfdf5399499eb11e30e35951e241c",
    "p1330-ab-cli-baseline.json": "8ac2d68d4be81b5e3b9ab6a46e1f8c7aea6469cfdcb421d69c328d5b52229b86",
    "p1330-ab-cli-expected.json": "7eefa5cf55e10b77631cf5ef8d68be2629892fa64309c1274f20d1305112695d",
    "p1330-ab-freeze.md": "637d1bf3de8480502a9c6b3be793101d5495dc9e4bbd932e6eb5d2e772be8edf",
}
BINS = {
    "BASE": {"path": "/tmp/p1329-target.bg3p5A/release/typst", "sha256": "9f347f742a5cdb5c4c36a4af985b4ff122ac1bb118a1e018b960e7f5d105c2ec"},
    "VANILLA": {"path": "/usr/local/bin/typst", "sha256": "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"},
    "CANDIDATE": {"path": "/tmp/p1330-target.f0lmDu/release/typst", "sha256": "6f1db621bc0b2a7fe4fc9d05925fb96636f33232b8527b83c040b793970fbda0"},
}

def digest(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def rows_by_key(rows):
    result = {(r["case"], r["profile"]): r for r in rows}
    assert len(rows) == len(result), "duplicate observations"
    return result

def obs(row):
    return {k: row[k] for k in ("exit", "stdout", "stderr")}

for name, pin in PINS.items():
    assert digest(D / name) == pin, name
for role, binary in BINS.items():
    assert digest(binary["path"]) == binary["sha256"], role
norm = hashlib.sha256("".join(line for line in (ROOT / "00_nucleo/prompts/compiler/stdlib/calc.md").read_text().splitlines(keepends=True) if not line.startswith("Hash do Código:")).encode()).hexdigest()
assert norm == "9a5d734e086297d49d80d567bdb5527919049de19b7aea11f43b4467a972d2db"
spec = importlib.util.spec_from_file_location("p1330_frozen_runner", D / "p1330-ab-cli.py")
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
expected_doc = json.loads((D / "p1330-ab-cli-expected.json").read_text())
expected = rows_by_key(expected_doc["expectations"])
baseline = rows_by_key(json.loads((D / "p1330-ab-cli-baseline.json").read_text())["cases"])
historical = rows_by_key(json.loads((D / "p1329-ab-cli-expected-r2.json").read_text())["expectations"])
assert len(expected) == len(baseline) == 332
assert expected_doc["manifest_sha256"] == PINS["p1330-manifest-r2.json"]
assert expected_doc["l0_norm_sha256"] == norm
assert expected_doc["baseline_sha256"] == PINS["p1330-ab-cli-baseline.json"]
assert expected_doc["historical_expectations_sha256"] == PINS["p1329-ab-cli-expected-r2.json"]
assert len(historical) == 252
for key, row in historical.items():
    if key[0] != "overflow":
        assert expected[key] == row, key
    else:
        assert expected[key]["expected"] == obs(baseline[key]["results"]["VANILLA"]), key

normal = None
summaries = []
for name, order in [("normal", "normal"), ("repeat", "normal"), ("reverse", "reverse")]:
    path = D / ("p1330-ab-cli-" + name + ".json")
    receipt = json.loads(path.read_text())
    assert receipt["head"] == "d31047d7b8af7837c84adae4ded3d2ff50c62093"
    assert receipt["order"] == order
    assert receipt["manifest_sha256"] == PINS["p1330-manifest-r2.json"]
    assert receipt["l0_norm_sha256"] == norm
    assert receipt["public_baseline_sha256"] == PINS["p1330-baseline-public.json"]
    assert receipt["runner_sha256"] == PINS["p1330-ab-cli.py"]
    assert receipt["historical_runner_sha256"] == PINS["p1329-ab-cli-r2.py"]
    assert receipt["expected_sha256"] == PINS["p1330-ab-cli-expected.json"]
    assert receipt["binaries"] == BINS
    assert receipt["diff_stat"] and receipt["utc"] > expected_doc["frozen_utc"]
    cases = runner.CASES if order == "normal" else list(reversed(runner.CASES))
    desired = [(n, p, expr, flags) for n, _, expr in cases for p, flags in runner.PROFILES]
    actual = rows_by_key(receipt["cases"])
    assert len(receipt["cases"]) == 332 and set(actual) == set(expected)
    for row, (case, profile, expr, flags) in zip(receipt["cases"], desired):
        key = (case, profile)
        assert (row["case"], row["profile"], row["expression"]) == (case, profile, expr), key
        assert row["expected"] == expected[key]["expected"], key
        assert row["classification"] == expected[key]["classification"], key
        assert obs(row["results"]["CANDIDATE"]) == expected[key]["expected"], key
        assert row["candidate_matches_frozen_policy"] is True, key
        for role in BINS:
            assert row["results"][role]["argv"] == [BINS[role]["path"], "--color", "never", "eval", *flags, expr], (key, role)
            if role != "CANDIDATE":
                assert obs(row["results"][role]) == obs(baseline[key]["results"][role]), (key, role)
        assert row["baseline_equals_vanilla"] == (obs(row["results"]["BASE"]) == obs(row["results"]["VANILLA"])), key
        if normal is not None:
            assert row == normal[key], key
    if normal is None:
        normal = actual
    summaries.append({"receipt": path.name, "sha256": digest(path), "utc": receipt["utc"], "order": order, "cells": len(actual), "literal_mismatches": 0})
print(json.dumps({"verdict": "PASS", "protected_file_pins": len(PINS), "binary_pins": len(BINS), "cells_checked": 996, "unknown": 0, "receipts": summaries}, indent=2))
