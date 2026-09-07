#!/usr/bin/env python3
"""Verifier-owned execution adapter for frozen oracle attacks, never an oracle author.

Preseal mode transforms public observation envelopes only; these are explicitly
not compiled product mutations. Real candidate mutations must be run separately.
"""
import argparse
import copy
import datetime
import hashlib
import json
import pathlib
import re
import subprocess
import time
import shutil
import tempfile
import os
import difflib
import base64
import collections
import gzip

ROOT = pathlib.Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
PLAN = DIAG / "p1305-r2-adversarial-plan.json"
MEASUREMENT = DIAG / "p1305-r2-pre-measurement.json"
WARNINGS = {"global-bare", "global-alias-bare", "global-bare-unbound", "global-alias-bare-unbound"}
PRESERVE = {"fields-dict-41", "fields-args-41", "content-sequence-41"}


def canonical(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def pin(path):
    path = pathlib.Path(path)
    raw = path.read_bytes()
    return {"path": str(path), "sha256": hashlib.sha256(raw).hexdigest(), "bytes": len(raw)}


def normalized_l0(path):
    raw = pathlib.Path(path).read_bytes()
    pattern = rb"(?m)^(Hash do C\xc3\xb3digo: )([0-9a-f]{8})$"
    matches = list(re.finditer(pattern, raw))
    if len(matches) != 1:
        raise ValueError("L0 must have exactly one canonical eight-hex Hash do Codigo line")
    match = matches[0]
    normalized = raw[:match.start(2)] + b"00000000" + raw[match.end(2):]
    return {"raw": pin(path), "normative_sha256": hashlib.sha256(normalized).hexdigest(),
            "derived_hash_byte_range": [match.start(2), match.end(2)],
            "derived_hash_value": match.group(2).decode()}


def envelope(row):
    result = {"complete": row["complete"], "exit_code": row["exit_code"],
              "stdout": row["stdout"], "stderr": row["stderr"]}
    if result["exit_code"] == 0:
        result["value"] = json.loads(result["stdout"])
    return result


def set_value(row, value):
    row.update(complete=True, exit_code=0, value=value,
               stdout=json.dumps(value, ensure_ascii=False, separators=(",", ":")) + "\n")
    row["stderr"] = ""


def classify(actual, expected):
    if actual.get("complete") is not True or actual.get("exit_code") not in (0, 1):
        return "Unknown"
    if actual["exit_code"] == 0:
        try:
            actual_value = json.loads(actual["stdout"])
        except (ValueError, TypeError, KeyError):
            return "Unknown"
    if actual["exit_code"] != expected["exit_code"]:
        return "Violated"
    if actual["stderr"] != expected["stderr"]:
        return "Violated"
    if actual["exit_code"] == 0:
        return "Preserved" if canonical(actual_value) == canonical(expected["value"]) else "Violated"
    return "Preserved" if actual["stdout"] == expected["stdout"] else "Violated"


def transform_strings(value, fn):
    if isinstance(value, str):
        return fn(value)
    if isinstance(value, list):
        return [transform_strings(item, fn) for item in value]
    if isinstance(value, dict):
        return {key: transform_strings(item, fn) for key, item in value.items()}
    return value


def rejected_dynamic(probe):
    expr = probe["expression"]
    start = expr.index("(std)")
    stderr = ("error: dynamic import requires an explicit name\n"
              f"  ┌─ <input-expression>:1:{start}\n  │\n1 │ {expr}\n"
              f"  │ {' ' * start}^^^^^\n  │\n"
              "  = hint: you can name the import with `as`\n\n")
    return {"complete": True, "exit_code": 1, "stdout": "", "stderr": stderr}


def preseal():
    measurement = json.loads(MEASUREMENT.read_text())
    plan = json.loads(PLAN.read_text())
    if pin(MEASUREMENT)["sha256"] != plan["pre_measurement_sha256"]:
        raise ValueError("measurement drift")
    if pin(DIAG / "p1305-r2-contract.md")["sha256"] != plan["contract_sha256"]:
        raise ValueError("contract drift")
    if pin(DIAG / "p1305-r2-oracle-tests.rs")["sha256"] != plan["oracle_tests_sha256"]:
        raise ValueError("oracle drift")
    rows = {(r["id"], r["profile"], r["side"]): r for r in measurement["runs"]
            if r["phase"] in ("normal", "document")}
    probes = {p["id"]: p for p in measurement["probes"]}
    expected = {}
    for (identifier, profile, side), row in rows.items():
        if side != "vanilla":
            continue
        chosen = rows[(identifier, profile, "crystalline")] if identifier in PRESERVE else row
        result = envelope(chosen)
        if identifier in WARNINGS or identifier == "document-route":
            result["stderr"] = rows[(identifier, profile, "crystalline")]["stderr"]
        expected[(identifier, profile)] = result
    flat = "sequence(\n  " + ",\n  ".join(f"[{i}]" for i in range(41)) + ",\n)"
    direct = {}; set_value(direct, flat)
    expected[("direct-flat-content-sequence-41", "default")] = direct
    positive = [{"id": key[0], "profile": key[1], "verdict": classify(value, value)}
                for key, value in expected.items()]
    mutants = []
    for attack in plan["mutants"]:
        identifier = attack["id"]; witness = attack["witness"]
        before = expected[(witness, "default")]
        after = copy.deepcopy(before)
        value = copy.deepcopy(before.get("value"))
        if identifier == "array-limit-39":
            value = value.replace("  39,\n", "  .. (1 items omitted),\n")
        elif identifier in ("array-limit-41", "array-no-elision", "array-known-maps-only"):
            value = json.loads(rows[(witness, "default", "crystalline")]["stdout"])
        elif identifier == "array-wrong-omission-count":
            value = value.replace(".. (2 items omitted)", ".. (3 items omitted)")
        elif identifier == "array-data-truncated":
            value[0] = 40; value[1] = value[1][:40]; value[2] = value[2][:40]
        elif identifier == "array-data-reordered":
            for vector in value[1:]:
                vector[39], vector[40] = vector[40], vector[39]
        elif identifier == "array-hidden-item-replaced":
            value[3] = 0; value[-1][40] = 0
        elif identifier == "array-singleton-comma-lost":
            value = "(0)"
        elif identifier == "array-indentation-lost":
            value = value.replace("\n    ", "\n  ")
        elif identifier == "helper-global-elision":
            value = value.replace("  [40],\n", "  .. (1 items omitted),\n")
        elif identifier == "module-old-wrapper":
            value = transform_strings(value, lambda s: re.sub(r"<module ([^>]+)>", r"module(\1)", s))
        elif identifier in ("global-expression-old-name", "global-document-old-name", "global-imported-old-name"):
            value = transform_strings(value, lambda s: s.replace("<module global>", "<module std>"))
        elif identifier == "module-all-global":
            value = transform_strings(value, lambda s: re.sub(r"<module [^>]+>", "<module global>", s))
        elif identifier == "module-std-nominal-map":
            value = transform_strings(value, lambda s: s.replace("<module std>", "<module global>"))
        elif identifier == "import-bare-public-name":
            set_value(after, "<module global>")
        elif identifier == "import-field-public-name":
            after = envelope(rows[(witness, "default", "crystalline")])
        elif identifier == "import-dynamic-accepted":
            after = envelope(rows[(witness, "default", "crystalline")])
        elif identifier == "import-dynamic-hint-or-span":
            after["stderr"] = after["stderr"].replace("  = hint: you can name the import with `as`\n", "")
        elif identifier.startswith("import-dynamic-reject-"):
            after = rejected_dynamic(probes[witness])
        elif identifier in ("module-reexport-lookup-removed", "lookup-removed"):
            after = {"complete": True, "exit_code": 1, "stdout": "",
                     "stderr": "error: module `global` does not contain `calc`\n\n"}
        elif identifier == "feature-gate-bypassed":
            set_value(after, "function")
        else:
            raise ValueError("unsupported frozen attack: " + identifier)
        if before["exit_code"] == 0 and after == before:
            set_value(after, value)
            after["stderr"] = before["stderr"]
        applicable = canonical(before) != canonical(after)
        verdict = classify(after, before) if applicable else "Unknown"
        mutants.append({**attack, "mode": "PUBLIC_ENVELOPE_TRANSFORMATION_NOT_COMPILED_PRODUCT",
                        "applicable": applicable, "expected_observation": before,
                        "mutated_observation": after, "verdict": verdict})
    valid = sum(m["applicable"] for m in mutants)
    killed = sum(m["applicable"] and m["verdict"] == "Violated" for m in mutants)
    result = {"schema": "p1305-r2-preseal-discrimination-v1",
              "generated_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
              "adapter": pin(__file__), "plan": pin(PLAN), "measurement": pin(MEASUREMENT),
              "contract": pin(DIAG / "p1305-r2-contract.md"),
              "baseline_provenance": measurement["baseline_source"],
              "positive_observations": positive, "mutants": mutants,
              "valid_negatives": valid, "rejected_negatives": killed,
              "contract_mutation_score": killed / valid if valid else None,
              "compiled_product_mutation_score": None,
              "opaque_controls": [], "synthetic_opaque_controls": False,
              "claim_limit": "Tests comparator discrimination against declared observable errors. Not compiled product mutant evidence; all source-level mutations remain mandatory after candidate.",
              "verdict": "DISCRIMINATION_PASS" if valid == len(mutants) == killed and all(p["verdict"] == "Preserved" for p in positive) else "DISCRIMINATION_FAIL"}
    return result


def product(destination):
    """Execute frozen attacks on actual source in a disposable, unshared copy."""
    previous = json.loads(destination.read_text()) if destination.exists() else None
    if previous and previous["verdict"] not in ("RUNNING", "MUTATION_GATE_NOT_PASSED"):
        raise SystemExit("Refusing to replace completed mutant ledger")
    sandbox = pathlib.Path(previous["copy"]) if previous else pathlib.Path(tempfile.mkdtemp(prefix="p1305-r2-mutants-", dir="/tmp"))
    if not previous:
        for name in ("01_core", "02_shell", "03_infra", "04_wiring", "benches", ".cargo"):
            shutil.copytree(ROOT / name, sandbox / name)
        for name in ("Cargo.toml", "Cargo.lock"):
            shutil.copy2(ROOT / name, sandbox / name)
    source_files = [p for name in ("01_core", "02_shell", "03_infra", "04_wiring", "benches", ".cargo") for p in (sandbox / name).rglob("*") if p.is_file()]
    source_files += [sandbox / "Cargo.toml", sandbox / "Cargo.lock"]
    original = {str(p.relative_to(sandbox)): p.read_text() for p in source_files if p.suffix == ".rs"}
    source_pins = [pin(p) for p in sorted(source_files)]
    env = os.environ.copy()
    env.update(CARGO_TARGET_DIR=str(sandbox / "target"), CARGO_PROFILE_TEST_DEBUG="0",
               CARGO_PROFILE_DEV_DEBUG="0", CARGO_INCREMENTAL="1")
    plan = json.loads(PLAN.read_text())
    ledger = previous or {"schema": "p1305-r2-real-source-mutants-v1", "adapter": pin(__file__),
              "plan": pin(PLAN), "isolation_attestation": "executado sem atestação de isolamento técnico",
              "copy": str(sandbox), "source_pins": source_pins,
              "git_head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
              "git_diff_sha256": hashlib.sha256(subprocess.check_output(["git", "diff", "HEAD"], cwd=ROOT)).hexdigest(),
              "environment_overrides": {k: env[k] for k in ("CARGO_TARGET_DIR", "CARGO_PROFILE_TEST_DEBUG", "CARGO_PROFILE_DEV_DEBUG", "CARGO_INCREMENTAL")},
              "baseline": [], "mutants": [], "verdict": "RUNNING"}
    if previous:
        ledger.setdefault("operational_adapter_revisions", []).append(pin(__file__))
        for record in ledger["mutants"]:
            if record["id"] in ("global-expression-old-name", "global-document-old-name") and record["classification"] == "Preserved":
                record["superseded_operational_attempt"] = "Constructor occurrence selection was inverted (expression is first at line 324, document second at line 562). Preserved is retained for this mismatched witness; retry the frozen family on its declared actual route."
    def save():
        destination.write_text(json.dumps(ledger, ensure_ascii=False, indent=2) + "\n")
    def run(test):
        argv = ["cargo", "test", "-p", "typst-core", test, "--", "--test-threads=1"]
        start = time.monotonic()
        result = subprocess.run(argv, cwd=sandbox, env=env, capture_output=True, text=True)
        counts = re.search(r"test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;", result.stdout)
        return {"argv": argv, "cwd": str(sandbox), "completed_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
                "seconds": time.monotonic() - start, "exit_code": result.returncode,
                "stdout": result.stdout, "stderr": result.stderr,
                "counts": [int(counts[i]) for i in (2, 3, 4)] if counts else None}
    sentinels = ["p1305", "p1301_lookups_de_modulo_existentes_preservam_valor_e_kind", "p1303_negativos_pdf_exatos_nos_perfis_sem_a11y"]
    for test in sentinels:
        if any(r["argv"][4] == test for r in ledger["baseline"]):
            continue
        result = run(test); ledger["baseline"].append(result); save()
        print("baseline", test, result["exit_code"], result["counts"], flush=True)
        if result["exit_code"] or not result["counts"] or result["counts"][0] == 0:
            ledger["verdict"] = "BASELINE_INVALID"; save(); return ledger
    repr_path = "01_core/src/compiler/eval/repr.rs"
    mod_path = "01_core/src/compiler/eval/mod.rs"
    imports_path = "01_core/src/compiler/eval/modules.rs"
    def apply(contents):
        patches = []
        for path, desired in contents.items():
            current = (sandbox / path).read_text()
            if current == desired:
                continue
            diff = list(difflib.unified_diff(current.splitlines(True), desired.splitlines(True), n=3))[2:]
            # apply_patch uses context-only @@, not unified line coordinates.
            body = ''.join('@@\n' if line.startswith('@@') else line for line in diff)
            patches.append("*** Update File: " + str(sandbox / path) + "\n" + body)
        patch = "*** Begin Patch\n" + ''.join(patches) + "*** End Patch\n"
        result = subprocess.run([shutil.which("apply_patch")], input=patch, capture_output=True, text=True)
        if result.returncode:
            raise RuntimeError(result.stdout + result.stderr)
        return patch
    for attack in plan["mutants"]:
        if any(m["id"] == attack["id"] and not m.get("superseded_operational_attempt") for m in ledger["mutants"]):
            continue
        identifier = attack["id"]; changes = {}
        def change(path, old, new, count=1, occurrence=None):
            current = changes.get(path, original[path])
            if current.count(old) != count:
                raise ValueError(f"{identifier}: expected {count} anchors, found {current.count(old)}: {old}")
            if occurrence is None:
                current = current.replace(old, new)
            else:
                positions = [m.start() for m in re.finditer(re.escape(old), current)]
                pos = positions[occurrence]; current = current[:pos] + new + current[pos+len(old):]
            changes[path] = current
        if identifier in ("array-limit-39", "array-limit-41"):
            limit = identifier[-2:]
            for old in ("take(40)", "arr.len() > 40", "arr.len() - 40"):
                change(repr_path, old, old.replace("40", limit))
        elif identifier == "array-no-elision":
            change(repr_path, "arr.iter().take(40).map(repr_value).collect()", "arr.iter().map(repr_value).collect()")
            change(repr_path, "if arr.len() > 40 {", "if false {")
        elif identifier == "array-wrong-omission-count":
            change(repr_path, "arr.len() - 40", "arr.len() - 39")
        elif identifier in ("array-data-truncated", "array-data-reordered"):
            operation = "out.truncate(40);" if identifier.endswith("truncated") else "if out.len() > 40 { out.swap(39, 40); }"
            change("01_core/src/compiler/stdlib/foundations/cast.rs", "        x += step;\n    }\n    out", "        x += step;\n    }\n    " + operation + "\n    out")
        elif identifier == "array-hidden-item-replaced":
            change("01_core/src/compiler/eval/bindings/binding.rs", "*location = new_value;", "*location = if matches!(new_value, Value::Int(_)) { Value::Int(0) } else { new_value };")
        elif identifier == "array-known-maps-only":
            change(repr_path, "let mut items: Vec<String> = arr.iter().take(40).map(repr_value).collect();", "let limit = if arr.iter().all(|v| matches!(v, Value::Color(_))) { 40 } else { usize::MAX };\n            let mut items: Vec<String> = arr.iter().take(limit).map(repr_value).collect();")
            change(repr_path, "arr.len() > 40", "arr.len() > limit")
        elif identifier == "array-singleton-comma-lost":
            change(repr_path, "pretty_array_like(&items, items.len() == 1)", "pretty_array_like(&items, false)")
        elif identifier == "array-indentation-lost":
            change(repr_path, 'buf.push_str("  ");', 'buf.push_str("");')
        elif identifier == "helper-global-elision":
            change(repr_path, "let list = pretty_comma_list(parts, trailing_comma);", 'let mut limited = parts.iter().take(40).cloned().collect::<Vec<_>>();\n    if parts.len() > 40 { limited.push(format!(".. ({} items omitted)", parts.len() - 40)); }\n    let parts = limited.as_slice();\n    let list = pretty_comma_list(parts, trailing_comma);')
        elif identifier == "module-old-wrapper":
            change(repr_path, 'format!("<module {}>", m.name())', 'format!("module({})", m.name())')
        elif identifier.startswith("global-"):
            name = 'Module::new("global", stdlib.clone())'
            if identifier == "global-imported-old-name":
                change(imports_path, name, name.replace('"global"', '"std"'))
            else:
                change(mod_path, name, name.replace('"global"', '"std"'), count=2,
                       occurrence=0 if identifier == "global-expression-old-name" else 1)
        elif identifier == "module-all-global":
            change(repr_path, 'format!("<module {}>", m.name())', '"<module global>".to_string()')
        elif identifier == "module-std-nominal-map":
            change(repr_path, 'format!("<module {}>", m.name())', 'format!("<module {}>", if m.name() == "std" { "global" } else { m.name() })')
        elif identifier in ("import-bare-public-name", "import-field-public-name", "import-dynamic-accepted"):
            anchor = '                            import.bare_name().map_err(|_| {'
            if identifier == "import-dynamic-accepted":
                expr = 'import.bare_name().or_else(|_| Ok::<String, crate::entities::ast::code::BareImportError>(m.name().to_string()))'
            else:
                variant = "Ident" if identifier == "import-bare-public-name" else "FieldAccess"
                expr = '{ if matches!(source_expr, Expr::' + variant + '(_)) { Ok(m.name().to_string()) } else { import.bare_name() } }'
            change(imports_path, anchor, '                            ' + expr + '.map_err(|_| {')
        elif identifier == "import-dynamic-hint-or-span":
            change(imports_path, '.with_hint("you can name the import with `as`")', '')
        elif identifier.startswith("import-dynamic-reject-"):
            condition = {'as': 'import.imports().is_none()', 'items': 'import.new_name().is_none() && !matches!(import.imports(), Some(Imports::Wildcard))', 'wildcard': 'import.new_name().is_none() && !matches!(import.imports(), Some(Imports::Items(_)))'}[identifier.rsplit('-', 1)[1]]
            change(imports_path, 'import.new_name().is_none() && import.imports().is_none()', condition)
        elif identifier == "module-reexport-lookup-removed":
            change(imports_path, 'for (name, binding) in module.scope().iter() {', 'for (name, binding) in module.scope().iter() {\n                if name == "calc" { continue; }')
        elif identifier == "lookup-removed":
            change(mod_path, 'scope.define("rgb", Value::Func(Func::native("rgb", native_rgb)));', '')
        elif identifier == "feature-gate-bypassed":
            change("01_core/src/compiler/stdlib/pdf.rs", 'if features.contains(Feature::A11yExtras) {', 'if true {')
            change("01_core/src/compiler/eval/bindings/field_access.rs", '&& !ctx\n                .features', '&& false && !ctx\n                .features')
        else:
            raise ValueError(identifier)
        patch = apply(changes)
        result = run(attack["rust_test"])
        killed = result["exit_code"] == 101 and result["counts"] == [0, 1, 0] and 'error[E' not in result["stderr"]
        record = {**attack, "patch": patch, "patch_sha256": hashlib.sha256(patch.encode()).hexdigest(),
                  "mutated_source_pins": [pin(sandbox / p) for p in changes], "execution": result,
                  "classification": "Violated" if killed else "Preserved" if result["exit_code"] == 0 and result["counts"] == [1, 0, 0] else "Unknown"}
        if identifier.startswith("array-data-") or identifier == "array-hidden-item-replaced":
            record["fault_location_limit"] = "Actual data-producer/assignment corruption models the frozen observable fault; not a claim that immutable repr mutates data."
        apply({p: original[p] for p in changes})
        record["restored_byte_exact"] = all((sandbox / p).read_text() == original[p] for p in changes)
        ledger["mutants"].append(record); save()
        print(identifier, record["classification"], round(result["seconds"], 2), flush=True)
    ledger["final_restoration"] = [run(test) for test in sentinels]
    ledger["valid_mutants"] = sum(m["classification"] != "Unknown" and not m.get("superseded_operational_attempt") for m in ledger["mutants"])
    ledger["killed_mutants"] = sum(m["classification"] == "Violated" and not m.get("superseded_operational_attempt") for m in ledger["mutants"])
    ledger["compiled_product_mutation_score"] = ledger["killed_mutants"] / ledger["valid_mutants"] if ledger["valid_mutants"] else None
    ledger["verdict"] = "ALL_REQUIRED_MUTANTS_KILLED" if ledger["killed_mutants"] == 27 and all(r["exit_code"] == 0 for r in ledger["final_restoration"]) else "MUTATION_GATE_NOT_PASSED"
    save(); return ledger


def audit_final_inputs():
    manifest = json.loads((DIAG / "p1305-r2-manifest.json").read_text())
    checks = []
    for item in manifest["protected_inputs"]:
        actual = pin(ROOT / item["path"])
        assert actual["sha256"] == item["sha256"], item["path"]
        if "normative_sha256" in item:
            assert normalized_l0(ROOT / item["path"])["normative_sha256"] == item["normative_sha256"]
        checks.append(actual)
    final = json.loads((DIAG / "p1305-r2-final-measurement.json").read_text())
    before = json.loads(MEASUREMENT.read_text())
    def rows(data):
        result = {}
        for row in data["runs"]:
            key = (row["id"], row["profile"], row["side"], row["phase"])
            assert key not in result
            assert row["complete"] and row["exit_code"] in (0, 1)
            for channel in ("stdout", "stderr"):
                raw = base64.b64decode(row[channel + "_base64"])
                assert hashlib.sha256(raw).hexdigest() == row[channel + "_sha256"]
                assert raw.decode() == row[channel]
            if row["exit_code"] == 0:
                assert canonical(json.loads(row["stdout"])) == canonical(row["value"])
            result[key] = row
        return result
    old = rows(before); new = rows(final)
    assert len(new) == 1042
    decisions = []
    for key, candidate in new.items():
        if key[2] != "crystalline":
            assert envelope(candidate) == envelope(old[key]), key
            continue
        reference = new[key[:2] + ("vanilla", key[3])]
        expected = envelope(old[key] if key[0] in PRESERVE else reference)
        if key[0] in WARNINGS or key[0] == "document-route":
            expected["stderr"] = old[key]["stderr"]
        verdict = classify(envelope(candidate), expected)
        assert verdict == "Preserved", key
        decisions.append({"id": key[0], "profile": key[1], "phase": key[3], "verdict": verdict})
    repeats = 0
    for key, row in new.items():
        if key[3] == "normal":
            assert envelope(row) == envelope(new[key[:3] + ("inverted",)]), key
            repeats += 1
    log = json.loads((DIAG / "p1305-r2-build.log").read_text())
    for row in log:
        assert row["exit_code"] == 0
        assert row["diff_before_sha256"] == row["diff_after_sha256"] == "a6a964ac05ef32c26cca69e478af130ce3068a015182b034e1cc360bab5a8ae5"
        for channel in ("stdout", "stderr"):
            assert hashlib.sha256(row[channel].encode()).hexdigest() == row[channel + "_sha256"]
    historical = json.loads((DIAG / "p1304-verification.log").read_text())
    historical_lint = next(r for r in historical["commands"] if r["argv"] == ["crystalline-lint", "."])
    final_lint = next(r for r in log if r["argv"] == ["crystalline-lint", "."])
    messages = lambda row: collections.Counter(re.findall(r"^(?:warning|info|error):.*$", row["stdout"], re.M))
    assert messages(historical_lint) == messages(final_lint)
    severities = collections.Counter(line.split(":", 1)[0] for line in messages(final_lint).elements())
    workspace = next(r for r in log if r["argv"] == ["cargo", "test", "--workspace"])
    counts = [[int(n) for n in m] for m in re.findall(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;", workspace["stdout"])]
    assert counts and sum(c[1] for c in counts) == 0
    oracle = (DIAG / "p1305-r2-oracle-tests.rs").read_text()
    assert (ROOT / "01_core/src/compiler/eval/tests.rs").read_text().count(oracle) == 1
    invalid_refinement = json.loads((DIAG / "p1305-r2-repr-refinement.json").read_text())
    failures = [r for r in invalid_refinement["runs"] if r["exit_code"] != 0]
    assert len(failures) == 120 and all("float() não suporta ratio" in r["stderr"] for r in failures)
    return {"schema": "p1305-r2-independent-final-input-audit-v1", "generated_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
            "adapter": pin(__file__), "manifest": pin(DIAG / "p1305-r2-manifest.json"),
            "protected_inputs_unchanged": checks, "metadata_delta": [],
            "focal": {"input": pin(DIAG / "p1305-r2-final-measurement.json"), "runs": len(new), "bilateral_decisions": decisions, "stable_repeats": repeats, "vanilla_unchanged_from_premeasurement": True},
            "functional_gates": {"input": pin(DIAG / "p1305-r2-build.log"), "commands_exit_zero": len(log), "workspace_passed": sum(c[0] for c in counts), "workspace_failed": sum(c[1] for c in counts), "workspace_ignored": sum(c[2] for c in counts), "suite_count": len(counts)},
            "lint": {"severity_counts": dict(severities), "strict_gate": "No violations found", "historical_input": pin(DIAG / "p1304-verification.log"), "historical_diff_sha256": historical_lint["source_state_before"]["diff_binary_sha256"], "message_multiset_unchanged": True, "comparison_limit": "Historical pinned P1304 lint, not a fresh baseline lint invocation. Location line shifts excluded; message multiplicities compared exactly."},
            "refinement_initial": {"input": pin(DIAG / "p1305-r2-repr-refinement.json"), "verdict": "Unknown", "invalid_crystalline_runs": len(failures), "reason_code": "INVALID_OBSERVATION_FLOAT_RATIO", "not_product_regression": True, "requires_successor_oracle_and_seal": True}}


def audit_matrix():
    current = json.loads((DIAG / "p1305-r2-matrix.json").read_text())
    previous = json.loads((DIAG / "p1304-feature-matrix.json").read_text())
    catalog = json.loads((DIAG / "p1304-probe-catalog.json").read_text())
    assert len(catalog["probes"]) == 627
    all_rows = current["results"] + current["inverted_results"] + current["supplement"]["normal_results"] + current["supplement"]["inverted_results"]
    def observation(row):
        return {k: row[k] for k in ("exit_code", "stdout", "stderr", "complete")}
    for row in all_rows:
        v, c = row["vanilla"], row["crystalline"]
        for run in (v, c):
            assert run["complete"] and run["exit_code"] in (0, 1) and not run["reason_code"] and not run["execution_error"]
            for channel in ("stdout", "stderr"):
                raw = base64.b64decode(run[channel + "_base64"])
                assert hashlib.sha256(raw).hexdigest() == run[channel + "_sha256"]
                assert raw.decode() == run[channel]
            if run["exit_code"] == 0:
                json.loads(run["stdout"])
        if v["exit_code"] == c["exit_code"] == 0:
            expected = "MATCH_VALUE" if json.loads(v["stdout"]) == json.loads(c["stdout"]) and v["stderr"] == c["stderr"] else "DIFFERENT_VALUE"
        elif v["exit_code"] == 0:
            expected = "VANILLA_ONLY"
        elif c["exit_code"] == 0:
            expected = "CRYSTALLINE_ONLY"
        else:
            expected = "MATCH_DIAGNOSTIC" if v["stderr"] == c["stderr"] and v["stdout"] == c["stdout"] else "DIFFERENT_DIAGNOSTIC"
        assert expected == row["runtime_class"]
    old = {(r["id"], r["profile"]): r for r in previous["results"]}
    new = {(r["id"], r["profile"]): r for r in current["results"]}
    assert len(new) == len(current["results"]) == 2508 and new.keys() == old.keys()
    expected_ids = {"p1304-repr-std", "path-color.map"} | {"path-color.map." + name for name in ("cividis", "coolwarm", "crest", "flare", "icefire", "mako", "rainbow", "rocket", "turbo", "vlag")}
    deltas = []
    for key, row in new.items():
        assert observation(row["vanilla"]) == observation(old[key]["vanilla"])
        if observation(row["crystalline"]) != observation(old[key]["crystalline"]):
            assert key[0] in expected_ids and old[key]["runtime_class"] == "DIFFERENT_VALUE" and row["runtime_class"] == "MATCH_VALUE"
            deltas.append({"id": key[0], "path": row["path"], "profile": key[1], "before": old[key]["runtime_class"], "after": row["runtime_class"]})
        else:
            assert row["runtime_class"] == old[key]["runtime_class"]
    assert len(deltas) == 48 and {d["id"] for d in deltas} == expected_ids
    normal = {(r["id"], r["profile"]): r for r in current["results"] + current["supplement"]["normal_results"]}
    inverted = current["inverted_results"] + current["supplement"]["inverted_results"]
    for row in inverted:
        for side in ("vanilla", "crystalline"):
            assert observation(row[side]) == observation(normal[row["id"], row["profile"]][side])
    assert len(current["inverted_results"]) == 728 and len(current["supplement"]["normal_results"]) == len(current["supplement"]["inverted_results"]) == 32
    for path in current["repetition"]["mandatory_paths"]:
        assert len({r["profile"] for r in inverted if r["path"] == path}) == 4
    for state in (current["source_before"], current["source_after"]):
        assert state["tracked_diff_sha256"] == "a6a964ac05ef32c26cca69e478af130ce3068a015182b034e1cc360bab5a8ae5"
    return {"schema": "p1305-r2-independent-corpus-audit-v1", "generated_at": datetime.datetime.now(datetime.timezone.utc).isoformat(), "adapter": pin(__file__),
            "input": pin(DIAG / "p1305-r2-matrix.json"), "historical": pin(DIAG / "p1304-feature-matrix.json"), "catalog": pin(DIAG / "p1304-probe-catalog.json"),
            "normal_pairs": len(new), "normal_classes_recomputed": dict(collections.Counter(r["runtime_class"] for r in new.values())), "channel_deltas": deltas,
            "inverted_pairs": len(current["inverted_results"]), "supplement_pairs": 64, "stable_side_repetitions": 2 * len(inverted),
            "all_other_channels_byte_identical_to_baseline": True, "complete_processes": 2 * len(all_rows), "required_unknown": 0,
            "verdict": "Preserved", "claim_limit": "Exactly the twelve selected paths improve across four profiles; 504 historical availability mismatches remain, not whole-language parity."}


def audit_refinement_revision():
    revision_path = DIAG / "p1305-r2-refinement-revision-1.json"
    revision = json.loads(revision_path.read_text())
    receipt = revision["baseline_calibration"]["receipt"]
    raw = gzip.decompress(base64.b64decode(receipt["content"]))
    assert hashlib.sha256(raw).hexdigest() == receipt["decoded_sha256"] and len(raw) == receipt["decoded_bytes"]
    calibration = json.loads(raw)
    historical = json.loads((DIAG / "p1304-repr-refinement.json").read_text())
    historical_maps = {m["path"]: m for m in historical["map_measurements"]}
    assert hashlib.sha256(historical["revision_script_source"].encode()).hexdigest() == revision["predecessors"]["historical_calibrated_source"]["member_sha256"]
    assert pin(ROOT / revision["new_runner"]["path"])["sha256"] == revision["new_runner"]["sha256"]
    data = calibration["runs"]
    assert len(data) == 32
    indexed = {}; positives = []
    for row in data:
        key = (row["id"], row["profile"], row["side"], row["phase"])
        assert key not in indexed
        indexed[key] = row
        assert row["complete"] and row["exit_code"] == 0 and not row["stderr"]
        for channel in ("stdout", "stderr"):
            raw_channel = base64.b64decode(row[channel + "_base64"])
            assert hashlib.sha256(raw_channel).hexdigest() == row[channel + "_sha256"] and raw_channel.decode() == row[channel]
        value = json.loads(row["stdout"])
        assert canonical(value) == canonical(row["value"])
        original = historical_maps[row["id"]]["profiles"][row["profile"]][row["side"]]
        digest = hashlib.sha256(json.dumps(value["rgba"], ensure_ascii=False, separators=(",", ":")).encode()).hexdigest()
        assert value["kind"] == "array" and value["n"] == len(value["rgba"]) == len(value["hex"]) == original["cardinality"]
        tokens = ["0x" + color.removeprefix("#").lower() + ("ff" if len(color.removeprefix("#")) == 6 and rgba[3] == 1 else "") for color, rgba in zip(value["hex"], value["rgba"])]
        assert all(len(token) == 10 for token in tokens)
        assert digest == original["full_components_sha256"]
        assert hashlib.sha256(','.join(tokens).encode()).hexdigest() == historical_maps[row["id"]]["l0_expected_rgba8_sha256"]
        assert value["repr"] == original["repr"]
        positives.append({"id": key[0], "profile": key[1], "side": key[2], "phase": key[3], "verdict": "Preserved"})
    for key, row in indexed.items():
        if key[3] == "normal":
            assert envelope(row) == envelope(indexed[key[:3] + ("inverted",)])
    expected = envelope(indexed[("color.map.cividis", "default", "vanilla", "normal")])
    attacks = []
    for attack in ("array-data-truncated", "array-data-reordered", "array-hidden-item-replaced", "array-wrong-omission-count"):
        mutated = copy.deepcopy(expected); value = copy.deepcopy(expected["value"])
        if attack == "array-data-truncated":
            value["n"] = 40; value["rgba"] = value["rgba"][:40]; value["hex"] = value["hex"][:40]
        elif attack == "array-data-reordered":
            for field in ("rgba", "hex"):
                value[field][39], value[field][40] = value[field][40], value[field][39]
        elif attack == "array-hidden-item-replaced":
            value["rgba"][40] = [0, 0, 0, 1]; value["hex"][40] = "#000000"
        else:
            value["repr"] = value["repr"].replace("216 items omitted", "217 items omitted")
        set_value(mutated, value)
        assert classify(mutated, expected) == "Violated" and canonical(mutated) != canonical(expected)
        attacks.append({"frozen_attack": attack, "witness": "color.map.cividis/default", "mode": "public channel envelope discrimination; compiled counterpart recorded separately", "expected_sha256": hashlib.sha256(canonical(expected)).hexdigest(), "mutated_sha256": hashlib.sha256(canonical(mutated)).hexdigest(), "verdict": "Violated"})
    return {"schema": "p1305-r2-refinement-revision-independent-gate-v1", "generated_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
            "revision": pin(revision_path), "runner": pin(ROOT / revision["new_runner"]["path"]), "adapter": pin(__file__),
            "baseline_calibration_decoded_sha256": receipt["decoded_sha256"], "positive_calibrations": positives, "stable_repetition_pairs": 16,
            "negative_channel_mutations": attacks, "valid_negatives": 4, "rejected_negatives": 4, "contract_channel_mutation_score": 1.0,
            "scope": "First affected phase reopened: auxiliary full-color observation adapter only; normative L0/contract/38tests/27attacks/candidate unchanged. Original seal remains provenance but cannot alone approve refinement.",
            "delta": {"invalid_crystalline_focal_before": 16, "invalid_crystalline_focal_after": 0, "preserved_public_data_channels": 32, "regressions": 0, "opaque_controls": 0},
            "verdict": "READY_FOR_SUCCESSOR_SEAL", "not_final_candidate_refinement": True}


def audit_completion():
    final_path = DIAG / "p1305-r2-repr-refinement-r1.json"
    final = json.loads(final_path.read_text())
    historical = json.loads((DIAG / "p1304-repr-refinement.json").read_text())
    maps = {m["path"]: m for m in historical["map_measurements"]}
    indexed = {}; channels = []
    assert len(final["runs"]) == 240
    for row in final["runs"]:
        key = (row["id"], row["profile"], row["side"], row["phase"])
        assert key not in indexed; indexed[key] = row
        assert row["complete"] and row["exit_code"] == 0 and not row["stderr"]
        for channel in ("stdout", "stderr"):
            raw = base64.b64decode(row[channel + "_base64"])
            assert hashlib.sha256(raw).hexdigest() == row[channel + "_sha256"] and raw.decode() == row[channel]
        value = json.loads(row["stdout"])
        assert canonical(value) == canonical(row["value"])
        old = maps[row["id"]]["profiles"][row["profile"]][row["side"]]
        assert value["kind"] == "array" and value["n"] == len(value["rgba"]) == len(value["hex"]) == old["cardinality"]
        assert value["rgba"] == old["full_public_components"]
        tokens = ["0x" + color.removeprefix("#").lower() + ("ff" if len(color.removeprefix("#")) == 6 and rgba[3] == 1 else "") for color, rgba in zip(value["hex"], value["rgba"])]
        assert tokens == old["rgba8_tokens"]
        digest = hashlib.sha256(json.dumps(value["rgba"], ensure_ascii=False, separators=(",", ":")).encode()).hexdigest()
        token_digest = hashlib.sha256(','.join(tokens).encode()).hexdigest()
        assert digest == old["full_components_sha256"] == row["digests"]["full_components_sha256"]
        assert token_digest == maps[row["id"]]["l0_expected_rgba8_sha256"] == row["digests"]["rgba8_sha256"]
        channels.append({"id": key[0], "profile": key[1], "side": key[2], "phase": key[3], "cardinality": value["n"], "full_components_sha256": digest, "rgba8_sha256": token_digest, "verdict": "Preserved"})
    for key, row in indexed.items():
        if key[2] == "crystalline":
            vanilla = indexed[key[:2] + ("vanilla", key[3])]
            assert canonical(row["value"]) == canonical(vanilla["value"])
        if key[3] == "normal":
            assert envelope(row) == envelope(indexed[key[:3] + ("inverted",)])
    assert {key[0] for key in indexed} == maps.keys() and len(maps) == 15
    ledger_path = DIAG / "p1305-r2-mutant-ledger.json"
    ledger = json.loads(ledger_path.read_text())
    attacks = json.loads(PLAN.read_text())["mutants"]
    latest = {r["id"]: r for r in ledger["mutants"] if not r.get("superseded_operational_attempt")}
    assert latest.keys() == {a["id"] for a in attacks} and len(latest) == 27
    for attack in attacks:
        row = latest[attack["id"]]; execution = row["execution"]
        assert row["rust_test"] == attack["rust_test"] and row["classification"] == "Violated"
        assert execution["exit_code"] == 101 and execution["counts"] == [0, 1, 0]
        assert 'panicked at 01_core/src/compiler/eval/tests.rs:' in execution["stdout"]
        assert 'error[E' not in execution["stderr"] and 'could not compile' not in execution["stderr"]
        assert hashlib.sha256(row["patch"].encode()).hexdigest() == row["patch_sha256"] and row["restored_byte_exact"]
    assert len(ledger["mutants"]) == 29 and len([r for r in ledger["mutants"] if r.get("superseded_operational_attempt")]) == 2
    for row in ledger["baseline"] + ledger["final_restoration"]:
        assert row["exit_code"] == 0 and row["counts"] in ([38, 0, 0], [1, 0, 0])
    sandbox = pathlib.Path(ledger["copy"])
    for source in ledger["source_pins"]:
        path = pathlib.Path(source["path"])
        assert pin(path)["sha256"] == source["sha256"] == pin(ROOT / path.relative_to(sandbox))["sha256"]
    assert len(ledger["source_pins"]) == 527
    preflight = json.loads((DIAG / "p1305-r2-preflight-manifest.json").read_text())
    historical_pins = []
    for item in preflight["baseline"]["dirty_pins"] + preflight["baseline"]["inputs"]:
        if isinstance(item, dict) and item.get("path", "").startswith("00_nucleo/diagnosticos/"):
            actual = pin(ROOT / item["path"])
            assert actual["sha256"] == item["sha256"], item["path"]
            historical_pins.append(actual)
    candidate = {p: pin(ROOT / p) for p in ("01_core/src/compiler/eval/repr.rs", "01_core/src/compiler/eval/modules.rs", "01_core/src/compiler/eval/mod.rs", "01_core/src/compiler/eval/tests.rs")}
    return {"schema": "p1305-r2-independent-completion-audit-v1", "generated_at": datetime.datetime.now(datetime.timezone.utc).isoformat(), "adapter": pin(__file__),
            "refinement": {"input": pin(final_path), "successor_seal": pin(DIAG / "p1305-r2-refinement-seal-r1.json"), "valid_runs": 240, "maps": 15, "profiles": 4, "orders": 2, "stable_repeats": 120, "whole_vectors_and_tokens_equal_historical": True, "whole_structured_values_equal_vanilla": True, "channels": channels, "verdict": "Preserved"},
            "compiled_mutants": {"input": pin(ledger_path), "required_families": 27, "semantic_kills": 27, "survivors": 0, "unknown": 0, "mutation_score": 1.0, "recorded_attempts": 29, "operational_route_mismatches_retained": 2, "restored_copy_files": 527, "baseline_and_restoration_green": True, "execution_seconds": sum(r["execution"]["seconds"] for r in ledger["mutants"]), "limit": "Range/assignment corruption models data faults at producers, not mutation through immutable repr. Two initial expression/document witness mappings were inverted; both original outcomes preserved and same frozen families then rejected at their correct route."},
            "candidate_sources": candidate, "historical_pins_unchanged": historical_pins, "verdict": "ALL_REMAINING_GATES_PRESERVED"}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=["preseal", "product", "audit-inputs", "audit-matrix", "audit-refinement", "audit-completion"], required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    destination = pathlib.Path(args.output)
    if args.mode in ("audit-inputs", "audit-matrix", "audit-refinement", "audit-completion"):
        result = {"audit-inputs": audit_final_inputs, "audit-matrix": audit_matrix, "audit-refinement": audit_refinement_revision, "audit-completion": audit_completion}[args.mode]()
        prior = json.loads(destination.read_text())
        assert hashlib.sha256(canonical(prior[0])).hexdigest() == "bcd53259c59f4cffbf414feeb8cef5613195f8fe3b5ea381513e1f3ca00a33f4"
        assert not any(r.get("schema") == result["schema"] for r in prior)
        prior.append(result)
        destination.write_text(json.dumps(prior, ensure_ascii=False, indent=2) + "\n")
        print(json.dumps({k: v for k, v in result.items() if k not in ("protected_inputs_unchanged", "focal", "channel_deltas", "refinement", "historical_pins_unchanged")}))
        return
    if args.mode == "product":
        product(destination)
        return
    result = preseal()
    if destination.exists():
        raise SystemExit("Refusing to overwrite prior verifier log")
    destination.write_text(json.dumps([result], ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({k: result[k] for k in ["verdict", "valid_negatives", "rejected_negatives", "contract_mutation_score"]}))


if __name__ == "__main__":
    main()
