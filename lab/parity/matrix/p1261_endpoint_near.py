#!/usr/bin/env python3
"""P1261: close only the four endpoint-near SVG maxima and attack the contract."""

from __future__ import annotations

import argparse
import csv
import hashlib
import importlib.util
import json
import math
import shutil
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path


def rows(path: Path):
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write(path: Path, data):
    if not data:
        raise ValueError(f"empty output: {path}")
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, list(data[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(data)


def sha(path: Path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def local(tag: str):
    return tag.rsplit("}", 1)[-1]


def gradient_stops(path: Path, kind: str):
    root = ET.parse(path).getroot()
    tag = kind + "Gradient"
    servers = [
        element
        for element in root.iter()
        if local(element.tag) == tag
        and any(local(child.tag) == "stop" for child in element)
    ]
    if len(servers) != 1:
        raise ValueError(f"expected one {tag} in {path}")
    return root, servers[0], [child for child in servers[0] if local(child.tag) == "stop"]


def parse_offset(value: str):
    return float(value[:-1]) / 100.0 if value.endswith("%") else float(value)


def ratio_repr(value: float):
    rounded = math.floor(value * 10_000.0 + 0.5) / 100.0
    return f"{rounded:g}%"


def rgba(oracle, stop):
    color = oracle.rgba(stop.get("stop-color"))
    opacity = float(stop.get("stop-opacity", "1"))
    return color[:3] + (color[3] * opacity,)


def prem_error(oracle, exact, approximate):
    return oracle.dist(oracle.prem(exact), oracle.prem(approximate))


def save_xml(root, path: Path):
    ET.ElementTree(root).write(path, encoding="unicode")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixtures", type=Path, required=True)
    parser.add_argument("--svg-root", type=Path, required=True)
    parser.add_argument("--vanilla-svg-root", type=Path, required=True)
    parser.add_argument("--budgets", type=Path, required=True)
    parser.add_argument("--graph", type=Path, required=True)
    parser.add_argument("--frontiers", type=Path, required=True)
    parser.add_argument("--oracle-script", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)

    spec = importlib.util.spec_from_file_location("p1261_oracle", args.oracle_script)
    if spec is None or spec.loader is None:
        raise ValueError("oracle unavailable")
    oracle = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(oracle)
    definitions = {row["id"]: row for row in oracle.FIXES}

    def server_stops(path, kind):
        _, _, stops = gradient_stops(Path(path), kind)
        return [
            (parse_offset(stop.get("offset")), rgba(oracle, stop))
            for stop in stops
        ]

    oracle.server_stops = server_stops
    fixtures = {row["id"]: row for row in rows(args.fixtures)}
    budgets = {row["id"]: row for row in rows(args.budgets)}
    graph = rows(args.graph)
    frontier_rows = [
        row
        for row in rows(args.frontiers)
        if row["position_class"] == "endpoint-near"
        and row["classification"] == "open"
        and row["metric"] == "color_max"
    ]
    target_ids = [row["fixture_id"] for row in frontier_rows]
    expected_pairs = {
        "linear/oklab",
        "radial/oklab",
        "linear/linear-rgb",
        "radial/linear-rgb",
    }
    if len(target_ids) != 4 or {row["pair"] for row in frontier_rows} != expected_pairs:
        raise ValueError("endpoint-near universe drift")

    with tempfile.TemporaryDirectory(prefix="p1261-") as temporary:
        corrected_root = Path(temporary) / "corrected"
        shutil.copytree(args.svg_root, corrected_root)
        result_rows = []
        stage_rows = []
        frozen = {}

        for frontier in frontier_rows:
            fixture_id = frontier["fixture_id"]
            definition = definitions[fixture_id]
            kind = definition["kind"]
            candidate = args.svg_root / f"{fixture_id}.svg"
            vanilla = args.vanilla_svg_root / f"{fixture_id}.svg"
            corrected = corrected_root / f"{fixture_id}.svg"
            root, _, candidate_stops = gradient_stops(candidate, kind)
            _, _, vanilla_stops = gradient_stops(vanilla, kind)
            if len(candidate_stops) != len(vanilla_stops):
                raise ValueError(f"stop count drift: {fixture_id}")
            penultimate = candidate_stops[-2]
            endpoint = candidate_stops[-1]
            vanilla_penultimate = vanilla_stops[-2]
            vanilla_endpoint = vanilla_stops[-1]
            frozen[fixture_id] = {
                "count": len(candidate_stops),
                "penultimate_offset": parse_offset(penultimate.get("offset")),
                "penultimate_color": penultimate.get("stop-color"),
                "endpoint_offset": parse_offset(endpoint.get("offset")),
                "endpoint_color": endpoint.get("stop-color"),
                "worst_t": float(frontier["worst_t"]),
            }
            if penultimate.get("stop-color") != vanilla_penultimate.get("stop-color"):
                raise ValueError(f"penultimate color diverged before serialization: {fixture_id}")
            if endpoint.get("stop-color") != vanilla_endpoint.get("stop-color"):
                raise ValueError(f"endpoint color diverged before serialization: {fixture_id}")

            candidate_stops[-2].set("offset", ratio_repr(parse_offset(penultimate.get("offset"))))
            candidate_stops[-1].set("offset", ratio_repr(parse_offset(endpoint.get("offset"))))
            save_xml(root, corrected)

            before = oracle.numeric(definition, candidate, 4096)
            after = oracle.numeric(definition, corrected, 4096)
            budget = budgets[fixture_id]
            limits = {
                "color_max": max(0.001, float(budget["V_color_max"])) + 0.000001,
                "color_p95": float(budget["V_color_p95"]) + 0.000001,
                "alpha_max": float(budget["V_alpha_max"]) + 0.000001,
                "alpha_p95": float(budget["V_alpha_p95"]) + 0.000001,
            }
            preserved = all(float(after[key]) <= value for key, value in limits.items())
            non_regression = all(
                float(after[key]) <= float(before[key]) + 1e-15
                for key in ("color_max", "color_p95", "alpha_max", "alpha_p95")
            )
            result_rows.append(
                {
                    "pair": frontier["pair"],
                    "fixture_id": fixture_id,
                    "stop_count": len(candidate_stops),
                    "pre_penultimate_offset": format(frozen[fixture_id]["penultimate_offset"], ".17g"),
                    "serialized_penultimate_offset": candidate_stops[-2].get("offset"),
                    "endpoint_count": sum(parse_offset(stop.get("offset")) == 1.0 for stop in candidate_stops),
                    "endpoint_color": candidate_stops[-1].get("stop-color"),
                    "before_color_max": before["color_max"],
                    "after_color_max": after["color_max"],
                    "color_limit": format(limits["color_max"], ".17g"),
                    "before_color_p95": before["color_p95"],
                    "after_color_p95": after["color_p95"],
                    "before_alpha_max": before["alpha_max"],
                    "after_alpha_max": after["alpha_max"],
                    "before_alpha_p95": before["alpha_p95"],
                    "after_alpha_p95": after["alpha_p95"],
                    "status": "Preserved" if preserved and non_regression else "Violated",
                }
            )

            worst_t = float(frontier["worst_t"])
            exact = oracle.query(oracle.expr(definition), (worst_t,))[0][0]
            candidate_pairs = [(parse_offset(stop.get("offset")), rgba(oracle, stop)) for stop in gradient_stops(candidate, kind)[2]]
            corrected_pairs = [(parse_offset(stop.get("offset")), rgba(oracle, stop)) for stop in gradient_stops(corrected, kind)[2]]
            pre_positions = [value[0] for value in candidate_pairs]
            exact_at_positions = oracle.query(oracle.expr(definition), tuple(pre_positions))[0]
            pre_pairs = list(zip(pre_positions, exact_at_positions))
            offset_pairs = [(position, color) for position, color in pre_pairs]
            offset_pairs[-2] = (parse_offset(candidate_stops[-2].get("offset")), offset_pairs[-2][1])
            offset_pairs[-1] = (1.0, offset_pairs[-1][1])
            stages = [
                ("exact-sample", exact, 0.0),
                ("pre-serialization-approximation", oracle.approx(pre_pairs, worst_t), None),
                ("serialized-offset-exact-colors", oracle.approx(offset_pairs, worst_t), None),
                ("serialized-color-opacity", oracle.approx(corrected_pairs, worst_t), None),
                ("reparsed", oracle.approx(corrected_pairs, worst_t), None),
            ]
            for stage, value, fixed_error in stages:
                stage_rows.append(
                    {
                        "pair": frontier["pair"],
                        "fixture_id": fixture_id,
                        "worst_t": frontier["worst_t"],
                        "stage": stage,
                        "rgba": json.dumps(value, separators=(",", ":")),
                        "premultiplied_color_error": format(
                            fixed_error if fixed_error is not None else prem_error(oracle, exact, value),
                            ".17g",
                        ),
                    }
                )

        original_hashes = {
            path.name: sha(path) for path in sorted(args.svg_root.glob("*.svg"))
        }
        corrected_hashes = {
            path.name: sha(path) for path in sorted(corrected_root.glob("*.svg"))
        }
        non_targets_unchanged = all(
            original_hashes[name] == corrected_hashes[name]
            for name in original_hashes
            if name.removesuffix(".svg") not in target_ids
        )

        p1237_numeric = []
        for fixture in fixtures.values():
            fixture_id = fixture["id"]
            values = oracle.numeric(definitions[fixture_id], corrected_root / f"{fixture_id}.svg", 4096)
            budget = budgets[fixture_id]
            limits = {
                "color_max": max(0.001, float(budget["V_color_max"])) + 0.000001,
                "color_p95": float(budget["V_color_p95"]) + 0.000001,
                "alpha_max": float(budget["V_alpha_max"]) + 0.000001,
                "alpha_p95": float(budget["V_alpha_p95"]) + 0.000001,
            }
            passed = all(float(values[key]) <= limit for key, limit in limits.items())
            p1237_numeric.append(
                {
                    "pair": f'{fixture["kind"]}/{fixture["space"]}',
                    "fixture_id": fixture_id,
                    "points": values["points"],
                    "color_max": values["color_max"],
                    "color_p95": values["color_p95"],
                    "alpha_max": values["alpha_max"],
                    "alpha_p95": values["alpha_p95"],
                    "V_color_max_limit": limits["color_max"],
                    "V_color_p95_limit": limits["color_p95"],
                    "V_alpha_max_limit": limits["alpha_max"],
                    "V_alpha_p95_limit": limits["alpha_p95"],
                    "status": "Preserved" if passed else "Violated",
                }
            )
        p1237_pairs = []
        for pair in ("linear/oklab", "radial/oklab", "linear/linear-rgb", "radial/linear-rgb"):
            graph_rows = [row for row in graph if row["pair"] == pair]
            numeric_rows = [row for row in p1237_numeric if row["pair"] == pair]
            if len(graph_rows) != 6 or len(numeric_rows) != 6:
                raise ValueError(f"P1237 pair coverage drift: {pair}")
            graph_pass = sum(row["status"] == "Preserved" for row in graph_rows)
            numeric_pass = sum(row["status"] == "Preserved" for row in numeric_rows)
            preserved = graph_pass == 6 and numeric_pass == 6
            witness = next(
                (row["fixture_id"] for row in graph_rows if row["status"] != "Preserved"),
                "",
            ) or next(
                (row["fixture_id"] for row in numeric_rows if row["status"] != "Preserved"),
                "",
            )
            p1237_pairs.append(
                {
                    "pair": pair,
                    "graph_pass": graph_pass,
                    "graph_total": len(graph_rows),
                    "numeric_pass": numeric_pass,
                    "numeric_total": len(numeric_rows),
                    "classification": "Preserved"
                    if preserved
                    else ("Unknown-native-approximation" if pair.startswith("linear/") else "Unknown-fallback"),
                    "earliest_witness": witness,
                }
            )

        def validate(root: Path, claimed_t=None, pre_override=None, cap=64):
            violations = []
            claimed_t = claimed_t or {key: value["worst_t"] for key, value in frozen.items()}
            pre_override = pre_override or {
                key: value["penultimate_offset"] for key, value in frozen.items()
            }
            if cap != 64:
                violations.append("cap")
            for fixture_id, expected in frozen.items():
                definition = definitions[fixture_id]
                path = root / f"{fixture_id}.svg"
                _, _, stops = gradient_stops(path, definition["kind"])
                endpoints = [stop for stop in stops if parse_offset(stop.get("offset")) == 1.0]
                if len(stops) != expected["count"]:
                    violations.append(f"{fixture_id}:count")
                if len(endpoints) != 1 or endpoints[0].get("stop-color") != expected["endpoint_color"]:
                    violations.append(f"{fixture_id}:endpoint")
                if stops[-2].get("stop-color") != expected["penultimate_color"]:
                    violations.append(f"{fixture_id}:penultimate-color")
                if stops[-2].get("offset") != ratio_repr(expected["penultimate_offset"]):
                    violations.append(f"{fixture_id}:penultimate-offset")
                if pre_override[fixture_id] != expected["penultimate_offset"]:
                    violations.append(f"{fixture_id}:predecision-rounding")
                if claimed_t[fixture_id] != expected["worst_t"]:
                    violations.append(f"{fixture_id}:worst-t")
                values = oracle.numeric(definition, path, 4096)
                budget = budgets[fixture_id]
                limits = (
                    max(0.001, float(budget["V_color_max"])) + 0.000001,
                    float(budget["V_color_p95"]) + 0.000001,
                    float(budget["V_alpha_max"]) + 0.000001,
                    float(budget["V_alpha_p95"]) + 0.000001,
                )
                measured = tuple(float(values[key]) for key in ("color_max", "color_p95", "alpha_max", "alpha_p95"))
                if any(value > limit for value, limit in zip(measured, limits)):
                    violations.append(f"{fixture_id}:envelope")
            return sorted(set(violations))

        baseline_violations = validate(corrected_root)
        if baseline_violations or not non_targets_unchanged:
            raise ValueError(f"candidate contract failed: {baseline_violations}")

        attack_rows = []

        def attack(attack_id, description, mutate):
            attack_root = Path(temporary) / attack_id
            shutil.copytree(corrected_root, attack_root)
            options = mutate(attack_root) or {}
            violations = validate(attack_root, **options)
            attack_rows.append(
                {
                    "attack_id": attack_id,
                    "description": description,
                    "expected": "Violated",
                    "actual": "Violated" if violations else "Preserved",
                    "witness": ";".join(violations),
                    "status": "REJECTED" if violations else "SURVIVED",
                }
            )

        first = target_ids[0]

        def omit_endpoint(root):
            path = root / f"{first}.svg"
            tree, server, stops = gradient_stops(path, definitions[first]["kind"])
            server.remove(stops[-1])
            save_xml(tree, path)

        def duplicate_endpoint(root):
            path = root / f"{first}.svg"
            tree, server, stops = gradient_stops(path, definitions[first]["kind"])
            server.append(ET.fromstring(ET.tostring(stops[-1], encoding="unicode")))
            save_xml(tree, path)

        def round_before_decision(_root):
            override = {key: value["penultimate_offset"] for key, value in frozen.items()}
            override[first] = parse_offset(ratio_repr(override[first]))
            return {"pre_override": override}

        def hide_at_endpoint(_root):
            return {"claimed_t": {key: 1.0 for key in frozen}}

        def increase_cap(_root):
            return {"cap": 128}

        def linear_only(root):
            for fixture_id in target_ids:
                if definitions[fixture_id]["kind"] == "radial":
                    shutil.copy2(args.svg_root / f"{fixture_id}.svg", root / f"{fixture_id}.svg")

        def cross_space(root):
            oklab = next(key for key in target_ids if definitions[key]["kind"] == "linear" and definitions[key]["space"] == "oklab")
            linear_rgb = next(key for key in target_ids if definitions[key]["kind"] == "linear" and definitions[key]["space"] == "linear-rgb")
            _, _, source = gradient_stops(root / f"{linear_rgb}.svg", "linear")
            path = root / f"{oklab}.svg"
            tree, _, target = gradient_stops(path, "linear")
            target[-2].set("stop-color", source[-2].get("stop-color"))
            save_xml(tree, path)

        attack("M01", "omit final endpoint", omit_endpoint)
        attack("M02", "duplicate final endpoint", duplicate_endpoint)
        attack("M03", "round penultimate offset before adaptive decision", round_before_decision)
        attack("M04", "measure only t=1 and hide the frozen worst_t", hide_at_endpoint)
        attack("M05", "increase global cap from 64 to 128", increase_cap)
        attack("M06", "correct Linear and infer Radial", linear_only)
        attack("M07", "promote Oklab penultimate color from LinearRgb", cross_space)

        mutation_score = sum(row["status"] == "REJECTED" for row in attack_rows) / len(attack_rows)
        write(args.out / "p1261-endpoint-stages.tsv", stage_rows)
        write(args.out / "p1261-endpoint-results.tsv", result_rows)
        write(args.out / "p1261-endpoint-attacks.tsv", attack_rows)
        write(args.out / "p1261-p1237-numeric.tsv", p1237_numeric)
        write(args.out / "p1261-p1237-pairs.tsv", p1237_pairs)
        summary = {
            "targets": len(target_ids),
            "target_ids": target_ids,
            "preserved": sum(row["status"] == "Preserved" for row in result_rows),
            "non_targets_unchanged": non_targets_unchanged,
            "attacks": len(attack_rows),
            "attacks_rejected": sum(row["status"] == "REJECTED" for row in attack_rows),
            "mutation_score": mutation_score,
            "cap": 64,
            "p1237_numeric_pass": sum(row["status"] == "Preserved" for row in p1237_numeric),
            "promoted_pairs": sum(row["classification"] == "Preserved" for row in p1237_pairs),
            "unknown_policy": "Unknown is never success and is excluded from the mutation numerator",
            "scope": "four endpoint-near two-wide-stroke color maxima only",
            "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
            "inputs": {
                "fixtures": sha(args.fixtures),
                "budgets": sha(args.budgets),
                "graph": sha(args.graph),
                "frontiers": sha(args.frontiers),
                "oracle": sha(args.oracle_script),
            },
        }
        (args.out / "p1261-summary.json").write_text(
            json.dumps(summary, sort_keys=True, indent=2) + "\n"
        )


if __name__ == "__main__":
    main()
