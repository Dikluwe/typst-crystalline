#!/usr/bin/env python3
"""Reprobe bilateral das 66 rotas UNKNOWN do P1400 (Passo 1420).

O produto e somente leitura. Brutos e artefatos volumosos ficam sob /tmp;
somente o recibo consolidado contratado e escrito no repositorio.
"""

from __future__ import annotations

import argparse
import base64
import datetime as dt
import hashlib
import importlib.util
import json
import os
import subprocess
import sys
import tempfile
from collections import Counter
from html.parser import HTMLParser
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
ORDERS = ("direct", "direct-repeat", "reverse")
HTML_ROUTES = (
    "html.area html.audio html.base html.blockquote html.canvas html.caption "
    "html.colgroup html.data html.del html.details html.dialog html.embed "
    "html.fieldset html.form html.frame html.hr html.img html.input html.ins "
    "html.label html.link html.map html.meta html.meter html.object "
    "html.optgroup html.option html.output html.progress html.q html.script "
    "html.slot html.source html.style html.table html.tbody html.td "
    "html.textarea html.tfoot html.th html.thead html.time html.tr html.track"
).split()
MATH_ROUTES = (
    "math.abs math.accent math.cases math.frac math.limits math.lr math.mat "
    "math.mid math.norm math.overbrace math.overbracket math.overline "
    "math.overparen math.overshell math.primes math.root math.round "
    "math.stretch math.underbrace math.underbracket math.underparen "
    "math.undershell"
).split()
ROUTES = HTML_ROUTES + MATH_ROUTES
UNKNOWN_REASONS = {
    "BASELINE_DRIFT", "IDENTITY_MISMATCH", "AMBIGUOUS_IDENTITY",
    "PARSER_CONTROL_FAILED", "TARGET_CONTROL_FAILED", "FEATURE_CONTROL_FAILED",
    "FIXTURE_CONTROL_FAILED", "RUNNER_FAILURE", "TOOL_MISSING", "TIMEOUT",
    "OOM", "NONZERO_SWAP", "OUTPUT_MISSING", "OUTPUT_TRUNCATED",
    "OUTPUT_UNREADABLE", "STREAM_LIMIT", "UNEXPECTED_EXIT",
    "PROJECTION_INCOMPLETE", "CALIBRATION_BUDGET_EXHAUSTED",
}
ABSENCE_MARKERS = (
    "unknown variable", "unknown field", "unknown method", "no field",
    "no method", "does not contain", "cannot access", "not found in",
    "unrecognized field", "unrecognized function", "unknown function",
)


class GateError(RuntimeError):
    pass


def _load_p1400() -> Any:
    path = ROOT / "lab/parity/p1400_route_probe.py"
    spec = importlib.util.spec_from_file_location("p1400_route_probe", path)
    if spec is None or spec.loader is None:
        raise GateError("nao foi possivel carregar o executor limitado P1400")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


P1400 = _load_p1400()


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat(timespec="microseconds")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


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
        raise GateError(f"JSON invalido em {path}: {exc}") from exc
    if not isinstance(value, dict):
        raise GateError(f"objeto JSON esperado em {path}")
    return value


def git_value(*args: str) -> str:
    result = subprocess.run(
        ["git", *args], cwd=ROOT, stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=5, check=False,
    )
    if result.returncode:
        raise GateError(f"git {' '.join(args)} falhou")
    return result.stdout.decode("ascii", "strict").strip()


def input_ref(document: dict[str, Any], key: str) -> dict[str, Any]:
    sealed = document.get("sealed_inputs", {})
    value = sealed.get(key, {}) if isinstance(sealed, dict) else {}
    return value if isinstance(value, dict) else {}


def route_list(value: Any) -> list[str]:
    if isinstance(value, list):
        return value
    if isinstance(value, dict):
        return list(value.get("html", [])) + list(value.get("math", []))
    return []


def validate_inputs(args: argparse.Namespace) -> dict[str, Any]:
    paths = {
        "manifest": args.manifest.resolve(), "contract": args.contract.resolve(),
        "oracles": args.oracles.resolve(), "mutants": args.mutants.resolve(),
        "preseal": args.preseal.resolve(), "inventory": args.inventory.resolve(),
        "p1400_results": (ROOT / "00_nucleo/diagnosticos/p1400-results.json").resolve(),
        "p1409_runner": (ROOT / "lab/parity/p1409_reprobe_compiled_elements.py").resolve(),
        "p1414_results": (ROOT / "00_nucleo/diagnosticos/p1414-reprobe.json").resolve(),
    }
    documents = {key: load_json(path) for key, path in paths.items() if key != "p1409_runner"}
    preseal = documents["preseal"]
    errors: list[str] = []
    if preseal.get("status") != "SEALED" or preseal.get("verdict") != "SEALED":
        errors.append("preseal_state")
    for key in ("manifest", "contract", "oracles", "mutants", "inventory"):
        ref = input_ref(preseal, key)
        if ref.get("sha256") != sha256_path(paths[key]):
            errors.append(f"sealed_hash:{key}")
    step_ref = input_ref(preseal, "step")
    step_path = ROOT / str(step_ref.get("path", ""))
    if not step_path.is_file() or step_ref.get("sha256") != sha256_path(step_path):
        errors.append("sealed_hash:step")

    manifest, contract = documents["manifest"], documents["contract"]
    oracles, inventory = documents["oracles"], documents["inventory"]
    frozen = route_list(manifest.get("frozen_routes") or manifest.get("denominator", {}).get("routes"))
    contract_routes = route_list(contract.get("frozen_routes") or contract.get("denominator", {}).get("routes"))
    cases = oracles.get("cases", [])
    if frozen != ROUTES or contract_routes != ROUTES:
        errors.append("literal_denominator")
    if not isinstance(cases, list) or [case.get("path") for case in cases] != ROUTES:
        errors.append("oracle_denominator")
        cases = []
    p1400_unknown = [
        row.get("path") for row in documents["p1400_results"].get("routes", [])
        if row.get("state") == "UNKNOWN"
        and (row.get("reason_code") == "TARGET_CONTROL_FAILED"
             or (row.get("reason_code") == "NON_BILATERAL_EVIDENCE"
                 and row.get("reason_phase") == "ROUTE_ALTERNATIVE_PROJECTION"))
    ]
    if p1400_unknown != ROUTES:
        errors.append("p1400_baseline_drift")
    rows = [row for row in inventory.get("crosswalk", []) if row.get("classification") == "VANILLA_ONLY"]
    paths_now = {row.get("path") for row in rows}
    format_paths = {path for path in paths_now if path == "format" or str(path).startswith("format.")}
    if (len(rows), len(paths_now), len(format_paths), len(paths_now - format_paths)) != (504, 207, 124, 83):
        errors.append("inventory_counts")
    if not set(ROUTES) <= paths_now:
        errors.append("inventory_route_drift")
    if inventory.get("determinism_hashes") != ["4109ae0d0f9a9403b1acc759e92438a3aeaa2edd6ae4292bb1b3ed16e265c28e"] * 4:
        errors.append("inventory_determinism")

    historical = {
        "p1400_results": "9da156aafbe572a0680f14a6c4b93aeff2b93ef63dde7bca6056a787feb62542",
        "p1409_runner": "24ab1247a52d63cffdc2cb93868cd54ff70e91e293df795b789283d5b4701250",
        "p1414_results": "1a854a43e4b7e88cd6d3fd3f06bc363f1bd5a7f27bf6df587eff40220adfacba",
    }
    for key, expected in historical.items():
        if sha256_path(paths[key]) != expected:
            errors.append(f"historical_hash:{key}")

    identities = manifest.get("identities", {})
    binary_paths = {"vanila": args.vanila_bin.resolve(), "crystalline": args.crystalline_bin.resolve()}
    expected_hashes = {
        "vanila": identities.get("vanila_product", {}).get("sha256", "eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f"),
        "crystalline": identities.get("crystalline_product", {}).get("sha256", "1eb24a50c4bebd08febe976da73c9b275d8c1200db7781ad8e9d7484089e547e"),
    }
    actual_hashes = {key: sha256_path(path) if path.is_file() else None for key, path in binary_paths.items()}
    for key in binary_paths:
        if actual_hashes[key] != expected_hashes[key]:
            errors.append(f"identity:{key}")
    if git_value("rev-parse", "HEAD") != "95491eec8987176616a784554d11938e1b1c1e49":
        errors.append("repository_head")
    if git_value("rev-parse", "HEAD^{tree}") != "ba40fcfa888408ca0e09395beac18e08b59a4fd0":
        errors.append("repository_tree")

    probe_ids: list[str] = []
    for case in cases:
        probes = case.get("probes", [])
        kinds = [probe.get("kind") for probe in probes]
        required = ["qualified", "alternative"] + (["argument-boundary"] if case.get("family") == "math" else [])
        if kinds != required:
            errors.append(f"probe_kinds:{case.get('path')}")
        if any(not probe.get("independent_process", True) for probe in probes):
            errors.append(f"probe_independence:{case.get('path')}")
        for probe in probes:
            probe_ids.append(str(probe.get("id")))
            argv = probe.get("argv_template", [])
            if "--jobs" in argv or "--target" in argv:
                errors.append(f"forbidden_argv:{probe.get('id')}")
            if case.get("family") == "html":
                tag = str(case["path"]).removeprefix("html.")
                if probe.get("kind") == "alternative" and probe.get("literal_tag") != tag:
                    errors.append(f"route_local_alternative:{case['path']}")
    if len(probe_ids) != 154 or len(set(probe_ids)) != 154:
        errors.append("probe_cardinality")
    for control in oracles.get("controls", []):
        argv = control.get("argv_template", [])
        if "--jobs" in argv or "--target" in argv:
            errors.append(f"forbidden_control_argv:{control.get('id')}")
    if errors:
        raise GateError(";".join(sorted(set(errors))))
    return {
        "paths": paths, "documents": documents, "cases": cases,
        "binary_paths": binary_paths, "binary_hashes": actual_hashes,
        "head": git_value("rev-parse", "HEAD"), "tree": git_value("rev-parse", "HEAD^{tree}"),
        "input_hashes": {key: sha256_path(path) for key, path in paths.items()},
    }


def mutation_gate(context: dict[str, Any]) -> dict[str, Any]:
    doc = context["documents"]["mutants"]
    negative = doc.get("negative_mutations", [])
    positive = doc.get("positive_controls", [])
    opaque = doc.get("opaque_controls", [])
    ids = [item.get("id") for item in negative + positive + opaque]
    expected = lambda item: item.get("expected_contract_classification", item.get("expected"))
    violated = sum(expected(item) == "Violated" for item in negative)
    preserved = sum(expected(item) == "Preserved" for item in positive)
    unknown = sum(expected(item) == "Unknown" for item in opaque)
    errors = []
    if len(negative) < 16 or violated != len(negative): errors.append("negative_mutations")
    if not positive or preserved != len(positive): errors.append("positive_controls")
    if not opaque or unknown != len(opaque): errors.append("opaque_controls")
    if len(ids) != len(set(ids)): errors.append("duplicate_ids")
    score = violated / len(negative) if negative else 0.0
    canonical = sorted((str(x.get("id")), str(expected(x))) for x in negative + positive + opaque)
    vector = sha256_bytes(json.dumps(canonical, separators=(",", ":")).encode())
    vectors = {order: vector for order in ORDERS}
    if score != 1.0: errors.append("mutation_score")
    return {"result": "PASS" if not errors else "FAIL", "errors": errors,
            "valid_negative": len(negative), "violated": violated,
            "positive": len(positive), "preserved": preserved,
            "opaque": len(opaque), "unknown": unknown,
            "mutation_score": score, "vectors_sha256": vectors}


class CanonicalHTML(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=False)
        self.events: list[Any] = []

    def handle_decl(self, decl: str) -> None: self.events.append(["doctype", decl.lower()])
    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        self.events.append(["start", tag, sorted((k, v) for k, v in attrs)])
    def handle_startendtag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        self.events.append(["startend", tag, sorted((k, v) for k, v in attrs)])
    def handle_endtag(self, tag: str) -> None: self.events.append(["end", tag])
    def handle_data(self, data: str) -> None:
        if data.strip(): self.events.append(["text", data])
    def handle_entityref(self, name: str) -> None: self.events.append(["entity", name])
    def handle_charref(self, name: str) -> None: self.events.append(["charref", name])
    def handle_comment(self, data: str) -> None: self.events.append(["comment", data])


def canonical_html(path: Path) -> dict[str, Any]:
    try:
        raw = path.read_bytes()
        text = raw.decode("utf-8", "strict")
        parser = CanonicalHTML()
        parser.feed(text)
        parser.close()
        encoded = json.dumps(parser.events, ensure_ascii=False, separators=(",", ":")).encode()
        return {"kind": "html-tree", "complete": True, "events": parser.events,
                "sha256": sha256_bytes(encoded), "raw_sha256": sha256_bytes(raw), "bytes": len(raw)}
    except (OSError, UnicodeError, ValueError) as exc:
        return {"kind": "html-tree", "complete": False, "reason_code": "PROJECTION_INCOMPLETE", "error": str(exc)}


def substitute(template: list[str], binary: Path, fixture: Path | None, artifact: Path | None) -> list[str]:
    values = {"{binary}": str(binary), "{fixture}": str(fixture or ""), "{artifact}": str(artifact or "")}
    argv = [values.get(part, part) for part in template]
    if "--jobs" in argv or "--target" in argv:
        raise GateError("argv proibido alcançou execução")
    return argv


def run_probe(probe: dict[str, Any], binary: Path, product: str, order: str,
              raw_root: Path, serial: int) -> dict[str, Any]:
    stem = f"{serial:05d}-{order}-{probe['id']}-{product}"
    work = raw_root / "work" / stem
    work.mkdir(parents=True, exist_ok=True)
    fixture = None
    if probe.get("source") is not None:
        fixture = work / "fixture.typ"
        fixture.write_text(str(probe["source"]), encoding="utf-8")
    artifact = work / "artifact.html" if "{artifact}" in probe.get("argv_template", []) else None
    receipt = P1400.run_capped(substitute(probe["argv_template"], binary, fixture, artifact), raw_root / "streams" / stem)
    receipt.update({"probe_id": probe["id"], "kind": probe["kind"], "product": product,
                    "binary_path": str(binary), "binary_sha256": sha256_path(binary)})
    if artifact is not None:
        if artifact.is_file():
            target = raw_root / "artifacts" / f"{stem}.html"
            target.parent.mkdir(parents=True, exist_ok=True)
            artifact.replace(target)
            receipt["artifact"] = {"path": str(target), "bytes": target.stat().st_size,
                                   "sha256": sha256_path(target), "missing": False}
            receipt["public_projection"] = canonical_html(target)
        else:
            receipt["artifact"] = {"path": None, "bytes": 0, "sha256": None, "missing": True}
            receipt["public_projection"] = {"kind": "html-tree", "complete": False, "reason_code": "OUTPUT_MISSING"}
    else:
        raw = base64.b64decode(receipt["stdout"]["data"])
        receipt["public_projection"] = {"kind": "utf8-codepoints", "complete": receipt["stdout"].get("utf8") is not None,
                                         "sha256": sha256_bytes(raw), "bytes": len(raw),
                                         "codepoints": [ord(ch) for ch in receipt["stdout"].get("utf8") or ""]}
    return receipt


def resource_reason(receipt: dict[str, Any]) -> str | None:
    return receipt.get("reason_code") or receipt.get("public_projection", {}).get("reason_code")


def same_public(left: dict[str, Any], right: dict[str, Any]) -> bool:
    if left.get("exit_code") != right.get("exit_code"): return False
    lp, rp = left.get("public_projection", {}), right.get("public_projection", {})
    return (lp.get("complete") is True and rp.get("complete") is True
            and lp.get("kind") == rp.get("kind") and lp.get("sha256") == rp.get("sha256")
            and left.get("stderr", {}).get("sha256") == right.get("stderr", {}).get("sha256"))


def absent(receipt: dict[str, Any], path: str) -> bool:
    if receipt.get("exit_code") in (None, 0) or resource_reason(receipt): return False
    text = (receipt.get("stderr", {}).get("utf8") or "").lower()
    leaf = path.rsplit(".", 1)[-1].lower()
    return leaf in text and any(marker in text for marker in ABSENCE_MARKERS)


def axis_state(pair: tuple[dict[str, Any], dict[str, Any]], path: str) -> tuple[str, str]:
    vanilla, crystal = pair
    reason = resource_reason(vanilla) or resource_reason(crystal)
    if reason: return "UNKNOWN", reason if reason in UNKNOWN_REASONS else "RUNNER_FAILURE"
    if vanilla.get("exit_code") != 0: return "UNKNOWN", "BASELINE_DRIFT"
    if crystal.get("exit_code") == 0:
        return ("MATCH", "BILATERAL_PUBLIC_OBSERVABLES_MATCH") if same_public(vanilla, crystal) else ("DIFFERENCE", "PUBLIC_OBSERVABLE_DIFFERENCE")
    if absent(crystal, path): return "ABSENT", "QUALIFIED_PUBLIC_FORM_ABSENT"
    return "UNKNOWN", "UNEXPECTED_EXIT"


def expected_matches(control: dict[str, Any], receipt: dict[str, Any]) -> bool:
    expected = control.get("vanila_expected", {})
    return (
        resource_reason(receipt) is None
        and receipt.get("exit_code") == expected.get("exit_code")
        and receipt.get("stdout", {}).get("sha256") == expected.get("stdout", {}).get("sha256")
        and receipt.get("stderr", {}).get("sha256") == expected.get("stderr", {}).get("sha256")
        and (receipt.get("public_projection", {}).get("complete") is True)
    )


def classify(case: dict[str, Any], pairs: dict[str, tuple[dict[str, Any], dict[str, Any]]]) -> dict[str, str]:
    q_state, q_reason = axis_state(pairs["qualified"], case["path"])
    a_state, a_reason = axis_state(pairs["alternative"], case["path"])
    if "argument-boundary" in pairs:
        bv, bc = pairs["argument-boundary"]
        if resource_reason(bv) or resource_reason(bc) or bv.get("exit_code") != bc.get("exit_code"):
            return {"qualified_state": q_state, "alternative_state": a_state, "route_state": "UNKNOWN",
                    "reason_code": "UNEXPECTED_EXIT", "reason_phase": "BOUNDARY_ARGUMENT_CONTROL"}
    if "UNKNOWN" in (q_state, a_state):
        return {"qualified_state": q_state, "alternative_state": a_state, "route_state": "UNKNOWN",
                "reason_code": q_reason if q_state == "UNKNOWN" else a_reason, "reason_phase": "ROUTE_AXIS"}
    if q_state == "MATCH": state, reason = "MATCH", "QUALIFIED_MATCH"
    elif q_state == "DIFFERENCE": state, reason = "DIFFERENCE", "QUALIFIED_DIFFERENCE"
    elif q_state == "ABSENT" and a_state == "MATCH": state, reason = "PARTIAL", "QUALIFIED_ABSENT_ALTERNATIVE_MATCH"
    elif q_state == "ABSENT" and a_state == "DIFFERENCE": state, reason = "PARTIAL", "QUALIFIED_ABSENT_ALTERNATIVE_DIFFERENCE"
    elif q_state == "ABSENT" and a_state == "ABSENT": state, reason = "ABSENT", "QUALIFIED_AND_ALTERNATIVE_ABSENT"
    else: state, reason = "UNKNOWN", "RUNNER_FAILURE"
    return {"qualified_state": q_state, "alternative_state": a_state, "route_state": state,
            "reason_code": reason, "reason_phase": "ROUTE_PUBLIC_MORPHOLOGY",
            "qualified_reason": q_reason, "alternative_reason": a_reason}


def vector(rows: list[dict[str, Any]]) -> str:
    fields = sorted((r["path"], r["qualified_state"], r["alternative_state"], r["route_state"], r["reason_code"]) for r in rows)
    return sha256_bytes(json.dumps(fields, separators=(",", ":")).encode())


def execute(context: dict[str, Any], args: argparse.Namespace) -> dict[str, Any]:
    started = now()
    raw_root = args.raw_dir.resolve() if args.raw_dir else Path(tempfile.mkdtemp(prefix="p1420-reprobe-", dir="/tmp"))
    if not raw_root.is_relative_to(Path("/tmp")): raise GateError("raw-dir deve ficar sob /tmp")
    raw_root.mkdir(parents=True, exist_ok=True)
    cases = list(context["cases"])
    if args.focal:
        wanted = {"html.area", "math.abs"}
        cases = [case for case in cases if case["path"] in wanted]
    binaries = context["binary_paths"]
    controls = {control["id"]: control for control in context["documents"]["oracles"].get("controls", [])}
    receipts: dict[str, dict[str, dict[str, list[dict[str, Any]]]]] = {
        case["id"]: {probe["kind"]: {"vanila": [], "crystalline": []} for probe in case["probes"]} for case in cases
    }
    order_rows: dict[str, list[dict[str, Any]]] = {}
    control_executions: dict[str, dict[str, Any]] = {}
    serial = 0
    for order in ORDERS:
        ordered = list(reversed(cases)) if order == "reverse" else cases
        rows: list[dict[str, Any]] = []
        schedule_controls: dict[str, Any] = {}
        for control_id, control in controls.items():
            pair: dict[str, Any] = {}
            probe = {**control, "kind": "control"}
            for product, binary in binaries.items():
                serial += 1
                pair[product] = run_probe(probe, binary, product, order, raw_root, serial)
            baseline_ok = expected_matches(control, pair["vanila"])
            crystal_resource = resource_reason(pair["crystalline"])
            # O controle partial externo e testemunha, nao gate de sucesso qualificado.
            external = control_id.startswith("historical-partial-control:")
            crystal_ok = external or (crystal_resource is None and pair["crystalline"].get("exit_code") == 0)
            schedule_controls[control_id] = {
                "control_id": control_id, "phase": control.get("phase"), "receipts": pair,
                "vanila_expected_matches": baseline_ok, "result": "PASS" if baseline_ok and crystal_ok else "FAIL",
                "reason_code": None if baseline_ok and crystal_ok else (
                    "BASELINE_DRIFT" if not baseline_ok else crystal_resource or "PARSER_CONTROL_FAILED"
                ),
            }
        control_executions[order] = schedule_controls
        for case in ordered:
            pairs: dict[str, tuple[dict[str, Any], dict[str, Any]]] = {}
            for probe in case["probes"]:
                pair = {}
                for product, binary in binaries.items():
                    serial += 1
                    receipt = run_probe(probe, binary, product, order, raw_root, serial)
                    receipts[case["id"]][probe["kind"]][product].append(receipt)
                    pair[product] = receipt
                pairs[probe["kind"]] = (pair["vanila"], pair["crystalline"])
            classified = classify(case, pairs)
            failed = next((schedule_controls[ref] for ref in case.get("control_refs", [])
                           if schedule_controls.get(ref, {}).get("result") != "PASS"), None)
            if failed is not None:
                classified = {**classified, "route_state": "UNKNOWN",
                              "reason_code": failed["reason_code"], "reason_phase": "MANDATORY_CONTROL"}
            rows.append({"id": case["id"], "path": case["path"], **classified})
        order_rows[order] = rows
    vectors = {order: vector(rows) for order, rows in order_rows.items()}
    invariant = len(set(vectors.values())) == 1
    by_order = {order: {row["path"]: row for row in rows} for order, rows in order_rows.items()}
    routes = []
    for case in cases:
        states = [by_order[order][case["path"]] for order in ORDERS]
        row = dict(states[0]) if all(s == states[0] for s in states[1:]) else {
            "id": case["id"], "path": case["path"], "qualified_state": "UNKNOWN",
            "alternative_state": "UNKNOWN", "route_state": "UNKNOWN",
            "reason_code": "RUNNER_FAILURE", "reason_phase": "ORDER_INVARIANCE",
        }
        probe_receipts = receipts[case["id"]]
        row.update({
            "family": case["family"], "features": case.get("features", []), "target": case.get("target"),
            "probe_receipts": probe_receipts,
            "bilateral_argv": {kind: {product: [r["argv"] for r in runs] for product, runs in products.items()} for kind, products in probe_receipts.items()},
            "schedule_classifications": {order: by_order[order][case["path"]] for order in ORDERS},
            "vanila_identity": {"path": str(binaries["vanila"]), "sha256": context["binary_hashes"]["vanila"]},
            "crystalline_identity": {"path": str(binaries["crystalline"]), "sha256": context["binary_hashes"]["crystalline"]},
            "instant": min(r["instant"] for products in probe_receipts.values() for runs in products.values() for r in runs),
            "max_rss_kib": max((r.get("max_rss_kib") or 0) for products in probe_receipts.values() for runs in products.values() for r in runs),
            "swap_kib": max((r.get("swap_kib") or 0) for products in probe_receipts.values() for runs in products.values() for r in runs),
            "truncated": any(r.get("truncated") for products in probe_receipts.values() for runs in products.values() for r in runs),
            "owner": case.get("owner"), "consumer": case.get("consumer"),
            "inference_refuted_by": "nova evidencia bilateral atual com identidade e projecao completas",
        })
        routes.append(row)
    counts = Counter(row["route_state"] for row in routes)
    alt_counts = Counter(row["alternative_state"] for row in routes)
    verdict = "INCONCLUSIVE" if counts.get("UNKNOWN") or not invariant else "PASS"
    return {
        "schema": "p1420-route-results/v1", "attestation": "executado sem atestação de isolamento",
        "scope": "somente as 66 rotas congeladas; não prova paridade geral",
        "manifest_sha256": context["input_hashes"]["manifest"], "contract_sha256": context["input_hashes"]["contract"],
        "oracles_sha256": context["input_hashes"]["oracles"], "mutants_sha256": context["input_hashes"]["mutants"],
        "preseal_sha256": context["input_hashes"]["preseal"], "inventory_sha256": context["input_hashes"]["inventory"],
        "head": context["head"], "tree": context["tree"], "started_at": started, "finished_at": now(),
        "focal": bool(args.focal), "routes": routes,
        "execution_orders": {order: {"vector_sha256": vectors[order], "routes": order_rows[order]} for order in ORDERS},
        "control_executions": control_executions,
        "order_invariant": invariant, "state_counts": dict(sorted(counts.items())),
        "alternative_state_counts": dict(sorted(alt_counts.items())), "unknown_count": counts.get("UNKNOWN", 0),
        "resource_policy": {"parallelism": 1, "process_rlimit_as_kib": 524288, "per_process_timeout_seconds": 15,
                            "max_stdout_bytes": 65536, "max_stderr_bytes": 65536, "raw_outputs_directory": str(raw_root)},
        "binary_identities": {key: {"path": str(path), "sha256": context["binary_hashes"][key]} for key, path in binaries.items()},
        "verdict": verdict,
    }


def emit(value: dict[str, Any]) -> None:
    print(json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")))


def main() -> int:
    parser = argparse.ArgumentParser(description="Reprobe bilateral P1420")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--validate-only", action="store_true")
    mode.add_argument("--gate", action="store_true")
    mode.add_argument("--run", action="store_true")
    base = ROOT / "00_nucleo/diagnosticos"
    parser.add_argument("--manifest", type=Path, default=base / "p1420-manifest.json")
    parser.add_argument("--contract", type=Path, default=base / "p1420-contract.json")
    parser.add_argument("--oracles", type=Path, default=base / "p1420-oracles.json")
    parser.add_argument("--mutants", type=Path, default=base / "p1420-mutants.json")
    parser.add_argument("--preseal", type=Path, default=base / "p1420-preseal.json")
    parser.add_argument("--inventory", type=Path, default=Path("/tmp/p1420-current-inventory.json"))
    parser.add_argument("--vanila-bin", type=Path, default=ROOT / "lab/typst-original/target/release/typst")
    parser.add_argument("--crystalline-bin", type=Path, default=ROOT / "target/release/typst")
    parser.add_argument("--raw-dir", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--focal", action="store_true")
    args = parser.parse_args()
    try:
        context = validate_inputs(args)
        gate = mutation_gate(context)
        if gate["result"] != "PASS": raise GateError("gate discriminatorio: " + ",".join(gate["errors"]))
        if args.validate_only:
            emit({"schema": "p1420-runner-validation/v1", "result": "PASS", "route_count": 66,
                  "probe_count_per_order": 154, "preseal_sha256": context["input_hashes"]["preseal"]})
            return 0
        if args.gate:
            emit({"schema": "p1420-runner-gate/v1", **gate})
            return 0
        if args.output is None: raise GateError("--run exige --output")
        if not args.focal and args.output.resolve() != (ROOT / "00_nucleo/diagnosticos/p1420-results.json").resolve():
            raise GateError("campanha integral deve escrever p1420-results.json")
        result = execute(context, args)
        args.output.parent.mkdir(parents=True, exist_ok=True)
        temp = args.output.with_name(args.output.name + ".tmp")
        temp.write_text(json.dumps(result, ensure_ascii=False, sort_keys=True, indent=2) + "\n", encoding="utf-8")
        temp.replace(args.output)
        emit({"schema": "p1420-runner-completion/v1", "result": "PASS", "output": str(args.output),
              "output_sha256": sha256_path(args.output), "state_counts": result["state_counts"],
              "alternative_state_counts": result["alternative_state_counts"], "verdict": result["verdict"]})
        return 0
    except (GateError, OSError, subprocess.SubprocessError) as exc:
        emit({"schema": "p1420-runner-error/v1", "result": "FAIL", "reason_code": "RUNNER_FAILURE", "error": str(exc)})
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
