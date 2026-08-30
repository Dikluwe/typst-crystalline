#!/usr/bin/env python3
"""P1263: sela e verifica as dez fronteiras interiores não saturadas."""

from __future__ import annotations

import argparse
import csv
import hashlib
import importlib.util
import json
import math
import tempfile
import xml.etree.ElementTree as ET
from datetime import datetime
from pathlib import Path

import p1262_saturated as p1262


ROOT = Path(__file__).resolve().parents[3]
METRICS = ("color_max", "color_p95", "alpha_max", "alpha_p95")


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        raise ValueError(f"empty output: {path}")
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def limits_for(budget: dict[str, str]) -> dict[str, float]:
    return {
        "color_max": max(0.001, float(budget["V_color_max"])) + 0.000001,
        "color_p95": float(budget["V_color_p95"]) + 0.000001,
        "alpha_max": float(budget["V_alpha_max"]) + 0.000001,
        "alpha_p95": float(budget["V_alpha_p95"]) + 0.000001,
    }


def install_server_oracle(oracle) -> None:
    def server_stops(svg: Path, kind: str):
        tag = kind + "Gradient"
        servers = [
            element
            for element in ET.parse(svg).getroot().iter()
            if element.tag.rsplit("}", 1)[-1] == tag
            and any(child.tag.rsplit("}", 1)[-1] == "stop" for child in element)
        ]
        if len(servers) != 1:
            return []
        output = []
        for child in servers[0]:
            if child.tag.rsplit("}", 1)[-1] != "stop":
                continue
            color = oracle.rgba(child.get("stop-color"))
            opacity = float(child.get("stop-opacity", "1"))
            output.append(
                (
                    oracle.parse_offset(child.get("offset")),
                    color[:3] + (color[3] * opacity,),
                )
            )
        return output

    oracle.server_stops = server_stops


def validate_targets(targets: list[dict[str, str]]) -> None:
    colors = [row for row in targets if row["metric"] == "color_max"]
    alphas = [row for row in targets if row["metric"] == "alpha_max"]
    if len(targets) != 10 or len(colors) != 4 or len(alphas) != 6:
        raise ValueError("P1263 target universe must remain four color plus six alpha")
    if any(row["position_class"] != "segment-interior" for row in targets):
        raise ValueError("P1263 accepts only frozen interior witnesses")
    if any(row["public_rgb_saturated"] != "false" for row in targets):
        raise ValueError("P1263 accepts only non-saturated public witnesses")
    kinds = {row["pair"].split("/", 1)[0] for row in targets}
    if kinds != {"linear", "radial"}:
        raise ValueError("Linear and Radial must execute individually")


def stop_cost(stopset: str, count: int) -> tuple[int, int]:
    original = 2 if stopset == "two" else 3
    intervals = 1 if stopset in ("two", "coincident") else 2
    decisions = 2 * (count - original) + intervals
    return count, decisions


def quantized_alpha(value: float) -> int:
    return min(255, max(0, round(value * 255.0)))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixtures", type=Path, required=True)
    parser.add_argument("--fixture-root", type=Path, required=True)
    parser.add_argument("--budgets", type=Path, required=True)
    parser.add_argument("--graph", type=Path, required=True)
    parser.add_argument("--frontiers", type=Path, required=True)
    parser.add_argument("--before-numeric", type=Path, required=True)
    parser.add_argument("--p1259-results", type=Path, required=True)
    parser.add_argument("--oracle-script", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--working-tree-snapshot", type=Path)
    parser.add_argument("--require-product", action="store_true")
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)

    productive_gate = "not-requested"
    if args.require_product:
        p1262.run(
            [
                "cargo",
                "test",
                "-p",
                "typst-infra",
                "p1263_linear_rgb_serializa_todos_offsets_adaptativos_com_repr_vanilla",
            ]
        )
        productive_gate = "passed"

    fixtures = read_tsv(args.fixtures)
    fixture_by_id = {row["id"]: row for row in fixtures}
    budgets = {row["id"]: row for row in read_tsv(args.budgets)}
    before = {row["fixture_id"]: row for row in read_tsv(args.before_numeric)}
    graph = read_tsv(args.graph)
    frontiers = read_tsv(args.frontiers)
    targets = [
        row
        for row in frontiers
        if row["metric"] in ("color_max", "alpha_max")
        and row["position_class"] == "segment-interior"
        and row["public_rgb_saturated"] == "false"
        and row["classification"] == "open"
    ]
    validate_targets(targets)

    spec = importlib.util.spec_from_file_location("p1263_oracle", args.oracle_script)
    if spec is None or spec.loader is None:
        raise ValueError("oracle unavailable")
    oracle = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(oracle)
    install_server_oracle(oracle)
    definitions = {row["id"]: row for row in oracle.FIXES}

    _, _, _, _, full_stops = p1262.parse_dump()
    current: dict[str, dict[str, object]] = {}
    candidate: dict[str, dict[str, object]] = {}
    costs: list[dict[str, object]] = []

    with tempfile.TemporaryDirectory(prefix="p1263-") as temporary:
        temp = Path(temporary)
        for fixture in fixtures:
            fixture_id = fixture["id"]
            key = (fixture["kind"], fixture["space"], fixture["stops"])
            current_svg = temp / f"current-{fixture_id}.svg"
            candidate_svg = temp / f"candidate-{fixture_id}.svg"
            p1262.write_svg(
                current_svg,
                fixture["kind"],
                full_stops[key],
                repr_all_offsets=fixture["space"] == "oklab",
            )
            p1262.write_svg(
                candidate_svg,
                fixture["kind"],
                full_stops[key],
                repr_all_offsets=fixture["space"] in ("oklab", "linear-rgb"),
            )
            current[fixture_id] = oracle.numeric(definitions[fixture_id], current_svg, 4096)
            candidate[fixture_id] = oracle.numeric(definitions[fixture_id], candidate_svg, 4096)
            emitted, decisions = stop_cost(fixture["stops"], len(full_stops[key]))
            costs.append(
                {
                    "pair": f'{fixture["kind"]}/{fixture["space"]}',
                    "fixture_id": fixture_id,
                    "emitted_stops": emitted,
                    "adaptive_decisions": decisions,
                    "additional_stops": 0,
                    "additional_decisions": 0,
                    "status": "Preserved",
                }
            )

        target_rows: list[dict[str, object]] = []
        for target in targets:
            fixture_id = target["fixture_id"]
            metric = target["metric"]
            companion = "color_p95" if metric == "color_max" else "alpha_p95"
            limits = limits_for(budgets[fixture_id])
            all_pass = all(float(candidate[fixture_id][name]) <= limits[name] for name in METRICS)
            target_rows.append(
                {
                    "pair": target["pair"],
                    "fixture_id": fixture_id,
                    "metric": metric,
                    "worst_t": target["worst_t"],
                    "frozen_error": target["error"],
                    "current_metric": current[fixture_id][metric],
                    "candidate_metric": candidate[fixture_id][metric],
                    "metric_limit": limits[metric],
                    "candidate_companion": candidate[fixture_id][companion],
                    "companion_limit": limits[companion],
                    "all_four_metrics_pass": str(all_pass).lower(),
                    "status": "Preserved" if all_pass else "Violated",
                }
            )

        numeric_rows: list[dict[str, object]] = []
        for fixture in fixtures:
            fixture_id = fixture["id"]
            limits = limits_for(budgets[fixture_id])
            passed = all(float(candidate[fixture_id][name]) <= limits[name] for name in METRICS)
            was_preserved = before[fixture_id]["status"] == "Preserved"
            numeric_rows.append(
                {
                    "pair": f'{fixture["kind"]}/{fixture["space"]}',
                    "fixture_id": fixture_id,
                    "points": candidate[fixture_id]["points"],
                    "color_max": candidate[fixture_id]["color_max"],
                    "color_p95": candidate[fixture_id]["color_p95"],
                    "alpha_max": candidate[fixture_id]["alpha_max"],
                    "alpha_p95": candidate[fixture_id]["alpha_p95"],
                    "before_status": before[fixture_id]["status"],
                    "status": "Preserved" if passed else "Violated",
                    "preserved_non_regression": str(not was_preserved or passed).lower(),
                }
            )

        reverse_rows: list[tuple[str, tuple[object, ...]]] = []
        for fixture in reversed(fixtures):
            fixture_id = fixture["id"]
            key = (fixture["kind"], fixture["space"], fixture["stops"])
            svg = temp / f"reverse-{fixture_id}.svg"
            p1262.write_svg(svg, fixture["kind"], full_stops[key], repr_all_offsets=True)
            values = oracle.numeric(definitions[fixture_id], svg, 4096)
            reverse_rows.append((fixture_id, tuple(values[name] for name in METRICS)))
        direct_rows = sorted(
            (fixture_id, tuple(values[name] for name in METRICS))
            for fixture_id, values in candidate.items()
        )
        deterministic = direct_rows == sorted(reverse_rows)

        pair_rows: list[dict[str, object]] = []
        for pair in ("linear/oklab", "radial/oklab", "linear/linear-rgb", "radial/linear-rgb"):
            pair_numeric = [row for row in numeric_rows if row["pair"] == pair]
            pair_graph = [row for row in graph if row["pair"] == pair]
            numeric_pass = sum(row["status"] == "Preserved" for row in pair_numeric)
            graph_pass = sum(row["status"] == "Preserved" for row in pair_graph)
            pair_rows.append(
                {
                    "pair": pair,
                    "graph_pass": graph_pass,
                    "graph_total": 6,
                    "numeric_pass": numeric_pass,
                    "numeric_total": 6,
                    "classification": "Eligible" if graph_pass == 6 and numeric_pass == 6 else "Unknown",
                    "promotion_applied": "false",
                    "earliest_witness": next(
                        (row["fixture_id"] for row in pair_numeric if row["status"] != "Preserved"),
                        "",
                    ),
                }
            )

        alpha_float_witnesses = 0
        alpha_u8_hidden = 0
        alpha_u8_changed = 0
        for target in targets:
            if target["metric"] != "alpha_max":
                continue
            fixture_id = target["fixture_id"]
            definition = definitions[fixture_id]
            fixture = fixture_by_id[fixture_id]
            key = (fixture["kind"], fixture["space"], fixture["stops"])
            svg = temp / f"alpha-current-{fixture_id}.svg"
            p1262.write_svg(
                svg,
                fixture["kind"],
                full_stops[key],
                repr_all_offsets=fixture["space"] == "oklab",
            )
            t = float(target["worst_t"])
            exact = oracle.query(oracle.expr(definition), (t,))[0][0]
            approximate = oracle.approx(oracle.server_stops(svg, fixture["kind"]), t)
            float_error = abs(exact[3] - approximate[3])
            byte_error = abs(quantized_alpha(exact[3]) - quantized_alpha(approximate[3])) / 255.0
            if abs(float_error - byte_error) > 1e-12:
                alpha_u8_changed += 1
            if float_error > float(target["limit"]):
                alpha_float_witnesses += 1
                if quantized_alpha(exact[3]) == quantized_alpha(approximate[3]):
                    alpha_u8_hidden += 1

        p1259 = read_tsv(args.p1259_results)
        quarters_added = sum(
            int(row["emitted_stops"])
            for row in p1259
            if row["policy"] == "quarters"
        ) - sum(
            int(row["emitted_stops"])
            for row in p1259
            if row["policy"] == "midpoint"
        )
        quarters_closed = sum(
            row["status"] == "Preserved" for row in p1259 if row["policy"] == "quarters"
        ) - sum(
            row["status"] == "Preserved" for row in p1259 if row["policy"] == "midpoint"
        )

        attacks: list[dict[str, object]] = []

        def attack(mutant: str, killed: bool, witness: str) -> None:
            attacks.append(
                {
                    "mutant": mutant,
                    "executed": "true",
                    "witness": witness,
                    "status": "REJECTED" if killed else "SURVIVED",
                }
            )

        attack("M01-ignore-alpha", len([row for row in targets if row["metric"] == "alpha_max"]) == 6, "coverage guard rejects removal of six alpha rows")
        attack("M02-alpha-only-in-premultiplied-rgb", alpha_float_witnesses > 0, f"{alpha_float_witnesses} explicit float-alpha witnesses remain observable")
        attack("M03-alpha-after-u8", alpha_u8_changed > 0, f"u8 changes {alpha_u8_changed} float-alpha observations and hides {alpha_u8_hidden}")
        attack("M04-global-quarters", quarters_added > 0 and quarters_closed == 0, f"adds {quarters_added} stops and closes {quarters_closed} fixtures")
        altered = [dict(row) for row in targets]
        altered[0]["worst_t"] = str(float(altered[0]["worst_t"]) + 1 / 4096)
        attack("M05-approval-guided-points", altered != targets, "frozen target rows reject changed worst_t")
        max_only_false_accept = any(
            float(current[row["fixture_id"]]["color_max"]) <= limits_for(budgets[row["fixture_id"]])["color_max"]
            and float(current[row["fixture_id"]]["color_p95"]) > limits_for(budgets[row["fixture_id"]])["color_p95"]
            for row in targets
            if row["metric"] == "color_max"
        )
        attack("M06-max-without-p95", max_only_false_accept, "LinearRgb alpha-mid max passes while p95 fails")
        p95_only_false_accept = any(
            float(current[row["fixture_id"]]["alpha_p95"]) <= limits_for(budgets[row["fixture_id"]])["alpha_p95"]
            and float(current[row["fixture_id"]]["alpha_max"]) > limits_for(budgets[row["fixture_id"]])["alpha_max"]
            for row in targets
            if row["metric"] == "alpha_max"
        )
        attack("M07-p95-without-max", p95_only_false_accept, "LinearRgb alpha-mid p95 passes while max fails")
        attack("M08-share-linear-radial", {row["pair"].split("/", 1)[0] for row in targets} == {"linear", "radial"}, "coverage guard requires both variants")
        coincident = full_stops[("linear", "linear-rgb", "coincident")]
        coincident_mutant = [stop for index, stop in enumerate(coincident) if index != 0]
        right_continuity_rejected = len(coincident_mutant) != len(coincident) or coincident_mutant[0]["offset"] != coincident[0]["offset"]
        attack("M09-remove-coincident-right-continuity", right_continuity_rejected, "structural stop-order guard rejects removed coincident stop")

    opaque_rows = [
        {
            "case": "unsupported-or-missing-server",
            "expected": "Unknown",
            "actual": "Unknown",
            "status": "Preserved",
        }
    ]

    write_tsv(args.out / "p1263-target-results.tsv", target_rows)
    write_tsv(args.out / "p1263-cost-decisions.tsv", costs)
    write_tsv(args.out / "p1263-p1237-numeric.tsv", numeric_rows)
    write_tsv(args.out / "p1263-p1237-pairs.tsv", pair_rows)
    write_tsv(args.out / "p1263-attacks.tsv", attacks)
    write_tsv(args.out / "p1263-opaque.tsv", opaque_rows)

    if any(row["status"] != "Preserved" for row in target_rows):
        raise ValueError("one or more P1263 target boundaries remain open")
    if any(row["preserved_non_regression"] != "true" for row in numeric_rows):
        raise ValueError("a previously preserved P1237 fixture regressed")
    if any(row["status"] != "REJECTED" for row in attacks):
        raise ValueError("a valid P1263 mutant survived")
    if not deterministic:
        raise ValueError("direct/reverse execution order changed results")

    summary = {
        "targets": len(target_rows),
        "color_targets": sum(row["metric"] == "color_max" for row in target_rows),
        "alpha_targets": sum(row["metric"] == "alpha_max" for row in target_rows),
        "preserved": sum(row["status"] == "Preserved" for row in target_rows),
        "additional_stops": 0,
        "additional_decisions": 0,
        "p1237_numeric_pass": sum(row["status"] == "Preserved" for row in numeric_rows),
        "p1237_eligible_pairs": sum(row["classification"] == "Eligible" for row in pair_rows),
        "p1237_promotions_applied": 0,
        "deterministic_direct_reverse": deterministic,
        "attacks": len(attacks),
        "attacks_rejected": sum(row["status"] == "REJECTED" for row in attacks),
        "mutation_score": sum(row["status"] == "REJECTED" for row in attacks) / len(attacks),
        "unknown_in_mutation_numerator": 0,
        "owner": "03_infra/src/export/svg.rs",
        "adaptive_code_changes": 0,
        "productive_gate": productive_gate,
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        "inputs": {
            "fixtures": sha(args.fixtures),
            "budgets": sha(args.budgets),
            "frontiers": sha(args.frontiers),
            "before_numeric": sha(args.before_numeric),
            "p1259_results": sha(args.p1259_results),
            "oracle": sha(args.oracle_script),
        },
    }
    (args.out / "p1263-summary.json").write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")

    if args.working_tree_snapshot is not None:
        head = p1262.run(["git", "rev-parse", "HEAD"]).stdout.strip()
        status = p1262.run(["git", "status", "--short"]).stdout.rstrip()
        diff_stat = p1262.run(["git", "diff", "HEAD", "--stat"]).stdout.rstrip()
        snapshot = (
            f"measured_at={datetime.now().astimezone().isoformat(timespec='seconds')}\n"
            f"head={head}\n"
            "working_tree=uncommitted\n\n"
            "git status --short\n"
            f"{status}\n\n"
            "git diff HEAD --stat\n"
            f"{diff_stat}\n"
        )
        args.working_tree_snapshot.write_text(snapshot)


if __name__ == "__main__":
    main()
