#!/usr/bin/env python3
"""P1287 bilateral oracle. Freeze with vanilla; verify any later --binary."""

from __future__ import annotations

import argparse
import hashlib
import html.parser
import io
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / "lab/parity/matrix/fixtures/p1287"
BASELINE = ROOT / "lab/parity/matrix/p1287-oracle-baseline.json"
MANIFEST = ROOT / "00_nucleo/diagnosticos/p1287-manifest.json"
MANIFEST_SHA = "5d4cc9a6180300c8402be4a91b30104db08874f8540c9bc8af5b895a9fdf725b"
VANILLA = Path("/usr/local/bin/typst")
VANILLA_SHA = "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8"
EXPECTED_BASELINE_SHA = "ff09c8146b73684b2e746c631a6103817b6df7070aa8248e2d88dbca34749488"
TIMEOUT = 45

# One real carrier per Cxx. E rows either inspect a real carrier or freeze an
# explicit Unknown when the 12-fixture budget cannot represent the full axis.
CASES = [
    *[
        {"id": f"C{i:02}", "source": f"C{i:02}.typ", "op": "pdf" if i == 7 else "png"}
        for i in range(1, 10)
    ],
    {"id": "C10", "source": "C10.typ", "op": "pdf"},
    {"id": "C11", "source": "C11.typ", "op": "query"},
    {"id": "C12", "source": "C12.typ", "op": "diagnostics"},
    {"id": "E01", "source": "C01.typ", "op": "png", "repeat": 2},
    {"id": "E02", "source": "C01.typ", "op": "pdf"},
    {"id": "E03", "source": "C01.typ", "op": "svg", "repeat": 2},
    {"id": "E04", "source": "C09.typ", "op": "svg", "opaque": "carrier does not separate the full Linear/Radial x Luma/CMYK product"},
    {"id": "E05", "source": "C09.typ", "op": "svg", "opaque": "carrier has Conic/RGB only; per-supported-space product is absent"},
    {"id": "E06", "source": "C09.typ", "op": "svg", "opaque": "carrier has content tiling only; image and gradient tilings are absent"},
    {"id": "E07", "source": "C07.typ", "op": "pdf", "opaque": "same/cross-page links exist, but no cross-file bundle carrier is frozen"},
    {"id": "E08", "source": "C08.typ", "op": "svg", "opaque": "SVG image exists; PNG/JPEG/GIF/WebP/SVGZ/PDF/external/unsupported product is absent"},
    {"id": "E09", "source": "C03.typ", "op": "svg", "opaque": "missing-font warning exists; collision and fontless-wrapper carriers are absent"},
    {"id": "E10", "source": "C09.typ", "op": "svg", "opaque": "alpha and rounded clip exist; mask/nested/even-odd carriers are absent"},
    {"id": "E11", "source": "C11.typ", "op": "html"},
]


def sha_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha_file(path: Path) -> str:
    return sha_bytes(path.read_bytes())


def canonical(value) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def suite_hash() -> str:
    return sha_bytes(canonical({"cases": CASES, "policy": "P1287-GLOBAL-PARITY-v1/exact-rgba/no-mask"}))


def fixture_hashes() -> dict[str, str]:
    files = sorted(FIXTURES.glob("*.typ"))
    expected = [f"C{i:02}.typ" for i in range(1, 13)]
    if [p.name for p in files] != expected:
        raise RuntimeError(f"fixture set must be exactly {expected}")
    return {p.name: sha_file(p) for p in files}


def fixture_manifest_hash() -> str:
    return sha_bytes(canonical(fixture_hashes()))


def normalize_diag(text: str, work: Path) -> str:
    text = text.replace(str(work), "<WORK>").replace(str(ROOT), "<ROOT>")
    text = re.sub(r"(?m)^warning: unknown font family.*\n?", "warning: unknown font family\n", text)
    return "\n".join(line.rstrip() for line in text.strip().splitlines())


def command(argv: list[str], work: Path) -> dict:
    env = {**os.environ, "LC_ALL": "C", "LANG": "C", "TZ": "UTC", "SOURCE_DATE_EPOCH": "0"}
    try:
        p = subprocess.run(argv, cwd=work, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=TIMEOUT)
        return {"kind": "completed", "exit": p.returncode, "stdout": p.stdout, "stderr": p.stderr}
    except (OSError, subprocess.TimeoutExpired) as exc:
        return {"kind": "harness", "reason": type(exc).__name__}


def png_observe(path: Path) -> dict:
    try:
        from PIL import Image
        with Image.open(path) as image:
            image.load()
            rgba = image.convert("RGBA")
            return {"state": "valid", "semantic": {"size": list(rgba.size), "rgba_sha256": sha_bytes(rgba.tobytes())},
                    "artifact_sha256": sha_file(path)}
    except Exception as exc:
        return {"state": "unknown", "reason": f"PNG decode: {type(exc).__name__}"}


def pdf_observe(path: Path, work: Path) -> dict:
    required = ["qpdf", "pdfinfo", "pdftotext", "pdffonts"]
    if any(shutil.which(tool) is None for tool in required):
        return {"state": "unknown", "reason": "PDF parser dependency missing"}
    checks = command(["qpdf", "--check", str(path)], work)
    if checks.get("exit") != 0:
        return {"state": "unknown", "reason": "malformed PDF"}
    info = command(["pdfinfo", str(path)], work)
    text = command(["pdftotext", "-layout", str(path), "-"], work)
    fonts = command(["pdffonts", str(path)], work)
    qdf = work / "out.qdf.pdf"
    q = command(["qpdf", "--qdf", "--object-streams=disable", str(path), str(qdf)], work)
    if any(x.get("exit") != 0 for x in (info, text, fonts, q)):
        return {"state": "unknown", "reason": "PDF observation failed"}
    info_s = info["stdout"].decode("utf-8", "replace")
    font_rows = []
    for line in fonts["stdout"].decode("utf-8", "replace").splitlines()[2:]:
        cols = line.split()
        if len(cols) >= 7:
            cols[0] = re.sub(r"^[A-Z]{6}\+", "<SUBSET>+", cols[0])
            font_rows.append(cols[:7])
    raw = qdf.read_bytes()
    semantic = {
        "pages": re.findall(r"(?m)^Pages:\s+(\d+)", info_s),
        "sizes": re.findall(r"(?m)^Page(?:\s+\d+)? size:\s+(.+)$", info_s),
        "text": text["stdout"].decode("utf-8", "replace").rstrip(),
        "fonts": font_rows,
        "marked": bool(re.search(rb"/Marked\s+true", raw)),
        "structure_tree": b"/StructTreeRoot" in raw,
        "uri_count": len(re.findall(rb"/URI\b", raw)),
        "dest_count": len(re.findall(rb"/(?:Dest|D)\b", raw)),
    }
    return {"state": "valid", "semantic": semantic, "artifact_sha256": sha_file(path)}


def svg_observe(path: Path) -> dict:
    raw = path.read_bytes()
    if b"\0" in raw:
        return {"state": "unknown", "reason": "SVG contains NUL"}
    try:
        root = ET.fromstring(raw)
    except ET.ParseError:
        return {"state": "unknown", "reason": "malformed SVG XML"}
    nodes = list(root.iter())
    local = lambda tag: tag.rsplit("}", 1)[-1]
    attrs = []
    for n in nodes:
        for k, v in n.attrib.items():
            key = local(k)
            if key in {"d", "transform", "fill", "stroke", "opacity", "fill-rule", "clip-path", "mask", "href", "viewBox", "width", "height"}:
                attrs.append([local(n.tag), key, v])
    semantic = {
        "root": local(root.tag), "tags": sorted(local(n.tag) for n in nodes),
        "attrs": sorted(attrs), "text": " ".join("".join(root.itertext()).split()),
    }
    return {"state": "valid", "semantic": semantic, "artifact_sha256": sha_file(path)}


class HtmlObserver(html.parser.HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.tags, self.links, self.text = [], [], []

    def handle_starttag(self, tag, attrs):
        self.tags.append(tag)
        for key, value in attrs:
            if key in {"href", "src"}:
                self.links.append([tag, key, value])

    def handle_data(self, data):
        if data.strip():
            self.text.append(" ".join(data.split()))


def html_observe(path: Path) -> dict:
    try:
        obs = HtmlObserver(); obs.feed(path.read_text("utf-8")); obs.close()
        return {"state": "valid", "semantic": {"tags": obs.tags, "links": obs.links, "text": obs.text},
                "artifact_sha256": sha_file(path)}
    except Exception as exc:
        return {"state": "unknown", "reason": f"HTML parse: {type(exc).__name__}"}


def compile_case(binary: Path, case: dict, work: Path, fmt: str) -> dict:
    work.mkdir(parents=True, exist_ok=True)
    source = FIXTURES / case["source"]
    output = work / f"out.{fmt}"
    argv = [str(binary), "compile", "--creation-timestamp", "0", "--jobs", "1", "--diagnostic-format", "short"]
    if fmt == "png":
        argv += ["--ppi", "144"]
    if fmt == "html":
        argv += ["--features", "html"]
    argv += ["--format", fmt, str(source), str(output)]
    result = command(argv, work)
    if result["kind"] != "completed":
        return {"state": "unknown", "reason": result["reason"]}
    diag = normalize_diag(result["stderr"].decode("utf-8", "replace"), work)
    if result["exit"] != 0:
        return {"state": "rejected", "semantic": {"diagnostic": diag}}
    if not output.is_file():
        return {"state": "unknown", "reason": "successful compile without artifact"}
    observed = {"png": png_observe, "svg": svg_observe, "html": html_observe}.get(fmt)
    value = pdf_observe(output, work) if fmt == "pdf" else observed(output)
    if value.get("state") == "valid" and case["id"] in {"E09"}:
        value["semantic"]["diagnostic"] = diag
    return value


def observe(binary: Path, case: dict, work: Path) -> dict:
    if case.get("opaque"):
        # Execute the real partial carrier, but never promote it to success.
        partial = compile_case(binary, case, work, case["op"])
        return {"state": "unknown", "reason": case["opaque"], "partial": partial.get("state")}
    op = case["op"]
    if op in {"png", "pdf", "svg", "html"}:
        runs = [compile_case(binary, case, work / f"r{i}", op) for i in range(case.get("repeat", 1))]
        return stable_runs(runs)
    if op == "query":
        values = {}
        for selector in ("metadata", "heading", "figure", "selector(heading).where(level: 9)"):
            r = command([str(binary), "query", "--diagnostic-format", "short", str(FIXTURES / case["source"]), selector], work)
            if r["kind"] != "completed": return {"state": "unknown", "reason": r["reason"]}
            values[selector] = {"exit": r["exit"], "stdout": r["stdout"].decode("utf-8", "replace").strip(),
                                "stderr": normalize_diag(r["stderr"].decode("utf-8", "replace"), work)}
        html = compile_case(binary, {**case, "id": "C11-html"}, work / "html", "html")
        if html.get("state") != "valid": return {"state": "unknown", "reason": "C11 HTML carrier unavailable"}
        return {"state": "valid", "semantic": {"queries": values, "html": html["semantic"]}}
    if op == "diagnostics":
        invalid = compile_case(binary, case, work / "invalid", "pdf")
        evals = {}
        for expr in ("repr((1 + 2, \"a\" + \"b\", calc.gcd(12, 18)))", "p1287_missing_name"):
            r = command([str(binary), "eval", "--diagnostic-format", "short", expr], work)
            if r["kind"] != "completed": return {"state": "unknown", "reason": r["reason"]}
            evals[expr] = {"exit": r["exit"], "stdout": r["stdout"].decode("utf-8", "replace").strip(),
                           "stderr": normalize_diag(r["stderr"].decode("utf-8", "replace"), work)}
        return {"state": "valid", "semantic": {"invalid_compile": invalid, "eval": evals}}
    raise AssertionError(op)


def stable_runs(runs: list[dict]) -> dict:
    if not runs: return {"state": "unknown", "reason": "no run"}
    first = runs[0]
    comparable = [{k: v for k, v in r.items() if k != "artifact_sha256"} for r in runs]
    if any(v != comparable[0] for v in comparable[1:]):
        return {"state": "unknown", "reason": "non-deterministic repeated observation"}
    if first.get("state") == "valid" and len(runs) > 1:
        hashes = [r.get("artifact_sha256") for r in runs]
        if len(set(hashes)) != 1:
            return {"state": "unknown", "reason": "non-deterministic artifact bytes"}
    return first


def run_order(binary: Path, cases: list[dict]) -> dict[str, dict]:
    result = {}
    with tempfile.TemporaryDirectory(prefix="p1287-") as tmp:
        base = Path(tmp)
        for case in cases:
            work = base / case["id"]; work.mkdir(parents=True)
            result[case["id"]] = observe(binary, case, work)
    return result


def protected(baseline: dict | None = None, allow_unfrozen=False) -> None:
    if sha_file(MANIFEST) != MANIFEST_SHA: raise RuntimeError("manifest hash drift")
    fixture_manifest_hash()
    if baseline is not None:
        if baseline["manifest_sha256"] != MANIFEST_SHA: raise RuntimeError("baseline manifest pin drift")
        if baseline["suite_sha256"] != suite_hash(): raise RuntimeError("suite hash drift")
        if baseline["fixtures_sha256"] != fixture_manifest_hash(): raise RuntimeError("fixture hash drift")
        if not allow_unfrozen and EXPECTED_BASELINE_SHA != sha_file(BASELINE): raise RuntimeError("baseline hash drift")


def classify(expected: dict, actual: dict) -> tuple[str, str]:
    if expected.get("state") == "not_applicable": return "NotApplicable", expected.get("reason", "structurally irrelevant")
    if expected.get("state") == "unknown": return "Unknown", expected.get("reason", "opaque baseline")
    if actual.get("state") == "unknown": return "Unknown", actual.get("reason", "opaque candidate")
    if expected.get("state") != actual.get("state"): return "Violated", "accept/reject changed"
    if expected.get("semantic") != actual.get("semantic"): return "Violated", "canonical observable differs"
    return "Preserved", "canonical observable equal"


def freeze() -> int:
    if VANILLA.resolve() != Path("/usr/local/bin/typst") or sha_file(VANILLA) != VANILLA_SHA:
        raise RuntimeError("freeze requires the pinned /usr/local/bin/typst")
    protected()
    forward = run_order(VANILLA, CASES)
    reverse = run_order(VANILLA, list(reversed(CASES)))
    frozen = {}
    for case in CASES:
        cid = case["id"]
        frozen[cid] = forward[cid] if forward[cid] == reverse[cid] else {"state": "unknown", "reason": "vanilla forward/reverse disagreement"}
    data = {"schema": "p1287-oracle-baseline/v1", "manifest_sha256": MANIFEST_SHA,
            "suite_sha256": suite_hash(), "fixtures_sha256": fixture_manifest_hash(),
            "fixture_hashes": fixture_hashes(), "binary_sha256": VANILLA_SHA,
            "binary_identity": "Unknown: association of pinned bytes to upstream a51e02804 is not independently proved",
            "png_policy": "exact decoded RGBA; no mask and no tolerance", "cases": frozen}
    BASELINE.write_bytes(canonical(data) + b"\n")
    print(json.dumps({"frozen": str(BASELINE), "sha256": sha_file(BASELINE),
                      "states": counts(v.get("state") for v in frozen.values())}, sort_keys=True))
    return 0


def counts(values) -> dict:
    out = {}
    for value in values: out[value] = out.get(value, 0) + 1
    return out


def verify(binary: Path) -> int:
    baseline = json.loads(BASELINE.read_text("utf-8")); protected(baseline)
    if not binary.is_file(): raise RuntimeError("binary does not exist")
    forward = run_order(binary, CASES); reverse = run_order(binary, list(reversed(CASES)))
    rows = []
    for case in CASES:
        cid = case["id"]
        actual = forward[cid] if forward[cid] == reverse[cid] else {"state": "unknown", "reason": "candidate forward/reverse disagreement"}
        state, reason = classify(baseline["cases"][cid], actual)
        rows.append({"id": cid, "classification": state, "reason": reason})
    result = {"schema": "p1287-bilateral-result/v1", "binary": str(binary), "binary_sha256": sha_file(binary),
              "baseline_sha256": sha_file(BASELINE), "manifest_sha256": MANIFEST_SHA,
              "classifications": counts(r["classification"] for r in rows), "cases": rows}
    print(json.dumps(result, ensure_ascii=False, sort_keys=True))
    classes = {r["classification"] for r in rows}
    return 1 if "Violated" in classes else (2 if "Unknown" in classes else 0)


def self_test() -> int:
    protected(json.loads(BASELINE.read_text("utf-8")) if BASELINE.exists() else None,
              allow_unfrozen=EXPECTED_BASELINE_SHA == "TO_BE_FROZEN")
    assert classify({"state": "valid", "semantic": 1}, {"state": "valid", "semantic": 1})[0] == "Preserved"
    assert classify({"state": "valid", "semantic": 1}, {"state": "valid", "semantic": 2})[0] == "Violated"
    assert classify({"state": "unknown"}, {"state": "valid", "semantic": 1})[0] == "Unknown"
    assert classify({"state": "not_applicable"}, {"state": "valid", "semantic": 1})[0] == "NotApplicable"
    print("P1287 self-test: PASS")
    return 0


def main() -> int:
    p = argparse.ArgumentParser()
    group = p.add_mutually_exclusive_group(required=True)
    group.add_argument("--freeze", action="store_true")
    group.add_argument("--binary", type=Path)
    group.add_argument("--self-test", action="store_true")
    group.add_argument("--list", action="store_true")
    args = p.parse_args()
    if args.freeze: return freeze()
    if args.binary: return verify(args.binary.resolve())
    if args.self_test: return self_test()
    print("\n".join(f"{c['id']}\t{c['op']}\t{c['source']}\t{c.get('opaque','')}" for c in CASES)); return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (RuntimeError, KeyError, json.JSONDecodeError) as exc:
        print(f"P1287 HARNESS_DEFECT: {exc}", file=sys.stderr)
        raise SystemExit(3)
