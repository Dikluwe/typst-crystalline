#!/usr/bin/env python3
"""P1233: join semântico P1229/P1231 sem ler campos candidate_*.

O script distingue igualdade exata de fixture (F01) de pertencimento às
classes normativas já exigidas por P1229 (F02). A seleção consome uma allowlist
de uma única coluna, sem vereditos anteriores. O gate numérico é calculado
exclusivamente das malhas 2048/4096 produzidas nesta execução.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import importlib.util
import json
import re
import xml.etree.ElementTree as ET
from pathlib import Path


PROMOTIONS = {
    ("linear", "oklab"),
    ("linear", "linear-rgb"),
    ("radial", "hsv"),
}


def balanced_call(text: str, marker: str) -> str:
    start = text.index(marker) + len(marker)
    depth = 1
    for index in range(start, len(text)):
        char = text[index]
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return text[start:index]
    raise ValueError(f"unclosed call: {marker}")


def top_level_parts(value: str) -> list[str]:
    parts: list[str] = []
    depth = 0
    start = 0
    for index, char in enumerate(value):
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
        elif char == "," and depth == 0:
            parts.append(value[start:index].strip())
            start = index + 1
    parts.append(value[start:].strip())
    return [part for part in parts if part]


def normalize(value: str) -> str:
    return re.sub(r"\s+", "", value).lower()


def parse_gradient(call: str, variant: str) -> dict[str, object]:
    parts = top_level_parts(call)
    stops = [normalize(part) for part in parts if part.startswith("(")]
    named = {}
    for part in parts:
        if ":" in part and not part.startswith("("):
            key, value = part.split(":", 1)
            named[normalize(key)] = normalize(value)
    space = named.get("space", "color.oklab").removeprefix("color.")
    if space == "rgb":
        space = "srgb"
    alpha_positions = []
    for index, stop in enumerate(stops):
        if "transparentize" in stop or re.search(r"#[0-9a-f]{8}", stop):
            alpha_positions.append(index)
    offsets = []
    for stop in stops:
        match = re.search(r",(-?[0-9.]+)%\)$", stop)
        offsets.append(match.group(1) if match else "auto")
    return {
        "variant": variant,
        "space": space,
        "stops": stops,
        "offsets": offsets,
        "alpha_positions": alpha_positions,
        "coincident": len(offsets) != len(set(offsets)),
        "geometry": {key: value for key, value in named.items() if key != "space"},
    }


def signature(record: dict[str, object]) -> str:
    payload = json.dumps(record, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(payload.encode()).hexdigest()


def p1229_records(root: Path) -> list[dict[str, object]]:
    records = []
    for path in sorted(root.glob("s*.typ")):
        text = path.read_text()
        for short, variant in (("lin", "linear"), ("rad", "radial")):
            marker = f"#let {short} = gradient.{variant}("
            if marker not in text:
                continue
            gradient = parse_gradient(balanced_call(text, marker), variant)
            if (variant, str(gradient["space"])) not in PROMOTIONS and gradient["space"] != "srgb":
                continue
            for role in ("fill", "stroke"):
                record = {
                    **gradient,
                    "role": role,
                    "box": "100ptx50pt",
                    "source": path.name,
                }
                record["signature"] = signature({key: value for key, value in record.items() if key != "source"})
                records.append(record)
    return records


def p1231_records(root: Path, allowed_ids: set[str]) -> list[dict[str, object]]:
    records = []
    for path in sorted(root.glob("regression-*.typ")):
        name = path.stem
        if name not in allowed_ids:
            continue
        match = re.match(r"regression-(linear|radial)-(srgb|oklab|linear-rgb|hsv)-", name)
        if not match:
            continue
        variant, space = match.groups()
        if (variant, space) not in PROMOTIONS and space != "srgb":
            continue
        text = path.read_text()
        marker = f"gradient.{variant}("
        gradient = parse_gradient(balanced_call(text, marker), variant)
        role = "stroke" if "stroke:" in text else "fill"
        rect = balanced_call(text, "rect(")
        width = re.search(r"width:([0-9]+pt)", normalize(rect))
        height = re.search(r"height:([0-9]+pt)", normalize(rect))
        record = {
            **gradient,
            "role": role,
            "box": f"{width.group(1) if width else 'auto'}x{height.group(1) if height else 'auto'}",
            "source": path.name,
        }
        record["signature"] = signature({key: value for key, value in record.items() if key != "source"})
        records.append(record)
    return records


def classes(record: dict[str, object]) -> set[str]:
    result = {str(record["role"])}
    stops = list(record["stops"])
    alpha = list(record["alpha_positions"])
    result.add("opaque" if not alpha else "alpha")
    for position in alpha:
        result.add(("alpha-first", "alpha-middle", "alpha-last")[min(position, 2)])
    if record["coincident"]:
        result.add("coincident")
    if len(stops) == 2:
        result.add("two-stop")
    if len(stops) >= 3:
        result.add("three-stop")
    width = int(re.match(r"([0-9]+)", str(record["box"])).group(1))
    height_match = re.search(r"x([0-9]+)", str(record["box"]))
    height = int(height_match.group(1)) if height_match else width
    if width > height * 2:
        result.add("wide")
    return result


def load_allowlist(path: Path) -> set[str]:
    with path.open(newline="") as handle:
        reader = csv.DictReader(handle, delimiter="\t")
        if reader.fieldnames != ["id"]:
            raise ValueError(f"allowlist must contain only the id column, got {reader.fieldnames}")
        identities = [row["id"] for row in reader]
    if len(identities) != len(set(identities)):
        raise ValueError("allowlist contains duplicate identities")
    return set(identities)


def normative_scope(record: dict[str, object]) -> tuple[list[str], list[str]]:
    """Validate every semantic dimension against P1229 sections 4 and 8.

    Box aspect ratios are harness coverage, not a new Typst construction. They
    are recorded but deliberately excluded from the normative-language test.
    """
    clauses: list[str] = []
    violations: list[str] = []
    variant = str(record["variant"])
    space = str(record["space"])
    role = str(record["role"])
    stops = list(record["stops"])
    offsets = list(record["offsets"])
    geometry = dict(record["geometry"])

    if (variant, space) in PROMOTIONS or space == "srgb":
        clauses.append("variant-space")
    else:
        violations.append("variant-space")
    if role in {"fill", "stroke"}:
        clauses.append(role)
    else:
        violations.append("role")
    if len(stops) in {2, 3}:
        clauses.append(f"{len(stops)}-stop")
    else:
        violations.append("stop-count")

    numeric_offsets: list[float] = []
    try:
        numeric_offsets = [float(offset) for offset in offsets]
    except ValueError:
        violations.append("explicit-offsets")
    if numeric_offsets:
        if all(0.0 <= offset <= 100.0 for offset in numeric_offsets) and numeric_offsets == sorted(numeric_offsets):
            clauses.append("ordered-offsets")
        else:
            violations.append("offset-domain-or-order")
        if len(numeric_offsets) != len(set(numeric_offsets)):
            clauses.append("coincident")

    alpha_positions = list(record["alpha_positions"])
    if alpha_positions:
        if all(0 <= int(position) < len(stops) for position in alpha_positions):
            clauses.append("alpha-at-stop")
        else:
            violations.append("alpha-position")
    else:
        clauses.append("opaque")

    allowed_geometry = {"angle"} if variant == "linear" else {"center", "radius", "focal-center", "focal-radius"}
    unknown_geometry = sorted(set(geometry) - allowed_geometry)
    if unknown_geometry:
        violations.append("geometry:" + ",".join(unknown_geometry))
    else:
        clauses.append("declared-geometry")
    clauses.append("box-harness-only")
    return sorted(clauses), sorted(violations)


def write_tsv(path: Path, header: list[str], rows: list[list[object]]) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.writer(handle, delimiter="\t", lineterminator="\n")
        writer.writerow(header)
        writer.writerows(rows)


def opacity_aware_stops(oracle: object, svg: Path, kind: str) -> list[tuple[float, tuple[float, float, float, float]]]:
    tag = "linearGradient" if kind == "linear" else "radialGradient"
    elements = list(ET.parse(svg).getroot().iter())
    servers = [
        element
        for element in elements
        if oracle.local(element.tag) == tag
        and any(oracle.local(child.tag) == "stop" for child in element)
    ]
    if not servers:
        return []
    stops = []
    for child in servers[0]:
        if oracle.local(child.tag) != "stop":
            continue
        color = oracle.rgba(child.get("stop-color"))
        opacity = float(child.get("stop-opacity", "1"))
        stops.append((oracle.parse_offset(child.get("offset")), color[:3] + (color[3] * opacity,)))
    return stops


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--p1229-fixtures", type=Path, required=True)
    parser.add_argument("--p1231-fixtures", type=Path, required=True)
    parser.add_argument("--allowlist", type=Path, required=True)
    parser.add_argument("--oracle-script", type=Path, required=True)
    parser.add_argument("--svg-root", type=Path, required=True)
    parser.add_argument("--vanilla-budget", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)

    allowed_ids = load_allowlist(args.allowlist)
    if len(allowed_ids) != 30:
        raise ValueError(f"R01 universe must contain exactly 30 fixtures, got {len(allowed_ids)}")
    old = p1229_records(args.p1229_fixtures)
    new = p1231_records(args.p1231_fixtures, allowed_ids)
    parsed_ids = {Path(str(record["source"])).stem for record in new}
    if parsed_ids != allowed_ids:
        missing = sorted(allowed_ids - parsed_ids)
        extra = sorted(parsed_ids - allowed_ids)
        raise ValueError(f"fixture identity mismatch: missing={missing}, extra={extra}")
    old_signatures = {str(record["signature"]) for record in old}
    join_rows = []
    for record in new:
        fixture_id = Path(str(record["source"])).stem
        exact = str(record["signature"]) in old_signatures
        obligations, violations = normative_scope(record)
        classification = "normative-coverage-extension" if not violations else "scope-expansion-or-unknown"
        join_rows.append([fixture_id, record["variant"], record["space"], record["role"], record["box"], str(exact).lower(), ",".join(obligations), ",".join(violations) or "none", classification])

    write_tsv(args.out / "f01-f02-join.tsv", ["fixture_id", "variant", "space", "role", "box", "exact_tuple", "p1229_obligations", "scope_violations", "classification"], join_rows)

    spec = importlib.util.spec_from_file_location("p1233_oracle", args.oracle_script)
    if spec is None or spec.loader is None:
        raise ValueError("unable to load frozen oracle")
    oracle = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(oracle)
    oracle.server_stops = lambda svg, kind: opacity_aware_stops(oracle, Path(svg), kind)
    fixture_meta = {item["id"]: item for item in oracle.FIXES}
    with args.vanilla_budget.open(newline="") as handle:
        budget_rows = {row["id"]: row for row in csv.DictReader(handle, delimiter="\t")}
    mesh_rows = []
    mesh_failures = 0
    fixture_verdicts: dict[str, str] = {}
    for fixture_id in sorted(allowed_ids):
        meta = fixture_meta[fixture_id]
        svg = args.svg_root / f"{fixture_id}.svg"
        if not svg.is_file():
            raise ValueError(f"missing frozen SVG: {svg}")
        budget = budget_rows[fixture_id]
        metrics = {mesh: oracle.numeric(meta, svg, mesh) for mesh in (2048, 4096)}
        verdicts = []
        for mesh in (2048, 4096):
            value = metrics[mesh]
            passed = (
                value["color_max"] <= max(0.001, float(budget["V_color_max"])) + 0.000001
                and value["color_p95"] <= float(budget["V_color_p95"]) + 0.000001
                and value["alpha_max"] <= float(budget["V_alpha_max"]) + 0.000001
                and value["alpha_p95"] <= float(budget["V_alpha_p95"]) + 0.000001
            )
            verdicts.append(passed)
            mesh_rows.append([
                fixture_id,
                mesh,
                f'{value["color_max"]:.12f}',
                f'{value["color_p95"]:.12f}',
                f'{value["alpha_max"]:.12f}',
                f'{value["alpha_p95"]:.12f}',
                "pass" if passed else "fail",
                value["query_sha256"],
            ])
        if not all(verdicts):
            mesh_failures += 1
            fixture_verdicts[fixture_id] = "fail"
        else:
            fixture_verdicts[fixture_id] = "pass"
    write_tsv(
        args.out / "mesh-2048-4096.tsv",
        ["fixture_id", "mesh", "color_max", "color_p95", "alpha_max", "alpha_p95", "verdict", "vanilla_query_sha256"],
        mesh_rows,
    )
    normative_ids = {row[0] for row in join_rows if row[8] == "normative-coverage-extension"}
    result_rows = [
        [fixture_id, fixture_verdicts[fixture_id], "P1229_REOPENED" if fixture_id in normative_ids and fixture_verdicts[fixture_id] == "fail" else "control"]
        for fixture_id in sorted(allowed_ids)
    ]
    write_tsv(args.out / "results.tsv", ["fixture_id", "mesh_2048_4096", "adjudication"], result_rows)
    summary = {
        "p1229_records": len(old),
        "p1231_records": len(new),
        "exact_tuple_matches": sum(row[5] == "true" for row in join_rows),
        "normative_coverage_extensions": sum(row[8] == "normative-coverage-extension" for row in join_rows),
        "scope_expansions_or_unknown": sum(row[8] == "scope-expansion-or-unknown" for row in join_rows),
        "numeric_failures": mesh_failures,
        "numeric_passes": len(allowed_ids) - mesh_failures,
        "prior_verdict_fields_read": 0,
    }
    (args.out / "summary.json").write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
