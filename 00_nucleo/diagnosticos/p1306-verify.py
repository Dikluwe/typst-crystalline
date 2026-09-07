#!/usr/bin/env python3
"""Independent P1306 verification. Writes receipts, never repairs judged inputs."""
import argparse
import collections
import copy
import datetime
import difflib
import hashlib
import importlib.util
import json
import os
import pathlib
import re
import shutil
import subprocess
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
OWNER = "01_core/src/compiler/eval/bindings/field_access.rs"
TESTS = "01_core/src/compiler/eval/tests.rs"
FIELD_L0 = "00_nucleo/prompts/compiler/eval/bindings/field_access.md"
TEST_L0 = "00_nucleo/prompts/compiler/eval/tests.md"


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def digest(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()


def pin(path):
    path = pathlib.Path(path)
    return {"path": str(path), "sha256": digest(path), "bytes": path.stat().st_size}


def load(name):
    return json.loads((DIAG / name).read_text())


def save(name, data):
    path = DIAG / name
    data = {"schema": name.removesuffix(".json") + "-v1", "issuer": "/root/p1306_verifier", "generated_at": now(), **data}
    with path.open("x") as out:
        json.dump(data, out, indent=2, ensure_ascii=False)
        out.write("\n")
    print(json.dumps({"receipt": str(path), "sha256": digest(path)}), flush=True)


def command(argv, cwd=ROOT, env=None, timeout=1800):
    start = time.monotonic()
    p = subprocess.run(argv, cwd=cwd, env=env, capture_output=True, timeout=timeout)
    return {"argv": list(map(str, argv)), "cwd": str(cwd), "finished_at": now(), "duration_seconds": time.monotonic()-start, "exit_code": p.returncode, "stdout": p.stdout.decode(errors="replace"), "stderr": p.stderr.decode(errors="replace")}


def oracle():
    spec = importlib.util.spec_from_file_location("p1306_oracle", DIAG / "p1306-oracle.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def state():
    return {key: command(["git", *args])["stdout"] for key, args in {"head": ["rev-parse", "HEAD"], "status": ["status", "--short"], "diff_stat": ["diff", "HEAD", "--stat"], "diff": ["diff", "HEAD", "--binary"]}.items()}


def preseal():
    o = oracle()
    frozen = load("p1306-oracle-cases.json")
    baseline = load("p1306-baseline-measurement.json")
    expected = {(r["id"], r["profile"]): r["expected"] for r in frozen["expectations"]}
    assert len(expected) == len(frozen["cases"]) * len(frozen["profiles"])
    positives = []
    baseline_counts = collections.Counter()
    for row in baseline["runs"]:
        judgment = o.discriminate(expected[row["id"], row["profile"]], row)
        if row["side"] == "crystalline":
            baseline_counts[judgment["verdict"]] += 1
        if judgment["verdict"] == "Preserved":
            positives.append({"id": row["id"], "profile": row["profile"], "side": row["side"], "phase": row["phase"], "judgment": judgment})
        assert judgment["verdict"] != "Unknown", row
    def envelope(case="ordinary-std", profile="default"):
        return copy.deepcopy(expected[case, profile])
    mutations = []
    def negative(name, change, case="ordinary-std", profile="default"):
        desired = envelope(case, profile)
        changed = copy.deepcopy(desired)
        change(changed)
        row = {**changed, "complete": True, "cwd": "/tmp/p1306-public-envelope-control"}
        assert changed != desired, name
        mutations.append({"id": name, "case": case, "profile": profile, "expected": desired, "observation": row})
    def stderr_replace(a, b):
        return lambda e: e.update(stderr=e["stderr"].replace(a, b))
    negative("old-nominal-conversion", stderr_replace("module `std`", "module `global`"))
    negative("all-global", stderr_replace("module `map`", "module `global`"), "ordinary-map")
    negative("real-global-as-std", stderr_replace("module `global`", "module `std`"), "global")
    negative("lexical-alias-name", stderr_replace("module `std`", "module `alias`"), "ordinary-alias")
    negative("only-nope-fixed", stderr_replace("module `std`", "module `global`"), "ordinary-alias")
    negative("one-profile-regressed", stderr_replace("module `std`", "module `global`"), profile="html")
    negative("target-span", stderr_replace(":1:50", ":1:41"))
    negative("whole-expression-underline", stderr_replace("^^^^", "^^^^^^^^^^^^^"))
    negative("added-diagnostic", lambda e: e.update(stderr=e["stderr"]+"warning: extra diagnostic\n"))
    negative("lost-diagnostic", lambda e: e.update(stderr=""))
    negative("changed-lookup", lambda e: e.update(stdout=e["stdout"].replace(",7]", ",99]")), "positive-std")
    negative("lost-lookup", lambda e: e.update(exit_code=1, stdout=""), "positive-std")
    negative("wrong-severity", stderr_replace("error:", "warning:"))
    negative("wrong-field", stderr_replace("contain `nope`", "contain `absent`"))
    negative("added-hint", lambda e: e.update(stderr=e["stderr"]+"hint: invented hint\n"))
    negative("removed-required-pdf-hint", lambda e: e.update(stderr="\n".join(line for line in e["stderr"].split("\n") if "hint:" not in line)), "pdf-data-cell")
    negative("stdout-on-error", lambda e: e.update(stdout="unexpected output\n"))
    negative("wrong-exit", lambda e: e.update(exit_code=0))
    trials = []
    vectors = []
    for phase in ("normal", "repeat", "reverse"):
        current = {}
        for item in reversed(mutations) if phase == "reverse" else mutations:
            judgment = o.discriminate(item["expected"], item["observation"])
            current[item["id"]] = judgment["verdict"]
            trials.append({"phase": phase, **item, "judgment": judgment})
        vectors.append(current)
    assert vectors[0] == vectors[1] == vectors[2]
    assert all(v == "Violated" for v in vectors[0].values())
    # A real known-valid compilation is interrupted before its public observation
    # completes. No opaque product expression or adapter error is manufactured.
    source = next(r for r in baseline["runs"] if r["id"] == "ordinary-std" and r["side"] == "crystalline")
    process = subprocess.Popen(source["argv"], cwd=source["cwd"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    try:
        process.communicate(timeout=0)
        raise AssertionError("Observation completed: this is not an Unknown control")
    except subprocess.TimeoutExpired:
        process.kill()
        stdout, stderr = process.communicate()
    incomplete = {"argv": source["argv"], "cwd": source["cwd"], "complete": False, "exit_code": process.returncode, "stdout": stdout.decode(errors="replace"), "stderr": stderr.decode(errors="replace"), "observation_budget_seconds": 0, "reason": "Real baseline process interrupted before complete observation; not a product verdict"}
    opaque = [o.discriminate(expected["ordinary-std", "default"], incomplete) for _ in range(3)]
    assert all(item["verdict"] == "Unknown" for item in opaque)
    save("p1306-discriminator.json", {"manifest": pin(DIAG / "p1306-manifest.json"), "oracle": pin(DIAG / "p1306-oracle.py"), "frozen_cases": pin(DIAG / "p1306-oracle-cases.json"), "measurement": pin(DIAG / "p1306-baseline-measurement.json"), "source": state(), "actual_measured_positive_rows": positives, "baseline_candidate_counts": dict(baseline_counts), "public_mutations": trials, "valid_public_mutations": len(mutations), "rejected_public_mutations": len(mutations), "public_mutation_score": 1.0, "stable_order_repeat": True, "incomplete_observation": incomplete, "opaque_judgments": opaque, "required_unknown": 0, "scope": "Preimplementation public envelope discrimination only. Real compiled source mutation score is established separately post-GREEN."})


def matrix():
    o = oracle()
    frozen = load("p1306-oracle-cases.json")
    measurement = load("p1306-final-measurement.json")
    expectations = {(r["id"], r["profile"]): r["expected"] for r in frozen["expectations"]}
    rows, counts, phases = [], collections.Counter(), collections.defaultdict(dict)
    for row in measurement["runs"]:
        judgment = o.discriminate(expectations[row["id"], row["profile"]], row)
        counts[judgment["verdict"]] += 1
        phases[row["phase"]][row["id"], row["profile"]] = o.public(row)
        rows.append({"id": row["id"], "profile": row["profile"], "phase": row["phase"], "judgment": judgment})
    stable = phases["normal"] == phases["repeat"] == phases["reverse"]
    complete = len(rows) == len(expectations) * 3 and all(len(p) == len(expectations) for p in phases.values())
    save("p1306-matrix.json", {"manifest": pin(DIAG / "p1306-manifest.json"), "seal": pin(DIAG / "p1306-seal.json"), "measurement": pin(DIAG / "p1306-final-measurement.json"), "counts": dict(counts), "complete_matrix": complete, "stable_repeat_reverse": stable, "rows": rows, "verdict": "Preserved" if complete and stable and set(counts) == {"Preserved"} else "Violated"})


def check_pins(after_candidate=False, metadata_reopening=False):
    manifest = load("p1306-manifest.json")
    mismatches = []
    overrides = {}
    if after_candidate and (DIAG / "p1306-successor-seal.json").exists():
        overrides = load("p1306-successor-seal.json")["raw_pin_successors"]
    for path, sha in manifest["protected_inputs"].items():
        if digest(ROOT / path) != sha:
            mismatches.append({"path": path, "expected": sha, "actual": digest(ROOT / path)})
    for path, sha in manifest["source_hashes_before_candidate"].items():
        if after_candidate and path in manifest["candidate_mutable_paths"]:
            continue
        if metadata_reopening and path == FIELD_L0:
            continue
        sha = overrides.get(path, sha)
        if digest(ROOT / path) != sha:
            mismatches.append({"path": path, "expected": sha, "actual": digest(ROOT / path)})
    for kind in ("baseline", "vanilla"):
        p = manifest["preflight"]
        path = p[kind + "_binary"]
        if digest(path) != p[kind + "_binary_sha256"]:
            mismatches.append({"path": path, "reason": "binary pin differs"})
    return mismatches


def seal():
    manifest = load("p1306-manifest.json")
    discriminator = load("p1306-discriminator.json")
    mismatches = check_pins()
    assert not mismatches, mismatches
    assert discriminator["manifest"]["sha256"] == digest(DIAG / "p1306-manifest.json")
    assert discriminator["public_mutation_score"] == 1
    assert discriminator["stable_order_repeat"] and discriminator["required_unknown"] == 0
    baseline = subprocess.check_output(["git", "show", manifest["preflight"]["baseline_head"]+":"+OWNER], cwd=ROOT).decode()
    current = (ROOT / OWNER).read_text()
    strip_header = lambda text: "\n".join(line for line in text.splitlines() if not line.startswith("//! @"))
    assert strip_header(baseline) == strip_header(current), "Productive body changed before seal"
    save("p1306-seal.json", {"verdict": "P1306_PREIMPLEMENTATION_SEALED", "regime": "executado sem atestacao de isolamento tecnico", "technical_isolation_attested": False, "manifest": pin(DIAG / "p1306-manifest.json"), "verifier": pin(__file__), "discriminator": pin(DIAG / "p1306-discriminator.json"), "contract": pin(DIAG / "p1306-contract.json"), "baseline": pin(manifest["preflight"]["baseline_binary"]), "vanilla": pin(manifest["preflight"]["vanilla_binary"]), "protected_inputs": manifest["protected_inputs"], "source_hashes_before_candidate": manifest["source_hashes_before_candidate"], "protected_l0_texts": {path: (ROOT/path).read_text() for path in (FIELD_L0, TEST_L0)}, "product_body_matches_baseline": True, "public_mutation_score": 1.0, "actual_compiled_source_mutants": "Required after candidate GREEN; not claimed by this seal", "unknown_policy": manifest["preflight"]["unknown_policy"], "protected_pin_mismatches": mismatches})


def successor():
    initial = load("p1306-seal.json")
    assert "metadata_reopening_policy" in load("p1306-manifest.json")
    assert initial["manifest"]["sha256"] == digest(DIAG / "p1306-manifest.json")
    assert initial["verifier"]["sha256"] == digest(__file__)
    assert not check_pins(after_candidate=True, metadata_reopening=True)
    before = initial["protected_l0_texts"][FIELD_L0]
    after = (ROOT / FIELD_L0).read_text()
    metadata = r"(?m)^Hash do Código: [0-9a-f]+$"
    assert len(re.findall(metadata, before)) == len(re.findall(metadata, after)) == 1
    assert re.sub(metadata, "Hash do Código: <metadata>", before) == re.sub(metadata, "Hash do Código: <metadata>", after)
    assert before != after, "No metadata changed; no successor required"
    source = (ROOT / OWNER).read_bytes()
    header_lines = [line for line in source.splitlines(keepends=True) if line.startswith(b"//! @prompt-hash ")]
    assert len(header_lines) == 1
    canonical_source = b"".join(line for line in source.splitlines(keepends=True) if not line.startswith(b"//! @prompt-hash "))
    calculated_code_hash = hashlib.sha256(canonical_source).hexdigest()[:8]
    assert re.findall(metadata, after)[0] == "Hash do Código: " + calculated_code_hash
    assert (ROOT / TEST_L0).read_text() == initial["protected_l0_texts"][TEST_L0]
    # Re-evaluate every frozen discriminator witness with the unchanged oracle.
    discriminator = load("p1306-discriminator.json")
    o = oracle()
    for trial in discriminator["public_mutations"]:
        assert o.discriminate(trial["expected"], trial["observation"]) == trial["judgment"]
    for judgment in discriminator["opaque_judgments"]:
        assert o.discriminate(load("p1306-oracle-cases.json")["expectations"][0]["expected"], discriminator["incomplete_observation"]) == judgment
    reopening = pin(DIAG / "p1306-lineage-reopening.json")
    semantic = hashlib.sha256(re.sub(metadata, "Hash do Código: <metadata>", after).encode()).hexdigest()
    save("p1306-successor-seal.json", {"verdict": "P1306_METADATA_ONLY_SUCCESSOR_SEALED", "manifest": pin(DIAG / "p1306-manifest.json"), "initial_seal": pin(DIAG / "p1306-seal.json"), "reopening": reopening, "raw_pin_successors": {FIELD_L0: digest(ROOT / FIELD_L0)}, "before_metadata": re.findall(metadata, before)[0], "after_metadata": re.findall(metadata, after)[0], "independently_calculated_code_hash": calculated_code_hash, "unchanged_semantic_l0_sha256": semantic, "all_other_protected_inputs_unchanged": True, "discriminator_classifications_unchanged": True, "normative_oracle_change": False, "causal_scope": "Restart only metadata lineage validation. Initial raw seal retained; no retroactive rewrite. Product implementation remains downstream of original semantic seal."})


def mutate_text(text, attack):
    functions = list(re.finditer(r"^pub[^\n]* fn ([A-Za-z0-9_]+)\(", text, re.MULTILINE))
    found = [i for i, match in enumerate(functions) if match.group(1) == attack["function"]]
    assert len(found) == 1, (attack["id"], "ambiguous function")
    index = found[0]
    begin = functions[index].start()
    end = functions[index+1].start() if index+1 < len(functions) else len(text)
    section = text[begin:end]
    operation = attack["operation"]
    if operation == "replace_between_unique_markers":
        start, stop = attack["start_marker"], attack["end_marker"]
        assert section.count(start) == section.count(stop) == 1
        left, right = section.index(start)+len(start), section.index(stop)
        assert left < right
        replacement = section[:left] + attack["replacement"] + section[right:]
    else:
        anchor = attack["anchor"]
        assert section.count(anchor) == 1, (attack["id"], "ambiguous anchor")
        replacement = section.replace(anchor, anchor+attack["insertion"] if operation == "insert_after_unique_anchor" else attack["replacement"])
    result = text[:begin] + replacement + text[end:]
    assert text != result, (attack["id"], "empty mutation")
    return result


def apply_source(path, before, after):
    assert path.read_text() == before, "Unexpected source state before apply_patch"
    lines = list(difflib.unified_diff(before.splitlines(True), after.splitlines(True), n=3))[2:]
    hunks = "".join("@@\n" if line.startswith("@@") else line for line in lines)
    patch = "*** Begin Patch\n*** Update File: " + str(path) + "\n" + hunks + "*** End Patch\n"
    p = subprocess.run([shutil.which("apply_patch")], input=patch.encode(), capture_output=True)
    assert p.returncode == 0, p.stderr.decode()
    assert path.read_text() == after, "Source after apply_patch differs"
    return patch


def copied_sources(directory):
    return {str(p.relative_to(directory)): digest(p) for part in ("01_core", "02_shell", "03_infra", "04_wiring", "benches", ".cargo") for p in (directory / part).rglob("*") if p.is_file()}


def mutants(seed_target=None):
    manifest = load("p1306-manifest.json")
    attacks = load("p1306-attacks.json")["actual_source_mutants"]
    assert not check_pins(after_candidate=True)
    workspace_before = state()
    directory = pathlib.Path(subprocess.check_output(["mktemp", "-d", "/dev/shm/p1306-mutants.XXXXXXXX"]).decode().strip())
    for part in ("01_core", "02_shell", "03_infra", "04_wiring", "benches", ".cargo"):
        shutil.copytree(ROOT / part, directory / part)
    for name in ("Cargo.toml", "Cargo.lock"):
        shutil.copy2(ROOT / name, directory / name)
    target = directory / "target"
    if seed_target:
        for part in ("debug",):
            if (pathlib.Path(seed_target) / part).exists():
                shutil.copytree(pathlib.Path(seed_target) / part, target / part, symlinks=True)
    env = {**os.environ, "CARGO_TARGET_DIR": str(target), "RUST_MIN_STACK": "33554432"}
    source = directory / OWNER
    original = source.read_text()
    original_sha = digest(source)
    frozen_tests = digest(directory / TESTS)
    source_snapshot = copied_sources(directory)
    control = command(["cargo", "test", "-p", "typst-core", "p1306", "--", "--test-threads=1"], directory, env)
    assert control["exit_code"] == 0 and re.search(r"test result: ok\. [1-9][0-9]* passed", control["stdout"]), control
    rows = []
    for attack in attacks:
        begin = time.monotonic()
        changed = mutate_text(original, attack)
        patch = apply_source(source, original, changed)
        changed_sha = digest(source)
        try:
            result = command(["cargo", "test", "-p", "typst-core", attack["witness_test"], "--", "--test-threads=1", "--nocapture"], directory, env)
            output = result["stdout"] + result["stderr"]
            ran = "running 1 test" in output and attack["witness_test"] in output
            assertion = bool(re.search(r"assertion .*failed|panicked at|expected [^\n]+", output))
            compile_error = "could not compile" in result["stderr"] or bool(re.search(r"^error\[E\d+\]", result["stderr"], re.MULTILINE))
            valid = ran and not compile_error
            verdict = "Violated" if valid and result["exit_code"] == 101 and assertion and "test result: FAILED" in output else "Preserved" if valid and result["exit_code"] == 0 else "Unknown"
        finally:
            restoration_patch = apply_source(source, changed, original)
        restored = digest(source) == original_sha and copied_sources(directory) == source_snapshot and digest(directory / TESTS) == frozen_tests
        row = {"id": attack["id"], "family": attack["family"], "attack": attack, "source_before_sha256": original_sha, "source_mutated_sha256": changed_sha, "source_restored_sha256": digest(source), "patch": patch, "restoration_patch": restoration_patch, "restored_all_sources": restored, "valid": valid, "verdict": verdict, "witness": result, "duration_seconds": time.monotonic()-begin}
        rows.append(row)
        print(json.dumps({"mutant": attack["id"], "valid": valid, "verdict": verdict, "restored": restored, "duration_seconds": row["duration_seconds"]}), flush=True)
    restored_control = command(["cargo", "test", "-p", "typst-core", "p1306", "--", "--test-threads=1"], directory, env)
    valid = sum(r["valid"] for r in rows)
    rejected = sum(r["valid"] and r["verdict"] == "Violated" for r in rows)
    save("p1306-mutant-ledger.json", {"manifest": pin(DIAG / "p1306-manifest.json"), "seal": pin(DIAG / "p1306-seal.json"), "attacks": pin(DIAG / "p1306-attacks.json"), "directory": str(directory), "target": str(target), "seed_target": seed_target, "environment": {key: env[key] for key in ("CARGO_TARGET_DIR", "RUST_MIN_STACK")}, "source_state_before": workspace_before, "source_state_after": state(), "candidate_source_sha256": digest(ROOT / OWNER), "candidate_tests_sha256": digest(ROOT / TESTS), "copied_source_hashes": source_snapshot, "unmutated_control": control, "mutants": rows, "restored_control": restored_control, "valid": valid, "rejected": rejected, "score": rejected/valid if valid else None, "required": len(attacks), "unknown": sum(r["verdict"] == "Unknown" for r in rows), "verdict": "Preserved" if valid == rejected == len(attacks) and all(r["restored_all_sources"] for r in rows) and restored_control["exit_code"] == 0 else "Violated"})


def certify():
    manifest = load("p1306-manifest.json")
    seal_data = load("p1306-seal.json")
    matrix_data = load("p1306-matrix.json")
    mutations = load("p1306-mutant-ledger.json")
    failures = check_pins(after_candidate=True)
    if seal_data["manifest"]["sha256"] != digest(DIAG / "p1306-manifest.json"):
        failures.append("Manifest changed after seal")
    if seal_data["verifier"]["sha256"] != digest(__file__):
        failures.append("Verifier changed after seal")
    if matrix_data["verdict"] != "Preserved" or mutations["verdict"] != "Preserved":
        failures.append("Matrix or compiled mutants failed")
    gates = load("p1306-gates.json")
    red, green, builds = load("p1306-red.json"), load("p1306-green.json"), load("p1306-build.json")
    red_valid = any(r["exit_code"] == 101 and "module `global`" in (r["stdout"]+r["stderr"]) and "module `std`" in (r["stdout"]+r["stderr"]) and "test result: FAILED" in (r["stdout"]+r["stderr"]) for r in red)
    if not red_valid:
        failures.append("Missing actual name-divergence RED assertion")
    if not green or any(r["exit_code"] != 0 for r in green):
        failures.append("GREEN failed")
    if not builds or any(r["exit_code"] != 0 for r in builds):
        failures.append("Fresh build failed")
    required = load("p1306-contract.json")["gates"]["final"]
    gate_checks = []
    all_receipts = gates + green + builds
    for text in required:
        tokens = text.split()
        matching = [r for r in all_receipts if r["argv"] == tokens]
        ok = bool(matching) and matching[-1]["exit_code"] == 0
        if tokens[:2] == ["cargo", "test"] and matching:
            ok = ok and bool(re.search(r"test result: ok\. [1-9][0-9]* passed", matching[-1]["stdout"]))
        gate_checks.append({"required_command": text, "passed": ok, "matches": len(matching)})
        if not ok:
            failures.append("Required gate missing/failed: "+text)
    # Read every new declared text artifact, including files invisible to git diff.
    whitespace = []
    for name in manifest["preflight"]["outputs_exact"]:
        path = DIAG / name
        if path.exists():
            result = command(["git", "diff", "--no-index", "--check", "/dev/null", str(path)])
            if result["stdout"] or result["stderr"]:
                whitespace.append(result)
    if whitespace:
        failures.append("New artifact whitespace check failed")
    source_paths = manifest["source_hashes_before_candidate"]
    changed = {path: {"before": sha, "after": digest(ROOT / path)} for path, sha in source_paths.items() if digest(ROOT / path) != sha}
    artifacts = ["p1306-manifest.json", "p1306-seal.json", "p1306-discriminator.json", "p1306-final-measurement.json", "p1306-matrix.json", "p1306-mutant-ledger.json", "p1306-build.json", "p1306-gates.json", "p1306-red.json", "p1306-green.json"]
    if (DIAG / "p1306-successor-seal.json").exists():
        artifacts.extend(["p1306-lineage-reopening.json", "p1306-successor-seal.json"])
    verdict = "P1306_PASS_NAMED_MODULE_DIAGNOSTIC_IDENTITY" if not failures else "P1306_BLOCKED"
    save("p1306-verification.json", {"verdict": verdict, "manifest": pin(DIAG / "p1306-manifest.json"), "seal": pin(DIAG / "p1306-seal.json"), "failures": failures, "gate_checks": gate_checks, "new_artifact_whitespace_failures": whitespace, "source_before_after": changed, "final_source_state": state(), "evidence": [pin(DIAG / name) for name in artifacts], "red_name_witness_valid": red_valid, "compiled_mutation_score": mutations["score"], "required_unknown": mutations["unknown"], "matrix_counts": matrix_data["counts"]})
    save("p1306-certificate.json", {"verdict": verdict, "regime": "executado sem atestacao de isolamento tecnico", "technical_isolation_attested": False, "manifest": pin(DIAG / "p1306-manifest.json"), "seal": pin(DIAG / "p1306-seal.json"), "verification": pin(DIAG / "p1306-verification.json"), "candidate_binary": load("p1306-final-measurement.json")["binaries"]["crystalline"], "candidate_source": pin(ROOT / OWNER), "baseline": pin(manifest["preflight"]["baseline_binary"]), "vanilla": pin(manifest["preflight"]["vanilla_binary"]), "vanilla_upstream": "a51e02804", "certified_fragment": "Named Module missing-field stored identity, precise primary message and resolved field span in frozen focal cases/four feature profiles; preservation of frozen lookup/import/repr and P1303 diagnostic controls; no whole-language or availability audit", "retained_debts": load("p1306-oracle-cases.json")["known_debts"], "out_of_scope": load("p1306-contract.json")["preservation"]["out_of_scope"], "failures": failures, "evidence": [pin(DIAG / name) for name in artifacts], "causal_dag": "Prompt/baseline -> manifest/contract/oracle -> discriminator -> preimplementation seal -> candidate RED/GREEN + focal matrix + real source mutants + gates -> verification -> certificate -> coordinator report. Certificate does not pin itself or downstream report."})


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["preseal", "seal", "successor", "matrix", "mutants", "certify"])
    parser.add_argument("--seed-target")
    args = parser.parse_args()
    if args.mode == "mutants":
        mutants(args.seed_target)
    else:
        {"preseal": preseal, "seal": seal, "successor": successor, "matrix": matrix, "certify": certify}[args.mode]()


if __name__ == "__main__":
    main()
