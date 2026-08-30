#!/usr/bin/env python3
"""P1234: localiza a primeira fronteira observável das falhas seladas por P1233.

O probe usa somente interfaces públicas para B01/B02 e o SVG congelado para
B05-B07. Fronteiras internas não observáveis permanecem Unknown.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import importlib.util
import json
import math
import re
import subprocess
import xml.etree.ElementTree as ET
from pathlib import Path


EPS = 1e-7
SIDES = ("minus", "exact", "plus")
BOUNDARIES = ("B01_VANILLA_SAMPLE", "B02_CRYSTALLINE_SAMPLE", "B03_TO_RGBA_F32",
              "B04_ADAPTIVE_METRIC", "B05_U8_QUANTIZATION", "B06_CSS_SERIALIZATION",
              "B07_CSS_REPARSE")


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write_tsv(path: Path, fields: list[str], rows: list[dict[str, object]]) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def balanced(text: str, marker: str) -> str:
    start = text.index(marker)
    depth = 0
    for index in range(start, len(text)):
        if text[index] == "(":
            depth += 1
        elif text[index] == ")":
            depth -= 1
            if depth == 0:
                return text[start:index + 1]
    raise ValueError(f"unclosed expression: {marker}")


def gradient_expression(path: Path) -> str:
    text = path.read_text()
    match = re.search(r"gradient\.(linear|radial)\(", text)
    if not match:
        raise ValueError(f"gradient absent: {path}")
    return balanced(text, match.group(0))


def component_number(value: str) -> float:
    value = value.strip()
    if value.endswith("%"):
        return float(value[:-1]) / 100.0
    if value.endswith("deg"):
        return float(value[:-3]) / 360.0
    return float(value)


def query(binary: Path, binary_sha256: str, expression: str, at: float) -> tuple[list[str] | None, str, str]:
    code = (f"let g = {expression}; let c = g.sample({at * 100:.15g}%); "
            "c.components().map(x => repr(x))")
    cp = subprocess.run([str(binary), "eval", "--format", "json", code],
                        text=True, capture_output=True)
    transcript = "\0".join((str(binary), binary_sha256, code, cp.stdout, cp.stderr, str(cp.returncode)))
    command_hash = hashlib.sha256(transcript.encode()).hexdigest()
    if cp.returncode:
        return None, cp.stderr.strip(), command_hash
    try:
        values = json.loads(cp.stdout)
        if not isinstance(values, list) or not all(isinstance(item, str) for item in values):
            raise ValueError("components are not a string array")
        return values, "public components().map(repr)", command_hash
    except Exception as error:
        return None, repr(error), command_hash


def delta(left: list[str] | None, right: list[str] | None) -> float | None:
    if left is None or right is None or len(left) != len(right):
        return None
    try:
        return max(abs(component_number(a) - component_number(b)) for a, b in zip(left, right))
    except ValueError:
        return None


def local(tag: str) -> str:
    return tag.rsplit("}", 1)[-1]


def css_rgba(color: str, opacity: str) -> tuple[float, float, float, float]:
    value = color.strip().lower()
    if re.fullmatch(r"#[0-9a-f]{6}", value):
        channels = [int(value[index:index + 2], 16) / 255 for index in (1, 3, 5)] + [1.0]
    elif re.fullmatch(r"#[0-9a-f]{8}", value):
        channels = [int(value[index:index + 2], 16) / 255 for index in (1, 3, 5, 7)]
    else:
        raise ValueError(f"unsupported CSS color: {color}")
    if opacity:
        channels[3] = min(1.0, max(0.0, float(opacity)))
    return tuple(channels)  # type: ignore[return-value]


def svg_stops(path: Path, kind: str) -> list[dict[str, object]]:
    wanted = kind + "Gradient"
    servers = [element for element in ET.parse(path).getroot().iter()
               if local(element.tag) == wanted
               and any(local(child.tag) == "stop" for child in element)]
    if len(servers) != 1:
        raise ValueError(f"{path}: expected one {wanted}, got {len(servers)}")
    rows = []
    for child in servers[0]:
        if local(child.tag) != "stop":
            continue
        raw = child.get("offset", "")
        offset = float(raw[:-1]) / 100 if raw.endswith("%") else float(raw)
        color, opacity = child.get("stop-color", ""), child.get("stop-opacity", "")
        rows.append({"offset": offset, "color": color, "opacity": opacity,
                     "rgba": css_rgba(color, opacity)})
    return rows


def enclosing(stops: list[dict[str, object]], at: float) -> tuple[dict[str, object], dict[str, object], float]:
    exact = [stop for stop in stops if abs(float(stop["offset"]) - at) < 1e-12]
    if exact:
        return exact[-1], exact[-1], 0.0
    lo = max((stop for stop in stops if float(stop["offset"]) < at),
             key=lambda stop: float(stop["offset"]), default=stops[0])
    hi = min((stop for stop in stops if float(stop["offset"]) > at),
             key=lambda stop: float(stop["offset"]), default=stops[-1])
    span = float(hi["offset"]) - float(lo["offset"])
    return lo, hi, 0.0 if span == 0 else (at - float(lo["offset"])) / span


def mix(left: tuple[float, ...], right: tuple[float, ...], amount: float) -> tuple[float, ...]:
    return tuple(a * (1 - amount) + b * amount for a, b in zip(left, right))


def diagnostic_checks() -> list[dict[str, object]]:
    """Declare safeguards actually enforced by the runner.

    These are execution checks, not semantic mutants, and therefore do not
    produce a mutation score.
    """
    return [
        {"check": "C01_EXACT_POPULATION", "observed": "15 fixtures;2 meshes each", "status": "enforced"},
        {"check": "C02_MISSING_INPUT", "observed": "fixture and SVG existence required", "status": "enforced"},
        {"check": "C03_BINARY_IDENTITY", "observed": "path and binary SHA included in transcript hash", "status": "enforced"},
        {"check": "C04_OPAQUE_BOUNDARY", "observed": "B03/B04 remain Unknown", "status": "enforced"},
        {"check": "C05_CAUSALITY", "observed": "public divergence never assigns owner", "status": "enforced"},
    ]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mesh", type=Path, required=True)
    parser.add_argument("--fixtures", type=Path, required=True)
    parser.add_argument("--svgs", type=Path, required=True)
    parser.add_argument("--oracle-script", type=Path, required=True)
    parser.add_argument("--vanilla", type=Path, required=True)
    parser.add_argument("--crystalline", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    vanilla_sha256 = sha(args.vanilla)
    crystalline_sha256 = sha(args.crystalline)

    meshes = read_tsv(args.mesh)
    failed = {row["fixture_id"] for row in meshes if row["verdict"] == "fail"}
    if len(failed) != 15:
        raise ValueError(f"expected 15 sealed failing fixtures, got {len(failed)}")
    selected = [row for row in meshes if row["fixture_id"] in failed]
    if len(selected) != 30:
        raise ValueError(f"expected two meshes for each failing fixture, got {len(selected)}")

    spec = importlib.util.spec_from_file_location("p1234_frozen_oracle", args.oracle_script)
    if spec is None or spec.loader is None:
        raise ValueError("cannot load frozen numeric oracle")
    oracle = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(oracle)
    meta = {row["id"]: row for row in oracle.FIXES}

    probes, boundaries, owners, commands = [], [], [], []
    ordered = sorted(selected, key=lambda row: (row["fixture_id"], int(row["mesh"])))
    for ordinal, item in enumerate(ordered, 1):
        fixture_id, mesh = item["fixture_id"], int(item["mesh"])
        fixture = args.fixtures / f"{fixture_id}.typ"
        svg = args.svgs / f"{fixture_id}.svg"
        if not fixture.is_file() or not svg.is_file():
            raise ValueError(f"missing sealed input for {fixture_id}")
        numeric = oracle.numeric(meta[fixture_id], svg, mesh)
        center = float(numeric["worst_t"])
        probe_id = f"P{ordinal:03d}"
        probes.append({"probe_id": probe_id, "fixture_id": fixture_id, "mesh": mesh,
                       "worst_t": format(center, ".17g"), "fixture_sha256": sha(fixture),
                       "svg_sha256": sha(svg)})
        expression = gradient_expression(fixture)
        stops = svg_stops(svg, str(meta[fixture_id]["kind"]))
        public_deltas: list[float] = []
        for side in SIDES:
            at = max(0.0, min(1.0, center + {"minus": -EPS, "exact": 0.0, "plus": EPS}[side]))
            vanilla, vw, vh = query(args.vanilla, vanilla_sha256, expression, at)
            crystal, cw, ch = query(args.crystalline, crystalline_sha256, expression, at)
            commands.extend([
                {"probe_id": probe_id, "side": side, "binary": str(args.vanilla), "command_sha256": vh,
                 "declared_reads": f"{fixture};{args.vanilla}", "status": "0" if vanilla else "nonzero"},
                {"probe_id": probe_id, "side": side, "binary": str(args.crystalline), "command_sha256": ch,
                 "declared_reads": f"{fixture};{args.crystalline}", "status": "0" if crystal else "nonzero"},
            ])
            observed_delta = delta(vanilla, crystal)
            rows = [
                (BOUNDARIES[0], "Measured" if vanilla else "Unknown", vanilla, "", vw),
                (BOUNDARIES[1], "Measured" if crystal else "Unknown", crystal,
                 "" if observed_delta is None else format(observed_delta, ".17g"), cw),
                (BOUNDARIES[2], "Unknown", None, "", "public to-rgba unavailable"),
                (BOUNDARIES[3], "Unknown", None, "", "pre-quantized encoded-sRGB tuple unavailable"),
            ]
            lo, hi, amount = enclosing(stops, at)
            quantized = mix(lo["rgba"], hi["rgba"], amount)  # type: ignore[arg-type]
            serial = f'{lo["color"]}|{lo["opacity"]};{hi["color"]}|{hi["opacity"]};u={amount:.17g}'
            rows.extend([
                (BOUNDARIES[4], "Measured-output-only", list(quantized), "", "interpolation of emitted quantized endpoints"),
                (BOUNDARIES[5], "Measured-output-only", list(quantized), "", serial),
                (BOUNDARIES[6], "Measured-output-only", list(quantized), "", "#RRGGBB/#RRGGBBAA output parser"),
            ])
            for boundary, verdict, value, adjacent_delta, witness in rows:
                boundaries.append({"probe_id": probe_id, "fixture_id": fixture_id, "mesh": mesh,
                                   "side": side, "t": format(at, ".17g"), "boundary": boundary,
                                   "verdict": verdict, "value": json.dumps(value, separators=(",", ":")),
                                   "boundary_delta": adjacent_delta, "alpha_delta": "",
                                   "serialized": serial if boundary in BOUNDARIES[4:] else "", "witness": witness})
            if observed_delta is not None and observed_delta > 0:
                public_deltas.append(observed_delta)
        observed_boundary = BOUNDARIES[1] if public_deltas else "None-observed"
        observed_delta_text = format(max(public_deltas), ".17g") if public_deltas else ""
        owner = "Unknown"
        verdict = ("DIVERGENCE_OBSERVED_CAUSALITY_UNKNOWN" if public_deltas
                   else "NO_PUBLIC_DIVERGENCE_CAUSALITY_UNKNOWN")
        owners.append({"probe_id": probe_id, "fixture_id": fixture_id, "mesh": mesh,
                       "first_observed_boundary": observed_boundary,
                       "observed_delta_max": observed_delta_text, "owner": owner,
                       "verdict": verdict})

    check_rows = diagnostic_checks()
    write_tsv(args.out / "probes.tsv", list(probes[0]), probes)
    write_tsv(args.out / "boundaries.tsv", list(boundaries[0]), boundaries)
    write_tsv(args.out / "owners.tsv", list(owners[0]), owners)
    write_tsv(args.out / "commands.tsv", list(commands[0]), commands)
    write_tsv(args.out / "attacks.tsv", list(check_rows[0]), check_rows)
    clusters = []
    for boundary in sorted({row["first_observed_boundary"] for row in owners}):
        members = [row["probe_id"] for row in owners if row["first_observed_boundary"] == boundary]
        clusters.append({"cluster": boundary, "probe_count": len(members), "probe_ids": ",".join(members)})
    write_tsv(args.out / "clusters.tsv", list(clusters[0]), clusters)
    summary = {"fixture_count": len(failed), "probe_count": len(probes),
               "boundary_rows": len(boundaries),
               "public_divergences": sum(row["first_observed_boundary"] == BOUNDARIES[1] for row in owners),
               "l1_causes": 0, "l3_causes": 0,
               "unknown_causes": sum(row["owner"] == "Unknown" for row in owners),
               "mutation_score": None,
               "isolation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO"}
    (args.out / "summary.json").write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
