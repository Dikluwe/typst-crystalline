#!/usr/bin/env python3
"""Frozen P1288 oracles; expectations never derive from the tested binary."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
ROOT = Path(__file__).resolve().parents[3]
DEFAULT_BASELINE = HERE / "p1288-oracle-baseline.json"
FIXTURES = HERE / "fixtures" / "p1288"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_baseline(path: Path = DEFAULT_BASELINE) -> dict[str, Any]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if data.get("schema") != "p1288-oracle-baseline/v1":
        raise ValueError("unsupported P1288 oracle baseline schema")
    return data


def verify_hashes(data: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    protected = dict(data["protected_inputs"])
    protected[data["contract_manifest"]["path"]] = data["contract_manifest"]["sha256"]
    protected[data["contract_receipt"]["path"]] = data["contract_receipt"]["sha256"]
    for relative, expected in sorted(protected.items()):
        path = ROOT / relative
        actual = sha256(path) if path.is_file() else "MISSING"
        if actual != expected:
            errors.append(f"hash drift: {relative}: expected {expected}, got {actual}")
    for name, expected in sorted(data["fixtures"].items()):
        path = FIXTURES / name
        actual = sha256(path) if path.is_file() else "MISSING"
        if actual != expected:
            errors.append(f"fixture drift: {name}: expected {expected}, got {actual}")
    return errors


def clean_env() -> dict[str, str]:
    env = dict(os.environ)
    env.pop("TYPST_FEATURES", None)
    env["SOURCE_DATE_EPOCH"] = "0"
    return env


def run_command(argv: list[str], *, cwd: Path = ROOT) -> dict[str, Any]:
    try:
        proc = subprocess.run(
            argv,
            cwd=cwd,
            env=clean_env(),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
    except (FileNotFoundError, PermissionError, OSError) as exc:
        return {"opaque": True, "reason": f"execution unavailable: {exc.__class__.__name__}"}
    return {
        "opaque": False,
        "exit": proc.returncode,
        "stdout": proc.stdout.decode("utf-8", "replace"),
        "stderr": proc.stderr.decode("utf-8", "replace"),
    }


def judge_process(expected: dict[str, Any], observed: dict[str, Any]) -> tuple[str, str]:
    if observed.get("opaque"):
        return "Unknown", str(observed.get("reason", "opaque execution"))
    mismatches: list[str] = []
    if observed["exit"] != expected["exit"]:
        mismatches.append(f"exit expected {expected['exit']} got {observed['exit']}")
    if "stdout" in expected and observed["stdout"] != expected["stdout"]:
        mismatches.append("stdout differs")
    if "stderr_contains" in expected and expected["stderr_contains"] not in observed["stderr"]:
        mismatches.append(f"stderr lacks {expected['stderr_contains']!r}")
    return ("Violated", "; ".join(mismatches)) if mismatches else ("Preserved", "matched frozen expectation")


def feature_args(features: list[str]) -> list[str]:
    args: list[str] = []
    for feature in features:
        args.extend(["--features", feature])
    return args


def run_eval(binary: str, case: dict[str, Any]) -> dict[str, Any]:
    argv = [binary, "eval", case["expr"], "--format", "json", *feature_args(case["features"])]
    observed = run_command(argv)
    status, witness = judge_process(case, observed)
    result = {"id": case["id"], "category": case["kind"], "status": status, "witness": witness}
    if not observed.get("opaque") and observed["exit"] == 0:
        result["payload_sha256"] = hashlib.sha256(observed["stdout"].encode()).hexdigest()
    if case.get("equivalence_group"):
        result["equivalence_group"] = case["equivalence_group"]
    return result


def token_count(text: str, token: str) -> int:
    if token in {"TR", "THead", "TBody"}:
        return sum(line.strip() == token for line in text.splitlines())
    return text.count(token)


def inspect_pdf(pdf: Path) -> dict[str, Any]:
    info = run_command(["pdfinfo", str(pdf)])
    structure = run_command(["pdfinfo", "-struct", str(pdf)])
    catalog = run_command(["mutool", "show", str(pdf), "trailer.Root"])
    parent = run_command(["mutool", "show", str(pdf), "trailer.Root.StructTreeRoot.ParentTree"])
    pages = run_command(["mutool", "show", str(pdf), "pages"])
    for item in (info, structure, catalog, parent, pages):
        if item.get("opaque"):
            return item
    return {
        "opaque": False,
        "info": info["stdout"],
        "structure": structure["stdout"],
        "catalog": catalog["stdout"],
        "parent": parent["stdout"],
        "pages": pages["stdout"],
    }


def compile_pdf(binary: str, fixture: str, output: Path, *, tags: bool = True) -> dict[str, Any]:
    argv = [binary, "compile", str(FIXTURES / fixture), str(output), "--features", "a11y-extras"]
    if not tags:
        argv.append("--no-pdf-tags")
    observed = run_command(argv)
    if observed.get("opaque"):
        return observed
    if observed["exit"] != 0:
        return {"opaque": False, "compile_failed": True, "exit": observed["exit"], "stderr": observed["stderr"]}
    if not output.is_file():
        return {"opaque": True, "reason": "successful compile produced no output"}
    return inspect_pdf(output)


def run_compile_case(binary: str, case: dict[str, Any], directory: Path) -> dict[str, Any]:
    pdf = directory / f"{case['id']}.pdf"
    inspected = compile_pdf(binary, case["fixture"], pdf)
    if inspected.get("opaque"):
        return {"id": case["id"], "category": "positive", "status": "Unknown", "witness": inspected["reason"]}
    if inspected.get("compile_failed"):
        return {"id": case["id"], "category": "positive", "status": "Violated", "witness": f"compile exit {inspected['exit']}"}
    failures: list[str] = []
    structure = inspected["structure"]
    if "Tagged:          yes" not in inspected["info"]:
        failures.append("PDF is not tagged by default")
    if "/StructTreeRoot" not in inspected["catalog"] or "/MarkInfo" not in inspected["catalog"]:
        failures.append("catalog lacks StructTreeRoot or MarkInfo")
    for token in case.get("struct_contains", []):
        if token not in structure:
            failures.append(f"structure lacks {token!r}")
    for token in case.get("struct_absent", []):
        if token in structure:
            failures.append(f"structure unexpectedly contains {token!r}")
    for token, expected in case.get("struct_counts", {}).items():
        actual = token_count(structure, token)
        if actual != expected:
            failures.append(f"count {token!r}: expected {expected}, got {actual}")
    for token in case.get("info_contains", []):
        if token not in inspected["info"]:
            failures.append(f"pdfinfo lacks {token!r}")
    if case["id"] == "multipage":
        page_dicts = run_command(["mutool", "show", str(pdf), "pages.1", "pages.2", "pages.3"])
        if page_dicts.get("opaque"):
            return {"id": case["id"], "category": "positive", "status": "Unknown", "witness": page_dicts["reason"]}
        for value in ("/StructParents 0", "/StructParents 1", "/StructParents 2"):
            if value not in page_dicts["stdout"]:
                failures.append(f"pages lack {value}")
        normalized = " ".join(inspected["parent"].split())
        if "/Nums [ 0" not in normalized or " 1 " not in normalized or " 2 " not in normalized:
            failures.append("ParentTree does not expose all three page keys")
    status = "Violated" if failures else "Preserved"
    return {"id": case["id"], "category": "positive", "status": status, "witness": "; ".join(failures) or "matched frozen PDF structure"}


def run_tags_disabled(binary: str, fixture: str, directory: Path) -> dict[str, Any]:
    inspected = compile_pdf(binary, fixture, directory / f"disabled-{fixture}.pdf", tags=False)
    case_id = f"tags-disabled-{fixture}"
    if inspected.get("opaque"):
        return {"id": case_id, "category": "negative", "status": "Unknown", "witness": inspected["reason"]}
    if inspected.get("compile_failed"):
        return {"id": case_id, "category": "negative", "status": "Violated", "witness": f"compile exit {inspected['exit']}"}
    failures: list[str] = []
    if "Tagged:          no" not in inspected["info"]:
        failures.append("pdfinfo does not report Tagged: no")
    if inspected["structure"].strip():
        failures.append("pdfinfo structure is not empty")
    if "/StructTreeRoot" in inspected["catalog"] or "/MarkInfo" in inspected["catalog"]:
        failures.append("catalog retains tagging keys")
    return {"id": case_id, "category": "negative", "status": "Violated" if failures else "Preserved", "witness": "; ".join(failures) or "tagging structure absent"}


def command_bytes(argv: list[str]) -> dict[str, Any]:
    observed = run_command(argv)
    if observed.get("opaque"):
        return observed
    if observed["exit"] != 0:
        return {"opaque": False, "failed": True, "exit": observed["exit"]}
    return {"opaque": False, "bytes": observed["stdout"].encode("utf-8")}


def render_hashes(pdf: Path, prefix: Path) -> dict[str, Any]:
    observed = run_command(["pdftoppm", "-png", "-r", "144", str(pdf), str(prefix)])
    if observed.get("opaque") or observed.get("exit") != 0:
        return {"opaque": True, "reason": "pdftoppm unavailable or failed"}
    return {"opaque": False, "hashes": [sha256(path) for path in sorted(prefix.parent.glob(prefix.name + "-*.png"))]}


def run_visual_pair(binary: str, fixture: str, directory: Path) -> dict[str, Any]:
    tagged = directory / f"pair-tagged-{fixture}.pdf"
    untagged = directory / f"pair-untagged-{fixture}.pdf"
    first = compile_pdf(binary, fixture, tagged, tags=True)
    second = compile_pdf(binary, fixture, untagged, tags=False)
    case_id = f"visual-pair-{fixture}"
    for inspected in (first, second):
        if inspected.get("opaque"):
            return {"id": case_id, "category": "positive", "status": "Unknown", "witness": inspected["reason"]}
        if inspected.get("compile_failed"):
            return {"id": case_id, "category": "positive", "status": "Violated", "witness": "compile failed"}
    failures: list[str] = []
    for args, label in ((["pdftotext", "-layout"], "text"), (["pdftotext", "-bbox-layout"], "bbox")):
        left = command_bytes([*args, str(tagged), "-"])
        right = command_bytes([*args, str(untagged), "-"])
        if left.get("opaque") or right.get("opaque"):
            return {"id": case_id, "category": "positive", "status": "Unknown", "witness": f"{label} tool unavailable"}
        if left.get("failed") or right.get("failed") or left["bytes"] != right["bytes"]:
            failures.append(f"{label} differs")
    boxes = re.compile(r"^(?:Pages|Page size|MediaBox|CropBox|BleedBox|TrimBox|ArtBox):.*$", re.M)
    if boxes.findall(first["info"]) != boxes.findall(second["info"]):
        failures.append("page boxes differ")
    left_raster = render_hashes(tagged, directory / f"raster-a-{fixture}")
    right_raster = render_hashes(untagged, directory / f"raster-b-{fixture}")
    if left_raster.get("opaque") or right_raster.get("opaque"):
        return {"id": case_id, "category": "positive", "status": "Unknown", "witness": "raster tool unavailable"}
    if left_raster["hashes"] != right_raster["hashes"]:
        failures.append("raster differs")
    return {"id": case_id, "category": "positive", "status": "Violated" if failures else "Preserved", "witness": "; ".join(failures) or "text, bbox, boxes and raster preserved"}


def compile_non_pdf(binary: str, fixture: str, output: Path, features: list[str]) -> dict[str, Any]:
    observed = run_command([binary, "compile", str(FIXTURES / fixture), str(output), *feature_args(features)])
    if observed.get("opaque"):
        return observed
    if observed["exit"] != 0 or not output.is_file():
        return {"opaque": False, "failed": True, "exit": observed["exit"]}
    return {"opaque": False, "sha256": sha256(output)}


def run_non_pdf_pair(binary: str, spec: dict[str, Any], fmt: str, directory: Path) -> dict[str, Any]:
    left = compile_non_pdf(binary, spec["plain"], directory / f"plain.{fmt}", ["html"] if fmt == "html" else [])
    features = ["html,a11y-extras"] if fmt == "html" else ["a11y-extras"]
    right = compile_non_pdf(binary, spec["a11y"], directory / f"a11y.{fmt}", features)
    case_id = f"non-pdf-pair-{fmt}"
    if left.get("opaque") or right.get("opaque"):
        return {"id": case_id, "category": "positive", "status": "Unknown", "witness": "target execution opaque"}
    if left.get("failed") or right.get("failed"):
        return {"id": case_id, "category": "positive", "status": "Violated", "witness": "target compile failed"}
    equal = left["sha256"] == right["sha256"]
    return {"id": case_id, "category": "positive", "status": "Preserved" if equal else "Violated", "witness": "fixture bytes equal" if equal else "fixture bytes differ"}


def canonical_results(results: list[dict[str, Any]]) -> str:
    return json.dumps(sorted(results, key=lambda item: item["id"]), sort_keys=True, separators=(",", ":"))


def run_suite(binary: str, data: dict[str, Any], order: str, directory: Path) -> list[dict[str, Any]]:
    tasks: list[tuple[str, Any]] = []
    tasks.extend(("eval", case) for case in data["eval_cases"])
    tasks.extend(("compile", case) for case in data["compile_cases"])
    tasks.extend(("disabled", fixture) for fixture in data["tag_disabled_cases"])
    tasks.extend(("visual", fixture) for fixture in data["visual_pairs"])
    tasks.extend(("nonpdf", fmt) for fmt in data["non_pdf_pair"]["formats"])
    tasks.extend(("opaque", case) for case in data["opaque_cases"])
    if order == "reverse":
        tasks.reverse()
    results: list[dict[str, Any]] = []
    for index, (kind, payload) in enumerate(tasks):
        work = directory / f"{index:03d}"
        work.mkdir()
        if kind == "eval":
            results.append(run_eval(binary, payload))
        elif kind == "compile":
            results.append(run_compile_case(binary, payload, work))
        elif kind == "disabled":
            results.append(run_tags_disabled(binary, payload, work))
        elif kind == "visual":
            results.append(run_visual_pair(binary, payload, work))
        elif kind == "nonpdf":
            results.append(run_non_pdf_pair(binary, data["non_pdf_pair"], payload, work))
        else:
            results.append({"id": payload["id"], "category": "opaque", "status": "Unknown", "witness": payload["reason"]})
    groups: dict[str, set[str]] = {}
    for result in results:
        if result.get("equivalence_group") and result.get("payload_sha256"):
            groups.setdefault(result["equivalence_group"], set()).add(result["payload_sha256"])
    if any(len(values) != 1 for values in groups.values()):
        results.append({"id": "equivalence-groups", "category": "control", "status": "Violated", "witness": "feature-order payload diverged"})
    return results


def self_test() -> int:
    expected = {"exit": 0, "stdout": "ok\n"}
    assert judge_process(expected, {"opaque": False, "exit": 0, "stdout": "ok\n", "stderr": ""})[0] == "Preserved"
    assert judge_process(expected, {"opaque": False, "exit": 1, "stdout": "", "stderr": ""})[0] == "Violated"
    assert judge_process(expected, {"opaque": True, "reason": "opaque"})[0] == "Unknown"
    sample = [{"id": "b", "status": "Unknown"}, {"id": "a", "status": "Preserved"}]
    assert canonical_results(sample) == canonical_results(list(reversed(sample)))
    data = load_baseline()
    assert not verify_hashes(data), "protected-input drift"
    assert any(case["id"] == "summary-explicit-none" and case["stderr_contains"] == "expected string, found none" for case in data["eval_cases"])
    print("SELF_TEST_OK")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, default=DEFAULT_BASELINE)
    parser.add_argument("--binary", default="/usr/local/bin/typst")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--order", choices=("forward", "reverse", "both"), default="both")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args(argv)
    if args.self_test:
        return self_test()
    data = load_baseline(args.baseline)
    drift = verify_hashes(data)
    if drift:
        payload = {"schema": "p1288-oracle-run/v1", "status": "Unknown", "input_drift": drift}
        text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
        args.output.write_text(text, encoding="utf-8") if args.output else sys.stdout.write(text)
        return 2
    binary_path = Path(args.binary)
    binary_hash = sha256(binary_path) if binary_path.is_file() else None
    identity = "ratified-vanilla" if binary_hash == data["vanilla"]["binary_sha256"] else "future-binary"
    orders = [args.order] if args.order != "both" else ["forward", "reverse"]
    runs: dict[str, list[dict[str, Any]]] = {}
    with tempfile.TemporaryDirectory(prefix="p1288-oracles-") as temp:
        for order in orders:
            directory = Path(temp) / order
            directory.mkdir()
            runs[order] = run_suite(args.binary, data, order, directory)
    order_agreement = True
    if len(orders) == 2:
        order_agreement = canonical_results(runs["forward"]) == canonical_results(runs["reverse"])
    all_results = runs[orders[0]]
    counts = {state: sum(item["status"] == state for item in all_results) for state in ("Preserved", "Violated", "Unknown")}
    payload = {
        "schema": "p1288-oracle-run/v1",
        "binary": {"path": args.binary, "sha256": binary_hash, "identity": identity},
        "baseline_sha256": sha256(args.baseline),
        "orders": orders,
        "order_agreement": order_agreement,
        "counts": counts,
        "results": sorted(all_results, key=lambda item: item["id"]),
        "note": "Oracle classifications are evidence only; this runner emits no refinement verdict.",
    }
    text = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    args.output.write_text(text, encoding="utf-8") if args.output else sys.stdout.write(text)
    return 0 if order_agreement and counts["Violated"] == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
