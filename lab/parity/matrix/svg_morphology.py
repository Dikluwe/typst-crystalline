#!/usr/bin/env python3
"""P1220/P1221 focal SVG morphology comparator.

It models only the paint vocabulary needed by the frozen plain-text fixture.
Unsupported tags/transforms are Unknown instead of silently equivalent.
"""

from __future__ import annotations

import argparse
import json
import pathlib
import re
from decimal import Decimal
from xml.etree import ElementTree as ET

NUMBER = re.compile(r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?")
TOKEN = re.compile(r"[A-Za-z]|[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?")
MATRIX = re.compile(r"matrix\(\s*([^)]*)\)")
TRANSLATE = re.compile(r"translate\(\s*([^)]*)\)")
TRANSFORM = re.compile(r"(matrix|translate)\(\s*([^)]*)\)")


def local(name: str) -> str:
    return name.rsplit("}", 1)[-1]


def number(value: str) -> Decimal:
    match = NUMBER.search(value)
    if not match:
        raise ValueError(f"missing number: {value}")
    return Decimal(match.group())


def path_tokens(value: str) -> tuple[str, ...]:
    result = []
    for token in TOKEN.findall(value):
        if token[0].isalpha():
            result.append(token)
        else:
            result.append(str(Decimal(token).normalize()))
    return tuple(result)


def matrix(value: str | None) -> tuple[Decimal, ...]:
    if value is None:
        return (Decimal(1), Decimal(0), Decimal(0), Decimal(1), Decimal(0), Decimal(0))
    raw = value.strip()
    found = list(TRANSFORM.finditer(raw))
    if not found or "".join(match.group(0) for match in found).replace(" ", "") != raw.replace(" ", ""):
        raise ValueError(f"unsupported transform: {value}")
    result = matrix(None)
    for match in found:
        values = tuple(Decimal(item) for item in re.split(r"[ ,]+", match.group(2).strip()))
        if match.group(1) == "matrix":
            if len(values) != 6:
                raise ValueError(f"invalid matrix: {value}")
            child = values
        else:
            if len(values) not in (1, 2):
                raise ValueError(f"invalid translate: {value}")
            child = (Decimal(1), Decimal(0), Decimal(0), Decimal(1), values[0], values[-1] if len(values) == 2 else Decimal(0))
        result = compose(result, child)
    return result


def compose(parent: tuple[Decimal, ...], child: tuple[Decimal, ...]) -> tuple[Decimal, ...]:
    a, b, c, d, e, f = parent
    g, h, i, j, k, l = child
    return (a*g+c*h, b*g+d*h, a*i+c*j, b*i+d*j, a*k+c*l+e, b*k+d*l+f)


def translate(base: tuple[Decimal, ...], x: Decimal, y: Decimal) -> tuple[Decimal, ...]:
    return compose(base, (Decimal(1), Decimal(0), Decimal(0), Decimal(1), x, y))


def point(base, x, y):
    a, b, c, d, e, f = base
    return (str((a*x+c*y+e).normalize()), str((b*x+d*y+f).normalize()))


def shape_paint(elem, geometry):
    fill = elem.attrib.get("fill", "#000000")
    stroke = elem.attrib.get("stroke")
    stroke_data = None if stroke is None else (
        stroke,
        elem.attrib.get("stroke-width", "1"),
        elem.attrib.get("stroke-linecap", "butt"),
        elem.attrib.get("stroke-linejoin", "miter"),
        elem.attrib.get("stroke-miterlimit", "4"),
        elem.attrib.get("stroke-dasharray", "none"),
        elem.attrib.get("stroke-dashoffset", "0"),
    )
    return ("shape", geometry, fill, elem.attrib.get("fill-rule", "nonzero"), stroke_data, elem.attrib.get("opacity", "1"))


def simple_path_geometry(d, current):
    tokens = path_tokens(d)
    # Canonical forms emitted by the ratified vanilla for focal rects/lines.
    if len(tokens) == 10 and tokens[0:3] == ("M", "0", "0") and tokens[3] == "v" and tokens[5] == "h" and tokens[7] == "v" and tokens[9] == "Z":
        h, w, back = Decimal(tokens[4]), Decimal(tokens[6]), Decimal(tokens[8])
        if back != -h:
            raise ValueError("open or malformed rect path")
        return ("rect", point(current, Decimal(0), Decimal(0)), point(current, w, h), "0", "0")
    if len(tokens) == 6 and tokens[0:3] == ("M", "0", "0") and tokens[3] == "l":
        return ("line", point(current, Decimal(0), Decimal(0)), point(current, Decimal(tokens[4]), Decimal(tokens[5])))
    commands = list(canonical_path(tokens, current))
    while len(commands) > 1 and commands[0][0] == "M" and commands[1][0] == "M":
        commands.pop(0)
    return ("path",) + normalize_closed_path(tuple(commands))


ARITY = {"M": 2, "L": 2, "H": 1, "V": 1, "C": 6, "S": 4, "Q": 4, "T": 2, "A": 7, "Z": 0}


def canonical_path(tokens, transform):
    out, i, command = [], 0, None
    x = y = Decimal(0)
    start = (x, y)
    last_cubic = last_quad = None
    while i < len(tokens):
        if tokens[i].isalpha():
            command = tokens[i]; i += 1
        if command is None or command.upper() not in ARITY:
            raise ValueError("unsupported path command")
        upper, relative = command.upper(), command.islower()
        if upper == "Z":
            out.append(("Z",)); x, y = start; command = None; last_cubic = last_quad = None
            continue
        arity = ARITY[upper]
        if i + arity > len(tokens) or any(token.isalpha() for token in tokens[i:i+arity]):
            raise ValueError("malformed path")
        values = [Decimal(token) for token in tokens[i:i+arity]]; i += arity
        def xy(a, b):
            return (a + x, b + y) if relative else (a, b)
        if upper == "M":
            x, y = xy(values[0], values[1]); start = (x, y); out.append(("M",) + point(transform, x, y)); command = "l" if relative else "L"
        elif upper == "L":
            x, y = xy(values[0], values[1]); out.append(("L",) + point(transform, x, y))
        elif upper == "H":
            x = values[0] + x if relative else values[0]; out.append(("L",) + point(transform, x, y))
        elif upper == "V":
            y = values[0] + y if relative else values[0]; out.append(("L",) + point(transform, x, y))
        elif upper == "C":
            c1 = xy(values[0], values[1]); c2 = xy(values[2], values[3]); x, y = xy(values[4], values[5]); last_cubic = c2; last_quad = None
            out.append(("C",) + point(transform, *c1) + point(transform, *c2) + point(transform, x, y))
        elif upper == "S":
            c1 = (2*x-last_cubic[0], 2*y-last_cubic[1]) if last_cubic else (x, y); c2 = xy(values[0], values[1]); x, y = xy(values[2], values[3]); last_cubic = c2; last_quad = None
            out.append(("C",) + point(transform, *c1) + point(transform, *c2) + point(transform, x, y))
        elif upper == "Q":
            control = xy(values[0], values[1]); x, y = xy(values[2], values[3]); last_quad = control; last_cubic = None
            out.append(("Q",) + point(transform, *control) + point(transform, x, y))
        elif upper == "T":
            control = (2*x-last_quad[0], 2*y-last_quad[1]) if last_quad else (x, y); x, y = xy(values[0], values[1]); last_quad = control; last_cubic = None
            out.append(("Q",) + point(transform, *control) + point(transform, x, y))
        elif upper == "A":
            x, y = xy(values[5], values[6]); last_cubic = last_quad = None
            out.append(("A", str(values[0].normalize()), str(values[1].normalize()), str(values[2].normalize()), str(values[3].normalize()), str(values[4].normalize())) + point(transform, x, y))
        if upper not in ("C", "S", "Q", "T"):
            last_cubic = last_quad = None
    return tuple(out)


def recognize_ellipse(geometry):
    if not geometry or geometry[0] != "path":
        return None
    commands = geometry[1:]
    kinds = [item[0] for item in commands]
    if kinds == ["M", "C", "C", "C", "C"]:
        left = tuple(Decimal(v) for v in commands[0][1:3])
        curves = commands[1:]
    elif kinds == ["M", "C", "C", "C", "C", "Z"]:
        left = tuple(Decimal(v) for v in commands[0][1:3])
        curves = commands[1:5]
    else:
        return None
    top = tuple(Decimal(v) for v in curves[0][-2:])
    right = tuple(Decimal(v) for v in curves[1][-2:])
    bottom = tuple(Decimal(v) for v in curves[2][-2:])
    close = tuple(Decimal(v) for v in curves[3][-2:])
    center = ((left[0] + right[0]) / 2, (top[1] + bottom[1]) / 2)
    epsilon = Decimal("0.000001")
    cardinal = (
        abs(left[1] - center[1]) <= epsilon
        and abs(right[1] - center[1]) <= epsilon
        and abs(top[0] - center[0]) <= epsilon
        and abs(bottom[0] - center[0]) <= epsilon
        and all(abs(a-b) <= epsilon for a, b in zip(left, close))
    )
    if not cardinal:
        return None
    return ("ellipse", tuple(str(v.normalize()) for v in center), str((abs(right[0]-left[0]) / Decimal(2)).normalize()), str((abs(bottom[1]-top[1]) / Decimal(2)).normalize()))


def normalize_closed_path(commands):
    commands = list(commands)
    if not commands or commands[-1][0] != "Z":
        return tuple(commands)
    while len(commands) > 1 and commands[0][0] == "M" and commands[1][0] == "M":
        commands.pop(0)
    if not commands or commands[0][0] != "M" or any(item[0] == "M" for item in commands[1:-1]):
        return tuple(commands)
    start = commands[0][1:3]
    segments = commands[1:-1]
    current = start
    starts = []
    for segment in segments:
        starts.append(current)
        current = segment[-2:]
    if current != start:
        starts.append(current)
        segments.append(("L",) + start)
    index = min(range(len(starts)), key=lambda n: tuple(Decimal(v) for v in starts[n]))
    rotated = segments[index:] + segments[:index]
    return (("M",) + starts[index], *rotated, ("Z",))


def deep_equal(a, b, tolerance):
    if a is None or b is None:
        return a is b
    if isinstance(a, (tuple, list)) and isinstance(b, (tuple, list)):
        return len(a) == len(b) and all(deep_equal(x, y, tolerance) for x, y in zip(a, b))
    try:
        return abs(Decimal(a) - Decimal(b)) <= tolerance
    except (ArithmeticError, ValueError):
        return a == b


def morphology(path: pathlib.Path) -> dict:
    root = ET.parse(path).getroot()
    if local(root.tag) != "svg":
        return {"state": "Unknown", "reason": "root is not svg"}
    try:
        width = number(root.attrib["width"])
        height = number(root.attrib["height"])
        symbols = {}
        for elem in root.iter():
            if local(elem.tag) == "symbol":
                ident = elem.attrib.get("id")
                paths = [child for child in elem if local(child.tag) == "path"]
                if not ident or len(paths) != 1:
                    return {"state": "Unknown", "reason": "unsupported symbol"}
                symbols[ident] = path_tokens(paths[0].attrib["d"])

        paints = []

        def visit(elem, current, in_defs=False):
            tag = local(elem.tag)
            if tag == "defs":
                return
            if tag == "g":
                child_matrix = compose(current, matrix(elem.attrib.get("transform")))
                for child in elem:
                    visit(child, child_matrix)
                return
            if tag == "use":
                href = next((value for key, value in elem.attrib.items() if local(key) == "href"), None)
                if not href or not href.startswith("#") or href[1:] not in symbols:
                    raise ValueError("broken local use reference")
                placed = translate(current, number(elem.attrib.get("x", "0")), number(elem.attrib.get("y", "0")))
                paints.append(("glyph", tuple(str(x.normalize()) for x in placed), elem.attrib.get("fill", "#000000"), elem.attrib.get("fill-rule", "nonzero"), symbols[href[1:]]))
                return
            if tag == "rect" and elem.attrib.get("fill") == "#ffffff":
                if number(elem.attrib.get("x", "0")) == 0 and number(elem.attrib.get("y", "0")) == 0:
                    paints.append(("page-background", "#ffffff"))
                    return
            if tag == "path" and elem.attrib.get("fill") == "#ffffff" and current == matrix(None):
                paints.append(("page-background", "#ffffff"))
                return
            child_matrix = compose(current, matrix(elem.attrib.get("transform")))
            if tag == "rect":
                x, y = number(elem.attrib.get("x", "0")), number(elem.attrib.get("y", "0"))
                w, h = number(elem.attrib["width"]), number(elem.attrib["height"])
                geom = ("rect", point(child_matrix, x, y), point(child_matrix, x+w, y+h), elem.attrib.get("rx", "0"), elem.attrib.get("ry", "0"))
                paints.append(shape_paint(elem, geom))
                return
            if tag == "line":
                geom = ("line", point(child_matrix, number(elem.attrib.get("x1", "0")), number(elem.attrib.get("y1", "0"))), point(child_matrix, number(elem.attrib.get("x2", "0")), number(elem.attrib.get("y2", "0"))))
                paints.append(shape_paint(elem, geom))
                return
            if tag == "ellipse":
                cx, cy = number(elem.attrib.get("cx", "0")), number(elem.attrib.get("cy", "0"))
                rx, ry = number(elem.attrib["rx"]), number(elem.attrib["ry"])
                # Analytic identity; vanilla path ellipses are recognized below.
                geom = ("ellipse", point(child_matrix, cx, cy), str(rx.normalize()), str(ry.normalize()))
                paints.append(shape_paint(elem, geom))
                return
            if tag == "path":
                geometry = simple_path_geometry(elem.attrib["d"], child_matrix)
                geometry = recognize_ellipse(geometry) or geometry
                paints.append(shape_paint(elem, geometry))
                return
            if tag == "svg":
                for child in elem:
                    visit(child, current)
                return
            raise ValueError(f"unsupported painted tag: {tag}")

        visit(root, matrix(None))
        return {"state": "Preserved", "width": str(width), "height": str(height), "paints": paints}
    except (KeyError, ValueError, ArithmeticError) as exc:
        return {"state": "Unknown", "reason": str(exc)}


def compare(left: pathlib.Path, right: pathlib.Path, tolerance=Decimal("0.00000001")) -> dict:
    a, b = morphology(left), morphology(right)
    if a["state"] == "Unknown" or b["state"] == "Unknown":
        return {"verdict": "Unknown", "left": a, "right": b}
    width_delta = abs(Decimal(a["width"]) - Decimal(b["width"]))
    height_delta = abs(Decimal(a["height"]) - Decimal(b["height"]))
    same = width_delta <= tolerance and height_delta <= tolerance and deep_equal(a["paints"], b["paints"], tolerance)
    return {"verdict": "Preserved" if same else "Violated", "width_delta": str(width_delta), "height_delta": str(height_delta), "left": a, "right": b}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("left", type=pathlib.Path)
    parser.add_argument("right", type=pathlib.Path)
    args = parser.parse_args()
    print(json.dumps(compare(args.left, args.right), indent=2))


if __name__ == "__main__":
    main()
