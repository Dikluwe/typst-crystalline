#!/usr/bin/env python3
"""Additive R5 repr/CBOR oracle: measured reference plus explicit fallback policy."""
import argparse
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("r5_frozen", HERE / "p1307-r5-content-measure.py")
r5 = importlib.util.module_from_spec(spec); spec.loader.exec_module(r5)
base = r5.base
NAME = "p1307-r5-repr-measurement.json"
SETTINGS = {"default": "", "en": '#set text(lang: "en")\n', "pt": '#set text(lang: "pt")\n', "numbering-1": '#set heading(numbering: "1")\n', "numbering-I": '#set heading(numbering: "I")\n'}

def byte_expr(value):
    return '{ let b = cbor.encode(' + value + '); range(b.len()).map(i => str(b.at(i))).join(",") }'

def corpus():
    cases = []
    predecessor = next(c for c in base.read("p1307-r4-oracle.json")["cases"] if c["id"] == "r2.construct-LocatedContent")
    for setting, prefix in SETTINGS.items():
        for projection, expr in {"repr": "repr(found.first())", "cbor-decoded": "repr(cbor(cbor.encode(found.first())))", "cbor-bytes": byte_expr("found.first()")}.items():
            doc = prefix + base.transport(expr)
            c = {"id": setting + "." + projection, "setting": setting, "projection": projection, "expression": expr, "route": "compile", "document": doc, "source_sha256": base.source_hash(doc), "expected_shape": "value"}
            if setting == "default" and projection == "repr":
                c.update(document=predecessor["document"], expression=predecessor["expression"], source_sha256=predecessor["source_sha256"], transport_marker="P1307R2:", predecessor_id=predecessor["id"])
            cases.append(c)
    for projection, expr in {"decoded": 'repr(cbor(cbor.encode("Probe α\\n")))', "bytes": byte_expr('"Probe α\\n"')}.items():
        cases.append({"id": "raw-text." + projection, "setting": "raw-text", "projection": projection, "expression": expr, "route": "eval", "source_sha256": base.source_hash(expr), "expected_shape": "value"})
    return cases

def provenance():
    p = r5.provenance()
    p.update(script_sha256=base.sha(__file__), authorization="Root additive instruction: write only p1307-r5-repr-{measure.py,measurement.json,oracle.json,note.md}; read/probe only, no candidate.", protected_inputs={n: base.sha(HERE / n) for n in ("p1307-r5-content-measure.py", "p1307-r5-content-measurement.json", "p1307-r5-content-oracle.json", "p1307-r5-content-note.md", "p1307-r4-oracle.json")})
    p["source_hashes"].update({n: base.sha(base.ROOT / n) for n in ("01_core/src/compiler/stdlib/loading.rs", "01_core/src/compiler/eval/repr.rs")})
    return p

def main():
    parser = argparse.ArgumentParser(); parser.add_argument("stage", choices=("focal", "bounded", "freeze")); args = parser.parse_args()
    r5.guard()
    data = base.read(NAME) if (HERE / NAME).exists() else {"schema": "p1307-r5-repr-supplement-measurement-v1", "candidate_read": False, "regime": "executado sem atestacao de isolamento tecnico", "binaries": r5.guard()}
    if args.stage == "focal" and "focal" in data:
        assert data["focal"]["issues"] and "focal_history" not in data, "Two focal revisions maximum"
        data["focal_history"] = [data.pop("focal")]
    assert args.stage not in data, "Preserve frozen stages and failed attempts"
    if args.stage != "freeze":
        cases = [c for c in corpus() if (c["setting"] in ("default", "raw-text")) == (args.stage == "focal")]
        if args.stage == "bounded":
            assert not data["focal"]["issues"]
        folder = base.fixtures(cases)
        rows = r5.measure(cases, ["default"])
        if args.stage == "bounded":
            controls = [c for c in corpus() if c["setting"] == "default"]
            base.fixtures(controls)
            rows += r5.measure(controls, [p for p in base.PROFILES if p != "default"])
            cases += controls
        data[args.stage] = {"provenance": provenance(), "cases": cases, "rows": rows, "fixture_directory": folder, "issues": base.issues(cases, rows), "finished": base.now()}
        base.save(NAME, data)
        print(json.dumps({"stage": args.stage, "runs": len(rows), "issues": data[args.stage]["issues"], "values": [{"case": r["case"], "side": r["side"], "profile": r["profile"], "value": r["observable"].get("value")} for r in rows]}, ensure_ascii=False)); return
    assert not data["focal"]["issues"] and not data["bounded"]["issues"]
    rows = data["focal"]["rows"] + data["bounded"]["rows"]
    cases = {c["id"]: c for part in (data["focal"], data["bounded"]) for c in part["cases"]}
    # Independent known-string controls prove exact bytes for the retained baseline CBOR
    # fallback. Their inputs are measured vanilla repr, never a candidate observation.
    literals = []
    for setting in SETTINGS:
        value = next(r["observable"]["value"] for r in rows if r["case"] == setting + ".repr" and r["side"] == "vanilla" and r["profile"] == "default")
        literal = json.dumps(value, ensure_ascii=False)
        for projection, expr in {"cbor-decoded": "repr(cbor(cbor.encode(" + literal + ")))", "cbor-bytes": byte_expr(literal)}.items():
            literals.append({"id": setting + ".literal-" + projection, "setting": setting, "projection": projection, "expression": expr, "route": "eval", "source_sha256": base.source_hash(expr), "expected_shape": "value", "input_origin": {"artifact": NAME, "case": setting + ".repr", "side": "vanilla", "profile": "default"}})
    lr = r5.measure(literals, ["default"])
    data["literal_controls"] = {"provenance": provenance(), "cases": literals, "rows": lr, "issues": base.issues(literals, lr), "finished": base.now()}
    assert not data["literal_controls"]["issues"]
    for c in literals:
        pair = [r["observable"] for r in lr if r["case"] == c["id"]]
        assert pair[0] == pair[1], "String control not bilateral: cannot derive fallback delta"
    for r in rows:
        if r["profile"] != "default":
            assert r["observable"] == next(x["observable"] for x in rows if x["case"] == r["case"] and x["side"] == r["side"] and x["profile"] == "default")
    historical = [r for h in data.get("focal_history", []) for r in h["rows"]]
    data["freeze"] = {"provenance": provenance(), "current_runs": len(rows) + len(lr), "historical_runs": len(historical), "total_runs": len(rows) + len(lr) + len(historical), "process_seconds": sum(r["elapsed_seconds"] for r in rows + lr + historical), "profile_control_stable": True, "revision_reason": "Requested cbor.decode does not exist in either binary: use existing cbor(bytes). repr(array-of-bytes) abbreviates long arrays; use per-byte decimal join to preserve every byte. Initial sources/outputs preserved in focal_history.", "hypothesis": "Vanilla realized repr omits label; baseline CBOR keeps Text(repr), so new exact fallback bytes are measured with a known String equal to vanilla repr. Do not substitute vanilla dedicated Content CBOR map as the baseline contract.", "finished": base.now()}
    base.save(NAME, data)
    output = []
    for key, c in cases.items():
        observations = {}
        for profile in dict.fromkeys(r["profile"] for r in rows if r["case"] == key):
            obs = {side: next(r["observable"] for r in rows if r["case"] == key and r["profile"] == profile and r["side"] == side) for side in ("vanilla", "baseline")}
            if c["setting"] != "raw-text" and c["projection"].startswith("cbor-"):
                control_id = c["setting"] + ".literal-" + c["projection"]
                expected = next(r["observable"] for r in lr if r["case"] == control_id and r["side"] == "baseline")
                policy = "baseline-Text-repr-fallback-preserved-with-measured-realized-repr-delta"
                ref = {"artifact": NAME, "collection": "literal_controls.rows", "case": control_id, "side": "baseline", "profile": "default", "profile_application": "Pure text CBOR expectation applied across four profiles; direct default Content CBOR profile controls were observed, not a general feature invariance claim."}
            else:
                expected, policy = obs["vanilla"], "ratified-vanilla-repr-or-shared-raw-text-control"
                ref = {"artifact": NAME, "collections": ["focal.rows", "bounded.rows"], "case": key, "side": "vanilla", "profile": profile}
            observations[profile] = {"vanilla_literal": obs["vanilla"], "baseline_literal": obs["baseline"], "future_expected": expected, "expectation_policy": policy, "expected_measurement_ref": ref}
        output.append(dict(c, observations=observations))
    obj = {"schema": "p1307-r5-repr-supplement-oracle-v1", "baseline_sha256": r5.PIN, "measurement_sha256": base.sha(HERE / NAME), "script_sha256": base.sha(__file__), "cases": output, "predecessor_mapping": {"r4_artifact": "p1307-r4-oracle.json", "r4_sha256": base.sha(HERE / "p1307-r4-oracle.json"), "r4_case": "r2.construct-LocatedContent", "successor_case": "default.repr", "same_source_sha256": cases["default.repr"]["source_sha256"], "old_policy": "baseline debt preserved", "new_proposed_policy": "ratified vanilla realized repr under R5 Content/introspection L0; activation requires owner-approved successor manifest, never modify predecessor"}, "regime": data["regime"], "candidate_read": False, "implementation_certified": False, "scope_out": "General CBOR Content map parity is not required; direct Content, symbols and other CBOR debt are not repaired by this supplement.", "finished": base.now()}
    assert not (HERE / "p1307-r5-repr-oracle.json").exists()
    base.save("p1307-r5-repr-oracle.json", obj)
    print(json.dumps({"runs": data["freeze"]["total_runs"], "cases": len(output), "measurement_sha256": obj["measurement_sha256"], "oracle_sha256": base.sha(HERE / "p1307-r5-repr-oracle.json"), "script_sha256": obj["script_sha256"]}))

if __name__ == "__main__": main()
