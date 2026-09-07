#!/usr/bin/env python3
"""Independent public-CLI P1307-R4 oracle. Expected never comes from a candidate."""
import argparse
import base64
import concurrent.futures
import datetime
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "00_nucleo/diagnosticos"
PROFILES = {"default": [], "html": ["--features", "html"], "a11y": ["--features", "a11y-extras"], "html+a11y": ["--features", "html,a11y-extras"]}
BASELINE_PIN = "52df1c661c20d9eb612bbd5c89ae3cae8c44735aeab27c11e4a1da0d4d145e57"
MARKER = "P1307R4:"

def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def read(name):
    return json.loads((OUT / name).read_text())

def save(name, obj):
    (OUT / name).write_text(json.dumps(obj, ensure_ascii=False, indent=2) + "\n")

def source_hash(source):
    return hashlib.sha256(source.encode()).hexdigest()

def provenance():
    return {"at": now(), "cwd": str(ROOT), "baseline_sha256": sha(OUT / "p1307-r4-baseline.json"), "preflight_sha256": sha(OUT / "p1307-r4-preflight.json"), "script_sha256": sha(__file__), "head": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True, cwd=ROOT).strip(), "diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], text=True, cwd=ROOT), "environment": {"NO_COLOR": "1", "TERM": "dumb", "TYPST_FEATURES": "unset; explicit CLI profile"}}

def binaries():
    assert sha(OUT / "p1307-r4-baseline.json") == BASELINE_PIN
    result = read("p1307-r4-baseline.json")["binaries"]
    for info in result.values():
        assert sha(info["path"]) == info["sha256"], "binary identity changed"
    return {"vanilla": result["vanilla"], "baseline": result["crystalline"]}

def transport(expression):
    body = '{ let raw = bytes(' + expression + '); panic("' + MARKER + '" + range(raw.len()).map(i => str(raw.at(i))).join(",") + ":END") }'
    return '= Probe <probe>\n#context { let found = query(<probe>); if found.len() > 0 { ' + body + ' } }\n'

def corpus():
    cases = []
    def add(key, expr, classes, shape="value", policy="vanilla", contextual=False):
        c = {"id": key, "expression": expr, "classes": classes.split(), "expected_shape": shape, "policy": policy, "route": "compile" if contextual else "eval", "mandatory": True}
        if contextual:
            c["document"] = transport(expr)
        c["source_sha256"] = source_hash(c.get("document", expr))
        cases.append(c)
    mixed = 'arguments(..arguments(z: 11), 22, ..arguments(z: 33, a: 44), 55)'
    for key, expr in {
        "constructor": 'repr(arguments(1, key: 2))',
        "mixed-duplicates": f'repr({mixed})',
        "projections": f'{{ let a = {mixed}; (repr(a.pos()), repr(a.named()), a.len(), a.at("z")) }}',
        "map-identity": f'repr({mixed}.map(v => v))',
        "map-value": f'repr({mixed}.map(v => v + 1))',
        "filter-all": f'repr({mixed}.filter(v => true))',
        "filter-first-duplicate": f'repr({mixed}.filter(v => v != 11))',
        "filter-last-duplicate": f'repr({mixed}.filter(v => v != 33))',
        "filter-empty": f'repr({mixed}.filter(v => false))',
        "map-static": f'repr(arguments.map({mixed}, v => v + 1))',
        "filter-static": f'repr(arguments.filter({mixed}, v => v != 11))',
        "sink-only": 'repr(((..a) => a)(1, key: 2))',
        "sink-mixed": f'repr(((..a) => a)(..{mixed}))',
        "sink-consumed-named": f'repr(((head, z: 0, ..rest) => rest)(..{mixed}))',
        "sink-before-positional": 'repr(((first, ..rest, last) => rest)(1, a: 2, 3, b: 4, 5))',
        "sink-after-positional": 'repr(((first, last, ..rest) => rest)(1, a: 2, 3, b: 4, 5))',
        "sink-default-absent": 'repr(((first, z: 99, ..rest) => rest)(1, a: 2, 3))',
        "join-collision": 'repr(arguments(1, z: 11, a: 12) + arguments(z: 21, b: 22, 2))',
        "join-duplicates": f'repr({mixed} + arguments(..arguments(z: 61), z: 62, 77))',
        "join-none": f'repr({mixed} + none)',
    }.items():
        add("args." + key, expr, "Args order occurrences " + key)
        if key == "sink-before-positional":
            cases[-1].update(policy="baseline", scope_out="Nonterminal sink positional binding is an existing ClosureRepr limitation, not a new approved public field; retain this paired constructed debt witness.")
    for key, expr in {
        "map-first-callback": f'{mixed}.map(v => panic(str(v)))',
        "filter-first-callback": f'{mixed}.filter(v => panic(str(v)))',
        "filter-non-bool": f'{mixed}.filter(v => 1)',
        "filter-preserve-arg-span": 'yaml.encode((:), ..arguments(nope: 1).filter(v => true))',
        "map-preserve-arg-span": 'yaml.encode((:), ..arguments(nope: 1).map(v => v))',
        "filter-preserve-value-span": 'json.encode((:), ..arguments(pretty: "bad").filter(v => true))',
        "map-detached-value-span": 'json.encode((:), ..arguments(pretty: "bad").map(v => v))',
        "map-changed-detached-value-span": 'json.encode((:), ..arguments(pretty: true).map(v => "bad"))',
        "join-retains-rhs-span": 'yaml.encode((:), ..(arguments(nope: 1) + arguments(nope: 2)))',
        "join-retains-lhs-span": 'yaml.encode((:), ..(arguments(nope: 1) + arguments(ok: 2)))',
        "with-keeps-invalid-earlier": 'json.encode.with(..arguments(pretty: "bad"))((:), pretty: true)',
    }.items():
        add("error." + key, expr, "Args diagnostic span " + key, "diagnostic")
        cases[-1]["required_message"] = ("panicked with: 11" if key.endswith("first-callback") else "expected boolean, found integer" if key == "filter-non-bool" else "expected boolean, found string" if "value-span" in key or key == "with-keeps-invalid-earlier" else "unexpected argument: nope")
    add("join.removes-invalid-earlier", 'json.encode((:), ..(arguments(pretty: "bad") + arguments(pretty: true)))', "Args join named-cast")
    for final in ("f", "g"):
        for method in ("filter(v => true)", "map(v => v)"):
            expr = '{\n let make(which) = {\n  let a = arguments(nope: 1);\n  let b = arguments(nope: 1);\n  yaml.encode.with(..(if which { a } else { b }).' + method + ')\n };\n let f = make(true);\n let g = make(false);\n ' + final + '((:))\n}'
            add(f"origin.{method.split('(')[0]}.{final}", expr, "Args origin factory diagnostic", "diagnostic")
            cases[-1]["required_message"] = "unexpected argument: nope"
    for name in ("json", "cbor.encode", "repr", "calc.abs", "text", "(x => x)"):
        add("with.repr." + name, f'repr({name}.with())', "With repr generic")
    for key, value in {
        "State": 'state("p1307-r4", 0)',
        "State-escaping": 'state("a\\\"b", (z: "x\\ny", a: (1, 2)))',
        "Counter-Page": "counter(page)", "Counter-Str": 'counter("page")',
        "Counter-Selector": "counter(heading)", "With-native": "calc.abs.with()",
        "With-closure": "(x => x).with()", "Args": mixed,
    }.items():
        add("repr." + key, f"repr({value})", "repr " + key)
        add("cbor.delta." + key, '{ let raw = cbor.encode(' + value + '); range(raw.len()).map(i => raw.at(i)) }', "CBOR repr-delta " + key)
    add("repr.Location", 'repr(found.first().location())', "Location repr context", contextual=True)
    add("cbor.delta.Location", '{ let raw = cbor.encode(found.first().location()); repr(range(raw.len()).map(i => raw.at(i))) }', "CBOR Location context", contextual=True)
    add("context.calibration", '"α\\n\\\"x\\\""', "context calibration", contextual=True)
    # These controls deliberately preserve baseline debts, not vanilla parity.
    for key, value in {"Symbol": "sym.alpha", "Content": "[hi]", "Bytes": "bytes((0,255,10))", "Dict": "(z:1,a:2)", "negative-zero": "-0.0"}.items():
        add("cbor.control." + key, '{ let raw = cbor.encode(' + value + '); range(raw.len()).map(i => raw.at(i)) }', "CBOR preservation " + key, policy="baseline")
    return cases

def fixtures(cases):
    folder = Path(tempfile.mkdtemp(prefix="p1307-r4-oracle-", dir="/tmp"))
    patches = []
    for i, c in enumerate(cases):
        if c["route"] != "compile":
            continue
        path = folder / f"case-{i:03d}.typ"
        c["fixture"] = str(path)
        c["pdf_output"] = str(path.with_suffix(".pdf"))
        patches.append(f"*** Add File: {path}\n" + "".join("+" + line + "\n" for line in c["document"].splitlines()))
    if patches:
        subprocess.run(["apply_patch"], input="*** Begin Patch\n" + "".join(patches) + "*** End Patch\n", text=True, capture_output=True, check=True, cwd=ROOT)
    return str(folder)

def canonical_stderr(stderr, case):
    # Only an exact fixture identity substitution. All messages, locations,
    # displayed source, whitespace, carets, hints and traces remain literal.
    if case.get("fixture"):
        identities = sorted({case["fixture"], os.path.relpath(case["fixture"], ROOT)}, key=len, reverse=True)
        for identity in identities:
            stderr = stderr.replace(identity, "<source:" + case["source_sha256"] + ">")
    return stderr

def envelope(returncode, stdout, stderr, case):
    stderr = canonical_stderr(stderr, case)
    if returncode == 0 and case["route"] == "eval":
        try:
            value = json.loads(stdout)
        except (TypeError, ValueError):
            return {"kind": "Unknown", "reason": "OUTPUT_NOT_JSON"}
        if stderr:
            return {"kind": "Unknown", "reason": "SUCCESS_SIDE_DIAGNOSTIC", "stderr": stderr}
        return {"kind": "value", "value": value, "stderr": "", "exit": 0}
    if returncode == 1 and case["route"] == "compile":
        marker = re.escape(case.get("transport_marker", MARKER))
        found = re.findall(r"^error: panicked with: " + marker + r"([0-9,]*):END$", stderr, re.M)
        if found and len(re.findall(r"^error:", stderr, re.M)) == 1 and not re.search(r"^warning:", stderr, re.M):
            try:
                value = bytes(int(x) for x in found[0].split(",") if x).decode("utf-8")
                return {"kind": "value", "value": value, "stderr": "", "exit": 0}
            except (ValueError, UnicodeError):
                return {"kind": "Unknown", "reason": "INVALID_BYTE_TRANSPORT"}
    if returncode == 1 and re.search(r"^error:", stderr, re.M):
        locations = re.findall(r"[┌└]─ ([^\n]+):(\d+):(\d+)", stderr)
        allowed = {"<input-expression>", "<source:" + case["source_sha256"] + ">"}
        if any(p not in allowed for p, _, _ in locations):
            return {"kind": "Unknown", "reason": "DIAGNOSTIC_SOURCE_UNRESOLVED", "stderr": stderr}
        return {"kind": "diagnostic", "exit": returncode, "stdout": stdout, "stderr": stderr, "messages": re.findall(r"^error: (.*)$", stderr, re.M), "hints": re.findall(r"^\s*= hint: (.*)$", stderr, re.M)}
    return {"kind": "Unknown", "reason": "CLI_OR_CONTEXT_EXECUTION_FAILURE", "exit": returncode, "stderr": stderr}

def classify(expected, observed):
    """Full public envelope classifier; malformed/missing observation is Unknown."""
    fields = {"value": {"kind", "value", "stderr", "exit"}, "diagnostic": {"kind", "exit", "stdout", "stderr", "messages", "hints"}}
    for obj in (expected, observed):
        if not isinstance(obj, dict) or obj.get("kind") not in fields or not fields[obj["kind"]] <= obj.keys():
            return "Unknown"
    return "Preserved" if expected == observed else "Violated"

def run(binary, side, case, profile):
    assert source_hash(case.get("document", case["expression"])) == case["source_sha256"], "case source changed"
    argv = [binary, case["route"]]
    if case["route"] == "eval":
        argv += [case["expression"], "--format", "json"]
    else:
        assert sha(case["fixture"]) == case["source_sha256"]
        argv += [case["fixture"], case["pdf_output"], "--format", "pdf"]
    argv += PROFILES[profile]
    env = {k: v for k, v in os.environ.items() if k != "TYPST_FEATURES"}
    env.update(NO_COLOR="1", TERM="dumb")
    start, clock = now(), time.monotonic()
    row = {"case": case["id"], "profile": profile, "side": side, "argv": argv, "cwd": str(ROOT), "started": start, "source_sha256": case["source_sha256"]}
    try:
        p = subprocess.run(argv, cwd=ROOT, env=env, capture_output=True, timeout=25)
        stdout, stderr = p.stdout.decode("utf-8"), p.stderr.decode("utf-8")
        row.update(returncode=p.returncode, stdout=stdout, stderr=stderr, observable=envelope(p.returncode, stdout, stderr, case))
    except (subprocess.TimeoutExpired, OSError, UnicodeError) as exc:
        row.update(observable={"kind": "Unknown", "reason": type(exc).__name__}, exception=str(exc))
    row["elapsed_seconds"] = time.monotonic() - clock
    return row

def measure(cases, profiles):
    bins = binaries()
    jobs = [(b["path"], side, c, p) for c in cases for p in profiles for side, b in bins.items()]
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        return list(pool.map(lambda x: run(*x), jobs))

def issues(cases, rows):
    lookup = {c["id"]: c for c in cases}
    return [{"case": r["case"], "side": r["side"], "profile": r["profile"], "reason": r["observable"].get("reason", "VANILLA_FIXTURE_PRECONDITION_MISMATCH")} for r in rows if r["observable"]["kind"] == "Unknown" or (r["side"] == "vanilla" and (r["observable"]["kind"] != lookup[r["case"]]["expected_shape"] or (lookup[r["case"]].get("required_message") and r["observable"].get("messages") != [lookup[r["case"]]["required_message"]])))]

def focal(args):
    path = OUT / "p1307-r4-measurement.json"
    data = read(path.name) if path.exists() else {"schema": "p1307-r4-independent-measurement-v1", "attempts": []}
    assert len(data["attempts"]) < 2, "focal revision budget exhausted"
    selected = [c for c in corpus() if c["id"] in {"args.constructor", "args.mixed-duplicates", "args.map-identity", "args.filter-all", "args.join-duplicates", "args.sink-only", "args.sink-mixed", "args.sink-consumed-named", "args.sink-before-positional", "args.sink-after-positional", "args.sink-default-absent", "error.map-detached-value-span", "with.repr.calc.abs", "repr.Location", "cbor.delta.Location", "context.calibration"}]
    folder = fixtures(selected)
    rows = measure(selected, ["default"])
    data["attempts"].append({"attempt": len(data["attempts"]) + 1, "hypothesis": "Existing eval and named-file compile decimal-byte transport observes ordered Args transformations, legitimate detached errors and contextual CBOR without new CLI; sink ordering measured, not assumed.", "provenance": provenance(), "fixture_directory": folder, "cases": selected, "rows": rows, "issues": issues(selected, rows), "finished": now()})
    save(path.name, data)
    print(json.dumps({"runs": len(rows), "issues": data["attempts"][-1]["issues"], "observations": [{k: r[k] for k in ("case", "side", "observable")} for r in rows]}, ensure_ascii=False), flush=True)

def full(args):
    data = read("p1307-r4-measurement.json")
    assert data["attempts"] and not data["attempts"][-1]["issues"]
    assert "matrix" not in data
    cases = corpus()
    folder = fixtures(cases)
    # First the still-new cases on default; do not launch full orders on a bad fixture.
    old_ids = {c["id"] for c in data["attempts"][-1]["cases"]}
    new_cases = [c for c in cases if c["id"] not in old_ids]
    previous = data.get("extension_focal")
    if previous:
        assert previous["issues"] and not data.get("extension_focal_history"), "extension focal revision budget exhausted"
        data["extension_focal_history"] = [previous]
        seen = {c["id"] for c in previous["cases"]}
        failed = {i["case"] for i in previous["issues"]}
        affected = [c for c in new_cases if c["id"] not in seen or c["id"] in failed or c["id"] == "with.repr.json"]
        retry = measure(affected, ["default"])
        retried = {c["id"] for c in affected}
        valid_ids = {c["id"] for c in new_cases}
        rows = [r for r in previous["rows"] if r["case"] in valid_ids and r["case"] not in retried] + retry
        data["extension_revision"] = {"hypothesis": "str is Type and has no with. Use native repr.with as the intended additional native-function witness; retain invalid str attempt, rerun only affected plus native json control.", "cases": affected, "rows": retry, "finished": now()}
    else:
        rows = measure(new_cases, ["default"])
    data["extension_focal"] = {"cases": new_cases, "rows": rows, "issues": issues(new_cases, rows), "finished": now(), "provenance": provenance()}
    save("p1307-r4-measurement.json", data)
    if data["extension_focal"]["issues"]:
        print(json.dumps(data["extension_focal"]["issues"]), flush=True)
        return
    orders = {}
    for name, ordered in (("normal", cases), ("repeat", cases), ("reverse", list(reversed(cases)))):
        print(f"{name}: {len(cases)} cases x 4 profiles x 2 binaries", flush=True)
        orders[name] = measure(ordered, list(PROFILES))
    first = {(r["case"], r["side"], r["profile"]): r for r in orders["normal"]}
    instability = [{"order": o, "case": r["case"], "side": r["side"], "profile": r["profile"]} for o, rs in orders.items() for r in rs if any(r.get(k) != first[r["case"],r["side"],r["profile"]].get(k) for k in ("returncode", "stdout", "stderr", "observable"))]
    data["matrix"] = {"provenance": provenance(), "fixture_directory": folder, "cases": cases, "orders": orders, "issues": {o: issues(cases, rs) for o, rs in orders.items()}, "instability": instability, "finished": now()}
    save("p1307-r4-measurement.json", data)
    print(json.dumps({"runs": sum(map(len, orders.values())), "issues": data["matrix"]["issues"], "instability": instability}), flush=True)

def cell(v, b, policy):
    return {"vanilla_literal": v, "baseline_literal": b, "future_expected": b if policy == "baseline" else v, "expected_side": "baseline" if policy == "baseline" else "vanilla", "baseline_comparison": classify(v, b), "expectation_policy": "preserve-existing-baseline-debt-or-control" if policy == "baseline" else "ratified-vanilla-literal"}

def consolidate():
    """Reuse frozen raw observations; never run candidate or generate candidate expected."""
    data = read("p1307-r4-measurement.json")
    assert "matrix" in data and not data["matrix"]["instability"] and not any(data["matrix"]["issues"].values())
    result, superseded = [], []
    old = read("p1307-oracle.json")
    measured = read("p1307-oracle-measurement.json")
    lookup = {(r["case_id"], side, r["profile"]): r for side, rs in measured["results"].items() for r in rs}
    for old_case in old["cases"]:
        if old_case.get("context"):
            superseded.append({"id": old_case["id"], "reason": "Unsupported baseline eval --in adapter superseded by independently measured R2 compile context cases, preserving Location/LocatedContent type/serialized-value coverage; new repr debt policies explicit."})
            continue
        policy = "baseline" if old_case["preservation_control"] else "vanilla"
        if old_case["id"] in ("construct.State", "construct.Counter") or (old_case["id"].startswith("decoder.") and old_case["id"].endswith(".with")):
            policy = "vanilla"
        c = {"id": "p1307." + old_case["id"], "expression": old_case["expression"], "route": "eval", "source_sha256": source_hash(old_case["expression"]), "classes": old_case["classes"], "mandatory": True, "policy": policy, "origin": "p1307-oracle-measurement.json results; raw observations retained", "observations": {}}
        for profile in PROFILES:
            obs = {}
            for side in ("vanilla", "baseline"):
                row = lookup[old_case["id"], side, profile]
                obs[side] = envelope(row["returncode"], row["stdout"], row["stderr"], c)
            c["observations"][profile] = cell(obs["vanilla"], obs["baseline"], policy)
        c["measurement_ref"] = {"artifact":"p1307-oracle-measurement.json", "collection":"results.<side>", "case_key":"case_id", "case_id":old_case["id"], "order":"normal", "profile_key":"profile", "sides":["vanilla","baseline"]}
        result.append(c)
    r2 = read("p1307-r2-measurement.json")["matrix"]
    lookup = {(r["case"], r["side"], r["profile"]): r for r in r2["orders"]["normal"]}
    for old_case in r2["cases"]:
        c = dict(old_case)
        old_id = c["id"]
        policy = "baseline" if old_id.startswith("cbor-payload-") or old_id == "construct-LocatedContent" else "vanilla"
        c.update(id="r2." + old_id, expression=c.get("expression", ""), source_sha256=source_hash(c.get("document", c.get("expression", ""))), classes=["R2", "context" if c["route"] == "compile" else "with-or-CBOR"], mandatory=True, policy=policy, origin="p1307-r2-measurement.json matrix.normal", observations={}, transport_marker="P1307R2:")
        for profile in PROFILES:
            obs = {}
            for side in ("vanilla", "baseline"):
                row = lookup[old_id, side, profile]
                stdout = row.get("stdout")
                if stdout is None:
                    stdout = base64.b64decode(row.get("stdout_base64") or "").decode("utf-8")
                obs[side] = envelope(row["returncode"], stdout, row["stderr"], c)
            c["observations"][profile] = cell(obs["vanilla"], obs["baseline"], policy)
        if policy == "baseline" and old_id == "construct-LocatedContent":
            c["scope_out"] = "Existing direct Content repr debt is not corrected generally; serialized Content is measured separately and must use dedicated fields."
        c["measurement_ref"] = {"artifact":"p1307-r2-measurement.json", "collection":"matrix.orders.normal", "case_key":"case", "case_id":old_id, "order":"normal", "profile_key":"profile", "side_key":"side"}
        result.append(c)
    lookup = {(r["case"], r["side"], r["profile"]): r for r in data["matrix"]["orders"]["normal"]}
    for old_case in data["matrix"]["cases"]:
        c = dict(old_case)
        old_id = c["id"]
        c.update(id="r4." + old_id, origin="p1307-r4-measurement.json matrix.normal", observations={})
        for profile in PROFILES:
            c["observations"][profile] = cell(lookup[old_id,"vanilla",profile]["observable"], lookup[old_id,"baseline",profile]["observable"], c["policy"])
        c["measurement_ref"] = {"artifact":"p1307-r4-measurement.json", "collection":"matrix.orders.normal", "case_key":"case", "case_id":old_id, "order":"normal", "profile_key":"profile", "side_key":"side"}
        result.append(c)
    for old_case in data.get("missing_matrix", {}).get("cases", []):
        c = dict(old_case)
        old_id = c["id"]
        lookup = {(r["case"],r["side"],r["profile"]):r for r in data["missing_matrix"]["orders"]["normal"]}
        c.update(id="r4."+old_id, origin="p1307-r4-measurement.json missing_matrix.normal", observations={})
        for profile in PROFILES:
            c["observations"][profile] = cell(lookup[old_id,"vanilla",profile]["observable"],lookup[old_id,"baseline",profile]["observable"],"vanilla")
        c["measurement_ref"] = {"artifact":"p1307-r4-measurement.json", "collection":"missing_matrix.orders.normal", "case_key":"case", "case_id":old_id, "order":"normal", "profile_key":"profile", "side_key":"side"}
        result.append(c)
    assert len({c["id"] for c in result}) == len(result)
    return result, superseded

def attacks(cases):
    def spec(key, owner, anchor, defect, witnesses):
        assert all(w in {c["id"] for c in cases} for w in witnesses)
        return {"id": key, "owner": owner, "baseline_anchor": anchor, "defect": defect, "witnesses": witnesses, "source_mutant_status": "NOT_EXECUTED; requires applicable compiling candidate mutation with restoration and independent verifier"}
    prefix = "01_core/src/"
    return [
        spec("R4-A01", prefix+"compiler/eval/call_dispatch.rs", "eval_args; existing AST Arg::Named and Spread", "Discard earlier named values when rebuilding views", ["r4.args.mixed-duplicates", "r4.error.with-keeps-invalid-earlier"]),
        spec("R4-A02", prefix+"compiler/eval/call_dispatch.rs", "merge_with_args", "Use join collision removal instead of With concatenation", ["r4.error.with-keeps-invalid-earlier", "r2.with-json-invalid-first-occurrence"]),
        spec("R4-A03", prefix+"compiler/eval/closures.rs", "apply_closure sink construction", "Keep lexical positional/named interleaving in sink instead of named remainder then captured positional", ["r4.args.sink-mixed", "r4.args.sink-only"]),
        spec("R4-A04", prefix+"compiler/eval/closures.rs", "apply_closure parameter consumption and sink", "Reinsert consumed named/positional/default into sink", ["r4.args.sink-consumed-named", "r4.args.sink-default-absent"]),
        spec("R4-A05", prefix+"compiler/stdlib/collections.rs", "arguments_filter", "Iterate only collapsed projections or drop original spans", ["r4.args.filter-all", "r4.args.filter-first-duplicate", "r4.error.filter-preserve-value-span", "r4.origin.filter.f", "r4.origin.filter.g"]),
        spec("R4-A06", prefix+"compiler/stdlib/collections.rs", "arguments_map", "Preserve old value_span after identity map or discard arg-span", ["r4.error.map-detached-value-span", "r4.error.map-preserve-arg-span", "r4.origin.map.f", "r4.origin.map.g"]),
        spec("R4-A07", prefix+"compiler/stdlib/collections.rs", "arguments_map/filter callback loop", "Reorder callbacks to positional-first or deduplicate named", ["r4.error.map-first-callback", "r4.error.filter-first-callback", "r4.args.map-value", "r4.args.projections"]),
        spec("R4-A08", prefix+"compiler/eval/operators/join.rs", "join Value::Args branch", "Concatenate colliding LHS named instead of removing all", ["r4.args.join-duplicates", "r4.join.removes-invalid-earlier"]),
        spec("R4-A09", prefix+"compiler/eval/operators/arithmetic.rs", "eval_binary_op BinOp::Add", "Leave Args+Args unconnected to join owner", ["r4.args.join-collision", "r4.args.join-duplicates"]),
        spec("R4-A10", prefix+"compiler/eval/repr.rs", "repr_value / repr_func With", "Format native With with underlying short function name", ["r4.with.repr.calc.abs", "r4.with.repr.repr", "r4.with.repr.json"]),
        spec("R4-A11", prefix+"compiler/eval/repr.rs", "repr_value State/Counter/Location/Args", "Omit key/init, quote Page as Str, or retain three-dot Location", ["r4.repr.State", "r4.repr.State-escaping", "r4.repr.Counter-Page", "r4.repr.Counter-Str", "r4.repr.Location"]),
        spec("R4-A12", prefix+"compiler/stdlib/loading.rs", "value_to_cbor fallback / native_cbor_encode", "Preserve obsolete repr inside CBOR or alter payload without changing byte count", ["r4.cbor.delta.State", "r4.cbor.delta.With-native", "r4.cbor.delta.Args", "r4.cbor.delta.Location"]),
        spec("R4-A13", prefix+"compiler/stdlib/loading.rs", "value_to_cbor dedicated branches", "Apply new human-readable Symbol/Content/Bytes classification to old CBOR", ["r4.cbor.control.Symbol", "r4.cbor.control.Content", "r4.cbor.control.Bytes"]),
        spec("R4-A14", prefix+"entities/args.rs", "from_occurrences/remove_positional/remove_named", "Leave views stale after causal removal or collapse equal-valued distinct origins", ["r4.args.projections", "r4.args.sink-consumed-named", "r2.with-spread-arguments-f", "r2.with-spread-arguments-g"]),
    ]

def freeze(args):
    audit = read("p1307-r4-measurement.json")
    if "path_adapter_design_review" not in audit:
        fixture_case = next(c for c in read("p1307-r2-measurement.json")["matrix"]["cases"] if c["id"] == "feature-html")
        fixture_case = {**fixture_case,"source_sha256":source_hash(fixture_case["document"])}
        absolute = fixture_case["fixture"]
        relative = os.path.relpath(absolute,ROOT)
        expected = "<source:"+fixture_case["source_sha256"]+">"
        controls = [{"input":absolute,"expected":expected,"observed":canonical_stderr(absolute,fixture_case)}, {"input":relative,"expected":expected,"observed":canonical_stderr(relative,fixture_case)}, {"input":"payload bytes(12) unchanged","expected":"payload bytes(12) unchanged","observed":canonical_stderr("payload bytes(12) unchanged",fixture_case)}]
        audit["path_adapter_design_review"]={"at":now(),"failed_validations":[{"revision":1,"reason":"DIAGNOSTIC_SOURCE_UNRESOLVED","kind":"static frozen-input reparse; no binaries executed","unknown_fields":8,"algorithm":"absolute fixture replacement only","public_witness":"relative ../../../tmp fixture path remains unrecognized"},{"revision":2,"reason":"DIAGNOSTIC_SOURCE_UNRESOLVED","kind":"static frozen-input reparse; no binaries executed","unknown_fields":8,"algorithm":"absolute replacement before relative replacement","public_witness":"../../..<source:...> remains because absolute suffix replaced first"}],"input_refs":{"artifact":"p1307-r2-measurement.json","collection":"matrix.orders.normal","cells":[["feature-html","vanilla","default"],["feature-html","vanilla","a11y"],["feature-a11y","vanilla","default"],["feature-a11y","vanilla","html"]]},"coordinator_design_review":"/root reviewed exact cause and authorized descending-length exact identity substitution, no payload normalization; preserve prior validations and focal controls, no repeated full matrix.","corrective_hypothesis":"Replace longer exact relative identity before its absolute suffix. Only fixture identity changes, not diagnostic/source/underline/trace text.","controls":controls,"controls_preserved":all(c["observed"]==c["expected"] for c in controls)}
        save("p1307-r4-measurement.json",audit)
        assert audit["path_adapter_design_review"]["controls_preserved"]
    cases, superseded = consolidate()
    unknowns = [{"case": c["id"], "profile": p, "side": s, "reason": cell_[s].get("reason")} for c in cases for p,cell_ in c["observations"].items() for s in ("vanilla_literal", "baseline_literal", "future_expected") if cell_[s].get("kind") == "Unknown"]
    if unknowns:
        print(json.dumps({"consolidation_unknowns": unknowns}), flush=True)
        return
    # Focal cross-adapter replay checks that reused evidence is executable,
    # without remeasuring the full predecessor matrix or changing expected.
    selected_ids = {"p1307.json.default", "p1307.toml.none-array", "p1307.decoder.json.with", "p1307.construct.State", "r2.construct-LocatedContent", "r2.feature-html", "r2.feature-a11y", "r2.context-json-Location", "r2.context-json-LocatedContent", "r2.with-json-invalid-first-occurrence", "r2.cbor-payload-Symbol", "r4.error.map-detached-value-span", "r4.cbor.delta.Location"}
    selected = [dict(c) for c in cases if c["id"] in selected_ids]
    folder = fixtures(selected)
    rows = measure(selected, list(PROFILES))
    bycase = {c["id"]: c for c in selected}
    problems = [{"case": r["case"], "profile": r["profile"], "side": r["side"], "verdict": classify(bycase[r["case"]]["observations"][r["profile"]][r["side"]+"_literal"], r["observable"])} for r in rows if classify(bycase[r["case"]]["observations"][r["profile"]][r["side"]+"_literal"], r["observable"]) != "Preserved"]
    measurement = read("p1307-r4-measurement.json")
    assert "consolidation_focal" not in measurement, "do not overwrite consolidation attempt"
    measurement["consolidation_focal"] = {"provenance": provenance(), "fixture_directory": folder, "cases": selected, "rows": rows, "problems": problems, "finished": now()}
    save("p1307-r4-measurement.json", measurement)
    if problems:
        print(json.dumps({"consolidation_focal_problems": problems}), flush=True)
        return
    inputs = {name: sha(OUT/name) for name in ("p1307-r4-preflight.json", "p1307-r4-baseline.json", "p1307-r3-lineage.json", "p1307-r4-contract-refinement.json", "p1307-r4-call-span-refinement.json", "p1307-oracle.json", "p1307-oracle-measurement.json", "p1307-r2-measurement.json", "p1307-mutant-plan.json", "p1307-r4-measurement.json")}
    base = read("p1307-r4-baseline.json")
    contract_paths = [entry["prompt"] for entry in base["approved_L0"]]
    contract_paths += ["00_nucleo/prompts/compiler/eval/bindings/value_methods.md", "00_nucleo/prompts/compiler/stdlib/counter.md", "00_nucleo/prompts/compiler/eval/operators/arithmetic.md"]
    obj = {"schema": "p1307-r4-consolidated-public-oracle-v1", "regime": "executado sem atestacao de isolamento tecnico", "executor": "/root/p1307_oracle", "candidate_read": False, "candidate_expected_generation": False, "at": now(), "provenance": provenance(), "inputs": inputs, "contract_pins": {p:sha(ROOT/p) for p in dict.fromkeys(contract_paths)}, "baseline_source_inventory": base["source_inventory"], "binaries": binaries(), "script_sha256": sha(__file__), "profiles": list(PROFILES), "comparison": "Exact decoded public values (encoder Str bytes integral); entire diagnostic stderr/stdout/exit including source display, carets, hints and traces. Only exact temporary fixture path identity is replaced by frozen source SHA; no sorting, whitespace or payload normalization.", "unknown_policy": "Missing fields, non-UTF8, invalid CLI, unexecuted context, unresolved source and missing mandatory data remain Unknown; only explicitly artificial classifier opacity controls may expect Unknown.", "cases": cases, "superseded_context_adapters": superseded, "inherited_source_mutant_plan": {"path":"p1307-mutant-plan.json", "families":20, "status":"all remain obligations; old pre-gate scope limitations superseded by approved owners where applicable; applicability must still be verified"}, "successor_source_mutant_plan": attacks(cases), "source_mutants_executed":0, "mutation_score":None, "limitations":["Nonterminal sink positional binding is a measured baseline debt control, not promoted to new ClosureRepr API.", "Direct Content repr, unrelated constructor/selector debts and other explicitly baseline-pinned controls are not general vanilla parity.", "Public tests cannot guarantee coherence of every externally mutable Rust Args field or every writer; independent internal tests and writer audit remain mandatory.", "Predecessor full matrices reused intact; current extension fully normal/repeat/reverse, consolidated adapter checked focally. Future candidate requires entire consolidated replay."]}
    save("p1307-r4-oracle.json", obj)
    print(json.dumps({"cases":len(cases), "profile_cells":len(cases)*len(PROFILES), "superseded_context_adapters":len(superseded), "consolidation_focal_runs":len(rows), "unknowns":unknowns, "source_mutants_executed":0}), flush=True)

def missing(args):
    data = read("p1307-r4-measurement.json")
    assert "missing_matrix" not in data
    cases = []
    for fmt in ("json","toml","yaml"):
        for route,expr in {"alias":f'{{ let f = {fmt}.encode; f() }}', "with-alias":f'{{ let f = {fmt}.encode.with(); f() }}', "with-direct":f'{fmt}.encode.with()()'}.items():
            cases.append({"id":f"missing.{fmt}.{route}", "expression":expr, "route":"eval", "source_sha256":source_hash(expr), "classes":["diagnostic","missing","whole-call","alias","With"], "mandatory":True, "policy":"vanilla", "expected_shape":"diagnostic", "required_message":"missing argument: value"})
    focal_rows = measure(cases,["default"])
    data["missing_focal"]={"provenance":provenance(),"cases":cases,"rows":focal_rows,"issues":issues(cases,focal_rows),"finished":now()}
    save("p1307-r4-measurement.json",data)
    if data["missing_focal"]["issues"]:
        print(json.dumps(data["missing_focal"]["issues"]))
        return
    orders={name:measure(ordered,list(PROFILES)) for name,ordered in (("normal",cases),("repeat",cases),("reverse",list(reversed(cases))))}
    first={(r["case"],r["side"],r["profile"]):r["observable"] for r in orders["normal"]}
    unstable=[(name,r["case"],r["side"],r["profile"]) for name,rs in orders.items() for r in rs if first[r["case"],r["side"],r["profile"]]!=r["observable"]]
    data["missing_matrix"]={"provenance":provenance(),"cases":cases,"orders":orders,"issues":{name:issues(cases,rs) for name,rs in orders.items()},"instability":unstable,"finished":now()}
    save("p1307-r4-measurement.json",data)
    print(json.dumps({"runs":sum(map(len,orders.values())),"issues":data["missing_matrix"]["issues"],"instability":unstable,"vanilla_default":[{"case":r["case"],"stderr":r["stderr"]} for r in focal_rows if r["side"]=="vanilla"]},ensure_ascii=False))

def replay(args):
    oracle = read("p1307-r4-oracle.json")
    assert sha(__file__) == oracle["script_sha256"]
    for name, expected in oracle["inputs"].items():
        assert sha(OUT/name) == expected, "frozen oracle input changed: " + name
    cases = [dict(c) for c in oracle["cases"] if not args.case or re.search(args.case,c["id"])]
    if args.reverse:
        cases.reverse()
    profiles = [args.profile] if args.profile else list(PROFILES)
    folder = fixtures(cases)
    rows = []
    jobs = [(args.binary, "candidate", c, p) for c in cases for p in profiles]
    bycase = {c["id"]:c for c in cases}
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        for row in pool.map(lambda x: run(*x), jobs):
            row["verdict"] = classify(bycase[row["case"]]["observations"][row["profile"]]["future_expected"], row["observable"])
            rows.append(row)
    print(json.dumps({"schema":"p1307-r4-public-replay-v1", "at":now(), "binary":args.binary, "binary_sha256":sha(args.binary), "oracle_sha256":sha(OUT/"p1307-r4-oracle.json"), "fixture_directory":folder, "rows":rows, "counts":{v:sum(r["verdict"]==v for r in rows) for v in ("Preserved","Violated","Unknown")}},ensure_ascii=False))

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["focal", "full", "missing", "freeze", "replay"])
    parser.add_argument("--binary")
    parser.add_argument("--profile", choices=list(PROFILES))
    parser.add_argument("--case", help="Optional regex subset; absent means full consolidated corpus")
    parser.add_argument("--reverse", action="store_true")
    opts = parser.parse_args()
    {"focal": focal, "full": full, "missing": missing, "freeze": freeze, "replay": replay}[opts.command](opts)
