#!/usr/bin/env python3
"""R6 independent public-CLI acceptance oracle. No candidate-derived expectations."""
import argparse
import concurrent.futures
import copy
import importlib.util
import json
import re
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("r4_frozen", HERE / "p1307-r4-oracle.py")
base = importlib.util.module_from_spec(spec); spec.loader.exec_module(base)
classify = base.classify
PIN = "8cc0eae00457d2e7d54b420024eae49344032b34b295b4eda536faeb1ec3c4a3"
NAME = "p1307-r6-oracle-measurement.json"
INPUT_NAMES = ("p1307-r6-baseline.json", "p1307-r4-oracle.py", "p1307-r4-oracle.json", "p1307-r4-measurement.json", "p1307-r4-math-oracle.py", "p1307-r4-math-oracle.json", "p1307-r4-math-measurement.json", "p1307-r5-content-measure.py", "p1307-r5-content-measurement.json", "p1307-r5-content-oracle.json", "p1307-r5-content-note.md", "p1307-r5-repr-measure.py", "p1307-r5-repr-measurement.json", "p1307-r5-repr-oracle.json", "p1307-r5-repr-note.md", "p1307-r5-contract-report.md")

def guard():
    assert base.sha(HERE / "p1307-r6-baseline.json") == PIN
    assert base.sha(HERE / "p1307-r4-oracle.py") == "ef102f3a800475855b0cb21f40db666312cdb2f96fd9c867ea625b13c22b8e68"
    b = base.read("p1307-r6-baseline.json")["binaries"]
    for info in b.values(): assert base.sha(info["path"]) == info["sha256"]
    return {"vanilla": b["vanilla"], "baseline": b["crystalline"]}

def provenance():
    return {"at": base.now(), "baseline_sha256": PIN, "script_sha256": base.sha(__file__), "inputs": {n: base.sha(HERE / n) for n in INPUT_NAMES}, "head": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True, cwd=base.ROOT).strip(), "diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], text=True, cwd=base.ROOT), "environment": {"NO_COLOR": "1", "TERM": "dumb", "TYPST_FEATURES": "unset; explicit CLI profile"}}

def emit(expr):
    return '{ let raw = bytes(' + expr + '); panic("P1307R4:" + range(raw.len()).map(i => str(raw.at(i))).join(",") + ":END") }'

def corpus():
    cases = []
    def add(key, expr, prefix="", wrapper=None, classes=()):
        doc = prefix + (wrapper.replace("EMIT", emit(expr)) if wrapper else base.transport(expr))
        cases.append({"id": "r6." + key, "expression": expr, "route": "compile", "document": doc, "source_sha256": base.source_hash(doc), "classes": ["R6", "LocatedHeading", *classes], "expected_shape": "value", "mandatory": True, "policy": "vanilla"})
    for key, setting in {"default": "", "pt": '#set text(lang: "pt")\n', "numbering-I": '#set heading(numbering: "I")\n', "numbering-none": '#set heading(numbering: none)\n'}.items():
        add("fields-json." + key, "json.encode(found.first().fields())", setting, classes=("fields", "ordered-map", key))
    access = 'json.encode({ let x = found.first(); (label: (x.has("label"), x.at("label", default: "absent")), unknown: (x.has("missing"), x.at("missing", default: "absent")), fields: (x.at("level"), x.at("depth"), x.at("offset"), x.at("numbering"), x.at("supplement"), x.at("outlined"), x.at("bookmarked"), x.at("hanging-indent"), x.at("body")), func: repr(x.func()), location: repr(type(x.location()))) })'
    add("access.labelled", access, classes=("at", "has", "label", "func", "location"))
    add("access.unlabelled", access, wrapper='= Probe\n#context { let found = query(heading); if found.len() == 1 { EMIT } }\n', classes=("at", "has", "label-absent"))
    add("access.static-alias", 'json.encode({ let x = found.first(); let fields = content.fields; let at = content.at; let has = content.has; (fields(x), at(x, "label", default: "absent"), has(x, "numbering")) })', classes=("static", "alias", "fields", "at", "has"))
    add("access.static-alias-control", 'repr({ let x = found.first(); let fields = content.fields; let at = content.at; let has = content.has; (fields(x).len(), at(x, "label", default: "absent"), has(x, "numbering"), repr(type(fields))) })', classes=("static", "alias", "encoder-independent-control"))
    add("access.direct-fields", 'json.encode({ let x = found.first(); (x.level, x.depth, x.offset, x.numbering, x.supplement, x.outlined, x.bookmarked, x.hanging-indent, x.body, x.label) })', classes=("field-access", "label"))
    original = '#set heading(numbering: "1")\n#set text(lang: "en")\n= Probe <probe>\n'
    for name, changes in (("before", ""), ("after", 'set heading(numbering: "I"); set text(lang: "pt");')):
        wrapper = '#context { let found = query(<probe>); if found.len() == 1 { let saved = (found.first(),); let get() = saved.first(); { ' + changes + ' EMIT } } }\n'
        add("escape." + name, "json.encode(get())", original, wrapper, ("escape", "array", "closure", "causal-style"))
        add("escape.fields-" + name, "json.encode(get().fields())", original, wrapper, ("escape", "fields", "causal-style"))
    pair = '#set heading(numbering: "1")\n#set text(lang: "en")\n= Probe <left>\n#set heading(numbering: "I")\n#set text(lang: "pt")\n= Probe <right>\n'
    wrapper = '#context { let found = query(heading); if found.len() == 2 { EMIT } }\n'
    add("two-same-body", "json.encode(found)", pair, wrapper, ("same-body", "distinct-occurrence", "numbering", "language"))
    add("two-same-body.fields", "json.encode(found.map(x => x.fields()))", pair, wrapper, ("same-body", "fields", "distinct-occurrence"))
    lang_pair = '#set text(lang: "en")\n= Probe\n#set text(lang: "pt")\n= Probe\n'
    add("equality.language", "repr(found.at(0) == found.at(1))", lang_pair, wrapper, ("language-equality", "language", "same-body"))
    first = '#set heading(numbering: "I")\n#set text(lang: "pt")\n#context { let found = query(<probe>); if found.len() == 1 { EMIT } }\n= Probe <probe>\n'
    add("first-context-before-heading", "json.encode(found.first())", wrapper=first, classes=("first-context", "causal-style", "label"))
    add("first-context-before-heading.fields", "json.encode(found.first().fields())", wrapper=first, classes=("first-context", "fields"))
    return cases

def measure(cases, profiles):
    bins = guard()
    jobs = [(b["path"], side, c, p) for c in cases for p in profiles for side, b in bins.items()]
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool: return list(pool.map(lambda x: base.run(*x), jobs))

def focal():
    data = base.read(NAME) if (HERE / NAME).exists() else {"schema": "p1307-r6-independent-measurement-v1", "regime": "executado sem atestacao de isolamento tecnico", "candidate_read": False, "binaries": guard(), "attempts": []}
    assert len(data["attempts"]) < 2
    cases = [c for c in corpus() if c["id"] in {"r6.fields-json.default", "r6.access.static-alias", "r6.escape.after", "r6.two-same-body", "r6.first-context-before-heading", "r6.equality.language"}]
    folder = base.fixtures(cases); rows = measure(cases, ["default"])
    data["attempts"].append({"provenance": provenance(), "cases": cases, "rows": rows, "issues": base.issues(cases, rows), "fixture_directory": folder, "finished": base.now()})
    base.save(NAME, data)
    print(json.dumps({"runs": len(rows), "issues": data["attempts"][-1]["issues"], "rows": [{"case": r["case"], "side": r["side"], "observable": r["observable"]} for r in rows]}, ensure_ascii=False))

def bounded():
    data = base.read(NAME); assert data["attempts"] and not data["attempts"][-1]["issues"] and "bounded" not in data
    cases = corpus(); folder = base.fixtures(cases)
    known = {c["id"] for c in data["attempts"][-1]["cases"]}
    fresh = [c for c in cases if c["id"] not in known]
    rows = measure(fresh, ["default"])
    issues = base.issues(fresh, rows)
    data["extension_focal"] = {"provenance": provenance(), "cases": fresh, "rows": rows, "issues": issues, "finished": base.now()}
    base.save(NAME, data)
    if issues: print(json.dumps({"issues": issues})); return
    # Only the new bounded witness set, not the inherited global corpus.
    rows += measure(cases, [p for p in base.PROFILES if p != "default"])
    rows += data["attempts"][-1]["rows"]
    default = {(r["case"], r["side"]): r["observable"] for r in rows if r["profile"] == "default"}
    unstable = [{"case": r["case"], "side": r["side"], "profile": r["profile"]} for r in rows if r["observable"] != default[r["case"], r["side"]]]
    data["bounded"] = {"provenance": provenance(), "cases": cases, "rows": rows, "issues": base.issues(cases, rows), "profile_instability": unstable, "fixture_directory": folder, "finished": base.now()}
    base.save(NAME, data); print(json.dumps({"runs": len(rows), "issues": data["bounded"]["issues"], "profile_instability": unstable}))

def freeze():
    data = base.read(NAME); assert "bounded" in data and not data["bounded"]["issues"] and not data["bounded"]["profile_instability"]
    assert not (HERE / "p1307-r6-oracle.json").exists()
    r4, math = base.read("p1307-r4-oracle.json"), base.read("p1307-r4-math-oracle.json")
    result = copy.deepcopy(r4["cases"] + math["cases"])
    assert len(r4["cases"]) == 455 and len(math["cases"]) == 11
    for c in result:
        parent = "p1307-r4-math-oracle.json" if c in math["cases"] else "p1307-r4-oracle.json"
        c["inherited_oracle_ref"] = {"artifact": parent, "sha256": base.sha(HERE / parent), "case": c["id"]}
        for p, cell in c["observations"].items():
            ref = dict(c["measurement_ref"], profile=p, side=cell["expected_side"])
            cell["expected_measurement_ref"] = ref
    repr5 = base.read("p1307-r5-repr-oracle.json")
    replacement = next(c for c in repr5["cases"] if c["id"] == "default.repr")
    target = next(c for c in result if c["id"] == "r2.construct-LocatedContent")
    assert target["source_sha256"] == replacement["source_sha256"]
    old_observations = copy.deepcopy(target["observations"])
    target.update(observations=copy.deepcopy(replacement["observations"]), policy="vanilla", supersession={"successor_artifact": "p1307-r5-repr-oracle.json", "successor_case": "default.repr", "reason": "Approved R5 realized Heading repr, same source; sole R4 supersession", "prior_observations": old_observations})
    target.pop("scope_out", None)
    for old in repr5["cases"]:
        c = copy.deepcopy(old); c.update(id="r5repr." + old["id"], mandatory=True, origin_oracle={"artifact": "p1307-r5-repr-oracle.json", "case": old["id"]})
        c["measurement_ref"] = {"artifact": "p1307-r5-repr-measurement.json", "collections": ["focal.rows", "bounded.rows"], "case": old["id"]}
        result.append(c)
    content5 = base.read("p1307-r5-content-oracle.json")
    selected = {s + ".json" for s in ("query-default", "numbering-1", "numbering-I", "numbering-none", "language-en", "language-pt")}
    selected |= {"equality." + s for s in ("inline-vs-query", "clone-through-array", "different-labels", "no-labels", "same-labels", "different-numbering")}
    selected |= {"access.error-label", "access.error-missing"}
    selection = []
    for old in content5["cases"]:
        active = old["id"] in selected
        if active:
            reason = "Approved R5 supported queried fields/JSON, language equality or exact access diagnostic"
        elif old["id"].startswith("direct-"):
            reason = "Raw constructor/sequence observation retained historically; existing R4 direct Content obligations and debt policies remain authoritative, no new general raw repr parity"
        elif old["scenario"] in {"numbering-callback", "supplement-auto", "supplement-none", "supplement-Alpha", "supplement-Beta", "supplement-callback", "set-level", "set-depth-markup", "set-depth-constructor", "set-offset", "explicit-depth-offset", "outlined-false", "bookmarked-true", "bookmarked-false", "hanging-indent"}:
            reason = "Producer/style/callback expansion not approved as new R5 obligation; original observation preserved, not counted as parity; auto default is covered separately by query-default"
        elif old["scenario"].startswith("frozen-") or old["scenario"] == "two-styles":
            reason = "Original witness combines unsupported supplement/offset with transport; retain evidence, replace acceptance witness with R6 numbering/language-only escape or same-body case"
        else:
            reason = "Composite repr(Dict) or repr(array-of-Dict) formatter debt remains out of scope; R6 JSON fields/access witnesses compare complete structured data without normalizing this string"
        selection.append({"artifact": "p1307-r5-content-oracle.json", "case": old["id"], "selected": active, "reason": reason})
        if not active: continue
        c = copy.deepcopy(old); c.update(id="r5content." + old["id"], mandatory=True, policy="vanilla", observations={})
        for p, pair in old["observations"].items():
            cell = base.cell(pair["vanilla"], pair["baseline"], "vanilla")
            cell["expected_measurement_ref"] = {"artifact": "p1307-r5-content-measurement.json", "collections": ["focal.rows", "bounded.rows"], "case": old["id"], "profile": p, "side": "vanilla"}
            c["observations"][p] = cell
        result.append(c)
    lookup = {(r["case"], r["side"], r["profile"]): r for r in data["bounded"]["rows"]}
    for old in data["bounded"]["cases"]:
        c = copy.deepcopy(old); c["observations"] = {}
        c["measurement_ref"] = {"artifact": NAME, "collection": "bounded.rows", "case": c["id"]}
        for p in base.PROFILES:
            cell = base.cell(lookup[c["id"], "vanilla", p]["observable"], lookup[c["id"], "baseline", p]["observable"], "vanilla")
            cell["expected_measurement_ref"] = {"artifact": NAME, "collection": "bounded.rows", "case": c["id"], "profile": p, "side": "vanilla"}
            c["observations"][p] = cell
        result.append(c)
    assert len({c["id"] for c in result}) == len(result)
    for c in result:
        for p, cell in c["observations"].items():
            ref = cell["expected_measurement_ref"]
            ref.setdefault("case_key", "case")
            ref.setdefault("case_id", ref.get("case", c["id"]))
            ref.setdefault("source_case_id", ref["case_id"])
            ref.setdefault("profile_key", "profile")
            ref.setdefault("profile", p)
            ref.setdefault("side_key", "side")
    assert all(classify(o["future_expected"], o["future_expected"]) == "Preserved" for c in result for o in c["observations"].values())
    inputs = {n: base.sha(HERE / n) for n in INPUT_NAMES}; inputs[NAME] = base.sha(HERE / NAME)
    inputs.update(r4["inputs"]); inputs.update(math["inputs"])
    out = {"schema": "p1307-r6-consolidated-public-oracle-v1", "regime": "executado sem atestacao de isolamento tecnico", "executor": "/root/p1307_oracle", "candidate_read": False, "baseline_sha256": PIN, "script_sha256": base.sha(__file__), "inputs": inputs, "at": base.now(), "profiles": list(base.PROFILES), "profile_policy": "Replay only profiles actually present per case; no unmeasured profile expectations invented", "cases": result, "r4_supersessions": ["r2.construct-LocatedContent"], "r5_content_selection": selection, "historical_unknown_policy": "R5 nested context and failed adapter attempts remain historical Unknown/precondition failures, never promoted; required current observations contain no Unknown", "comparison": "classify(expected,observed) delegates frozen R4 full envelope classifier; no sorting, roundtrip-only, repr-trimming or payload normalization", "attack_obligations": ["snapshot missing one known field", "label inserted as none or selector-derived", "field order sorted", "func/location inserted into snapshot fields", "snapshot recomputed from consumer style", "same body occurrences aliased", "array/closure carrier lost", "static methods drop snapshot", "first context sees raw fallback", "language equality includes label/location", "language equality ignores numbering/supplement", "CBOR replaced with vanilla map instead of retained text fallback", "CBOR bytes last-byte mutation", "diagnostic origin/range/message/hints changed"], "source_mutants_executed": False, "mutation_score": None, "limitations": ["No candidate read, implementation certificate or source mutation execution", "R5 callbacks/custom supplement/offset/unsupported producer fields and general repr(Dict) are out of this extension", "No global inherited corpus rerun before seal; predecessor raw data and orders preserved", "CBOR text fallback expectations derive from independently measured literal controls, not direct vanilla Content maps"]}
    base.save("p1307-r6-oracle.json", out)
    print(json.dumps({"cases": len(result), "cells": sum(len(c["observations"]) for c in result), "selected_r5content": len(selected), "new_witnesses": len(data["bounded"]["cases"]), "sha256": base.sha(HERE / "p1307-r6-oracle.json"), "measurement_sha256": inputs[NAME], "script_sha256": out["script_sha256"]}))

def replay(args):
    oracle = base.read("p1307-r6-oracle.json")
    assert base.sha(__file__) == oracle["script_sha256"]
    for n, sha in oracle["inputs"].items(): assert base.sha(HERE / n) == sha, "Protected input changed: " + n
    cases = [copy.deepcopy(c) for c in oracle["cases"] if not args.case or re.search(args.case, c["id"])]
    if args.reverse: cases.reverse()
    folder = base.fixtures(cases); byid = {c["id"]: c for c in cases}
    jobs = [(args.binary, "candidate", c, p) for c in cases for p in c["observations"] if not args.profile or p == args.profile]
    rows = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        for row in pool.map(lambda x: base.run(*x), jobs):
            row["verdict"] = classify(byid[row["case"]]["observations"][row["profile"]]["future_expected"], row["observable"]); rows.append(row)
    print(json.dumps({"schema": "p1307-r6-replay-v1", "at": base.now(), "binary": args.binary, "binary_sha256": base.sha(args.binary), "oracle_sha256": base.sha(HERE / "p1307-r6-oracle.json"), "fixture_directory": folder, "rows": rows, "counts": {v: sum(r["verdict"] == v for r in rows) for v in ("Preserved", "Violated", "Unknown")}}, ensure_ascii=False))

if __name__ == "__main__":
    parser = argparse.ArgumentParser(); parser.add_argument("command", choices=("focal", "bounded", "freeze", "replay")); parser.add_argument("--binary"); parser.add_argument("--case"); parser.add_argument("--profile", choices=list(base.PROFILES)); parser.add_argument("--reverse", action="store_true"); args = parser.parse_args()
    if args.command == "replay": replay(args)
    else: {"focal": focal, "bounded": bounded, "freeze": freeze}[args.command]()
