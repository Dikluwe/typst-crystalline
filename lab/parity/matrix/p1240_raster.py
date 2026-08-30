#!/usr/bin/env python3
"""P1240: raster SVG local derivado apenas do grafo e geometria declarada."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
import subprocess
import tempfile
import xml.etree.ElementTree as ET
from dataclasses import dataclass
from pathlib import Path

NUMBER = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
TRANSFORM = re.compile(r"([a-zA-Z]+)\s*\(([^)]*)\)")
URL = re.compile(r"url\(#([^)]+)\)")


@dataclass(frozen=True)
class Box:
    x0: float
    y0: float
    x1: float
    y1: float

    def union(self, other: "Box") -> "Box":
        return Box(min(self.x0, other.x0), min(self.y0, other.y0),
                   max(self.x1, other.x1), max(self.y1, other.y1))

    def intersect(self, other: "Box") -> "Box | None":
        out = Box(max(self.x0, other.x0), max(self.y0, other.y0),
                  min(self.x1, other.x1), min(self.y1, other.y1))
        return out if out.x1 > out.x0 and out.y1 > out.y0 else None


IDENTITY = (1.0, 0.0, 0.0, 1.0, 0.0, 0.0)


def compose(outer, inner):
    a, b, c, d, e, f = outer
    g, h, i, j, k, l = inner
    return (a*g+c*h, b*g+d*h, a*i+c*j, b*i+d*j,
            a*k+c*l+e, b*k+d*l+f)


def transform(value: str | None):
    current = IDENTITY
    for name, raw in TRANSFORM.findall(value or ""):
        nums = [float(x) for x in re.findall(NUMBER, raw)]
        if name == "matrix" and len(nums) == 6:
            part = tuple(nums)
        elif name == "translate" and len(nums) in (1, 2):
            part = (1.0, 0.0, 0.0, 1.0, nums[0], nums[1] if len(nums) == 2 else 0.0)
        elif name == "scale" and len(nums) in (1, 2):
            part = (nums[0], 0.0, 0.0, nums[-1], 0.0, 0.0)
        elif name == "rotate" and len(nums) in (1, 3):
            angle = math.radians(nums[0]); co, si = math.cos(angle), math.sin(angle)
            rotation = (co, si, -si, co, 0.0, 0.0)
            if len(nums) == 3:
                x, y = nums[1:]
                part = compose((1, 0, 0, 1, x, y), compose(rotation, (1, 0, 0, 1, -x, -y)))
            else:
                part = rotation
        else:
            raise ValueError(f"unsupported transform {name}({raw})")
        current = compose(current, part)
    return current


def point(matrix, x, y):
    a, b, c, d, e, f = matrix
    return a*x+c*y+e, b*x+d*y+f


def rect_box(elem: ET.Element, matrix) -> Box:
    x = float(elem.get("x", 0)); y = float(elem.get("y", 0))
    width = float(elem.get("width", 0)); height = float(elem.get("height", 0))
    points = [point(matrix, x, y), point(matrix, x+width, y),
              point(matrix, x, y+height), point(matrix, x+width, y+height)]
    box = Box(min(p[0] for p in points), min(p[1] for p in points),
              max(p[0] for p in points), max(p[1] for p in points))
    stroke = float(elem.get("stroke-width", 1)) if elem.get("stroke", "none") != "none" else 0.0
    if stroke:
        a, b, c, d, _, _ = matrix
        ex = stroke * math.hypot(a, c) / 2; ey = stroke * math.hypot(b, d) / 2
        box = Box(box.x0-ex, box.y0-ey, box.x1+ex, box.y1+ey)
    return box


def local_name(elem: ET.Element) -> str:
    return elem.tag.rsplit("}", 1)[-1]


def graph_box(root: ET.Element) -> Box:
    ids = {elem.get("id"): elem for elem in root.iter() if elem.get("id")}

    def visit(elem: ET.Element, parent, in_defs=False) -> Box | None:
        name = local_name(elem)
        current = compose(parent, transform(elem.get("transform")))
        if name == "defs" and not in_defs:
            return None
        if name == "use":
            href = elem.get("href") or elem.get("{http://www.w3.org/1999/xlink}href")
            if not href or not href.startswith("#") or href[1:] not in ids:
                raise ValueError("broken use reference")
            offset = (1, 0, 0, 1, float(elem.get("x", 0)), float(elem.get("y", 0)))
            box = visit(ids[href[1:]], compose(current, offset), True)
        elif name == "rect":
            box = rect_box(elem, current)
        elif name in {"svg", "g", "mask", "clipPath"}:
            boxes = [visit(child, current, in_defs or name in {"mask", "clipPath"}) for child in elem]
            boxes = [item for item in boxes if item is not None]
            box = None if not boxes else boxes[0]
            for item in boxes[1:]:
                box = box.union(item) if box else item
        else:
            box = None
        mask = elem.get("mask")
        if box is not None and mask:
            match = URL.fullmatch(mask.strip())
            if not match or match.group(1) not in ids:
                raise ValueError("broken mask reference")
            mask_box = visit(ids[match.group(1)], parent, True)
            box = box.intersect(mask_box) if mask_box else None
        return box

    result = visit(root, IDENTITY)
    if result is None or result.x1 <= result.x0 or result.y1 <= result.y0:
        raise ValueError("empty or unsupported painted geometry")
    return result


def normalized_svg(path: Path) -> tuple[bytes, Box]:
    root = ET.fromstring(path.read_bytes())
    box = graph_box(root)
    root.set("viewBox", f"{box.x0:.9g} {box.y0:.9g} {box.x1-box.x0:.9g} {box.y1-box.y0:.9g}")
    root.set("width", f"{box.x1-box.x0:.9g}")
    root.set("height", f"{box.y1-box.y0:.9g}")
    return ET.tostring(root, encoding="utf-8"), box


def render(path: Path, renderer: Path, scale: int) -> dict[str, object]:
    data, box = normalized_svg(path)
    with tempfile.TemporaryDirectory(prefix="p1240-") as directory:
        source = Path(directory) / "normalized.svg"
        output = Path(directory) / "raster.png"
        source.write_bytes(data)
        argv = [str(renderer), "--zoom", str(scale), "--output", str(output), str(source)]
        cp = subprocess.run(argv, capture_output=True)
        if cp.returncode:
            raise RuntimeError(cp.stderr.decode(errors="replace"))
        raster = output.read_bytes()
    return {"scale": scale, "sha256": hashlib.sha256(raster).hexdigest(),
            "box": [box.x0, box.y0, box.x1, box.y1],
            "declared_inputs": [str(path.resolve()), str(renderer.resolve())],
            "renderer_sha256": hashlib.sha256(renderer.read_bytes()).hexdigest(),
            "stdout_sha256": hashlib.sha256(cp.stdout).hexdigest(),
            "stderr_sha256": hashlib.sha256(cp.stderr).hexdigest(),
            "exit_status": cp.returncode,
            "argv_shape": [renderer.name, "--zoom", str(scale), "--output", "<tmp.png>", "<tmp.svg>"]}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--left", type=Path, required=True)
    parser.add_argument("--right", type=Path, required=True)
    parser.add_argument("--renderer", type=Path, default=Path("/home/linuxbrew/.linuxbrew/bin/rsvg-convert"))
    parser.add_argument("--reverse", action="store_true")
    args = parser.parse_args()
    rows = []
    scales = (4, 2, 1) if args.reverse else (1, 2, 4)
    for scale in scales:
        left = render(args.left, args.renderer, scale)
        right = render(args.right, args.renderer, scale)
        rows.append({"scale": scale, "left": left, "right": right,
                     "verdict": "Preserved" if left["sha256"] == right["sha256"] else "Violated"})
    rows.sort(key=lambda row: row["scale"])
    print(json.dumps({"rows": rows, "reverse_stable": all(
        row["left"]["sha256"] == row["right"]["sha256"] for row in rows)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
