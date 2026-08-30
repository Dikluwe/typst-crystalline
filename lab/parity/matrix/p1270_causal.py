#!/usr/bin/env python3
"""P1270: contrafactuais causais sobre a populacao numerica congelada por P1269.

Este runner e somente diagnostico. Nao altera baseline, budgets, L0 ou produto.
"""

from __future__ import annotations

import csv
import importlib.util
import math
import subprocess
import sys
from datetime import datetime
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
BASE_PATH = ROOT / "lab/parity/matrix/p1266_generalize.py"
SPEC = importlib.util.spec_from_file_location("p1266_generalize", BASE_PATH)
assert SPEC and SPEC.loader
BASE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = BASE
SPEC.loader.exec_module(BASE)

DIAG = ROOT / "00_nucleo/diagnosticos"
MATRIX = DIAG / "p1266-generalization-matrix.tsv"
FROZEN = DIAG / "p1269-owner-numeric-failure-freeze.tsv"
NUMERIC = DIAG / "p1269-owner-numeric.tsv"
CANDIDATE = ROOT / "target/debug/typst"
VANILLA = Path("/usr/local/bin/typst")
PROBE_F32 = ROOT / "lab/parity/matrix/p1266_probe/target/debug/p1266-svg-generalization-probe"
PROBE_F64 = ROOT / "lab/parity/matrix/p1270_probe/target/debug/p1270-causal-probe"
OUT = Path("/tmp/p1270-causal")


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        raise ValueError(f"empty output: {path}")
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def run(command: list[str]) -> None:
    completed = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    if completed.returncode:
        raise RuntimeError(f"failed: {' '.join(command)}\n{completed.stdout}\n{completed.stderr}")


def as_stops(rows: list[dict[str, object]], serialized: bool) -> list[tuple[float, tuple[float, float, float, float]]]:
    output = []
    for row in rows:
        offset = float(row["offset"])
        if serialized:
            offset = math.floor(offset * 10_000.0 + 0.5) / 10_000.0
        color = tuple(int(row[channel]) / 255 for channel in ("r", "g", "b", "a"))
        output.append((offset, color))
    return output


def server_rows(stops: list[tuple[float, tuple[float, float, float, float]]]) -> list[dict[str, object]]:
    return [
        {
            "offset": offset,
            "r": round(color[0] * 255),
            "g": round(color[1] * 255),
            "b": round(color[2] * 255),
            "a": round(color[3] * 255),
        }
        for offset, color in stops
    ]


def first_stop_delta(left: list[dict[str, object]], right: list[dict[str, object]]) -> tuple[int, float, str]:
    count = min(len(left), len(right))
    for index in range(count):
        delta = abs(float(left[index]["offset"]) - float(right[index]["offset"]))
        color_changed = any(int(left[index][key]) != int(right[index][key]) for key in ("r", "g", "b", "a"))
        if delta > 0 or color_changed:
            return index, delta, "offset+color" if delta > 0 and color_changed else ("offset" if delta > 0 else "color")
    if len(left) != len(right):
        return count, math.inf, "count"
    return -1, 0.0, "none"


def raw_srgb(components: list[float], space: str) -> tuple[float, float, float, float]:
    c0, c1, c2, alpha = map(float, components)
    if space == "oklab":
        ll = c0 + 0.3963377774 * c1 + 0.2158037573 * c2
        mm = c0 - 0.1055613458 * c1 - 0.0638541728 * c2
        ss = c0 - 0.0894841775 * c1 - 1.2914855480 * c2
        l3, m3, s3 = ll**3, mm**3, ss**3
        linear = (
            4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3,
            -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3,
            -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3,
        )
    else:
        linear = (c0, c1, c2)

    def gamma(value: float) -> float:
        return 12.92 * value if value <= 0.0031308 else 1.055 * value ** (1 / 2.4) - 0.055

    return tuple(gamma(value) for value in linear) + (alpha,)


def stage_delta(left_samples: list[list[float]], right_samples: list[list[float]], space: str) -> dict[str, object]:
    maxima = {"native": (0.0, -1), "srgb": (0.0, -1), "clamp": (0.0, -1), "premult": (0.0, -1)}
    for index, (left, right) in enumerate(zip(left_samples, right_samples)):
        native = max(abs(float(a) - float(b)) for a, b in zip(left, right))
        lraw, rraw = raw_srgb(left, space), raw_srgb(right, space)
        srgb = max(abs(a - b) for a, b in zip(lraw, rraw))
        lclamp = tuple(max(0.0, min(1.0, value)) for value in lraw)
        rclamp = tuple(max(0.0, min(1.0, value)) for value in rraw)
        clamp = max(abs(a - b) for a, b in zip(lclamp, rclamp))
        lp = tuple(lclamp[channel] * lclamp[3] for channel in range(3)) + (lclamp[3],)
        rp = tuple(rclamp[channel] * rclamp[3] for channel in range(3)) + (rclamp[3],)
        premult = max(abs(a - b) for a, b in zip(lp, rp))
        for name, value in (("native", native), ("srgb", srgb), ("clamp", clamp), ("premult", premult)):
            if value > maxima[name][0]:
                maxima[name] = (value, index)
    return {key: value for name, pair in maxima.items() for key, value in ((f"{name}_max_delta", pair[0]), (f"{name}_first_or_max_index", pair[1]))}


def public_stop_delta(left_meta: list[object], right_meta: list[object]) -> dict[str, object]:
    left, right = left_meta[0], right_meta[0]
    for stop_index, (left_stop, right_stop) in enumerate(zip(left, right)):
        left_components, left_offset = left_stop
        right_components, right_offset = right_stop
        for component, (a, b) in enumerate(zip(left_components, right_components)):
            if float(a) != float(b):
                return {
                    "public_first_stop": stop_index,
                    "public_first_field": f"component-{component}",
                    "public_candidate_value": a,
                    "public_vanilla_value": b,
                    "public_first_abs_delta": abs(float(a) - float(b)),
                }
        if float(left_offset) != float(right_offset):
            return {
                "public_first_stop": stop_index,
                "public_first_field": "offset",
                "public_candidate_value": left_offset,
                "public_vanilla_value": right_offset,
                "public_first_abs_delta": abs(float(left_offset) - float(right_offset)),
            }
    return {
        "public_first_stop": -1,
        "public_first_field": "none",
        "public_candidate_value": "",
        "public_vanilla_value": "",
        "public_first_abs_delta": 0.0,
    }


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    matrix = {row["seed"]: row for row in read_tsv(MATRIX) if row["validity"] == "valid"}
    frozen = read_tsv(FROZEN)
    numeric = {(row["fixture_id"], row["metric"]): row for row in read_tsv(NUMERIC)}
    if len(frozen) != 24:
        raise ValueError("P1269 frozen population must contain exactly 24 fixtures")

    fixtures = []
    for row in frozen:
        fixture = BASE.make_fixture(matrix[row["seed"]], row["pair"])
        if fixture.fixture_id != row["fixture_id"]:
            raise ValueError(f"fixture identity drift: {row['fixture_id']}")
        fixtures.append((fixture, row))

    checkpoints: list[dict[str, object]] = []
    counterfactuals: list[dict[str, object]] = []
    stopsets: list[dict[str, object]] = []
    for fixture, frozen_row in fixtures:
        kind, space = fixture.pair.split("/")
        source = OUT / f"{fixture.fixture_id}.typ"
        source.write_text(fixture.source)
        vanilla_svg = OUT / f"vanilla-{fixture.fixture_id}.svg"
        run([str(VANILLA), "compile", str(source), str(vanilla_svg)])
        candidate = BASE.public_observation(CANDIDATE, fixture.expression, mesh=True)
        vanilla = BASE.public_observation(VANILLA, fixture.expression, mesh=True)
        budgets = BASE.numeric_metrics(vanilla["samples"], space, BASE.server_stops(vanilla_svg, kind))

        legacy = BASE.probe_stops(PROBE_F32, fixture, candidate["meta"])
        offset_f64 = BASE.probe_stops(PROBE_F64, fixture, candidate["meta"])
        vanilla_meta = BASE.probe_stops(PROBE_F64, fixture, vanilla["meta"])
        first_index, first_delta, first_kind = first_stop_delta(legacy, offset_f64)
        vanilla_index, vanilla_delta, vanilla_kind = first_stop_delta(offset_f64, vanilla_meta)
        vanilla_svg_stops = BASE.server_stops(vanilla_svg, kind)
        algorithm_index, algorithm_delta, algorithm_kind = first_stop_delta(
            server_rows(as_stops(vanilla_meta, True)), server_rows(vanilla_svg_stops)
        )
        stages = stage_delta(candidate["samples"], vanilla["samples"], space)
        public_delta = public_stop_delta(candidate["meta"], vanilla["meta"])
        checkpoints.append({
            "fixture_id": fixture.fixture_id,
            "pair": fixture.pair,
            "seed": fixture.seed["seed"],
            **public_delta,
            **stages,
            "legacy_stop_count": len(legacy),
            "offset_f64_stop_count": len(offset_f64),
            "first_offset_precision_decision_index": first_index,
            "first_offset_precision_delta": first_delta,
            "first_offset_precision_delta_kind": first_kind,
            "first_public_meta_decision_index": vanilla_index,
            "first_public_meta_delta": vanilla_delta,
            "first_public_meta_delta_kind": vanilla_kind,
            "first_adaptive_vs_vanilla_index": algorithm_index,
            "first_adaptive_vs_vanilla_delta": algorithm_delta,
            "first_adaptive_vs_vanilla_delta_kind": algorithm_kind,
            "source": "p1270-causal.py; adaptive.rs:84-165; gradient.rs:903-965",
        })
        stopsets.append({
            "fixture_id": fixture.fixture_id,
            "pair": fixture.pair,
            "legacy_count": len(legacy),
            "offset_f64_count": len(offset_f64),
            "vanilla_meta_count": len(vanilla_meta),
            "legacy_offsets": ";".join(BASE.fmt(float(row["offset"])) for row in legacy),
            "offset_f64_offsets": ";".join(BASE.fmt(float(row["offset"])) for row in offset_f64),
            "vanilla_svg_offsets": ";".join(BASE.fmt(row[0]) for row in vanilla_svg_stops),
        })

        variants = {
            "legacy-serialized": (candidate["samples"], as_stops(legacy, True)),
            "offset-f64-serialized": (candidate["samples"], as_stops(offset_f64, True)),
            "offset-f64-raw": (candidate["samples"], as_stops(offset_f64, False)),
            "vanilla-meta-serialized": (candidate["samples"], as_stops(vanilla_meta, True)),
            "vanilla-svg": (candidate["samples"], vanilla_svg_stops),
            "offset-f64-serialized@vanilla-exact": (vanilla["samples"], as_stops(offset_f64, True)),
            "vanilla-meta-serialized@vanilla-exact": (vanilla["samples"], as_stops(vanilla_meta, True)),
            "vanilla-svg@vanilla-exact": (vanilla["samples"], vanilla_svg_stops),
        }
        metrics = {name: BASE.numeric_metrics(samples, space, stops) for name, (samples, stops) in variants.items()}
        for metric in frozen_row["failed_metrics"].split(";"):
            frozen_metric = numeric[(fixture.fixture_id, metric)]
            frozen_limit = float(frozen_metric["effective_limit"])
            fresh_limit = budgets[metric] + float(frozen_metric["allowed_slack"])
            for variant, values in metrics.items():
                observed = values[metric]
                counterfactuals.append({
                    "fixture_id": fixture.fixture_id,
                    "pair": fixture.pair,
                    "metric": metric,
                    "variant": variant,
                    "sample_basis": "vanilla-exact" if variant.endswith("@vanilla-exact") else "candidate-exact",
                    "observed": observed,
                    "frozen_limit": frozen_limit,
                    "fresh_vanilla_limit": fresh_limit,
                    "frozen_limit_delta": fresh_limit - frozen_limit,
                    "excess_vs_frozen": max(0.0, observed - frozen_limit),
                    "status_vs_frozen": "Preserved" if observed <= frozen_limit else "Violated",
                })

    write_tsv(OUT / "p1270-checkpoints.tsv", checkpoints)
    write_tsv(OUT / "p1270-counterfactuals.tsv", counterfactuals)
    write_tsv(OUT / "p1270-stopsets.tsv", stopsets)
    (OUT / "measurement.txt").write_text(
        f"measured_at={datetime.now().astimezone().isoformat(timespec='seconds')}\n"
        f"head={subprocess.run(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True, capture_output=True, check=True).stdout.strip()}\n"
        "working_tree=uncommitted\n"
    )
    print(f"fixtures={len(fixtures)} failed_metrics={len(counterfactuals) // len(variants)} rows={len(counterfactuals)} out={OUT}")


if __name__ == "__main__":
    main()
