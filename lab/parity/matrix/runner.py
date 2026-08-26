#!/usr/bin/env python3
"""P1137 parity matrix runner. Uses only the Python standard library."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
import pathlib
import re
import subprocess
import tempfile
import zlib
from collections import Counter
from html.parser import HTMLParser
from xml.etree import ElementTree as ET

from svg_morphology import compare as compare_svg_morphology

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[2]
DEFAULT_VANILLA = ROOT / "lab/typst-original/target/release/typst"
DEFAULT_CRYSTALLINE = ROOT / "target/release/typst"
STATES = {"MATCH", "DIFF", "ABSENT", "PARTIAL", "ERROR", "UNMEASURED", "NOT_APPLICABLE"}
CLASSES = {"LANGUAGE_SEMANTICS", "LANGUAGE_SYNTAX", "LANGUAGE_MORPHOLOGY", "PUBLIC_DIAGNOSTIC", "PUBLIC_CLI", "PUBLIC_FORMAT", "MECHANICS_ALLOWED", "HARNESS_DEFECT", "BASELINE_DEFECT", "UNKNOWN", None}
AXES = set("SEBILXC")


def load_manifest(path: pathlib.Path) -> dict:
    # JSON is a strict subset of YAML 1.2, avoiding an undeclared PyYAML dependency.
    return json.loads(path.read_text(encoding="utf-8"))


def validate_manifest(data: dict) -> list[str]:
    errors: list[str] = []
    if data.get("schema_version") != 1:
        errors.append("schema_version must be 1")
    if data.get("vanilla_revision") != "a51e02804":
        errors.append("vanilla_revision must be a51e02804")
    cases = data.get("cases")
    if not isinstance(cases, list):
        return errors + ["cases must be an array"]
    required = {"id", "eixo", "feature", "caso", "fonte_typ", "observavel", "oraculo", "cristalino", "estado", "classe", "proveniencia_vanilla", "proveniencia_cristalino", "comando_reproducao", "artefactos", "nota", "comparison"}
    seen: set[str] = set()
    for index, case in enumerate(cases):
        prefix = f"cases[{index}]"
        missing = required - set(case)
        if missing:
            errors.append(f"{prefix}: missing {sorted(missing)}")
        case_id = case.get("id")
        if case_id in seen:
            errors.append(f"{prefix}: duplicate id {case_id}")
        seen.add(case_id)
        if case.get("eixo") not in AXES:
            errors.append(f"{prefix}: invalid eixo")
        if case.get("estado") not in STATES:
            errors.append(f"{prefix}: invalid estado")
        if case.get("classe") not in CLASSES:
            errors.append(f"{prefix}: invalid classe")
        if case.get("estado") != "MATCH" and case.get("classe") is None:
            errors.append(f"{prefix}: non-MATCH requires classe")
        for side in ("oraculo", "cristalino"):
            if not isinstance(case.get(side, {}).get("args"), list):
                errors.append(f"{prefix}: {side}.args must be an array")
            side_env = case.get(side, {}).get("env", {})
            if not isinstance(side_env, dict) or not all(
                isinstance(key, str) and isinstance(value, str)
                for key, value in side_env.items()
            ):
                errors.append(f"{prefix}: {side}.env must be a string map")
    return errors


def invoke(binary: pathlib.Path, args: list[str], source: pathlib.Path | None, output: pathlib.Path, extra_env: dict[str, str] | None = None) -> dict:
    output.parent.mkdir(parents=True, exist_ok=True)
    substitutions = {
        "source": str(source or ""),
        "output": str(output),
        "root": str(ROOT),
        "fixtures": str(HERE / "fixtures"),
    }
    command = [str(binary), *(arg.format(**substitutions) for arg in args)]
    env = os.environ.copy()
    env.update({key: value.format(**substitutions) for key, value in (extra_env or {}).items()})
    try:
        proc = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, text=True, timeout=30, check=False)
        artifacts = sorted(output.parent.glob(output.name + ".*"))
        return {"command": command, "exit_code": proc.returncode, "stdout": proc.stdout, "stderr": proc.stderr, "artifact_exists": bool(artifacts), "artifact_paths": [str(path) for path in artifacts]}
    except (OSError, subprocess.TimeoutExpired) as exc:
        return {"command": command, "exit_code": None, "stdout": "", "stderr": str(exc), "artifact_exists": False, "harness_error": True}


def normalize_diagnostic(text: str) -> list[str]:
    text = re.sub(r"\x1b\[[0-9;]*m", "", text)
    text = text.replace(str(ROOT), "<ROOT>")
    return [line.rstrip() for line in text.splitlines() if line.strip()]


def typed_value(text: str):
    def tag(value):
        if value is None: return ["none"]
        if isinstance(value, bool): return ["bool", value]
        if isinstance(value, int): return ["int", value]
        if isinstance(value, float): return ["float", value]
        if isinstance(value, str): return ["string", value]
        if isinstance(value, list): return ["array", [tag(item) for item in value]]
        if isinstance(value, dict): return ["dict", [[key, tag(value[key])] for key in sorted(value)]]
        raise ValueError(type(value).__name__)
    return tag(json.loads(text))


class _HtmlTree(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.events = []
    def handle_starttag(self, tag, attrs): self.events.append(("start", tag, tuple(sorted(attrs))))
    def handle_endtag(self, tag): self.events.append(("end", tag))
    def handle_data(self, data):
        if data.strip(): self.events.append(("text", data))


def artifact(result: dict, suffix: str) -> pathlib.Path:
    paths = [pathlib.Path(path) for path in result.get("artifact_paths", []) if path.endswith(suffix)]
    if len(paths) != 1: raise ValueError(f"expected one {suffix} artifact, found {len(paths)}")
    return paths[0]


def semantic_tree(path: pathlib.Path):
    if path.suffix == ".svg":
        def node(elem):
            tag = elem.tag.rsplit("}", 1)[-1]
            attrs = tuple(sorted((key.rsplit("}", 1)[-1], value) for key, value in elem.attrib.items() if key != "id"))
            return tag, attrs, (elem.text or "").strip(), tuple(node(child) for child in elem)
        return node(ET.parse(path).getroot())
    parser = _HtmlTree()
    parser.feed(path.read_text(encoding="utf-8"))
    return parser.events


def pdf_geometry(path: pathlib.Path):
    proc = subprocess.run(["pdfinfo", str(path)], capture_output=True, text=True, check=True)
    pages = re.search(r"^Pages:\s+(\d+)", proc.stdout, re.M)
    size = re.search(r"^Page size:\s+([0-9.]+) x ([0-9.]+) pts", proc.stdout, re.M)
    if not pages or not size: raise ValueError("pdfinfo lacks pages/page size")
    return int(pages.group(1)), round(float(size.group(1)), 3), round(float(size.group(2)), 3)


def pdf_observables(path: pathlib.Path):
    info = pdf_geometry(path)
    text = subprocess.run(["pdftotext", "-layout", str(path), "-"], capture_output=True, text=True, check=True).stdout
    fonts = subprocess.run(["pdffonts", str(path)], capture_output=True, text=True, check=True).stdout.splitlines()[2:]
    font_names = sorted({line.split()[0].split("+")[-1] for line in fonts if line.split()})
    return info, text.rstrip(), font_names


def png_pixels(path: pathlib.Path):
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n": raise ValueError("not PNG")
    pos, chunks = 8, {}
    compressed = bytearray()
    while pos < len(data):
        length = int.from_bytes(data[pos:pos+4], "big"); kind = data[pos+4:pos+8]; body = data[pos+8:pos+8+length]; pos += 12 + length
        if kind == b"IHDR": chunks["IHDR"] = body
        elif kind == b"IDAT": compressed.extend(body)
        elif kind == b"IEND": break
    width = int.from_bytes(chunks["IHDR"][:4], "big"); height = int.from_bytes(chunks["IHDR"][4:8], "big")
    depth, color = chunks["IHDR"][8], chunks["IHDR"][9]
    if depth != 8 or color not in (2, 6): raise ValueError(f"unsupported PNG depth/color {depth}/{color}")
    bpp = 3 if color == 2 else 4; stride = width * bpp; raw = zlib.decompress(bytes(compressed)); rows = []; offset = 0; prior = bytearray(stride)
    for _ in range(height):
        mode = raw[offset]; scan = bytearray(raw[offset+1:offset+1+stride]); offset += stride + 1
        for i in range(stride):
            left = scan[i-bpp] if i >= bpp else 0; up = prior[i]; upper_left = prior[i-bpp] if i >= bpp else 0
            if mode == 1: scan[i] = (scan[i] + left) & 255
            elif mode == 2: scan[i] = (scan[i] + up) & 255
            elif mode == 3: scan[i] = (scan[i] + ((left + up)//2)) & 255
            elif mode == 4:
                p = left + up - upper_left; pa = abs(p-left); pb = abs(p-up); pc = abs(p-upper_left); scan[i] = (scan[i] + (left if pa <= pb and pa <= pc else up if pb <= pc else upper_left)) & 255
            elif mode != 0: raise ValueError(f"unsupported PNG filter {mode}")
        rows.append(bytes(scan)); prior = scan
    pixels = b"".join(rows)
    if color == 2: pixels = b"".join(pixels[i:i+3] + b"\xff" for i in range(0, len(pixels), 3))
    return width, height, pixels


def write_png_rgba(path: pathlib.Path, width: int, height: int, pixels: bytes):
    def chunk(kind, body):
        return len(body).to_bytes(4, "big") + kind + body + (zlib.crc32(kind + body) & 0xffffffff).to_bytes(4, "big")
    rows = b"".join(b"\0" + pixels[y*width*4:(y+1)*width*4] for y in range(height))
    ihdr = width.to_bytes(4, "big") + height.to_bytes(4, "big") + bytes([8, 6, 0, 0, 0])
    path.write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", ihdr) + chunk(b"IDAT", zlib.compress(rows)) + chunk(b"IEND", b""))


def comparison_observables(comparison: str, oracle: dict, crystalline: dict, artifact_dir: pathlib.Path):
    if comparison == "diagnostic":
        return {"oracle": normalize_diagnostic(oracle["stderr"]), "crystalline": normalize_diagnostic(crystalline["stderr"])}
    if comparison == "typed_value":
        return {"oracle": typed_value(oracle["stdout"]), "crystalline": typed_value(crystalline["stdout"])}
    if comparison == "semantic_tree":
        suffix = ".html" if any(path.endswith(".html") for path in oracle.get("artifact_paths", [])) else ".svg"
        if suffix == ".svg":
            return compare_svg_morphology(artifact(oracle, suffix), artifact(crystalline, suffix))
        return {"oracle": semantic_tree(artifact(oracle, suffix)), "crystalline": semantic_tree(artifact(crystalline, suffix))}
    if comparison == "geometry":
        return {"unit": "pt", "rounding": 0.001, "oracle": pdf_geometry(artifact(oracle, ".pdf")), "crystalline": pdf_geometry(artifact(crystalline, ".pdf"))}
    if comparison == "pdf_observables":
        return {"oracle": pdf_observables(artifact(oracle, ".pdf")), "crystalline": pdf_observables(artifact(crystalline, ".pdf"))}
    if comparison == "raster":
        ow, oh, op = png_pixels(artifact(oracle, ".png")); cw, ch, cp = png_pixels(artifact(crystalline, ".png"))
        summary = {"oracle_size": [ow, oh], "crystalline_size": [cw, ch]}
        if (ow, oh) == (cw, ch):
            deltas = [abs(a-b) for a, b in zip(op, cp)]
            pixel_diffs = sum(any(op[i+j] != cp[i+j] for j in range(4)) for i in range(0, len(op), 4))
            diff = bytes(max(abs(op[i+j]-cp[i+j]) for j in range(3)) for i in range(0, len(op), 4) for _ in range(3) for __ in [0])
            rgba = b"".join(diff[i:i+3] + b"\xff" for i in range(0, len(diff), 3))
            diff_path = artifact_dir / "pixel-diff.png"; write_png_rgba(diff_path, ow, oh, rgba)
            summary.update({"different_pixels": pixel_diffs, "total_pixels": ow*oh, "max_channel_delta": max(deltas, default=0), "mean_channel_delta": sum(deltas)/len(deltas) if deltas else 0, "diff_artifact": str(diff_path)})
        return summary
    return None


def classify(comparison: str, oracle: dict, crystalline: dict) -> tuple[str, str | None]:
    if oracle.get("harness_error") or crystalline.get("harness_error"):
        return "ERROR", "HARNESS_DEFECT"
    if comparison == "exit_code":
        return ("MATCH", None) if oracle["exit_code"] == crystalline["exit_code"] else ("DIFF", "LANGUAGE_SYNTAX")
    if comparison == "diagnostic":
        same = oracle["exit_code"] == crystalline["exit_code"] and normalize_diagnostic(oracle["stderr"]) == normalize_diagnostic(crystalline["stderr"])
        return ("MATCH", None) if same else ("DIFF", "PUBLIC_DIAGNOSTIC")
    if comparison == "typed_value":
        try: same = oracle["exit_code"] == crystalline["exit_code"] == 0 and typed_value(oracle["stdout"]) == typed_value(crystalline["stdout"])
        except (ValueError, json.JSONDecodeError): return "ERROR", "HARNESS_DEFECT"
        return ("MATCH", None) if same else ("DIFF", "LANGUAGE_SEMANTICS")
    if comparison in {"semantic_tree", "geometry", "raster", "pdf_observables"}:
        try:
            suffix = ({"geometry": ".pdf", "raster": ".png", "pdf_observables": ".pdf"}.get(comparison)
                      or (".html" if any(path.endswith(".html") for path in oracle.get("artifact_paths", [])) else ".svg"))
            left, right = artifact(oracle, suffix), artifact(crystalline, suffix)
            if comparison == "semantic_tree" and suffix == ".svg":
                verdict = compare_svg_morphology(left, right)["verdict"]
                if verdict == "Unknown":
                    return "PARTIAL", "UNKNOWN"
                same = oracle["exit_code"] == crystalline["exit_code"] == 0 and verdict == "Preserved"
            else:
                extract = {"semantic_tree": semantic_tree, "geometry": pdf_geometry, "raster": png_pixels, "pdf_observables": pdf_observables}[comparison]
                same = oracle["exit_code"] == crystalline["exit_code"] == 0 and extract(left) == extract(right)
        except (OSError, ValueError, subprocess.SubprocessError, ET.ParseError): return "ERROR", "HARNESS_DEFECT"
        classes = {"semantic_tree": "LANGUAGE_MORPHOLOGY", "geometry": "LANGUAGE_MORPHOLOGY", "raster": "PUBLIC_FORMAT", "pdf_observables": "PUBLIC_FORMAT"}
        return ("MATCH", None) if same else ("DIFF", classes[comparison])
    if comparison == "exact_output":
        same = oracle["exit_code"] == crystalline["exit_code"] and oracle["stdout"] == crystalline["stdout"] and oracle["stderr"] == crystalline["stderr"]
        return ("MATCH", None) if same else ("DIFF", "PUBLIC_CLI")
    if comparison == "semantic_version":
        pattern = re.compile(r"\b\d+\.\d+\.\d+\b")
        oracle_version = pattern.search(oracle["stdout"])
        crystal_version = pattern.search(crystalline["stdout"])
        if not oracle_version or not crystal_version:
            return "ERROR", "HARNESS_DEFECT"
        return ("MATCH", None) if oracle_version.group() == crystal_version.group() else ("DIFF", "LANGUAGE_SEMANTICS")
    if comparison == "cli_surface":
        def surface(stdout: str) -> tuple[set[str], set[str]]:
            commands: set[str] = set()
            options: set[str] = set()
            section = None
            for line in stdout.splitlines():
                if line == "Commands:":
                    section = "commands"
                    continue
                if line == "Options:":
                    section = "options"
                    continue
                if line and not line.startswith(" "):
                    section = None
                if section == "commands" and re.match(r"^  [a-z]", line):
                    commands.add(line.strip().split()[0])
                elif section == "options" and line.startswith("  "):
                    options.update(re.findall(r"--[a-z][a-z-]*", line))
            return commands, options

        oracle_surface = surface(oracle["stdout"])
        crystal_surface = surface(crystalline["stdout"])
        if oracle["exit_code"] != crystalline["exit_code"]:
            return "DIFF", "PUBLIC_CLI"
        if oracle_surface == crystal_surface:
            return "MATCH", None
        command_subset = bool(crystal_surface[0]) and crystal_surface[0] < oracle_surface[0]
        option_subset = crystal_surface[1] <= oracle_surface[1]
        if command_subset and option_subset:
            return "PARTIAL", "PUBLIC_CLI"
        return "DIFF", "PUBLIC_CLI"
    if comparison == "artifact_presence":
        same = oracle["exit_code"] == 0 and crystalline["exit_code"] == 0 and oracle["artifact_exists"] and crystalline["artifact_exists"]
        return ("MATCH", None) if same else ("DIFF", "LANGUAGE_MORPHOLOGY")
    if comparison == "capability":
        if oracle["exit_code"] == 0 and crystalline["exit_code"] != 0:
            return "ABSENT", None
        same = oracle["exit_code"] == crystalline["exit_code"] and oracle["stdout"] == crystalline["stdout"]
        return ("MATCH", None) if same else ("DIFF", None)
    return "ERROR", "HARNESS_DEFECT"


def run_case(case: dict, vanilla: pathlib.Path, crystalline: pathlib.Path, artifact_dir: pathlib.Path) -> dict:
    if case.get("expected_state") in {"UNMEASURED", "NOT_APPLICABLE"}:
        return {"id": case["id"], "eixo": case["eixo"], "estado": case["expected_state"], "classe": case["classe"], "nota": case["nota"]}
    source = HERE / case["fonte_typ"] if case["fonte_typ"] else None
    oracle = invoke(vanilla, case["oraculo"]["args"], source, artifact_dir / "oracle", case["oraculo"].get("env"))
    crystal = invoke(crystalline, case["cristalino"]["args"], source, artifact_dir / "crystalline", case["cristalino"].get("env"))
    state, inferred_class = classify(case["comparison"], oracle, crystal)
    try:
        observed = comparison_observables(case["comparison"], oracle, crystal, artifact_dir)
    except (OSError, ValueError, subprocess.SubprocessError, ET.ParseError, json.JSONDecodeError) as error:
        observed = {"harness_error": str(error)}
    result_class = None if state == "MATCH" else (case["classe"] if case["classe"] not in (None, "UNKNOWN") else inferred_class or "UNKNOWN")
    return {"id": case["id"], "eixo": case["eixo"], "estado": state, "classe": result_class, "expected_state": case.get("expected_state"), "expectation_met": state == case.get("expected_state"), "oracle": oracle, "crystalline": crystal, "observed": observed, "nota": case["nota"]}


def main() -> int:
    parser = argparse.ArgumentParser(description="Run the P1137 Typst parity matrix")
    parser.add_argument("--manifest", type=pathlib.Path, default=HERE / "manifest.yaml")
    parser.add_argument("--vanilla", type=pathlib.Path, default=DEFAULT_VANILLA)
    parser.add_argument("--crystalline", type=pathlib.Path, default=DEFAULT_CRYSTALLINE)
    parser.add_argument("--case")
    parser.add_argument("--output", type=pathlib.Path)
    parser.add_argument("--validate-only", action="store_true")
    args = parser.parse_args()
    manifest = load_manifest(args.manifest)
    errors = validate_manifest(manifest)
    if errors:
        for error in errors:
            print(error)
        return 2
    if args.validate_only:
        print(f"valid manifest: {len(manifest['cases'])} cases")
        return 0
    selected = [case for case in manifest["cases"] if not args.case or case["id"] == args.case]
    if not selected:
        print(f"unknown case: {args.case}")
        return 2
    if args.output:
        artifact_root = args.output.parent / f"{args.output.stem}-artifacts"
        artifact_root.mkdir(parents=True, exist_ok=True)
        results = [run_case(case, args.vanilla.resolve(), args.crystalline.resolve(), artifact_root / case["id"]) for case in selected]
    else:
        with tempfile.TemporaryDirectory(prefix="p1138-") as tmp:
            results = [run_case(case, args.vanilla.resolve(), args.crystalline.resolve(), pathlib.Path(tmp) / case["id"]) for case in selected]
    payload = {
        "generated_at": dt.datetime.now(dt.timezone.utc).isoformat(),
        "vanilla_revision": manifest["vanilla_revision"],
        "vanilla_binary": str(args.vanilla.resolve()),
        "crystalline_binary": str(args.crystalline.resolve()),
        "counts": dict(sorted(Counter(result["estado"] for result in results).items())),
        "results": results,
    }
    rendered = json.dumps(payload, indent=2, ensure_ascii=False) + "\n"
    if args.output:
        args.output.write_text(rendered, encoding="utf-8")
    else:
        print(rendered, end="")
    return 0 if all(result.get("expectation_met", True) for result in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
