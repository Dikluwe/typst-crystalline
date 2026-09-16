#!/usr/bin/env python3
"""Retoma a campanha integral P1420 após o reparo HTML do P1421.

O payload público e o executor limitado continuam sendo os artefatos históricos
protegidos. Este sucessor valida a nova identidade, aplica o focal obrigatório e
exige uma repetição integral independente antes de emitir o resultado P1422.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from types import SimpleNamespace
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAGNOSTICS = ROOT / "00_nucleo/diagnosticos"
CAUSAL_HEAD = "fc803d05f0e7eec7f6bc6a8e363e3cfe1450077d"
CAUSAL_TREE = "11a404824b77d608b1eda29ad44fb281d89aef03"
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


def load_historical_runner() -> Any:
    path = ROOT / "lab/parity/p1420_unknown_route_reprobe.py"
    spec = importlib.util.spec_from_file_location("p1420_unknown_route_reprobe", path)
    if spec is None or spec.loader is None:
        raise GateError("não foi possível carregar o runner histórico P1420")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


P1420 = load_historical_runner()


def axis_state_p1422(
    pair: tuple[dict[str, Any], dict[str, Any]], path: str
) -> tuple[str, str]:
    """Classifica ausência conclusiva antes do artefato esperado só em sucesso.

    Uma forma qualificada inexistente termina com exit não zero e, por definição,
    não produz o HTML de saída. Nesse caso OUTPUT_MISSING é consequência do
    diagnóstico público de ausência, não uma falha instrumental independente.
    """
    vanilla, crystal = pair
    vanilla_reason = P1420.resource_reason(vanilla)
    if vanilla_reason:
        return "UNKNOWN", (
            vanilla_reason
            if vanilla_reason in P1420.UNKNOWN_REASONS
            else "RUNNER_FAILURE"
        )
    if vanilla.get("exit_code") != 0:
        return "UNKNOWN", "BASELINE_DRIFT"
    if crystal.get("exit_code") == 0:
        crystal_reason = P1420.resource_reason(crystal)
        if crystal_reason:
            return "UNKNOWN", (
                crystal_reason
                if crystal_reason in P1420.UNKNOWN_REASONS
                else "RUNNER_FAILURE"
            )
        if P1420.same_public(vanilla, crystal):
            return "MATCH", "BILATERAL_PUBLIC_OBSERVABLES_MATCH"
        return "DIFFERENCE", "PUBLIC_OBSERVABLE_DIFFERENCE"
    stderr = (crystal.get("stderr", {}).get("utf8") or "").lower()
    leaf = path.rsplit(".", 1)[-1].lower()
    conclusively_absent = (
        crystal.get("exit_code") not in (None, 0)
        and leaf in stderr
        and any(marker in stderr for marker in P1420.ABSENCE_MARKERS)
    )
    if conclusively_absent:
        return "ABSENT", "QUALIFIED_PUBLIC_FORM_ABSENT"
    crystal_reason = P1420.resource_reason(crystal)
    if crystal_reason and crystal_reason != "OUTPUT_MISSING":
        return "UNKNOWN", (
            crystal_reason
            if crystal_reason in P1420.UNKNOWN_REASONS
            else "RUNNER_FAILURE"
        )
    return "UNKNOWN", "UNEXPECTED_EXIT"


# `classify` resolve `axis_state` no namespace do módulo histórico.
P1420.axis_state = axis_state_p1422


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
        "step": ROOT / "00_nucleo/materialization/typst-passo-1422.md",
        "manifest": args.manifest.resolve(),
        "contract": args.contract.resolve(),
        "oracles": args.oracles.resolve(),
        "mutants": args.mutants.resolve(),
        "preseal": args.preseal.resolve(),
        "inventory": args.inventory.resolve(),
        "p1420_oracles": DIAGNOSTICS / "p1420-oracles.json",
    }
    documents = {key: load_json(path) for key, path in paths.items() if key != "step"}
    preseal = documents["preseal"]
    errors: list[str] = []
    if preseal.get("status") != "SEALED" or preseal.get("verdict") != "SEALED":
        errors.append("preseal_state")

    for key, reference in preseal.get("sealed_inputs", {}).items():
        raw_path = str(reference.get("path", ""))
        path = Path(raw_path) if raw_path.startswith("/") else ROOT / raw_path
        if not path.is_file() or sha256_path(path) != reference.get("sha256"):
            errors.append(f"sealed_hash:{key}")

    manifest = documents["manifest"]
    contract = documents["contract"]
    p1420_oracles = documents["p1420_oracles"]
    if manifest.get("step") != "P1422" or contract.get("step") != 1422:
        errors.append("step_identity")
    if manifest.get("immutable_public_oracle", {}).get("sha256") != sha256_path(paths["p1420_oracles"]):
        errors.append("authoritative_payload")

    canonical_fields = {
        "cases": "oracles_cases_canonical",
        "controls": "oracles_controls_canonical",
        "denominator": "oracles_denominator_canonical",
        "state_composition": "oracles_state_composition_canonical",
        "projection_policy": "oracles_projection_policy_canonical",
        "transport_policy": "oracles_transport_policy_canonical",
        "unknown_policy": "oracles_unknown_policy_canonical",
    }
    expected_canonical = contract.get("authoritative_payload_hashes", {})
    for section, field in canonical_fields.items():
        if canonical_hash(p1420_oracles.get(section)) != expected_canonical.get(field):
            errors.append(f"canonical_payload:{section}")

    sealed_cases = p1420_oracles.get("cases", [])
    routes = [case.get("path") for case in sealed_cases] if isinstance(sealed_cases, list) else []
    if routes != P1420.ROUTES or len(set(routes)) != 66:
        errors.append("literal_denominator")
    probe_count = sum(len(case.get("probes", [])) for case in sealed_cases)
    if probe_count != 154:
        errors.append("probe_cardinality")
    if len(p1420_oracles.get("controls", [])) != 12:
        errors.append("control_cardinality")

    inventory = documents["inventory"]
    rows = [
        row for row in inventory.get("crosswalk", [])
        if row.get("classification") == "VANILLA_ONLY"
    ]
    paths_now = {row.get("path") for row in rows}
    format_paths = {
        path for path in paths_now
        if path == "format" or str(path).startswith("format.")
    }
    if (len(rows), len(paths_now), len(format_paths), len(paths_now - format_paths)) != (504, 207, 124, 83):
        errors.append("inventory_counts")
    if not set(P1420.ROUTES) <= paths_now:
        errors.append("inventory_route_drift")
    expected_determinism = [
        "4109ae0d0f9a9403b1acc759e92438a3aeaa2edd6ae4292bb1b3ed16e265c28e"
    ] * 4
    if inventory.get("determinism_hashes") != expected_determinism:
        errors.append("inventory_determinism")

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

    control_ids = {control.get("id") for control in p1420_oracles.get("controls", [])}
    missing_control_refs = {
        reference
        for case in sealed_cases
        for reference in case.get("control_refs", [])
        if reference not in control_ids
    }
    if missing_control_refs != set(CONTROL_REFERENCE_ALIASES):
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
    for control in p1420_oracles.get("controls", []):
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
        "documents": {
            **documents,
            "oracles": p1420_oracles,
            "mutants": documents["mutants"],
        },
        "cases": cases,
        "binary_paths": binary_paths,
        "binary_hashes": binary_hashes,
        "head": git_value("rev-parse", "HEAD"),
        "tree": git_value("rev-parse", "HEAD^{tree}"),
        "input_hashes": {key: sha256_path(path) for key, path in paths.items()},
        "control_reference_aliases": CONTROL_REFERENCE_ALIASES,
    }


def mutation_gate(context: dict[str, Any]) -> dict[str, Any]:
    gate = P1420.mutation_gate(context)
    mutants = context["documents"]["mutants"]
    if (
        gate.get("valid_negative") != 27
        or gate.get("positive") != 8
        or gate.get("opaque") != 9
    ):
        gate["result"] = "FAIL"
        gate.setdefault("errors", []).append("p1422_mutant_cardinality")
    return gate


def run_once(context: dict[str, Any], focal: bool, raw_prefix: str) -> dict[str, Any]:
    raw_dir = Path(tempfile.mkdtemp(prefix=raw_prefix, dir="/tmp"))
    execution_args = SimpleNamespace(focal=focal, raw_dir=raw_dir)
    result = P1420.execute(context, execution_args)
    result["schema"] = "p1422-route-results/v1"
    result["step"] = 1422
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
    vectors = {
        value.get("vector_sha256")
        for value in result.get("execution_orders", {}).values()
    }
    return next(iter(vectors)) if len(vectors) == 1 else None


def write_json(path: Path, value: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp")
    temporary.write_text(
        json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
        encoding="utf-8",
    )
    temporary.replace(path)


def execute_campaign(context: dict[str, Any], args: argparse.Namespace) -> dict[str, Any]:
    if args.focal:
        result = run_once(context, True, "p1422-focal-")
        focal_pass = (
            len(result.get("routes", [])) == 2
            and result.get("order_invariant") is True
            and controls_pass(result)
            and result.get("unknown_count") == 0
        )
        result["focal_gate"] = "PASS" if focal_pass else "FAIL"
        result["verdict"] = "PASS" if focal_pass else "INCONCLUSIVE"
        return result

    focal_path = Path("/tmp/p1422-focal.json")
    if not focal_path.is_file():
        raise GateError("campanha integral exige /tmp/p1422-focal.json")
    focal = load_json(focal_path)
    if focal.get("focal_gate") != "PASS" or focal.get("runner_sha256") != sha256_path(Path(__file__)):
        raise GateError("focal ausente, inválido ou executado por outro runner")

    primary = run_once(context, False, "p1422-full-primary-")
    repeat = run_once(context, False, "p1422-full-repeat-")
    repeat_path = Path("/tmp/p1422-full-repeat.json")
    write_json(repeat_path, repeat)
    primary_vector = result_vector(primary)
    repeat_vector = result_vector(repeat)
    stable = (
        len(primary.get("routes", [])) == 66
        and len(repeat.get("routes", [])) == 66
        and primary.get("order_invariant") is True
        and repeat.get("order_invariant") is True
        and primary_vector is not None
        and primary_vector == repeat_vector
        and controls_pass(primary)
        and controls_pass(repeat)
    )
    primary["independent_full_repeat"] = {
        "path": str(repeat_path),
        "sha256": sha256_path(repeat_path),
        "route_count": len(repeat.get("routes", [])),
        "state_counts": repeat.get("state_counts"),
        "alternative_state_counts": repeat.get("alternative_state_counts"),
        "unknown_count": repeat.get("unknown_count"),
        "vector_sha256": repeat_vector,
        "order_invariant": repeat.get("order_invariant"),
        "controls_pass": controls_pass(repeat),
    }
    primary["full_repeat_stable"] = stable
    primary["focal_receipt"] = {
        "path": str(focal_path),
        "sha256": sha256_path(focal_path),
        "vector_sha256": result_vector(focal),
    }
    primary["verdict"] = (
        "PASS"
        if stable and primary.get("unknown_count") == 0 and repeat.get("unknown_count") == 0
        else "INCONCLUSIVE"
    )
    return primary


def emit(value: dict[str, Any]) -> None:
    print(json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")))


def main() -> int:
    parser = argparse.ArgumentParser(description="Retomada integral P1422 das 66 rotas P1420")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--validate-only", action="store_true")
    mode.add_argument("--gate", action="store_true")
    mode.add_argument("--run", action="store_true")
    parser.add_argument("--manifest", type=Path, default=DIAGNOSTICS / "p1422-manifest.json")
    parser.add_argument("--contract", type=Path, default=DIAGNOSTICS / "p1422-contract.json")
    parser.add_argument("--oracles", type=Path, default=DIAGNOSTICS / "p1422-oracles.json")
    parser.add_argument("--mutants", type=Path, default=DIAGNOSTICS / "p1422-mutants.json")
    parser.add_argument("--preseal", type=Path, default=DIAGNOSTICS / "p1422-preseal.json")
    parser.add_argument("--inventory", type=Path, default=Path("/tmp/p1420-current-inventory.json"))
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
                "schema": "p1422-runner-validation/v1",
                "result": "PASS",
                "route_count": 66,
                "probe_count_per_order": 154,
                "control_count": 12,
                "preseal_sha256": context["input_hashes"]["preseal"],
            })
            return 0
        if args.gate:
            emit({"schema": "p1422-runner-gate/v1", **gate})
            return 0
        if args.output is None:
            raise GateError("--run exige --output")
        expected = DIAGNOSTICS / "p1422-results.json"
        if not args.focal and args.output.resolve() != expected.resolve():
            raise GateError("campanha integral deve escrever p1422-results.json")
        result = execute_campaign(context, args)
        write_json(args.output.resolve(), result)
        emit({
            "schema": "p1422-runner-completion/v1",
            "result": "PASS",
            "output": str(args.output.resolve()),
            "output_sha256": sha256_path(args.output.resolve()),
            "route_count": len(result.get("routes", [])),
            "state_counts": result.get("state_counts"),
            "alternative_state_counts": result.get("alternative_state_counts"),
            "full_repeat_stable": result.get("full_repeat_stable"),
            "verdict": result.get("verdict"),
        })
        return 0 if result.get("verdict") == "PASS" else 3
    except (GateError, OSError, subprocess.SubprocessError) as exc:
        emit({
            "schema": "p1422-runner-error/v1",
            "result": "FAIL",
            "reason_code": "RUNNER_FAILURE",
            "error": str(exc),
        })
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
