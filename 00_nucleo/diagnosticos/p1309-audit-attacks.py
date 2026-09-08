#!/usr/bin/env python3
"""P1309 D: JSON-patch attacks against an independently authored audit(bundle).

This program does not classify audit artifacts. It only changes copies, invokes
the external auditor, and records whether a valid positive control is accepted
and a targeted negative is rejected. It emits JSON to stdout and writes no files.
"""

import argparse
import copy
import datetime
import hashlib
import importlib.util
import json
import pathlib
import subprocess
import sys
import time

sys.dont_write_bytecode = True
ROOT = pathlib.Path(__file__).resolve().parents[2]


def sha_bytes(data):
    return hashlib.sha256(data).hexdigest()


def canonical(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False,
                      separators=(",", ":"), allow_nan=False).encode()


def tokens(pointer):
    if not pointer.startswith("/"):
        raise ValueError("patch pointer must be an absolute non-root JSON pointer")
    return [x.replace("~1", "/").replace("~0", "~") for x in pointer[1:].split("/")]


def apply_operations(bundle, operations):
    result = copy.deepcopy(bundle)
    changes = []
    for operation in operations:
        route = tokens(operation["path"])
        parent = result
        for token in route[:-1]:
            parent = parent[int(token)] if isinstance(parent, list) else parent[token]
        key = int(route[-1]) if isinstance(parent, list) and route[-1] != "-" else route[-1]
        op = operation["op"]
        if op == "test":
            if parent[key] != operation["value"]:
                raise ValueError("stale witness: " + operation["path"])
            continue
        before = copy.deepcopy(parent[key]) if key != "-" and (
            isinstance(parent, list) and 0 <= key < len(parent)
            or isinstance(parent, dict) and key in parent) else None
        if op == "remove":
            del parent[key]
        elif op == "replace":
            if key == "-" or isinstance(parent, dict) and key not in parent:
                raise ValueError("replace requires an existing target")
            parent[key] = copy.deepcopy(operation["value"])
        elif op == "add":
            value = copy.deepcopy(operation["value"])
            if isinstance(parent, list):
                parent.insert(len(parent) if key == "-" else key, value)
            else:
                parent[key] = value
        else:
            raise ValueError("unsupported operation: " + op)
        changes.append({"operation": operation, "before": before})
    return result, changes


def evaluate(auditor):
    spec = importlib.util.spec_from_file_location("p1309_external_auditor", auditor)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    bundle = json.load(sys.stdin)
    failures = module.audit(bundle)
    if not isinstance(failures, list):
        raise TypeError("external audit(bundle) must return a list of failures")
    if any(not isinstance(f, dict) or not isinstance(f.get("code"), str) for f in failures):
        raise TypeError("each auditor failure must have a string code")
    print(json.dumps({"failures": failures}, ensure_ascii=False, allow_nan=False))


def call_auditor(auditor, bundle, timeout):
    argv = [sys.executable, "-B", str(pathlib.Path(__file__).resolve()),
            "--evaluate", str(auditor)]
    begin = time.monotonic()
    receipt = {"argv": argv, "cwd": str(ROOT),
               "bundle_canonical_sha256": sha_bytes(canonical(bundle))}
    try:
        run = subprocess.run(argv, input=canonical(bundle), capture_output=True,
                             cwd=ROOT, timeout=timeout)
        receipt.update(exit=run.returncode, stdout=run.stdout.decode(errors="replace"),
                       stderr=run.stderr.decode(errors="replace"))
        if run.returncode != 0:
            receipt["execution_unknown"] = "AUDITOR_PROCESS_FAILED"
        else:
            result = json.loads(run.stdout)
            receipt["failures"] = result["failures"]
    except subprocess.TimeoutExpired:
        receipt["execution_unknown"] = "AUDITOR_TIMEOUT"
    except (OSError, ValueError, KeyError) as error:
        receipt["execution_unknown"] = type(error).__name__ + ": " + str(error)
    receipt["seconds"] = time.monotonic() - begin
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--evaluate", type=pathlib.Path)
    parser.add_argument("--auditor", type=pathlib.Path)
    parser.add_argument("--bundle", type=pathlib.Path)
    parser.add_argument("--cases", type=pathlib.Path)
    parser.add_argument("--timeout-seconds", type=float, default=30)
    args = parser.parse_args()
    if args.evaluate:
        evaluate(args.evaluate.resolve(strict=True))
        return
    if not all((args.auditor, args.bundle, args.cases)):
        parser.error("--auditor, --bundle and --cases are required")
    begin = time.monotonic()
    external_anchors = args.auditor.resolve().parent / "p1309-audit-anchors.json"
    inputs = [args.auditor, args.bundle, args.cases, external_anchors,
              pathlib.Path(__file__).resolve()]
    frozen = {str(p): sha_bytes(p.read_bytes()) for p in inputs}
    bundle = json.loads(args.bundle.read_text())
    cases = json.loads(args.cases.read_text())["cases"]
    base = call_auditor(args.auditor.resolve(), bundle, args.timeout_seconds)
    records = []
    for case in cases:
        record = {"id": case["id"], "description": case["description"],
                  "target_codes": case["target_codes"], "witness": case["witness"]}
        try:
            control_operations = case.get("control_operations", [])
            if any(op["path"] != "/scope" or op["op"] not in ("add", "replace", "test")
                   for op in control_operations):
                raise ValueError("positive controls may select a focal scope only; no repaired evidence")
            control, positive_changes = apply_operations(bundle, control_operations)
            if control.get("scope", "all") not in (
                    "all", "catalog", "matrix", "ledger", "sentinels", "selection", "integrity"):
                raise ValueError("unsupported focal scope")
            positive = (base if not positive_changes else
                        call_auditor(args.auditor.resolve(), control, args.timeout_seconds))
            negative, changes = apply_operations(control, case["operations"])
            if negative.get("anchors") != control.get("anchors"):
                raise ValueError("attacks must not alter authenticated anchors")
            if negative.get("scope", "all") != control.get("scope", "all"):
                raise ValueError("positive and negative must use the identical audit subsystem")
            record.update(control_operations=positive_changes, mutation_operations=changes,
                          positive=positive)
            if canonical(control) == canonical(negative):
                record["status"] = "INVALID_NO_CHANGE"
            elif positive.get("execution_unknown") or positive.get("failures") != []:
                record["status"] = "BLOCKED_CONTROL_NOT_ACCEPTED"
            else:
                attacked = call_auditor(args.auditor.resolve(), negative, args.timeout_seconds)
                record["negative"] = attacked
                codes = {f["code"] for f in attacked.get("failures", [])}
                if attacked.get("execution_unknown"):
                    record["status"] = "UNKNOWN_AUDITOR_EXECUTION"
                elif codes.intersection(case["target_codes"]):
                    record["status"] = "REJECTED_WITH_WITNESS"
                elif codes:
                    record["status"] = "BLOCKED_UNRELATED_REJECTION"
                else:
                    record["status"] = "SURVIVED"
        except (KeyError, IndexError, TypeError, ValueError) as error:
            record["status"] = "INVALID_OR_STALE_WITNESS"
            record["error"] = type(error).__name__ + ": " + str(error)
        records.append(record)
    unchanged = all(sha_bytes(p.read_bytes()) == frozen[str(p)] for p in inputs)
    killed = sum(r["status"] == "REJECTED_WITH_WITNESS" for r in records)
    survived = sum(r["status"] == "SURVIVED" for r in records)
    expected_ids = {"A%02d" % i for i in range(1, 19)}
    complete = (len(records) == 18 and {r["id"] for r in records} == expected_ids
                and killed + survived == 18 and unchanged)
    receipt = {"schema": "p1309-adversarial-replay-v1", "role": "D",
               "regime": "executado sem atestação de isolamento técnico",
               "at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
               "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT).decode().strip(),
               "diff_head_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=ROOT).decode(),
               "input_sha256": frozen, "inputs_unchanged": unchanged,
               "base_control": base, "attacks": records, "rejected": killed,
               "survived": survived, "required": 18,
               "mutation_score": killed / 18 if complete else None,
               "discriminatory_gate": "PASS" if complete and killed == 18 else "BLOCKED",
               "seconds": time.monotonic() - begin}
    print(json.dumps(receipt, ensure_ascii=False, sort_keys=True, allow_nan=False))


if __name__ == "__main__":
    main()
