#!/usr/bin/env python3
"""P1303 final bilateral runner. Writes a detached, reproducible JSON measurement."""

import datetime
import hashlib
import json
import os
import pathlib
import re
import subprocess
import time

REPO = pathlib.Path("/repos/Antigravity/typst-crystalline")
OUT = REPO / "00_nucleo/diagnosticos/p1303-final-measurement.json"
VANILLA = pathlib.Path("/usr/local/bin/typst")
CRYSTAL = pathlib.Path("/dev/shm/p1303-p7-final.rRLQWN/release/typst")
EXPECTED_VANILLA_SHA = "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
EXPECTED_CRYSTAL_SHA = "a31da4d06bf6f51e1d690a2c186c39bb78fd46a7a06de607d25168474c533668"
FIELDS = ["data-cell", "header-cell", "table-summary"]
PROFILES = [
    ("default", []),
    ("html", ["html"]),
    ("a11y", ["a11y-extras"]),
    ("html+a11y", ["html", "a11y-extras"]),
]
EXPECTED_RANGES = {
    "data-cell": [14, 23],
    "header-cell": [14, 25],
    "table-summary": [14, 27],
}
EXPECTED_VALUES = {
    "data-cell": '"(function, \\"data-cell\\")"\n',
    "header-cell": '"(function, \\"header-cell\\")"\n',
    "table-summary": '"(function, \\"table-summary\\")"\n',
}
EXPECTED_HINTS = [
    "try enabling the `a11y-extras` feature",
    "see https://typst.app/help/compiler-features for more details",
]


def sha_bytes(data):
    return hashlib.sha256(data).hexdigest()


def sha_file(path):
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def now():
    return datetime.datetime.now().astimezone().isoformat(timespec="microseconds")


def diagnostic_facts(stderr, expression):
    messages = re.findall(r"(?m)^error: (.*)$", stderr)
    hints = re.findall(r"(?m)^  = hint: (.*)$", stderr)
    location = re.search(r"<input-expression>:1:(\d+)", stderr)
    caret_line = next((line for line in stderr.splitlines() if "^" in line), None)
    byte_range = None
    if location is not None and caret_line is not None:
        start = int(location.group(1)) - 1
        width = caret_line.count("^")
        byte_range = {
            "start": start,
            "end": start + width,
            "notation": f"{start}..{start + width}",
            "source_slice": expression[start:start + width],
            "derivation": "ASCII canary: one-based diagnostic column minus one plus caret count",
        }
    return {
        "primary_error_count": len(messages),
        "secondary_diagnostic_count": len(re.findall(r"(?m)^(warning|help|note): ", stderr)),
        "messages": messages,
        "hints": hints,
        "byte_range": byte_range,
    }


def execute(run_id, order, ordinal, side, binary, binary_sha, profile, features, field):
    expression = f"repr((type(pdf.{field}), repr(pdf.{field})))"
    argv = [str(binary), "--color", "never", "eval"]
    if features:
        argv.extend(["--features", ",".join(features)])
    argv.append(expression)
    started_at = now()
    started_ns = time.monotonic_ns()
    unknown_reason = None
    try:
        proc = subprocess.run(argv, cwd=REPO, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
        exit_code = proc.returncode
        stdout_raw = proc.stdout
        stderr_raw = proc.stderr
    except Exception as exc:
        exit_code = None
        stdout_raw = b""
        stderr_raw = f"{type(exc).__name__}: {exc}".encode("utf-8", errors="replace")
        unknown_reason = "execution exception"
    duration_ns = time.monotonic_ns() - started_ns
    try:
        stdout = stdout_raw.decode("utf-8")
        stdout_utf8 = True
    except UnicodeDecodeError:
        stdout = stdout_raw.decode("utf-8", errors="replace")
        stdout_utf8 = False
    try:
        stderr = stderr_raw.decode("utf-8")
        stderr_utf8 = True
    except UnicodeDecodeError:
        stderr = stderr_raw.decode("utf-8", errors="replace")
        stderr_utf8 = False
    facts = diagnostic_facts(stderr, expression)
    if not stdout_utf8 or not stderr_utf8:
        unknown_reason = unknown_reason or "non-UTF-8 output"
    if exit_code is not None and exit_code != 0 and facts["byte_range"] is None:
        unknown_reason = unknown_reason or "diagnostic range not derivable"
    return {
        "run_id": run_id,
        "order": order,
        "ordinal_within_order": ordinal,
        "side": side,
        "profile": profile,
        "features": features,
        "field": field,
        "expression": expression,
        "argv": argv,
        "binary_path": str(binary),
        "binary_sha256": binary_sha,
        "started_at": started_at,
        "completed_at": now(),
        "duration_ns": duration_ns,
        "exit_code": exit_code,
        "stdout": stdout,
        "stdout_sha256": sha_bytes(stdout_raw),
        "stderr": stderr,
        "stderr_sha256": sha_bytes(stderr_raw),
        "diagnostic": facts,
        "completion_state": "Unknown" if unknown_reason else "complete",
        "unknown_reason": unknown_reason,
    }


def classify(vanilla, crystal):
    if vanilla["completion_state"] != "complete" or crystal["completion_state"] != "complete":
        return "EXECUTION_UNKNOWN"
    if vanilla["exit_code"] == 0 and crystal["exit_code"] == 0:
        if vanilla["stdout_sha256"] == crystal["stdout_sha256"] and vanilla["stderr_sha256"] == crystal["stderr_sha256"]:
            return "MATCH_VALUE"
        return "DIFFERENT_VALUE"
    if vanilla["exit_code"] != 0 and crystal["exit_code"] != 0:
        if vanilla["stdout_sha256"] == crystal["stdout_sha256"] and vanilla["stderr_sha256"] == crystal["stderr_sha256"]:
            return "MATCH_DIAGNOSTIC"
        return "DIFFERENT_DIAGNOSTIC"
    return "EXECUTION_UNKNOWN"


head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=REPO, text=True).strip()
status_before = subprocess.check_output(
    ["git", "status", "--short", "--untracked-files=all"], cwd=REPO, text=True
)
vanilla_sha = sha_file(VANILLA)
crystal_sha_before = sha_file(CRYSTAL)
cases = [(profile, features, field) for profile, features in PROFILES for field in FIELDS]
runs = []
comparisons = []
run_counter = 0
for order, case_sequence, side_sequence in [
    ("normal", cases, [("vanilla", VANILLA, vanilla_sha), ("crystalline", CRYSTAL, crystal_sha_before)]),
    ("inverted", list(reversed(cases)), [("crystalline", CRYSTAL, crystal_sha_before), ("vanilla", VANILLA, vanilla_sha)]),
]:
    ordinal = 0
    for profile, features, field in case_sequence:
        pair = {}
        for side, binary, digest in side_sequence:
            run_counter += 1
            ordinal += 1
            run = execute(f"run-{run_counter:02d}", order, ordinal, side, binary, digest, profile, features, field)
            runs.append(run)
            pair[side] = run
        classification = classify(pair["vanilla"], pair["crystalline"])
        comparisons.append({
            "comparison_id": f"cmp-{len(comparisons) + 1:02d}",
            "order": order,
            "profile": profile,
            "features": features,
            "field": field,
            "vanilla_run_id": pair["vanilla"]["run_id"],
            "crystalline_run_id": pair["crystalline"]["run_id"],
            "classification": classification,
            "complete": classification != "EXECUTION_UNKNOWN",
        })

crystal_sha_after = sha_file(CRYSTAL)


def find_run(order, profile, field, side):
    return next(
        run for run in runs
        if run["order"] == order and run["profile"] == profile
        and run["field"] == field and run["side"] == side
    )


summary = {}
for order in ["normal", "inverted"]:
    summary[order] = {}
    for profile, _features in PROFILES:
        classes = [
            item["classification"] for item in comparisons
            if item["order"] == order and item["profile"] == profile
        ]
        summary[order][profile] = {key: classes.count(key) for key in sorted(set(classes))}

repeat_mismatches = []
for profile, _features in PROFILES:
    for field in FIELDS:
        for side in ["vanilla", "crystalline"]:
            first = find_run("normal", profile, field, side)
            second = find_run("inverted", profile, field, side)
            keys = ["exit_code", "stdout_sha256", "stderr_sha256"]
            if any(first[key] != second[key] for key in keys) or first["diagnostic"] != second["diagnostic"]:
                repeat_mismatches.append({"profile": profile, "field": field, "side": side})

contract_checks = []
for order in ["normal", "inverted"]:
    for profile, _features in PROFILES:
        for field in FIELDS:
            vanilla = find_run(order, profile, field, "vanilla")
            crystal = find_run(order, profile, field, "crystalline")
            comparison = next(
                item for item in comparisons
                if item["order"] == order and item["profile"] == profile and item["field"] == field
            )
            if profile in ["default", "html"]:
                expected_message = f"cannot access field `{field}` because the `a11y-extras` feature is not enabled"
                expected_range = EXPECTED_RANGES[field]
                passed = (
                    comparison["classification"] == "MATCH_DIAGNOSTIC"
                    and vanilla["exit_code"] == 1 and crystal["exit_code"] == 1
                    and vanilla["stdout"] == "" and crystal["stdout"] == ""
                    and vanilla["diagnostic"]["primary_error_count"] == 1
                    and crystal["diagnostic"]["primary_error_count"] == 1
                    and vanilla["diagnostic"]["secondary_diagnostic_count"] == 0
                    and crystal["diagnostic"]["secondary_diagnostic_count"] == 0
                    and vanilla["diagnostic"]["messages"] == [expected_message]
                    and crystal["diagnostic"]["messages"] == [expected_message]
                    and vanilla["diagnostic"]["hints"] == EXPECTED_HINTS
                    and crystal["diagnostic"]["hints"] == EXPECTED_HINTS
                    and [vanilla["diagnostic"]["byte_range"]["start"], vanilla["diagnostic"]["byte_range"]["end"]] == expected_range
                    and [crystal["diagnostic"]["byte_range"]["start"], crystal["diagnostic"]["byte_range"]["end"]] == expected_range
                )
            else:
                passed = (
                    comparison["classification"] == "MATCH_VALUE"
                    and vanilla["exit_code"] == 0 and crystal["exit_code"] == 0
                    and vanilla["stderr"] == "" and crystal["stderr"] == ""
                    and vanilla["stdout"] == EXPECTED_VALUES[field]
                    and crystal["stdout"] == EXPECTED_VALUES[field]
                )
            contract_checks.append({"order": order, "profile": profile, "field": field, "passed": passed})

expected_vector = {
    "default": {"MATCH_DIAGNOSTIC": 3},
    "html": {"MATCH_DIAGNOSTIC": 3},
    "a11y": {"MATCH_VALUE": 3},
    "html+a11y": {"MATCH_VALUE": 3},
}
unknown_count = sum(run["completion_state"] == "Unknown" for run in runs)
confirmed = (
    vanilla_sha == EXPECTED_VANILLA_SHA
    and crystal_sha_before == EXPECTED_CRYSTAL_SHA
    and crystal_sha_after == EXPECTED_CRYSTAL_SHA
    and oct(os.stat(CRYSTAL).st_mode & 0o777) == "0o555"
    and summary["normal"] == expected_vector
    and summary["inverted"] == expected_vector
    and not repeat_mismatches
    and unknown_count == 0
    and all(item["passed"] for item in contract_checks)
)

document = {
    "schema": "p1303-final-measurement-v1",
    "generated_at": now(),
    "step": "P1303",
    "role": "EXECUTOR_ADVERSARIAL_E_VERIFICACAO_P7",
    "regime": "executed_without_technical_isolation_attestation",
    "verdict": "P1303_FINAL_BILATERAL_MATRIX_PASS" if confirmed else "P1303_BLOCKED_VERIFICATION",
    "source_state": {"repository": str(REPO), "head": head, "git_status_before": status_before},
    "binaries": {
        "vanilla": {"path": str(VANILLA), "sha256": vanilla_sha, "expected_sha256": EXPECTED_VANILLA_SHA},
        "crystalline": {
            "path": str(CRYSTAL),
            "sha256_before": crystal_sha_before,
            "sha256_after": crystal_sha_after,
            "expected_sha256": EXPECTED_CRYSTAL_SHA,
            "mode": oct(os.stat(CRYSTAL).st_mode & 0o777),
            "size_bytes": os.stat(CRYSTAL).st_size,
            "version_stdout": subprocess.check_output([str(CRYSTAL), "--version"], text=True),
        },
    },
    "order_protocol": {
        "normal_cases": [{"profile": p, "features": f, "field": field} for p, f, field in cases],
        "inverted_cases": [{"profile": p, "features": f, "field": field} for p, f, field in reversed(cases)],
        "normal_binary_precedence": ["vanilla", "crystalline"],
        "inverted_binary_precedence": ["crystalline", "vanilla"],
    },
    "runs": runs,
    "comparisons": comparisons,
    "checks": {
        "expected_vector_each_order": expected_vector,
        "observed_vector": summary,
        "different_diagnostic_count": sum(item["classification"] == "DIFFERENT_DIAGNOSTIC" for item in comparisons),
        "execution_unknown_count": sum(item["classification"] == "EXECUTION_UNKNOWN" for item in comparisons),
        "unknown_run_count": unknown_count,
        "repeat_mismatches": repeat_mismatches,
        "contract_checks": contract_checks,
        "all_contract_checks_passed": all(item["passed"] for item in contract_checks),
        "measurement_confirmed": confirmed,
    },
    "claim_limit": "Only the three feature-gated pdf fields, four profiles, two integral orders and C-P1303-v1 observables; no general equivalence claim.",
}
OUT.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(json.dumps({
    "path": str(OUT),
    "sha256": sha_file(OUT),
    "verdict": document["verdict"],
    "run_count": len(runs),
    "comparison_count": len(comparisons),
    "observed_vector": summary,
    "different_diagnostic_count": document["checks"]["different_diagnostic_count"],
    "execution_unknown_count": document["checks"]["execution_unknown_count"],
    "unknown_run_count": unknown_count,
    "repeat_mismatches": repeat_mismatches,
    "failed_contract_checks": [item for item in contract_checks if not item["passed"]],
}, indent=2))
