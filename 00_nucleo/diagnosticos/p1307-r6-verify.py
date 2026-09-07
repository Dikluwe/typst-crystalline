#!/usr/bin/env python3
"""R6 independent documentary integrity and public-envelope discrimination.

No judged input is edited. A preimplementation seal is not product acceptance.
Product mutation execution and final verification remain separate obligations.
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
BASELINE_PIN = "8cc0eae00457d2e7d54b420024eae49344032b34b295b4eda536faeb1ec3c4a3"
HELPER_PIN = "38ef5898bccc75f06c050a25ef81c38e76e8e4259ff90c8ab47f910260791241"
CLASSIFIER_PIN = "ef102f3a800475855b0cb21f40db666312cdb2f96fd9c867ea625b13c22b8e68"
REVIEWED_PRESEAL_DELTAS = {
    "00_nucleo/prompts/compiler/eval/bindings.md":
        "3f6a4ab89174439f623ac37dfab9fb4ba03cf4e9d2f6c808704c75b617fda195",
}
OUTPUTS = {"p1307-r6-discriminator.json", "p1307-r6-seal.json",
           "p1307-r6-verification.json", "p1307-r6-certificate.json"}


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def path(value):
    value = Path(value)
    if value.is_absolute():
        return value
    return ROOT / value if len(value.parts) > 1 or (ROOT / value).exists() else D / value


def sha(value):
    return hashlib.sha256(path(value).read_bytes()).hexdigest()


def pin(value):
    return {"path": str(path(value)), "sha256": sha(value)}


def read(value):
    return json.loads(path(value).read_text())


def save(name, data):
    if name not in OUTPUTS:
        raise ValueError("Output is outside verifier authority")
    with (D / name).open("x") as output:
        json.dump(data, output, ensure_ascii=False, indent=2)
        output.write("\n")


def load_module(filename, digest, module_name):
    if sha(filename) != digest:
        raise ValueError("Changed executable input: " + filename)
    spec = importlib.util.spec_from_file_location(module_name, path(filename))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def helpers():
    return load_module("p1307-r4-verify.py", HELPER_PIN, "r6_frozen_helpers")


def classifier():
    return load_module("p1307-r4-oracle.py", CLASSIFIER_PIN, "r6_frozen_classifier")


def command(argv):
    started = now()
    result = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
    return {"argv": argv, "cwd": str(ROOT), "at": started,
            "exit": result.returncode, "stdout": result.stdout, "stderr": result.stderr}


def state():
    return {key: command(["git", *args]) for key, args in {
        "head": ["rev-parse", "HEAD"], "status": ["status", "--short"],
        "diff_stat": ["diff", "HEAD", "--stat"],
        "staged": ["diff", "--cached", "--binary"]}.items()}


def semantic_l0_hash(value):
    """Only the causal code-hash mirror is metadata, never arbitrary prose."""
    text = path(value).read_text()
    normalized, count = re.subn(r"(?m)^Hash do Código: [^\n]*$", "Hash do Código: <metadata>", text)
    if count > 1:
        raise ValueError("Ambiguous L0 metadata")
    return hashlib.sha256(normalized.encode()).hexdigest()


def walk_pins(value):
    if isinstance(value, dict):
        if "path" in value and "sha256" in value:
            yield value
        else:
            for child in value.values():
                yield from walk_pins(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk_pins(child)


def integrity(manifest=None):
    baseline = read("p1307-r6-baseline.json")
    checks = {"baseline_pin": sha("p1307-r6-baseline.json") == BASELINE_PIN}
    changes = [name for name, digest in baseline["source_inventory"].items() if sha(name) != digest]
    predecessors = [name for name, digest in baseline["protected_predecessors"].items() if sha(name) != digest]
    approved_deltas = []
    if manifest is not None:
        for item in manifest.get("preseal_amendments", []):
            name = item["path"]
            if (REVIEWED_PRESEAL_DELTAS.get(name) == item["after_sha256"]
                    and baseline["source_inventory"].get(name) == item["before_sha256"]
                    and sha(name) == item["after_sha256"]
                    and sha(item["receipt"]["path"]) == item["receipt"]["sha256"]):
                approved_deltas.append(name)
    checks["baseline_unchanged_except_reviewed_preseal_contract_delta"] = not (set(changes) - set(approved_deltas))
    checks["protected_predecessors_unchanged"] = not predecessors
    checks["baseline_binaries_pinned"] = all(sha(v["path"]) == v["sha256"] for v in baseline["binaries"].values())
    current = state()
    checks["HEAD_unchanged"] = current["head"]["stdout"] == baseline["state"]["head"]
    checks["index_unchanged"] = current["staged"]["stdout"] == baseline["state"]["staged"]
    checks["R5_review_pinned"] = sha("p1307-r5-review.json") == baseline["review_sha256"]
    checks["approved_R5_contracts_unchanged"] = all(sha(v["path"]) == v["sha256"] for v in read("p1307-r5-review.json")["contracts"])
    checks["user_authorization"] = baseline["authorization"]["message"] == "autorizo"
    checks["helpers_pinned"] = sha("p1307-r4-verify.py") == HELPER_PIN
    checks["classifier_pinned"] = sha("p1307-r4-oracle.py") == CLASSIFIER_PIN
    failed_pins = []
    if manifest is not None:
        pins = list(walk_pins(manifest))
        for item in pins:
            try:
                if sha(item["path"]) != item["sha256"]:
                    failed_pins.append(item)
            except OSError:
                failed_pins.append(item)
        checks["manifest_pins"] = bool(pins) and not failed_pins
        checks["manifest_baseline"] = any(Path(v["path"]).name == "p1307-r6-baseline.json" and v["sha256"] == BASELINE_PIN for v in pins)
        checks["manifest_candidate_absent"] = manifest.get("candidate_exists") is False
        checks["manifest_authorization"] = bool(manifest.get("authorization"))
        checks["manifest_unknown_policy"] = bool(manifest.get("unknown_policy"))
        checks["manifest_suites"] = bool(manifest.get("suites"))
        checks["manifest_tests"] = bool(manifest.get("independent_tests"))
        contracts = {str(path(v["path"])) for v in manifest.get("current_contracts", [])}
        checks["all_R5_contracts_in_manifest"] = all(str(path(v["path"])) in contracts for v in read("p1307-r5-review.json")["contracts"])
    return {"checks": checks, "changed_baseline_files": changes,
            "reviewed_preseal_contract_deltas": approved_deltas,
            "changed_predecessors": predecessors, "failed_pins": failed_pins,
            "state": current}


def discriminate_suite(suite):
    """Public envelope calibration only, not causal Rust mutation testing."""
    oracle = read(suite["oracle"]["path"])
    compare, helper = classifier(), helpers()
    provenance = expected_provenance(oracle)
    lineage = inherited_selection(oracle)
    rows, positives, negatives, opaque = [], 0, 0, 0
    ids = set()
    for case in oracle["cases"]:
        if case["id"] in ids:
            raise ValueError("Duplicate case id")
        ids.add(case["id"])
        if hashlib.sha256(case.get("document", case["expression"]).encode()).hexdigest() != case["source_sha256"]:
            raise ValueError("Fixture identity mismatch: " + case["id"])
        for profile, cell in case["observations"].items():
            if profile not in {"default", "html", "a11y", "html+a11y"}:
                raise ValueError("Unknown profile")
            expected = cell["future_expected"]
            rows.append((case["id"], profile, expected))
    observations = []
    for order in ("normal", "repeat", "reverse"):
        ordered = list(reversed(rows)) if order == "reverse" else rows
        for case, profile, expected in ordered:
            observed = copy.deepcopy(expected)
            positive = compare.classify(expected, observed)
            negatives_here = []
            for mutation, mutated in helper.observation_mutants(expected):
                negatives_here.append({"mutation": mutation, "observed": mutated,
                                       "verdict": compare.classify(expected, mutated)})
            incomplete_here = []
            for cause, missing in (("no-observation", None),
                                   ("incomplete-envelope", {"kind": expected.get("kind")}),
                                   ("execution-interrupted", {"kind": "Unknown", "reason": "TimeoutExpired"})):
                incomplete_here.append({"cause": cause, "observed": missing,
                                        "verdict": compare.classify(expected, missing)})
            observations.append({"case": case, "profile": profile, "order": order,
                                 "positive": positive, "negative_public_counterfactuals": negatives_here,
                                 "absent_observation_controls": incomplete_here})
            positives += positive == "Preserved"
            negatives += sum(x["verdict"] == "Violated" for x in negatives_here)
            opaque += sum(x["verdict"] == "Unknown" for x in incomplete_here)
    total_negative = sum(len(x["negative_public_counterfactuals"]) for x in observations)
    success = (provenance["success"] and lineage["success"]
               and bool(rows) and total_negative > 0 and positives == len(observations)
               and negatives == total_negative and opaque == 3 * len(observations))
    return {"oracle": suite["oracle"], "cases": len(ids), "case_profile_cells": len(rows),
            "expected_provenance": provenance, "inherited_selection": lineage,
            "positive_Preserved": positives, "negative_Violated": negatives,
            "opaque_Unknown": opaque, "public_counterfactual_rejection_fraction": negatives / total_negative if total_negative else None,
            "observations": observations, "success": success,
            "actual_Rust_mutants": 0, "actual_Rust_mutation_score": None}


def inherited_selection(oracle):
    """Explicit refinement, not silent weakening or broad expectation rewriting."""
    cases = {c["id"]: c for c in oracle["cases"]}
    evidence, failures = [], []
    replacement = next(c for c in read("p1307-r5-repr-oracle.json")["cases"] if c["id"] == "default.repr")
    for parent_name in ("p1307-r4-oracle.json", "p1307-r4-math-oracle.json"):
        for old in read(parent_name)["cases"]:
            new = cases.get(old["id"])
            selected = replacement if old["id"] == "r2.construct-LocatedContent" else old
            okay = bool(new) and new["source_sha256"] == old["source_sha256"]
            okay = okay and set(new["observations"]) == set(selected["observations"])
            okay = okay and all(new["observations"][p]["future_expected"] == c["future_expected"] for p, c in selected["observations"].items())
            item = {"case": old["id"], "parent": parent_name, "source_and_expected_preserved_or_explicitly_superseded": okay}
            evidence.append(item)
            if not okay:
                failures.append(item)
    selected_r5 = {s + ".json" for s in ("query-default", "numbering-1", "numbering-I", "numbering-none", "language-en", "language-pt")}
    selected_r5 |= {"equality." + s for s in ("inline-vs-query", "clone-through-array", "different-labels", "no-labels", "same-labels", "different-numbering")}
    selected_r5 |= {"access.error-label", "access.error-missing"}
    found_r5 = {c["id"].removeprefix("r5content.") for c in oracle["cases"] if c["id"].startswith("r5content.")}
    if found_r5 != selected_r5:
        failures.append({"reason": "R5 selection differs from reviewed supported-domain set", "actual": sorted(found_r5)})
    if oracle.get("r4_supersessions") != ["r2.construct-LocatedContent"]:
        failures.append({"reason": "Undeclared or excessive predecessor supersession"})
    return {"success": not failures, "inherited_cases_checked": len(evidence),
            "supported_R5_cases": sorted(selected_r5), "evidence": evidence, "failures": failures}


def expected_provenance(oracle):
    """Independently reparse every expected cell's exact pinned raw origin."""
    compare = classifier()
    artifacts, indices, collections = {}, {}, {}
    evidence, failures = [], []
    for name, digest in oracle["inputs"].items():
        if sha(name) != digest:
            failures.append({"reason": "oracle input changed", "path": name})

    def source_index(value, result):
        if isinstance(value, dict):
            if "expression" in value and ("id" in value or "case_id" in value):
                result.setdefault(value.get("id", value.get("case_id")), []).append(value)
            for child in value.values():
                source_index(child, result)
        elif isinstance(value, list):
            for child in value:
                source_index(child, result)

    for case in oracle["cases"]:
        for profile, cell in case["observations"].items():
            ref = cell.get("expected_measurement_ref", {})
            item = {"case": case["id"], "profile": profile, "reference": ref}
            try:
                artifact = ref["artifact"]
                if artifact not in oracle["inputs"]:
                    raise ValueError("Unpinned measurement origin")
                if artifact not in artifacts:
                    artifacts[artifact] = read(artifact)
                    indices[artifact] = {}
                    source_index(artifacts[artifact], indices[artifact])
                side = ref["side"]
                origin_profile = ref.get("profile", profile)
                case_id = ref.get("case_id", ref.get("case"))
                case_key = ref.get("case_key", "case")
                rows = []
                names = ref.get("collections", [ref.get("collection")])
                for name in names:
                    side_is_collection_index = "<side>" in name
                    name = name.replace("<side>", side)
                    key = artifact, name
                    if key not in collections:
                        collection = artifacts[artifact]
                        for part in name.split("."):
                            collection = collection[part]
                        collections[key] = collection
                    rows.extend(r for r in collections[key]
                                if r[case_key] == case_id
                                and r[ref.get("profile_key", "profile")] == origin_profile
                                and (not ref.get("side_key")
                                     or r.get(ref["side_key"], side if side_is_collection_index else None) == side)
                                and ("side" not in r or r["side"] == side))
                if len(rows) != 1:
                    raise ValueError("Expected origin is ambiguous or missing: " + str(len(rows)))
                row = rows[0]
                origin_id = ref.get("source_case_id", case_id)
                candidates = indices[artifact].get(origin_id, [])
                digest = row.get("source_sha256", row.get("fixture_sha256"))
                if candidates:
                    matching = [c for c in candidates if digest is None or c.get("source_sha256", c.get("fixture_sha256")) == digest]
                    if not matching:
                        raise ValueError("Origin source hash mismatch")
                    source_case = copy.deepcopy(matching[0])
                elif digest is None or digest == case["source_sha256"]:
                    source_case = copy.deepcopy(case)
                else:
                    raise ValueError("Different origin source is not recorded")
                actual_source = hashlib.sha256(source_case.get("document", source_case["expression"]).encode()).hexdigest()
                if digest and actual_source != digest:
                    raise ValueError("Raw origin document does not match its hash")
                source_case["source_sha256"] = actual_source
                source_case.setdefault("route", case["route"])
                argv = row.get("argv", [])
                if source_case["route"] == "eval":
                    if argv[1:3] != ["eval", source_case["expression"]]:
                        raise ValueError("Eval argv does not identify the source expression")
                elif len(argv) > 3 and argv[1] == "compile":
                    source_case["fixture"] = argv[2]
                    source_case["pdf_output"] = argv[3]
                stdout = row.get("stdout")
                if stdout is None:
                    stdout = base64.b64decode(row["stdout_base64"], validate=True).decode("utf-8")
                observed = compare.envelope(row["returncode"], stdout, row["stderr"], source_case)
                verdict = compare.classify(cell["future_expected"], observed)
                item.update(verdict=verdict, origin_source_sha256=actual_source,
                            stdout_sha256=hashlib.sha256(stdout.encode()).hexdigest(),
                            stderr_sha256=hashlib.sha256(row["stderr"].encode()).hexdigest())
                if verdict != "Preserved":
                    failures.append(item)
            except (KeyError, ValueError, TypeError, UnicodeError, AttributeError) as error:
                item["failure"] = type(error).__name__ + ": " + str(error)
                failures.append(item)
            evidence.append(item)
    return {"success": not failures, "expected_cells_reparsed": len(evidence),
            "evidence": evidence, "failures": failures,
            "adapter_calibration": {"attempt_1": "1328 original P1307 cells had no physical side key; the frozen results.<side> collection already carries that identity. Other 654 origins and all inherited selection checks passed.",
                                    "adjustment": "Resolve side from explicit <side> collection indexing only when the raw row lacks its physical side field; other missing side fields remain failures.",
                                    "judged_inputs_edited": False, "product_processes": 0},
            "limits": "Raw origin may intentionally differ from candidate input for measured CBOR literal controls. Inherited binary repeat/reverse runs are not invented."}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["prepare", "discriminate", "seal"])
    parser.add_argument("--manifest", default="p1307-r6-manifest.json")
    args = parser.parse_args()
    started, tick = now(), time.monotonic()
    manifest = read(args.manifest) if path(args.manifest).exists() else None
    checked = integrity(manifest)
    if args.mode == "prepare":
        print(json.dumps(checked, ensure_ascii=False, indent=2))
        return
    if manifest is None or not all(checked["checks"].values()):
        raise SystemExit("Gate refused: missing manifesto or failed integrity check")
    if args.mode == "discriminate":
        suites = [discriminate_suite(s) for s in manifest["suites"]]
        commands = [command(["git", "diff", "--check"]),
                    command(["crystalline-lint", "--checks", "v15,v26", "--fail-on", "warning", "."])]
        for item in manifest["independent_tests"]:
            if str(item["path"]).endswith(".patch"):
                patch = path(item["path"]).read_text()
                targets = re.findall(r"(?m)^\+\+\+ b/(.+)$", patch)
                if targets != ["01_core/src/compiler/eval/tests.rs"]:
                    raise SystemExit("Test patch contains unreviewed productive targets")
                commands.append(command(["git", "apply", "--check", str(path(item["path"]))]))
        success = all(s["success"] for s in suites) and all(c["exit"] == 0 for c in commands)
        data = {"schema": "p1307-r6-public-discriminator-v1", "started": started,
                "finished": now(), "seconds": time.monotonic() - tick,
                "regime": "executado sem atestacao de isolamento tecnico",
                "executor": "/root/p1306_oracle", "verifier": pin(__file__),
                "manifest": pin(args.manifest), "integrity": checked, "commands": commands,
                "suites": suites, "public_discriminator_passed": success,
                "actual_Rust_mutants": 0, "actual_Rust_mutation_score": None,
                "limit": "Envelope counterfactuals and missing observations only. No product mutation was run; repeated orders are in-memory replay, not new binary matrices."}
        save("p1307-r6-discriminator.json", data)
        print(json.dumps({"success": success, "receipt": pin("p1307-r6-discriminator.json")}))
        if not success:
            raise SystemExit(1)
        return
    discriminator = read("p1307-r6-discriminator.json")
    if (discriminator["manifest"]["sha256"] != sha(args.manifest)
            or discriminator["verifier"]["sha256"] != sha(__file__)
            or not discriminator["public_discriminator_passed"]):
        raise SystemExit("Seal refused: changed discriminator inputs or failed gate")
    data = {"schema": "p1307-r6-preimplementation-seal-v1", "at": now(),
            "executor": "/root/p1306_oracle", "regime": "executado sem atestacao de isolamento tecnico",
            "manifest": pin(args.manifest), "baseline": pin("p1307-r6-baseline.json"),
            "verifier": pin(__file__), "discriminator": pin("p1307-r6-discriminator.json"),
            "integrity": checked, "sealed_for_implementation": True,
            "contract_semantic_hashes": {v["path"]: semantic_l0_hash(v["path"]) for v in manifest["current_contracts"]},
            "metadata_policy": "After implementation only the exact Hash do Código line may change as a causal code-hash mirror; other L0 changes invalidate this seal.",
            "actual_Rust_mutants": 0, "actual_Rust_mutation_score": None,
            "remaining_gates": ["independent semantic RED", "candidate GREEN", "actual compiled Rust mutations rejected", "complete pinned candidate public matrix", "final lineage/build/architecture checks", "independent final verification"],
            "scope": "Approved inputs for implementation, not product correctness or general parity certificate."}
    save("p1307-r6-seal.json", data)
    print(json.dumps(pin("p1307-r6-seal.json")))


if __name__ == "__main__":
    main()
