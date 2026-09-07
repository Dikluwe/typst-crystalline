#!/usr/bin/env python3
"""Independent R4 gates. Does not author or repair any judged input.

prepare is read-only. discriminate tests frozen public observation envelopes;
it does not execute or certify product mutants. seal requires separate complete
inputs and leaves the actual-source mutation obligation explicitly pending.
"""
import argparse
import base64
import copy
import datetime
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"
BASELINE_SHA = "52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57"
OUTPUTS = {"p1307-r4-discriminator.json", "p1307-r4-seal.json",
           "p1307-r4-verification.json", "p1307-r4-mutant-ledger.json",
           "p1307-r4-certificate.json"}


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def resolve(value):
    path = Path(value)
    if path.is_absolute():
        return path
    return ROOT / path if "/" in value or (ROOT / path).exists() else D / path


def sha(path):
    return hashlib.sha256(resolve(str(path)).read_bytes()).hexdigest()


def pin(path):
    path = resolve(str(path))
    return {"path": str(path), "sha256": sha(path), "bytes": path.stat().st_size}


def read(path):
    return json.loads(resolve(str(path)).read_text())


def save(name, payload):
    if name not in OUTPUTS:
        raise ValueError("Verifier output outside declared allowlist")
    with (D / name).open("x") as stream:
        json.dump(payload, stream, indent=2, ensure_ascii=False)
        stream.write("\n")


def source_state():
    return {key: subprocess.check_output(["git", *args], cwd=ROOT, text=True)
            for key, args in {"head": ["rev-parse", "HEAD"],
                              "status": ["status", "--short"],
                              "diff_stat": ["diff", "HEAD", "--stat"],
                              "staged": ["diff", "--cached", "--binary"]}.items()}


def check_pin(item):
    try:
        return (isinstance(item, dict) and isinstance(item.get("path"), str)
                and re.fullmatch(r"[a-f0-9]{64}", item.get("sha256", "")) is not None
                and sha(item["path"]) == item["sha256"])
    except (OSError, TypeError, ValueError):
        return False


def walk_pins(value, prefix=""):
    if isinstance(value, dict):
        if "path" in value and "sha256" in value:
            yield prefix, value
        else:
            for key, child in value.items():
                yield from walk_pins(child, prefix + "/" + key)
    elif isinstance(value, list):
        for index, child in enumerate(value):
            yield from walk_pins(child, prefix + "/" + str(index))


def baseline_checks():
    baseline = read("p1307-r4-baseline.json")
    checks = {"baseline_identity": sha("p1307-r4-baseline.json") == BASELINE_SHA}
    checks["head_unchanged"] = source_state()["head"].strip() == baseline["state"]["head"].strip()
    changed_sources = []
    changed_contracts = []
    for path, digest in baseline["source_inventory"].items():
        if sha(path) != digest:
            (changed_contracts if path.startswith("00_nucleo/prompts/")
             else changed_sources).append(path)
    checks["no_candidate_source_change"] = not changed_sources
    predecessor_changes = [name for name, digest in baseline["protected_predecessors"].items()
                           if sha(name) != digest]
    checks["predecessors_unchanged"] = not predecessor_changes
    closure_changes = [key for key, item in baseline["p1306_closure"].items()
                       if not check_pin(item)]
    checks["P1306_closure_unchanged"] = not closure_changes
    checks["baseline_binaries_unchanged"] = all(check_pin(v) for v in baseline["binaries"].values())
    checks["index_unstaged"] = source_state()["staged"] == baseline["state"]["staged"]
    return {"checks": checks, "changed_sources": changed_sources,
            "changed_contracts": changed_contracts,
            "changed_predecessors": predecessor_changes,
            "changed_P1306_closure": closure_changes}


def manifest_checks(manifest):
    failed = []
    items = list(walk_pins(manifest.get("inputs", {}), "inputs"))
    items += list(walk_pins(manifest.get("current_contracts", []), "current_contracts"))
    items += list(walk_pins(manifest.get("suites", []), "suites"))
    for location, item in items:
        if not check_pin(item):
            failed.append({"location": location, "pin": item})
    baselines = [item for _, item in items if Path(item["path"]).name == "p1307-r4-baseline.json"]
    checks = {"manifest_has_checked_inputs": bool(items),
              "manifest_input_hashes": not failed,
              "manifest_baseline_pinned": any(i["sha256"] == BASELINE_SHA for i in baselines),
              "manifest_has_contracts": bool(manifest.get("current_contracts")),
              "manifest_has_suites": bool(manifest.get("suites")),
              "no_candidate_declared": manifest.get("candidate_exists") is False,
              "authorization_recorded": bool(manifest.get("authorization")),
              "unknown_policy_recorded": bool(manifest.get("unknown_policy")),
              "writer_audit_pinned": any(Path(i["path"]).name == "p1307-r4-writers.md" for _, i in items)}
    state = baseline_checks()
    declared = {str(resolve(i["path"]).relative_to(ROOT))
                for _, i in walk_pins(manifest.get("current_contracts", []))}
    checks["all_changed_contracts_declared"] = set(state["changed_contracts"]) <= declared
    checks.update(state["checks"])
    return {"checks": checks, "failed_pins": failed, "baseline_details": state}


def load_runner(item, number):
    if not check_pin(item):
        raise ValueError("Cannot execute an unpinned or changed classifier")
    spec = importlib.util.spec_from_file_location("p1307_r4_judged_oracle_" + str(number), resolve(item["path"]))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    if not callable(getattr(module, "classify", None)):
        raise ValueError("Frozen runner has no public classify function")
    return module


def changed_value(value):
    if isinstance(value, bool):
        return not value
    if isinstance(value, int):
        return value + 1
    if isinstance(value, float):
        return 0.0 if value != 0.0 else 1.0
    if isinstance(value, str):
        return value + "\n"
    if isinstance(value, list):
        return value + ["p1307-r4-observation-mutation"]
    if isinstance(value, dict):
        return {**value, "p1307-r4-observation-mutation": True}
    return "p1307-r4-observation-mutation"


def observation_mutants(expected):
    """Shape-valid counterfactual observations; no claim of product reachability."""
    mutations = []

    def emit(name, key, value):
        mutant = copy.deepcopy(expected)
        mutant[key] = value
        if mutant != expected:
            mutations.append((name, mutant))

    if expected.get("kind") == "value":
        value = expected["value"]
        emit("changed-complete-value", "value", changed_value(value))
        if isinstance(value, str) and value:
            emit("truncated-string", "value", value[:-1])
            emit("changed-leading-character", "value", "!" + value[1:])
        if isinstance(value, list) and value:
            replacement = copy.deepcopy(value)
            replacement[0] = changed_value(replacement[0])
            emit("changed-payload-member", "value", replacement)
            emit("lost-payload-member", "value", value[:-1])
            emit("reordered-payload", "value", list(reversed(value)))
        if "stderr" in expected:
            emit("unexpected-success-diagnostic", "stderr", "warning: unexpected\n")
        if "exit" in expected:
            emit("success-exit-changed", "exit", 1)
    else:
        key = "stderr" if "stderr" in expected else "stderr_literal"
        stderr = expected[key]
        emit("lost-diagnostic", key, "")
        emit("extra-diagnostic", key, stderr + "\nerror: unexpected diagnostic\n")
        emit("changed-severity", key, stderr.replace("error:", "warning:", 1))
        emit("changed-primary-message", "messages", ["incorrect diagnostic", *expected["messages"][1:]])
        emit("added-hint", "hints", expected["hints"] + ["unexpected hint"])
        if expected["hints"]:
            emit("lost-hint", "hints", expected["hints"][:-1])
            emit("reordered-hints", "hints", list(reversed(expected["hints"])))
        emit("changed-source-identity", key, stderr.replace("<input-expression>", "<different-source>", 1))
        match = re.search(r"([┌└]─ [^\n]+):(\d+):(\d+)", stderr)
        if match:
            for name, index in [("changed-line", 2), ("changed-column", 3)]:
                start, end = match.span(index)
                emit(name, key, stderr[:start] + str(int(match[index]) + 1) + stderr[end:])
        if "^" in stderr:
            emit("expanded-field-range", key, stderr.replace("^", "^^", 1))
        if "stdout" in expected:
            emit("error-stdout-polluted", "stdout", expected["stdout"] + "unexpected\n")
        if "exit" in expected:
            emit("error-exit-changed", "exit", 0)
    return mutations


def classify_safely(module, expected, actual):
    try:
        result = module.classify(expected, actual)
    except Exception as exc:
        return {"verdict": "Unknown", "exception": type(exc).__name__ + ": " + str(exc)}
    if isinstance(result, dict):
        return result
    return {"verdict": result}


def measured_provenance(oracle, module):
    """Resolve every expected cell to pinned raw CLI output and reparse it."""
    failures, evidence, cached = [], [], {}
    for name, digest in oracle.get("inputs", {}).items():
        if not isinstance(digest, str) or sha(name) != digest:
            failures.append({"reason": "oracle-input-pin-mismatch", "path": name})
    for path, digest in oracle.get("contract_pins", {}).items():
        if sha(path) != digest:
            failures.append({"reason": "oracle-contract-pin-mismatch", "path": path})
    for case in oracle.get("cases", []):
        ref = case.get("measurement_ref", {})
        artifact = ref.get("artifact")
        if not artifact or artifact not in oracle.get("inputs", {}):
            failures.append({"case": case.get("id"), "reason": "measurement-not-pinned"})
            continue
        if artifact not in cached:
            cached[artifact] = read(artifact)
        for profile, cell in case["observations"].items():
            for side in ("vanilla", "baseline"):
                collection = cached[artifact]
                try:
                    for part in ref["collection"].replace("<side>", side).split("."):
                        collection = collection[part]
                    rows = [r for r in collection
                            if r[ref["case_key"]] == ref["case_id"]
                            and r[ref["profile_key"]] == profile
                            and (not ref.get("side_key") or r[ref["side_key"]] == side)]
                    if len(rows) != 1:
                        raise ValueError("ambiguous or missing raw measurement row")
                    row = rows[0]
                    stdout = row.get("stdout")
                    if stdout is None:
                        stdout = base64.b64decode(row["stdout_base64"], validate=True).decode("utf-8")
                    observed = module.envelope(row["returncode"], stdout, row["stderr"], case)
                    literal = cell[side + "_literal"]
                    verdict = classify_safely(module, literal, observed)["verdict"]
                    source_checked = True
                    if case["route"] == "eval":
                        argv = row.get("argv", [])
                        source_checked = len(argv) > 2 and argv[1:3] == ["eval", case["expression"]]
                    item = {"case": case["id"], "profile": profile, "side": side,
                            "measurement": ref, "reparse_verdict": verdict,
                            "source_argument_checked": source_checked,
                            "stdout_sha256": hashlib.sha256(stdout.encode()).hexdigest(),
                            "stderr_sha256": hashlib.sha256(row["stderr"].encode()).hexdigest()}
                    evidence.append(item)
                    if verdict != "Preserved" or not source_checked:
                        failures.append(item)
                except (KeyError, TypeError, ValueError, UnicodeError) as exc:
                    failures.append({"case": case["id"], "profile": profile,
                                     "side": side, "reason": str(exc)})
            chosen = cell.get("expected_side")
            if chosen not in ("vanilla", "baseline") or cell.get("future_expected") != cell.get(str(chosen) + "_literal"):
                failures.append({"case": case["id"], "profile": profile,
                                 "reason": "expected-not-equal-to-declared-measured-side"})
    order_limits = []
    for name, data in cached.items():
        if data.get("orders", {}).get("repeat") == "NOT_EXECUTED_OBSERVABILITY_STOP":
            order_limits.append({"artifact": name, "fact": "Predecessor normal raw outputs are reused; repeat/reverse binary runs were not executed. No full preseal product-order stability is asserted; full final candidate orders remain mandatory."})
    return {"success": not failures and bool(evidence), "raw_reparse": evidence,
            "failures": failures, "order_limits": order_limits}


def discriminate_suite(suite, number):
    oracle = read(suite["oracle"]["path"])
    module = load_runner(suite["runner"], number)
    samples = []
    case_ids = [case.get("id") for case in oracle.get("cases", [])]
    if len(case_ids) != len(set(case_ids)):
        raise ValueError("Duplicate oracle case identities")
    for case in oracle.get("cases", []):
        if set(case.get("observations", {})) != {"default", "html", "a11y", "html+a11y"}:
            raise ValueError("Missing or unexpected mandatory feature profile: " + str(case.get("id")))
        if case.get("source_sha256"):
            source = case.get("document", case.get("expression"))
            if not isinstance(source, str) or hashlib.sha256(source.encode()).hexdigest() != case["source_sha256"]:
                raise ValueError("Fixture source hash mismatch: " + str(case.get("id")))
        for profile, observation in case.get("observations", {}).items():
            samples.append((case, profile, observation))
    if not samples:
        raise ValueError("No measured case/profile expectations")
    provenance = measured_provenance(oracle, module)
    positives, negatives, incomplete, baseline = [], [], [], []
    for order, ordered in [("normal", samples), ("repeat", samples), ("reverse", list(reversed(samples)))]:
        for case, profile, observation in ordered:
            expected = observation.get("future_expected")
            key = {"case": case["id"], "profile": profile, "order": order}
            judgment = classify_safely(module, expected, copy.deepcopy(expected))
            positives.append({**key, **judgment})
            if judgment.get("verdict") != "Preserved":
                continue
            for mutation, actual in observation_mutants(expected):
                judgment = classify_safely(module, expected, actual)
                negatives.append({**key, "mutation": mutation, "counterfactual": actual,
                                  **judgment})
            # Real absence/malformed observation state, not a fabricated Typst input.
            for cause, actual in [("missing-public-observation", None),
                                  ("incomplete-public-observation", {"kind": expected["kind"]}),
                                  ("execution-failure", {"kind": "Unknown", "reason": "TimeoutExpired"})]:
                incomplete.append({**key, "cause": cause,
                                   **classify_safely(module, expected, actual)})
            if order == "normal":
                observed = observation.get("baseline_literal")
                baseline.append({**key, **classify_safely(module, expected, observed)})
    killed = sum(r.get("verdict") == "Violated" for r in negatives)
    success = (provenance["success"] and bool(negatives) and all(r.get("verdict") == "Preserved" for r in positives)
               and killed == len(negatives)
               and all(r.get("verdict") == "Unknown" for r in incomplete))
    return {"oracle": suite["oracle"], "runner": suite["runner"],
            "measured_provenance": provenance,
            "positive_observation_self_controls": positives,
            "negative_public_counterfactuals": negatives,
            "incomplete_observation_controls": incomplete,
            "baseline_comparisons": baseline,
            "case_profile_pairs": len(samples),
            "public_counterfactual_rejection_fraction": killed / len(negatives) if negatives else None,
            "success": success,
            "actual_product_mutants_executed": 0,
            "actual_product_mutation_score": None}


def prepare(args):
    result = {"at": now(), "mode": "read-only-preparation", **baseline_checks()}
    if args.manifest and resolve(args.manifest).exists():
        result["manifest"] = manifest_checks(read(args.manifest))
    print(json.dumps(result, ensure_ascii=False, indent=2))


def discriminate(args):
    manifest = read(args.manifest)
    started, tick = now(), time.monotonic()
    integrity = manifest_checks(manifest)
    suites = []
    failures = []
    if all(integrity["checks"].values()):
        for number, suite in enumerate(manifest["suites"]):
            try:
                suites.append(discriminate_suite(suite, number))
            except Exception as exc:
                failures.append({"suite": number, "reason": type(exc).__name__ + ": " + str(exc)})
    success = all(integrity["checks"].values()) and bool(suites) and not failures and all(s["success"] for s in suites)
    data = {"schema": "p1307-r4-public-discriminator-v1", "executor": "/root/p1306_oracle",
            "regime": "executado sem atestacao de isolamento tecnico",
            "started": started, "finished": now(), "seconds": time.monotonic() - tick,
            "manifest": pin(args.manifest), "verifier": pin(__file__),
            "integrity": integrity, "suites": suites, "failures": failures,
            "public_discriminator_passed": success,
            "actual_product_mutants_executed": 0, "actual_product_mutation_score": None,
            "preparation_instrumentation": [{"issue": "Root Cargo.lock initially resolved under diagnostics in read-only prepare", "repair": "Verifier path resolver checks existing root-level paths", "recheck": "All frozen baseline integrity checks passed 2026-09-07T17:41:46.998475+00:00", "judged_inputs_changed": False}],
            "limit": "This measures the frozen comparator's sensitivity to public counterfactuals. It does not demonstrate reachable Rust mutation death or candidate correctness."}
    save("p1307-r4-discriminator.json", data)
    print(json.dumps({"success": success, "failures": failures, "integrity": integrity["checks"]}))
    return 0 if success else 1


def seal(args):
    manifest = read(args.manifest)
    discriminator = read("p1307-r4-discriminator.json")
    integrity = manifest_checks(manifest)
    success = (all(integrity["checks"].values())
               and discriminator.get("public_discriminator_passed") is True
               and discriminator["manifest"]["sha256"] == sha(args.manifest)
               and check_pin(discriminator["verifier"]))
    if not success:
        raise SystemExit("Seal refused: incomplete gate or changed frozen input")
    data = {"schema": "p1307-r4-preimplementation-seal-v1", "at": now(),
            "executor": "/root/p1306_oracle", "manifest": pin(args.manifest),
            "discriminator": pin("p1307-r4-discriminator.json"), "verifier": pin(__file__),
            "baseline": pin("p1307-r4-baseline.json"), "sealed": True,
            "regime": "executado sem atestacao de isolamento tecnico",
            "public_discriminator_passed": True,
            "actual_product_mutation_score": None,
            "remaining_obligations": ["independent semantic RED", "candidate GREEN",
                                      "actual applicable Rust mutants rejected with correct witnesses",
                                      "full frozen public matrix stable", "workspace and lineage gates",
                                      "independent final certificate"],
            "scope": "Inputs frozen for implementation; not a product correctness certificate."}
    save("p1307-r4-seal.json", data)
    print(json.dumps(pin("p1307-r4-seal.json")))


def blocked(args):
    """Record the pre-manifest information-boundary finding, never a seal."""
    started = now()
    integrity = baseline_checks()
    if not all(integrity["checks"].values()):
        raise SystemExit("Cannot establish blocked-state provenance: baseline changed")
    focal = read("p1307-r4-content-observability.json")
    if sha("p1307-r4-content-observability.json") != "26a27995e98dcde01a7dbb47838a40ad8394fc39ac3af8e2e7863cc6639e6634":
        raise SystemExit("Changed Content observation receipt")
    oracle = read("p1307-r4-oracle.json")
    suite = {"oracle": pin("p1307-r4-oracle.json"),
             "runner": {"path": str(D / "p1307-r4-oracle.py"),
                        "sha256": oracle["script_sha256"]}}
    runner = load_runner(suite["runner"], "content-audit")
    cases = {case["id"]: case for case in focal["cases"]}
    matched = 0
    for row in focal["rows"]:
        case = cases[row["case"]]
        assert hashlib.sha256(case["document"].encode()).hexdigest() == case["source_sha256"]
        assert runner.envelope(row["returncode"], row["stdout"], row["stderr"], case) == row["observable"]
        matched += 1
    assert not focal["unknowns"] and not focal["failed_content_construction"]
    for pair in focal["pairs"]:
        if pair["projection"] in ("fields", "repr"):
            assert pair["observations"]["baseline"]["same_literal"] is True
            assert pair["observations"]["vanilla"]["same_literal"] is False
    provisional = discriminate_suite(suite, 0)
    source_specs = [
        ("00_nucleo/prompts/compiler/stdlib/loading.md", [269, 288], "Pure encoder, no World or new context execution; all public Content fields required, no fixture/origin special cases."),
        ("01_core/src/entities/elements/heading.rs", [23, 41], "Five declared fields: level, body, outlined, bookmarked, set_fields. No realized numbering pattern or supplement payload."),
        ("01_core/src/compiler/eval/rules.rs", [970, 1003], "Numbering string is accepted and stored in heading.numbering.pattern on style chain. Other heading arguments are explicitly ignored (DEBT-10)."),
        ("01_core/src/compiler/introspect.rs", [1328, 1350], "Only numbering-active bool baked into payload; label separate in ElementInfo; queried elements store content.clone()."),
        ("01_core/src/compiler/introspect.rs", [1887, 1890], "Styled traversal passes styles via temporary chain while descending into bare body."),
        ("01_core/src/entities/introspector.rs", [365, 388], "elements stores Content; heading_numbering stores bool, not pattern; label registry is separate."),
        ("01_core/src/compiler/stdlib/foundations/query.rs", [50, 64], "Query clones element_at content into LocatedContent(c.clone(), loc)."),
        ("01_core/src/entities/value.rs", [58, 67], "LocatedContent transports Content and Location only."),
        ("01_core/src/entities/content.rs", [3522, 3526], "Heading field access delegates to HeadingElem get_field."),
    ]
    inputs = ["p1307-r4-preflight.json", "p1307-r4-baseline.json", "p1307-r3-lineage.json",
              "p1307-r4-lineage.json", "p1307-r4-transport-amendment.json",
              "p1307-r4-contract-refinement.json", "p1307-r4-call-span-refinement.json",
              "p1307-r4-oracle.json", "p1307-r4-oracle.py", "p1307-r4-measurement.json",
              "p1307-r4-content-observability.py", "p1307-r4-content-observability.json",
              "p1307-r4-math-oracle.py", "p1307-r4-math-oracle.json",
              "p1307-r4-math-measurement.json", "p1307-r4-writers.md"]
    assert sha("p1307-r4-lineage.json") == "7d1b1130de0461f0c9f8a030f9d7b476fee3eb6efbf4bdf4c9c443c3b334f3fb"
    data = {
        "schema": "p1307-r4-independent-preimplementation-verification-v1",
        "verdict": "BLOCKED_CONTRACT_INPUT_DATA",
        "phase": "preimplementation-contract-audit-interrupted-before-final-manifest",
        "started": started, "finished": now(), "executor": "/root/p1306_oracle",
        "regime": "executado sem atestacao de isolamento tecnico",
        "authority": {"prior_role": "P1306 oracle/adversary/test author, not P1307 author",
                      "P1307_contract_or_product_or_oracle_authorship": False,
                      "judged_inputs_edited": False, "candidate_patch_read": False,
                      "writes": ["p1307-r4-verify.py", "p1307-r4-verification.json"],
                      "environment": "shared workspace; role allowlists procedural, no technical isolation attestation"},
        "manifest": None, "manifest_absence_reason": "Gate stopped on input-information defect before final manifesto; no synthetic manifesto created.",
        "seal_issued": False, "certificate_issued": False, "candidate_exists": False,
        "actual_product_mutants_executed": 0, "actual_product_mutation_score": None,
        "candidate_functional_verdict": "NOT_EXECUTED",
        "inputs": [pin(name) for name in inputs], "verifier": pin(__file__),
        "reproduction": {"argv": [sys.executable, str(Path(__file__).resolve()), "blocked"],
                         "cwd": str(ROOT), "binary_reexecution": False,
                         "method": "Reparse pinned raw measurements and independently audit baseline sources; public counterfactual replay is in-memory only."},
        "state": source_state(), "integrity": integrity,
        "focal": {"receipt": pin("p1307-r4-content-observability.json"),
                  "cases": len(cases), "raw_rows_reparsed_and_matched": matched,
                  "profile": "default only", "unknowns": focal["unknowns"],
                  "failed_content_construction": focal["failed_content_construction"],
                  "pairs": focal["pairs"],
                  "limits": ["Equal repr/fields alone does not prove internal information loss; source audit supplies causal support.",
                             "Baseline json.encode absence is not evidence of missing Content data.",
                             "Supplement Alpha/Beta includes earlier ignored-set-rule debt, distinct from accepted numbering 1/I transport loss.",
                             "This focal is not a four-profile or repeated binary acceptance gate."]},
        "source_witnesses": [{**pin(path), "lines": lines, "finding": finding}
                             for path, lines, finding in source_specs],
        "finding": "The approved pure Value-to-encoder boundary lacks realized heading information demanded by mandatory queried-Content encoding. Accepted distinct numbering patterns are stored in the style chain but not in queried Content. A bare LocatedContent location does not itself contain the missing fields. Filling fixture defaults would fabricate language data.",
        "scope_of_conclusion": "Insufficiency of the currently approved owner/input contract, not impossibility of all designs and not proof that a new public API is unavoidable. Upstream realization/transport contracts must be specified and authorized, or mandatory scope legitimately revised, before implementation.",
        "provisional_main_suite_review": {
            "status": "READ_ONLY_CALIBRATION_NOT_FORMAL_GATE_OR_SEAL",
            "cases": len(oracle["cases"]), "case_profile_cells": provisional["case_profile_pairs"],
            "measured_provenance": provisional["measured_provenance"],
            "positive_self_controls_Preserved": sum(x["verdict"] == "Preserved" for x in provisional["positive_observation_self_controls"]),
            "negative_public_counterfactuals_Violated": sum(x["verdict"] == "Violated" for x in provisional["negative_public_counterfactuals"]),
            "absent_incomplete_observation_controls_Unknown": sum(x["verdict"] == "Unknown" for x in provisional["incomplete_observation_controls"]),
            "public_counterfactual_rejection_fraction": provisional["public_counterfactual_rejection_fraction"],
            "limits": ["Normal/repeat/reverse here traverse recorded envelopes, not fresh binary runs.",
                       "The 332 inherited predecessor cases have normal raw binary evidence only; repeat/reverse were previously stopped.",
                       "Counterfactual rejection is comparator sensitivity, not reachable Rust mutant death or candidate correctness.",
                       "Unknown controls are missing or malformed observations, not fabricated opaque Typst product fixtures.",
                       "Math supplement is pinned, but its full discriminator is not executed because the contract boundary is blocked."]},
        "next_required_decision": "Authorize reopening the necessary Content/query/style-realization/introspection L0 owners, with measured language obligations and transport design audited before any corresponding Rust or tests. Existing Args approval is not revoked or silently expanded.",
        "skill_stop_basis": {"path": "/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/artefatos-e-gates.md",
                            "rule": "Information absent from public input/output requires an insufficiency diagnosis, not a seal; Unknown never becomes success."},
    }
    save("p1307-r4-verification.json", data)
    print(json.dumps({"verdict": data["verdict"], "artifact": pin("p1307-r4-verification.json"),
                      "integrity": integrity["checks"], "raw_rows": matched}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["prepare", "discriminate", "seal", "blocked"])
    parser.add_argument("--manifest", default="p1307-r4-manifest.json")
    args = parser.parse_args()
    sys.exit({"prepare": prepare, "discriminate": discriminate, "seal": seal, "blocked": blocked}[args.mode](args))
