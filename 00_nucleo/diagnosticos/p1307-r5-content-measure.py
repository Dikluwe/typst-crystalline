#!/usr/bin/env python3
"""Independent bounded Heading observations, no candidate or production writes."""
import argparse
import concurrent.futures
import importlib.util
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("p1307_frozen", HERE / "p1307-r4-oracle.py")
base = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(base)
PIN = "32bae26c9d5175cb4567a6c0b4c1cbae17e8bb0818466879182936fd473d5d7a"
RUNNER = "ef102f3a800475855b0cb21f40db666312cdb2f96fd9c867ea625b13c22b8e68"
NAME = "p1307-r5-content-measurement.json"

def emit(expr):
    return '{ let raw = bytes(' + expr + '); panic("P1307R4:" + range(raw.len()).map(i => str(raw.at(i))).join(",") + ":END") }'

def corpus():
    cases = []
    def add(key, expr, prefix="", value="found.first()", direct=False, wrapper=None, classes=None):
        for projection in ("fields", "json"):
            expression = "repr(" + value + ".fields())" if projection == "fields" else "json.encode(" + value + ")"
            if direct:
                document = None
            elif wrapper:
                document = prefix + wrapper.replace("EMIT", emit(expression))
            else:
                document = prefix + base.transport(expression)
            case = {"id": key + "." + projection, "scenario": key, "projection": projection, "expression": expression, "route": "eval" if direct else "compile", "classes": classes or ["Heading", key, projection], "expected_shape": "value"}
            if document is not None:
                case["document"] = document
            case["source_sha256"] = base.source_hash(document if document is not None else expression)
            cases.append(case)
    add("direct-default", "", value="heading[Probe]", direct=True)
    add("direct-explicit", "", value='heading(depth: 2, offset: 1, numbering: "I", supplement: [Alpha], outlined: false, bookmarked: true, hanging-indent: 10pt)[Probe]', direct=True)
    add("direct-markup-label", "", value="[= Probe <probe>]", direct=True)
    settings = {
        "query-default": "", "set-level": "#set heading(level: 4)\n",
        "set-depth-markup": "#set heading(depth: 3)\n", "set-offset": "#set heading(offset: 2)\n",
        "numbering-1": '#set heading(numbering: "1")\n', "numbering-I": '#set heading(numbering: "I")\n',
        "numbering-none": "#set heading(numbering: none)\n", "numbering-callback": '#set heading(numbering: (..nums) => "N")\n',
        "supplement-auto": "#set heading(supplement: auto)\n", "supplement-none": "#set heading(supplement: none)\n",
        "supplement-Alpha": "#set heading(supplement: [Alpha])\n", "supplement-Beta": "#set heading(supplement: [Beta])\n",
        "supplement-callback": '#set heading(supplement: it => [Callback])\n',
        "language-en": '#set text(lang: "en")\n', "language-pt": '#set text(lang: "pt")\n',
        "outlined-false": "#set heading(outlined: false)\n", "bookmarked-true": "#set heading(bookmarked: true)\n",
        "bookmarked-false": "#set heading(bookmarked: false)\n", "hanging-indent": "#set heading(hanging-indent: 10pt)\n",
    }
    for key, setting in settings.items():
        add(key, "", prefix=setting)
    add("set-depth-constructor", "", prefix="#set heading(depth: 3)\n", wrapper="#heading[Probe] <probe>\n#context { let found = query(<probe>); if found.len() > 0 { EMIT } }\n")
    add("explicit-depth-offset", "", wrapper="#heading(depth: 2, offset: 1)[Probe] <probe>\n#context { let found = query(<probe>); if found.len() > 0 { EMIT } }\n")
    original = '#set heading(numbering: "1", supplement: [Alpha], offset: 1)\n= Probe <probe>\n'
    for key, inner in (("frozen-array-before", ""), ("frozen-array-after", '#set heading(numbering: "I", supplement: [Beta], offset: 3)\n#set text(lang: "pt")\n')):
        # A captured Content Value crosses an array, a closure and a new contextual node.
        code_inner = inner.replace("#set", "set").replace("\n", "; ")
        add(key, "", value="get().first()", prefix=original,
            wrapper='#context { let found = query(<probe>); if found.len() > 0 { let saved = (found.first(),); let get() = saved; { ' + code_inner + ' EMIT } } }\n')
    pair_prefix = '#set heading(numbering: "1", supplement: [Alpha], offset: 1)\n= Probe <left>\n#set heading(numbering: "I", supplement: [Beta], offset: 2)\n= Probe <right>\n'
    for projection in ("fields", "json"):
        expression = "repr(found.map(x => x.fields()))" if projection == "fields" else "json.encode(found)"
        doc = pair_prefix + '#context { let found = query(heading); if found.len() == 2 { ' + emit(expression) + ' } }\n'
        cases.append({"id": "two-styles." + projection, "scenario": "two-styles", "projection": projection, "expression": expression, "document": doc, "route": "compile", "source_sha256": base.source_hash(doc), "expected_shape": "value", "classes": ["Heading", "frozen", "distinct-styles"]})
    equality = {
        "inline-vs-query": ('= Probe <probe>\n', 'let found = query(<probe>); if found.len() == 1', 'repr(heading[Probe] == found.first())'),
        "clone-through-array": ('= Probe <probe>\n', 'let found = query(<probe>); if found.len() == 1', 'repr({ let a = (found.first(),); let get() = a.first(); found.first() == get() })'),
        "different-labels": ('= Probe <left>\n= Probe <right>\n', 'let found = query(heading); if found.len() == 2', 'repr(found.at(0) == found.at(1))'),
        "no-labels": ('= Probe\n= Probe\n', 'let found = query(heading); if found.len() == 2', 'repr(found.at(0) == found.at(1))'),
        "same-labels": ('= Probe <same>\n= Probe <same>\n', 'let found = query(heading); if found.len() == 2', 'repr(found.at(0) == found.at(1))'),
        "different-numbering": ('#set heading(numbering: "1")\n= Probe\n#set heading(numbering: "I")\n= Probe\n', 'let found = query(heading); if found.len() == 2', 'repr(found.at(0) == found.at(1))'),
    }
    for key, (prefix, guard, expression) in equality.items():
        doc = prefix + '#context { ' + guard + ' { ' + emit(expression) + ' } }\n'
        cases.append({"id": "equality." + key, "scenario": "equality." + key, "projection": "equality", "expression": expression, "document": doc, "route": "compile", "source_sha256": base.source_hash(doc), "expected_shape": "value", "classes": ["Heading", "language-equality", key]})
    for key, heading in (("unlabelled", "= Probe\n"), ("labelled", "= Probe <probe>\n")):
        expression = 'repr({ let x = found.first(); (label: (x.has("label"), x.at("label", default: "absent")), unknown: (x.has("missing"), x.at("missing", default: "absent")), known: (x.has("numbering"), x.at("numbering", default: "absent"), x.has("supplement"), x.at("supplement", default: "absent"), x.has("body"), x.at("body"), repr(x.func()))) })'
        doc = heading + '#context { let found = query(heading); if found.len() == 1 { ' + emit(expression) + ' } }\n'
        cases.append({"id": "access." + key, "scenario": "access." + key, "projection": "access", "expression": expression, "document": doc, "route": "compile", "source_sha256": base.source_hash(doc), "expected_shape": "value", "classes": ["Heading", "at", "has", "label", "func"]})
    for field in ("label", "missing"):
        expression = 'repr(found.first().at("' + field + '"))'
        doc = '= Probe\n#context { let found = query(heading); if found.len() == 1 { ' + emit(expression) + ' } }\n'
        cases.append({"id": "access.error-" + field, "scenario": "access.error-" + field, "projection": "diagnostic", "expression": expression, "document": doc, "route": "compile", "source_sha256": base.source_hash(doc), "expected_shape": "diagnostic", "classes": ["Heading", "at", "missing-field"]})
    return cases

def guard():
    assert base.sha(HERE / "p1307-r5-baseline.json") == PIN
    assert base.sha(HERE / "p1307-r4-oracle.py") == RUNNER
    data = base.read("p1307-r5-baseline.json")["binaries"]
    bins = {"vanilla": data["vanilla"], "baseline": data["crystalline"]}
    for b in bins.values():
        assert base.sha(b["path"]) == b["sha256"]
    return bins

def provenance():
    sources = ["lab/typst-original/crates/typst-library/src/model/heading.rs", "lab/typst-original/crates/typst-library/src/foundations/content/mod.rs", "lab/typst-original/crates/typst-library/src/foundations/content/raw.rs", "01_core/src/entities/elements/heading.rs", "01_core/src/compiler/eval/operators/equality.rs"]
    return {"at": base.now(), "baseline_sha256": PIN, "runner_sha256": RUNNER, "script_sha256": base.sha(__file__), "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=base.ROOT, text=True).strip(), "diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=base.ROOT, text=True), "source_hashes": {s: base.sha(base.ROOT / s) for s in sources}, "environment": {"NO_COLOR": "1", "TERM": "dumb", "TYPST_FEATURES": "unset; explicit CLI profiles"}}

def measure(cases, profiles):
    bins = guard()
    jobs = [(b["path"], side, c, p) for c in cases for p in profiles for side, b in bins.items()]
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        return list(pool.map(lambda x: base.run(*x), jobs))

def main():
    parser = argparse.ArgumentParser(); parser.add_argument("stage", choices=["focal", "bounded", "freeze"]); args = parser.parse_args()
    guard()
    if args.stage == "freeze":
        data = base.read(NAME)
        assert "bounded" in data
        assert not data["focal"]["unknowns"] and not data["bounded"]["unknowns"]
        assert not data["focal"]["vanilla_precondition_failures"] and not data["bounded"]["vanilla_precondition_failures"]
        assert "finalization" not in data, "measurement already frozen"
        rows = data["focal"]["rows"] + data["bounded"]["rows"]
        controls = [r for r in rows if r["profile"] != "default"]
        assert all(r["observable"] == next(x["observable"] for x in rows if x["case"] == r["case"] and x["side"] == r["side"] and x["profile"] == "default") for r in controls)
        history_rows = [r for part in data.get("focal_history", []) for r in part["rows"]]
        data["finalization"] = {"provenance": provenance(), "current_runs": len(rows), "historical_runs": len(history_rows), "total_runs": len(rows) + len(history_rows), "historical_unknowns": [r for r in history_rows if r["observable"]["kind"] == "Unknown"], "profile_controls_stable": True, "summed_process_seconds": sum(r["elapsed_seconds"] for r in rows + history_rows), "focal_revision_reason": "Revision 1's captured array/closure into nested context ran on vanilla but baseline returned exit 0 without transport marker. Preserve as unobserved nested-context boundary. Revision 2 observes the same captured Value after a new style block in the existing context, not a claim to fix nested context.", "scope": "Bounded default-profile contract input with five cases controlled in four feature profiles; PDF target only, no HTML-target claim. No full repeated matrix, candidate, test or implementation certificate.", "predecessor_hashes": {name: base.sha(HERE / name) for name in ("p1307-r4-content-observability.py", "p1307-r4-content-observability.json", "p1307-r4-oracle.py", "p1307-r4-oracle.json", "p1307-r4-measurement.json", "p1307-r4-math-oracle.py", "p1307-r4-math-oracle.json", "p1307-r4-math-measurement.json")}}
        base.save(NAME, data)
        cases = {c["id"]: c for part in (data["focal"], data["bounded"]) for c in part["cases"]}
        result = []
        for key, c in cases.items():
            obs = {p: {side: next(r["observable"] for r in rows if r["case"] == key and r["profile"] == p and r["side"] == side) for side in ("vanilla", "baseline")} for p in dict.fromkeys(r["profile"] for r in rows if r["case"] == key)}
            c = dict(c, observations=obs, measurement_ref={"artifact": NAME, "collections": ["focal.rows", "bounded.rows"], "case": key, "profile_key": "profile", "side_key": "side"}, interpretation="Measured contract input, not automatically an implementation obligation. Callback/phase and equality boundaries require owner classification.")
            result.append(c)
        out = {"schema": "p1307-r5-heading-contract-input-v1", "regime": "executado sem atestacao de isolamento tecnico", "baseline_sha256": PIN, "measurement_sha256": base.sha(HERE / NAME), "script_sha256": base.sha(__file__), "cases": result, "candidate_read": False, "implementation_certified": False, "classify_api": "Frozen p1307-r4-oracle.py classify(expected, observed); full literal envelopes, no sorting/payload normalization", "finished": base.now()}
        assert not (HERE / "p1307-r5-content-oracle.json").exists()
        base.save("p1307-r5-content-oracle.json", out)
        print(json.dumps({"cases": len(result), "oracle_sha256": base.sha(HERE / "p1307-r5-content-oracle.json"), "measurement_sha256": base.sha(HERE / NAME)})); return
    data = base.read(NAME) if (HERE / NAME).exists() else {"schema": "p1307-r5-heading-bounded-measurement-v1", "regime": "executado sem atestacao de isolamento tecnico", "candidate_read": False, "binaries": guard()}
    if args.stage == "focal" and "focal" in data:
        assert data["focal"]["unknowns"] and "focal_history" not in data, "two-attempt limit"
        data["focal_history"] = [data.pop("focal")]
    assert args.stage not in data, "preserve prior attempt"
    all_cases = corpus()
    focal_names = {"direct-default", "query-default", "numbering-callback", "supplement-callback", "frozen-array-after", "equality.different-labels", "equality.no-labels"}
    selected = [c for c in all_cases if (c["scenario"] in focal_names) == (args.stage == "focal")]
    if args.stage == "bounded":
        assert data["focal"] and not data["focal"]["unknowns"] and not data["focal"]["vanilla_precondition_failures"]
    folder = base.fixtures(selected)
    rows = measure(selected, ["default"])
    if args.stage == "bounded":
        controls = [dict(c) for c in all_cases if c["id"] in {"query-default.fields", "query-default.json", "frozen-array-after.fields", "frozen-array-after.json", "equality.no-labels"}]
        base.fixtures(controls)
        rows += measure(controls, [p for p in base.PROFILES if p != "default"])
        selected += controls
    unknowns = [{"case": r["case"], "side": r["side"], "reason": r["observable"].get("reason")} for r in rows if r["observable"]["kind"] == "Unknown"]
    shapes = {c["id"]: c["expected_shape"] for c in selected}
    failures = [{"case": r["case"], "observable": r["observable"]} for r in rows if r["side"] == "vanilla" and r["observable"]["kind"] != shapes[r["case"]]]
    data[args.stage] = {"provenance": provenance(), "fixture_directory": folder, "cases": selected, "rows": rows, "unknowns": unknowns, "vanilla_precondition_failures": failures, "finished": base.now()}
    base.save(NAME, data)
    print(json.dumps({"stage": args.stage, "runs": len(rows), "unknowns": unknowns, "vanilla_precondition_failures": failures, "observations": [{k: r[k] for k in ("case", "side", "profile", "observable")} for r in rows]}, ensure_ascii=False))

if __name__ == "__main__": main()
