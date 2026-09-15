#!/usr/bin/env python3
"""Reclassifica o recorte P1409 sem o argumento mecânico ``--jobs 1``."""

from __future__ import annotations

import argparse
import copy
import datetime as dt
import hashlib
import json
import os
import subprocess
import sys
import tempfile
from collections import Counter
from pathlib import Path
from typing import Any

from p1400_route_probe import classify_case, run_probe


ROOT = Path(__file__).resolve().parents[2]
ORDERS = ("direct", "direct-repeat", "reverse")
SELECTED_IDS = (
    "P1400-ELEM-007",
    "P1400-ELEM-008",
    "P1400-ELEM-009",
    "P1400-ELEM-010",
    "P1400-ELEM-011",
    "P1400-ELEM-013",
)
EXPECTED_PROBES_PER_ORDER = 14
EXPECTED_VANILA_SHA256 = "eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f"


class GateError(RuntimeError):
    pass


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat(timespec="microseconds")


def sha256_path(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def remove_jobs_pair(argv: list[str]) -> list[str]:
    """Remove exatamente um par adjacente ``--jobs 1`` sem tocar no resto."""
    positions = [index for index in range(len(argv) - 1)
                 if argv[index:index + 2] == ["--jobs", "1"]]
    if len(positions) != 1:
        raise GateError(f"esperado um par --jobs 1, encontrados {len(positions)}")
    index = positions[0]
    return argv[:index] + argv[index + 2:]


def normalized_case(case: dict[str, Any]) -> dict[str, Any]:
    normalized = copy.deepcopy(case)
    for probe in normalized["probes"]:
        argv = probe["argv_template"]
        if len(argv) >= 2 and argv[1] == "compile":
            probe["argv_template"] = remove_jobs_pair(argv)
        elif "--jobs" in argv:
            raise GateError("--jobs fora de um template compile")
    return normalized


def self_test() -> None:
    original = ["typst", "compile", "--jobs", "1", "--features", "html", "in.typ", "out.html"]
    expected = ["typst", "compile", "--features", "html", "in.typ", "out.html"]
    assert remove_jobs_pair(original) == expected
    assert original[2:4] == ["--jobs", "1"]
    eval_case = {"probes": [{"argv_template": ["typst", "eval", "repr(1)"]}]}
    assert normalized_case(eval_case) == eval_case
    for invalid in (
        ["typst", "compile"],
        ["typst", "compile", "--jobs", "2"],
        ["typst", "compile", "--jobs", "1", "--jobs", "1"],
        ["typst", "compile", "--jobs", "--features", "1"],
    ):
        try:
            remove_jobs_pair(invalid)
        except GateError:
            pass
        else:
            raise AssertionError(f"template inválido aceite: {invalid}")
    oom = {"exit_code": -6, "reason_code": None,
           "stderr": {"utf8": "memory allocation of 210496 bytes failed"}}
    crash = {"exit_code": -11, "reason_code": None, "stderr": {"utf8": ""}}
    absent = {"exit_code": 1, "reason_code": None,
              "stderr": {"utf8": "unknown field"},
              "public_projection": {
                  "kind": "pdftotext-layout", "reason_code": "OUTPUT_MISSING",
              }}
    classify_process_failure(oom)
    classify_process_failure(crash)
    classify_process_failure(absent)
    assert oom["reason_code"] == "OOM"
    assert crash["reason_code"] == "RUNNER_FAILURE"
    assert "reason_code" not in absent["public_projection"]


def load_inputs() -> tuple[list[dict[str, Any]], dict[str, Any], dict[str, str]]:
    oracle_path = ROOT / "00_nucleo/diagnosticos/p1400-oracles.json"
    result_path = ROOT / "00_nucleo/diagnosticos/p1400-results.json"
    oracles = json.loads(oracle_path.read_text(encoding="utf-8"))
    previous = json.loads(result_path.read_text(encoding="utf-8"))
    previous_by_id = {route["id"]: route for route in previous["routes"]}
    cases_by_id = {case["id"]: case for case in oracles["cases"]}
    if set(SELECTED_IDS) - cases_by_id.keys() or set(SELECTED_IDS) - previous_by_id.keys():
        raise GateError("seleção P1409 não está contida nos artefatos P1400")
    for route_id in SELECTED_IDS:
        previous_route = previous_by_id[route_id]
        if (previous_route["state"], previous_route["reason_code"]) != ("UNKNOWN", "OUTPUT_MISSING"):
            raise GateError(f"baseline P1400 inesperado para {route_id}")
    cases = [normalized_case(cases_by_id[route_id]) for route_id in SELECTED_IDS]
    if len(cases) != 6 or sum(len(case["probes"]) for case in cases) != EXPECTED_PROBES_PER_ORDER:
        raise GateError("cardinalidade do recorte P1409 divergente")
    hashes = {"oracles": sha256_path(oracle_path), "p1400_results": sha256_path(result_path)}
    return cases, previous, hashes


def projection_sha(receipt: dict[str, Any]) -> str | None:
    projection = receipt.get("public_projection", {})
    if projection.get("kind") == "pdftotext-layout":
        return projection.get("bytes", {}).get("sha256")
    if projection.get("kind") == "html-bytes":
        return projection.get("bytes", {}).get("sha256")
    return receipt.get("stdout", {}).get("sha256")


def compact_receipt(receipt: dict[str, Any]) -> dict[str, Any]:
    return {
        "argv": receipt["argv"],
        "exit_code": receipt["exit_code"],
        "reason_code": receipt.get("reason_code"),
        "stdout_sha256": receipt["stdout"]["sha256"],
        "stderr_sha256": receipt["stderr"]["sha256"],
        "projection_sha256": projection_sha(receipt),
        "duration_ms": receipt["duration_ms"],
        "max_rss_kib": receipt.get("max_rss_kib"),
        "swap_kib": receipt.get("swap_kib"),
        "timeout": receipt.get("timeout"),
        "truncated": receipt.get("truncated"),
        "raw_stdout": receipt["raw_stdout"],
        "raw_stderr": receipt["raw_stderr"],
    }


def classify_process_failure(receipt: dict[str, Any]) -> None:
    """Não permite que um encerramento por sinal pareça artefato ausente."""
    exit_code = receipt.get("exit_code")
    projection = receipt.get("public_projection", {})
    if (exit_code or 0) > 0 and projection.get("reason_code") == "OUTPUT_MISSING":
        projection.pop("reason_code")
    if receipt.get("reason_code") is not None or (exit_code or 0) >= 0:
        return
    stderr = receipt.get("stderr", {}).get("utf8") or ""
    receipt["reason_code"] = (
        "OOM" if "memory allocation of " in stderr and " failed" in stderr
        else "RUNNER_FAILURE"
    )


def execute(cases: list[dict[str, Any]], vanila: Path, crystalline: Path,
            raw_root: Path) -> dict[str, Any]:
    binaries = {"vanila": vanila, "crystalline": crystalline}
    serial = 0
    classifications: dict[str, list[tuple[str, str, str]]] = {
        case["id"]: [] for case in cases
    }
    receipts: dict[str, dict[str, Any]] = {case["id"]: {} for case in cases}
    started = now()
    for order in ORDERS:
        ordered = list(reversed(cases)) if order == "reverse" else cases
        for case in ordered:
            local: dict[str, tuple[dict[str, Any], dict[str, Any]]] = {}
            receipts[case["id"]][order] = {}
            for probe in case["probes"]:
                pair: dict[str, dict[str, Any]] = {}
                for product, binary in binaries.items():
                    serial += 1
                    receipt = run_probe(probe, binary, product, order, raw_root, serial)
                    classify_process_failure(receipt)
                    pair[product] = receipt
                local[probe["kind"]] = (pair["vanila"], pair["crystalline"])
                receipts[case["id"]][order][probe["kind"]] = {
                    product: compact_receipt(receipt) for product, receipt in pair.items()
                }
            classifications[case["id"]].append(classify_case(case, local))

    routes = []
    for case in cases:
        observed = classifications[case["id"]]
        stable = len(set(observed)) == 1
        state, reason, phase = observed[0] if stable else (
            "UNKNOWN", "RUNNER_FAILURE", "ORDER_INVARIANCE"
        )
        routes.append({
            "id": case["id"], "path": case["path"], "state": state,
            "reason_code": reason, "reason_phase": phase,
            "order_invariant": stable,
            "classifications": dict(zip(ORDERS, observed)),
            "receipts": receipts[case["id"]],
        })
    resource_receipts = [receipt
                         for route in routes
                         for order in route["receipts"].values()
                         for probe in order.values()
                         for receipt in probe.values()]
    counts = Counter(route["state"] for route in routes)
    resource_failure = any(
        receipt["reason_code"] is not None or receipt["timeout"]
        or receipt["truncated"] or (receipt["swap_kib"] or 0) != 0
        for receipt in resource_receipts
    )
    return {
        "schema": "p1409-reprobe-compiled-elements/v1",
        "started_at": started,
        "finished_at": now(),
        "orders": list(ORDERS),
        "routes": routes,
        "state_counts": dict(sorted(counts.items())),
        "order_invariant": all(route["order_invariant"] for route in routes),
        "process_count": len(resource_receipts),
        "resource_failure": resource_failure,
        "runner_result": "PASS" if all(route["order_invariant"] for route in routes) else "FAIL",
        "max_rss_kib": max((receipt["max_rss_kib"] or 0) for receipt in resource_receipts),
        "max_swap_kib": max((receipt["swap_kib"] or 0) for receipt in resource_receipts),
        "attestation": "executado sem atestação de isolamento",
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--run", action="store_true")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--raw-dir", type=Path)
    parser.add_argument("--vanila-bin", type=Path,
                        default=ROOT / "lab/typst-original/target/release/typst")
    parser.add_argument("--crystalline-bin", type=Path,
                        default=ROOT / "target/debug/typst")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        print('{"result":"PASS","test":"p1409-normalization"}')
        return 0
    if not args.run or args.output is None:
        parser.error("use --self-test ou --run --output PATH")
    cases, previous, input_hashes = load_inputs()
    vanila, crystalline = args.vanila_bin.resolve(), args.crystalline_bin.resolve()
    if sha256_path(vanila) != EXPECTED_VANILA_SHA256:
        raise GateError("identidade do vanila divergente")
    raw_root = (args.raw_dir.resolve() if args.raw_dir
                else Path(tempfile.mkdtemp(prefix="p1409-reprobe-", dir="/tmp")))
    if not raw_root.is_relative_to(Path("/tmp")):
        raise GateError("brutos devem ficar sob /tmp")
    old_threads = os.environ.get("RAYON_NUM_THREADS")
    os.environ["RAYON_NUM_THREADS"] = "1"
    try:
        result = execute(cases, vanila, crystalline, raw_root)
    finally:
        if old_threads is None:
            os.environ.pop("RAYON_NUM_THREADS", None)
        else:
            os.environ["RAYON_NUM_THREADS"] = old_threads
    result.update({
        "head": subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, check=True,
            stdout=subprocess.PIPE, text=True,
        ).stdout.strip(),
        "working_tree": "working tree não commitado",
        "input_hashes": input_hashes,
        "source_p1400_verdict": previous["verdict"],
        "binary_identities": {
            "vanila": {"path": str(vanila), "sha256": sha256_path(vanila)},
            "crystalline": {"path": str(crystalline), "sha256": sha256_path(crystalline)},
        },
        "resource_policy": {
            "parallelism": 1, "rayon_threads": 1, "rlimit_as_kib": 512 * 1024,
            "timeout_seconds": 15, "stream_limit_bytes": 64 * 1024,
            "raw_directory": str(raw_root),
        },
    })
    args.output.write_text(json.dumps(result, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
                           encoding="utf-8")
    print(json.dumps({
        "result": result["runner_result"],
        "state_counts": result["state_counts"], "process_count": result["process_count"],
        "max_rss_kib": result["max_rss_kib"], "max_swap_kib": result["max_swap_kib"],
        "output": str(args.output), "output_sha256": sha256_path(args.output),
    }, ensure_ascii=False, sort_keys=True))
    return 0 if result["order_invariant"] else 2


if __name__ == "__main__":
    sys.exit(main())
