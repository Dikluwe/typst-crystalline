#!/usr/bin/env python3
"""Independent P1307 documentary oracle. Never writes Rust or derives expected from candidate.

freeze --baseline-binary PATH measures pinned vanilla and certified baseline.
classify(expected, observed) is the public-envelope API used by the verifier.
replay --binary PATH checks a future binary against frozen literal expectations.
"""
import argparse
import concurrent.futures
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "00_nucleo/diagnosticos"
VANILLA = "/usr/local/bin/typst"
PIN = "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
PROFILES = {"default": [], "html": ["--features", "html"], "a11y": ["--features", "a11y-extras"], "html+a11y": ["--features", "html,a11y-extras"]}
CONTEXT = "= Probe <p1307>\n"
SOURCES = ["01_core/src/entities/value.rs", "01_core/src/compiler/eval/repr.rs", "01_core/src/compiler/stdlib/loading.rs", "01_core/src/compiler/stdlib/mod.rs", "01_core/src/compiler/eval/mod.rs", "lab/typst-original/crates/typst-library/src/foundations/value.rs", "lab/typst-original/crates/typst-library/src/foundations/bytes.rs", "lab/typst-original/crates/typst-library/src/foundations/symbol.rs", "lab/typst-original/crates/typst-library/src/foundations/content/mod.rs", *[f"lab/typst-original/crates/typst-library/src/loading/{x}.rs" for x in ("json", "toml", "yaml", "cbor")]]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def stamp():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def save(name, obj):
    (OUT / name).write_text(json.dumps(obj, ensure_ascii=False, indent=2) + "\n")


def corpus():
    cases = []
    def add(key, expr, classes, control=False, context=None):
        cases.append(dict(id=key, expression=expr, classes=classes, preservation_control=control, context=context, mandatory=True))
    values = {
        "None": "none", "Bool": "true", "Int": "42", "Float": "1.25", "Str": '"text"',
        "Array": "(1, false, (2, 3))", "Dict": "(z: 1, a: 2)", "Module": "calc",
        "Datetime": "datetime(year: 2024, month: 2, day: 29)", "Func": "(x => x)",
        "Content": "[hi]", "LocatedContent": "query(<p1307>).first()", "Auto": "auto",
        "Length": "2pt", "Relative": "50% + 2pt", "Ratio": "25%", "Angle": "90deg",
        "Color": "red", "Stroke": "stroke()", "Fraction": "2fr", "Align": "left + top",
        "Location": "query(<p1307>).first().location()", "Gradient": "gradient.linear(red, blue)",
        "Regex": 'regex("a+")', "Tiling": "tiling(size: (1pt, 1pt), [])", "Bytes": "bytes((0, 255, 10))",
        "Decimal": 'decimal("1.20")', "Duration": "duration(seconds: 90)", "Version": "version(1, 2, 3)",
        "Selector": "heading.where(level: 1)", "Symbol": "sym.alpha", "Args": "((..a) => a)(1, key: 2)",
        "State": 'state("p1307", 0)', "Counter": "counter(heading)", "Label": "<p1307>",
        "Dir": "ltr", "Path": 'path("x")', "Type": "int",
    }
    for variant, value in values.items():
        context = CONTEXT if variant in ("LocatedContent", "Location") else None
        add(f"construct.{variant}", f"(repr(type({value})), repr({value}))", ["constructibility", variant], True, context)
        for fmt in ("json", "toml", "yaml"):
            arg = f"(value: {value})" if fmt == "toml" else value
            add(f"{fmt}.value.{variant}", f"{fmt}.encode({arg})", ["serialization", variant], context=context)
    extra = {
        "empty-string": '""', "unicode": '"ação α 😀 中"', "escapes": r'"quote\" backslash\\ tab\t nul\u{0} backspace\u{8} cr\r lf\n"',
        "multiline": r'"first\nsecond\n"', "multiline-no-end": r'"first\nsecond"',
        "multiline-blank": r'"\nfirst\n\nsecond\n\n"', "yaml-reserved": '("yes", "no", "null", "true", "1.0", "---", "#tag", "a: b", "", " ")',
        "empty-array": "()", "empty-dict": "(:)", "nested": '(z: (q: 1, a: (2, 3)), a: "x", m: (b: false))',
        "key-escaping": '("z z": 1, "a:b": 2, "quo\\\"te": 3, "ü": 4)',
        "array-heterogeneous": '(1, "x", true, (a: 2))', "int-max": "9223372036854775807", "int-min": "-9223372036854775807 - 1",
        "negative-zero": "-0.0", "nan": 'float("nan")', "inf": 'float("inf")', "negative-inf": '-float("inf")',
        "float-max": "1.7976931348623157e308", "float-min-normal": "2.2250738585072014e-308", "float-subnormal": "5e-324",
        "float-safe-integer": "9007199254740992.0", "float-exponent": "1e20", "float-small": "1e-7",
        "bytes-empty": "bytes(())", "symbol-modifier": "sym.arrow.r.double", "content-empty": "[]", "content-strong": "[*bold*]", "content-sequence": "[a *b* c]",
    }
    for tag, value in extra.items():
        for fmt in ("json", "toml", "yaml"):
            arg = f"(value: {value})" if fmt == "toml" else value
            add(f"{fmt}.edge.{tag}", f"{fmt}.encode({arg})", ["edge", tag])
    for fmt in ("json", "toml", "yaml", "cbor"):
        control = fmt == "cbor"
        add(f"{fmt}.identity", f"(repr(type({fmt})), repr({fmt}), repr(type({fmt}.encode)), repr({fmt}.encode))", ["namespace", "identity"], control)
        valid = "(z: (1, 2), a: 3)"
        for tag, call in {"missing": "", "extra": f"{valid}, 2", "unexpected": f"{valid}, other: true", "named-value": f"value: {valid}"}.items():
            add(f"{fmt}.arg.{tag}", f"{fmt}.encode({call})", ["diagnostic", "argument", tag], control)
        for route in ("parent-empty", "parent-bound", "parent-twice", "encoder-empty", "encoder-bound", "encoder-twice"):
            expr = {"parent-empty": f"{fmt}.with().encode({valid})", "parent-bound": f'{fmt}.with(bytes("x")).encode({valid})', "parent-twice": f"{fmt}.with().with().encode({valid})", "encoder-empty": f"{fmt}.encode.with()({valid})", "encoder-bound": f"{fmt}.encode.with({valid})()", "encoder-twice": f"{fmt}.encode.with().with({valid})()"}[route]
            add(f"{fmt}.with.{route}", expr, ["with", route], control)
        if fmt != "cbor":
            add(f"{fmt}.default", f"{fmt}.encode({valid})", ["pretty", "default"])
            for pretty in ("true", "false", '"x"'):
                add(f"{fmt}.pretty.{pretty}", f"{fmt}.encode({valid}, pretty: {pretty})", ["pretty", "argument"])
            add(f"{fmt}.roundtrip", f"repr({fmt}(bytes({fmt}.encode({valid}))))", ["roundtrip"])
    for key, expression in {
        "toml.none-array": "toml.encode((z: (none,)))", "toml.top-none": "toml.encode(none)",
        "toml.top-array": "toml.encode((1, 2))", "toml.top-string": 'toml.encode("x")',
        "toml.none-nested": "toml.encode((z: (q: none, a: 1), a: none))",
        "json.with-pretty": "json.encode.with(pretty: false)((z: 1, a: 2))",
        "toml.with-pretty": "toml.encode.with(pretty: false)((z: (1, 2), a: 3))",
    }.items():
        add(key, expression, ["argument", "serialization-boundary"])
    for fmt, expression in {
        "read": 'read("Cargo.toml").slice(0, 8)', "csv": 'csv(bytes("a,b\\nc,d\\n"))',
        "json": 'json(bytes("{\\\"z\\\":1,\\\"a\\\":2}"))', "yaml": 'yaml(bytes("z: 1\\na: 2\\n"))',
        "toml": 'toml(bytes("z=1\\na=2\\n"))', "cbor": "cbor(bytes((130, 1, 2)))", "xml": 'xml(bytes("<a key=\\\"v\\\">x</a>"))',
    }.items():
        add(f"decoder.{fmt}", f"repr({expression})", ["decoder", fmt], True)
        add(f"decoder.{fmt}.missing", f"{fmt}()", ["decoder", "diagnostic"], True)
        add(f"decoder.{fmt}.wrong-type", f"{fmt}(42)", ["decoder", "diagnostic"], True)
        add(f"decoder.{fmt}.with", f"{fmt}.with()", ["decoder", "with"], True)
    for fmt in ("csv", "xml", "read"):
        add(f"negative.{fmt}.encode", f"{fmt}.encode", ["absence", "negative"], True)
    for tag, value in {"primitive": "(z: 1, a: 2)", "bytes": "bytes((0, 255, 10))", "Symbol": "sym.alpha", "Content": "[hi]"}.items():
        add(f"cbor.preserve.{tag}", f"repr(cbor(cbor.encode({value})))", ["cbor", "preservation", tag], True)
    return cases


def observe(binary, case, profile):
    argv = [str(binary), "eval", case["expression"], "--format", "json", *PROFILES[profile]]
    if case["context"]:
        argv += ["--in", "-"]
    started = stamp()
    begin = time.monotonic()
    try:
        environment = {k: v for k, v in os.environ.items() if k != "TYPST_FEATURES"}
        environment.update(NO_COLOR="1", TERM="dumb")
        proc = subprocess.run(argv, input=case["context"], text=True, capture_output=True, cwd=ROOT, timeout=15, env=environment)
    except (subprocess.TimeoutExpired, OSError) as exc:
        return dict(case_id=case["id"], profile=profile, argv=argv, started=started, elapsed=time.monotonic()-begin, observable={"kind": "Unknown", "reason": type(exc).__name__}, stdout="", stderr=str(exc), returncode=None)
    obs = envelope(proc.returncode, proc.stdout, proc.stderr, case)
    return dict(case_id=case["id"], profile=profile, argv=argv, started=started, elapsed=time.monotonic()-begin, observable=obs, stdout=proc.stdout, stderr=proc.stderr, returncode=proc.returncode)


def envelope(code, stdout, stderr, case):
    if code == 0:
        try:
            value = json.loads(stdout)
        except (ValueError, TypeError):
            return {"kind": "Unknown", "reason": "OUTPUT_NOT_JSON"}
        if stderr:
            return {"kind": "Unknown", "reason": "UNPARSED_SUCCESS_STDERR", "stderr": stderr}
        return {"kind": "value", "value": value}
    if code != 1 or not stderr.startswith("error:"):
        return {"kind": "Unknown", "reason": "EXECUTION_FAILURE", "returncode": code}
    messages = re.findall(r"^error: (.*)$", stderr, re.M)
    hints = re.findall(r"^\s*= hint: (.*)$", stderr, re.M)
    locations = re.findall(r"[┌└]─ ([^\n]+):(\d+):(\d+)", stderr)
    spans = []
    for source, line, col in locations:
        if source != "<input-expression>":
            return {"kind": "Unknown", "reason": "DIAGNOSTIC_SOURCE_UNRESOLVED", "stderr": stderr}
        lines = case["expression"].splitlines(keepends=True)
        line = int(line)
        col = int(col)
        if not 1 <= line <= len(lines):
            return {"kind": "Unknown", "reason": "DIAGNOSTIC_LINE_UNRESOLVED"}
        # Typst eval emits zero-based UTF-8 source columns; underline width is
        # a character count for our single-line ASCII argument-error fixtures.
        marks = re.findall(r"^\s*│\s*(\^+)(?:[^\n]*)$", stderr, re.M)
        if len(marks) != len(messages):
            return {"kind": "Unknown", "reason": "DIAGNOSTIC_RANGE_UNRESOLVED", "stderr": stderr}
        start = sum(len(s.encode()) for s in lines[:line-1]) + col
        width = len(marks[len(spans)])
        # A Unicode suffix outside the underlined range cannot invalidate a
        # known ASCII prefix span. Never infer display-width-to-byte conversion
        # if the prefix through the underline itself contains non-ASCII.
        if any(ord(c) > 127 for c in lines[line-1][:col+width]):
            return {"kind": "Unknown", "reason": "DIAGNOSTIC_RANGE_UNRESOLVED", "stderr": stderr}
        spans.append({"source": source, "start": start, "end": start + width, "half_open": True, "line": line, "column": col})
    if messages and not locations and "┌" not in stderr and "└" not in stderr:
        spans = [{"kind": "detached"} for _ in messages]
    if len(spans) != len(messages) or not messages:
        return {"kind": "Unknown", "reason": "DIAGNOSTIC_CARDINALITY_OR_RANGE_UNRESOLVED", "stderr": stderr}
    return {"kind": "diagnostic", "messages": messages, "cardinality": len(messages), "hints": hints, "spans": spans, "stderr_literal": stderr}


def classify(expected, observed):
    """Compare full language envelopes. Missing/opaque input is Unknown, never PASS."""
    if not isinstance(expected, dict) or not isinstance(observed, dict):
        return "Unknown"
    if expected.get("kind") not in ("value", "diagnostic") or observed.get("kind") not in ("value", "diagnostic"):
        return "Unknown"
    required = {"value": {"kind", "value"}, "diagnostic": {"kind", "messages", "cardinality", "hints", "spans", "stderr_literal"}}
    if not required[expected["kind"]] <= expected.keys() or not required[observed["kind"]] <= observed.keys():
        return "Unknown"
    return "Preserved" if expected == observed else "Violated"


def provenance():
    def git(*args):
        return subprocess.check_output(["git", *args], cwd=ROOT, text=True)
    return {"timestamp": stamp(), "head": git("rev-parse", "HEAD").strip(), "tracked_diff_stat": git("diff", "HEAD", "--stat"), "tracked_diff_sha256": hashlib.sha256(git("diff", "HEAD").encode()).hexdigest(), "cwd": str(ROOT), "script_sha256": sha(__file__), "preflight_sha256": sha(OUT / "p1307-preflight.json"), "sources": {p: sha(ROOT / p) for p in SOURCES}, "env": {"NO_COLOR": "1", "TERM": "dumb", "TYPST_FEATURES": "unset"}}


def run_matrix(binary, cases, profiles):
    jobs = [(case, p) for case in cases for p in profiles]
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        return list(pool.map(lambda job: observe(binary, *job), jobs))


def freeze(args):
    assert sha(VANILLA) == PIN, "unratified vanilla"
    cases = corpus()
    start = provenance()
    previous_path = OUT / "p1307-oracle-measurement.json"
    previous = json.loads(previous_path.read_text()) if previous_path.exists() else None
    attempt = 0 if previous is None else previous["attempt"] + 1
    assert attempt <= 2, "focal revision budget exhausted"
    profiles = list(PROFILES)
    results = {}
    for side, binary in (("vanilla", VANILLA), ("baseline", args.baseline_binary)):
        print(f"measuring {side} {len(cases)} cases x {len(profiles)} profiles", flush=True)
        results[side] = run_matrix(binary, cases, profiles)
    expected = {(r["case_id"], r["profile"]): r["observable"] for r in results["vanilla"]}
    baseline = {(r["case_id"], r["profile"]): r["observable"] for r in results["baseline"]}
    oracle_cases = []
    for case in cases:
        observations = {}
        for profile in profiles:
            v, b = expected[case["id"], profile], baseline[case["id"], profile]
            observations[profile] = {"vanilla_literal": v, "baseline_literal": b, "future_expected": b if case["preservation_control"] else v, "baseline_comparison": classify(v, b), "expectation_policy": "preserve-existing-baseline-control" if case["preservation_control"] else "ratified-vanilla-literal"}
        oracle_cases.append({**case, "source_numbered": "\n".join(f"{i+1}: {line}" for i, line in enumerate(case["expression"].splitlines())), "observations": observations})
    unknowns = [{"side": side, "case": r["case_id"], "profile": r["profile"], "reason": r["observable"].get("reason")} for side, rows in results.items() for r in rows if r["observable"]["kind"] == "Unknown"]
    measurement = dict(protocol="P1307-documentary-oracle-v1", phase="pre-human-gate; no candidate", attempt=attempt, revision_policy="maximum two focal revisions; previous attempts nested intact", previous_attempts=[] if previous is None else [previous], provenance=start, binaries={"vanilla": {"path": VANILLA, "sha256": PIN}, "baseline": {"path": args.baseline_binary, "sha256": sha(args.baseline_binary)}}, results=results, unknowns=unknowns, finished=stamp())
    save("p1307-oracle-measurement.json", measurement)
    save("p1307-oracle.json", {"protocol": "P1307-literal-oracle-v1", "regime": "executado sem atestacao de isolamento tecnico", "candidate_exists": False, "source_state": start, "all_current_value_variants": list(dict.fromkeys(c["classes"][1] for c in cases if c["id"].startswith("construct."))), "comparison": "exact decoded JSON envelope; complete serialized public Str unchanged; no sorting, whitespace normalization or roundtrip substitution", "unknown_policy": "mandatory Unknown blocks seal; construction diagnostic is not proof of constructed value", "cases": oracle_cases, "unknowns": unknowns, "measurement_sha256": sha(OUT / "p1307-oracle-measurement.json")})
    print(json.dumps({"cases": len(cases), "unknowns": unknowns}, ensure_ascii=False), flush=True)


def replay(args):
    frozen = json.loads((OUT / "p1307-oracle.json").read_text())
    rows = []
    for case in frozen["cases"]:
        for profile in PROFILES:
            observed = observe(args.binary, case, profile)
            rows.append({**observed, "verdict": classify(case["observations"][profile]["future_expected"], observed["observable"])})
    print(json.dumps(rows, ensure_ascii=False))


def calibrate(args):
    selected = [c for c in corpus() if c["id"].startswith("construct.")]
    rows = run_matrix(VANILLA, selected, ["default"])
    save("p1307-oracle-measurement.json", {"protocol": "P1307-documentary-oracle-v1", "attempt": 0, "phase": "adapter-focal-calibration", "hypothesis": "CLI environment controls allow construction probes", "provenance": provenance(), "results": {"vanilla": rows}, "classification_delta": {"Preserved": 0, "Violated": 0, "Unknown": len(rows)}, "reason_code": "EMPTY_TYPST_FEATURES_REJECTED", "full_corpus_run": False})


def orders(args):
    measurement = json.loads((OUT / "p1307-oracle-measurement.json").read_text())
    oracle = json.loads((OUT / "p1307-oracle.json").read_text())
    cases = oracle["cases"]
    repetitions = {}
    mismatches = []
    for order in ("repeat", "reverse"):
        repetitions[order] = {}
        for side in ("vanilla", "baseline"):
            binary = measurement["binaries"][side]["path"]
            assert sha(binary) == measurement["binaries"][side]["sha256"]
            print(f"measuring {order} {side}", flush=True)
            rows = run_matrix(binary, cases if order == "repeat" else list(reversed(cases)), list(PROFILES))
            repetitions[order][side] = rows
            lookup = {(r["case_id"], r["profile"]): r["observable"] for r in measurement["results"][side]}
            for row in rows:
                result = classify(lookup[row["case_id"], row["profile"]], row["observable"])
                if result != "Preserved":
                    mismatches.append({"order": order, "side": side, "case": row["case_id"], "profile": row["profile"], "verdict": result})
    measurement["repeat_reverse"] = {"provenance": provenance(), "results": repetitions, "mismatches": mismatches, "finished": stamp()}
    save("p1307-oracle-measurement.json", measurement)
    oracle["measurement_sha256"] = sha(OUT / "p1307-oracle-measurement.json")
    oracle["order_stability"] = {"orders": ["normal", "repeat", "reverse"], "mismatches": mismatches}
    save("p1307-oracle.json", oracle)
    print(json.dumps({"order_mismatches": mismatches}), flush=True)


def refine(args):
    previous = json.loads((OUT / "p1307-oracle-measurement.json").read_text())
    assert previous["attempt"] == 1, "final focal revision only"
    cases = {c["id"]: c for c in corpus()}
    measurements = {}
    focal = []
    for side, rows in previous["results"].items():
        measurements[side] = []
        for row in rows:
            row = dict(row)
            row["observable"] = envelope(row["returncode"], row["stdout"], row["stderr"], cases[row["case_id"]])
            measurements[side].append(row)
        affected = [c for c in cases.values() if any(r["case_id"] == c["id"] and r["profile"] == "default" and r["observable"]["kind"] == "Unknown" for r in rows)]
        if affected:
            focal.extend({"side": side, **r} for r in run_matrix(previous["binaries"][side]["path"], affected, ["default"]))
    unresolved = [{"side": side, "case": r["case_id"], "profile": r["profile"], "reason": r["observable"].get("reason")} for side, rows in measurements.items() for r in rows if r["observable"]["kind"] == "Unknown"]
    now = provenance()
    measurement = {**previous, "attempt": 2, "provenance": now, "previous_attempts": [previous], "results": measurements, "unknowns": unresolved, "final_focal_revision": {"hypothesis": "Model source-less diagnostics explicitly as detached; resolve ASCII underlined prefixes with Unicode only outside span. Unsupported baseline --in remains Unknown, not a value construction.", "rows": focal, "unknown_before": len(previous["unknowns"]), "unknown_after": len(unresolved), "no_more_adapter_revisions": True}, "orders": {"normal": "EXECUTED", "repeat": "NOT_EXECUTED_OBSERVABILITY_STOP" if unresolved else "PENDING", "reverse": "NOT_EXECUTED_OBSERVABILITY_STOP" if unresolved else "PENDING"}, "finished": stamp()}
    save("p1307-oracle-measurement.json", measurement)
    oracle = json.loads((OUT / "p1307-oracle.json").read_text())
    byside = {side: {(r["case_id"], r["profile"]): r["observable"] for r in rows} for side, rows in measurements.items()}
    for c in oracle["cases"]:
        for profile, cell in c["observations"].items():
            v, b = byside["vanilla"][c["id"], profile], byside["baseline"][c["id"], profile]
            cell.update(vanilla_literal=v, baseline_literal=b, future_expected=b if c["preservation_control"] else v, baseline_comparison=classify(v, b))
    oracle.update(unknowns=unresolved, measurement_sha256=sha(OUT / "p1307-oracle-measurement.json"), order_stability=measurement["orders"], documentary_status="BLOCKED_CONTEXT_OBSERVABILITY" if unresolved else "OBSERVABLE_PENDING_ORDERS")
    save("p1307-oracle.json", oracle)
    print(json.dumps({"unknown_before": len(previous["unknowns"]), "unknown_after": len(unresolved), "unknowns": unresolved}), flush=True)


def finalize(args):
    inputs = {n: sha(OUT / n) for n in ("p1307-preflight.json", "p1307-baseline.json", "p1307-contract.md", "p1307-oracle.json", "p1307-oracle-measurement.json")}
    registration = {"path": "01_core/src/compiler/eval/mod.rs", "function": "make_stdlib_with_features", "baseline_lines": [1474, 1723, 1724, 1725, 1727, 1742], "anchor": 'scope.define("json", Value::Func(Func::native("json", native_json)));'}
    loading = {"path": "01_core/src/compiler/stdlib/loading.rs", "function": "native_loader! and native_cbor_encode; future authorized private encoder adapters", "baseline_lines": [267, 540, 541, 542], "anchor": 'native_loader!(native_json, "json", decode_json);'}
    families = [
        (1, "install only one/two members", registration, ["json.identity", "toml.identity", "yaml.identity"], "Omit one future namespace registration; require surviving build then public missing-field witness."),
        (2, "flat alias instead of namespace", registration, ["json.identity"], "Register flat alias json_encode while parent remains namespace-free."),
        (3, "replace callable decoder by noncallable module", registration, ["decoder.json", "decoder.toml", "decoder.yaml"], "Replace one parent with module carrying encoder; decoder call must fail observably."),
        (4, "long internal/public function name", registration, ["json.identity"], 'Change Func::native name from encode to json.encode.'),
        (5, "lose namespace through observed with route", registration, ["json.with.parent-empty", "json.with.parent-bound", "json.with.parent-twice"], "Construct parent with namespace whose encode binding is a partially applied function with an injected extra argument, exposing .with path divergence. A proper route-specific hook may require scope reopening; do not claim applicability before compiling and witnessing it."),
        (6, "JSON compact default", loading, ["json.default", "json.pretty.true"], "Change omitted pretty to false."),
        (7, "TOML compact default", loading, ["toml.default", "toml.pretty.true"], "Change omitted pretty to false for dictionary containing array."),
        (8, "YAML silently accepts pretty", loading, ["yaml.pretty.true"], "Remove named-argument rejection for pretty."),
        (9, "TOML accepts non-dict top", loading, ["toml.top-none", "toml.top-array", "toml.top-string"], "Coerce top-level non-dict instead of required cast."),
        (10, "sort/swap dictionary keys", loading, ["json.value.Dict", "toml.edge.nested", "yaml.value.Dict"], "Sort keys before emission or swap the first two within one formatting class."),
        (11, "change final newline", loading, ["yaml.default", "toml.default", "json.default"], "Drop/add one newline from returned public Str."),
        (12, "escaping or multiline morphology", loading, ["json.edge.escapes", "yaml.edge.multiline", "toml.edge.unicode"], "Change escaping or choose quoted scalar where ratified emitter uses literal multiline; preserve syntax where possible."),
        (13, "Debug instead of public repr", loading, ["json.value.Color", "yaml.value.State", "toml.value.Counter"], "Replace opaque fallback helper with Rust debug output."),
        (14, "generic fallback for Symbol/Content", loading, ["json.value.Symbol", "json.value.Content", "yaml.edge.content-strong"], "Route each dedicated branch to generic public repr fallback."),
        (15, "human readable Bytes as numeric array", loading, ["json.value.Bytes", "yaml.value.Bytes", "toml.value.Bytes"], "Use byte sequence serialization on human readable emitters."),
        (16, "none/nonfinite/negative-zero", loading, ["json.edge.nan", "yaml.edge.negative-zero", "toml.none-array", "toml.value.None"], "One submutant per distinct class: reject or misencode None, NaN, inf, -inf, -0.0."),
        (17, "detached/whole-call error span", loading, ["toml.none-array", 'json.pretty."x"'], "Replace original positional-value span with detached or call span. If no scoped implementation access exists, report scope blocker rather than instrument unrelated owner."),
        (18, "regress CBOR/decoder", loading, ["cbor.preserve.bytes", "decoder.json", "decoder.csv", "decoder.xml"], "Mutate existing native_cbor_encode output or one native decoder result; comparator uses separately frozen baseline debt controls."),
        (19, "invent csv/xml encoder", registration, ["negative.csv.encode", "negative.xml.encode", "negative.read.encode"], "Give forbidden parent a namespace containing encode."),
        (20, "feature-gated encoder exposure", registration, ["json.identity", "toml.identity", "yaml.identity"], "Guard registrations by html/a11y capability; run exact same witness under every frozen profile."),
    ]
    oracle = json.loads((OUT / "p1307-oracle.json").read_text())
    ids = {c["id"] for c in oracle["cases"]}
    assert all(w in ids for _, _, _, ws, _ in families for w in ws)
    plan = {"protocol": "P1307-source-mutant-plan-v1", "phase": "documentary plan only; future execution requires ADR0127 confirmation and implementation", "inputs": inputs, "source_anchors_sha256": {p: sha(ROOT / p) for p in (registration["path"], loading["path"])}, "actual_source_mutants_executed": 0, "mutation_score": None, "applicability_policy": "All twenty families are obligations, not claimed applicable kills. Locate actual candidate function within existing owner after gate; record concrete patch and restoration, compilation success, process returncode and literal witness. No fake products; nonapplying/noncompiling mutant is Unknown, never killed.", "scope_reopening": "Family 5 route-specific instrumentation and 17 span access require verification against authorized owners. Do not change Func/Args/field_access solely to make mutation or implementation convenient.", "families": [{"id": n, "defect": name, "owner_anchor": anchor, "witnesses": ws, "planned_mutation": change, "execution_status": "NOT_EXECUTED_PRE_HUMAN_GATE"} for n, name, anchor, ws, change in families]}
    save("p1307-mutant-plan.json", plan)
    source_classification = {
        "direct": ["None", "Bool", "Int", "Float", "Str", "Array", "Dict"],
        "dedicated": {"Bytes": "human-readable repr string; CBOR byte-string", "Symbol": "resolved Unicode string, not symbol repr", "Content": "ordered map func plus public fields", "LocatedContent": "same public content category with context-generated construction; location does not become serializer field by default"},
        "opaque": [v for v in oracle["all_current_value_variants"] if v not in ("None", "Bool", "Int", "Float", "Str", "Array", "Dict", "Bytes", "Symbol", "Content", "LocatedContent")],
        "source": "ratified foundations/value.rs:343-364; symbol.rs:394; bytes.rs:362; content/mod.rs:709",
        "scope_risks": "Baseline repr.rs:87 Location and :122-123 State/Counter demonstrably contain non-vanilla generic text. Public bilateral construction witnesses decide concrete debt. New encoder expectations remain vanilla; preservation controls do not authorize weakening them.",
        "bilateral_construction": {"observed_variants": 36, "unknown_variants": ["Location", "LocatedContent"], "all_38_vanilla_constructed": True},
        "coverage_limits": ["Context baseline --in is unsupported, leaving 32 mandatory Unknown after final focal revision; no ready seal.", "CBOR preservation probes check decoded public values and bytes(N) transport, not every byte of raw CBOR payload. A future complete preservation oracle must add a full public byte-array projection.", "This independent corpus measures basic parent/encoder with routes and bound pretty success. The contract also freezes named override and prebound invalid-named origin diagnostics; those finer routes are in the separately authored contract measurement, and require additional independent oracle witnesses in a reopened chain before implementation sealing.", "Family 5 route-specific namespace-loss mutation lacks a demonstrated applicable scoped instrument before a candidate exists. Twenty listed families are an obligation plan, not an applicable mutation suite or score."],
    }
    save("p1307-oracle-receipt.json", {"protocol": "P1307-independent-documentary-oracle-receipt-v1", "timestamp": stamp(), "executor": "/root/p1307_oracle", "regime": "executado sem atestacao de isolamento tecnico", "role": "oracle/adversary author, no candidate or verdict authority", "inherited_context": "fresh agent task; parent assignment only", "read_allowlist": ["exact P1307 step", "repository CLAUDE and relevant ADR", "skill+both references", "preflight", "baseline", "canonical contract", "ratified serializer sources", "baseline Value/registration/loading/repr sources"], "write_allowlist": ["p1307-oracle.py", "p1307-oracle-measurement.json", "p1307-oracle.json", "p1307-oracle-receipt.json", "p1307-mutant-plan.json"], "forbidden_writes_performed": False, "candidate_read": False, "inputs": inputs, "step_sha256": sha(ROOT / "00_nucleo/materialization/typst-passo-1307.md"), "script_sha256": sha(__file__), "mutant_plan_sha256": sha(OUT / "p1307-mutant-plan.json"), "ratified_source_hashes": {p: sha(ROOT / p) for p in SOURCES}, "source_classification": source_classification, "unknowns": oracle["unknowns"], "scope": "Literal future encoder oracle plus separately identified preexisting baseline preservation debt; no implementation parity or mutation score claimed"})


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    commands = parser.add_subparsers(dest="command", required=True)
    f = commands.add_parser("freeze")
    f.add_argument("--baseline-binary", required=True)
    r = commands.add_parser("replay")
    r.add_argument("--binary", required=True)
    commands.add_parser("calibrate")
    commands.add_parser("finalize")
    commands.add_parser("orders")
    commands.add_parser("refine")
    opts = parser.parse_args()
    {"freeze": freeze, "replay": replay, "calibrate": calibrate, "finalize": finalize, "orders": orders, "refine": refine}[opts.command](opts)
