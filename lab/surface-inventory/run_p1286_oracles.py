#!/usr/bin/env python3
"""Independent black-box oracle runner for P1286.

The runner freezes observations only from the ratified vanilla executable and
later compares an arbitrary candidate executable without inspecting its code.
Unknown is always a failing result.
"""

from __future__ import annotations

import argparse
import hashlib
import html
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_BASELINE = ROOT / "lab/surface-inventory/p1286-oracle-baseline.json"
VANILLA = Path("/usr/local/bin/typst")
VANILLA_SHA256 = "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
CONTRACT_SHA256 = "16597543965ca13d37fdabf9be184f3477df92da4f6702ba14f0a4ca4918f9bb"
POST_GATE_SHA256 = "27e2f5b931d898396319d7a62875ca377511e93ebe0e8b97cdd4f6817d6663c0"
REQUIRED_TOOLS = ("qpdf", "pdfdetach", "mutool", "pdftotext")


@dataclass(frozen=True)
class Case:
    name: str
    family: str
    mode: str
    source: str
    selector: str | None = None
    optional_capability: str | None = None
    comparison: str = "semantic"


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def run(argv: list[str], cwd: Path) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(argv, cwd=cwd, stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE, check=False)


def text(data: bytes) -> str:
    return data.decode("utf-8", errors="replace")


def primary_error(stderr: bytes) -> str | None:
    match = re.search(r"^error: .+$", text(stderr), flags=re.MULTILINE)
    if not match:
        return None
    return re.sub(r"/tmp/p1286-oracles-[^/]+/", "<WORK>/", match.group(0))


def version_of(executable: Path) -> str:
    result = run([str(executable), "--version"], ROOT)
    return text(result.stdout).strip() or text(result.stderr).strip()


def tool_version(name: str) -> str:
    executable = shutil.which(name)
    if executable is None:
        return "missing"
    alternatives = ([executable, "--version"], [executable, "-v"],
                    [executable, "-version"])
    for argv in alternatives:
        result = run(list(argv), ROOT)
        output = (text(result.stdout) + text(result.stderr)).strip()
        if output:
            return output.splitlines()[0]
    return "present-version-unknown"


def cases() -> list[Case]:
    smartquote = [
        Case("smartquote_default_de", "smartquote", "pdf_text",
             '#set text(lang: "de")\n#smartquote()Default#smartquote().'),
        Case("smartquote_alternative_de", "smartquote", "pdf_text",
             '#set text(lang: "de")\n#smartquote(alternative: true)Alt#smartquote(alternative: true).'),
        Case("smartquote_explicit_beats_alternative", "smartquote", "pdf_text",
             '#set text(lang: "de")\n#smartquote(alternative: true, quotes: "()")Explicit#smartquote(alternative: true, quotes: "()").'),
        Case("smartquote_disabled_double", "smartquote", "pdf_text",
             '#smartquote(double: true, enabled: false)'),
        Case("smartquote_disabled_single", "smartquote", "pdf_text",
             '#smartquote(double: false, enabled: false)'),
        Case("smartquote_array", "smartquote", "pdf_text",
             '#smartquote(quotes: ("[[", "]]"))Array#smartquote(quotes: ("[[", "]]"))'),
        Case("smartquote_dict_partial", "smartquote", "pdf_text",
             '#set text(lang: "de")\n#set smartquote(alternative: true, quotes: (single: ("<", ">"), double: auto))\n\'Single\' and "Double"'),
        Case("smartquote_error_string_cardinality", "smartquote", "error",
             '#smartquote(quotes: "x")'),
        Case("smartquote_error_array_cardinality", "smartquote", "error",
             '#smartquote(quotes: ("x",))'),
        Case("smartquote_error_dict_key", "smartquote", "error",
             '#smartquote(quotes: (triple: ("x", "y")))'),
    ]

    line = [
        Case("line_positive_origin", "line", "svg",
             '#line(start: (10pt, 20pt), end: (40pt, 50pt))'),
        Case("line_negative_origin", "line", "svg",
             '#line(start: (-10pt, -20pt), end: (30pt, 40pt))'),
        Case("line_inverted_vector", "line", "svg",
             '#line(start: (40pt, 50pt), end: (10pt, 20pt))'),
        Case("line_end_precedence", "line", "svg",
             '#line(start: (10pt, 20pt), end: (40pt, 50pt), length: 99pt, angle: 180deg)'),
        Case("line_length_angle", "line", "svg",
             '#line(start: (10pt, 20pt), length: 30pt, angle: 90deg)'),
        Case("line_default_angle", "line", "svg",
             '#line(start: (10pt, 20pt), length: 30pt)'),
        Case("line_negative_length", "line", "svg",
             '#line(start: (40pt, 50pt), length: -30pt, angle: 0deg)'),
        Case("line_error_dx", "line", "error",
             '#line(start: (10pt, 20pt), end: (40pt, 50pt), dx: 3pt)',
             comparison="baseline_only"),
        Case("line_error_dy", "line", "error",
             '#line(start: (10pt, 20pt), end: (40pt, 50pt), dy: 3pt)',
             comparison="baseline_only"),
        Case("line_error_start_arity", "line", "error",
             '#line(start: (10pt,))'),
    ]

    color_valid = {
        "color_three_default": "rgb(color.mix(red, green, blue))",
        "color_method_three": "rgb(red.mix(green, blue))",
        "color_float_weights": "rgb(color.mix((red, 1), (green, 2), (blue, 3)))",
        "color_ratio_weights": "rgb(color.mix((red, 20%), (green, 30%), (blue, 10%)))",
        "color_zero_weight": "rgb(color.mix((red, 0), (green, 5), (blue, 5)))",
        "color_negative_positive_sum": "rgb(color.mix((red, -1), (green, 1), (blue, 2), space: rgb))",
        "color_one": "rgb(color.mix(red))",
        "color_space_rgb": "rgb(color.mix(red, green, blue, space: rgb))",
        "color_space_oklab": "rgb(color.mix(red, green, blue, space: oklab))",
        "color_space_luma": "rgb(color.mix(red, green, blue, space: luma))",
        "color_space_cmyk": "rgb(color.mix(red, green, blue, space: cmyk))",
    }
    color = [
        Case(name, "color.mix", "query",
             f"#metadata({expression}) <probe>", selector="<probe>")
        for name, expression in color_valid.items()
    ]
    color.extend([
        Case("color_error_zero_colors", "color.mix", "error", '#color.mix()'),
        Case("color_error_zero_sum", "color.mix", "error", '#color.mix((red, 0), (blue, 0))'),
        Case("color_error_cancelled_sum", "color.mix", "error", '#color.mix((red, -1), (blue, 1))'),
        Case("color_error_hsl_n3", "color.mix", "error", '#color.mix(red, green, blue, space: color.hsl)'),
        Case("color_error_hsv_n3", "color.mix", "error", '#color.mix(red, green, blue, space: color.hsv)'),
        Case("color_error_oklch_n3", "color.mix", "error", '#color.mix(red, green, blue, space: oklch)'),
        Case("color_error_bad_pair", "color.mix", "error", '#color.mix((red, green, blue), white)'),
        Case("color_error_weight_type", "color.mix", "error", '#color.mix((red, "heavy"), blue)'),
        Case("color_error_space_type", "color.mix", "error", '#color.mix(red, blue, space: "rgb")'),
    ])

    attach = [
        Case("attach_path", "pdf.attach", "attach_path", '#pdf.attach("payload.txt")'),
        Case("attach_bytes_metadata", "pdf.attach", "attach_bytes",
             '#pdf.attach("virtual.bin", bytes((0, 65, 255)), relationship: "supplement", mime-type: "application/octet-stream", description: "three bytes")'),
        Case("attach_bytes_metadata_a3", "pdf.attach", "attach_a3",
             '#pdf.attach("virtual.bin", bytes((0, 65, 255)), relationship: "supplement", mime-type: "application/octet-stream", description: "three bytes")',
             optional_capability="--pdf-standard"),
        Case("attach_duplicate", "pdf.attach", "error",
             '#pdf.attach("same.bin", bytes((1,)))\n#pdf.attach("same.bin", bytes((2,)))'),
        Case("attach_error_content_data", "pdf.attach", "error", '#pdf.attach("virtual.bin", [hello])'),
        Case("attach_error_string_data", "pdf.attach", "error", '#pdf.attach("virtual.bin", "hello")'),
        Case("attach_error_missing_path", "pdf.attach", "error", '#pdf.attach()'),
        Case("attach_error_missing_file", "pdf.attach", "error", '#pdf.attach("does-not-exist.bin")'),
        Case("attach_error_relationship", "pdf.attach", "error", '#pdf.attach("virtual.bin", bytes((1,)), relationship: "primary")'),
        Case("attach_error_named_data", "pdf.attach", "error", '#pdf.attach("payload.txt", data: bytes((1,)))'),
        Case("attach_error_bytes_path", "pdf.attach", "error", '#pdf.attach(bytes((1,)))'),
        Case("attach_error_mime", "pdf.attach", "error", '#pdf.attach("virtual.bin", bytes((1,)), mime-type: "not a mime")'),
    ]

    artifact = [
        Case("artifact_plain_control", "pdf.artifact", "artifact_plain",
             'plain-before. artifact-secret plain-after.'),
        Case("artifact_default_other", "pdf.artifact", "artifact",
             'plain-before. #pdf.artifact[artifact-secret] plain-after.'),
        Case("artifact_explicit_other", "pdf.artifact", "artifact",
             'plain-before. #pdf.artifact(kind: "other")[artifact-secret] plain-after.'),
        Case("artifact_header", "pdf.artifact", "artifact",
             'plain-before. #pdf.artifact(kind: "header")[artifact-secret] plain-after.'),
        Case("artifact_background_pdf20", "pdf.artifact", "artifact_pdf20",
             'plain-before. #pdf.artifact(kind: "background")[artifact-secret] plain-after.'),
        Case("artifact_tags_disabled", "pdf.artifact", "artifact_no_tags",
             'plain-before. #pdf.artifact[artifact-secret] plain-after.',
             optional_capability="--no-pdf-tags"),
        Case("artifact_error_kind", "pdf.artifact", "error", '#pdf.artifact(kind: "decorative")[x]'),
        Case("artifact_error_missing_body", "pdf.artifact", "error", '#pdf.artifact(kind: "other")'),
        Case("artifact_error_body_type", "pdf.artifact", "error", '#pdf.artifact(42)'),
    ]
    return smartquote + line + color + attach + artifact


def write_fixture(work: Path, case: Case) -> Path:
    path = work / f"{case.name}.typ"
    prefix = '#set page(width: 200pt, height: 200pt, margin: 0pt)\n' if case.mode == "svg" else ""
    path.write_text(prefix + case.source + "\n", encoding="utf-8")
    return path


def compile_cmd(executable: Path, source: Path, output: Path,
                extra: list[str] | None = None) -> list[str]:
    return [str(executable), "compile", *(extra or []), str(source), str(output)]


def html_observation(output: Path) -> dict[str, Any]:
    raw = output.read_text(encoding="utf-8", errors="replace")
    paragraphs = re.findall(r"<p(?:\s[^>]*)?>(.*?)</p>", raw,
                            flags=re.DOTALL | re.IGNORECASE)
    visible = []
    for paragraph in paragraphs:
        stripped = re.sub(r"<[^>]+>", "", paragraph)
        visible.append(html.unescape(stripped).strip())
    return {"paragraphs": visible}


def number(value: str) -> float:
    rounded = round(float(value), 6)
    return 0.0 if rounded == -0.0 else rounded


def svg_observation(output: Path) -> dict[str, Any]:
    raw = output.read_text(encoding="utf-8", errors="replace")
    geometries: list[dict[str, Any]] = []
    for tag in re.findall(r"<path\b[^>]*>", raw, flags=re.IGNORECASE):
        transform_match = re.search(r'transform="translate\(\s*([-+0-9.eE]+)(?:[ ,]+)([-+0-9.eE]+)\s*\)"', tag)
        d_match = re.search(r'\bd="([^"]+)"', tag)
        if not transform_match or not d_match:
            continue
        d = d_match.group(1).strip()
        delta: tuple[float, float] | None = None
        match = re.fullmatch(r"M\s*0\s+0\s*l\s*([-+0-9.eE]+)\s+([-+0-9.eE]+)", d)
        if match:
            delta = (number(match.group(1)), number(match.group(2)))
        match_h = re.fullmatch(r"M\s*0\s+0\s*h\s*([-+0-9.eE]+)", d)
        if match_h:
            delta = (number(match_h.group(1)), 0.0)
        match_v = re.fullmatch(r"M\s*0\s+0\s*v\s*([-+0-9.eE]+)", d)
        if match_v:
            delta = (0.0, number(match_v.group(1)))
        if delta is None:
            continue
        start = (number(transform_match.group(1)), number(transform_match.group(2)))
        geometries.append({
            "start": list(start),
            "delta": list(delta),
            "end": [number(str(start[0] + delta[0])), number(str(start[1] + delta[1]))],
        })
    return {"open_line_geometries": geometries}


def qdf_text(pdf: Path, work: Path) -> tuple[str | None, str | None]:
    qdf = work / f"{pdf.stem}.qdf.pdf"
    result = run(["qpdf", "--qdf", "--object-streams=disable", str(pdf), str(qdf)], work)
    if result.returncode != 0:
        return None, f"qpdf exit {result.returncode}: {text(result.stderr).strip()}"
    return qdf.read_bytes().decode("latin-1", errors="replace"), None


def extracted_text(pdf: Path, work: Path) -> tuple[str | None, str | None]:
    target = work / f"{pdf.stem}.txt"
    result = run(["pdftotext", str(pdf), str(target)], work)
    if result.returncode != 0:
        return None, f"pdftotext exit {result.returncode}: {text(result.stderr).strip()}"
    normalized = " ".join(target.read_text(encoding="utf-8", errors="replace").split())
    return normalized, None


def inspect_attachment(pdf: Path, work: Path, expected: bytes) -> tuple[dict[str, Any] | None, str | None]:
    listing = run(["pdfdetach", "-list", str(pdf)], work)
    if listing.returncode != 0:
        return None, f"pdfdetach -list exit {listing.returncode}: {text(listing.stderr).strip()}"
    list_text = text(listing.stdout) + text(listing.stderr)
    count_match = re.search(r"(\d+) embedded files?", list_text)
    names = re.findall(r"^\s*\d+:\s+(.+?)\s*$", list_text, flags=re.MULTILINE)
    extracted = work / f"{pdf.stem}.embedded"
    saved = run(["pdfdetach", "-save", "1", "-o", str(extracted), str(pdf)], work)
    if saved.returncode != 0 or not extracted.exists():
        return None, f"pdfdetach extraction failed with exit {saved.returncode}: {text(saved.stderr).strip()}"
    qdf, error = qdf_text(pdf, work)
    if error:
        return None, error
    mutool = run(["mutool", "show", str(pdf), "grep", "EmbeddedFile"], work)
    if mutool.returncode != 0:
        return None, f"mutool show exit {mutool.returncode}: {text(mutool.stderr).strip()}"
    extracted_bytes = extracted.read_bytes()
    assert qdf is not None
    return {
        "embedded_count": int(count_match.group(1)) if count_match else None,
        "names": names,
        "extracted_sha256": sha256_bytes(extracted_bytes),
        "extracted_hex": extracted_bytes.hex(),
        "matches_expected_payload": extracted_bytes == expected,
        "has_embedded_file": "/EmbeddedFile" in qdf,
        "has_filespec": "/Filespec" in qdf,
        "has_name_tree": "/EmbeddedFiles" in qdf,
        "has_description": "/Desc" in qdf,
        "has_octet_stream_mime": bool(re.search(r"/Subtype\s*/application#2[fF]octet-stream", qdf)),
        "has_supplement_relationship": bool(re.search(r"/AFRelationship\s*/Supplement", qdf)),
        "mutool_finds_embedded_file": "EmbeddedFile" in (text(mutool.stdout) + text(mutool.stderr)),
    }, None


def inspect_artifact(pdf: Path, work: Path) -> tuple[dict[str, Any] | None, str | None]:
    qdf, error = qdf_text(pdf, work)
    if error:
        return None, error
    extracted, error = extracted_text(pdf, work)
    if error:
        return None, error
    mutool = run(["mutool", "show", str(pdf), "grep", "Artifact"], work)
    if mutool.returncode not in (0, 1):
        return None, f"mutool show exit {mutool.returncode}: {text(mutool.stderr).strip()}"
    assert qdf is not None
    openings = re.findall(r"/Artifact\s*(?:<<(?P<props>.*?)>>\s*)?(?:BMC|BDC)", qdf,
                          flags=re.DOTALL)
    normalized_props = [re.sub(r"\s+", "", item) for item in openings]
    return {
        "text": extracted,
        "artifact_openings": len(openings),
        "artifact_properties": normalized_props,
        "artifact_has_own_mcid": any("/MCID" in item for item in openings),
        "artifact_struct_elem": bool(re.search(r"/S\s*/Artifact\b", qdf)),
    }, None


def execute_case(executable: Path, case: Case, work: Path,
                 capabilities: set[str]) -> dict[str, Any]:
    if case.optional_capability and case.optional_capability not in capabilities:
        return {"state": "NotApplicable", "reason": f"candidate does not expose {case.optional_capability}"}
    source = write_fixture(work, case)
    if case.mode == "query":
        result = run([str(executable), "query", str(source), case.selector or "<probe>",
                      "--field", "value", "--one"], work)
        if result.returncode != 0:
            return {"state": "Observed", "exit": result.returncode,
                    "error": primary_error(result.stderr)}
        return {"state": "Observed", "exit": 0, "value_json": text(result.stdout).strip()}

    suffix = ".html" if case.mode == "html" else ".svg" if case.mode == "svg" else ".pdf"
    output = work / f"{case.name}{suffix}"
    extra: list[str] = []
    if case.mode == "html":
        extra = ["--features", "html", "--format", "html"]
    elif case.mode == "svg":
        extra = ["--format", "svg"]
    elif case.mode == "artifact_pdf20":
        extra = ["--pdf-standard", "2.0"]
    elif case.mode == "attach_a3":
        extra = ["--pdf-standard", "a-3b"]
    elif case.mode == "artifact_no_tags":
        extra = ["--no-pdf-tags"]
    result = run(compile_cmd(executable, source, output, extra), work)
    if case.mode == "error":
        return {"state": "Observed", "exit": result.returncode,
                "error": primary_error(result.stderr)}
    if result.returncode != 0 or not output.exists():
        return {"state": "Observed", "exit": result.returncode,
                "error": primary_error(result.stderr)}
    if case.mode == "html":
        return {"state": "Observed", "exit": 0, **html_observation(output)}
    if case.mode == "svg":
        return {"state": "Observed", "exit": 0, **svg_observation(output)}
    if case.mode == "pdf_text":
        extracted, error = extracted_text(output, work)
        return {"state": "Unknown", "reason": error} if error else {
            "state": "Observed", "exit": 0, "text": extracted,
        }
    if case.mode == "attach_path":
        observation, error = inspect_attachment(output, work, b"P1286 path payload\n")
        return {"state": "Unknown", "reason": error} if error else {"state": "Observed", "exit": 0, **(observation or {})}
    if case.mode in ("attach_bytes", "attach_a3"):
        observation, error = inspect_attachment(output, work, bytes((0, 65, 255)))
        return {"state": "Unknown", "reason": error} if error else {"state": "Observed", "exit": 0, **(observation or {})}
    if case.mode in ("artifact", "artifact_plain", "artifact_pdf20", "artifact_no_tags"):
        observation, error = inspect_artifact(output, work)
        return {"state": "Unknown", "reason": error} if error else {"state": "Observed", "exit": 0, **(observation or {})}
    return {"state": "Unknown", "reason": f"unhandled mode {case.mode}"}


def capability_set(executable: Path) -> set[str]:
    help_result = run([str(executable), "compile", "--help"], ROOT)
    output = text(help_result.stdout) + text(help_result.stderr)
    return {flag for flag in ("--no-pdf-tags", "--pdf-standard") if flag in output}


def execute_pass(executable: Path, ordered: list[Case]) -> tuple[dict[str, Any], dict[str, str]]:
    observations: dict[str, Any] = {}
    fixture_hashes: dict[str, str] = {}
    capabilities = capability_set(executable)
    with tempfile.TemporaryDirectory(prefix="p1286-oracles-") as raw_work:
        work = Path(raw_work)
        (work / "payload.txt").write_bytes(b"P1286 path payload\n")
        (work / "same.bin").write_bytes(b"duplicate path payload\n")
        for case in ordered:
            prefix = '#set page(width: 200pt, height: 200pt, margin: 0pt)\n' if case.mode == "svg" else ""
            fixture_hashes[case.name] = sha256_bytes((prefix + case.source + "\n").encode())
            observations[case.name] = execute_case(executable, case, work, capabilities)
    return observations, fixture_hashes


def check_environment() -> list[str]:
    return [name for name in REQUIRED_TOOLS if shutil.which(name) is None]


def freeze(executable: Path, baseline_path: Path, force: bool) -> int:
    resolved = executable.resolve()
    if resolved != VANILLA.resolve():
        print(json.dumps({"verdict": "Unknown", "reason": "--freeze only accepts /usr/local/bin/typst"}))
        return 3
    actual_hash = sha256_file(resolved)
    if actual_hash != VANILLA_SHA256:
        print(json.dumps({"verdict": "Unknown", "reason": "vanilla SHA-256 mismatch", "actual": actual_hash}))
        return 3
    missing = check_environment()
    if missing:
        print(json.dumps({"verdict": "Unknown", "reason": "required PDF tools missing", "tools": missing}))
        return 3
    if baseline_path.exists() and not force:
        print(json.dumps({"verdict": "Unknown", "reason": "baseline exists; pass --force to replace"}))
        return 3
    suite = cases()
    first, fixture_hashes = execute_pass(resolved, suite)
    second, second_hashes = execute_pass(resolved, list(reversed(suite)))
    if first != second or fixture_hashes != second_hashes:
        differing = sorted(name for name in first if first.get(name) != second.get(name))
        print(json.dumps({"verdict": "Unknown", "reason": "vanilla observations changed under reverse order", "cases": differing}))
        return 3
    unknown = sorted(name for name, observation in first.items()
                     if observation.get("state") == "Unknown")
    if unknown:
        print(json.dumps({"verdict": "Unknown", "reason": "vanilla suite contains Unknown", "cases": unknown}))
        return 3
    payload = {
        "schema": "p1286-oracle-baseline-v1",
        "frozen_at_utc": datetime.now(timezone.utc).isoformat(),
        "policy": {"unknown_is_success": False, "byte_equality_is_parity": False},
        "contract": {"sha256": CONTRACT_SHA256,
                     "post_gate_l0_receipt_sha256": POST_GATE_SHA256},
        "vanilla": {"path": str(resolved), "sha256": actual_hash,
                    "version": version_of(resolved)},
        "tools": {name: tool_version(name) for name in REQUIRED_TOOLS},
        "passes": [
            {"name": "forward", "order": [case.name for case in suite]},
            {"name": "reverse", "order": [case.name for case in reversed(suite)]},
        ],
        "fixture_sha256": dict(sorted(fixture_hashes.items())),
        "comparison_policy": {case.name: case.comparison for case in suite},
        "cases": dict(sorted(first.items())),
    }
    baseline_path.parent.mkdir(parents=True, exist_ok=True)
    baseline_path.write_text(json.dumps(payload, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
                             encoding="utf-8")
    print(json.dumps({"verdict": "Frozen", "baseline": str(baseline_path),
                      "sha256": sha256_file(baseline_path), "cases": len(first)}, ensure_ascii=False))
    return 0


def compare_case(expected: dict[str, Any], actual: dict[str, Any], optional: bool) -> tuple[str, str | None]:
    if actual.get("state") == "Unknown":
        return "Unknown", actual.get("reason", "unknown observation")
    if optional and actual.get("state") == "NotApplicable":
        return "NotApplicable", actual.get("reason")
    if actual.get("state") == "NotApplicable":
        return "Unknown", actual.get("reason", "required case not applicable")
    if actual == expected:
        return "Preserved", None
    return "Violated", "semantic observation differs from frozen vanilla"


def verify(executable: Path, baseline_path: Path) -> int:
    missing = check_environment()
    if missing:
        print(json.dumps({"verdict": "Unknown", "reason": "required PDF tools missing", "tools": missing}))
        return 3
    if not executable.is_file() or not os.access(executable, os.X_OK):
        print(json.dumps({"verdict": "Unknown", "reason": "candidate is not an executable file", "path": str(executable)}))
        return 3
    if not baseline_path.is_file():
        print(json.dumps({"verdict": "Unknown", "reason": "frozen baseline missing", "path": str(baseline_path)}))
        return 3
    baseline = json.loads(baseline_path.read_text(encoding="utf-8"))
    if baseline.get("schema") != "p1286-oracle-baseline-v1":
        print(json.dumps({"verdict": "Unknown", "reason": "unsupported baseline schema"}))
        return 3
    if baseline.get("contract", {}).get("sha256") != CONTRACT_SHA256:
        print(json.dumps({"verdict": "Unknown", "reason": "contract pin mismatch"}))
        return 3
    suite = cases()
    first, fixture_hashes = execute_pass(executable.resolve(), suite)
    second, second_hashes = execute_pass(executable.resolve(), list(reversed(suite)))
    nondeterministic = sorted(name for name in first if first.get(name) != second.get(name))
    fixture_drift = fixture_hashes != baseline.get("fixture_sha256") or second_hashes != fixture_hashes
    by_name = {case.name: case for case in suite}
    results: dict[str, Any] = {}
    for name, expected in baseline["cases"].items():
        if by_name[name].comparison == "baseline_only":
            results[name] = {"verdict": "BaselineOnly",
                             "reason": "intentional crystalline extension is outside parity"}
            continue
        if name in nondeterministic:
            results[name] = {"verdict": "Unknown", "reason": "candidate changed under reverse order",
                             "forward": first.get(name), "reverse": second.get(name)}
            continue
        actual = first.get(name, {"state": "Unknown", "reason": "case missing"})
        verdict, reason = compare_case(expected, actual, bool(by_name[name].optional_capability))
        item: dict[str, Any] = {"verdict": verdict}
        if reason:
            item["reason"] = reason
        if verdict in ("Unknown", "Violated"):
            item["expected"] = expected
            item["actual"] = actual
        results[name] = item
    counts = {key: sum(1 for item in results.values() if item["verdict"] == key)
              for key in ("Preserved", "Violated", "Unknown", "NotApplicable", "BaselineOnly")}
    if fixture_drift:
        overall = "Unknown"
        reason = "fixture hashes differ from frozen baseline"
    elif counts["Unknown"]:
        overall = "Unknown"
        reason = "one or more observations are Unknown"
    elif counts["Violated"]:
        overall = "Violated"
        reason = "one or more semantic observations differ"
    else:
        overall = "Preserved"
        reason = None
    report = {
        "schema": "p1286-oracle-verification-v1",
        "verdict": overall,
        "reason": reason,
        "baseline": {"path": str(baseline_path.resolve()), "sha256": sha256_file(baseline_path)},
        "candidate": {"path": str(executable.resolve()), "sha256": sha256_file(executable.resolve()),
                      "version": version_of(executable.resolve())},
        "passes": ["forward", "reverse"],
        "counts": counts,
        "cases": results,
    }
    print(json.dumps(report, indent=2, ensure_ascii=False, sort_keys=True))
    return 0 if overall == "Preserved" else 3 if overall == "Unknown" else 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True,
                        help="vanilla executable for --freeze, otherwise candidate executable")
    parser.add_argument("--baseline", type=Path, default=DEFAULT_BASELINE)
    parser.add_argument("--freeze", action="store_true",
                        help="freeze baseline; accepted only for the pinned vanilla executable")
    parser.add_argument("--force", action="store_true",
                        help="replace an existing baseline during --freeze")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.force and not args.freeze:
        print(json.dumps({"verdict": "Unknown", "reason": "--force requires --freeze"}))
        return 3
    return freeze(args.binary, args.baseline, args.force) if args.freeze else verify(args.binary, args.baseline)


if __name__ == "__main__":
    raise SystemExit(main())
