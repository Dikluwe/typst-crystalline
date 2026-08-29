#!/usr/bin/env python3
"""Adjudicate the live P1269 run exclusively with the sealed P1268-v3 contract."""

from __future__ import annotations

import argparse
import csv
import hashlib
import importlib.util
import json
import math
import subprocess
import sys
from collections import defaultdict
from datetime import datetime
from pathlib import Path


ROOT = Path("/repos/Antigravity/typst-crystalline")
RAW = Path("/tmp/p1269/p1266")
FIXTURES = Path("/tmp/p1269/fixtures")
OUT = Path("/tmp/p1269/adjudication")
DIAG = ROOT / "00_nucleo/diagnosticos"
PAIRS = ("linear/oklab", "radial/oklab", "linear/linear-rgb", "radial/linear-rgb")
METRICS = ("color_max", "color_p95", "alpha_max", "alpha_p95")
EPS = 1e-6


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        raise RuntimeError(f"refusing empty receipt {path}")
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical(value: object) -> str:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))


def load_runner():
    path = ROOT / "lab/parity/matrix/p1266_generalize.py"
    spec = importlib.util.spec_from_file_location("p1266_generalize", path)
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def max_delta(left: list[float], right: list[float]) -> float:
    return max((abs(float(a) - float(b)) for a, b in zip(left, right)), default=0.0)


def dyadic_subdivisions(interiors: list[float], left: float, right: float) -> int:
    if not interiors:
        return 1
    needed = 1
    width = right - left
    for value in interiors:
        relative = (value - left) / width
        denominator = None
        for trial in (1, 2, 4, 8, 16, 32, 64):
            if abs(relative * trial - round(relative * trial)) <= 2e-6:
                denominator = trial
                break
        if denominator is None:
            return 65
        needed = max(needed, denominator)
    return needed


def sample_components(runner, binary: Path, expression: str, position: float) -> list[float]:
    normalizer = "x => if type(x) == ratio { x / 100% } else { x }"
    code = (
        f"let norm = {normalizer}; let g = {expression}; "
        f"g.sample({format(position, '.17g')} * 100%).components().map(norm)"
    )
    return list(map(float, runner.eval_json(binary, code)))


def sample_components_at_stop(runner, binary: Path, expression: str, index: int, step: float = 0.0) -> list[float]:
    normalizer = "x => if type(x) == ratio { x / 100% } else { x }"
    position = f"g.stops().at({index}).at(1)"
    if step:
        position += f" + {format(step * 100.0, '.17g')}%"
    code = (
        f"let norm = {normalizer}; let g = {expression}; "
        f"g.sample({position}).components().map(norm)"
    )
    return list(map(float, runner.eval_json(binary, code)))


def execute_mutation_suite() -> tuple[list[dict[str, object]], list[dict[str, object]]]:
    runner = DIAG / "p1268-adversary-attack-runner.py"
    attacks_path = DIAG / "p1268-adversary-attacks.tsv"
    opaque_path = DIAG / "p1268-adversary-opaque-cases.tsv"
    verdicts = {row["attack_id"]: row for row in read_tsv(DIAG / "p1268-verifier-v3-mutation-verdicts.tsv")}
    opaque_verdicts = {row["case_id"]: row for row in read_tsv(DIAG / "p1268-verifier-v3-opaque-verdicts.tsv")}
    mutant_dir = OUT / "mutants"
    opaque_dir = OUT / "opaque"
    mutant_dir.mkdir(parents=True, exist_ok=True)
    opaque_dir.mkdir(parents=True, exist_ok=True)
    attack_rows = []
    for row in read_tsv(attacks_path):
        attack_id = row["attack_id"]
        destination = mutant_dir / f"{attack_id}.json"
        completed = subprocess.run([
            "python3", str(runner), "--suite", str(attacks_path), "--case", attack_id,
            "materialize", "--state", "after", "--output", str(destination),
        ], cwd=ROOT, text=True, capture_output=True)
        materialized = json.loads(destination.read_text()) if completed.returncode == 0 and destination.exists() else None
        expected_after = json.loads(row["after_json"])
        execution = completed.returncode == 0 and canonical(materialized) == canonical(expected_after)
        frozen = verdicts[attack_id]
        adjudicated = frozen["adjudicated"] if execution else "Unknown"
        killed = row["mutation_score"] == "included" and adjudicated == "Violated"
        attack_rows.append({
            "attack_id": attack_id, "kind": row["kind"], "validity": row["validity"],
            "expected": row["expected"], "adjudicated": adjudicated,
            "clause": row["clause"], "runner_exit": completed.returncode,
            "materialized_after_equal": str(execution).lower(),
            "materialized_sha256": sha(destination) if destination.exists() else "missing",
            "mutation_denominator": row["mutation_score"],
            "killed": str(killed).lower() if row["mutation_score"] == "included" else "not-applicable",
            "witness": frozen["witness"] if execution else (completed.stderr.strip() or "materialization mismatch"),
            "status": "Preserved" if execution and adjudicated == row["expected"] else "Unknown",
        })
    opaque_rows = []
    for row in read_tsv(opaque_path):
        case_id = row["case_id"]
        destination = opaque_dir / f"{case_id}.json"
        completed = subprocess.run([
            "python3", str(runner), "--suite", str(opaque_path), "--opaque", "--case", case_id,
            "materialize", "--state", "after", "--output", str(destination),
        ], cwd=ROOT, text=True, capture_output=True)
        materialized = json.loads(destination.read_text()) if completed.returncode == 0 and destination.exists() else None
        expected_state = json.loads(row["canonical_state_json"])
        execution = completed.returncode == 0 and canonical(materialized) == canonical(expected_state)
        frozen = opaque_verdicts[case_id]
        opaque_rows.append({
            "case_id": case_id, "expected": row["expected"],
            "adjudicated": frozen["adjudicated"] if execution else "Unknown",
            "counts_as_success": "false", "mutation_denominator": "excluded",
            "runner_exit": completed.returncode, "materialized_state_equal": str(execution).lower(),
            "materialized_sha256": sha(destination) if destination.exists() else "missing",
            "witness": frozen["witness"] if execution else (completed.stderr.strip() or "materialization mismatch"),
            "status": "ExplicitOpacity" if execution else "Unknown",
        })
    return attack_rows, opaque_rows


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--controls-status", choices=("Preserved", "Violated", "Pending"), default="Pending")
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    runner = load_runner()
    matrix = read_tsv(DIAG / "p1266-generalization-matrix.tsv")
    seeds = [row for row in matrix if row["validity"] == "valid"]
    fixtures = [runner.make_fixture(seed, pair) for seed in seeds for pair in PAIRS]
    raw_results = {row["fixture_id"]: row for row in read_tsv(RAW / "p1266-results.tsv")}
    raw_graph = {row["fixture_id"]: row for row in read_tsv(RAW / "p1266-graph.tsv")}
    raw_raster = {row["fixture_id"]: row for row in read_tsv(RAW / "p1266-raster.tsv")}
    raw_product = {row["fixture_id"]: row for row in read_tsv(RAW / "p1266-product-boundary.tsv")}
    budgets = {row["fixture_id"]: row for row in read_tsv(DIAG / "p1268-oracle-v3-budgets.tsv")}
    frozen_intervals = defaultdict(list)
    for row in read_tsv(DIAG / "p1268-oracle-v3-interval-cost.tsv"):
        frozen_intervals[row["fixture_id"]].append(row)
    frozen_coincidence = defaultdict(list)
    for row in read_tsv(DIAG / "p1268-oracle-v3-coincidence.tsv"):
        frozen_coincidence[row["fixture_id"]].append(row)
    degenerates = {row["fixture_id"]: row for row in read_tsv(DIAG / "p1268-oracle-v3-degenerates.tsv")}
    candidate = ROOT / "target/debug/typst"
    vanilla = Path("/usr/local/bin/typst")
    probe = ROOT / "lab/parity/matrix/p1266_probe/target/debug/p1266-svg-generalization-probe"

    fixture_rows: list[dict[str, object]] = []
    numeric_rows: list[dict[str, object]] = []
    interval_rows: list[dict[str, object]] = []
    coincidence_rows: list[dict[str, object]] = []
    public_rows: list[dict[str, object]] = []

    for fixture in fixtures:
        fid = fixture.fixture_id
        pair = fixture.pair
        raw = raw_results[fid]
        budget = budgets[fid]
        candidate_obs = runner.public_observation(candidate, fixture.expression, mesh=False)
        vanilla_obs = runner.public_observation(vanilla, fixture.expression, mesh=False)
        candidate_stops = candidate_obs["meta"][0]
        vanilla_stops = vanilla_obs["meta"][0]
        morphology = len(candidate_stops) == len(vanilla_stops)
        offset_delta = 0.0
        component_delta = 0.0
        if morphology:
            for candidate_stop, vanilla_stop in zip(candidate_stops, vanilla_stops):
                offset_delta = max(offset_delta, abs(float(candidate_stop[1]) - float(vanilla_stop[1])))
                component_delta = max(component_delta, max_delta(candidate_stop[0], vanilla_stop[0]))
            candidate_offsets = [float(stop[1]) for stop in candidate_stops]
            vanilla_offsets = [float(stop[1]) for stop in vanilla_stops]
            morphology = (
                offset_delta <= EPS
                and all(candidate_offsets[index] <= candidate_offsets[index + 1] + EPS for index in range(len(candidate_offsets) - 1))
                and [sum(abs(value - pivot) <= EPS for value in candidate_offsets) for pivot in candidate_offsets]
                    == [sum(abs(value - pivot) <= EPS for value in vanilla_offsets) for pivot in vanilla_offsets]
                and candidate_obs["meta"][1:3] == vanilla_obs["meta"][1:3]
            )
        public_rows.append({
            "fixture_id": fid, "pair": pair, "candidate_M": len(candidate_stops),
            "vanilla_M": len(vanilla_stops), "max_semantic_offset_delta": offset_delta,
            "max_component_carrier_delta": component_delta,
            "kind_space_equal": str(candidate_obs["meta"][1:3] == vanilla_obs["meta"][1:3]).lower(),
            "ordered_multiplicity_preserved": str(morphology).lower(),
            "mechanics_note": "component carrier delta is not an independent verdict; G05 owns the language envelope",
            "status": "Preserved" if morphology else "Violated",
        })

        live_source = FIXTURES / f"{fid}.typ"
        source_identity = sha(live_source) == budget["source_sha256"]
        live_oracle_svg = RAW / f"oracle-{fid}.svg"
        vanilla_identity = sha(live_oracle_svg) == budget["vanilla_svg_sha256"]
        adaptive = runner.probe_stops(probe, fixture, candidate_obs["meta"])
        adaptive_offsets = [float(stop["offset"]) for stop in adaptive]
        frozen_rows = sorted(frozen_intervals[fid], key=lambda row: int(row["interval"]))
        live_M = len(candidate_stops)
        live_R = 0
        sum_a = 0
        interval_ok = len(frozen_rows) == max(0, live_M - 1)
        for frozen in frozen_rows:
            index = int(frozen["interval"])
            left = float(candidate_stops[index][1])
            right = float(candidate_stops[index + 1][1])
            positive = right > left + 1e-12
            eligible = positive and fixture.anti_alias
            interiors = [value for value in adaptive_offsets if value > left + 1e-10 and value < right - 1e-10]
            a_i = len(interiors) if eligible else 0
            s_i = dyadic_subdivisions(interiors, left, right) if eligible else 0
            d_i = 2 * a_i + 1 if eligible else 0
            b_i = 2 * a_i if eligible else 0
            q_i = d_i + b_i
            live_R += int(eligible)
            sum_a += a_i
            exact_frozen = (
                abs(left - float(frozen["left"])) <= EPS
                and abs(right - float(frozen["right"])) <= EPS
                and str(positive).lower() == frozen["positive_width"]
                and str(eligible).lower() == frozen["eligible"]
                and s_i == int(frozen["s_i"])
                and a_i == int(frozen["a_i"])
                and d_i == int(frozen["midpoint_decisions_d_i"])
                and b_i == int(frozen["boundary_sampler_calls_b_i"])
                and q_i == int(frozen["total_sampler_calls_q_i"])
            )
            bounds = s_i <= 64 and a_i <= 63 and d_i <= 127 and b_i <= 126 and q_i <= 253
            interval_ok = interval_ok and exact_frozen and bounds
            interval_rows.append({
                "fixture_id": fid, "pair": pair, "interval": index,
                "left": format(left, ".17g"), "right": format(right, ".17g"),
                "positive_width": str(positive).lower(), "eligible": str(eligible).lower(),
                "s_i": s_i, "a_i": a_i, "d_i": d_i, "b_i": b_i, "q_i": q_i,
                "bounds_64_63_127_126_253": "Preserved" if bounds else "Violated",
                "frozen_trace_equal": str(exact_frozen).lower(),
                "status": "Preserved" if exact_frozen and bounds else "Violated",
            })
        live_E = len(adaptive)
        cost_identity = (
            interval_ok and live_M == int(budget["M_effective_public_stops"])
            and live_R == int(budget["R_eligible_intervals"])
            and live_E == live_M + sum_a
            and live_E == int(budget["E_emitted_stops"])
            and live_E <= live_M + 63 * live_R
            and live_M + 63 * live_R <= 64 * live_M - 63
        )

        metric_statuses = []
        deltas = []
        for metric in METRICS:
            observed = float(raw[metric])
            limit = float(budget[f"V_{metric}"]) + EPS
            preserved = observed <= limit + 1e-15
            metric_statuses.append(preserved)
            deltas.append(max(0.0, observed - limit))
            numeric_rows.append({
                "fixture_id": fid, "pair": pair, "metric": metric,
                "candidate_observed": format(observed, ".17g"),
                "vanilla_frozen": budget[f"V_{metric}"], "allowed_slack": "0.000001",
                "effective_limit": format(limit, ".17g"),
                "excess": format(max(0.0, observed - limit), ".17g"),
                "status": "Preserved" if preserved else "Violated",
            })
        numeric_ok = all(metric_statuses)

        exact_coincidence_ok = True
        epsilon_coincidence_ok = True
        for frozen in frozen_coincidence[fid]:
            exact_index = int(frozen["exact_expected_index"])
            exact_position = float(candidate_stops[exact_index][1])
            exact = sample_components_at_stop(runner, candidate, fixture.expression, exact_index)
            expected_exact = list(map(float, candidate_stops[exact_index][0]))
            exact_delta = max_delta(exact, expected_exact)
            if frozen["epsilon"] not in ("", "n/a"):
                semantic_step = float(frozen["epsilon"]) - float(frozen["offset"])
                epsilon = exact_position + semantic_step
                epsilon_observed = sample_components_at_stop(
                    runner, candidate, fixture.expression, exact_index, semantic_step
                )
                epsilon_index = int(frozen["epsilon_expected_last_index"])
                epsilon_expected = list(map(float, candidate_stops[epsilon_index][0]))
                epsilon_delta = max_delta(epsilon_observed, epsilon_expected)
                frozen_right_delta = float(frozen["epsilon_delta_to_last"])
                epsilon_ok = epsilon_delta <= frozen_right_delta + EPS
            else:
                epsilon = None
                epsilon_observed = None
                epsilon_delta = 0.0
                epsilon_ok = True
            exact_ok = exact_delta <= EPS
            exact_coincidence_ok = exact_coincidence_ok and exact_ok
            epsilon_coincidence_ok = epsilon_coincidence_ok and epsilon_ok
            coincidence_rows.append({
                "fixture_id": fid, "pair": pair, "offset": frozen["offset"],
                "multiplicity": frozen["multiplicity"], "candidate_exact_json": canonical(exact),
                "frozen_exact_json": frozen["exact_observed_json"],
                "exact_max_delta": format(exact_delta, ".17g"),
                "epsilon": "" if epsilon is None else format(epsilon, ".17g"),
                "candidate_epsilon_json": "" if epsilon_observed is None else canonical(epsilon_observed),
                "frozen_epsilon_json": frozen["epsilon_observed_json"],
                "epsilon_max_delta": format(epsilon_delta, ".17g"),
                "structural_rule": "candidate exact selects expected first/zero special-case index; epsilon-right selects expected last index",
                "status": "Preserved" if exact_ok and epsilon_ok else "Violated",
            })

        graph_ok = raw_graph[fid]["status"] == "Preserved" and vanilla_identity
        raster = raw_raster[fid]
        raster_ok = (
            float(raster["candidate_raster_max"]) <= float(raster["V_raster_max"]) + float(raster["byte_slack"]) + 1e-15
            and float(raster["candidate_raster_p95"]) <= float(raster["V_raster_p95"]) + float(raster["byte_slack"]) + 1e-15
        )
        s20_ok = True
        s21_ok = True
        if fid in degenerates:
            expected_pixels = int(degenerates[fid]["positive_pixels"])
            actual_pixels = int(raster["mask_pixels"])
            if fixture.seed["seed"] == "S20":
                s20_ok = actual_pixels == expected_pixels and actual_pixels > 0 and graph_ok and raster_ok
            if fixture.seed["seed"] == "S21":
                s21_ok = actual_pixels == expected_pixels == 0 and graph_ok
        determinism_ok = True
        product_ok = raw_product[fid]["status"] == "Preserved" and raw_product[fid]["productive_promotion"] == "false"
        gates = {
            "G01": source_identity,
            "G02": True,
            "G03": graph_ok,
            "G04A": morphology and exact_coincidence_ok,
            "G04B": morphology and epsilon_coincidence_ok and interval_ok,
            "G05": numeric_ok,
            "G06": raster_ok,
            "G07A": s20_ok,
            "G07B": s21_ok,
            "G08": cost_identity and determinism_ok,
            "G09": graph_ok,
            "G10": cost_identity,
            "G11": args.controls_status == "Preserved",
            "G12": False,
            "G13": determinism_ok,
        }
        gates["G12"] = all(gates[key] for key in gates if key != "G12") and product_ok
        failed = [gate for gate, value in gates.items() if not value]
        fixture_rows.append({
            "fixture_id": fid, "pair": pair, "seed": fixture.seed["seed"],
            **{gate: "Preserved" if value else "Violated" for gate, value in gates.items()},
            "M": live_M, "R": live_R, "E": live_E, "sum_a_i": sum_a,
            "source_identity": str(source_identity).lower(), "vanilla_svg_identity": str(vanilla_identity).lower(),
            "product_boundary": "Preserved" if product_ok else "Violated",
            "failed_gates": ";".join(failed),
            "status": "Preserved" if not failed and product_ok else "Violated",
        })

    invalid_rows = read_tsv(RAW / "p1266-invalid-domain.tsv")
    determinism_rows = read_tsv(RAW / "p1266-determinism.tsv")
    invalid_ok = len(invalid_rows) == 28 and all(row["status"] == "Preserved" for row in invalid_rows)
    determinism_ok = len(determinism_rows) == 192 and all(row["status"] == "Preserved" for row in determinism_rows)
    for fixture_row in fixture_rows:
        if not determinism_ok:
            fixture_row["G13"] = "Violated"
            fixture_row["G12"] = "Violated"
            fixture_row["status"] = "Violated"
        if not invalid_ok:
            fixture_row["G02"] = "Violated"
            fixture_row["G12"] = "Violated"
            fixture_row["status"] = "Violated"

    attack_rows, opaque_rows = execute_mutation_suite()
    negatives = [row for row in attack_rows if row["mutation_denominator"] == "included"]
    mutation_ok = len(negatives) == 24 and all(row["killed"] == "true" and row["status"] == "Preserved" for row in negatives)
    control_ok = len(attack_rows) == 25 and all(row["status"] == "Preserved" for row in attack_rows)
    opaque_ok = len(opaque_rows) == 10 and all(row["status"] == "ExplicitOpacity" and row["counts_as_success"] == "false" for row in opaque_rows)

    pair_rows = []
    all_failures = []
    for pair in PAIRS:
        members = [row for row in fixture_rows if row["pair"] == pair]
        preserved = sum(row["status"] == "Preserved" for row in members)
        numeric_preserved = sum(row["G05"] == "Preserved" for row in members)
        nonnumeric_failures = sorted({
            gate for row in members for gate in row["failed_gates"].split(";")
            if gate and gate not in ("G05", "G12")
        })
        if not mutation_ok or not control_ok:
            nonnumeric_failures.append("ATTACK-SUITE")
        if not opaque_ok:
            nonnumeric_failures.append("OPAQUE-POLICY")
        if not invalid_ok:
            nonnumeric_failures.append("INVALID-DOMAIN")
        if not determinism_ok:
            nonnumeric_failures.append("DETERMINISM")
        classification = "Generalization-Preserved" if preserved == 24 and not nonnumeric_failures else "Numeric-Failures-Frozen" if not nonnumeric_failures else "Owner-Return"
        pair_rows.append({
            "pair": pair, "population": "24/24", "all_gates": f"{preserved}/24",
            "numeric": f"{numeric_preserved}/24", "nonnumeric_failures": ";".join(sorted(set(nonnumeric_failures))),
            "invalid_rejected": f"{sum(row['pair'] == pair and row['status'] == 'Preserved' for row in invalid_rows)}/{sum(row['pair'] == pair for row in invalid_rows)}",
            "direct_inverse_repeat": "192/192" if determinism_ok else "Violated",
            "mutation_score": f"{sum(row['killed'] == 'true' for row in negatives)}/24",
            "opaque_successes": "0/10", "productive_promotion": "false",
            "classification": classification,
        })
        all_failures.extend((row["fixture_id"], row["failed_gates"]) for row in members if row["status"] != "Preserved")

    nonnumeric_global = any(row["nonnumeric_failures"] for row in pair_rows)
    all_pairs_pass = all(row["classification"] == "Generalization-Preserved" for row in pair_rows)
    numeric_failure_rows = [row for row in numeric_rows if row["status"] == "Violated"]
    if all_pairs_pass:
        route = "P1273"
        verdict = "Generalization-Preserved"
        basis = "all four pairs are 24/24 under G01-G13"
    elif numeric_failure_rows and not nonnumeric_global:
        route = "P1270"
        verdict = "Numeric-Failures-Frozen"
        basis = "only G05 numeric envelope failures remain"
    else:
        route = "OWNER"
        verdict = "Owner-Return"
        basis = "at least one contract/graph/domain/determinism/control obligation failed"

    write_tsv(OUT / "p1269-public-stops.tsv", public_rows)
    write_tsv(OUT / "p1269-numeric.tsv", numeric_rows)
    write_tsv(OUT / "p1269-interval-cost.tsv", interval_rows)
    write_tsv(OUT / "p1269-coincidence.tsv", coincidence_rows)
    write_tsv(OUT / "p1269-fixture-gates.tsv", fixture_rows)
    write_tsv(OUT / "p1269-invalid-domain.tsv", invalid_rows)
    write_tsv(OUT / "p1269-determinism.tsv", determinism_rows)
    write_tsv(OUT / "p1269-mutants.tsv", attack_rows)
    write_tsv(OUT / "p1269-opaque.tsv", opaque_rows)
    write_tsv(OUT / "p1269-pairs.tsv", pair_rows)
    if numeric_failure_rows:
        by_fixture = defaultdict(list)
        for row in numeric_failure_rows:
            by_fixture[row["fixture_id"]].append(row)
        freeze_rows = []
        for fid, failures in sorted(by_fixture.items()):
            fixture = next(item for item in fixtures if item.fixture_id == fid)
            freeze_rows.append({
                "fixture_id": fid, "pair": fixture.pair, "seed": fixture.seed["seed"],
                "source_sha256": sha(FIXTURES / f"{fid}.typ"),
                "frozen_corpus_sha256": sha(RAW / "p1266-corpus.tsv"),
                "failed_metrics": ";".join(row["metric"] for row in failures),
                "max_excess": max(float(row["excess"]) for row in failures),
                "status": "FROZEN-FOR-P1270",
            })
        write_tsv(OUT / "p1269-numeric-failure-freeze.tsv", freeze_rows)
    route_rows = [{
        "verdict": verdict, "route": route, "basis": basis,
        "fixtures": len(fixtures), "numeric_failed_metrics": len(numeric_failure_rows),
        "numeric_failed_fixtures": len({row['fixture_id'] for row in numeric_failure_rows}),
        "invalid_rejected": f"{sum(row['status'] == 'Preserved' for row in invalid_rows)}/{len(invalid_rows)}",
        "determinism": f"{sum(row['status'] == 'Preserved' for row in determinism_rows)}/{len(determinism_rows)}",
        "mutation_score": f"{sum(row['killed'] == 'true' for row in negatives)}/24",
        "opaque_cases": f"{len(opaque_rows)}/10 excluded from success",
        "controls": args.controls_status,
        "classification_boundary": "no productive promotion; no equivalence claim; no P1268 reseal",
    }]
    write_tsv(OUT / "p1269-route.tsv", route_rows)
    print(canonical(route_rows[0]))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
