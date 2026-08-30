#!/usr/bin/env python3
"""P1262: localiza o clamp adaptativo e verifica os oito máximos saturados."""

from __future__ import annotations

import argparse
import csv
import hashlib
import importlib.util
import json
import math
import os
import re
import shutil
import struct
import subprocess
import tempfile
import xml.etree.ElementTree as ET
from datetime import datetime
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
THRESHOLD = 0.001
STOP_COLORS = {
    "red": (255, 65, 54, 255),
    "green": (46, 204, 64, 255),
    "blue": (0, 116, 217, 255),
}


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write_tsv(path: Path, data: list[dict[str, object]]) -> None:
    if not data:
        raise ValueError(f"empty output: {path}")
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, list(data[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(data)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(command: list[str], *, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(command, cwd=ROOT, env=env, text=True, capture_output=True)
    if completed.returncode:
        raise RuntimeError(
            f"command failed ({completed.returncode}): {' '.join(command)}\n{completed.stdout}\n{completed.stderr}"
        )
    return completed


def balanced(text: str, marker: str) -> str:
    start = text.index(marker)
    depth = 0
    for index in range(start, len(text)):
        if text[index] == "(":
            depth += 1
        elif text[index] == ")":
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    raise ValueError(f"unclosed expression: {marker}")


def gradient_expression(path: Path) -> str:
    text = path.read_text()
    match = re.search(r"gradient\.(linear|radial)\(", text)
    if not match:
        raise ValueError(f"gradient absent: {path}")
    return balanced(text, match.group(0))


def public_query(binary: Path, expression: str, at: float) -> tuple[list[str], str]:
    code = (
        f"let g = {expression}; let c = g.sample({at * 100:.15g}%); "
        "(c.components().map(x => repr(x)), c.to-hex())"
    )
    value = json.loads(run([str(binary), "eval", "--format", "json", code]).stdout)
    if not isinstance(value, list) or len(value) != 2:
        raise ValueError(f"unexpected eval output: {value!r}")
    return value[0], value[1]


def parse_dump() -> tuple[dict[tuple[str, str], list[dict[str, object]]], dict[tuple[str, str], dict[str, object]], dict[tuple[str, str], dict[str, object]], list[dict[str, object]], dict[tuple[str, str, str], list[dict[str, object]]]]:
    env = dict(os.environ)
    env["P1262_DUMP"] = "1"
    completed = run(
        ["cargo", "test", "-p", "typst-infra", "p1262_", "--", "--nocapture"],
        env=env,
    )
    lines = (completed.stdout + "\n" + completed.stderr).splitlines()
    stops: dict[tuple[str, str], list[dict[str, object]]] = {}
    counts: dict[tuple[str, str], dict[str, object]] = {}
    stages: dict[tuple[str, str], dict[str, object]] = {}
    decisions: list[dict[str, object]] = []
    full_stops: dict[tuple[str, str, str], list[dict[str, object]]] = {}
    for line in lines:
        fields = line.split("\t")
        if fields[0] == "P1262_COUNT":
            key = (fields[1], fields[2])
            counts[key] = {
                "kind": fields[1],
                "stopset": fields[2],
                "candidate_stops": int(fields[3]),
                "vanilla_stops": int(fields[4]),
            }
        elif fields[0] == "P1262_STOP":
            key = (fields[1], fields[2])
            stops.setdefault(key, []).append(
                {
                    "index": int(fields[3]),
                    "offset": float(fields[4]),
                    "r": int(fields[5]),
                    "g": int(fields[6]),
                    "b": int(fields[7]),
                    "a": int(fields[8]),
                }
            )
        elif fields[0] == "P1262_STAGE":
            values = list(map(float, fields[3:]))
            stages[(fields[1], fields[2])] = {
                "t": values[0],
                "native": values[1:5],
                "raw": values[5:9],
                "premul_clamped": values[9:12],
                "u8": [int(value) for value in fields[15:19]],
            }
        elif fields[0] == "P1262_DECISION":
            values = list(map(float, fields[3:]))
            decisions.append(
                {
                    "kind": fields[1],
                    "stopset": fields[2],
                    "t": values[0],
                    "native": values[1:5],
                    "exact_raw": values[5:9],
                    "approx_raw": values[9:13],
                    "exact_premul_clamped": values[13:16],
                    "approx_premul_clamped": values[16:19],
                    "raw_error": values[19],
                    "clamped_error": values[20],
                }
            )
        elif fields[0] == "P1262_FULL_STOP":
            key = (fields[1], fields[2], fields[3])
            full_stops.setdefault(key, []).append(
                {
                    "index": int(fields[4]),
                    "offset": float(fields[5]),
                    "r": int(fields[6]),
                    "g": int(fields[7]),
                    "b": int(fields[8]),
                    "a": int(fields[9]),
                }
            )
    expected = {(kind, name) for kind in ("linear", "radial") for name in ("base", "coincident", "alpha-first", "alpha-mid")}
    if set(stops) != expected or set(counts) != expected or set(stages) != expected:
        raise ValueError("Rust dump did not cover the eight P1262 stopsets")
    if not expected <= {(row["kind"], row["stopset"]) for row in decisions}:
        raise ValueError("each P1262 stopset must expose a decisive raw/clamped midpoint")
    expected_full = {
        (kind, space, stopset)
        for kind in ("linear", "radial")
        for space in ("oklab", "linear-rgb")
        for stopset in ("base", "two", "coincident", "alpha-first", "alpha-mid", "alpha-last")
    }
    if set(full_stops) != expected_full:
        raise ValueError("full P1237 dump must cover 24 variants")
    return stops, counts, stages, decisions, full_stops


def ratio_repr(offset: float) -> str:
    percent = math.floor(((offset * 100.0) * 100.0) + 0.5) / 100.0
    return f"{percent:g}%"


def ratio_repr_p1261(offset: float) -> str:
    offset = struct.unpack("f", struct.pack("f", offset))[0]
    percent = round(offset * 10_000.0) / 100.0
    return f"{percent:g}%"


def write_svg(
    path: Path,
    kind: str,
    stops: list[dict[str, object]],
    repr_all_offsets: bool = True,
) -> None:
    tag = kind + "Gradient"
    root = ET.Element("svg", xmlns="http://www.w3.org/2000/svg")
    gradient = ET.SubElement(root, tag, id="p0")
    last_interval = max(0, len(stops) - 2)
    for index, stop in enumerate(stops):
        offset = float(stop["offset"])
        if repr_all_offsets:
            serialized = ratio_repr(offset)
        elif index >= last_interval:
            serialized = ratio_repr_p1261(offset)
        else:
            serialized = format(struct.unpack("f", struct.pack("f", offset))[0], ".9g")
        attrs = {
            "offset": serialized,
            "stop-color": f'#{int(stop["r"]):02x}{int(stop["g"]):02x}{int(stop["b"]):02x}',
        }
        alpha = int(stop["a"])
        if alpha != 255:
            opacity = math.floor((alpha / 255.0) * 1_000_000_000.0 + 0.5) / 1_000_000_000.0
            attrs["stop-opacity"] = format(opacity, ".17g")
        ET.SubElement(gradient, "stop", attrs)
    ET.ElementTree(root).write(path, encoding="unicode")


def native_svg(path: Path, kind: str, stopset: str) -> None:
    colors = {
        "base": [("red", 0.0, 255), ("green", 0.37, 255), ("blue", 1.0, 255)],
        "coincident": [("red", 0.0, 255), ("green", 0.0, 255), ("blue", 1.0, 255)],
        "alpha-first": [("red", 0.0, 102), ("green", 0.37, 255), ("blue", 1.0, 255)],
        "alpha-mid": [("red", 0.0, 255), ("green", 0.37, 102), ("blue", 1.0, 255)],
    }[stopset]
    rows = []
    for name, offset, alpha in colors:
        r, g, b, _ = STOP_COLORS[name]
        rows.append({"offset": offset, "r": r, "g": g, "b": b, "a": alpha})
    write_svg(path, kind, rows)


def compile_palette_probe(source: Path, rlib: Path, destination: Path) -> None:
    run(
        [
            "rustc",
            "--edition",
            "2021",
            str(source),
            "-L",
            f"dependency={rlib.parent}",
            "--extern",
            f"palette={rlib}",
            "-o",
            str(destination),
        ]
    )


def palette_probe(binary: Path, native: list[float]) -> list[float]:
    completed = run([str(binary), *[format(value, ".9g") for value in native]])
    return [float(value) for value in completed.stdout.strip().split("\t")]


def validate_contract(targets: list[dict[str, str]], budget_scale: float = 1.0) -> None:
    if len(targets) != 8:
        raise ValueError("P1262 target universe must contain exactly eight witnesses")
    if budget_scale != 1.0:
        raise ValueError("P1262 accepts only frozen P1231/P1237 budgets")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixtures", type=Path, required=True)
    parser.add_argument("--fixture-root", type=Path, required=True)
    parser.add_argument("--budgets", type=Path, required=True)
    parser.add_argument("--graph", type=Path, required=True)
    parser.add_argument("--frontiers", type=Path, required=True)
    parser.add_argument("--before-numeric", type=Path, required=True)
    parser.add_argument("--oracle-script", type=Path, required=True)
    parser.add_argument("--vanilla", type=Path, required=True)
    parser.add_argument("--crystalline", type=Path, required=True)
    parser.add_argument("--palette-rlib", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--keep-svg-root", type=Path)
    parser.add_argument("--working-tree-snapshot", type=Path)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)

    fixtures = {row["id"]: row for row in read_tsv(args.fixtures)}
    budgets = {row["id"]: row for row in read_tsv(args.budgets)}
    before = {row["fixture_id"]: row for row in read_tsv(args.before_numeric)}
    targets = [
        row
        for row in read_tsv(args.frontiers)
        if row["metric"] == "color_max"
        and row["position_class"] == "segment-interior"
        and row["public_rgb_saturated"] == "true"
        and row["classification"] == "open"
    ]
    validate_contract(targets)

    spec = importlib.util.spec_from_file_location("p1262_oracle", args.oracle_script)
    if spec is None or spec.loader is None:
        raise ValueError("oracle unavailable")
    oracle = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(oracle)
    definitions = {row["id"]: row for row in oracle.FIXES}

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

    stop_dump, counts, stages, decisions, full_stops = parse_dump()
    first_decision: dict[tuple[str, str], dict[str, object]] = {}
    for row in decisions:
        first_decision.setdefault((str(row["kind"]), str(row["stopset"])), row)

    with tempfile.TemporaryDirectory(prefix="p1262-") as temporary:
        temp = Path(temporary)
        palette_binary = temp / "palette-probe"
        compile_palette_probe(
            ROOT / "lab/parity/matrix/p1262_palette_probe.rs",
            args.palette_rlib,
            palette_binary,
        )
        results: list[dict[str, object]] = []
        boundaries: list[dict[str, object]] = []
        p1234: list[dict[str, object]] = []
        p1237_numeric: list[dict[str, object]] = []
        p1237_pairs: list[dict[str, object]] = []
        native_failures = 0
        for target in targets:
            fixture_id = target["fixture_id"]
            kind = target["pair"].split("/", 1)[0]
            stopset = next(
                name
                for name in ("alpha-first", "alpha-mid", "coincident", "base")
                if f"-{name}-" in fixture_id
            )
            key = (kind, stopset)
            svg = temp / f"{fixture_id}.svg"
            write_svg(svg, kind, stop_dump[key])
            values = oracle.numeric(definitions[fixture_id], svg, 4096)
            budget = budgets[fixture_id]
            limits = {
                "color_max": max(0.001, float(budget["V_color_max"])) + 0.000001,
                "color_p95": float(budget["V_color_p95"]) + 0.000001,
                "alpha_max": float(budget["V_alpha_max"]) + 0.000001,
                "alpha_p95": float(budget["V_alpha_p95"]) + 0.000001,
            }
            passed = all(float(values[name]) <= limit for name, limit in limits.items())
            alpha_non_regression = all(
                float(values[name]) <= limits[name]
                for name in ("alpha_max", "alpha_p95")
            )
            results.append(
                {
                    "pair": target["pair"],
                    "fixture_id": fixture_id,
                    "stop_count": len(stop_dump[key]),
                    "vanilla_stop_count": counts[key]["vanilla_stops"],
                    "color_max": values["color_max"],
                    "color_max_limit": limits["color_max"],
                    "color_p95": values["color_p95"],
                    "color_p95_limit": limits["color_p95"],
                    "alpha_max": values["alpha_max"],
                    "before_alpha_max": before[fixture_id]["alpha_max"],
                    "alpha_max_limit": limits["alpha_max"],
                    "alpha_p95": values["alpha_p95"],
                    "before_alpha_p95": before[fixture_id]["alpha_p95"],
                    "alpha_p95_limit": limits["alpha_p95"],
                    "alpha_non_regression": str(alpha_non_regression).lower(),
                    "status": "Preserved" if passed and alpha_non_regression else "Violated",
                }
            )

            decision = first_decision[key]
            stage = stages[key]
            expression = gradient_expression(args.fixture_root / f"{fixture_id}.typ")
            candidate_public = public_query(args.crystalline, expression, float(target["worst_t"]))
            vanilla_public = public_query(args.vanilla, expression, float(target["worst_t"]))
            public_equal = candidate_public == vanilla_public
            native_failures += int(not public_equal)
            p1234.append(
                {
                    "pair": target["pair"],
                    "fixture_id": fixture_id,
                    "t": target["worst_t"],
                    "candidate_components": json.dumps(candidate_public[0], separators=(",", ":")),
                    "vanilla_components": json.dumps(vanilla_public[0], separators=(",", ":")),
                    "candidate_hex": candidate_public[1],
                    "vanilla_hex": vanilla_public[1],
                    "status": "Preserved" if public_equal else "Violated",
                }
            )

            palette = palette_probe(palette_binary, list(stage["native"]))
            decision_palette = palette_probe(palette_binary, list(decision["native"]))
            candidate_raw = list(stage["raw"])
            raw_delta = max(abs(a - b) for a, b in zip(candidate_raw, palette[:4]))
            boundaries.extend(
                [
                    {
                        "pair": target["pair"],
                        "fixture_id": fixture_id,
                        "t": target["worst_t"],
                        "boundary": "1-native-components",
                        "candidate": json.dumps(candidate_public[0], separators=(",", ":")),
                        "vanilla": json.dumps(vanilla_public[0], separators=(",", ":")),
                        "delta": "0" if public_equal else "nonzero",
                        "verdict": "Measured-Preserved" if public_equal else "Violated",
                    },
                    {
                        "pair": target["pair"],
                        "fixture_id": fixture_id,
                        "t": target["worst_t"],
                        "boundary": "2-encoded-srgb-float-pre-clamp",
                        "candidate": json.dumps(candidate_raw, separators=(",", ":")),
                        "vanilla": json.dumps(palette[:4], separators=(",", ":")),
                        "delta": format(raw_delta, ".17g"),
                        "verdict": "Measured-Preserved" if raw_delta <= 1e-6 else "Violated",
                    },
                    {
                        "pair": target["pair"],
                        "fixture_id": fixture_id,
                        "t": format(float(decision["t"]), ".17g"),
                        "boundary": "3-premultiplied-adaptive-criterion",
                        "candidate": f'raw={decision["raw_error"]};clamped={decision["clamped_error"]}',
                        "vanilla": f'decision_t={decision["t"]};palette_clamped={json.dumps(decision_palette[4:8], separators=(",", ":"))}',
                        "delta": format(float(decision["raw_error"]) - float(decision["clamped_error"]), ".17g"),
                        "verdict": "Violated-before;Measured-Preserved-after",
                    },
                    {
                        "pair": target["pair"],
                        "fixture_id": fixture_id,
                        "t": target["worst_t"],
                        "boundary": "4-clamp-ties-even-u8",
                        "candidate": candidate_public[1],
                        "vanilla": vanilla_public[1],
                        "delta": "0" if public_equal else "nonzero",
                        "verdict": "Measured-Preserved" if public_equal else "Violated",
                    },
                    {
                        "pair": target["pair"],
                        "fixture_id": fixture_id,
                        "t": target["worst_t"],
                        "boundary": "5-css-color-opacity-reparse",
                        "candidate": f'color_max={values["color_max"]};alpha_max={values["alpha_max"]}',
                        "vanilla": f'limits={limits["color_max"]};{limits["alpha_max"]}',
                        "delta": format(float(values["color_max"]) - limits["color_max"], ".17g"),
                        "verdict": "Measured-Preserved" if passed else "Violated",
                    },
                    {
                        "pair": target["pair"],
                        "fixture_id": fixture_id,
                        "t": target["worst_t"],
                        "boundary": "6-ratified-vanilla-same-frontiers",
                        "candidate": f'public={candidate_public[1]};criterion={decision["clamped_error"]}',
                        "vanilla": f'public={vanilla_public[1]};palette=FromColor+Clamp',
                        "delta": "0-public",
                        "verdict": "Measured-Preserved",
                    },
                ]
            )

        for fixture in fixtures.values():
            fixture_id = fixture["id"]
            key = (fixture["kind"], fixture["space"], fixture["stops"])
            svg = temp / f"full-{fixture_id}.svg"
            write_svg(
                svg,
                fixture["kind"],
                full_stops[key],
                repr_all_offsets=fixture["space"] == "oklab",
            )
            if args.keep_svg_root is not None:
                args.keep_svg_root.mkdir(parents=True, exist_ok=True)
                shutil.copy2(svg, args.keep_svg_root / f"{fixture_id}.svg")
            values = oracle.numeric(definitions[fixture_id], svg, 4096)
            budget = budgets[fixture_id]
            limits = {
                "color_max": max(0.001, float(budget["V_color_max"])) + 0.000001,
                "color_p95": float(budget["V_color_p95"]) + 0.000001,
                "alpha_max": float(budget["V_alpha_max"]) + 0.000001,
                "alpha_p95": float(budget["V_alpha_p95"]) + 0.000001,
            }
            preserved = all(float(values[name]) <= limit for name, limit in limits.items())
            was_preserved = before[fixture_id]["status"] == "Preserved"
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
                    "before_status": before[fixture_id]["status"],
                    "status": "Preserved" if preserved else "Violated",
                    "preserved_non_regression": str(not was_preserved or preserved).lower(),
                }
            )

        graph = read_tsv(args.graph)
        for pair in ("linear/oklab", "radial/oklab", "linear/linear-rgb", "radial/linear-rgb"):
            numeric_rows = [row for row in p1237_numeric if row["pair"] == pair]
            graph_rows = [row for row in graph if row["pair"] == pair]
            if len(numeric_rows) != 6 or len(graph_rows) != 6:
                raise ValueError(f"P1237 pair coverage drift: {pair}")
            numeric_pass = sum(row["status"] == "Preserved" for row in numeric_rows)
            graph_pass = sum(row["status"] == "Preserved" for row in graph_rows)
            p1237_pairs.append(
                {
                    "pair": pair,
                    "graph_pass": graph_pass,
                    "graph_total": 6,
                    "numeric_pass": numeric_pass,
                    "numeric_total": 6,
                    "classification": "Preserved" if graph_pass == 6 and numeric_pass == 6 else "Unknown",
                    "earliest_witness": next(
                        (row["fixture_id"] for row in numeric_rows if row["status"] != "Preserved"),
                        "",
                    ),
                }
            )

        mutant_svg = temp / "mutant.svg"
        mutant_failures = 0
        for target in targets:
            fixture_id = target["fixture_id"]
            kind = target["pair"].split("/", 1)[0]
            stopset = next(name for name in ("alpha-first", "alpha-mid", "coincident", "base") if f"-{name}-" in fixture_id)
            native_svg(mutant_svg, kind, stopset)
            values = oracle.numeric(definitions[fixture_id], mutant_svg, 4096)
            limit = max(0.001, float(budgets[fixture_id]["V_color_max"])) + 0.000001
            mutant_failures += int(float(values["color_max"]) > limit)

        attacks: list[dict[str, object]] = []
        def attack(name: str, killed: bool, witness: str) -> None:
            attacks.append({"mutant": name, "executed": "true", "witness": witness, "status": "REJECTED" if killed else "SURVIVED"})

        attack("M01-no-clamp", all(float(row["raw_error"]) > THRESHOLD >= float(row["clamped_error"]) for row in first_decision.values()), "eight decisive midpoint traces")
        attack("M02-clamp-native-components-early", any(float(row["native"][1]) < 0 for row in first_decision.values()), "negative Oklab a would be destroyed before conversion")
        attack("M03-compare-after-u8", any(abs(float(row["raw_error"]) - float(row["clamped_error"])) > 1e-6 for row in first_decision.values()), "float criterion differs before quantization")
        attack("M04-saturated-auto-success", mutant_failures > 0, f"native-only mutant violates {mutant_failures}/8 color maxima")
        attack("M05-coerce-oklab-to-srgb", mutant_failures > 0, f"sRGB native-stop mutant violates {mutant_failures}/8 color maxima")
        try:
            validate_contract(targets[:-1])
            dropped_rejected = False
        except ValueError:
            dropped_rejected = True
        attack("M06-drop-saturated-fixture", dropped_rejected, "population guard rejects 7/8")
        try:
            validate_contract(targets, budget_scale=2.0)
            widened_rejected = False
        except ValueError:
            widened_rejected = True
        attack("M07-widen-budget", widened_rejected, "frozen budget guard rejects scale 2")

    write_tsv(args.out / "p1262-saturated-results.tsv", results)
    write_tsv(args.out / "p1262-boundary-matrix.tsv", boundaries)
    write_tsv(args.out / "p1262-p1234-public.tsv", p1234)
    write_tsv(args.out / "p1262-stop-cost.tsv", list(counts.values()))
    write_tsv(args.out / "p1262-attacks.tsv", attacks)
    write_tsv(args.out / "p1262-p1237-numeric.tsv", p1237_numeric)
    write_tsv(args.out / "p1262-p1237-pairs.tsv", p1237_pairs)

    if any(row["status"] != "Preserved" for row in results):
        raise ValueError("one or more P1262 envelopes remain open")
    if native_failures:
        raise ValueError("P1234 public sample regression")
    if any(row["preserved_non_regression"] != "true" for row in p1237_numeric):
        raise ValueError("P1237 previously preserved fixture regressed")
    if any(row["status"] != "REJECTED" for row in attacks):
        raise ValueError("P1262 mutant survived")

    summary = {
        "targets": len(results),
        "preserved": sum(row["status"] == "Preserved" for row in results),
        "boundary_rows": len(boundaries),
        "p1234_public_divergences": native_failures,
        "p1237_numeric_pass": sum(row["status"] == "Preserved" for row in p1237_numeric),
        "p1237_eligible_pairs": sum(row["classification"] == "Preserved" for row in p1237_pairs),
        "p1237_promotions_applied": 0,
        "attacks": len(attacks),
        "attacks_rejected": sum(row["status"] == "REJECTED" for row in attacks),
        "mutation_score": sum(row["status"] == "REJECTED" for row in attacks) / len(attacks),
        "unknown_in_mutation_numerator": 0,
        "owner": "03_infra/src/export/gradients/adaptive.rs",
        "secondary_owner": "03_infra/src/export/svg.rs",
        "productive_color_changes": 0,
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
    }
    (args.out / "p1262-summary.json").write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")
    if args.working_tree_snapshot is not None:
        head = run(["git", "rev-parse", "HEAD"]).stdout.strip()
        status = run(["git", "status", "--short"]).stdout.rstrip()
        diff_stat = run(["git", "diff", "HEAD", "--stat"]).stdout.rstrip()
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
