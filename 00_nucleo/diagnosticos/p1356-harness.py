#!/usr/bin/env python3
"""P1356 fail-closed parity harness.

Raw process observations are confined to a caller supplied directory below /tmp.
Committed inputs and product binaries are read-only.  This program deliberately
does not interpret rendered bytes as a language observable.
"""

from __future__ import annotations

import argparse
import concurrent.futures
import copy
import hashlib
from html.parser import HTMLParser
import json
import os
from pathlib import Path
import subprocess
import struct
import sys
import tempfile
from typing import Any
import xml.etree.ElementTree as ET


ROOT = Path(__file__).resolve().parents[2]
PROFILE_ORDER = ["default", "html", "a11y-extras", "html+a11y"]
PROFILE_FEATURES = {
    "default": [],
    "html": ["html"],
    "a11y-extras": ["a11y-extras"],
    "html+a11y": ["html", "a11y-extras"],
}
CLASS_ORDER = [
    "MATCH_VALUE", "MATCH_DIAGNOSTIC", "VANILLA_ONLY", "CRYSTALLINE_ONLY",
    "DIFFERENT_VALUE", "DIFFERENT_DIAGNOSTIC", "UNKNOWN",
]
REQUIRED_ROUTES = [
    "format", "format.pdf", "pdf", "format.html", "html", "format.svg",
    "svg", "format.png", "png", "format.bundle", "asset",
]
ALIASES = {"pdf": "format.pdf", "html": "format.html", "svg": "format.svg",
           "png": "format.png", "asset": "format.bundle"}
UNKNOWN_REASONS = {
    "AMBIGUOUS_PRODUCT_IDENTITY", "SOURCE_MISSING", "TIMEOUT",
    "PARSER_UNSUPPORTED", "OBSERVER_INCOMPLETE", "ENVIRONMENT_INVALID",
    "BUDGET_EXHAUSTED",
}


def sha_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha_file(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True,
                      separators=(",", ":")).encode("utf-8")


def load(path: str | Path) -> Any:
    with Path(path).open(encoding="utf-8") as stream:
        return json.load(stream)


def emit(value: Any, output: str | None = None) -> None:
    data = canonical(value) + b"\n"
    if output:
        Path(output).write_bytes(data)
    else:
        sys.stdout.buffer.write(data)


def fail(message: str) -> None:
    raise ValueError(message)


def profile_index(profile: str) -> int:
    try:
        return PROFILE_ORDER.index(profile)
    except ValueError:
        return len(PROFILE_ORDER)


def cell_key(cell: dict[str, Any]) -> tuple[str, int]:
    return (cell["id"], profile_index(cell["profile"]))


def fixture_verdict(fixture: dict[str, Any]) -> tuple[str, str | None]:
    data = fixture.get("input", {})
    if fixture.get("id", "").startswith("O"):
        reason = fixture.get("reason_code")
        if data.get("complete") is False and reason in UNKNOWN_REASONS:
            return "Unknown", reason
        return "Violated", "OPAQUE_FIXTURE_MALFORMED"
    left, right = data.get("left"), data.get("right")
    if left is not None or right is not None:
        if not left or not right or not left.get("complete") or not right.get("complete"):
            return "Unknown", "OBSERVER_INCOMPLETE"
        same = (left.get("outcome_kind"), left.get("public_observable")) == (
            right.get("outcome_kind"), right.get("public_observable"))
        return ("Preserved", None) if same else ("Violated", "OBSERVABLE_DIFF")
    if data.get("bilateral_feature_vectors_equal") is False:
        return "Violated", "ASYMMETRIC_PROFILE"
    if data.get("left_features") != data.get("right_features") and "left_features" in data:
        return "Violated", "ASYMMETRIC_PROFILE"
    if data.get("declared_render_bytes_as_public_observable") is False:
        same = data.get("left_public_morphology") == data.get("right_public_morphology")
        return ("Preserved", None) if same else ("Violated", "MORPHOLOGY_DIFF")
    return "Preserved", None


def base_gate_model(constants: dict[str, Any]) -> dict[str, Any]:
    routes = {p: {"path": p, "provenance": "required_seed"} for p in REQUIRED_ROUTES}
    for alias in ALIASES:
        routes[alias]["provenance"] = "prelude_path_alias"
    cells = []
    for ident in ["a", "b"]:
        for profile in PROFILE_ORDER:
            side = {"complete": True, "outcome_kind": "VALUE", "reason_code": None,
                    "canonical_observable_sha256": sha_bytes(b"same"),
                    "rendered_byte_sha256": "render-a", "features": PROFILE_FEATURES[profile]}
            cells.append({"id": ident, "path": ident, "profile": profile,
                          "class": "MATCH_VALUE", "left": copy.deepcopy(side),
                          "right": {**copy.deepcopy(side), "rendered_byte_sha256": "render-b"},
                          "p1353_historical_class": "DIFFERENT_VALUE"})
    return {
        "baseline": {"vanilla": {"commit": constants["vanilla_commit"],
                     "binary_sha256": constants["vanilla_binary_sha256"]}},
        "catalog": routes,
        "sentinels": {"html": {"kind": "function"}, "pdf": {"kind": "function"}},
        "cells": cells,
        "reverse_cells": copy.deepcopy(cells),
        "comparison_catalogs": {name: sorted(routes) for name in
            ["current_crystalline_vs_historical_snapshot", "current_crystalline_vs_vanilla",
             "historical_snapshot_vs_vanilla"]},
        "delta": {"catalog_intersection": ["a"], "catalog_entries": ["b"],
                  "catalog_exits": ["c"]},
        "attributions": [{"path": "a", "attribution": "TARGET_DRIFT", "inferred": True,
                          "measured_premises": ["crystalline_snapshot=MATCH_VALUE",
                                                "snapshot_vanilla=DIFFERENT_VALUE"],
                          "refuting_observation": "snapshot_vanilla becomes MATCH_VALUE",
                          "comparator_can_represent": True}],
    }


def derived_class(cell: dict[str, Any]) -> str:
    left, right = cell["left"], cell["right"]
    if not left.get("complete") or not right.get("complete"):
        return "UNKNOWN"
    lk, rk = left.get("outcome_kind"), right.get("outcome_kind")
    lo, ro = left.get("canonical_observable_sha256"), right.get("canonical_observable_sha256")
    if lk == rk == "VALUE":
        return "MATCH_VALUE" if lo == ro else "DIFFERENT_VALUE"
    if lk == rk == "DIAGNOSTIC":
        return "MATCH_DIAGNOSTIC" if lo == ro else "DIFFERENT_DIAGNOSTIC"
    if lk == "VALUE" and rk == "DIAGNOSTIC" and right.get("absence_witness"):
        return "VANILLA_ONLY"
    if rk == "VALUE" and lk == "DIAGNOSTIC" and left.get("absence_witness"):
        return "CRYSTALLINE_ONLY"
    return "DIFFERENT_DIAGNOSTIC"


def model_violations(model: dict[str, Any], constants: dict[str, Any]) -> list[str]:
    issues = []
    vanilla = model["baseline"]["vanilla"]
    if vanilla["commit"] != constants["vanilla_commit"]: issues.append("VANILLA_PIN_MISMATCH")
    if vanilla["binary_sha256"] != constants["vanilla_binary_sha256"]: issues.append("VANILLA_SHA_MISMATCH")
    missing = sorted(set(REQUIRED_ROUTES) - set(model["catalog"]))
    if missing: issues.append("REQUIRED_ROUTES_MISSING:" + ",".join(missing))
    for alias in ALIASES:
        if alias not in model["catalog"] or model["catalog"][alias].get("provenance") != "prelude_path_alias":
            issues.append("PRELUDE_ALIAS_MISSING:" + alias)
    if model["sentinels"]["html"]["kind"] != "function": issues.append("HTML_KIND_STALE")
    expected = sorted(model["cells"], key=cell_key)
    if model["cells"] != expected: issues.append("CELL_ORDER_NONCANONICAL")
    if model["reverse_cells"] != sorted(model["reverse_cells"], key=cell_key):
        issues.append("REVERSE_ORDER_NONCANONICAL")
    expected_ids = {(c["id"], p) for c in model["cells"] for p in PROFILE_ORDER}
    actual_ids = {(c["id"], c["profile"]) for c in model["cells"]}
    if expected_ids != actual_ids: issues.append("MISSING_PROFILE_CELL")
    for cell in model["cells"]:
        if cell["left"].get("features") != PROFILE_FEATURES[cell["profile"]] or \
           cell["right"].get("features") != PROFILE_FEATURES[cell["profile"]]:
            issues.append("ASYMMETRIC_PROFILE")
        if cell["class"] != derived_class(cell): issues.append("CLASS_NOT_DERIVED:" + cell["id"])
    catalogs = list(model["comparison_catalogs"].values())
    if any(x != catalogs[0] for x in catalogs[1:]): issues.append("MIXED_COMPARISON_CATALOG")
    if not all(k in model["delta"] for k in ["catalog_intersection", "catalog_entries", "catalog_exits"]):
        issues.append("RAW_TOTAL_DELTA")
    for item in model["attributions"]:
        if not item.get("measured_premises") or not item.get("refuting_observation") or not item.get("inferred"):
            issues.append("UNSUPPORTED_ATTRIBUTION")
        if not item.get("comparator_can_represent", True) and item["attribution"] != "HARNESS_GAP":
            issues.append("HARNESS_GAP_MISCLASSIFIED")
    return sorted(set(issues))


def inject(model: dict[str, Any], mutant: dict[str, Any]) -> None:
    mid = mutant["id"]
    if mid == "M01_OLD_PIN": model["baseline"]["vanilla"]["commit"] = mutant["deterministic_injection"]["to"]
    elif mid == "M02_WRONG_VANILLA_SHA": model["baseline"]["vanilla"]["binary_sha256"] = mutant["deterministic_injection"]["to"]
    elif mid == "M03_ORDER_SENSITIVE_PAYLOAD": model["reverse_cells"].reverse()
    elif mid == "M04_ASYMMETRIC_PROFILE": next(c for c in model["cells"] if c["profile"] == "html+a11y")["right"]["features"] = ["html"]
    elif mid == "M05_OMIT_PRELUDE_ALIAS": del model["catalog"][sorted(ALIASES)[0]]
    elif mid == "M06_STALE_PUBLIC_KIND": model["sentinels"]["html"]["kind"] = "module"
    elif mid == "M07_UNKNOWN_TO_MATCH":
        c=model["cells"][0]; c["left"].update(complete=False,reason_code="TIMEOUT",outcome_kind="UNKNOWN"); c["class"]="MATCH_VALUE"
    elif mid == "M08_RENDER_BYTES_AS_LANGUAGE": model["cells"][0]["class"] = "DIFFERENT_VALUE"
    elif mid == "M09_DROP_REQUIRED_SEED_ROUTE": del model["catalog"]["format.bundle"]
    elif mid == "M10_ALIAS_COLLAPSE": del model["catalog"]["asset"]
    elif mid == "M11_MISSING_PROFILE_CELL": model["cells"] = [c for c in model["cells"] if not (c["id"] == "a" and c["profile"] == "html+a11y")]
    elif mid == "M12_HISTORICAL_CLASS_PREASSIGNMENT": model["cells"][0]["class"] = model["cells"][0]["p1353_historical_class"]
    elif mid == "M13_RAW_TOTAL_DELTA": model["delta"] = {"net_gap_delta": 1}
    elif mid == "M14_MIXED_CATALOG_TRIPLE": model["comparison_catalogs"]["historical_snapshot_vs_vanilla"] = sorted(model["catalog"])[1:]
    elif mid == "M15_UNSUPPORTED_ATTRIBUTION": model["attributions"][0].update(inferred=False, measured_premises=[], refuting_observation=None)
    elif mid == "M16_HARNESS_GAP_AS_PRODUCT_GAP": model["attributions"][0].update(comparator_can_represent=False, attribution="NEW_CRYSTALLINE_GAP")
    else: fail("unsupported mutant " + mid)


def command_gate(args: argparse.Namespace) -> None:
    manifest, contract, oracles, mutants = map(load, [args.manifest, args.contract, args.oracles, args.mutants])
    paths = {"contract": Path(args.contract), "oracles": Path(args.oracles), "mutants": Path(args.mutants)}
    drift = [name for name, path in paths.items()
             if sha_file(path) != manifest["protected_inputs"][name]["sha256"]]
    if drift: fail("protected input hash drift: " + ",".join(drift))
    if contract["unit"]["profile_order"] != PROFILE_ORDER: fail("profile order mismatch")
    if set(contract["unknown_policy"]["allowed_reason_codes"]) != UNKNOWN_REASONS: fail("Unknown policy mismatch")
    fixture_rows = []
    for fixture in oracles["positive_contract_fixtures"] + oracles["opaque_fixtures"]:
        verdict, reason = fixture_verdict(fixture)
        fixture_rows.append({"id": fixture["id"], "verdict": verdict, "reason_code": reason})
        if verdict != fixture["expected"] or reason != fixture.get("reason_code"): fail("fixture failed: " + fixture["id"])
    constants = mutants["baseline_constants"]
    base = base_gate_model(constants)
    if model_violations(base, constants): fail("internal conforming fixture is invalid")
    mutant_rows = []
    for mutant in mutants["mutants"]:
        candidate = copy.deepcopy(base); inject(candidate, mutant)
        issues = model_violations(candidate, constants)
        verdict = "Violated" if issues else "Preserved"
        mutant_rows.append({"id": mutant["id"], "verdict": verdict, "witnesses": issues})
        if mutant.get("valid") and verdict != mutant["expected"]: fail("mutant survived: " + mutant["id"])
    physical = fixture_rows + mutant_rows
    hashes = []
    for order in [physical, list(reversed(physical))]:
        for _ in range(2): hashes.append(sha_bytes(canonical(sorted(order, key=lambda x: x["id"]))))
    if len(set(hashes)) != 1: fail("gate nondeterministic")
    valid = sum(bool(x.get("valid")) for x in mutants["mutants"])
    rejected = sum(r["verdict"] == "Violated" for r in mutant_rows)
    emit({"schema":"p1356-harness-gate-v1", "verdict":"PASS", "mutation_score":rejected/valid,
          "valid_mutants":valid, "rejected":rejected, "fixtures":fixture_rows,
          "mutants":mutant_rows, "canonical_payload_sha256":hashes[0]}, args.output)


def recover_catalog() -> tuple[dict[str, Any], bytes]:
    object_name = "2f5e49e72^:00_nucleo/diagnosticos/p1322-probe-catalog.json"
    run = subprocess.run(["git", "show", object_name], cwd=ROOT, capture_output=True, check=True)
    return json.loads(run.stdout), run.stdout


def command_catalog(args: argparse.Namespace) -> None:
    manifest = load(args.manifest); old, raw = recover_catalog()
    frozen = manifest["protected_inputs"]["p1353_catalog_blob"]
    if sha_bytes(raw) != frozen["sha256"] or len(old["probes"]) != frozen["probes"]:
        fail("historical catalog identity mismatch")
    probes = {p["path"]: copy.deepcopy(p) for p in old["probes"]}
    inventory_inputs = []
    for name in args.inventory:
        data = load(name); entries = data.get("entries")
        if not isinstance(entries, dict): fail("inventory lacks entries: " + name)
        inventory_inputs.append({"path":name,"sha256":sha_file(Path(name)),"entries":len(entries)})
        for route, entry in entries.items():
            if route not in probes:
                probes[route] = {"id":"p1356-path-"+route, "path":route,
                    "expression":f"repr((type({route}), repr({route})))",
                    "origins":["p1356:inventory-union"], "roles":["fresh_surface_route"]}
    for route in REQUIRED_ROUTES:
        probes.setdefault(route, {"id":"p1356-path-"+route, "path":route,
            "expression":f"repr((type({route}), repr({route})))", "origins":[], "roles":[]})
        origin = "p1356:prelude_path_alias" if route in ALIASES else "p1356:required-format-route"
        probes[route].setdefault("origins", []).append(origin)
    result = {"schema":"p1356-catalog-v1", "profiles":PROFILE_FEATURES,
              "historical":{"git_object":"2f5e49e72^:00_nucleo/diagnosticos/p1322-probe-catalog.json",
                            "sha256":sha_bytes(raw),"probes":len(old["probes"])},
              "inventory_inputs":inventory_inputs, "required_routes":REQUIRED_ROUTES,
              "probes":sorted(probes.values(), key=lambda p:(p["id"],p["path"]))}
    result["counts"]={"probes":len(result["probes"]),"added":len(result["probes"])-len(old["probes"])}
    emit(result,args.output)


def binary(name: str, path: str, expected: str | None) -> dict[str, str]:
    p=Path(path).resolve(); actual=sha_file(p)
    if expected and actual != expected: fail(f"{name} binary SHA mismatch")
    return {"product_id":name,"path":str(p),"binary_sha256":actual}


def observe(product: dict[str,str], probe: dict[str,Any], profile: str, timeout: float) -> dict[str,Any]:
    features = PROFILE_FEATURES[profile]
    argv=[product["path"],"eval",probe["expression"],"--format","json"]
    if features: argv += ["--features", ",".join(features)]
    reason=None
    try:
        p=subprocess.run(argv,cwd=ROOT,capture_output=True,timeout=timeout)
        code,out,err=p.returncode,p.stdout,p.stderr
    except subprocess.TimeoutExpired as e:
        code,out,err=None,e.stdout or b"",e.stderr or b""; reason="TIMEOUT"
    except OSError as e:
        code,out,err=None,b"",str(e).encode(); reason="ENVIRONMENT_INVALID"
    kind="UNKNOWN"; observable=None
    if reason is None and code == 0:
        try:
            value=json.loads(out.decode("utf-8"),parse_constant=lambda x: fail("nonfinite JSON"))
            observable=sha_bytes(canonical(value)); kind="VALUE"
        except (ValueError,UnicodeError): reason="PARSER_UNSUPPORTED"
    elif reason is None and code == 1 and err.strip():
        observable=sha_bytes(err); kind="DIAGNOSTIC"
    elif reason is None:
        reason="OBSERVER_INCOMPLETE"
    text=err.decode("utf-8",errors="replace").lower()
    absent=kind=="DIAGNOSTIC" and any(x in text for x in ["unknown variable","unknown field","does not contain","unresolved"])
    return {"product_id":product["product_id"],"binary_sha256":product["binary_sha256"],
            "exit_code":code,"outcome_kind":kind,"canonical_observable_sha256":observable,
            "stdout_sha256":sha_bytes(out),"stderr_sha256":sha_bytes(err),
            "complete":reason is None,"reason_code":reason,"absence_witness":absent,
            "features":features,"raw":{"argv":argv,"stdout":out.decode(errors="replace"),
                                         "stderr":err.decode(errors="replace")}}


def command_run(args: argparse.Namespace) -> None:
    catalog=load(args.catalog); manifest=load(args.manifest) if args.manifest else None
    expected = manifest.get("baselines",{}) if manifest else {}
    left=binary(args.left_id,args.left, args.left_sha or expected.get(args.left_id,{}).get("binary_sha256"))
    right=binary(args.right_id,args.right, args.right_sha or expected.get(args.right_id,{}).get("binary_sha256"))
    raw_dir=Path(args.raw_dir).resolve() if args.raw_dir else Path(tempfile.mkdtemp(prefix="p1356-raw.",dir="/tmp"))
    raw_dir.mkdir(parents=True,exist_ok=True)
    if raw_dir != Path("/tmp") and Path("/tmp") not in raw_dir.parents: fail("raw-dir must be below /tmp")
    probes=catalog["probes"][:args.limit or None]
    jobs=[(p,profile) for p in sorted(probes,key=lambda p:p["id"]) for profile in PROFILE_ORDER]
    baseline_payload=None; selected=None; run_hashes=[]
    for order_name in ["canonical","reverse"]:
        physical=jobs if order_name=="canonical" else list(reversed(jobs))
        for repetition in [1,2]:
            def one(job: tuple[dict[str,Any],str]) -> dict[str,Any]:
                probe,profile=job
                l=observe(left,probe,profile,args.timeout); r=observe(right,probe,profile,args.timeout)
                raw={"id":probe["id"],"path":probe["path"],"profile":profile,"left":l.pop("raw"),"right":r.pop("raw")}
                return {"id":probe["id"],"path":probe["path"],"profile":profile,
                        "class":derived_class({"left":l,"right":r}),"left":l,"right":r,"_raw":raw}
            with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
                rows=list(pool.map(one,physical))
            raw_path=raw_dir/f"{order_name}-{repetition}.jsonl"
            with raw_path.open("wb") as stream:
                for row in rows: stream.write(canonical(row.pop("_raw"))+b"\n")
            payload=sorted(rows,key=cell_key); digest=sha_bytes(canonical(payload));run_hashes.append(digest)
            if baseline_payload is None: baseline_payload=payload; selected=rows
            elif payload != baseline_payload: fail(f"nondeterminism in {order_name}/{repetition}")
    counts={name:0 for name in CLASS_ORDER}
    for row in selected or []: counts[row["class"]]+=1
    emit({"schema":"p1356-pair-matrix-v1","pair_id":args.pair_id,"left":left,"right":right,
          "catalog_sha256":sha_file(Path(args.catalog)),"profiles":PROFILE_FEATURES,
          "probes":len(probes),"cells":len(selected or []),"counts":counts,
          "canonical_payload_sha256":run_hashes[0],"determinism_hashes":run_hashes,
          "raw_directory":str(raw_dir),"results":selected},args.output)


def command_delta(args: argparse.Namespace) -> None:
    matrices={k:load(v) for k,v in {"crystalline_old":args.crystalline_old,
        "crystalline_new":args.crystalline_new,"old_new":args.old_new}.items()}
    maps={k:{(c["id"],c["profile"]):c for c in m["results"]} for k,m in matrices.items()}
    sets={k:set(v) for k,v in maps.items()}; common=set.intersection(*sets.values())
    historical_catalog,_ = recover_catalog()
    historical_paths={p["path"] for p in historical_catalog["probes"]}
    current_paths={c["path"] for c in matrices["crystalline_new"]["results"]}
    entry_paths=sorted(current_paths-historical_paths)
    exit_paths=sorted(historical_paths-current_paths)
    historical_divergent=set()
    if args.historical_compact:
        historical_divergent=set(load(args.historical_compact)["paths"]["divergent_paths"])
    rows=[]
    for key in sorted(common,key=lambda k:(k[0],profile_index(k[1]))):
        co,cn,on=(maps[x][key] for x in ["crystalline_old","crystalline_new","old_new"])
        classes=[co["class"],cn["class"],on["class"]]
        # Attribution describes current gaps.  A triple match is evidence of
        # preservation, not evidence that the product resolved a historical gap.
        if cn["class"].startswith("MATCH_"):
            continue
        unknown=any(x=="UNKNOWN" for x in classes)
        representable=not unknown
        if unknown: attr="HARNESS_GAP"
        elif co["class"].startswith("MATCH_") and not on["class"].startswith("MATCH_"): attr="TARGET_DRIFT"
        elif cn["path"] in historical_paths: attr="RETAINED_GAP"
        else: attr="NEW_CRYSTALLINE_GAP"
        rows.append({"id":key[0],"path":cn["path"],"profile":key[1],"attribution":attr,
            "inferred":True,"comparator_can_represent":representable,
            "measured_premises":{"crystalline_old":co["class"],"crystalline_new":cn["class"],"old_new":on["class"]},
            "refuting_observation":"a repeated complete cell changes any recorded pair class"})
    counts={}
    for row in rows: counts[row["attribution"]]=counts.get(row["attribution"],0)+1
    by_path={}
    for row in rows:
        by_path.setdefault(row["path"],{}).setdefault(row["attribution"],[]).append(row["profile"])
    current_by_path={}
    for cell in matrices["crystalline_new"]["results"]:
        current_by_path.setdefault(cell["path"],[]).append(cell["class"])
    candidate_paths=sorted(p for p,classes in current_by_path.items()
                           if p in historical_divergent and len(classes)==len(PROFILE_ORDER)
                           and all(c.startswith("MATCH_") for c in classes))
    resolved_candidates={"status":"INCONCLUSIVE_PROFILE_GRANULARITY",
        "count":len(candidate_paths),"paths":candidate_paths,
        "reason_code":"HISTORICAL_PROFILE_EVIDENCE_MISSING",
        "refuting_observation":"a P1353 receipt identifying the non-MATCH profiles for each path"}
    emit({"schema":"p1356-delta-v2",
          "catalog_delta":{"historical_paths":len(historical_paths),"current_paths":len(current_paths),
                           "intersection":len(historical_paths & current_paths),
                           "entry_paths":entry_paths,"entries_count":len(entry_paths),
                           "exit_paths":exit_paths,"exits_count":len(exit_paths)},
          "comparison_cell_intersection":len(common),"current_nonmatch_cells":len(rows),
          "counts":counts,"attributions":rows,
          "paths":[{"path":p,"profiles_by_attribution":v} for p,v in sorted(by_path.items())],
          "resolved_candidates":resolved_candidates,
          "resolution_policy":"No RESOLVED_BY_PRODUCT without a historical P1353 non-MATCH receipt for the exact path/profile."},args.output)


class AuthorMetaParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__(); self.authors=[]
    def handle_starttag(self, tag: str, attrs: list[tuple[str,str|None]]) -> None:
        values=dict(attrs)
        if tag.lower()=="meta" and values.get("name","").lower()=="author":
            self.authors.append(values.get("content"))


def public_command(argv: list[str], timeout: float) -> dict[str,Any]:
    try:
        p=subprocess.run(argv,cwd=ROOT,capture_output=True,timeout=timeout)
        code,out,err=p.returncode,p.stdout,p.stderr; reason=None
    except subprocess.TimeoutExpired as exc:
        code,out,err=None,exc.stdout or b"",exc.stderr or b"";reason="TIMEOUT"
    except OSError as exc:
        code,out,err=None,b"",str(exc).encode();reason="ENVIRONMENT_INVALID"
    return {"argv":argv,"exit_code":code,"stdout":out,"stderr":err,"reason_code":reason}


def channel_observable(receipt: dict[str,Any]) -> dict[str,Any]:
    if receipt["reason_code"]:
        return {"complete":False,"reason_code":receipt["reason_code"],"outcome_kind":"UNKNOWN",
                "canonical_observable_sha256":None}
    kind="VALUE" if receipt["exit_code"]==0 else "DIAGNOSTIC"
    public=receipt["stdout"] if kind=="VALUE" else receipt["stderr"]
    if not public and kind=="DIAGNOSTIC":
        return {"complete":False,"reason_code":"OBSERVER_INCOMPLETE","outcome_kind":"UNKNOWN",
                "canonical_observable_sha256":None}
    return {"complete":True,"reason_code":None,"outcome_kind":kind,
            "exit_code":receipt["exit_code"],"canonical_observable_sha256":sha_bytes(public),
            "stdout_sha256":sha_bytes(receipt["stdout"]),"stderr_sha256":sha_bytes(receipt["stderr"])}


def xml_witness(path: Path) -> Any:
    root=ET.parse(path).getroot(); rows=[]
    for elem in root.iter():
        tag=elem.tag.rsplit("}",1)[-1]
        if tag in {"text","tspan","path","line","rect","g"}:
            attrs={k.rsplit("}",1)[-1]:v for k,v in elem.attrib.items()
                   if k.rsplit("}",1)[-1] in {"x","y","x1","x2","y1","y2","d","transform","class"}}
            rows.append([tag,attrs,"".join(elem.itertext()) if tag in {"text","tspan"} else None])
    return rows


def output_observable(receipt: dict[str,Any], output: Path, observer: str) -> dict[str,Any]:
    base=channel_observable(receipt)
    if not base["complete"] or base["outcome_kind"]=="DIAGNOSTIC": return base
    try:
        if observer=="xml": value=xml_witness(output)
        elif observer=="png":
            data=output.read_bytes()
            if data[:8] != b"\x89PNG\r\n\x1a\n" or len(data) < 24: raise ValueError("invalid PNG")
            width,height=struct.unpack(">II",data[16:24]);value={"width":width,"height":height}
        elif observer=="html":
            parser=AuthorMetaParser();parser.feed(output.read_text(encoding="utf-8"));value=parser.authors
        elif observer=="pdf":
            info=public_command(["pdfinfo",str(output)],30)
            text=public_command(["pdftotext","-bbox-layout",str(output),"-"],30)
            if info["exit_code"] or text["exit_code"]: raise ValueError("PDF observer failed")
            tagged=[line for line in info["stdout"].decode(errors="replace").splitlines()
                    if line.startswith(("Pages:","Tagged:"))]
            value={"pdfinfo":tagged,"text_bbox_xml":sha_bytes(text["stdout"])}
        else: raise ValueError("unknown observer")
    except (OSError,ValueError,ET.ParseError,UnicodeError) as exc:
        return {"complete":False,"reason_code":"PARSER_UNSUPPORTED","outcome_kind":"UNKNOWN",
                "canonical_observable_sha256":None,"observer_error":str(exc)}
    base["canonical_observable_sha256"]=sha_bytes(canonical(value));base["public_observable"]=value
    return base


def sentinel_side(product: dict[str,str], case: dict[str,Any], root: Path,
                  timeout: float) -> tuple[dict[str,Any],dict[str,Any]]:
    case_dir=root/product["product_id"]/case["id"];case_dir.mkdir(parents=True,exist_ok=True)
    source=case.get("source")
    if case["mode"]=="gap":
        obs={"complete":False,"reason_code":"PARSER_UNSUPPORTED","outcome_kind":"UNKNOWN",
             "canonical_observable_sha256":None,"detail":case["detail"]}
        return obs,{"id":case["id"],"not_executed":case["detail"]}
    argv=[product["path"]]
    output=None
    if case["mode"]=="eval":
        argv += ["eval",source,"--format","json",*case.get("options",[])]
    elif case["mode"]=="help": argv += case["argv"]
    else:
        src=case_dir/"probe.typ";src.write_text(source,encoding="utf-8")
        output=case_dir/("out."+case["extension"])
        output.unlink(missing_ok=True)
        argv += ["compile",*case.get("options",[]),str(src),str(output)]
    receipt=public_command(argv,timeout)
    raw={"id":case["id"],"product_id":product["product_id"],"argv":argv,
         "exit_code":receipt["exit_code"],"reason_code":receipt["reason_code"],
         "stdout":receipt["stdout"].decode(errors="replace"),"stderr":receipt["stderr"].decode(errors="replace")}
    obs=(output_observable(receipt,output,case["observer"]) if output and output.exists()
         else channel_observable(receipt))
    return obs,raw


def sentinel_cases() -> dict[str,list[dict[str,Any]]]:
    return {
      "S01":[{"id":"kinds","mode":"eval","options":["--features","html,a11y-extras"],"source":"repr((type(format), type(format.pdf), type(pdf), type(format.html), type(html), type(format.svg), type(svg), type(format.png), type(png), type(format.bundle), type(asset)))"}],
      "S02":[
        {"id":"top-level","mode":"compile","extension":"pdf","observer":"pdf","source":"#set pdf(tagged: true, pretty: true)\nDocument control"},
        {"id":"nested-rejection","mode":"compile","extension":"pdf","observer":"pdf","source":"#box[#set pdf(tagged: false)\nNested rejection control]"}],
      "S03":[
        {"id":"named-args","mode":"compile","extension":"pdf","observer":"pdf","source":"#set pdf(pages: auto, standard: auto, tagged: true, pretty: true)\nPDF option probe"},
        {"id":"incompatible","mode":"compile","extension":"pdf","observer":"pdf","options":["--pdf-standard","ua-1","--pdf-tagged=false"],"source":"PDF incompatible control"}],
      "S04":[{"id":"luma-calls","mode":"eval","source":"repr((luma(), luma(128), luma(128, 0), luma(128, 255)))"}],
      "S05":[{"id":"invalid-key","mode":"eval","source":"((\"ok\", 1), (1, 2)).to-dict()"}],
      "S06":[{"id":"columns-svg","mode":"compile","extension":"svg","observer":"xml","options":["--format","svg"],"source":"#columns(2, separator: line(length: 100%))[A #colbreak() B]\n#set text(dir: rtl)\n#columns(2, separator: [|])[يمين #colbreak()]"}],
      "S07":[{"id":"tagged-block","mode":"compile","extension":"pdf","observer":"pdf","options":["--features","a11y-extras"],"source":"#block(height: 100%)[#metadata(\"tagged-lone\")<lone> Lone]\n#block(height: 100%)[First #metadata(\"tagged-multi\")<multi> Second]"}],
      "S08":[{"id":"cjk-svg","mode":"compile","extension":"svg","observer":"xml","options":["--format","svg"],"source":"#set page(width: 50pt)\n｟甲｠ 〘乙〙 〝丙〟"}],
      "S09":[{"id":"author-meta","mode":"compile","extension":"html","observer":"html","options":["--features","html","--format","html"],"source":"#set document(author: (\"Ada\", \"Grace\", \"Linus\"))\nAuthors"}],
      "S10":[
        {"id":"input","mode":"compile","extension":"pdf","observer":"pdf","options":["-i","key=value"],"source":"#repr(sys.inputs.at(\"key\", default: none))"},
        {"id":"pretty-tagged-true","mode":"compile","extension":"pdf","observer":"pdf","options":["--pretty=true","--pdf-tagged=true"],"source":"CLI flags"},
        {"id":"pretty-tagged-false","mode":"compile","extension":"pdf","observer":"pdf","options":["--pretty=false","--pdf-tagged=false"],"source":"CLI flags"},
        {"id":"ppi","mode":"compile","extension":"png","observer":"png","options":["--ppi","144"],"source":"CLI PPI"},
        {"id":"watch-options","mode":"help","argv":["watch","--no-fullscreen","--open=VIEWER","--help"],"source":None},
        {"id":"compile-open","mode":"help","argv":["compile","--open=VIEWER","--help"],"source":None}],
      "S11":[{"id":"duration-boundaries","mode":"gap","detail":"No public deterministic CLI accepts an injected duration boundary vector; watch timing is process scheduling and cannot supply the frozen exact boundaries."}],
      "S12":[{"id":"sys-version","mode":"eval","source":"repr((sys.version, sys.version == version(0, 15, 1)))"}],
    }


def command_sentinels(args: argparse.Namespace) -> None:
    oracle=load(args.oracles)
    expected_ids={x["id"].split("_",1)[0] for x in oracle["required_public_sentinels"]}
    if expected_ids != set(sentinel_cases()): fail("sentinel implementation/oracle ID mismatch")
    products=[binary("vanilla",args.vanilla,args.vanilla_sha),
              binary("crystalline",args.crystalline,args.crystalline_sha)]
    raw_dir=Path(args.raw_dir).resolve() if args.raw_dir else Path(tempfile.mkdtemp(prefix="p1356-sentinels.",dir="/tmp"))
    raw_dir.mkdir(parents=True,exist_ok=True)
    if raw_dir != Path("/tmp") and Path("/tmp") not in raw_dir.parents: fail("raw-dir must be below /tmp")
    results=[];raw_rows=[]
    for family,cases in sentinel_cases().items():
        observations={p["product_id"]:[] for p in products}
        for case in cases:
            for product in products:
                obs,raw=sentinel_side(product,case,raw_dir,args.timeout)
                observations[product["product_id"]].append({"case":case["id"],**obs});raw_rows.append(raw)
        sides=list(observations.values()); incomplete=any(not o["complete"] for side in sides for o in side)
        if incomplete: status="HARNESS_GAP"
        else:
            left=[(o["outcome_kind"],o["canonical_observable_sha256"]) for o in sides[0]]
            right=[(o["outcome_kind"],o["canonical_observable_sha256"]) for o in sides[1]]
            status="MATCH" if left==right else "DIFFERENCE"
        results.append({"id":family,"status":status,"observations":observations})
    with (raw_dir/"commands.jsonl").open("wb") as stream:
        for row in raw_rows: stream.write(canonical(row)+b"\n")
    totals={}
    for row in results: totals[row["status"]]=totals.get(row["status"],0)+1
    verdict="PASS" if not totals.get("HARNESS_GAP") else "INCONCLUSIVE"
    emit({"schema":"p1356-sentinels-v1","verdict":verdict,"totals":totals,
          "raw_directory":str(raw_dir),"results":results},args.output)


def parser() -> argparse.ArgumentParser:
    p=argparse.ArgumentParser(description=__doc__); sub=p.add_subparsers(dest="command",required=True)
    g=sub.add_parser("gate");
    for n in ["manifest","contract","oracles","mutants"]: g.add_argument("--"+n,required=True)
    g.add_argument("--output");g.set_defaults(func=command_gate)
    c=sub.add_parser("catalog",aliases=["build-catalog"]);c.add_argument("--manifest",required=True);c.add_argument("--inventory",action="append",default=[],required=True);c.add_argument("--output");c.set_defaults(func=command_catalog)
    r=sub.add_parser("run",aliases=["run-matrix"]);r.add_argument("--catalog",required=True);r.add_argument("--manifest");r.add_argument("--pair-id",required=True);r.add_argument("--left",required=True);r.add_argument("--right",required=True);r.add_argument("--left-id",required=True);r.add_argument("--right-id",required=True);r.add_argument("--left-sha");r.add_argument("--right-sha");r.add_argument("--raw-dir");r.add_argument("--output");r.add_argument("--workers",type=int,default=4);r.add_argument("--timeout",type=float,default=30);r.add_argument("--limit",type=int);r.set_defaults(func=command_run)
    d=sub.add_parser("delta",aliases=["summarize"]);d.add_argument("--crystalline-historical",dest="crystalline_old",required=True);d.add_argument("--crystalline-vanilla",dest="crystalline_new",required=True);d.add_argument("--historical-vanilla",dest="old_new",required=True);d.add_argument("--output");d.set_defaults(func=command_delta)
    d.add_argument("--historical-compact")
    s=sub.add_parser("sentinels");s.add_argument("--oracles",required=True);s.add_argument("--vanilla",required=True);s.add_argument("--crystalline",required=True);s.add_argument("--vanilla-sha");s.add_argument("--crystalline-sha");s.add_argument("--raw-dir");s.add_argument("--timeout",type=float,default=30);s.add_argument("--output");s.set_defaults(func=command_sentinels)
    return p


def main() -> None:
    try:
        args = parser().parse_args()
        args.func(args)
    except (ValueError,KeyError,FileNotFoundError,subprocess.CalledProcessError) as exc:
        print(json.dumps({"schema":"p1356-harness-error-v1","verdict":"INVALID","error":str(exc)},ensure_ascii=False),file=sys.stderr)
        raise SystemExit(2)


if __name__ == "__main__": main()
