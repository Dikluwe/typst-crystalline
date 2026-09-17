#!/usr/bin/env python3
"""Executa as quatro suítes Rust e V01-V05 em ordens A/B/C."""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time


GROUPS = [
    "compiler::stdlib::structural::p1430_tests",
    "compiler::eval::p1430_tests",
    "entities::elements::p1430_math_cases_tests",
    "compiler::math::layout::p1430_tests",
    "certification-policy:V01-V05",
]


def execute(group: str):
    if group == "certification-policy:V01-V05":
        command = [
            "python3",
            "00_nucleo/diagnosticos/p1430-certification-policy-oracle.py",
            "--policy",
            "00_nucleo/diagnosticos/p1430-certification-policy.py",
        ]
    else:
        command = ["cargo", "test", "-p", "typst-core", group, "--", "--nocapture", "--test-threads=1"]
    env = os.environ.copy()
    env["RUSTFLAGS"] = "-Awarnings"
    started = time.monotonic()
    proc = subprocess.run(command, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, env=env)
    output = proc.stdout
    if group == "certification-policy:V01-V05":
        try:
            policy = json.loads(output)
            tests = [(name, "ok") for name in sorted(policy.get("vectors", {}))] if policy.get("status") == "PASS" else [("V01-V05", "FAILED")]
        except json.JSONDecodeError:
            tests = [("V01-V05", "FAILED")]
    else:
        tests = re.findall(r"^test ([^ ]+) \.\.\. (ok|FAILED)$", output, re.MULTILINE)
    return {
        "group": group,
        "command": " ".join(command),
        "exit_code": proc.returncode,
        "seconds": round(time.monotonic() - started, 3),
        "tests": [{"name": name, "status": status} for name, status in tests],
        "output_sha256": hashlib.sha256(output.encode()).hexdigest(),
        "output_tail": output[-8000:],
    }


orders = [
    ("A-forward", GROUPS),
    ("B-forward-repeat", GROUPS),
    ("C-reverse", list(reversed(GROUPS))),
]
results = []
for name, groups in orders:
    runs = [execute(group) for group in groups]
    vector = sorted(
        (test["name"], test["status"])
        for run in runs
        for test in run["tests"]
    )
    results.append({
        "order": name,
        "groups": groups,
        "runs": runs,
        "vector": vector,
        "passed": sum(status == "ok" for _, status in vector),
        "failed": sum(status != "ok" for _, status in vector),
    })

same = all(result["vector"] == results[0]["vector"] for result in results[1:])
receipt = {
    "schema": "tekt.materialization.order-results/v2",
    "revision": "R2-with-certification-policy-V01-V05",
    "step": 1430,
    "candidate_commit": "f7dcbe86ca7618bf1e6d840e51ea168f7b1c3c8e",
    "attestation": "executado sem atestacao de isolamento",
    "orders": results,
    "identical_vectors": same,
    "expected_total": 28,
    "pass": same and all(r["passed"] == 28 and r["failed"] == 0 for r in results),
    "final_verdict": "NOT_EMITTED_BY_ADVERSARY",
}
Path("00_nucleo/diagnosticos/p1430-order-results-r2.json").write_text(
    json.dumps(receipt, indent=2, ensure_ascii=False) + "\n"
)
print(json.dumps({"pass": receipt["pass"], "vectors_equal": same, "counts": [(r["passed"], r["failed"]) for r in results]}))
