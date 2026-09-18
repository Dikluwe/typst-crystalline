#!/usr/bin/env python3
"""Recertifica as 66 rotas P1420 após a convergência P1423-P1451."""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import subprocess
import tempfile
from pathlib import Path
from types import SimpleNamespace
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAGNOSTICS = ROOT / "00_nucleo/diagnosticos"
CAUSAL_HEAD = "10798bc8907fd27c102c9ca522adfa54d0dded7b"
CAUSAL_TREE = "14625f6aa023410c42a1ee6a4b93d32d53a352c7"
PRODUCT_PATHS = (
    "00_nucleo/prompts",
    "01_core",
    "02_shell",
    "03_infra",
    "04_wiring",
    "Cargo.toml",
    "Cargo.lock",
)
CONTROL_REFERENCE_ALIASES = {
    "eval-parser-feature-common:a11y-extras": "eval-parser-feature-common:html+a11y",
}


class GateError(RuntimeError):
    pass


def load_p1422() -> Any:
    path = ROOT / "lab/parity/p1422_unknown_route_reprobe.py"
    spec = importlib.util.spec_from_file_location("p1422_unknown_route_reprobe", path)
    if spec is None or spec.loader is None:
        raise GateError("não foi possível carregar o runner histórico P1422")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


P1422 = load_p1422()
P1420 = P1422.P1420


def sha256_path(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def canonical_hash(value: Any) -> str:
    encoded = json.dumps(
        value, ensure_ascii=False, sort_keys=True, separators=(",", ":")
    ).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise GateError(f"JSON inválido em {path}: {exc}") from exc
    if not isinstance(value, dict):
        raise GateError(f"objeto JSON esperado em {path}")
    return value


def git_value(*args: str) -> str:
    result = subprocess.run(
        ["git", *args],
        cwd=ROOT,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=10,
        check=False,
    )
    if result.returncode:
        raise GateError(f"git {' '.join(args)} falhou")
    return result.stdout.decode("utf-8", "strict").strip()


def product_is_causally_frozen() -> bool:
    diff = subprocess.run(
        ["git", "diff", "--quiet", CAUSAL_HEAD, "--", *PRODUCT_PATHS],
        cwd=ROOT,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        timeout=10,
        check=False,
    )
    status = subprocess.run(
        ["git", "status", "--porcelain", "--untracked-files=all", "--", *PRODUCT_PATHS],
        cwd=ROOT,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        timeout=10,
        check=False,
    )
    return diff.returncode == 0 and status.returncode == 0 and not status.stdout


def validate_inputs(args: argparse.Namespace) -> dict[str, Any]:
    paths = {
        "step": ROOT / "00_nucleo/materialization/typst-passo-1452.md",
        "manifest": args.manifest.resolve(),
        "contract": args.contract.resolve(),
        "oracles": args.oracles.resolve(),
        "mutants": args.mutants.resolve(),
        "preseal": args.preseal.resolve(),
        "p1420_oracles": DIAGNOSTICS / "p1420-oracles.json",
    }
    documents = {key: load_json(path) for key, path in paths.items() if key != "step"}
    errors: list[str] = []
    preseal = documents["preseal"]
    if preseal.get("status") != "SEALED" or preseal.get("verdict") != "SEALED":
        errors.append("preseal_state")
    for key, reference in preseal.get("sealed_inputs", {}).items():
        raw_path = str(reference.get("path", ""))
        path = Path(raw_path) if raw_path.startswith("/") else ROOT / raw_path
        if not path.is_file() or sha256_path(path) != reference.get("sha256"):
            errors.append(f"sealed_hash:{key}")

    manifest = documents["manifest"]
    contract = documents["contract"]
    authoritative = documents["p1420_oracles"]
    if manifest.get("step") != "P1452" or contract.get("step") != 1452:
        errors.append("step_identity")
    if manifest.get("immutable_public_oracle", {}).get("sha256") != sha256_path(paths["p1420_oracles"]):
        errors.append("authoritative_payload")

    canonical_fields = {
        "cases": "cases_canonical",
        "controls": "controls_canonical",
        "denominator": "denominator_canonical",
        "state_composition": "state_composition_canonical",
        "projection_policy": "projection_policy_canonical",
        "transport_policy": "transport_policy_canonical",
        "unknown_policy": "unknown_policy_canonical",
    }
    expected = contract.get("authoritative_payload", {})
    for section, field in canonical_fields.items():
        if canonical_hash(authoritative.get(section)) != expected.get(field):
            errors.append(f"canonical_payload:{section}")

    sealed_cases = authoritative.get("cases", [])
    routes = [case.get("path") for case in sealed_cases] if isinstance(sealed_cases, list) else []
    if routes != P1420.ROUTES or len(set(routes)) != 66:
        errors.append("literal_denominator")
    probe_count = sum(len(case.get("probes", [])) for case in sealed_cases)
    if probe_count != 154:
        errors.append("probe_cardinality")
    if len(authoritative.get("controls", [])) != 12:
        errors.append("control_cardinality")

    binary_paths = {
        "vanila": args.vanila_bin.resolve(),
        "crystalline": args.crystalline_bin.resolve(),
    }
    identities = manifest.get("identities", {})
    expected_binary_hashes = {
        "vanila": identities.get("vanilla_product", {}).get("sha256"),
        "crystalline": identities.get("crystalline_product", {}).get("sha256"),
    }
    binary_hashes: dict[str, str | None] = {}
    for product, path in binary_paths.items():
        binary_hashes[product] = sha256_path(path) if path.is_file() else None
        if binary_hashes[product] != expected_binary_hashes[product]:
            errors.append(f"identity:{product}")
    if not product_is_causally_frozen():
        errors.append("product_causal_drift")

    control_ids = {control.get("id") for control in authoritative.get("controls", [])}
    missing_refs = {
        reference
        for case in sealed_cases
        for reference in case.get("control_refs", [])
        if reference not in control_ids
    }
    if missing_refs != set(CONTROL_REFERENCE_ALIASES):
        errors.append("unexpected_control_reference_drift")
    if not set(CONTROL_REFERENCE_ALIASES.values()) <= control_ids:
        errors.append("control_reference_alias_target")

    for case in sealed_cases:
        for probe in case.get("probes", []):
            argv = probe.get("argv_template", [])
            if "--jobs" in argv or "--target" in argv:
                errors.append(f"forbidden_argv:{probe.get('id')}")
        if case.get("family") == "html":
            tag = str(case.get("path", "")).removeprefix("html.")
            alternative = next(
                (probe for probe in case.get("probes", []) if probe.get("kind") == "alternative"),
                {},
            )
            if alternative.get("literal_tag") != tag:
                errors.append(f"route_local_alternative:{case.get('path')}")
    for control in authoritative.get("controls", []):
        argv = control.get("argv_template", [])
        if "--jobs" in argv or "--target" in argv:
            errors.append(f"forbidden_control_argv:{control.get('id')}")

    if errors:
        raise GateError(";".join(sorted(set(errors))))
    cases = copy.deepcopy(sealed_cases)
    for case in cases:
        case["control_refs"] = [
            CONTROL_REFERENCE_ALIASES.get(reference, reference)
            for reference in case.get("control_refs", [])
        ]
    return {
        "paths": paths,
        "documents": {**documents, "oracles": authoritative},
        "cases": cases,
        "binary_paths": binary_paths,
        "binary_hashes": binary_hashes,
        "head": git_value("rev-parse", "HEAD"),
        "tree": git_value("rev-parse", "HEAD^{tree}"),
        "input_hashes": {
            **{key: sha256_path(path) for key, path in paths.items()},
            # Compatibilidade de chamada com o executor P1420. P1452 não usa o
            # inventário VANILLA_ONLY histórico; o campo é removido do recibo
            # logo após a execução e substituído pelo payload autoritativo.
            "inventory": "NOT_APPLICABLE_P1452",
        },
        "control_reference_aliases": CONTROL_REFERENCE_ALIASES,
    }


def mutation_gate(context: dict[str, Any]) -> dict[str, Any]:
    gate = P1420.mutation_gate(context)
    if (
        gate.get("valid_negative") != 27
        or gate.get("positive") != 8
        or gate.get("opaque") != 9
    ):
        gate["result"] = "FAIL"
        gate.setdefault("errors", []).append("p1452_mutant_cardinality")
    return gate


def run_once(context: dict[str, Any], focal: bool, raw_prefix: str) -> dict[str, Any]:
    raw_dir = Path(tempfile.mkdtemp(prefix=raw_prefix, dir="/tmp"))
    result = P1420.execute(context, SimpleNamespace(focal=focal, raw_dir=raw_dir))
    result.pop("inventory_sha256", None)
    result["authoritative_payload_sha256"] = context["input_hashes"]["p1420_oracles"]
    result["schema"] = "p1452-route-results/v1"
    result["step"] = 1452
    result["product_causal_head"] = CAUSAL_HEAD
    result["product_causal_tree"] = CAUSAL_TREE
    result["runner_sha256"] = sha256_path(Path(__file__))
    result["raw_outputs_directory"] = str(raw_dir)
    result["control_reference_aliases"] = context["control_reference_aliases"]
    for row in result.get("routes", []):
        row["schedule_stable"] = all(
            row.get("schedule_classifications", {}).get(order)
            == row.get("schedule_classifications", {}).get("direct")
            for order in P1420.ORDERS
        )
    return result


def controls_pass(result: dict[str, Any]) -> bool:
    controls = result.get("control_executions", {})
    return all(
        len(schedule) == 12
        and all(control.get("result") == "PASS" for control in schedule.values())
        for schedule in controls.values()
    )


def result_vector(result: dict[str, Any]) -> str | None:
    vectors = {value.get("vector_sha256") for value in result.get("execution_orders", {}).values()}
    return next(iter(vectors)) if len(vectors) == 1 else None


def write_json(path: Path, value: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp")
    temporary.write_text(
        json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
        encoding="utf-8",
    )
    temporary.replace(path)


def all_routes_match(result: dict[str, Any], expected_count: int) -> bool:
    routes = result.get("routes", [])
    return len(routes) == expected_count and all(
        row.get("qualified_state") == "MATCH"
        and row.get("route_state") == "MATCH"
        and row.get("schedule_stable") is True
        for row in routes
    )


def execute_campaign(context: dict[str, Any], args: argparse.Namespace) -> dict[str, Any]:
    if args.focal:
        result = run_once(context, True, "p1452-focal-")
        passed = (
            all_routes_match(result, 2)
            and result.get("order_invariant") is True
            and controls_pass(result)
            and result.get("unknown_count") == 0
        )
        result["focal_gate"] = "PASS" if passed else "FAIL"
        result["verdict"] = "PASS" if passed else "INCONCLUSIVE"
        return result

    focal_path = Path("/tmp/p1452-focal.json")
    if not focal_path.is_file():
        raise GateError("campanha integral exige /tmp/p1452-focal.json")
    focal = load_json(focal_path)
    if focal.get("focal_gate") != "PASS" or focal.get("runner_sha256") != sha256_path(Path(__file__)):
        raise GateError("focal ausente, inválido ou executado por outro runner")

    primary = run_once(context, False, "p1452-full-primary-")
    repeat = run_once(context, False, "p1452-full-repeat-")
    repeat_path = Path("/tmp/p1452-full-repeat.json")
    write_json(repeat_path, repeat)
    primary_vector = result_vector(primary)
    repeat_vector = result_vector(repeat)
    stable = (
        primary.get("order_invariant") is True
        and repeat.get("order_invariant") is True
        and primary_vector is not None
        and primary_vector == repeat_vector
        and controls_pass(primary)
        and controls_pass(repeat)
    )
    converged = all_routes_match(primary, 66) and all_routes_match(repeat, 66)
    primary["independent_full_repeat"] = {
        "path": str(repeat_path),
        "sha256": sha256_path(repeat_path),
        "route_count": len(repeat.get("routes", [])),
        "state_counts": repeat.get("state_counts"),
        "qualified_state_counts": repeat.get("qualified_state_counts"),
        "unknown_count": repeat.get("unknown_count"),
        "vector_sha256": repeat_vector,
        "order_invariant": repeat.get("order_invariant"),
        "controls_pass": controls_pass(repeat),
        "all_routes_match": all_routes_match(repeat, 66),
    }
    primary["full_repeat_stable"] = stable
    primary["closure_gate"] = "PASS" if stable and converged else "FAIL"
    primary["focal_receipt"] = {
        "path": str(focal_path),
        "sha256": sha256_path(focal_path),
        "vector_sha256": result_vector(focal),
    }
    primary["verdict"] = (
        "PASS"
        if stable
        and converged
        and primary.get("unknown_count") == 0
        and repeat.get("unknown_count") == 0
        else "INCONCLUSIVE"
    )
    return primary


def emit(value: dict[str, Any]) -> None:
    print(json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")))


def main() -> int:
    parser = argparse.ArgumentParser(description="Recertificação P1452 das 66 rotas P1420")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--validate-only", action="store_true")
    mode.add_argument("--gate", action="store_true")
    mode.add_argument("--run", action="store_true")
    parser.add_argument("--manifest", type=Path, default=DIAGNOSTICS / "p1452-manifest.json")
    parser.add_argument("--contract", type=Path, default=DIAGNOSTICS / "p1452-contract.json")
    parser.add_argument("--oracles", type=Path, default=DIAGNOSTICS / "p1452-oracles.json")
    parser.add_argument("--mutants", type=Path, default=DIAGNOSTICS / "p1452-mutants.json")
    parser.add_argument("--preseal", type=Path, default=DIAGNOSTICS / "p1452-preseal.json")
    parser.add_argument("--vanila-bin", type=Path, default=ROOT / "lab/typst-original/target/release/typst")
    parser.add_argument("--crystalline-bin", type=Path, default=ROOT / "target/release/typst")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--focal", action="store_true")
    args = parser.parse_args()
    try:
        context = validate_inputs(args)
        gate = mutation_gate(context)
        if gate.get("result") != "PASS":
            raise GateError("gate discriminatório: " + ",".join(gate.get("errors", [])))
        if args.validate_only:
            emit({
                "schema": "p1452-runner-validation/v1",
                "result": "PASS",
                "route_count": 66,
                "probe_count_per_order": 154,
                "control_count": 12,
                "preseal_sha256": context["input_hashes"]["preseal"],
            })
            return 0
        if args.gate:
            emit({"schema": "p1452-runner-gate/v1", **gate})
            return 0
        if args.output is None:
            raise GateError("--run exige --output")
        expected = DIAGNOSTICS / "p1452-results.json"
        if not args.focal and args.output.resolve() != expected.resolve():
            raise GateError("campanha integral deve escrever p1452-results.json")
        result = execute_campaign(context, args)
        write_json(args.output.resolve(), result)
        emit({
            "schema": "p1452-runner-completion/v1",
            "result": "PASS" if result.get("verdict") == "PASS" else "INCONCLUSIVE",
            "output": str(args.output.resolve()),
            "output_sha256": sha256_path(args.output.resolve()),
            "route_count": len(result.get("routes", [])),
            "state_counts": result.get("state_counts"),
            "qualified_state_counts": result.get("qualified_state_counts"),
            "full_repeat_stable": result.get("full_repeat_stable"),
            "closure_gate": result.get("closure_gate"),
            "verdict": result.get("verdict"),
        })
        return 0 if result.get("verdict") == "PASS" else 3
    except (GateError, OSError, subprocess.SubprocessError) as exc:
        emit({
            "schema": "p1452-runner-error/v1",
            "result": "FAIL",
            "reason_code": "RUNNER_FAILURE",
            "error": str(exc),
        })
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
