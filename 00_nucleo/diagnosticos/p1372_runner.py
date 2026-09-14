#!/usr/bin/env python3
"""Runner bilateral limitado para a relação vertical S07."""

from __future__ import annotations

import argparse
import importlib.util
import json
import tempfile
import xml.etree.ElementTree as ET
from datetime import datetime, timezone
from decimal import Decimal, InvalidOperation, ROUND_HALF_UP
from pathlib import Path
from typing import Any


PRODUCTS = ("vanila", "cristalino")
PROFILES = ("a11y-extras", "html+a11y")
CASES = ("lone", "control")
PROFILE_FEATURES = {
    "a11y-extras": ("a11y-extras",),
    "html+a11y": ("html", "a11y-extras"),
}
PAGE_PREFIX = "#set page(width: 200pt, height: 240pt, margin: 10pt)\n"
CASE_INPUTS = {
    "lone": '#block(height: 100%)[#metadata("s07")<s07>Probe]\n#parbreak()\nAfter\n',
    "control": "#block(height: 100%)[P#box(width: 0pt)[]robe]\n#parbreak()\nAfter\n",
}
EXPECTED_TEXT = "ProbeAfter"
EXPECTED_HASHES = {
    "vanila": "eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f",
    "cristalino": "60851811dca31074eca38f5d4e397ce04a89eccb7bf49c9d6e28f6759ba761f9",
    "pdftotext": "0fb98ea179e19154a90202608c164f2a319b79f16576fa6534b2d601033565e7",
}
PRECISION = Decimal("0.001")


def _load_bounded_runner() -> Any:
    path = Path(__file__).with_name("p1369_runner.py")
    spec = importlib.util.spec_from_file_location("p1369_runner_for_p1372", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("P1369 bounded runner loader unavailable")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


BOUNDED = _load_bounded_runner()


def _unknown(reason_code: str) -> dict[str, Any]:
    return {"complete": False, "value": None, "reason_code": reason_code}


def _normalized_text(element: ET.Element) -> str:
    return "".join("".join(element.itertext()).split())


def parse_bbox(payload: bytes) -> dict[str, Any]:
    """Extrai o texto e a coordenada do único marcador, fechando em Unknown."""
    try:
        root = ET.fromstring(payload)
    except (ET.ParseError, ValueError):
        return _unknown("BBOX_XML_INVALID")

    words: list[tuple[int, ET.Element, str]] = []
    pages = [element for element in root.iter() if element.tag.rsplit("}", 1)[-1] == "page"]
    for page_index, page in enumerate(pages):
        for element in page.iter():
            if element.tag.rsplit("}", 1)[-1] == "word":
                words.append((page_index, element, _normalized_text(element)))

    text = "".join(word for _, _, word in words)
    if text != EXPECTED_TEXT:
        return _unknown("TEXT_MISMATCH")

    markers = [(page, element) for page, element, word in words if word == "After"]
    if len(markers) != 1:
        return _unknown("MARKER_CARDINALITY")

    page, marker = markers[0]
    raw_y = marker.get("yMin")
    if raw_y is None:
        return _unknown("MARKER_Y_MISSING")
    try:
        y_min = Decimal(raw_y).quantize(PRECISION, rounding=ROUND_HALF_UP)
    except InvalidOperation:
        return _unknown("MARKER_Y_INVALID")
    if not y_min.is_finite():
        return _unknown("MARKER_Y_INVALID")

    return {
        "complete": True,
        "text": text,
        "after": {"page": page, "y_min": format(y_min, ".3f")},
        "reason_code": None,
    }


def vertical_relation(lone: dict[str, Any], control: dict[str, Any]) -> dict[str, Any]:
    if not lone.get("complete") or not control.get("complete"):
        return _unknown("INCOMPLETE_OBSERVATION")
    if lone.get("text") != EXPECTED_TEXT or control.get("text") != EXPECTED_TEXT:
        return _unknown("TEXT_MISMATCH")
    try:
        page_delta = int(lone["after"]["page"]) - int(control["after"]["page"])
        y_delta = (Decimal(lone["after"]["y_min"]) - Decimal(control["after"]["y_min"])).quantize(
            PRECISION, rounding=ROUND_HALF_UP
        )
    except (KeyError, TypeError, ValueError, InvalidOperation):
        return _unknown("RELATION_INVALID")
    if not y_delta.is_finite():
        return _unknown("RELATION_INVALID")
    return {
        "complete": True,
        "value": {"page_delta": page_delta, "y_delta": format(y_delta, ".3f")},
        "reason_code": None,
    }


def vertical_signature(lone: dict[str, Any], control: dict[str, Any]) -> dict[str, Any]:
    """Conserva os absolutos e o delta para impedir cancelamento comum."""
    relation = vertical_relation(lone, control)
    if not relation.get("complete"):
        return relation
    try:
        lone_after = {
            "page": int(lone["after"]["page"]),
            "y_min": format(Decimal(lone["after"]["y_min"]), ".3f"),
        }
        control_after = {
            "page": int(control["after"]["page"]),
            "y_min": format(Decimal(control["after"]["y_min"]), ".3f"),
        }
    except (KeyError, TypeError, ValueError, InvalidOperation):
        return _unknown("SIGNATURE_INVALID")
    return {
        "complete": True,
        "value": {
            "lone": lone_after,
            "control": control_after,
            "relation": relation["value"],
        },
        "reason_code": None,
    }


def classify_bilateral(vanila: dict[str, Any], cristalino: dict[str, Any]) -> str:
    if not vanila.get("complete") or not cristalino.get("complete"):
        return "Unknown"
    return "MATCH" if vanila.get("value") == cristalino.get("value") else "DIFFERENCE"


def planned_jobs() -> list[dict[str, str]]:
    return [
        {"product": product, "profile": profile, "case": case}
        for product in PRODUCTS
        for profile in PROFILES
        for case in CASES
    ]


def execute_job(
    job: dict[str, str], binaries: dict[str, Path], pdftotext: Path, root: Path
) -> dict[str, Any]:
    case_root = root / job["product"] / job["profile"] / job["case"]
    case_root.mkdir(parents=True, exist_ok=False)
    source_path = case_root / "probe.typ"
    output_path = case_root / "out.pdf"
    source_path.write_text(PAGE_PREFIX + CASE_INPUTS[job["case"]], encoding="utf-8")

    features = ",".join(PROFILE_FEATURES[job["profile"]])
    compile_process = BOUNDED._bounded_process(
        [
            str(binaries[job["product"]]),
            "compile",
            "--features",
            features,
            str(source_path),
            str(output_path),
        ]
    )
    compile_receipt = BOUNDED._receipt(compile_process)
    if compile_process["reason_code"] is not None:
        return {
            "job": job,
            "compile": compile_receipt,
            "pdftotext": None,
            "observation": _unknown(compile_process["reason_code"]),
        }
    if compile_process["exit_code"] != 0:
        return {
            "job": job,
            "compile": compile_receipt,
            "pdftotext": None,
            "observation": _unknown(f"COMPILE_EXIT_{compile_process['exit_code']}"),
        }
    if not output_path.is_file():
        return {
            "job": job,
            "compile": compile_receipt,
            "pdftotext": None,
            "observation": _unknown("OUTPUT_MISSING"),
        }
    if output_path.stat().st_size > BOUNDED.FILE_SIZE_BYTES:
        return {
            "job": job,
            "compile": compile_receipt,
            "pdftotext": None,
            "observation": _unknown("OUTPUT_FILE_LIMIT"),
        }

    text_process = BOUNDED._bounded_process(
        [str(pdftotext), "-bbox-layout", str(output_path), "-"]
    )
    text_receipt = BOUNDED._receipt(text_process)
    if text_process["reason_code"] is not None:
        observation = _unknown(text_process["reason_code"])
    elif text_process["exit_code"] != 0:
        observation = _unknown(f"PDFTOTEXT_EXIT_{text_process['exit_code']}")
    else:
        observation = parse_bbox(text_process["stdout"])
    return {
        "job": job,
        "compile": compile_receipt,
        "pdftotext": text_receipt,
        "output_size": output_path.stat().st_size,
        "output_sha256": BOUNDED.sha256_path(output_path),
        "observation": observation,
    }


def execute(args: argparse.Namespace) -> dict[str, Any]:
    binaries = {
        "vanila": Path(args.vanila).resolve(),
        "cristalino": Path(args.cristalino).resolve(),
    }
    pdftotext = Path(args.pdftotext).resolve()
    identities = {
        "vanila": BOUNDED.sha256_path(binaries["vanila"]),
        "cristalino": BOUNDED.sha256_path(binaries["cristalino"]),
        "pdftotext": BOUNDED.sha256_path(pdftotext),
    }
    if identities != EXPECTED_HASHES:
        raise ValueError(f"frozen identity mismatch: {identities}")

    with tempfile.TemporaryDirectory(prefix="p1372-s07.") as directory:
        root = Path(directory)
        rows = [execute_job(job, binaries, pdftotext, root) for job in planned_jobs()]

    indexed = {
        (row["job"]["product"], row["job"]["profile"], row["job"]["case"]): row
        for row in rows
    }
    profiles = []
    for profile in PROFILES:
        signatures = {
            product: vertical_signature(
                indexed[(product, profile, "lone")]["observation"],
                indexed[(product, profile, "control")]["observation"],
            )
            for product in PRODUCTS
        }
        profiles.append(
            {
                "profile": profile,
                "products": signatures,
                "bilateral": classify_bilateral(signatures["vanila"], signatures["cristalino"]),
            }
        )

    counts: dict[str, int] = {}
    for profile in profiles:
        counts[profile["bilateral"]] = counts.get(profile["bilateral"], 0) + 1
    return {
        "schema": "p1372-s07-bilateral/v1",
        "created_at": datetime.now(timezone.utc).isoformat(),
        "identities": identities,
        "limits": {
            "parallel_processes": BOUNDED.MAX_PARALLEL_PROCESSES,
            "timeout_seconds": BOUNDED.TIMEOUT_SECONDS,
            "address_space_bytes": BOUNDED.ADDRESS_SPACE_BYTES,
            "file_size_bytes": BOUNDED.FILE_SIZE_BYTES,
            "cpu_seconds": BOUNDED.CPU_SECONDS,
            "channel_read_bytes": BOUNDED.CHANNEL_READ_BYTES,
        },
        "job_count": len(rows),
        "pdftotext_count": sum(row["pdftotext"] is not None for row in rows),
        "counts": counts,
        "profiles": profiles,
        "rows": rows,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--vanila", required=True)
    parser.add_argument("--cristalino", required=True)
    parser.add_argument("--pdftotext", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    result = execute(args)
    Path(args.output).write_text(
        json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )


if __name__ == "__main__":
    main()
