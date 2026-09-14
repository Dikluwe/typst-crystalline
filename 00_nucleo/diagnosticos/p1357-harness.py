#!/usr/bin/env python3
"""P1357 focal observability harness; raw artifacts are restricted to /tmp."""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
from typing import Any
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[2]
PROFILES = ["default", "html", "a11y-extras", "html+a11y"]
FEATURES = {"default": [], "html": ["html"], "a11y-extras": ["a11y-extras"],
            "html+a11y": ["html", "a11y-extras"]}
NORMATIVE = "586e1bd43fae6c9a973218163d3165c53ab8d16d"
UNKNOWN_REASONS = {"AMBIGUOUS_PRODUCT_IDENTITY", "SOURCE_MISSING", "TIMEOUT",
                   "PARSER_UNSUPPORTED", "OBSERVER_INCOMPLETE", "ENVIRONMENT_INVALID",
                   "BUDGET_EXHAUSTED"}
S04_IDS = ["S04_LUMA_EMPTY", "S04_LUMA_HINT", "S04_ALPHA_0", "S04_ALPHA_255",
           "S04_ALPHA_MINUS_1", "S04_ALPHA_256"]
S10_IDS = ["S10_PRETTY_BARE", "S10_PRETTY_TRUE", "S10_PRETTY_FALSE",
           "S10_PDF_TAGGED_BARE", "S10_PDF_TAGGED_TRUE", "S10_PDF_TAGGED_FALSE"]


def canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True,
                      separators=(",", ":")).encode()


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def file_digest(path: str | Path) -> str:
    with Path(path).open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def load(path: str | Path) -> Any:
    with Path(path).open(encoding="utf-8") as stream:
        return json.load(stream)


def emit(value: Any, output: str | None) -> None:
    payload = canonical(value) + b"\n"
    if output: Path(output).write_bytes(payload)
    else: sys.stdout.buffer.write(payload)


def require_tmp(path: str | None, prefix: str) -> Path:
    target = Path(path).resolve() if path else Path(tempfile.mkdtemp(prefix=prefix, dir="/tmp"))
    if target != Path("/tmp") and Path("/tmp") not in target.parents:
        raise ValueError("raw output must be below /tmp")
    target.mkdir(parents=True, exist_ok=True)
    return target


def base_model() -> dict[str, Any]:
    envelopes = [{"product": product, "profile": profile,
                  "features": FEATURES[profile], "binary_sha256": "a" * 64,
                  "catalog_sha256": "b" * 64, "route_count": 1}
                 for product in ["vanilla", "crystalline"] for profile in PROFILES]
    return {"baseline": {"vanilla": {"commit": NORMATIVE, "normative": True}},
            "profile_features": copy.deepcopy(FEATURES), "inventory_envelopes": envelopes,
            "S04": [{"id": ident, "process_id": ident, "complete": True} for ident in S04_IDS],
            "S10": list(S10_IDS),
            "opaque": {"O_S07_PARSER_UNSUPPORTED": {"complete": False, "verdict": "Unknown", "reason_code": "PARSER_UNSUPPORTED"},
                       "O_S11_BINDING_UNAVAILABLE": {"complete": False, "verdict": "Unknown", "reason_code": "PARSER_UNSUPPORTED"}},
            "S11": {"input_source": "frozen_boundary_vector", "frozen_boundary_vector_sha256": "c" * 64},
            "backlog": [{"classification": "PRODUCT_GAP", "owner_l0": "00_nucleo/prompts/example.md"}]}


def violations(model: dict[str, Any]) -> list[str]:
    issues = []
    if model["baseline"]["vanilla"]["commit"] != NORMATIVE: issues.append("NON_NORMATIVE_VANILLA")
    expected = {(p, q) for p in ["vanilla", "crystalline"] for q in PROFILES}
    actual = {(e["product"], e["profile"]) for e in model["inventory_envelopes"]}
    if actual != expected or len(model["inventory_envelopes"]) != 8: issues.append("INVENTORY_PROFILE_COVERAGE")
    for envelope in model["inventory_envelopes"]:
        if envelope["features"] != FEATURES[envelope["profile"]]: issues.append("PROFILE_MAPPING")
    by_profile = {}
    for envelope in model["inventory_envelopes"]: by_profile.setdefault(envelope["profile"], []).append(envelope["features"])
    if any(len(v) != 2 or v[0] != v[1] for v in by_profile.values()): issues.append("ASYMMETRIC_FEATURES")
    plans = model["S04"]
    if {x["id"] for x in plans} != set(S04_IDS) or len({x["process_id"] for x in plans}) != len(S04_IDS) or not all(x["complete"] for x in plans):
        issues.append("S04_NOT_ISOLATED")
    if set(model["S10"]) != set(S10_IDS): issues.append("S10_FORMS_INCOMPLETE")
    for opaque in model["opaque"].values():
        if opaque["complete"] or opaque["verdict"] != "Unknown" or opaque["reason_code"] not in UNKNOWN_REASONS:
            issues.append("UNKNOWN_PROMOTED")
    if model["S11"]["input_source"] in {"wall_clock", "wall_clock_elapsed", "scheduler"} or not model["S11"].get("frozen_boundary_vector_sha256"):
        issues.append("S11_CLOCK_INPUT")
    if any(x["classification"] == "PRODUCT_GAP" and not x.get("owner_l0") for x in model["backlog"]):
        issues.append("PRODUCT_GAP_WITHOUT_OWNER")
    return sorted(set(issues))


def inject(model: dict[str, Any], mutant: dict[str, Any]) -> None:
    mid = mutant["id"]
    if mid == "M01_HISTORICAL_VANILLA": model["baseline"]["vanilla"]["commit"] = "a51e028041cac426f97d34335bb01d8f1d8e5e8f"
    elif mid == "M02_MISSING_PROFILE_INVENTORY": model["inventory_envelopes"] = [e for e in model["inventory_envelopes"] if not (e["product"] == "crystalline" and e["profile"] == "html+a11y")]
    elif mid == "M03_HTML_A11Y_SINGLE_FEATURE":
        for e in model["inventory_envelopes"]:
            if e["profile"] == "html+a11y": e["features"] = ["html"]
    elif mid == "M04_ASYMMETRIC_FEATURES":
        next(e for e in model["inventory_envelopes"] if e["product"] == "crystalline" and e["profile"] == "html+a11y")["features"] = ["html"]
    elif mid == "M05_GROUPED_S04":
        for e in model["S04"]: e["process_id"] = "shared"
        model["S04"][-1]["complete"] = False
    elif mid == "M06_OMIT_S10_BARE_FORMS": model["S10"] = [x for x in model["S10"] if x not in {"S10_PRETTY_BARE", "S10_PDF_TAGGED_BARE"}]
    elif mid == "M07_UNKNOWN_TO_MATCH":
        for e in model["opaque"].values(): e["verdict"] = "MATCH"
    elif mid == "M08_CLOCK_DRIVEN_S11": model["S11"] = {"input_source": "wall_clock_elapsed", "frozen_boundary_vector_sha256": None}
    elif mid == "M09_BACKLOG_WITHOUT_OWNER_L0": model["backlog"][0]["owner_l0"] = None
    else: raise ValueError("unsupported mutant: " + mid)


def command_gate(args: argparse.Namespace) -> None:
    oracles, mutants = load(args.oracles), load(args.mutants)
    expected = {"step": "fd4046587ebaedc5dec0714e288b769453f77023a69e9fdc98c58c441ac9a031",
                "oracles": "39992f97f74eff4593596a9996d4f5b78ecc7977ddd78e7abd3d9047fec9e604",
                "mutants": "697ee83f87d9ed749e142a2b27a33853a79c26a60afcd8ed1febebcf83c1856f"}
    actual = {"step": file_digest(args.step), "oracles": file_digest(args.oracles), "mutants": file_digest(args.mutants)}
    if actual != expected: raise ValueError("frozen input hash drift")
    if oracles["execution_contract"]["profile_order"] != PROFILES or oracles["execution_contract"]["profile_features"] != FEATURES:
        raise ValueError("oracle profile contract mismatch")
    base = base_model()
    if violations(base): raise ValueError("internal positive fixture invalid")
    opaque = [{"id": x["id"], "verdict": "Unknown", "reason_code": x["reason_code"]}
              for x in oracles["opaque_fixtures"]]
    if any(x["reason_code"] not in UNKNOWN_REASONS for x in opaque): raise ValueError("opaque reason outside allowlist")
    rows = []
    for mutant in mutants["mutants"]:
        candidate = copy.deepcopy(base); inject(candidate, mutant); found = violations(candidate)
        verdict = "Violated" if found else "Preserved"
        rows.append({"id": mutant["id"], "verdict": verdict, "witnesses": found})
        if mutant["valid"] and verdict != mutant["expected"]: raise ValueError("mutant survived: " + mutant["id"])
    payload = sorted(rows + opaque, key=lambda x: x["id"])
    hashes = [digest(canonical(sorted(order, key=lambda x: x["id"]))) for order in [payload, list(reversed(payload))] for _ in range(2)]
    if len(set(hashes)) != 1: raise ValueError("gate nondeterministic")
    valid = sum(x["valid"] for x in mutants["mutants"]); rejected = sum(x["verdict"] == "Violated" for x in rows)
    emit({"schema": "p1357-gate-v1", "verdict": "PASS", "input_hashes": actual,
          "valid_mutants": valid, "rejected": rejected, "mutation_score": rejected / valid,
          "opaque": opaque, "mutants": rows, "determinism_hashes": hashes}, args.output)


def product(name: str, path: str, expected: str | None) -> dict[str, str]:
    resolved = Path(path).resolve(); actual = file_digest(resolved)
    if expected and actual != expected: raise ValueError(name + " binary SHA mismatch")
    return {"id": name, "path": str(resolved), "sha256": actual}


def run(argv: list[str], timeout: float = 120) -> dict[str, Any]:
    started = time.monotonic_ns()
    try:
        p = subprocess.run(argv, cwd=ROOT, capture_output=True, timeout=timeout)
        code, out, err, reason = p.returncode, p.stdout, p.stderr, None
    except subprocess.TimeoutExpired as exc:
        code, out, err, reason = None, exc.stdout or b"", exc.stderr or b"", "TIMEOUT"
    except OSError as exc:
        code, out, err, reason = None, b"", str(exc).encode(), "ENVIRONMENT_INVALID"
    return {"argv": argv, "exit_code": code, "stdout": out, "stderr": err,
            "reason_code": reason, "duration_ns": time.monotonic_ns() - started}


def command_inventory(args: argparse.Namespace) -> None:
    raw = require_tmp(args.raw_dir, "p1357-inventory.")
    vanilla = product("vanilla", args.vanilla_binary, args.vanilla_sha)
    crystalline = product("crystalline", args.crystalline_binary, args.crystalline_sha)
    baseline = None
    baseline_raw = None
    hashes = []
    executions = []
    phases = [("canonical-1", PROFILES), ("canonical-2", PROFILES),
              ("reverse-1", list(reversed(PROFILES))),
              ("reverse-2", list(reversed(PROFILES)))]
    for phase, profile_order in phases:
        phase_root = raw / phase
        phase_root.mkdir(parents=True, exist_ok=True)
        phase_rows = []
        phase_raw = {}
        # Each crystalline enumerator consumes its same-phase vanilla inventory.
        # The dependency fixes side order; profile order is the reversed dimension.
        for profile in profile_order:
            for side in ["vanilla", "crystalline"]:
                output = phase_root / f"{side}-{profile}.json"
                if side == "vanilla":
                    argv = [str(Path(args.vanilla_enumerator).resolve()), str(output),
                            profile, vanilla["path"]]
                else:
                    hint = phase_root / f"vanilla-{profile}.json"
                    argv = [str(Path(args.crystalline_enumerator).resolve()), str(output),
                            str(hint), profile, args.seeds, crystalline["path"]]
                receipt = run(argv, args.timeout)
                executions.append({"phase": phase, "product": side, "profile": profile,
                                   "argv": argv, "exit_code": receipt["exit_code"],
                                   "reason_code": receipt["reason_code"],
                                   "duration_ns": receipt["duration_ns"]})
                if receipt["reason_code"] or receipt["exit_code"] != 0:
                    raise ValueError(f"inventory failed: {phase}/{side}/{profile}")
                envelope = load(output)
                for field in ["profile", "features", "product_sha256", "catalog_sha256",
                              "route_count", "entries"]:
                    if field not in envelope:
                        raise ValueError(f"inventory field missing: {side}/{profile}/{field}")
                identity = vanilla if side == "vanilla" else crystalline
                if (envelope["profile"] != profile or envelope["features"] != FEATURES[profile]
                        or envelope["product_sha256"] != identity["sha256"]
                        or envelope["route_count"] != len(envelope["entries"])):
                    raise ValueError(f"inventory identity mismatch: {side}/{profile}")
                phase_raw[(side, profile)] = envelope
                phase_rows.append({"product": side, "profile": profile,
                                   "features": envelope["features"],
                                   "binary_sha256": envelope["product_sha256"],
                                   "catalog_sha256": envelope["catalog_sha256"],
                                   "route_count": envelope["route_count"],
                                   "entries_sha256": digest(canonical(envelope["entries"]))})
        phase_rows.sort(key=lambda x: (PROFILES.index(x["profile"]), x["product"]))
        phase_hash = digest(canonical(phase_rows)); hashes.append(phase_hash)
        if baseline is None:
            baseline, baseline_raw = phase_rows, phase_raw
        elif phase_rows != baseline:
            raise ValueError("inventory nondeterminism across repetition/order")

    crosswalk = []
    crosswalk_counts = {}
    assert baseline_raw is not None
    for profile in PROFILES:
        vanilla_paths = set(baseline_raw[("vanilla", profile)]["entries"])
        crystalline_paths = set(baseline_raw[("crystalline", profile)]["entries"])
        counts = {"BOTH_PRESENT": 0, "VANILLA_ONLY": 0, "CRYSTALLINE_ONLY": 0}
        for path in sorted(vanilla_paths | crystalline_paths):
            vp, cp = path in vanilla_paths, path in crystalline_paths
            classification = "BOTH_PRESENT" if vp and cp else ("VANILLA_ONLY" if vp else "CRYSTALLINE_ONLY")
            counts[classification] += 1
            crosswalk.append({"profile": profile, "path": path,
                              "vanilla_present": vp, "crystalline_present": cp,
                              "classification": classification})
        crosswalk_counts[profile] = counts
    emit({"schema": "p1357-eight-inventories-v2", "verdict": "PASS",
          "profiles": FEATURES, "execution_model": "two canonical plus two reverse-profile runs",
          "envelope_count": len(baseline or []), "envelopes": baseline,
          "determinism_hashes": hashes, "crosswalk_semantics": "presence-only",
          "crosswalk_counts": crosswalk_counts, "crosswalk": crosswalk,
          "raw_directory": str(raw), "executions": executions}, args.output)


def features_args(profile: str) -> list[str]:
    return ["--features", ",".join(FEATURES[profile])] if FEATURES[profile] else []


def normalize(receipt: dict[str, Any], product_data: dict[str, str], source: str,
              semantic: Any = None, reason: str | None = None) -> dict[str, Any]:
    unknown = reason or receipt["reason_code"]
    if unknown:
        return {"product_id": product_data["id"], "binary_sha256": product_data["sha256"],
                "source_sha256": digest(source.encode()), "complete": False,
                "outcome_kind": "UNKNOWN", "reason_code": unknown, "canonical_observable_sha256": None}
    kind = "VALUE" if receipt["exit_code"] == 0 else "DIAGNOSTIC"
    public = canonical(semantic) if semantic is not None else (receipt["stdout"] if kind == "VALUE" else receipt["stderr"])
    if not public and kind == "DIAGNOSTIC": return normalize(receipt, product_data, source, reason="OBSERVER_INCOMPLETE")
    return {"product_id": product_data["id"], "binary_sha256": product_data["sha256"],
            "source_sha256": digest(source.encode()), "complete": True, "outcome_kind": kind,
            "reason_code": None, "exit_code": receipt["exit_code"],
            "canonical_observable_sha256": digest(public), "stdout_sha256": digest(receipt["stdout"]),
            "stderr_sha256": digest(receipt["stderr"]), "public_observable": semantic}


def tool_identity(tool: str) -> tuple[str, dict[str, Any]]:
    resolved = shutil.which(tool)
    if resolved is None:
        raise ValueError(tool + " unavailable")
    resolved = str(Path(resolved).resolve())
    version = run([resolved, "-v" if tool == "pdftotext" else "--version"], 30)
    if version["exit_code"] != 0:
        raise ValueError(tool + " version unavailable")
    return resolved, {"tool": tool, "resolved_path": resolved,
                      "binary_sha256": file_digest(resolved),
                      "version_argv": version["argv"], "version_exit_status": version["exit_code"],
                      "version_stdout": version["stdout"].decode(errors="replace").strip(),
                      "version_stderr": version["stderr"].decode(errors="replace").strip()}


def qpdf_morphology(path: Path) -> tuple[Any, dict[str, Any]]:
    executable, identity = tool_identity("qpdf")
    receipt = run([executable, "--json", str(path)], 30)
    if receipt["exit_code"] != 0: raise ValueError("qpdf structural observer failed")
    doc = json.loads(receipt["stdout"])
    objects = doc.get("qpdf", [{}, {}])[1]
    rows = []
    for key, item in sorted(objects.items()):
        value = item.get("value", {}) if isinstance(item, dict) else {}
        if not isinstance(value, dict):
            continue
        if value.get("/Type") in {"/StructTreeRoot", "/StructElem", "/MCR", "/OBJR"}:
            rows.append({"type": value.get("/Type"), "role": value.get("/S"),
                         "alt": value.get("/Alt"), "actual_text": value.get("/ActualText"),
                         "kids_shape": type(value.get("/K")).__name__})
    provenance = {**identity, "argv": receipt["argv"],
                  "exit_status": receipt["exit_code"]}
    return rows, provenance


def vertical_witness(path: Path) -> tuple[Any, dict[str, Any]]:
    executable, identity = tool_identity("pdftotext")
    receipt = run([executable, "-bbox-layout", str(path), "-"], 30)
    if receipt["exit_code"] != 0:
        raise ValueError("pdftotext vertical observer failed")
    root = ET.fromstring(receipt["stdout"])
    pages = []
    for page in root.findall(".//{*}page"):
        words = []
        for word in page.findall(".//{*}word"):
            words.append({"text": "".join(word.itertext()),
                          "x_min": word.get("xMin"), "x_max": word.get("xMax"),
                          "y_min": word.get("yMin"), "y_max": word.get("yMax")})
        pages.append({"width": page.get("width"), "height": page.get("height"),
                      "words": words})
    if not pages:
        raise ValueError("pdftotext returned no measurable pages")
    witness = {"measurement": "page_extent_and_text_vertical_bounding_boxes",
               "page_count": len(pages), "pages": pages}
    provenance = {**identity, "argv": receipt["argv"],
                  "exit_status": receipt["exit_code"]}
    return witness, provenance


def cases() -> list[dict[str, Any]]:
    values = {
      "S02_TOP_LEVEL_SET": ("compile", "#set pdf(tagged: true)\nTop", []),
      "S02_NESTED_REJECTION": ("compile", "#box[#set pdf(tagged: false)\nNested]", []),
      "S02_DOCUMENT_OVER_CLI": ("compile_pdf", "#set pdf(tagged: false)\nDocument", ["--pdf-tagged=true"]),
      "S02_CLI_ONLY_CONTROL": ("compile_pdf", "CLI", ["--pdf-tagged=true"]),
      "S04_LUMA_EMPTY": ("eval", "repr(luma())", []),
      "S04_LUMA_HINT": ("eval", "repr(luma(128))", []),
      "S04_ALPHA_0": ("eval", "repr(luma(128, 0))", []),
      "S04_ALPHA_255": ("eval", "repr(luma(128, 255))", []),
      "S04_ALPHA_MINUS_1": ("eval", "repr(luma(128, -1))", []),
      "S04_ALPHA_256": ("eval", "repr(luma(128, 256))", []),
      "S07_LONE_TAGGED_BLOCK": ("pdf_tree", "#block(height: 100%)[#metadata(\"tagged-lone\")<lone> Lone]", []),
      "S07_MULTIPLE_CHILD_CONTROL": ("pdf_tree", "#block(height: 100%)[First #metadata(\"tagged-multi\")<multi> Second]", []),
    }
    for ident, option in zip(S10_IDS, ["--pretty", "--pretty=true", "--pretty=false", "--pdf-tagged", "--pdf-tagged=true", "--pdf-tagged=false"]):
        values[ident] = ("compile", "CLI option", [option])
    return [{"id": ident, "family": ident.split("_", 1)[0], "mode": mode, "source": source, "options": options}
            for ident, (mode, source, options) in values.items()]


def execute_case(case: dict[str, Any], profile: str, product_data: dict[str, str], root: Path,
                 timeout: float) -> tuple[dict[str, Any], dict[str, Any]]:
    case_root = root / case["id"] / profile / product_data["id"]; case_root.mkdir(parents=True, exist_ok=True)
    source = case["source"]
    if case["mode"] == "eval": argv = [product_data["path"], "eval", source, "--format", "json", *features_args(profile)]
    else:
        src = case_root / "probe.typ"; src.write_text(source, encoding="utf-8")
        out = case_root / "out.pdf"; out.unlink(missing_ok=True)
        argv = [product_data["path"], "compile", *features_args(profile), *case["options"], str(src), str(out)]
    receipt = run(argv, timeout); semantic = None; reason = None; observer = None
    witnesses = None
    if case["mode"] == "compile_pdf" and receipt["exit_code"] == 0:
        info = run(["pdfinfo", str(out)], 30); observer = info
        if info["exit_code"] == 0:
            semantic = [line for line in info["stdout"].decode(errors="replace").splitlines() if line.startswith("Tagged:")]
        else: reason = "PARSER_UNSUPPORTED"
    elif case["mode"] == "pdf_tree" and receipt["exit_code"] == 0:
        witnesses = {"tag_morphology": {"complete": False},
                     "vertical_expansion": {"complete": False}}
        try:
            morphology, morphology_observer = qpdf_morphology(out)
            witnesses["tag_morphology"] = {"complete": True, "value": morphology,
                                             "observer": morphology_observer}
        except (ValueError, json.JSONDecodeError):
            morphology = None
        try:
            vertical, vertical_observer = vertical_witness(out)
            witnesses["vertical_expansion"] = {"complete": True, "value": vertical,
                                                "observer": vertical_observer}
        except (ValueError, ET.ParseError):
            vertical = None
        if morphology is None or vertical is None:
            reason = "PARSER_UNSUPPORTED"
        else:
            semantic = {"tag_morphology": morphology, "vertical_expansion": vertical}
    observation = normalize(receipt, product_data, source, semantic, reason)
    if witnesses is not None:
        observation["S07_witnesses"] = witnesses
    raw = {"case": case["id"], "profile": profile, "product": product_data["id"], "argv": argv,
           "exit_code": receipt["exit_code"], "stdout": receipt["stdout"].decode(errors="replace"),
           "stderr": receipt["stderr"].decode(errors="replace"), "reason_code": receipt["reason_code"],
           "observer_argv": observer["argv"] if observer else None,
           "observer_exit": observer["exit_code"] if observer else None,
           "S07_witnesses": witnesses}
    return observation, raw


def command_sentinels(args: argparse.Namespace) -> None:
    oracle = load(args.oracles); expected = {x["id"] for x in oracle["focal_oracles"]}
    executable = {x["id"] for x in cases()}
    if executable != expected - {"S11_EXACT_BOUNDARY_VECTOR"}: raise ValueError("focal oracle/case mismatch")
    products = [product("vanilla", args.vanilla, args.vanilla_sha), product("crystalline", args.crystalline, args.crystalline_sha)]
    raw_root = require_tmp(args.raw_dir, "p1357-sentinels."); jobs=[]
    for case in cases():
        oracle_case = next(x for x in oracle["focal_oracles"] if x["id"] == case["id"])
        for profile in oracle_case["profiles"]: jobs.append((case, profile))
    baseline = None; hashes=[]; raw_rows=[]
    for physical in [jobs, list(reversed(jobs))]:
        for _ in range(2):
            rows=[]
            for case, profile in physical:
                sides={}
                for item in products:
                    observation, raw = execute_case(case, profile, item, raw_root, args.timeout)
                    sides[item["id"]] = observation; raw_rows.append(raw)
                status = "Unknown" if any(not x["complete"] for x in sides.values()) else (
                    "MATCH" if (sides["vanilla"]["outcome_kind"], sides["vanilla"]["canonical_observable_sha256"]) ==
                               (sides["crystalline"]["outcome_kind"], sides["crystalline"]["canonical_observable_sha256"]) else "DIFFERENCE")
                rows.append({"id": case["id"], "profile": profile, "status": status, "sides": sides})
            rows.sort(key=lambda x: (x["id"], PROFILES.index(x["profile"])))
            payload_hash = digest(canonical(rows)); hashes.append(payload_hash)
            if baseline is None: baseline = rows
            elif rows != baseline: raise ValueError("sentinel nondeterminism")
    # No comparable crystalline adapter exists. Preserve the explicitly opaque result;
    # never substitute elapsed compilation/watch time.
    s11 = {"id": "S11_EXACT_BOUNDARY_VECTOR", "profile": "default", "status": "Unknown",
           "reason_code": "PARSER_UNSUPPORTED", "complete": False,
           "missing_symbol": "crystalline::format_duration(Duration) -> Display",
           "clock_fallback_used": False,
           "vanilla_symbol": "typst_utils::format_duration", "boundary_vector": None}
    with (raw_root / "commands.jsonl").open("wb") as stream:
        for row in raw_rows: stream.write(canonical(row) + b"\n")
    results = (baseline or []) + [s11]; counts={}
    for row in results: counts[row["status"]] = counts.get(row["status"], 0) + 1
    s07_pairs = []
    indexed = {(row["id"], row["profile"]): row for row in baseline or []}
    for profile in ["a11y-extras", "html+a11y"]:
        lone = indexed[("S07_LONE_TAGGED_BLOCK", profile)]
        multiple = indexed[("S07_MULTIPLE_CHILD_CONTROL", profile)]
        for item in products:
            lone_w = lone["sides"][item["id"]].get("S07_witnesses", {}).get("vertical_expansion")
            multiple_w = multiple["sides"][item["id"]].get("S07_witnesses", {}).get("vertical_expansion")
            complete = bool(lone_w and multiple_w and lone_w["complete"] and multiple_w["complete"])
            s07_pairs.append({"profile": profile, "product": item["id"], "complete": complete,
                              "status": "OBSERVED" if complete else "Unknown",
                              "reason_code": None if complete else "PARSER_UNSUPPORTED",
                              "lone_value_sha256": digest(canonical(lone_w["value"])) if complete else None,
                              "multiple_value_sha256": digest(canonical(multiple_w["value"])) if complete else None})
    emit({"schema": "p1357-focal-sentinels-v2", "verdict": "PASS_WITH_DECLARED_LIMITATION",
          "products": products, "determinism_hashes": hashes, "counts": counts,
          "S07_vertical_lone_vs_multiple": s07_pairs,
          "raw_directory": str(raw_root), "results": results}, args.output)


def parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(description=__doc__); sub = p.add_subparsers(dest="command", required=True)
    g = sub.add_parser("gate"); g.add_argument("--step", required=True); g.add_argument("--oracles", required=True); g.add_argument("--mutants", required=True); g.add_argument("--output"); g.set_defaults(func=command_gate)
    i = sub.add_parser("inventory"); i.add_argument("--vanilla-enumerator", required=True); i.add_argument("--crystalline-enumerator", required=True); i.add_argument("--vanilla-binary", required=True); i.add_argument("--crystalline-binary", required=True); i.add_argument("--vanilla-sha"); i.add_argument("--crystalline-sha"); i.add_argument("--seeds", required=True); i.add_argument("--raw-dir"); i.add_argument("--timeout", type=float, default=900); i.add_argument("--output"); i.set_defaults(func=command_inventory)
    s = sub.add_parser("sentinels"); s.add_argument("--oracles", required=True); s.add_argument("--vanilla", required=True); s.add_argument("--crystalline", required=True); s.add_argument("--vanilla-sha"); s.add_argument("--crystalline-sha"); s.add_argument("--raw-dir"); s.add_argument("--timeout", type=float, default=30); s.add_argument("--output"); s.set_defaults(func=command_sentinels)
    return p


def main() -> None:
    try:
        args = parser().parse_args(); args.func(args)
    except (ValueError, KeyError, FileNotFoundError, subprocess.CalledProcessError) as exc:
        print(json.dumps({"schema": "p1357-error-v1", "verdict": "INVALID", "error": str(exc)}, ensure_ascii=False), file=sys.stderr)
        raise SystemExit(2)


if __name__ == "__main__": main()
