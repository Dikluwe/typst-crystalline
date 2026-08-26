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
    match = MATRIX.fullmatch(raw)
    if match:
        values = tuple(Decimal(item) for item in re.split(r"[ ,]+", match.group(1).strip()))
        if len(values) != 6:
            raise ValueError(f"invalid matrix: {value}")
        return values
    match = TRANSLATE.fullmatch(raw)
    if match:
        values = tuple(Decimal(item) for item in re.split(r"[ ,]+", match.group(1).strip()))
        if len(values) not in (1, 2):
            raise ValueError(f"invalid translate: {value}")
        return (Decimal(1), Decimal(0), Decimal(0), Decimal(1), values[0], values[-1] if len(values) == 2 else Decimal(0))
    raise ValueError(f"unsupported transform: {value}")


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
    raise ValueError("unsupported path geometry")


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
            if tag == "path":
                paints.append(shape_paint(elem, simple_path_geometry(elem.attrib["d"], child_matrix)))
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


def compare(left: pathlib.Path, right: pathlib.Path, tolerance=Decimal("0.000000001")) -> dict:
    a, b = morphology(left), morphology(right)
    if a["state"] == "Unknown" or b["state"] == "Unknown":
        return {"verdict": "Unknown", "left": a, "right": b}
    width_delta = abs(Decimal(a["width"]) - Decimal(b["width"]))
    height_delta = abs(Decimal(a["height"]) - Decimal(b["height"]))
    same = width_delta <= tolerance and height_delta <= tolerance and a["paints"] == b["paints"]
    return {"verdict": "Preserved" if same else "Violated", "width_delta": str(width_delta), "height_delta": str(height_delta), "left": a, "right": b}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("left", type=pathlib.Path)
    parser.add_argument("right", type=pathlib.Path)
    args = parser.parse_args()
    print(json.dumps(compare(args.left, args.right), indent=2))


if __name__ == "__main__":
    main()
