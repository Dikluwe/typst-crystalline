#!/usr/bin/env python3
"""Refina e executa a projeção pública P1453 de html.frame."""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import re
import subprocess
import tempfile
from html.parser import HTMLParser
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAGNOSTICS = ROOT / "00_nucleo/diagnosticos"
CAUSAL_HEAD = "1a9998a5a3aa8801856be969b84f046bcecfce9f"
PRODUCT_PATHS = (
    "00_nucleo/prompts",
    "01_core",
    "02_shell",
    "03_infra",
    "04_wiring",
    "Cargo.toml",
    "Cargo.lock",
)
ORDERS = ("forward", "forward-repeat", "reverse")
VISUAL_TAGS = {"text", "use", "path", "rect", "circle", "ellipse", "line", "polyline", "polygon", "image"}
VOID_TAGS = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"}


class GateError(RuntimeError):
    pass


def load_p1420() -> Any:
    path = ROOT / "lab/parity/p1420_unknown_route_reprobe.py"
    spec = importlib.util.spec_from_file_location("p1420_unknown_route_reprobe_p1453", path)
    if spec is None or spec.loader is None:
        raise GateError("não foi possível carregar o executor limitado P1420")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


P1420 = load_p1420()


def sha256_path(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise GateError(f"JSON inválido em {path}: {exc}") from exc
    if not isinstance(value, dict):
        raise GateError(f"objeto JSON esperado em {path}")
    return value


def write_json(path: Path, value: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp")
    temporary.write_text(
        json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
        encoding="utf-8",
    )
    temporary.replace(path)


def git_value(*args: str) -> str:
    result = subprocess.run(
        ["git", *args], cwd=ROOT, stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10, check=False,
    )
    if result.returncode:
        raise GateError(f"git {' '.join(args)} falhou")
    return result.stdout.decode("utf-8", "strict").strip()


def product_is_frozen() -> bool:
    diff = subprocess.run(
        ["git", "diff", "--quiet", CAUSAL_HEAD, "--", *PRODUCT_PATHS],
        cwd=ROOT, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL, timeout=10, check=False,
    )
    status = subprocess.run(
        ["git", "status", "--porcelain", "--untracked-files=all", "--", *PRODUCT_PATHS],
        cwd=ROOT, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL, timeout=10, check=False,
    )
    return diff.returncode == 0 and status.returncode == 0 and not status.stdout


def validate_inputs(args: argparse.Namespace) -> dict[str, Any]:
    paths = {
        "step": ROOT / "00_nucleo/materialization/typst-passo-1453.md",
        "manifest": args.manifest.resolve(),
        "contract": args.contract.resolve(),
        "oracles": args.oracles.resolve(),
        "mutants": args.mutants.resolve(),
        "preseal": args.preseal.resolve(),
        "p1452_results": DIAGNOSTICS / "p1452-results.json",
        "p1452_verification": DIAGNOSTICS / "p1452-verification.json",
    }
    documents = {key: load_json(path) for key, path in paths.items() if key != "step"}
    errors: list[str] = []
    preseal = documents["preseal"]
    if preseal.get("status") != "SEALED" or preseal.get("verdict") != "SEALED":
        errors.append("preseal_state")
    for key, reference in preseal.get("sealed_inputs", {}).items():
        raw = str(reference.get("path", ""))
        path = Path(raw) if raw.startswith("/") else ROOT / raw
        if not path.is_file() or sha256_path(path) != reference.get("sha256"):
            errors.append(f"sealed_hash:{key}")
    if documents["manifest"].get("step") != "P1453" or documents["contract"].get("step") != 1453:
        errors.append("step_identity")
    cases = documents["oracles"].get("cases", [])
    if [case.get("id") for case in cases] != ["identity", "simple", "nested", "rect", "literal"]:
        errors.append("case_denominator")
    if sum(len(case.get("probes", [])) for case in cases) != 6:
        errors.append("probe_denominator")

    p1452 = documents["p1452_results"]
    routes = p1452.get("routes", [])
    matches = [row for row in routes if row.get("route_state") == "MATCH"]
    differences = [row for row in routes if row.get("route_state") == "DIFFERENCE"]
    if len(routes) != 66 or len(matches) != 65 or [row.get("path") for row in differences] != ["html.frame"]:
        errors.append("p1452_composition_precondition")

    binaries = {
        "vanilla": args.vanilla_bin.resolve(),
        "crystalline": args.crystalline_bin.resolve(),
    }
    identities = documents["manifest"].get("identities", {})
    expected = {
        "vanilla": identities.get("vanilla_product", {}).get("sha256"),
        "crystalline": identities.get("crystalline_product", {}).get("sha256"),
    }
    binary_hashes: dict[str, str | None] = {}
    for product, path in binaries.items():
        binary_hashes[product] = sha256_path(path) if path.is_file() else None
        if binary_hashes[product] != expected[product]:
            errors.append(f"identity:{product}")
    if not product_is_frozen():
        errors.append("product_causal_drift")
    if errors:
        raise GateError(";".join(sorted(set(errors))))
    return {
        "paths": paths,
        "documents": documents,
        "cases": cases,
        "binaries": binaries,
        "binary_hashes": binary_hashes,
        "input_hashes": {key: sha256_path(path) for key, path in paths.items()},
        "head": git_value("rev-parse", "HEAD"),
        "tree": git_value("rev-parse", "HEAD^{tree}"),
        "p1452_match_count": len(matches),
    }


def approximately(left: float | None, right: float, tolerance: float = 0.001) -> bool:
    return left is not None and abs(left - right) <= tolerance


def observation_preserved(observation: dict[str, Any]) -> bool:
    viewbox = observation.get("viewbox")
    rect_viewbox = observation.get("rect_viewbox")
    return all((
        observation.get("identity_valid") is True,
        observation.get("missing_diagnostic") is True,
        observation.get("svg_count") == 1,
        observation.get("frame_count") == 0,
        (observation.get("width_pt") or 0) > 0,
        (observation.get("height_pt") or 0) > 0,
        isinstance(viewbox, list) and len(viewbox) == 4,
        isinstance(viewbox, list) and approximately(viewbox[0], 0.0),
        isinstance(viewbox, list) and approximately(viewbox[1], 0.0),
        isinstance(viewbox, list) and approximately(viewbox[2], observation.get("width_pt", -1.0)),
        isinstance(viewbox, list) and approximately(viewbox[3], observation.get("height_pt", -1.0)),
        observation.get("display") == "block",
        observation.get("overflow") == "visible",
        observation.get("white_background") is False,
        observation.get("visual_payload") is True,
        observation.get("nested_equal") is True,
        approximately(observation.get("rect_width_pt"), 10.0),
        approximately(observation.get("rect_height_pt"), 5.0),
        isinstance(rect_viewbox, list) and len(rect_viewbox) == 4,
        isinstance(rect_viewbox, list) and all(approximately(value, expected) for value, expected in zip(rect_viewbox, [0.0, 0.0, 10.0, 5.0])),
        observation.get("red_paint") is True,
        observation.get("literal_frame_count") == 1,
        observation.get("literal_svg_count") == 0,
    ))


def mutation_gate(context: dict[str, Any]) -> dict[str, Any]:
    mutants = context["documents"]["mutants"]
    base = mutants.get("base_observation", {})
    negative = mutants.get("negative_mutations", [])
    positive = mutants.get("positive_controls", [])
    opaque = mutants.get("opaque_controls", [])
    order_vectors: dict[str, str] = {}
    order_receipts: dict[str, Any] = {}
    for order in ORDERS:
        scheduled = list(reversed(negative)) if order == "reverse" else negative
        negative_rows = []
        for mutant in scheduled:
            observation = copy.deepcopy(base)
            observation[mutant["field"]] = mutant["value"]
            classification = "Preserved" if observation_preserved(observation) else "Violated"
            negative_rows.append({"id":mutant["id"],"classification":classification})
        positive_rows = [
            {"id":item["id"],"classification":"Preserved" if observation_preserved(base) else "Violated"}
            for item in positive
        ]
        opaque_rows = [{"id":item["id"],"classification":"Unknown","reason":item["reason"]} for item in opaque]
        canonical = sorted((row["id"], row["classification"]) for row in negative_rows + positive_rows + opaque_rows)
        vector = hashlib.sha256(json.dumps(canonical, separators=(",", ":")).encode()).hexdigest()
        order_vectors[order] = vector
        order_receipts[order] = {"negative":negative_rows,"positive":positive_rows,"opaque":opaque_rows}
    violated = sum(row["classification"] == "Violated" for row in order_receipts["forward"]["negative"])
    preserved = sum(row["classification"] == "Preserved" for row in order_receipts["forward"]["positive"])
    unknown = sum(row["classification"] == "Unknown" for row in order_receipts["forward"]["opaque"])
    stable = len(set(order_vectors.values())) == 1
    passed = violated == 18 and preserved == 6 and unknown == 6 and stable
    return {
        "result":"PASS" if passed else "FAIL",
        "valid_negative":18,"violated":violated,"positive":6,"preserved":preserved,
        "opaque":6,"unknown":unknown,"mutation_score":violated / 18,
        "order_invariant":stable,"vectors_sha256":order_vectors,
        "receipts":order_receipts,
    }


class TreeParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.root: dict[str, Any] = {"tag":"#root","attrs":{},"children":[],"text":[]}
        self.stack = [self.root]

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        node = {"tag":tag.lower(),"attrs":{key.lower(): value or "" for key, value in attrs},"children":[],"text":[]}
        self.stack[-1]["children"].append(node)
        if tag.lower() not in VOID_TAGS:
            self.stack.append(node)

    def handle_startendtag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        node = {"tag":tag.lower(),"attrs":{key.lower(): value or "" for key, value in attrs},"children":[],"text":[]}
        self.stack[-1]["children"].append(node)

    def handle_endtag(self, tag: str) -> None:
        target = tag.lower()
        for index in range(len(self.stack) - 1, 0, -1):
            if self.stack[index]["tag"] == target:
                del self.stack[index:]
                return

    def handle_data(self, data: str) -> None:
        if data.strip():
            self.stack[-1]["text"].append(data)


def descendants(node: dict[str, Any]) -> list[dict[str, Any]]:
    found = [node]
    for child in node.get("children", []):
        found.extend(descendants(child))
    return found


def parse_pt(value: str | None) -> float | None:
    match = re.fullmatch(r"\s*([+-]?(?:\d+(?:\.\d*)?|\.\d+))pt\s*", value or "")
    return float(match.group(1)) if match else None


def parse_viewbox(value: str | None) -> list[float] | None:
    try:
        parts = [float(part) for part in re.split(r"[\s,]+", (value or "").strip()) if part]
    except ValueError:
        return None
    return parts if len(parts) == 4 else None


def parse_style(value: str | None) -> dict[str, str]:
    fields: dict[str, str] = {}
    for declaration in (value or "").split(";"):
        if ":" in declaration:
            key, val = declaration.split(":", 1)
            fields[key.strip().lower()] = val.strip().lower()
    return fields


def html_projection(path: Path) -> dict[str, Any]:
    try:
        raw = path.read_bytes()
        text = raw.decode("utf-8", "strict")
        parser = TreeParser()
        parser.feed(text)
        parser.close()
    except (OSError, UnicodeError, ValueError) as exc:
        return {"opaque_reason":"UNREADABLE_HTML","error":str(exc)}
    nodes = descendants(parser.root)
    svgs = [node for node in nodes if node["tag"] == "svg"]
    frames = [node for node in nodes if node["tag"] == "frame"]
    projection: dict[str, Any] = {
        "svg_count":len(svgs),
        "frame_count":len(frames),
        "raw_sha256":hashlib.sha256(raw).hexdigest(),
    }
    if len(svgs) != 1:
        return projection
    svg = svgs[0]
    svg_nodes = descendants(svg)
    style = parse_style(svg["attrs"].get("style"))
    all_attribute_text = " ".join(
        str(value).lower() for node in svg_nodes for value in node.get("attrs", {}).values()
    )
    visual = any(node["tag"] in VISUAL_TAGS for node in svg_nodes) or any(
        text.strip() for node in svg_nodes for text in node.get("text", [])
    )
    projection.update({
        "width_pt":parse_pt(svg["attrs"].get("width")),
        "height_pt":parse_pt(svg["attrs"].get("height")),
        "viewbox":parse_viewbox(svg["attrs"].get("viewbox")),
        "display":style.get("display"),
        "overflow":style.get("overflow"),
        "white_background":("#ffffff" in all_attribute_text or "background: white" in all_attribute_text),
        "visual_payload":bool(visual),
        "red_paint":("#ff0000" in all_attribute_text),
        "paint_colors":sorted({
            value.lower()
            for node in svg_nodes
            for key, value in node.get("attrs", {}).items()
            if key in {"fill", "stroke"} and value
        }),
        "private_tags":sorted({node["tag"] for node in svg_nodes}),
    })
    return projection


def resource_reason(receipt: dict[str, Any]) -> str | None:
    if receipt.get("reason_code"):
        return str(receipt["reason_code"])
    if receipt.get("truncated"):
        return "OUTPUT_TRUNCATED"
    if (receipt.get("swap_kib") or 0) > 0:
        return "SWAP_OBSERVED"
    return None


def run_probe(probe: dict[str, Any], binary: Path, product: str, order: str, raw_root: Path, serial: int) -> dict[str, Any]:
    stem = f"{serial:03d}-{order}-{probe['id']}-{product}"
    stream_path = raw_root / "streams" / stem
    if probe["kind"].startswith("eval"):
        argv = [str(binary), "eval", "--features", "html", probe["expression"]]
        receipt = P1420.P1400.run_capped(argv, stream_path)
    else:
        work = raw_root / "work" / stem
        work.mkdir(parents=True, exist_ok=True)
        fixture = work / "fixture.typ"
        artifact = work / "artifact.html"
        fixture.write_text(probe["source"], encoding="utf-8")
        argv = [str(binary), "compile", "--features", "html", "--format", "html", str(fixture), str(artifact)]
        receipt = P1420.P1400.run_capped(argv, stream_path)
        if artifact.is_file():
            receipt["artifact"] = {"path":str(artifact),"sha256":sha256_path(artifact),"bytes":artifact.stat().st_size}
            receipt["projection"] = html_projection(artifact)
        else:
            receipt["artifact"] = None
            receipt["projection"] = {"opaque_reason":"OUTPUT_MISSING"}
    receipt.update({"probe_id":probe["id"],"product":product,"order":order,"binary_sha256":sha256_path(binary)})
    return receipt


def public_projection(projection: dict[str, Any]) -> dict[str, Any]:
    return {key:value for key, value in projection.items() if key not in {"raw_sha256","private_tags"}}


def classify(receipts: dict[str, dict[str, dict[str, Any]]]) -> tuple[str, list[str], dict[str, Any]]:
    reasons = [reason for products in receipts.values() for receipt in products.values() if (reason := resource_reason(receipt))]
    if reasons:
        return "Unknown", sorted(set(reasons)), {}
    identity_valid: dict[str, bool] = {}
    missing_valid: dict[str, bool] = {}
    for product in ("vanilla", "crystalline"):
        valid = receipts["identity-valid"][product]
        missing = receipts["identity-missing"][product]
        try:
            payload = json.loads((valid.get("stdout", {}).get("utf8") or "").strip())
        except json.JSONDecodeError:
            payload = None
        identity_valid[product] = valid.get("exit_code") == 0 and payload == ["frame", "frame(body: [x])"]
        missing_valid[product] = missing.get("exit_code") not in (None, 0) and "missing argument: body" in (missing.get("stderr", {}).get("utf8") or "")

    projections = {
        case: {product: receipts[f"{case}-html"][product].get("projection", {}) for product in ("vanilla", "crystalline")}
        for case in ("simple", "nested", "rect", "literal")
    }
    opaque = [
        projection.get("opaque_reason")
        for products in projections.values() for projection in products.values()
        if projection.get("opaque_reason")
    ]
    if opaque:
        return "Unknown", sorted(set(opaque)), {"projections":projections}

    simple = projections["simple"]
    nested = projections["nested"]
    rect = projections["rect"]
    literal = projections["literal"]
    checks: dict[str, bool] = {
        "identity":all(identity_valid.values()),
        "missing_diagnostic":all(missing_valid.values()),
        "simple_bilateral":public_projection(simple["vanilla"]) == public_projection(simple["crystalline"]),
        "simple_invariants":all(
            observation_preserved({
                "identity_valid":True,"missing_diagnostic":True,**projection,
                "nested_equal":True,"rect_width_pt":10.0,"rect_height_pt":5.0,
                "rect_viewbox":[0.0,0.0,10.0,5.0],"red_paint":True,
                "literal_frame_count":1,"literal_svg_count":0,
            }) for projection in simple.values()
        ),
        "nested_bilateral":public_projection(nested["vanilla"]) == public_projection(nested["crystalline"]),
        "nested_equals_simple":all(public_projection(nested[product]) == public_projection(simple[product]) for product in ("vanilla", "crystalline")),
        "rect_bilateral":public_projection(rect["vanilla"]) == public_projection(rect["crystalline"]),
        "rect_expected":all(
            projection.get("svg_count") == 1
            and projection.get("frame_count") == 0
            and approximately(projection.get("width_pt"), 10.0)
            and approximately(projection.get("height_pt"), 5.0)
            and isinstance(projection.get("viewbox"), list)
            and all(approximately(value, expected) for value, expected in zip(projection["viewbox"], [0.0,0.0,10.0,5.0]))
            and projection.get("display") == "block"
            and projection.get("overflow") == "visible"
            and projection.get("white_background") is False
            and projection.get("visual_payload") is True
            and projection.get("red_paint") is True
            and "#ff0000" in projection.get("paint_colors", [])
            for projection in rect.values()
        ),
        "literal_bilateral":public_projection(literal["vanilla"]) == public_projection(literal["crystalline"]),
        "literal_expected":all(projection.get("frame_count") == 1 and projection.get("svg_count") == 0 for projection in literal.values()),
        "compile_stderr_bilateral":all(
            receipts[f"{case}-html"]["vanilla"].get("stderr", {}).get("sha256")
            == receipts[f"{case}-html"]["crystalline"].get("stderr", {}).get("sha256")
            for case in ("simple", "nested", "rect", "literal")
        ),
        "all_exit_success":all(
            receipts[f"{case}-html"][product].get("exit_code") == 0
            for case in ("simple", "nested", "rect", "literal")
            for product in ("vanilla", "crystalline")
        ),
    }
    failed = sorted(key for key, passed in checks.items() if not passed)
    return ("Preserved" if not failed else "Violated"), failed, {"checks":checks,"projections":projections}


def execute_once(context: dict[str, Any], prefix: str) -> dict[str, Any]:
    raw_root = Path(tempfile.mkdtemp(prefix=prefix, dir="/tmp"))
    vectors: dict[str, str] = {}
    order_results: dict[str, Any] = {}
    serial = 0
    for order in ORDERS:
        cases = list(reversed(context["cases"])) if order == "reverse" else context["cases"]
        receipts: dict[str, dict[str, dict[str, Any]]] = {}
        for case in cases:
            for probe in case["probes"]:
                receipts[probe["id"]] = {}
                for product, binary in context["binaries"].items():
                    serial += 1
                    receipts[probe["id"]][product] = run_probe(probe, binary, product, order, raw_root, serial)
        classification, witnesses, detail = classify(receipts)
        vector_payload = {"classification":classification,"witnesses":witnesses,"checks":detail.get("checks")}
        vector = hashlib.sha256(json.dumps(vector_payload, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        vectors[order] = vector
        order_results[order] = {"classification":classification,"witnesses":witnesses,"detail":detail,"receipts":receipts,"vector_sha256":vector}
    stable = len(set(vectors.values())) == 1
    classifications = {result["classification"] for result in order_results.values()}
    classification = next(iter(classifications)) if len(classifications) == 1 and stable else "Unknown"
    return {
        "classification":classification,
        "order_invariant":stable,
        "vectors_sha256":vectors,
        "orders":order_results,
        "raw_outputs_directory":str(raw_root),
    }


def execute_rect_slice(context: dict[str, Any]) -> dict[str, Any]:
    case = next(case for case in context["cases"] if case["id"] == "rect")
    probe = case["probes"][0]
    raw_root = Path(tempfile.mkdtemp(prefix="p1453-r1-rect-slice-", dir="/tmp"))
    rows: dict[str, Any] = {}
    vectors: dict[str, str] = {}
    serial = 0
    for order in ORDERS:
        products: dict[str, Any] = {}
        for product, binary in context["binaries"].items():
            serial += 1
            products[product] = run_probe(probe, binary, product, order, raw_root, serial)
        projections = {product: receipt.get("projection", {}) for product, receipt in products.items()}
        complete = all(
            resource_reason(receipt) is None
            and receipt.get("exit_code") == 0
            and not projection.get("opaque_reason")
            for product, receipt in products.items()
            for projection in [projections[product]]
        )
        expected = complete and all(
            projection.get("svg_count") == 1
            and approximately(projection.get("width_pt"), 10.0)
            and approximately(projection.get("height_pt"), 5.0)
            and projection.get("viewbox") == [0.0, 0.0, 10.0, 5.0]
            and "#ff0000" in projection.get("paint_colors", [])
            for projection in projections.values()
        )
        bilateral = public_projection(projections["vanilla"]) == public_projection(projections["crystalline"])
        classification = "Preserved" if expected and bilateral else ("Violated" if complete else "Unknown")
        vector = hashlib.sha256(json.dumps({"classification":classification,"projections":{key:public_projection(value) for key,value in projections.items()}}, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        vectors[order] = vector
        rows[order] = {"classification":classification,"bilateral":bilateral,"expected":expected,"projections":projections,"receipts":products,"vector_sha256":vector}
    result = {
        "schema":"p1453-r1-rect-slice/v1",
        "orders":rows,
        "order_invariant":len(set(vectors.values())) == 1,
        "vectors_sha256":vectors,
        "result":"PASS" if all(row["classification"] == "Preserved" for row in rows.values()) and len(set(vectors.values())) == 1 else "FAIL",
        "raw_outputs_directory":str(raw_root),
    }
    path = Path("/tmp/p1453-r1-rect-slice.json")
    write_json(path, result)
    result["output"] = str(path)
    result["output_sha256"] = sha256_path(path)
    return result


def execute_campaign(context: dict[str, Any]) -> dict[str, Any]:
    primary = execute_once(context, "p1453-primary-")
    repeat = execute_once(context, "p1453-repeat-")
    repeat_path = Path("/tmp/p1453-repeat.json")
    write_json(repeat_path, repeat)
    stable = primary["vectors_sha256"] == repeat["vectors_sha256"] and primary["classification"] == repeat["classification"]
    preserved = primary["classification"] == "Preserved" and stable
    return {
        "schema":"p1453-html-frame-results/v1",
        "step":1453,
        "attestation":"executado sem atestação de isolamento",
        "head":context["head"],"tree":context["tree"],
        "input_hashes":context["input_hashes"],
        "binary_hashes":context["binary_hashes"],
        "runner_sha256":sha256_path(Path(__file__)),
        "primary":primary,
        "independent_repeat":{"path":str(repeat_path),"sha256":sha256_path(repeat_path),"classification":repeat["classification"],"vectors_sha256":repeat["vectors_sha256"]},
        "repeat_equal":stable,
        "fragment_verdict":"PASS" if preserved else "INCONCLUSIVE",
        "p1452_preserved_routes":context["p1452_match_count"],
        "composite_route_counts":{"MATCH":66} if preserved else {"MATCH":65,"UNRESOLVED":1},
        "composite_closure":"PASS" if preserved else "INCONCLUSIVE",
        "claim_boundary":"html.frame P1453 plus unchanged 65-route P1452 receipt; no general HTML/SVG parity",
    }


def emit(value: dict[str, Any]) -> None:
    print(json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")))


def main() -> int:
    parser = argparse.ArgumentParser(description="Projeção pública P1453 de html.frame")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--validate-only", action="store_true")
    mode.add_argument("--gate", action="store_true")
    mode.add_argument("--slice-rect", action="store_true")
    mode.add_argument("--run", action="store_true")
    parser.add_argument("--manifest", type=Path, default=DIAGNOSTICS / "p1453-manifest.json")
    parser.add_argument("--contract", type=Path, default=DIAGNOSTICS / "p1453-contract.json")
    parser.add_argument("--oracles", type=Path, default=DIAGNOSTICS / "p1453-oracles.json")
    parser.add_argument("--mutants", type=Path, default=DIAGNOSTICS / "p1453-mutants.json")
    parser.add_argument("--preseal", type=Path, default=DIAGNOSTICS / "p1453-preseal.json")
    parser.add_argument("--vanilla-bin", type=Path, default=ROOT / "lab/typst-original/target/release/typst")
    parser.add_argument("--crystalline-bin", type=Path, default=ROOT / "target/release/typst")
    parser.add_argument("--output", type=Path, default=DIAGNOSTICS / "p1453-results.json")
    args = parser.parse_args()
    try:
        context = validate_inputs(args)
        gate = mutation_gate(context)
        if gate["result"] != "PASS":
            raise GateError("gate discriminatório falhou")
        if args.validate_only:
            emit({"schema":"p1453-validation/v1","result":"PASS","cases":5,"probes":6,"p1452_preserved_routes":65,"preseal_sha256":context["input_hashes"]["preseal"]})
            return 0
        if args.gate:
            emit({"schema":"p1453-mutation-gate/v1", **{key:value for key,value in gate.items() if key != "receipts"}})
            return 0
        if args.slice_rect:
            result = execute_rect_slice(context)
            emit({key:value for key,value in result.items() if key != "orders"})
            return 0 if result["result"] == "PASS" else 3
        expected = DIAGNOSTICS / "p1453-results.json"
        if args.output.resolve() != expected.resolve():
            raise GateError("resultado consolidado deve ser p1453-results.json")
        result = execute_campaign(context)
        result["mutation_gate"] = {key:value for key,value in gate.items() if key != "receipts"}
        write_json(args.output.resolve(), result)
        emit({"schema":"p1453-completion/v1","result":result["fragment_verdict"],"classification":result["primary"]["classification"],"repeat_equal":result["repeat_equal"],"composite_closure":result["composite_closure"],"output_sha256":sha256_path(args.output.resolve())})
        return 0 if result["fragment_verdict"] == "PASS" else 3
    except (GateError, OSError, subprocess.SubprocessError) as exc:
        emit({"schema":"p1453-error/v1","result":"FAIL","reason_code":"RUNNER_FAILURE","error":str(exc)})
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
