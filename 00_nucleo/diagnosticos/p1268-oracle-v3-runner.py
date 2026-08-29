#!/usr/bin/env python3
"""P1268 V3 oracle reexecutor: vanilla-first, candidate-blind evidence.

Reads are restricted to the frozen P1268 allowlist. All writes stay below OUT.
No candidate binary, historical candidate result, attack output, or verifier output
is accepted as an argument or referenced by this program.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import os
import re
import shutil
import subprocess
import sys
import xml.etree.ElementTree as ET
from collections import Counter
from datetime import datetime
from pathlib import Path
from typing import Any

from PIL import Image


ROOT = Path("/repos/Antigravity/typst-crystalline")
OUT = Path("/tmp/p1268/oracle_v3")
VANILLA = Path("/usr/local/bin/typst")
MATRIX = ROOT / "00_nucleo/diagnosticos/p1266-generalization-matrix.tsv"
CORPUS = ROOT / "00_nucleo/diagnosticos/p1266-corpus.tsv"
FIXTURE_ROOT = ROOT / "lab/parity/matrix/fixtures/p1266"
HISTORICAL_BUDGETS = ROOT / "00_nucleo/diagnosticos/p1266-oracle-budgets.tsv"
CONTRACT_FREEZE = ROOT / "00_nucleo/diagnosticos/p1268-contract-freeze.tsv"
CONTRACT = Path("/tmp/p1268/refiner/contract-v3.tsv")
UNKNOWN_POLICY = Path("/tmp/p1268/refiner/unknown-v3.tsv")
OBSERVABLES = Path("/tmp/p1268/refiner/observables-v3.tsv")
PREORACLE = ROOT / "00_nucleo/diagnosticos/p1268-preoracle-manifest-v3.tsv"
STEP = ROOT / "typst-passo-1268.md"
ROLE_CAPABILITIES = ROOT / "00_nucleo/diagnosticos/p1268-role-capabilities-v3.tsv"
TEMPLATE_SOURCE = Path("/tmp/p1268/oracle_clean/runner.py")
SKILL = Path("/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md")
SKILL_ROLES = Path("/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/papeis-e-capacidades.md")
SKILL_GATES = Path("/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/artefatos-e-gates.md")
ADR_ROOT = ROOT / "00_nucleo/adr"
VANILLA_SOURCE = ROOT / "lab/typst-original/crates/typst-library/src/visualize/gradient.rs"

PAIRS = ("linear/oklab", "radial/oklab", "linear/linear-rgb", "radial/linear-rgb")
METRICS = ("color_max", "color_p95", "alpha_max", "alpha_p95")
MESH_N = 8192
DYADIC_N = 128
EXPECTED = {
    PREORACLE: "6ed7f67f11d4d886edef3c39bf327cb7ae96e3b2bdd17517fe90b3145f3fa76c",
    ROLE_CAPABILITIES: "bdb8b2ed03f9f83655c3505c73e53b85141a935f22c8238c76ec3fade5f6f876",
    CONTRACT_FREEZE: "7d8064a939e653961a084088e40353342de401908e7a8dcd0ba229e07489579d",
    CONTRACT: "cb2aabaf36703bef3ef869f9a150ca8f38f5e596f1a8523c19016a283b024399",
    UNKNOWN_POLICY: "2ab6446d8b50c803240f19bde92ff55e121bc0f4a9793f8faeaecd80979b5530",
    OBSERVABLES: "e32a8b201a6625156e0416303892c23ce441957e09ecbde0bb3f459d966e969c",
    VANILLA: "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8",
    VANILLA_SOURCE: "ed7a1be4e398fa47dff9ede8aa31c939514e341f9e47ac5099787d422f67967c",
    MATRIX: "091d50354eb2b206d595c88a96b76eb8b9d8e8a881b2f0d3f5901edf9cce5406",
    CORPUS: "54b5f0fdb0153321544a9d0e147867ea0473ecd9cedfc6d959e3c0a1c9522894",
}
HISTORICAL_BUDGETS_SHA = "b8153842849d7a5f74d1af0a006407d5e3ebebc1ece44c1456379ae396686b46"

PRIOR_METADATA_READS = [
    "rg --files ... | rg P1268/ADR path discovery [filenames only]",
    "wc -l [authorized frozen inputs/template/source] [line-count metadata]",
    "git rev-parse HEAD && git diff HEAD --stat && git status --short [repository metadata only]",
]
PRIOR_ACTIONS = [
    "mkdir -p /tmp/p1268/oracle_v3",
    "apply_patch Add File /tmp/p1268/oracle_v3/runner.py from sha256:522611321055458b5a0be3fc4462f0fd1dfa27bc3e8fe00decf77b5a4d423145 with V3-only mechanical substitutions",
    "syntax/diff/output audit; mechanical receipt wording correction; clean regeneration of owned V3 outputs",
]
PRIOR_CONTENT_READS = [
    SKILL, SKILL_ROLES, SKILL_GATES, TEMPLATE_SOURCE, STEP, PREORACLE,
    ROLE_CAPABILITIES, CONTRACT_FREEZE, CONTRACT, UNKNOWN_POLICY, OBSERVABLES,
    MATRIX, CORPUS, HISTORICAL_BUDGETS, VANILLA, VANILLA_SOURCE,
] + sorted(ADR_ROOT.glob("*.md"))

READS: list[dict[str, str]] = []
COMMANDS: list[dict[str, str]] = []


def sha(path: Path, *, purpose: str = "") -> str:
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    READS.append({"path": str(path), "sha256": digest, "purpose": purpose})
    return digest


def sha_unlogged(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sha_text(value: str) -> str:
    return hashlib.sha256(value.encode()).hexdigest()


def read_tsv(path: Path, *, purpose: str) -> list[dict[str, str]]:
    digest = sha(path, purpose=purpose)
    with path.open(newline="") as handle:
        rows = list(csv.DictReader(handle, delimiter="\t"))
    if not rows:
        raise ValueError(f"empty TSV: {path} ({digest})")
    return rows


def assert_write_path(path: Path) -> None:
    try:
        path.resolve().relative_to(OUT.resolve())
    except ValueError as error:
        raise RuntimeError(f"write outside frozen root blocked: {path}") from error


def write_tsv(path: Path, rows: list[dict[str, Any]], fieldnames: list[str] | None = None) -> None:
    assert_write_path(path)
    if not rows:
        raise ValueError(f"refusing empty output: {path}")
    path.parent.mkdir(parents=True, exist_ok=True)
    names = fieldnames or list(rows[0])
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, names, delimiter="\t", lineterminator="\n", extrasaction="ignore")
        writer.writeheader()
        writer.writerows(rows)


def run(command: list[str], *, check: bool = True) -> subprocess.CompletedProcess[str]:
    forbidden = ("target/debug/typst", "/tmp/p1268/oracle/", "/tmp/p1268/oracle_clean/", "/tmp/p1268/oracle_v2/", "/adversary/", "/verifier/", "/candidate/")
    joined = " ".join(command)
    if any(token in joined for token in forbidden):
        raise RuntimeError(f"forbidden command target: {joined}")
    for token in command:
        if token.startswith("/tmp/p1268/"):
            assert_write_path(Path(token))
    if command and command[0].endswith("typst") and Path(command[0]) != VANILLA:
        raise RuntimeError(f"non-vanilla Typst execution blocked: {command[0]}")
    completed = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    COMMANDS.append({
        "command": joined,
        "exit": str(completed.returncode),
        "stdout_sha256": sha_text(completed.stdout),
        "stderr_sha256": sha_text(completed.stderr),
    })
    if check and completed.returncode:
        raise RuntimeError(
            f"command failed ({completed.returncode}): {joined}\n{completed.stdout}\n{completed.stderr}"
        )
    return completed


def eval_json(code: str) -> Any:
    return json.loads(run([str(VANILLA), "eval", "--format", "json", code]).stdout)


def local(tag: str) -> str:
    return tag.rsplit("}", 1)[-1]


def fmt(value: float) -> str:
    if abs(value) < 1e-15:
        value = 0.0
    return format(value, ".17g")


def percentile(values: list[float], portion: float) -> float:
    ordered = sorted(values)
    return ordered[max(0, math.ceil(portion * len(ordered)) - 1)]


def encoded_srgb(components: list[float], space: str) -> tuple[float, float, float, float]:
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

    rgb = tuple(max(0.0, min(1.0, gamma(value))) for value in linear)
    return rgb + (max(0.0, min(1.0, alpha)),)


def parse_color(value: str) -> tuple[float, float, float, float]:
    match = re.fullmatch(r"#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})", value)
    if not match:
        raise ValueError(f"unsupported SVG color: {value}")
    raw = match.group(1) + ("ff" if len(match.group(1)) == 6 else "")
    return tuple(int(raw[index:index + 2], 16) / 255 for index in range(0, 8, 2))


def server(svg: Path, kind: str) -> ET.Element:
    servers = [
        element for element in ET.parse(svg).getroot().iter()
        if local(element.tag) == f"{kind}Gradient"
        and any(local(child.tag) == "stop" for child in element)
    ]
    if len(servers) != 1:
        raise ValueError(f"expected one {kind}Gradient with stops in {svg}, got {len(servers)}")
    return servers[0]


def server_stops(svg: Path, kind: str) -> list[tuple[float, tuple[float, float, float, float]]]:
    output = []
    for child in server(svg, kind):
        if local(child.tag) != "stop":
            continue
        raw_offset = child.get("offset", "")
        offset = float(raw_offset[:-1]) / 100 if raw_offset.endswith("%") else float(raw_offset)
        color = parse_color(child.get("stop-color", ""))
        opacity = float(child.get("stop-opacity", "1"))
        output.append((offset, color[:3] + (color[3] * opacity,)))
    return output


def approximate(stops: list[tuple[float, tuple[float, float, float, float]]], t: float) -> tuple[float, float, float, float]:
    if t >= 1.0:
        return stops[-1][1]
    index = 0
    while index + 1 < len(stops) and stops[index + 1][0] <= t:
        index += 1
    if index + 1 == len(stops) or stops[index + 1][0] <= stops[index][0]:
        return stops[index][1]
    amount = (t - stops[index][0]) / (stops[index + 1][0] - stops[index][0])
    return tuple(stops[index][1][channel] * (1 - amount) + stops[index + 1][1][channel] * amount for channel in range(4))


def numeric_metrics(samples: list[list[float]], space: str, stops: list[tuple[float, tuple[float, float, float, float]]]) -> dict[str, float]:
    color_errors: list[float] = []
    alpha_errors: list[float] = []
    for index, components in enumerate(samples):
        exact = encoded_srgb(components, space)
        observed = approximate(stops, index / MESH_N)
        color_errors.append(math.sqrt(sum(
            (exact[channel] * exact[3] - observed[channel] * observed[3]) ** 2
            for channel in range(3)
        )))
        alpha_errors.append(abs(exact[3] - observed[3]))
    return {
        "color_max": max(color_errors),
        "color_p95": percentile(color_errors, 0.95),
        "alpha_max": max(alpha_errors),
        "alpha_p95": percentile(alpha_errors, 0.95),
    }


def extract_expression(source: str) -> str:
    match = re.search(r"(?m)^#let g = (.+)$", source)
    if not match:
        raise ValueError("fixture does not contain one-line #let g expression")
    return match.group(1)


def public_meta(expression: str) -> list[Any]:
    norm = "x => if type(x) == ratio { x / 100% } else { x }"
    metadata = (
        "(g.stops().map(s => (s.at(0).components().map(norm), s.at(1) / 100%)), "
        "repr(g.kind()), repr(g.space()), repr(g.angle()), repr(g.center()), "
        "repr(g.radius()), repr(g.focal-center()), repr(g.focal-radius()))"
    )
    return eval_json(f"let norm = {norm}; let g = {expression}; {metadata}")


def sample_observation(expression: str, probe_positions: list[float]) -> tuple[list[list[float]], list[list[list[float]]], list[list[float]]]:
    norm = "x => if type(x) == ratio { x / 100% } else { x }"
    probes = ",".join(f"{fmt(value * 100)}%" for value in probe_positions)
    code = (
        f"let norm = {norm}; let g = {expression}; let ss = g.stops(); "
        f"let mesh = range({MESH_N + 1}).map(i => g.sample(i / {MESH_N} * 100%).components().map(norm)); "
        f"let dyadic = range(ss.len() - 1).map(i => {{ let a = ss.at(i).at(1); let b = ss.at(i + 1).at(1); "
        f"range({DYADIC_N + 1}).map(k => g.sample(a + (b - a) * k / {DYADIC_N}).components().map(norm)) }}); "
        f"let probes = ({probes},).map(t => g.sample(t).components().map(norm)); (mesh, dyadic, probes)"
    )
    mesh, dyadic, probe_values = eval_json(code)
    return mesh, dyadic, probe_values


def trailing_zeros(value: int) -> int:
    return (value & -value).bit_length() - 1


def simulate_interval(
    samples: list[list[float]], space: str, left: float, right: float,
    owner_left: list[float], owner_right: list[float],
) -> dict[str, Any]:
    # The ratified iterator initializes from the two owning GradientStops.
    # At a coincidence these deliberately differ from self.sample(endpoint).
    prev_color = encoded_srgb(owner_left, space)
    next_color = encoded_srgb(owner_right, space)
    next_offset = right
    n = 1
    i = 1
    max_n = 1
    decisions = 0
    boundary_calls = 0
    adaptive: list[dict[str, Any]] = []
    while True:
        mid_index = round(((i - 0.5) / n) * DYADIC_N)
        exact = encoded_srgb(samples[mid_index], space)
        approx = tuple((prev_color[channel] + next_color[channel]) * 0.5 for channel in range(4))
        error = math.sqrt(sum(
            (exact[channel] * exact[3] - approx[channel] * approx[3]) ** 2
            for channel in range(3)
        ))
        decisions += 1
        if n < 64 and error > 0.001:
            n *= 2
            i = 2 * i - 1
            max_n = max(max_n, n)
        elif i >= n:
            break
        else:
            adaptive.append({"offset": next_offset, "color": next_color})
            prev_color = next_color
            shift = trailing_zeros(i)
            n >>= shift
            i >>= shift
            i += 1
        next_index = round((i / n) * DYADIC_N)
        next_offset = left + (right - left) * i / n
        next_color = encoded_srgb(samples[next_index], space)
        boundary_calls += 1
    return {
        "subdivisions": max_n,
        "adaptive": adaptive,
        "a_i": len(adaptive),
        "d_i": decisions,
        "b_i": boundary_calls,
        "q_i": decisions + boundary_calls,
    }


def graph_observation(svg: Path, kind: str, expected_role: str) -> dict[str, Any]:
    root = ET.parse(svg).getroot()
    elements = list(root.iter())
    id_map = {element.get("id"): element for element in elements if element.get("id")}
    ids = set(id_map)
    unresolved: list[str] = []
    role_refs: list[tuple[str, str]] = []
    for element in elements:
        for key, value in element.attrib.items():
            refs = re.findall(r"url\(#([^\)]+)\)", value)
            unresolved.extend(ref for ref in refs if ref not in ids)
            if local(key) == "href" and value.startswith("#") and value[1:] not in ids:
                unresolved.append(value[1:])
            if local(key) in ("fill", "stroke"):
                role_refs.extend((local(key), ref) for ref in refs)
    gradient = server(svg, kind)
    gradient_id = gradient.get("id", "")

    def resolves_to_server(reference: str) -> bool:
        seen: set[str] = set()
        while reference and reference not in seen:
            seen.add(reference)
            node = id_map.get(reference)
            if node is None:
                return False
            if reference == gradient_id:
                return True
            href = next((value for key, value in node.attrib.items() if local(key) == "href"), "")
            reference = href[1:] if href.startswith("#") else ""
        return False

    chain_nodes = []
    for role, reference in role_refs:
        if role != expected_role:
            continue
        current = reference
        seen: set[str] = set()
        while current and current not in seen and current in id_map:
            seen.add(current)
            node = id_map[current]
            chain_nodes.append((local(node.tag), sorted((local(k), v) for k, v in node.attrib.items() if local(k) not in ("id", "href"))))
            href = next((value for key, value in node.attrib.items() if local(key) == "href"), "")
            current = href[1:] if href.startswith("#") else ""
    normalized_transforms = sorted(
        (local(element.tag), element.get("transform", ""))
        for element in elements if element.get("transform")
    )
    geometry = sorted(chain_nodes)
    return {
        "variant_nodes": sum(local(element.tag) == f"{kind}Gradient" for element in elements),
        "unresolved": len(unresolved),
        "raster_nodes": sum(local(element.tag) == "image" for element in elements),
        "role_urls": len(role_refs),
        "expected_role_urls": sum(role == expected_role and resolves_to_server(ref) for role, ref in role_refs),
        "geometry_json": json.dumps(geometry, separators=(",", ":")),
        "transform_digest": sha_text(json.dumps(normalized_transforms, separators=(",", ":"))),
        "graph_closed": not unresolved and bool(gradient_id) and any(resolves_to_server(ref) for _, ref in role_refs),
    }


def semantic_svg(svg: Path, kind: str, expected_role: str) -> dict[str, Any]:
    graph = graph_observation(svg, kind, expected_role)
    stops = server_stops(svg, kind)
    return {
        "variant_nodes": graph["variant_nodes"],
        "unresolved": graph["unresolved"],
        "raster_nodes": graph["raster_nodes"],
        "role_urls": graph["role_urls"],
        "expected_role_urls": graph["expected_role_urls"],
        "geometry_json": graph["geometry_json"],
        "transform_digest": graph["transform_digest"],
        "stops": [[fmt(offset), *[fmt(v) for v in color]] for offset, color in stops],
    }


def solid_mask_svg(source: Path, destination: Path) -> None:
    tree = ET.parse(source)
    gradient_ids = {
        element.get("id") for element in tree.getroot().iter()
        if local(element.tag) in ("linearGradient", "radialGradient") and element.get("id")
    }
    for element in tree.getroot().iter():
        for key, value in list(element.attrib.items()):
            match = re.fullmatch(r"url\(#([^\)]+)\)", value)
            if match and match.group(1) in gradient_ids:
                element.set(key, "#ffffff")
    tree.write(destination, encoding="unicode")


def mask_stats(path: Path) -> dict[str, Any]:
    image = Image.open(path).convert("RGBA")
    alpha = image.getchannel("A")
    bbox = alpha.getbbox()
    values = list(alpha.getdata())
    positive = sum(value > 0 for value in values)
    return {
        "width": image.width,
        "height": image.height,
        "positive_pixels": positive,
        "bbox": "none" if bbox is None else ",".join(map(str, bbox)),
        "alpha_digest": hashlib.sha256(bytes(values)).hexdigest(),
    }


def canonical(value: Any) -> str:
    return sha_text(json.dumps(value, sort_keys=True, separators=(",", ":")))


def find_coincident_groups(stops: list[Any]) -> list[tuple[int, int, float]]:
    groups: list[tuple[int, int, float]] = []
    index = 0
    while index < len(stops):
        end = index + 1
        while end < len(stops) and math.isclose(float(stops[end][1]), float(stops[index][1]), abs_tol=1e-15):
            end += 1
        if end - index > 1:
            groups.append((index, end - 1, float(stops[index][1])))
        index = end
    return groups


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", type=Path, default=OUT)
    args = parser.parse_args()
    if args.out.resolve() != OUT:
        raise ValueError("write root is frozen to /tmp/p1268/oracle_v3")
    OUT.mkdir(parents=True, exist_ok=True)
    unexpected = [path for path in OUT.iterdir() if path.name != "runner.py"]
    if unexpected:
        raise ValueError(f"clean restart output is not empty: {[str(p) for p in unexpected]}")

    measured_at = datetime.now().astimezone().isoformat(timespec="seconds")
    if shutil.which("rsvg-convert") is None:
        raise RuntimeError("rsvg-convert unavailable; masks would be Unknown")

    # Record all pre-run content reads and distinguish them from metadata-only discovery.
    for path in PRIOR_CONTENT_READS:
        sha(path, purpose="content:authorized-pre-run-read")

    repository_head = run(["git", "rev-parse", "HEAD"]).stdout.strip()
    repository_status = run(["git", "status", "--short"]).stdout

    # Validate every frozen input before materializing the oracle.
    for path, expected in EXPECTED.items():
        observed = sha(path, purpose="frozen-input-validation")
        if observed != expected:
            raise ValueError(f"frozen input drift: {path}: {observed} != {expected}")

    matrix = read_tsv(MATRIX, purpose="population-shape")
    corpus = read_tsv(CORPUS, purpose="96-fixture-identity")
    contract = read_tsv(CONTRACT, purpose="oracle-obligations")
    unknown_policy = read_tsv(UNKNOWN_POLICY, purpose="honest-Unknown-policy")
    observables = read_tsv(OBSERVABLES, purpose="required-observables")
    read_tsv(CONTRACT_FREEZE, purpose="contract-freeze")
    read_tsv(PREORACLE, purpose="preoracle-manifest")
    if len([row for row in matrix if row["validity"] == "valid"]) != 24:
        raise ValueError("matrix does not contain 24 valid seeds")
    if len(corpus) != 96 or Counter(row["pair"] for row in corpus) != Counter({pair: 24 for pair in PAIRS}):
        raise ValueError("corpus is not exactly 96 fixtures / 24 per pair")
    if len(contract) != 17 or len(observables) != 13:
        raise ValueError("contract/observable cardinality drift")

    matrix_by_seed = {row["seed"]: row for row in matrix if row["validity"] == "valid"}
    sources_rows: list[dict[str, Any]] = []
    fixture_data: list[dict[str, Any]] = []
    for row in corpus:
        source = FIXTURE_ROOT / f"{row['fixture_id']}.typ"
        digest = sha(source, purpose="pinned-fixture-source")
        if digest != row["source_sha256"]:
            raise ValueError(f"fixture hash drift: {source}")
        text = source.read_text()
        expression = extract_expression(text)
        expression_digest = sha_text(expression)
        if expression_digest != row["expression_sha256"]:
            raise ValueError(f"fixture expression drift: {source}")
        sources_rows.append({
            "fixture_id": row["fixture_id"], "pair": row["pair"], "seed": row["seed"],
            "source_path": str(source), "source_sha256": digest,
            "expression_sha256": expression_digest, "identity": "Frozen",
        })
        fixture_data.append({**row, "source_path": source, "expression": expression, "seed_row": matrix_by_seed[row["seed"]]})
    write_tsv(OUT / "sources.tsv", sources_rows)

    binaries = OUT / "binaries"
    masks = OUT / "masks"
    for phase in ("direct", "inverse", "repeat"):
        (binaries / phase).mkdir(parents=True, exist_ok=True)
    masks.mkdir(parents=True, exist_ok=True)

    budgets_rows: list[dict[str, Any]] = []
    interval_rows: list[dict[str, Any]] = []
    mask_rows: list[dict[str, Any]] = []
    landmark_rows: list[dict[str, Any]] = []
    coincidence_rows: list[dict[str, Any]] = []
    degenerate_rows: list[dict[str, Any]] = []
    positive_rows: list[dict[str, Any]] = []
    direct_semantic: dict[str, str] = {}

    # Direct pass freezes every vanilla budget and mask before the historical
    # budget reference is opened below.
    for fixture in fixture_data:
        fixture_id = fixture["fixture_id"]
        pair = fixture["pair"]
        kind, space = pair.split("/")
        direct_svg = binaries / "direct" / f"{fixture_id}.svg"
        run([str(VANILLA), "compile", str(fixture["source_path"]), str(direct_svg)])
        meta = public_meta(fixture["expression"])
        public_stops = meta[0]
        M = len(public_stops)
        coincidence_groups = find_coincident_groups(public_stops)
        probe_positions: list[float] = []
        probe_plan: list[dict[str, Any]] = []
        for start, end, offset in coincidence_groups:
            probe_positions.append(offset)
            exact_probe = len(probe_positions) - 1
            epsilon_probe: int | None = None
            if offset < 1.0:
                next_greater = next(float(stop[1]) for stop in public_stops[end + 1:] if float(stop[1]) > offset)
                epsilon = offset + min(next_greater - offset, 1.0 - offset) / (2**20)
                probe_positions.append(epsilon)
                epsilon_probe = len(probe_positions) - 1
            probe_plan.append({
                "start": start, "end": end, "offset": offset,
                "exact_probe": exact_probe, "epsilon_probe": epsilon_probe,
                "epsilon": None if epsilon_probe is None else probe_positions[epsilon_probe],
            })
        if not probe_positions:
            probe_positions = [0.0]
        mesh, dyadic, probe_values = sample_observation(fixture["expression"], probe_positions)
        observed_stops = server_stops(direct_svg, kind)
        metrics = numeric_metrics(mesh, space, observed_stops)

        eligible = str(fixture["anti_alias"]).lower() == "true"
        R = 0
        adaptive_total = 0
        expected_offsets = [float(public_stops[0][1])]
        trace_known = True
        for index in range(M - 1):
            left = float(public_stops[index][1])
            right = float(public_stops[index + 1][1])
            interval_eligible = eligible and right > left
            if interval_eligible:
                R += 1
                trace = simulate_interval(
                    dyadic[index], space, left, right,
                    public_stops[index][0], public_stops[index + 1][0],
                )
                adaptive_total += trace["a_i"]
                expected_offsets.extend(item["offset"] for item in trace["adaptive"])
                expected_offsets.append(right)
            else:
                trace = {"subdivisions": 0, "a_i": 0, "d_i": 0, "b_i": 0, "q_i": 0, "adaptive": []}
                expected_offsets.append(right)
            bounds_ok = (
                trace["subdivisions"] <= 64 and trace["a_i"] <= 63
                and trace["d_i"] <= 127 and trace["b_i"] <= 126 and trace["q_i"] <= 253
            )
            interval_rows.append({
                "fixture_id": fixture_id, "pair": pair, "interval": index,
                "left": fmt(left), "right": fmt(right), "positive_width": str(right > left).lower(),
                "eligible": str(interval_eligible).lower(), "s_i": trace["subdivisions"],
                "a_i": trace["a_i"], "midpoint_decisions_d_i": trace["d_i"],
                "boundary_sampler_calls_b_i": trace["b_i"], "total_sampler_calls_q_i": trace["q_i"],
                "bounds_64_63_127_126_253": "Preserved" if bounds_ok else "Violated",
                "classification": "Measured" if interval_eligible else "Ineligible",
            })

        E = len(observed_stops)
        expected_E = M + adaptive_total
        offset_match = len(expected_offsets) == E and all(
            # Vanilla writes offsets to 0.01 percentage point, i.e. an
            # absolute ratio quantization of at most 0.00005.
            abs(left - right) <= 5.0000001e-5 for left, right in zip(expected_offsets, (stop[0] for stop in observed_stops))
        )
        if not offset_match:
            trace_known = False
            for row in interval_rows:
                if row["fixture_id"] == fixture_id and row["eligible"] == "true":
                    row["classification"] = "Unknown"
        affine_ok = E == expected_E and E <= M + 63 * R and (M < 2 or E <= 64 * M - 63)
        bounds_ok = all(
            row["bounds_64_63_127_126_253"] == "Preserved"
            for row in interval_rows if row["fixture_id"] == fixture_id
        )
        budgets_rows.append({
            "fixture_id": fixture_id, "pair": pair, "mesh": MESH_N,
            **{f"V_{metric}": fmt(metrics[metric]) for metric in METRICS},
            "M_effective_public_stops": M, "R_eligible_intervals": R,
            "E_emitted_stops": E, "expected_E_M_plus_sum_a": expected_E,
            "affine_bound_M_plus_63R": M + 63 * R,
            "global_bound_64M_minus_63": 64 * M - 63 if M >= 2 else "n/a",
            "identity_and_bounds": "Measured" if trace_known and affine_ok and bounds_ok else "Unknown",
            "source_sha256": fixture["source_sha256"], "vanilla_svg_sha256": sha_unlogged(direct_svg),
            "classification": "VanillaBaselineFrozen",
        })

        graph = graph_observation(direct_svg, kind, fixture["role"])
        landmark_status = (
            graph["variant_nodes"] >= 1 and graph["unresolved"] == 0 and graph["raster_nodes"] == 0
            and graph["expected_role_urls"] >= 1 and graph["graph_closed"]
        )
        landmark_rows.append({
            "fixture_id": fixture_id, "pair": pair, "seed": fixture["seed"],
            "variant_nodes": graph["variant_nodes"], "unresolved_urls": graph["unresolved"],
            "raster_nodes": graph["raster_nodes"], "role": fixture["role"],
            "role_urls": graph["role_urls"], "expected_role_urls": graph["expected_role_urls"],
            "geometry_json": graph["geometry_json"], "transform_digest": graph["transform_digest"],
            "graph_closed": str(graph["graph_closed"]).lower(),
            "status": "Measured" if landmark_status else "Unknown",
        })

        mask_svg = masks / f"{fixture_id}.svg"
        mask_png = masks / f"{fixture_id}.png"
        solid_mask_svg(direct_svg, mask_svg)
        run(["rsvg-convert", "--zoom", "1", "-o", str(mask_png), str(mask_svg)])
        stats = mask_stats(mask_png)
        mask_rows.append({
            "fixture_id": fixture_id, "pair": pair, "seed": fixture["seed"],
            "mask_svg": str(mask_svg), "mask_svg_sha256": sha_unlogged(mask_svg),
            "mask_binary": str(mask_png), "mask_binary_sha256": sha_unlogged(mask_png),
            **stats, "derivation": "vanilla-graph-solid-paint", "status": "Measured",
        })

        if fixture["seed"] in ("S20", "S21"):
            expect_positive = fixture["seed"] == "S20"
            morphology_ok = (stats["positive_pixels"] > 0) == expect_positive
            degenerate_rows.append({
                "fixture_id": fixture_id, "pair": pair, "seed": fixture["seed"],
                "role": fixture["role"], "box": fixture["box"],
                "expected_support": "positive-stroke" if expect_positive else "zero-fill",
                "positive_pixels": stats["positive_pixels"], "bbox": stats["bbox"],
                "alpha_digest": stats["alpha_digest"], "graph_closed": str(graph["graph_closed"]).lower(),
                "status": "Measured" if morphology_ok and graph["graph_closed"] else "Unknown",
            })

        for plan in probe_plan:
            start, end, offset = plan["start"], plan["end"], plan["offset"]
            exact_value = probe_values[plan["exact_probe"]]
            expected_exact_index = end if math.isclose(offset, 0.0, abs_tol=1e-15) else start
            exact_expected = public_stops[expected_exact_index][0]
            exact_competing = public_stops[start][0] if expected_exact_index == end else public_stops[end][0]
            exact_discriminating = any(abs(float(a) - float(b)) > 1e-9 for a, b in zip(exact_expected, exact_competing))
            exact_delta = max(abs(float(a) - float(b)) for a, b in zip(exact_value, exact_expected))
            epsilon_value: list[float] | None = None
            epsilon_last_delta: float | None = None
            epsilon_first_delta: float | None = None
            epsilon_ok: bool | None = None
            epsilon_discriminating: bool | None = None
            if plan["epsilon_probe"] is not None:
                epsilon_value = probe_values[plan["epsilon_probe"]]
                epsilon_last_delta = max(abs(float(a) - float(b)) for a, b in zip(epsilon_value, public_stops[end][0]))
                epsilon_first_delta = max(abs(float(a) - float(b)) for a, b in zip(epsilon_value, public_stops[start][0]))
                epsilon_discriminating = any(
                    abs(float(a) - float(b)) > 1e-9
                    for a, b in zip(public_stops[end][0], public_stops[start][0])
                )
                epsilon_ok = (not epsilon_discriminating) or epsilon_last_delta < epsilon_first_delta
            coincidence_rows.append({
                "fixture_id": fixture_id, "pair": pair, "seed": fixture["seed"],
                "offset": fmt(offset), "first_index": start, "last_index": end,
                "multiplicity": end - start + 1, "ordered_components_json": json.dumps([stop[0] for stop in public_stops[start:end + 1]], separators=(",", ":")),
                "exact_expected_index": expected_exact_index, "exact_observed_json": json.dumps(exact_value, separators=(",", ":")),
                "exact_max_delta": fmt(exact_delta),
                "exact_runtime_discriminating": str(exact_discriminating).lower(),
                "epsilon": "n/a" if plan["epsilon"] is None else fmt(plan["epsilon"]),
                "epsilon_expected_last_index": "n/a" if plan["epsilon"] is None else end,
                "epsilon_observed_json": "n/a" if epsilon_value is None else json.dumps(epsilon_value, separators=(",", ":")),
                "epsilon_delta_to_last": "n/a" if epsilon_last_delta is None else fmt(epsilon_last_delta),
                "epsilon_delta_to_first": "n/a" if epsilon_first_delta is None else fmt(epsilon_first_delta),
                "epsilon_runtime_discriminating": "n/a" if epsilon_discriminating is None else str(epsilon_discriminating).lower(),
                "exact_point_status": "MeasuredStructural" if exact_delta <= 1e-6 else "Unknown",
                "epsilon_right_status": "NotApplicableTerminal" if epsilon_ok is None else ("MeasuredStructural" if epsilon_ok else "Unknown"),
                "structural_basis": "ratified-gradient.rs:sample_stops partition rule plus ordered effective stops",
            })

        semantic = canonical({"meta": meta, "svg": semantic_svg(direct_svg, kind, fixture["role"])})
        direct_semantic[fixture_id] = semantic
        required_known = trace_known and affine_ok and bounds_ok and landmark_status
        if fixture["seed"] in ("S20", "S21"):
            required_known = required_known and degenerate_rows[-1]["status"] == "Measured"
        positive_rows.append({
            "fixture_id": fixture_id, "pair": pair, "seed": fixture["seed"],
            "expected_candidate_classification": "Preserved",
            "vanilla_public_observable": "Measured", "budget": "Frozen",
            "graph_geometry": "Measured" if landmark_status else "Unknown",
            "interval_cost": "Measured" if trace_known and affine_ok and bounds_ok else "Unknown",
            "painted_support": "Measured",
            "oracle_status": "Available" if required_known else "Unknown",
            "unknown_reason": "" if required_known else "one or more independently required observables could not be adjudicated",
        })

    # Freeze new budgets before historical identity comparison.
    write_tsv(OUT / "budgets.tsv", budgets_rows)
    write_tsv(OUT / "interval-cost.tsv", interval_rows)
    write_tsv(masks / "manifest.tsv", mask_rows)
    write_tsv(OUT / "landmarks.tsv", landmark_rows)
    write_tsv(OUT / "coincidence.tsv", coincidence_rows)
    write_tsv(OUT / "degenerates.tsv", degenerate_rows)
    write_tsv(OUT / "positive.tsv", positive_rows)
    new_budget_digest_before_history = sha_unlogged(OUT / "budgets.tsv")

    # The historical oracle is opened only here, after the new vanilla-first
    # budgets are immutable for this run. Its identity never affects a budget,
    # interval classification, or positive-oracle status.
    observed_history_sha = sha(HISTORICAL_BUDGETS, purpose="post-freeze-identity-comparison-only")
    if observed_history_sha != HISTORICAL_BUDGETS_SHA:
        raise ValueError("historical budget identity drift")
    historical_rows = read_tsv(HISTORICAL_BUDGETS, purpose="post-freeze-value-identity-comparison-only")
    historical_by_id = {row["fixture_id"]: row for row in historical_rows}
    comparable_columns = [f"V_{metric}" for metric in METRICS]
    historical_identity_matches = 0
    historical_mismatch_ids: list[str] = []
    for row in budgets_rows:
        old = historical_by_id.get(row["fixture_id"])
        if old and all(float(row[column]) == float(old[column]) for column in comparable_columns):
            historical_identity_matches += 1
        else:
            historical_mismatch_ids.append(row["fixture_id"])
    if sha_unlogged(OUT / "budgets.tsv") != new_budget_digest_before_history:
        raise RuntimeError("historical comparison changed new budgets")

    determinism_rows: list[dict[str, Any]] = []
    for phase, ordered in (("inverse", list(reversed(fixture_data))), ("repeat", fixture_data)):
        for fixture in ordered:
            fixture_id = fixture["fixture_id"]
            kind = fixture["pair"].split("/")[0]
            svg = binaries / phase / f"{fixture_id}.svg"
            run([str(VANILLA), "compile", str(fixture["source_path"]), str(svg)])
            meta = public_meta(fixture["expression"])
            observed = canonical({"meta": meta, "svg": semantic_svg(svg, kind, fixture["role"])})
            determinism_rows.append({
                "phase": phase, "fixture_id": fixture_id, "pair": fixture["pair"],
                "direct_semantic_digest": direct_semantic[fixture_id],
                "observed_semantic_digest": observed,
                "status": "MeasuredEqual" if observed == direct_semantic[fixture_id] else "Violated",
            })
    write_tsv(OUT / "determinism.tsv", determinism_rows)

    opaque_rows: list[dict[str, Any]] = []
    for row in unknown_policy:
        opaque_rows.append({
            "case": row["case"], "observable": row["observable"], "population": row["population"],
            "trigger": row["unknown"], "expected_classification": row["classification"],
            "counts_as_success": row["counts_as_success"], "mutation_numerator": row["mutation_numerator"],
            "effect": row["effect"], "resolution_required": row["resolution_required"],
            "oracle_expectation": "Unknown" if "Unknown" in row["classification"] or row["classification"] in ("NOT-COUNTED", "Not-a-violation") else row["classification"],
            "status": "FrozenOpaqueCase",
        })
    write_tsv(OUT / "opaque.tsv", opaque_rows)

    if len(sources_rows) != 96 or len(budgets_rows) != 96 or len(mask_rows) != 96 or len(landmark_rows) != 96 or len(positive_rows) != 96:
        raise RuntimeError("required 96-row oracle artifact missing")
    if len(degenerate_rows) != 8:
        raise RuntimeError("S20/S21 must contribute eight pair-local rows")
    if len(determinism_rows) != 192:
        raise RuntimeError("inverse/repeat determinism must contain 192 comparisons")

    eligible_intervals = [row for row in interval_rows if row["eligible"] == "true"]
    interval_maxima = {
        "s_i": max(int(row["s_i"]) for row in eligible_intervals),
        "a_i": max(int(row["a_i"]) for row in eligible_intervals),
        "d_i": max(int(row["midpoint_decisions_d_i"]) for row in eligible_intervals),
        "b_i": max(int(row["boundary_sampler_calls_b_i"]) for row in eligible_intervals),
        "q_i": max(int(row["total_sampler_calls_q_i"]) for row in eligible_intervals),
    }
    budget_totals = {
        "M": sum(int(row["M_effective_public_stops"]) for row in budgets_rows),
        "R": sum(int(row["R_eligible_intervals"]) for row in budgets_rows),
        "E": sum(int(row["E_emitted_stops"]) for row in budgets_rows),
    }
    exact_unknown = sum(row["exact_point_status"] == "Unknown" for row in coincidence_rows)
    epsilon_unknown = sum(row["epsilon_right_status"] == "Unknown" for row in coincidence_rows)
    epsilon_terminal = sum(row["epsilon_right_status"] == "NotApplicableTerminal" for row in coincidence_rows)

    receipt_rows: list[dict[str, Any]] = []
    for index, command in enumerate(PRIOR_METADATA_READS, 1):
        receipt_rows.append({"record_type": "metadata-read-before-runner", "key": f"M{index:03d}", "value": command, "sha256": "", "detail": "metadata only; no unlisted file content opened"})
    for index, action in enumerate(PRIOR_ACTIONS, 1):
        receipt_rows.append({"record_type": "action-before-runner", "key": f"A{index:03d}", "value": action, "sha256": "", "detail": "write/action record; not a content read"})
    for index, command in enumerate(COMMANDS, 1):
        receipt_rows.append({
            "record_type": "runner-command", "key": f"C{index:04d}", "value": command["command"],
            "sha256": command["stdout_sha256"], "detail": f"exit={command['exit']};stderr_sha256={command['stderr_sha256']}",
        })
    dedup_reads: dict[tuple[str, str, str], dict[str, str]] = {}
    for read in READS:
        dedup_reads[(read["path"], read["sha256"], read["purpose"])] = read
    for index, read in enumerate(dedup_reads.values(), 1):
        receipt_rows.append({
            "record_type": "content-read", "key": f"R{index:04d}", "value": read["path"],
            "sha256": read["sha256"], "detail": read["purpose"],
        })
    required_outputs = [
        OUT / "runner.py", OUT / "sources.tsv", OUT / "positive.tsv", OUT / "budgets.tsv",
        OUT / "interval-cost.tsv", masks / "manifest.tsv", OUT / "landmarks.tsv",
        OUT / "coincidence.tsv", OUT / "degenerates.tsv", OUT / "determinism.tsv", OUT / "opaque.tsv",
    ]
    for path in required_outputs:
        receipt_rows.append({
            "record_type": "output", "key": path.name, "value": str(path),
            "sha256": sha_unlogged(path), "detail": "oracle-author artifact",
        })
    written_before_receipt = sorted(path for path in OUT.rglob("*") if path.is_file())
    for path in written_before_receipt:
        assert_write_path(path)
    write_root_listing = "\n".join(str(path) for path in written_before_receipt)
    receipt_rows.extend([
        {"record_type": "run", "key": "measured_at", "value": measured_at, "sha256": "", "detail": "America/Sao_Paulo process timezone"},
        {"record_type": "run", "key": "repository_head", "value": repository_head, "sha256": "", "detail": f"git status metadata sha256={sha_text(repository_status)};working tree has untracked P1268 metadata;no repository writes by runner"},
        {"record_type": "run", "key": "baseline", "value": "upstream/main a51e02804 ratified", "sha256": EXPECTED[VANILLA], "detail": str(VANILLA)},
        {"record_type": "run", "key": "regime", "value": "Tekt full protocol; ORACLE-REEXECUTOR-V3 only", "sha256": EXPECTED[PREORACLE], "detail": "candidate-blind preseal phase"},
        {"record_type": "capability", "key": "read_allowlist", "value": "V3 exact frozen paths + 96 corpus-pinned fixtures + authorized ADR/skill references + post-freeze historical budget identity", "sha256": "", "detail": "no recursive scans"},
        {"record_type": "capability", "key": "write_allowlist", "value": str(OUT), "sha256": "", "detail": "repository unchanged"},
        {"record_type": "capability", "key": "write_root_proof", "value": f"{len(written_before_receipt)} files all under {OUT}", "sha256": sha_text(write_root_listing), "detail": "assert_write_path enforced on TSV and /tmp command paths;recursive output enumeration passed"},
        {"record_type": "capability", "key": "context", "value": "none", "sha256": "", "detail": "isolated V3 reexecutor authority"},
        {"record_type": "capability", "key": "typst_execution", "value": str(VANILLA), "sha256": EXPECTED[VANILLA], "detail": "no candidate execution"},
        {"record_type": "limitation", "key": "filesystem_isolation", "value": "not attested", "sha256": "", "detail": "allowlist enforced by role and runner guards, not OS-level read sandbox"},
        {"record_type": "limitation", "key": "repository_commit_state", "value": "metadata-only", "sha256": "", "detail": f"HEAD={repository_head};git-status-sha256={sha_text(repository_status)};content evidence remains pinned per input and fixture"},
        {"record_type": "historical-comparison", "key": "new_budget_sha_before_history", "value": str(OUT / "budgets.tsv"), "sha256": new_budget_digest_before_history, "detail": "frozen before historical read"},
        {"record_type": "historical-comparison", "key": "matching_rows", "value": f"{historical_identity_matches}/96", "sha256": observed_history_sha, "detail": f"comparison only; no decision or budget mutation;mismatch_ids={','.join(historical_mismatch_ids) or 'none'}"},
        {"record_type": "gate", "key": "population", "value": "96;24 per pair", "sha256": EXPECTED[CORPUS], "detail": "complete"},
        {"record_type": "gate", "key": "effective-stop-accounting", "value": f"M={budget_totals['M']};R={budget_totals['R']};E={budget_totals['E']}", "sha256": sha_unlogged(OUT / "budgets.tsv"), "detail": "96 current vanilla-first fixtures;not historical accounting"},
        {"record_type": "gate", "key": "interval-maxima", "value": ";".join(f"{key}={value}" for key, value in interval_maxima.items()), "sha256": sha_unlogged(OUT / "interval-cost.tsv"), "detail": f"eligible={len(eligible_intervals)};ineligible={len(interval_rows)-len(eligible_intervals)};limits=64/63/127/126/253"},
        {"record_type": "gate", "key": "interval-bound-failures", "value": str(sum(row["bounds_64_63_127_126_253"] != "Preserved" for row in interval_rows)), "sha256": sha_unlogged(OUT / "interval-cost.tsv"), "detail": "zero required"},
        {"record_type": "gate", "key": "affine-identity-unknown", "value": str(sum(row["identity_and_bounds"] != "Measured" for row in budgets_rows)), "sha256": sha_unlogged(OUT / "budgets.tsv"), "detail": "E=M+sum(a_i)<=M+63R<=64M-63"},
        {"record_type": "gate", "key": "coincidence", "value": f"groups={len(coincidence_rows)};exact_unknown={exact_unknown};epsilon_unknown={epsilon_unknown};terminal_epsilon_na={epsilon_terminal}", "sha256": sha_unlogged(OUT / "coincidence.tsv"), "detail": "exact-point and epsilon-right are separate;structural basis recorded"},
        {"record_type": "gate", "key": "degenerates", "value": f"S20={sum(r['seed']=='S20' and r['status']=='Measured' for r in degenerate_rows)}/4;S21={sum(r['seed']=='S21' and r['status']=='Measured' for r in degenerate_rows)}/4", "sha256": sha_unlogged(OUT / "degenerates.tsv"), "detail": "positive stroke support versus zero fill support"},
        {"record_type": "gate", "key": "determinism", "value": f"{sum(r['status'] == 'MeasuredEqual' for r in determinism_rows)}/192", "sha256": sha_unlogged(OUT / "determinism.tsv"), "detail": "direct/inverse/repeat semantic comparison"},
        {"record_type": "gate", "key": "unknown_honesty", "value": str(sum(r["oracle_status"] == "Unknown" for r in positive_rows)), "sha256": sha_unlogged(OUT / "opaque.tsv"), "detail": "Unknown is never success"},
        {"record_type": "gate", "key": "breach", "value": "false", "sha256": "", "detail": "no prohibited content read or out-of-root write observed;true would invalidate this receipt and oracle"},
        {"record_type": "verdict-boundary", "key": "seal", "value": "NOT ISSUED", "sha256": "", "detail": "oracle author does not write seal, attacks, contract, product, or verdict"},
        {"record_type": "attestation", "key": "status", "value": "EXECUTADO SEM ATESTACAO DE ISOLAMENTO", "sha256": "", "detail": "role segregation recorded; filesystem isolation not proven"},
    ])
    write_tsv(OUT / "receipt.tsv", receipt_rows, ["record_type", "key", "value", "sha256", "detail"])
    artifact_paths = required_outputs + [OUT / "receipt.tsv"]
    print(json.dumps({
        "status": "ORACLE-REEXECUTOR-V3-COMPLETE-NO-SEAL",
        "population": len(sources_rows),
        "positive_available": sum(row["oracle_status"] == "Available" for row in positive_rows),
        "budget_totals": budget_totals,
        "eligible_intervals": len(eligible_intervals),
        "interval_maxima": interval_maxima,
        "coincidence_groups": len(coincidence_rows),
        "exact_unknown": exact_unknown,
        "epsilon_unknown": epsilon_unknown,
        "degenerates_measured": sum(row["status"] == "Measured" for row in degenerate_rows),
        "determinism_equal": sum(row["status"] == "MeasuredEqual" for row in determinism_rows),
        "historical_identity_matches": historical_identity_matches,
        "historical_mismatch_ids": historical_mismatch_ids,
        "breach": False,
        "seal_issued": False,
        "artifacts": {str(path): sha_unlogged(path) for path in artifact_paths},
    }, sort_keys=True))


if __name__ == "__main__":
    main()
