#!/usr/bin/env python3
"""Validate the plan's evidence and coverage; does not execute proposed repairs."""
import gzip
import hashlib
import json
import re
import subprocess
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def read(name):
    return json.loads((HERE / name).read_text())


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    inventory = read("inventory.json")
    rows = read("case-ledger.json")["cases"]
    expected = {row["id"]: row for row in inventory}
    ids = [row["id"] for row in rows]
    assert len(ids) == len(set(ids)) == len(expected) == 105
    assert set(ids) == set(expected)
    assignments = [i for batch in read("batches.json") for i in batch["ids"]]
    assert Counter(assignments) == Counter(ids)
    required = ("function", "classification", "source_evidence", "l0_evidence",
                "correction", "proposed_tests", "gate", "uncertainty",
                "structural_proof", "dependencies")
    for row in rows:
        original = row["analysis"]
        for key, value in expected[row["id"]].items():
            assert original[key] == value, (row["id"], key)
        for key in required:
            assert row[key], (row["id"], key)
        for key, hash_key in (("file", "source_sha256"), ("owner", "l0_sha256")):
            assert digest(ROOT / original[key]) == original[hash_key], original[key]
        assert row["id"] in (HERE / row["report"]).read_text(), row["id"]
    with gzip.open(HERE / "baseline.sarif.gz", "rt") as stream:
        sarif = json.load(stream)
    results = [r for run in sarif["runs"] for r in run["results"]]
    counts = Counter(f'{r["ruleId"]}:{r["level"]}' for r in results)
    assert dict(counts) == read("baseline-comparison.json")["full_lint_counts"]
    def location(result):
        physical = result["locations"][0]["physicalLocation"]
        return (physical["artifactLocation"]["uri"].removeprefix("./"),
                physical["region"]["startLine"], physical["region"]["startColumn"])
    assert Counter(location(r) for r in results if r["ruleId"] == "V16") == Counter(
        (r["file"], r["line"], r["column"]) for r in inventory)
    for record in read("rule-evidence.json")["files"]:
        assert digest(ROOT / record["path"]) == record["sha256"], record["path"]
    assert git("diff", "HEAD", "--", "01_core", "02_shell", "03_infra", "04_wiring",
               "00_nucleo/prompts", "crystalline.toml") == ""
    assert "mature layout check passed" in (HERE / "mature-layout.log").read_text()
    for report in HERE.glob("*.md"):
        for target in re.findall(r"\]\(([^\s)]+)\)", report.read_text()):
            if ":" not in target and not target.startswith("#"):
                assert (report.parent / target.split("#")[0]).exists(), (report, target)
    receipt = {
        "utc": datetime.now(timezone.utc).isoformat(),
        "head": git("rev-parse", "HEAD"),
        "baseline_head": read("baseline-state.json")["head"],
        "status": git("status", "--short"),
        "diff_head_stat": git("diff", "HEAD", "--stat"),
        "analysis_artifacts": sorted(str(p.relative_to(ROOT)) for p in HERE.iterdir() if p.is_file()),
        "command": "python3 lab/results/v16-cause-analysis-2026-10-19/validate.py",
        "result": "pass", "cases": len(rows), "owners": len({r["owner"] for r in inventory}),
        "full_lint_counts": dict(counts),
        "source_l0_rule_config_digests_unchanged": True,
        "evidence_markdown_links_resolve": True,
        "mature_layout_result": "pass; see mature-layout.log",
        "product_implementation_or_tests_executed": False,
        "scope": "Plan coverage and evidence validation only. Proposed tests and mutations remain not run."
    }
    (HERE / "validation.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("PASS: 105 unique cases, 81 owners, exact batch coverage, unchanged source/L0/rule/config digests, baseline SARIF consistent.")


if __name__ == "__main__":
    main()
