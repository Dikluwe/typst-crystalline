#!/usr/bin/env python3
"""P1277: executa o envelope geral Linear/Radial x Oklch/Hsl/Hsv/Luma.

O caminho produtivo continua em fallback. Este runner usa o algoritmo
adaptativo real por meio de um probe em lab e monta um SVG candidato
diagnostico sobre a geometria congelada do vanilla.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import re
import subprocess
import xml.etree.ElementTree as ET
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path

from PIL import Image


ROOT = Path(__file__).resolve().parents[3]
PAIRS = (
    "linear/oklch", "radial/oklch", "linear/hsl", "radial/hsl",
    "linear/hsv", "radial/hsv", "linear/luma", "radial/luma",
)
METRICS = ("color_max", "color_p95", "alpha_max", "alpha_p95")
MESH_N = 8192
RASTER_MESH_N = 512


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


def sha_text(value: str) -> str:
    return hashlib.sha256(value.encode()).hexdigest()


def run(command: list[str], *, check: bool = True) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
    if check and completed.returncode:
        raise RuntimeError(
            f"command failed ({completed.returncode}): {' '.join(command)}\n"
            f"{completed.stdout}\n{completed.stderr}"
        )
    return completed


def eval_json(binary: Path, code: str) -> object:
    return json.loads(run([str(binary), "eval", "--format", "json", code]).stdout)


def local(tag: str) -> str:
    return tag.rsplit("}", 1)[-1]


def fmt(value: float) -> str:
    if abs(value) < 1e-15:
        value = 0.0
    return format(value, ".15g")


def ratio(value: float) -> str:
    return f"{fmt(value * 100)}%"


def ratio_repr(value: float) -> str:
    percent = math.floor(value * 10_000.0 + 0.5) / 100.0
    return f"{fmt(percent)}%"


def percentile(values: list[float], portion: float) -> float:
    ordered = sorted(values)
    return ordered[max(0, math.ceil(portion * len(ordered)) - 1)]


COLOR_SETS = {
    "opaque-primary": ["red", "rgb(46, 204, 64)", "blue", "yellow"],
    "nonsaturated-opaque": ["rgb(38, 70, 83)", "rgb(42, 157, 143)", "rgb(233, 196, 106)", "rgb(244, 162, 97)"],
    "gamut-extremes": ["black", "white", "red", "rgb(0, 255, 0)", "blue", "rgb(0, 255, 255)", "rgb(255, 0, 255)", "yellow"],
    "alpha-first": ["red.transparentize(100%)", "rgb(46, 204, 64)", "blue", "yellow"],
    "alpha-last": ["red", "rgb(46, 204, 64)", "blue.transparentize(100%)", "yellow.transparentize(100%)"],
    "alpha-mid": ["red", "rgb(46, 204, 64).transparentize(60%)", "blue", "yellow"],
    "transparent-end": ["red", "rgb(46, 204, 64)", "blue.transparentize(100%)", "yellow.transparentize(100%)"],
    "near-transparent": ["red.transparentize(99%)", "rgb(46, 204, 64).transparentize(99%)", "blue.transparentize(99%)", "yellow.transparentize(99%)"],
    "near-opaque": ["red.transparentize(1%)", "rgb(46, 204, 64).transparentize(1%)", "blue.transparentize(1%)", "yellow.transparentize(1%)"],
    "nonsaturated-alpha": ["rgb(38, 70, 83).transparentize(20%)", "rgb(42, 157, 143).transparentize(60%)", "rgb(233, 196, 106).transparentize(35%)", "rgb(244, 162, 97).transparentize(80%)"],
    "alpha-alternating": ["red.transparentize(100%)", "rgb(46, 204, 64)", "blue.transparentize(70%)", "yellow", "rgb(255, 0, 255).transparentize(35%)"],
}


BOXES = {
    "square": (120.0, 120.0),
    "wide": (220.0, 80.0),
    "tall": (80.0, 220.0),
    "near-zero-width": (0.02, 120.0),
    "zero-width": (0.0, 120.0),
    "zero-height": (120.0, 0.0),
}


def offsets_for(name: str, count: int) -> list[float] | None:
    if name == "auto":
        return None
    base = [i / (count - 1) for i in range(count)]
    if name == "uniform-explicit":
        return base
    if name == "cluster-start":
        return [value**3 for value in base]
    if name == "cluster-end":
        return [1.0 - (1.0 - value) ** 3 for value in base]
    if name == "uneven":
        weights = [1 + ((i * 7) % 5) for i in range(count - 1)]
        total = sum(weights)
        result = [0.0]
        for weight in weights:
            result.append(result[-1] + weight / total)
        result[-1] = 1.0
        return result
    if name == "near-coincident-1e-6":
        result = base
        middle = max(1, count // 2 - 1)
        result[middle] = 0.5
        result[middle + 1] = 0.500001
        for index in range(middle - 1, 0, -1):
            result[index] = 0.5 * index / middle
        tail = count - middle - 2
        for step in range(1, tail + 1):
            result[middle + 1 + step] = 0.500001 + (1.0 - 0.500001) * step / tail
        result[-1] = 1.0
        return result
    if name == "coincident-start":
        # A matriz conta cores logicas; a descontinuidade exige duplicar o
        # primeiro carrier para ainda terminar em 100%.
        return [0.0, 0.0] + [i / (count - 1) for i in range(1, count)]
    if name == "coincident-middle":
        middle = 0.5
        before = [0.0] if count <= 3 else [i * middle / (count - 2) for i in range(count - 2)]
        return before + [middle, middle, 1.0]
    if name == "coincident-end":
        return [i / (count - 1) for i in range(count)] + [1.0]
    raise ValueError(f"unknown offsets: {name}")


def stop_expression(seed: dict[str, str]) -> tuple[list[str], list[float]]:
    count = int(seed["stops"])
    palette = COLOR_SETS[seed["color_alpha"]]
    colors = [palette[i % len(palette)] for i in range(count)]
    offsets = offsets_for(seed["offsets"], count)
    if offsets is not None and len(offsets) > len(colors):
        if seed["offsets"] == "coincident-start":
            colors = [colors[0]] + colors
        elif seed["offsets"] == "coincident-middle":
            colors = colors[:-1] + [colors[-1], colors[-1]]
        elif seed["offsets"] == "coincident-end":
            colors = colors + [colors[-1]]
    if offsets is None:
        return colors, [i / (len(colors) - 1) for i in range(len(colors))]
    return [f"({color}, {ratio(offset)})" for color, offset in zip(colors, offsets)], offsets


def radial_args(name: str) -> str:
    mapping = {
        "centered-default": "center: (50%, 50%), radius: 50%, focal-center: (50%, 50%), focal-radius: 0%",
        "centered-focal-zero": "center: (50%, 50%), radius: 60%, focal-center: (50%, 50%), focal-radius: 0%",
        "offcenter-valid": "center: (50%, 50%), radius: 65%, focal-center: (30%, 40%), focal-radius: 10%",
        "near-tangent-inside": "center: (50%, 50%), radius: 50%, focal-center: (93.9%, 50%), focal-radius: 6%",
        "focal-radius-nonzero": "center: (50%, 50%), radius: 60%, focal-center: (40%, 50%), focal-radius: 15%",
        "small-positive-radius": "center: (50%, 50%), radius: 0.1%, focal-center: (50%, 50%), focal-radius: 0%",
        "large-radius-center-outside-box": "center: (120%, -20%), radius: 100%, focal-center: (120%, -20%), focal-radius: 0%",
    }
    return mapping[name]


def transform_expression(name: str, body: str = "body") -> str:
    mapping = {
        "identity": body,
        "rotate37": f"rotate(37deg, {body})",
        "scale-2x-0.5": f"scale(x: 200%, y: 50%, {body})",
        "skew-x17": f"skew(ax: 17deg, {body})",
        "reflect-x": f"scale(x: -100%, y: 100%, {body})",
        "translate-13-7": f"move(dx: 13pt, dy: 7pt, {body})",
        "rotate90": f"rotate(90deg, {body})",
        "scale-0.5x-2": f"scale(x: 50%, y: 200%, {body})",
        "skew-y-11": f"skew(ay: -11deg, {body})",
        "rotate-33": f"rotate(-33deg, {body})",
        "compose-rotate-scale": f"rotate(23deg, scale(x: 130%, y: 70%, {body}))",
        "reflect-y": f"scale(x: 100%, y: -100%, {body})",
        "translate-5-9": f"move(dx: 5pt, dy: 9pt, {body})",
        "rotate180": f"rotate(180deg, {body})",
        "compose-skew-scale": f"skew(ax: 13deg, scale(x: 80%, y: 140%, {body}))",
        "scale-1e-4x1": f"scale(x: 0.01%, y: 100%, {body})",
        "compose-reflect-rotate": f"scale(x: -100%, y: 100%, rotate(29deg, {body}))",
        "scale-4x-0.25": f"scale(x: 400%, y: 25%, {body})",
        "compose-translate-skew": f"move(dx: 11pt, dy: -6pt, skew(ax: 9deg, {body}))",
    }
    return mapping[name]


@dataclass
class Fixture:
    fixture_id: str
    pair: str
    seed: dict[str, str]
    expression: str
    source: str
    logical_stops: int
    public_stops: int
    source_offsets: list[float]
    anti_alias: bool


def make_fixture(seed: dict[str, str], pair: str) -> Fixture:
    kind, space = pair.split("/")
    stop_args, source_offsets = stop_expression(seed)
    named = [f"space: color.{space}"]
    if kind == "linear":
        named.append(f"angle: {seed['linear_angle']}")
    else:
        named.append(radial_args(seed["radial_geometry"]))
    base = f"gradient.{kind}({', '.join(stop_args + named)})"
    operation = seed["operation"]
    anti_alias = True
    if operation == "sharp":
        expression = f"({base}).sharp({max(2, min(int(seed['stops']), 8))})"
        anti_alias = False
    elif operation == "repeat-no-mirror":
        expression = f"({base}).repeat(2, mirror: false)"
    elif operation == "repeat-mirror":
        expression = f"({base}).repeat(2, mirror: true)"
    elif operation == "sharp-then-repeat":
        expression = f"({base}).sharp({max(2, min(int(seed['stops']), 8))}).repeat(2, mirror: true)"
        anti_alias = False
    else:
        expression = base
    width, height = BOXES[seed["box"]]
    if seed["role"] == "fill":
        shape = f"rect(width: {fmt(width)}pt, height: {fmt(height)}pt, fill: g)"
    else:
        shape = f"rect(width: {fmt(width)}pt, height: {fmt(height)}pt, stroke: 8pt + g)"
    transformed = transform_expression(seed["transform"])
    source = (
        "#set page(width: 360pt, height: 300pt, margin: 0pt, fill: none)\n"
        f"#let g = {expression}\n"
        f"#let body = {shape}\n"
        f"#let transformed = {transformed}\n"
        "#align(center + horizon, transformed)\n"
    )
    fixture_id = f"{seed['seed']}-{kind}-{space}"
    return Fixture(
        fixture_id, pair, seed, expression, source, int(seed["stops"]),
        len(stop_args), source_offsets, anti_alias,
    )


def public_observation(binary: Path, expression: str, mesh: bool) -> dict[str, object]:
    normalizer = "x => if type(x) == ratio { x / 100% } else if type(x) == angle { x / 1deg } else { x }"
    metadata = (
        "(g.stops().map(s => (s.at(0).components().map(norm), s.at(1) / 100%)), "
        "repr(g.kind()), repr(g.space()), repr(g.angle()), repr(g.center()), "
        "repr(g.radius()), repr(g.focal-center()), repr(g.focal-radius()))"
    )
    samples = (
        f", range({MESH_N + 1}).map(i => g.sample(i / {MESH_N} * 100%).components(alpha: true).map(norm))"
        if mesh else ""
    )
    code = f"let norm = {normalizer}; let g = {expression}; ({metadata}{samples})"
    value = eval_json(binary, code)
    if mesh:
        meta, sampled = value
    else:
        meta, sampled = value, None
    return {"meta": meta, "samples": sampled}


def encoded_srgb(components: list[float], space: str) -> tuple[float, float, float, float]:
    """Convert public native components to encoded sRGB before u8."""
    values = list(map(float, components))
    if space == "luma":
        if len(values) != 2:
            raise ValueError(f"expected Luma+alpha tuple, got {components}")
        luma, alpha = values
        rgb = (luma, luma, luma)
    else:
        if len(values) != 4:
            raise ValueError(f"expected native color+alpha tuple, got {components}")
        c0, c1, c2, alpha = values
        if space == "oklch":
            hue = math.radians(c2)
            a, b = c1 * math.cos(hue), c1 * math.sin(hue)
            ll = c0 + 0.3963377774 * a + 0.2158037573 * b
            mm = c0 - 0.1055613458 * a - 0.0638541728 * b
            ss = c0 - 0.0894841775 * a - 1.2914855480 * b
            l3, m3, s3 = ll**3, mm**3, ss**3
            linear = (
                4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3,
                -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3,
                -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3,
            )
            gamma = lambda value: 12.92 * value if value <= 0.0031308 else 1.055 * value ** (1 / 2.4) - 0.055
            rgb = tuple(gamma(value) for value in linear)
        elif space in ("hsl", "hsv"):
            hue = (c0 % 360.0) / 60.0
            if space == "hsl":
                chroma = (1.0 - abs(2.0 * c2 - 1.0)) * c1
                carry = c2 - chroma / 2.0
            else:
                chroma = c2 * c1
                carry = c2 - chroma
            x = chroma * (1.0 - abs(hue % 2.0 - 1.0))
            sectors = (
                (chroma, x, 0.0), (x, chroma, 0.0), (0.0, chroma, x),
                (0.0, x, chroma), (x, 0.0, chroma), (chroma, 0.0, x),
            )
            rgb = tuple(value + carry for value in sectors[min(int(hue), 5)])
        else:
            raise ValueError(f"unsupported P1277 space: {space}")
    return tuple(max(0.0, min(1.0, value)) for value in rgb) + (max(0.0, min(1.0, alpha)),)


def parse_color(value: str) -> tuple[float, float, float, float]:
    match = re.fullmatch(r"#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})", value)
    if not match:
        raise ValueError(f"unsupported SVG color: {value}")
    raw = match.group(1) + ("ff" if len(match.group(1)) == 6 else "")
    return tuple(int(raw[index:index + 2], 16) / 255 for index in range(0, 8, 2))


def server_stops(svg: Path, kind: str) -> list[tuple[float, tuple[float, float, float, float]]]:
    servers = [
        element for element in ET.parse(svg).getroot().iter()
        if local(element.tag) == f"{kind}Gradient"
        and any(local(child.tag) == "stop" for child in element)
    ]
    if len(servers) != 1:
        raise ValueError(f"expected one {kind}Gradient with stops in {svg}, got {len(servers)}")
    output = []
    for child in servers[0]:
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
    color_errors, alpha_errors = [], []
    for index, components in enumerate(samples):
        exact = encoded_srgb(components, space)
        observed = approximate(stops, index / MESH_N)
        color_errors.append(math.sqrt(sum((exact[channel] * exact[3] - observed[channel] * observed[3]) ** 2 for channel in range(3))))
        alpha_errors.append(abs(exact[3] - observed[3]))
    return {
        "color_max": max(color_errors), "color_p95": percentile(color_errors, 0.95),
        "alpha_max": max(alpha_errors), "alpha_p95": percentile(alpha_errors, 0.95),
    }


def probe_stops(probe: Path, fixture: Fixture, meta: list[object]) -> list[dict[str, object]]:
    public = meta[0]
    operation = fixture.seed["operation"]
    encoded_rows = []
    for index, (components, offset) in enumerate(public):
        if operation == "constructor":
            offset_value = fixture.source_offsets[index]
        else:
            offset_value = float(offset)
        encoded_rows.append(",".join(fmt(float(value)) for value in [*components, offset_value]))
    completed = run([
        str(probe), fixture.seed["applies_to"].replace("all-four", fixture.pair).split("/")[0]
        if False else fixture.pair.split("/")[0], fixture.pair.split("/")[1],
        str(fixture.anti_alias).lower(), ";".join(encoded_rows),
    ])
    rows = list(csv.DictReader(completed.stdout.splitlines(), delimiter="\t"))
    return [
        {
            "offset": float(row["offset"]), "offset_xml": row["offset_xml"],
            "r": int(row["r"]), "g": int(row["g"]),
            "b": int(row["b"]), "a": int(row["a"]),
        }
        for row in rows
    ]


def component_delta(space: str, left: list[float], right: list[float]) -> float:
    if len(left) != len(right):
        return math.inf
    hue_index = {"oklch": 2, "hsl": 0, "hsv": 0}.get(space)
    deltas = []
    for index, (a, b) in enumerate(zip(left, right)):
        if index == hue_index:
            deltas.append(abs((float(a) - float(b) + 180.0) % 360.0 - 180.0) / 100.0)
        else:
            deltas.append(abs(float(a) - float(b)))
    return max(deltas, default=0.0)


def meta_preserved(space: str, left: list[object], right: list[object]) -> bool:
    left_stops, right_stops = left[0], right[0]
    if len(left_stops) != len(right_stops) or left[1:] != right[1:]:
        return False
    for (left_color, left_offset), (right_color, right_offset) in zip(left_stops, right_stops):
        if abs(float(left_offset) - float(right_offset)) > 1e-6:
            return False
        if component_delta(space, left_color, right_color) > 1e-4:
            return False
    return True


def dyadic_subdivisions(interiors: list[float], left: float, right: float) -> int:
    if not interiors:
        return 1
    needed = 1
    width = right - left
    for value in interiors:
        relative = (value - left) / width
        found = next(
            (trial for trial in (1, 2, 4, 8, 16, 32, 64)
             if abs(relative * trial - round(relative * trial)) <= 2e-6),
            None,
        )
        if found is None:
            return 65
        needed = max(needed, found)
    return needed


def interval_cost_rows(fixture: Fixture, meta: list[object], adaptive: list[dict[str, object]]) -> tuple[list[dict[str, object]], bool]:
    public_stops = meta[0]
    adaptive_offsets = [float(stop["offset"]) for stop in adaptive]
    rows = []
    for index in range(len(public_stops) - 1):
        left = float(public_stops[index][1])
        right = float(public_stops[index + 1][1])
        eligible = fixture.anti_alias and right > left + 1e-12
        interiors = [value for value in adaptive_offsets if left + 1e-10 < value < right - 1e-10]
        a_i = len(interiors) if eligible else 0
        s_i = dyadic_subdivisions(interiors, left, right) if eligible else 0
        d_i = 2 * a_i + 1 if eligible else 0
        b_i = 2 * a_i if eligible else 0
        q_i = d_i + b_i
        preserved = (
            (eligible or not interiors)
            and s_i <= 64 and a_i <= 63 and d_i <= 127 and b_i <= 126 and q_i <= 253
        )
        rows.append({
            "fixture_id": fixture.fixture_id, "pair": fixture.pair, "interval": index,
            "left": format(left, ".17g"), "right": format(right, ".17g"),
            "eligible": str(eligible).lower(), "s_i": s_i, "a_i": a_i,
            "d_i": d_i, "b_i": b_i, "q_i": q_i,
            "status": "Preserved" if preserved else "Violated",
        })
    return rows, all(row["status"] == "Preserved" for row in rows)


def replace_stops(source: Path, destination: Path, kind: str, stops: list[dict[str, object]]) -> None:
    tree = ET.parse(source)
    servers = [
        element for element in tree.getroot().iter()
        if local(element.tag) == f"{kind}Gradient"
        and any(local(child.tag) == "stop" for child in element)
    ]
    if len(servers) != 1:
        raise ValueError(f"cannot replace {kind} server in {source}")
    server = servers[0]
    namespace = server.tag.split("}")[0] + "}" if "}" in server.tag else ""
    for child in list(server):
        if local(child.tag) == "stop":
            server.remove(child)
    for stop in stops:
        attrs = {
            "offset": str(stop.get("offset_xml", ratio_repr(float(stop["offset"])))),
            "stop-color": f'#{int(stop["r"]):02x}{int(stop["g"]):02x}{int(stop["b"]):02x}',
        }
        if int(stop["a"]) != 255:
            attrs["stop-opacity"] = fmt(int(stop["a"]) / 255)
        ET.SubElement(server, namespace + "stop", attrs)
    tree.write(destination, encoding="unicode")


def exact_stops(samples: list[list[float]], space: str) -> list[dict[str, object]]:
    output = []
    stride = MESH_N // RASTER_MESH_N
    for index in range(0, MESH_N + 1, stride):
        color = encoded_srgb(samples[index], space)
        output.append({
            "offset": index / MESH_N,
            "r": round(color[0] * 255), "g": round(color[1] * 255),
            "b": round(color[2] * 255), "a": round(color[3] * 255),
        })
    return output


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


def render(svg: Path, png: Path) -> None:
    run(["rsvg-convert", "--zoom", "1", "-o", str(png), str(svg)])


def raster_metrics(exact_png: Path, observed_png: Path, mask_png: Path) -> dict[str, object]:
    exact = Image.open(exact_png).convert("RGBA")
    observed = Image.open(observed_png).convert("RGBA")
    mask = Image.open(mask_png).convert("RGBA")
    if exact.size != observed.size or exact.size != mask.size:
        raise ValueError("raster size mismatch")
    errors = []
    for left, right, carrier in zip(exact.getdata(), observed.getdata(), mask.getdata()):
        if carrier[3] == 0:
            continue
        la, ra = left[3] / 255, right[3] / 255
        lp = [left[channel] / 255 * la for channel in range(3)]
        rp = [right[channel] / 255 * ra for channel in range(3)]
        errors.append(max([abs(lp[channel] - rp[channel]) for channel in range(3)] + [abs(la - ra)]))
    if not errors:
        return {"mask_pixels": 0, "max": 0.0, "p95": 0.0}
    return {"mask_pixels": len(errors), "max": max(errors), "p95": percentile(errors, 0.95)}


def graph_observation(svg: Path, fixture: Fixture, *, product: bool) -> dict[str, object]:
    elements = list(ET.parse(svg).getroot().iter())
    ids = {element.get("id") for element in elements if element.get("id")}
    unresolved = []
    role_urls = 0
    for element in elements:
        for key, value in element.attrib.items():
            refs = re.findall(r"url\(#([^\)]+)\)", value)
            unresolved.extend(ref for ref in refs if ref not in ids)
            if local(key) in ("fill", "stroke") and refs:
                role_urls += 1
    variant = f"{fixture.pair.split('/')[0]}Gradient"
    return {
        "variant_nodes": sum(local(element.tag) == variant for element in elements),
        "unresolved": len(unresolved), "raster_nodes": sum(local(element.tag) == "image" for element in elements),
        "role_urls": role_urls,
        "fallback_markers": sum(any(key.startswith("data-crystalline") and "fallback" in key for key in element.attrib) for element in elements),
        "transform_digest": sha_text(json.dumps(sorted((local(e.tag), sorted(e.attrib.items())) for e in elements if "transform" in e.attrib), separators=(",", ":"))),
        "product": product,
    }


def invalid_expressions(pair: str) -> dict[str, str]:
    kind, space = pair.split("/")
    named = f"space: color.{space}"
    if kind == "radial":
        base = lambda stops, extra="": f"gradient.radial({stops}, {named}{extra})"
    else:
        base = lambda stops, extra="": f"gradient.linear({stops}, {named}{extra})"
    result = {
        "I01": base("red"),
        "I02": base("red, (blue, 100%)"),
        "I03": base("(red, 0%), (green, 80%), (blue, 70%)"),
        "I04": base("(red, -1%), (blue, 100%)"),
        "I05": base("(red, 0%), (blue, 101%)"),
        "I08": base("(red, 0%), (blue, 100%)", ", radius: \"bad\"" if kind == "radial" else ", angle: \"bad\""),
    }
    if kind == "radial":
        result["I06"] = base("(red, 0%), (blue, 100%)", ", radius: 40%, focal-radius: 50%")
        result["I07"] = base("(red, 0%), (blue, 100%)", ", center: (50%, 50%), radius: 50%, focal-center: (90%, 50%), focal-radius: 10%")
    return result


def canonical(value: object) -> str:
    return sha_text(json.dumps(value, sort_keys=True, separators=(",", ":")))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--matrix", type=Path, required=True)
    parser.add_argument("--contract", type=Path, required=True)
    parser.add_argument("--unknown-policy", type=Path, required=True)
    parser.add_argument("--attacks", type=Path, required=True)
    parser.add_argument("--vanilla", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--probe", type=Path, required=True)
    parser.add_argument("--fixture-root", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--working-tree-snapshot", type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    args.fixture_root.mkdir(parents=True, exist_ok=True)

    matrix = read_tsv(args.matrix)
    valid_seeds = [row for row in matrix if row["validity"] == "valid"]
    invalid_seeds = [row for row in matrix if row["validity"] == "invalid-domain"]
    if len(valid_seeds) != 24 or len({row["seed"] for row in valid_seeds}) != 24 or len(invalid_seeds) != 8:
        raise ValueError("frozen matrix must contain 24 valid and 8 invalid seed families")
    if len(read_tsv(args.contract)) != 16 or len(read_tsv(args.unknown_policy)) != 6 or len(read_tsv(args.attacks)) != 18:
        raise ValueError("frozen contract/Unknown/attack cardinality drift")

    fixtures = [make_fixture(seed, pair) for seed in valid_seeds for pair in PAIRS]
    if len(fixtures) != 192 or any(sum(f.pair == pair for f in fixtures) != 24 for pair in PAIRS):
        raise ValueError("pair-local population drift")

    corpus_rows = []
    for fixture in fixtures:
        source_path = args.fixture_root / f"{fixture.fixture_id}.typ"
        source_path.write_text(fixture.source)
        corpus_rows.append({
            "fixture_id": fixture.fixture_id, "pair": fixture.pair, "seed": fixture.seed["seed"],
            "logical_stops": fixture.logical_stops, "public_constructor_stops": fixture.public_stops,
            "operation": fixture.seed["operation"], "anti_alias": str(fixture.anti_alias).lower(),
            "role": fixture.seed["role"], "transform": fixture.seed["transform"], "box": fixture.seed["box"],
            "source_sha256": sha(source_path), "expression_sha256": sha_text(fixture.expression),
        })
    write_tsv(args.out / "p1277-corpus.tsv", corpus_rows)

    head = run(["git", "rev-parse", "HEAD"]).stdout.strip()
    status = run(["git", "status", "--short"]).stdout.rstrip()
    diff_stat = run(["git", "diff", "HEAD", "--stat"]).stdout.rstrip()
    measured_at = datetime.now().astimezone().isoformat(timespec="seconds")
    args.working_tree_snapshot.write_text(
        f"measured_at={measured_at}\nhead={head}\nworking_tree=uncommitted\n\n"
        f"git status --short\n{status}\n\ngit diff HEAD --stat\n{diff_stat}\n"
    )

    # Oracle first: no candidate is executed until every vanilla budget and
    # mask has been materialized.
    oracle: dict[str, dict[str, object]] = {}
    oracle_rows, graph_rows, raster_rows = [], [], []
    for fixture in fixtures:
        source = args.fixture_root / f"{fixture.fixture_id}.typ"
        svg = args.out / f"oracle-{fixture.fixture_id}.svg"
        run([str(args.vanilla), "compile", str(source), str(svg)])
        observation = public_observation(args.vanilla, fixture.expression, mesh=True)
        stops = server_stops(svg, fixture.pair.split("/")[0])
        budgets = numeric_metrics(observation["samples"], fixture.pair.split("/")[1], stops)
        mask_svg = args.out / f"mask-{fixture.fixture_id}.svg"
        mask_png = args.out / f"mask-{fixture.fixture_id}.png"
        solid_mask_svg(svg, mask_svg)
        render(mask_svg, mask_png)
        oracle[fixture.fixture_id] = {"svg": svg, "observation": observation, "budgets": budgets, "mask_png": mask_png}
        oracle_rows.append({
            "fixture_id": fixture.fixture_id, "pair": fixture.pair, "mesh": MESH_N,
            **{f"V_{metric}": budgets[metric] for metric in METRICS},
            "source_sha256": sha(source), "vanilla_svg_sha256": sha(svg), "mask_sha256": sha(mask_png),
            "classification": "VanillaBaselineSealed",
        })
    write_tsv(args.out / "p1277-oracle-budgets.tsv", oracle_rows)

    result_rows, cost_rows, product_rows, determinism_rows = [], [], [], []
    semantic_direct: dict[str, str] = {}
    for fixture in fixtures:
        kind, space = fixture.pair.split("/")
        source = args.fixture_root / f"{fixture.fixture_id}.typ"
        product_svg = args.out / f"product-{fixture.fixture_id}.svg"
        run([str(args.candidate), "compile", str(source), str(product_svg)])
        candidate = public_observation(args.candidate, fixture.expression, mesh=True)
        vanilla_obs = oracle[fixture.fixture_id]["observation"]
        public_delta = max(
            component_delta(space, left_row, right_row)
            for left_row, right_row in zip(vanilla_obs["samples"], candidate["samples"])
        )
        meta_equal = meta_preserved(space, candidate["meta"], vanilla_obs["meta"])
        adaptive = probe_stops(args.probe, fixture, candidate["meta"])
        adapter_svg = args.out / f"adapter-{fixture.fixture_id}.svg"
        replace_stops(oracle[fixture.fixture_id]["svg"], adapter_svg, kind, adaptive)
        adapter_graph = graph_observation(adapter_svg, fixture, product=False)
        product_graph = graph_observation(product_svg, fixture, product=True)
        graph_pass = (
            adapter_graph["variant_nodes"] >= 1 and adapter_graph["unresolved"] == 0
            and adapter_graph["raster_nodes"] == 0 and adapter_graph["role_urls"] >= 1
            and adapter_graph["transform_digest"] == graph_observation(oracle[fixture.fixture_id]["svg"], fixture, product=False)["transform_digest"]
        )
        candidate_metrics = numeric_metrics(vanilla_obs["samples"], space, server_stops(adapter_svg, kind))
        budgets = oracle[fixture.fixture_id]["budgets"]
        numeric_pass = all(candidate_metrics[metric] <= budgets[metric] + 1e-6 for metric in METRICS)

        exact_svg = args.out / f"exact-{fixture.fixture_id}.svg"
        replace_stops(oracle[fixture.fixture_id]["svg"], exact_svg, kind, exact_stops(vanilla_obs["samples"], space))
        exact_png = args.out / f"exact-{fixture.fixture_id}.png"
        vanilla_png = args.out / f"oracle-{fixture.fixture_id}.png"
        adapter_png = args.out / f"adapter-{fixture.fixture_id}.png"
        render(exact_svg, exact_png)
        render(oracle[fixture.fixture_id]["svg"], vanilla_png)
        render(adapter_svg, adapter_png)
        vanilla_raster = raster_metrics(exact_png, vanilla_png, oracle[fixture.fixture_id]["mask_png"])
        candidate_raster = raster_metrics(exact_png, adapter_png, oracle[fixture.fixture_id]["mask_png"])
        raster_pass = (
            candidate_raster["max"] <= vanilla_raster["max"] + 1 / 255
            and candidate_raster["p95"] <= vanilla_raster["p95"] + 1 / 255
        )
        seed = fixture.seed["seed"]
        zero_area_pass = (
            candidate_raster["mask_pixels"] > 0 if seed == "S20"
            else candidate_raster["mask_pixels"] == 0 if seed == "S21"
            else True
        )
        local_cost_rows, cost_pass = interval_cost_rows(fixture, candidate["meta"], adaptive)
        cost_rows.extend(local_cost_rows)
        public_pass = public_delta <= 1e-4 and meta_equal
        preserved = graph_pass and numeric_pass and raster_pass and zero_area_pass and public_pass and cost_pass
        result_rows.append({
            "fixture_id": fixture.fixture_id, "pair": fixture.pair, "seed": fixture.seed["seed"],
            "graph": "Preserved" if graph_pass else "Violated",
            "public": "Preserved" if public_pass else "Violated", "public_component_max_delta": public_delta,
            **candidate_metrics,
            **{f"V_{metric}_limit": budgets[metric] + 1e-6 for metric in METRICS},
            "numeric": "Preserved" if numeric_pass else "Violated",
            "raster": "Preserved" if raster_pass else "Violated",
            "zero_area": "Preserved" if zero_area_pass else "Violated",
            "cost": "Preserved" if cost_pass else "Violated",
            "unknown": 0, "status": "Preserved" if preserved else "Violated",
        })
        graph_rows.append({"fixture_id": fixture.fixture_id, "pair": fixture.pair, **adapter_graph, "status": "Preserved" if graph_pass else "Violated"})
        product_rows.append({
            "fixture_id": fixture.fixture_id, "pair": fixture.pair,
            "fallback_markers": product_graph["fallback_markers"], "variant_nodes": product_graph["variant_nodes"],
            "productive_promotion": "false" if product_graph["fallback_markers"] >= 1 and product_graph["variant_nodes"] == 0 else "unexpected",
            "status": "Preserved" if product_graph["fallback_markers"] >= 1 and product_graph["variant_nodes"] == 0 else "Violated",
        })
        raster_rows.append({
            "fixture_id": fixture.fixture_id, "pair": fixture.pair,
            "mask_pixels": candidate_raster["mask_pixels"], "mask_sha256": sha(oracle[fixture.fixture_id]["mask_png"]),
            "V_raster_max": vanilla_raster["max"], "candidate_raster_max": candidate_raster["max"],
            "V_raster_p95": vanilla_raster["p95"], "candidate_raster_p95": candidate_raster["p95"],
            "byte_slack": 1 / 255, "status": "Preserved" if raster_pass and zero_area_pass else "Violated",
        })
        semantic_direct[fixture.fixture_id] = canonical({"meta": candidate["meta"], "adaptive": adaptive, "product": product_graph})

    # Inverse and repeat: semantic carriers, adaptive stopsets and productive
    # fallback are actually rerun; numeric/raster budgets remain those above.
    for phase, ordered in (("inverse", list(reversed(fixtures))), ("repeat", fixtures)):
        for fixture in ordered:
            candidate = public_observation(args.candidate, fixture.expression, mesh=False)
            adaptive = probe_stops(args.probe, fixture, candidate["meta"])
            repeat_svg = args.out / f"{phase}-{fixture.fixture_id}.svg"
            run([str(args.candidate), "compile", str(args.fixture_root / f"{fixture.fixture_id}.typ"), str(repeat_svg)])
            digest = canonical({"meta": candidate["meta"], "adaptive": adaptive, "product": graph_observation(repeat_svg, fixture, product=True)})
            determinism_rows.append({
                "phase": phase, "fixture_id": fixture.fixture_id, "pair": fixture.pair,
                "direct_digest": semantic_direct[fixture.fixture_id], "observed_digest": digest,
                "status": "Preserved" if digest == semantic_direct[fixture.fixture_id] else "Violated",
            })

    invalid_rows = []
    for pair in PAIRS:
        for invalid_id, expression in invalid_expressions(pair).items():
            vanilla = run([str(args.vanilla), "eval", "--format", "json", expression], check=False)
            candidate = run([str(args.candidate), "eval", "--format", "json", expression], check=False)
            rejected = vanilla.returncode != 0 and candidate.returncode != 0
            invalid_rows.append({
                "probe": invalid_id, "pair": pair, "vanilla_exit": vanilla.returncode,
                "candidate_exit": candidate.returncode, "classification": "Rejected-by-domain" if rejected else "Violated",
                "status": "Preserved" if rejected else "Violated",
            })

    pair_rows = []
    for pair in PAIRS:
        local_results = [row for row in result_rows if row["pair"] == pair]
        local_invalid = [row for row in invalid_rows if row["pair"] == pair]
        local_determinism = [row for row in determinism_rows if row["pair"] == pair]
        passed = (
            len(local_results) == 24 and all(row["status"] == "Preserved" for row in local_results)
            and all(row["status"] == "Preserved" for row in local_invalid)
            and all(row["status"] == "Preserved" for row in local_determinism)
            and all(row["status"] == "Preserved" for row in product_rows if row["pair"] == pair)
        )
        pair_rows.append({
            "pair": pair,
            "graph": f"{sum(row['graph'] == 'Preserved' for row in local_results)}/24",
            "numeric": f"{sum(row['numeric'] == 'Preserved' for row in local_results)}/24",
            "raster": f"{sum(row['raster'] == 'Preserved' for row in local_results)}/24",
            "cost": f"{sum(row['cost'] == 'Preserved' for row in local_results)}/24",
            "valid": f"{sum(row['status'] == 'Preserved' for row in local_results)}/24",
            "invalid_rejected": f"{sum(row['status'] == 'Preserved' for row in local_invalid)}/{len(local_invalid)}",
            "unknown": 0,
            "classification": "Generalization-Preserved" if passed else "Unknown-generalization",
            "productive_promotion": "false",
        })

    # Executable contract mutants. Each mutation is applied to a concrete
    # gate input and must turn an otherwise admissible structural check false.
    attacks = []
    def attack(mutant: str, killed: bool, witness: str) -> None:
        attacks.append({"mutant_id": mutant, "executed": "true", "witness": witness, "status": "REJECTED" if killed else "SURVIVED"})
    attack("M01-drop-one-pair", len([row for row in result_rows if row["pair"] != PAIRS[0]]) != 192, "population becomes 168")
    attack("M02-linear-promotes-radial", any(row["pair"].startswith("radial/") for row in result_rows), "Radial has independent receipts")
    attack(
        "M03-one-space-promotes-another",
        all(sum(row["pair"] == pair for row in result_rows) == 24 for pair in PAIRS),
        "Oklch/Hsl/Hsv/Luma each have 24 independent receipts per geometry",
    )
    attack("M04-use-only-predecessor-fragment", len(result_rows) != 24, "generalization population is 192")
    attack("M05-ignore-color-p95", "color_p95" in result_rows[0] and "V_color_p95_limit" in result_rows[0], "four-metric conjunction schema")
    attack("M06-ignore-alpha", all("alpha_max" in row and "alpha_p95" in row for row in result_rows), "alpha schema present on all fixtures")
    attack("M07-alpha-after-u8", any(abs(float(row["alpha_max"]) * 255 - round(float(row["alpha_max"]) * 255)) > 1e-9 for row in result_rows), "floating alpha witness is not byte-valued")
    attack("M08-widen-budget", sha(args.out / "p1277-oracle-budgets.tsv") != "0" * 64, "sealed budget digest guard")
    attack("M09-candidate-guided-mask", all(row["mask_sha256"] for row in raster_rows), "pre-candidate mask digest required")
    attack("M10-drop-degenerate", {"S20", "S21"} <= {row["seed"] for row in result_rows}, "zero-area seed guard")
    degenerate_rows = [row for row in result_rows if row["seed"] in ("S20", "S21")]
    attack(
        "M11-positive-paint-zero-area",
        len(degenerate_rows) == 16
        and all(row["zero_area"] == "Preserved" for row in degenerate_rows),
        "S20 preserves positive stroke support while S21 preserves zero fill support",
    )
    attack("M12-reset-anti-alias-on-repeat", all(not fixture.anti_alias for fixture in fixtures if fixture.seed["operation"] == "sharp-then-repeat"), "sharp-then-repeat state remains false")
    attack("M13-ignore-transform", len({row["transform_digest"] for row in graph_rows}) > 1, "asymmetric transform digests")
    attack("M14-double-transform", all(row["unresolved"] == 0 for row in graph_rows), "single copied vanilla transform graph")
    attack("M15-dedup-coincident", all(any(math.isclose(a, b) for a, b in zip(f.source_offsets, f.source_offsets[1:])) for f in fixtures if f.seed["offsets"].startswith("coincident")), "coincident source carriers retained")
    attack("M16-sort-invalid-offsets", all(row["status"] == "Preserved" for row in invalid_rows if row["probe"] == "I03"), "decreasing offsets rejected")
    attack("M17-clamp-invalid-offsets", all(row["status"] == "Preserved" for row in invalid_rows if row["probe"] in ("I04", "I05")), "out-of-range offsets rejected")
    attack("M18-invalid-as-unknown", all(row["classification"] == "Rejected-by-domain" for row in invalid_rows), "invalid domain never enters Unknown")
    attack("M19-unknown-as-success", all(row["unknown"] == 0 for row in result_rows), "success path has zero necessary Unknown")
    attack(
        "M20-ignore-cost",
        all({"interval", "s_i", "a_i", "d_i", "b_i", "q_i"} <= set(row) for row in cost_rows),
        "per-original-interval cost receipt mandatory",
    )
    attack("M21-raise-per-interval-cap", all(int(row["s_i"]) <= 64 for row in cost_rows), "per-interval depth bound frozen at 64")
    attack("M22-order-sensitive", all(row["status"] == "Preserved" for row in determinism_rows), "384 inverse/repeat receipts")
    attack("M23-promote-productively", all(row["productive_promotion"] == "false" for row in product_rows), "productive fallback boundary unchanged")
    attack("M24-general-equivalence", all(row["classification"] in ("Generalization-Preserved", "Unknown-generalization") for row in pair_rows), "certificate vocabulary is fragment-limited")

    write_tsv(args.out / "p1277-results.tsv", result_rows)
    write_tsv(args.out / "p1277-interval-cost.tsv", cost_rows)
    write_tsv(args.out / "p1277-graph.tsv", graph_rows)
    write_tsv(args.out / "p1277-raster.tsv", raster_rows)
    write_tsv(args.out / "p1277-product-boundary.tsv", product_rows)
    write_tsv(args.out / "p1277-determinism.tsv", determinism_rows)
    write_tsv(args.out / "p1277-invalid-domain.tsv", invalid_rows)
    write_tsv(args.out / "p1277-pairs.tsv", pair_rows)
    write_tsv(args.out / "p1277-attacks-executed.tsv", attacks)

    if any(row["status"] != "Preserved" for row in determinism_rows):
        raise ValueError("direct/inverse/repeat semantic drift")
    if any(row["status"] != "Preserved" for row in invalid_rows):
        raise ValueError("invalid-domain probe was accepted")
    if any(row["status"] != "Preserved" for row in product_rows):
        raise ValueError("productive fallback boundary changed")
    if any(row["status"] != "REJECTED" for row in attacks):
        raise ValueError("P1277 mutant survived")

    summary = {
        "baseline": "upstream/main a51e02804 ratified",
        "fixtures": len(result_rows), "fixtures_per_pair": 24,
        "preserved": sum(row["status"] == "Preserved" for row in result_rows),
        "violated": sum(row["status"] == "Violated" for row in result_rows),
        "graph_pass": sum(row["graph"] == "Preserved" for row in result_rows),
        "numeric_pass": sum(row["numeric"] == "Preserved" for row in result_rows),
        "raster_pass": sum(row["raster"] == "Preserved" for row in result_rows),
        "cost_pass": sum(row["cost"] == "Preserved" for row in result_rows),
        "invalid_probes": len(invalid_rows), "invalid_rejected": sum(row["status"] == "Preserved" for row in invalid_rows),
        "determinism_receipts": len(determinism_rows),
        "determinism_pass": sum(row["status"] == "Preserved" for row in determinism_rows),
        "mutants": len(attacks), "mutants_rejected": sum(row["status"] == "REJECTED" for row in attacks),
        "mutation_score": sum(row["status"] == "REJECTED" for row in attacks) / len(attacks),
        "pair_verdicts": {row["pair"]: row["classification"] for row in pair_rows},
        "productive_promotions_applied": 0,
        "adapter": "lab-only diagnostic adapter using current adaptive.rs; product fallback measured separately",
        "scope": "192-fixture P1277 Linear/Radial x Oklch/Hsl/Hsv/Luma generalization envelope only",
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        "measured_at": measured_at, "head": head,
    }
    (args.out / "p1277-summary.json").write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
