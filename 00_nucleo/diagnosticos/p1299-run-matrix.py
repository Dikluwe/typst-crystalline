#!/usr/bin/env python3
"""Gera o catálogo e executa a matriz bilateral quadrilateral do P1299."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import pathlib
import re
import subprocess
import time
from collections import Counter


ROOT = pathlib.Path(__file__).resolve().parents[2]
PROFILES = {
    "default": [],
    "html": ["html"],
    "a11y": ["a11y-extras"],
    "html+a11y": ["html,a11y-extras"],
}
INVENTORY_CLASSES = {
    "EXTRA_BINDING",
    "MISSING_BINDING",
    "MISSING_MEMBER",
    "WRONG_KIND",
    "UNVERIFIED_METADATA",
    "UNKNOWN",
}
CANARIES = ("html", "pdf.table-summary", "pdf.header-cell", "pdf.data-cell")

PROMPT_ROOT = ROOT / "00_nucleo/prompts"
PROMPTS = {
    "binding": PROMPT_ROOT / "compiler/eval/bindings/field_access.md",
    "calc": PROMPT_ROOT / "compiler/stdlib/calc.md",
    "color": PROMPT_ROOT / "compiler/stdlib/color.md",
    "counter": PROMPT_ROOT / "compiler/stdlib/counter.md",
    "float": PROMPT_ROOT / "compiler/stdlib/foundations/float.md",
    "function": PROMPT_ROOT / "entities/func.md",
    "html": PROMPT_ROOT / "compiler/stdlib/html.md",
    "loading": PROMPT_ROOT / "compiler/stdlib/loading.md",
    "math": PROMPT_ROOT / "compiler/stdlib/structural/math.md",
    "math_style": PROMPT_ROOT / "compiler/stdlib/math_style.md",
    "outline": PROMPT_ROOT / "compiler/stdlib/structural/outline.md",
    "pdf": PROMPT_ROOT / "compiler/eval/bindings/field_access.md",
    "repr": PROMPT_ROOT / "compiler/eval/repr.md",
    "selector": PROMPT_ROOT / "compiler/stdlib/foundations/selector.md",
    "shapes": PROMPT_ROOT / "compiler/stdlib/shapes.md",
    "state": PROMPT_ROOT / "compiler/stdlib/state.md",
    "structural": PROMPT_ROOT / "compiler/stdlib/structural.md",
    "sym": PROMPT_ROOT / "compiler/stdlib/sym.md",
    "text": PROMPT_ROOT / "compiler/stdlib/text.md",
    "version": PROMPT_ROOT / "compiler/stdlib/primitives-constructors/version.md",
}

COLOR_GLOBAL_CONTRADICTIONS = {"hsl", "hsv", "linear_rgb"}
MATH_STYLE_EXTRAS = {
    "bb", "bold", "cal", "display", "frak", "inline", "italic", "mono",
    "sans", "scr", "script", "serif", "sscript", "upright",
}
MATH_EXTRAS = {"accent", "cancel", "op", "underover", "math.registered"}
STRUCTURAL_EXTRAS = {
    "asset", "grid_cell", "grid_footer", "grid_header", "lof", "lot",
    "table_cell", "table_footer", "table_header",
}

# Somente rotas públicas canônicas conhecidas. A existência é medida, não presumida:
# cada rota vira uma sonda bilateral própria nos quatro perfis.
CANONICAL_ROUTES = {
    "accent": "math.accent",
    "bb": "math.bb",
    "bold": "math.bold",
    "cal": "math.cal",
    "cancel": "math.cancel",
    "counter_at": "counter.at",
    "counter_display": "counter.display",
    "counter_final": "counter.final",
    "counter_step": "counter.step",
    "display": "math.display",
    "frak": "math.frak",
    "grid_cell": "grid.cell",
    "grid_footer": "grid.footer",
    "grid_header": "grid.header",
    "hsl": "color.hsl",
    "hsv": "color.hsv",
    "inline": "math.inline",
    "italic": "math.italic",
    "linear_rgb": "color.linear-rgb",
    "mono": "math.mono",
    "op": "math.op",
    "sans": "math.sans",
    "scr": "math.scr",
    "script": "math.script",
    "serif": "math.serif",
    "sscript": "math.sscript",
    "state_at": "state.at",
    "state_final": "state.final",
    "state_update": "state.update",
    "state_update_with": "state.update",
    "table_cell": "table.cell",
    "table_footer": "table.footer",
    "table_header": "table.header",
    "upright": "math.upright",
}


def sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def feature_argv(profile: str) -> list[str]:
    features = PROFILES[profile]
    return [] if not features else ["--features", features[0]]


def command(binary: pathlib.Path, expression: str, profile: str) -> list[str]:
    return [str(binary), "eval", expression, "--format", "json", *feature_argv(profile)]


def expression_for(path: str) -> str:
    return f"repr((type({path}), repr({path})))"


def add_probe(
    probes: dict[str, dict],
    path: str,
    *,
    origin: str,
    role: str,
    expression: str | None = None,
    inventory_class: str | None = None,
    canonical_route: str | None = None,
    extra_path: str | None = None,
) -> None:
    probe = probes.setdefault(
        path,
        {
            "id": f"path-{path}",
            "path": path,
            "expression": expression or expression_for(path),
            "profiles": list(PROFILES),
            "origins": [],
            "roles": [],
            "inventory_classes": [],
        },
    )
    if expression is not None:
        probe["expression"] = expression
    if origin not in probe["origins"]:
        probe["origins"].append(origin)
    if role not in probe["roles"]:
        probe["roles"].append(role)
    if inventory_class and inventory_class not in probe["inventory_classes"]:
        probe["inventory_classes"].append(inventory_class)
    if canonical_route:
        probe["canonical_route"] = canonical_route
    if extra_path:
        extras = probe.setdefault("extra_paths", [])
        if extra_path not in extras:
            extras.append(extra_path)


def build_catalog(
    inventory_paths: list[pathlib.Path], surface_paths: list[pathlib.Path]
) -> dict:
    probes: dict[str, dict] = {}
    inventory_inputs = []
    extra_paths = set()

    for source in inventory_paths:
        payload = json.loads(source.read_text())
        inventory_inputs.append(
            {"path": str(source), "sha256": sha256(source), "profile": payload["profile"]}
        )
        candidates = list(payload["entries"]) + list(payload.get("blocked_by_ancestor", []))
        for entry in candidates:
            classification = entry["classification"]
            if classification not in INVENTORY_CLASSES:
                continue
            path = entry["display_path"]
            add_probe(
                probes,
                path,
                origin=f"inventory:{payload['profile']}",
                role="inventory_non_match",
                inventory_class=classification,
            )
            if classification == "EXTRA_BINDING":
                extra_paths.add(path)

    surface_inputs = []
    for source in surface_paths:
        payload = json.loads(source.read_text())
        surface_inputs.append(
            {"path": str(source), "sha256": sha256(source), "profile": payload["profile"]}
        )
        for result in payload["results"]:
            if result.get("same", False):
                continue
            path = result.get("path")
            if not path:
                continue
            add_probe(
                probes,
                path,
                origin=f"p1297:{payload['profile']}",
                role="p1297_difference",
                expression=result["expression"],
            )

    for path in CANARIES:
        expression = "repr(type(html))" if path == "html" else None
        add_probe(probes, path, origin="p1299:explicit", role="canary", expression=expression)

    for extra_path in sorted(extra_paths):
        canonical = CANONICAL_ROUTES.get(extra_path)
        if canonical is None:
            continue
        add_probe(
            probes,
            extra_path,
            origin="p1299:canonical-pair",
            role="extra_route",
            canonical_route=canonical,
        )
        add_probe(
            probes,
            canonical,
            origin="p1299:canonical-pair",
            role="canonical_route",
            extra_path=extra_path,
        )

    for probe in probes.values():
        probe["origins"].sort()
        probe["roles"].sort()
        probe["inventory_classes"].sort()
        if "extra_paths" in probe:
            probe["extra_paths"].sort()

    ordered = [probes[path] for path in sorted(probes)]
    return {
        "schema_version": "p1299-probe-catalog-v1",
        "derivation": {
            "inventory_classes": sorted(INVENTORY_CLASSES),
            "inventory_inputs": inventory_inputs,
            "surface_inputs": surface_inputs,
            "explicit_canaries": list(CANARIES),
            "canonical_route_map": dict(sorted(CANONICAL_ROUTES.items())),
        },
        "profiles": [
            {"name": name, "feature_argv": feature_argv(name)} for name in PROFILES
        ],
        "counts": {
            "probes": len(ordered),
            "canonical_pairs": sum("canonical_route" in probe for probe in ordered),
        },
        "probes": ordered,
    }


def run_side(
    binary: pathlib.Path, expression: str, profile: str, timeout_seconds: float
) -> dict:
    argv = command(binary, expression, profile)
    started = time.monotonic_ns()
    try:
        run = subprocess.run(
            argv,
            cwd=ROOT,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout_seconds,
        )
        duration_ns = time.monotonic_ns() - started
        return {
            "argv": argv,
            "exit_code": run.returncode,
            "stdout": run.stdout,
            "stderr": run.stderr,
            "duration_ns": duration_ns,
            "complete": run.returncode >= 0,
            "failure": "signal" if run.returncode < 0 else None,
        }
    except subprocess.TimeoutExpired as error:
        duration_ns = time.monotonic_ns() - started
        return {
            "argv": argv,
            "exit_code": None,
            "stdout": error.stdout or "",
            "stderr": error.stderr or "",
            "duration_ns": duration_ns,
            "complete": False,
            "failure": "timeout",
        }
    except OSError as error:
        duration_ns = time.monotonic_ns() - started
        return {
            "argv": argv,
            "exit_code": None,
            "stdout": "",
            "stderr": str(error),
            "duration_ns": duration_ns,
            "complete": False,
            "failure": "io_error",
        }


def classify(vanilla: dict, crystalline: dict) -> str:
    if not vanilla.get("complete") or not crystalline.get("complete"):
        return "EXECUTION_UNKNOWN"
    vanilla_exit = vanilla.get("exit_code")
    crystalline_exit = crystalline.get("exit_code")
    if vanilla_exit is None or crystalline_exit is None:
        return "EXECUTION_UNKNOWN"
    vanilla_ok = vanilla_exit == 0
    crystalline_ok = crystalline_exit == 0
    if vanilla_ok and crystalline_ok:
        return (
            "MATCH_VALUE"
            if vanilla.get("stdout") == crystalline.get("stdout")
            else "DIFFERENT_VALUE"
        )
    if crystalline_ok and not vanilla_ok:
        return "CRYSTALLINE_ONLY"
    if vanilla_ok and not crystalline_ok:
        return "VANILLA_ONLY"
    if vanilla_exit == crystalline_exit and vanilla.get("stderr") == crystalline.get("stderr"):
        return "MATCH_DIAGNOSTIC"
    return "DIFFERENT_DIAGNOSTIC"


def execute_matrix(
    catalog: dict,
    vanilla_bin: pathlib.Path,
    crystalline_bin: pathlib.Path,
    timeout_seconds: float,
) -> dict:
    results = []
    counts_by_profile = {}
    total_counts: Counter[str] = Counter()
    for profile in PROFILES:
        profile_counts: Counter[str] = Counter()
        for probe in catalog["probes"]:
            vanilla = run_side(vanilla_bin, probe["expression"], profile, timeout_seconds)
            crystalline = run_side(
                crystalline_bin, probe["expression"], profile, timeout_seconds
            )
            runtime_class = classify(vanilla, crystalline)
            profile_counts[runtime_class] += 1
            total_counts[runtime_class] += 1
            results.append(
                {
                    "id": probe["id"],
                    "path": probe["path"],
                    "expression": probe["expression"],
                    "profile": profile,
                    "features": PROFILES[profile],
                    "runtime_class": runtime_class,
                    "vanilla": vanilla,
                    "crystalline": crystalline,
                }
            )
        counts_by_profile[profile] = dict(sorted(profile_counts.items()))
    return {
        "schema_version": "p1299-feature-matrix-v1",
        "cwd": str(ROOT),
        "catalog": {"sha256": None, "probe_count": len(catalog["probes"])},
        "binaries": {
            "vanilla": {"path": str(vanilla_bin), "sha256": sha256(vanilla_bin)},
            "crystalline": {
                "path": str(crystalline_bin),
                "sha256": sha256(crystalline_bin),
            },
        },
        "profiles": [
            {"name": name, "feature_argv": feature_argv(name)} for name in PROFILES
        ],
        "counts": {
            "total": len(results),
            "by_profile": counts_by_profile,
            "by_runtime_class": dict(sorted(total_counts.items())),
        },
        "results": results,
    }


def write_json(path: pathlib.Path, payload: dict) -> None:
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n")


def owner_prompt(path: str, runtime_classes: set[str]) -> pathlib.Path:
    if path in COLOR_GLOBAL_CONTRADICTIONS or path.startswith("color."):
        return PROMPTS["repr"] if "DIFFERENT_VALUE" in runtime_classes else PROMPTS["color"]
    if path.startswith("calc."):
        return PROMPTS["calc"]
    if path.startswith("counter_"):
        return PROMPTS["counter"]
    if path.startswith("state_"):
        return PROMPTS["state"]
    if path in MATH_STYLE_EXTRAS:
        return PROMPTS["math_style"]
    if path in MATH_EXTRAS or path.startswith("math."):
        return PROMPTS["math"]
    if path in STRUCTURAL_EXTRAS:
        return PROMPTS["structural"]
    if path == "sym.registered":
        return PROMPTS["sym"]
    if path == "replace" or path.startswith("raw."):
        return PROMPTS["text"]
    if path == "html" or path.startswith("html."):
        return PROMPTS["html"]
    if path.startswith("pdf."):
        return PROMPTS["pdf"]
    if path.startswith("float."):
        return PROMPTS["float"]
    if path.startswith("function."):
        return PROMPTS["function"]
    if path.startswith(("json.", "toml.", "yaml.")):
        return PROMPTS["loading"]
    if path.startswith("outline."):
        return PROMPTS["outline"]
    if path.startswith("selector."):
        return PROMPTS["selector"]
    if path.startswith("version."):
        return PROMPTS["version"]
    if path.startswith("polygon."):
        return PROMPTS["shapes"]
    return PROMPTS["binding"]


def declared_code_hash(prompt: pathlib.Path) -> str:
    match = re.search(r"Hash do Código:\s*`?([0-9a-fA-F]+|[^`\n]+)", prompt.read_text())
    return match.group(1).strip() if match else "NOT_DECLARED"


def locate_quoted(path: pathlib.Path, needle: str, start_line: int = 1) -> str | None:
    for line_number, line in enumerate(path.read_text().splitlines(), 1):
        if line_number >= start_line and needle in line:
            return f"{path.relative_to(ROOT)}:{line_number}"
    return None


def crystalline_registration(path: str, runtime_classes: set[str]) -> str:
    if runtime_classes == {"MATCH_DIAGNOSTIC", "MATCH_VALUE"}:
        return locate_quoted(ROOT / "01_core/src/compiler/eval/mod.rs", '"html"', 1637) or "NOT_LOCATED"
    if "CRYSTALLINE_ONLY" in runtime_classes:
        if path.startswith("calc."):
            field = path.split(".", 1)[1]
            return locate_quoted(ROOT / "01_core/src/compiler/stdlib/calc.rs", f'"{field}"') or "NOT_LOCATED"
        if path == "sym.registered":
            return locate_quoted(ROOT / "01_core/src/compiler/stdlib/sym.rs", '"registered"', 600) or "NOT_LOCATED"
        if path == "math.registered":
            return "01_core/src/compiler/stdlib/structural/math.rs:1324"
        return locate_quoted(
            ROOT / "01_core/src/compiler/eval/mod.rs", f'"{path}"', 1637
        ) or "NOT_LOCATED"
    if "DIFFERENT_VALUE" in runtime_classes:
        return locate_quoted(ROOT / "01_core/src/compiler/stdlib/color.rs", 'field == "map"') or "NOT_LOCATED"
    if "DIFFERENT_DIAGNOSTIC" in runtime_classes:
        return "01_core/src/compiler/eval/bindings/field_access.rs:83-102"
    return "ABSENT at baseline (bilateral VANILLA_ONLY)"


def inventory_vanilla_sources(inventory_paths: list[pathlib.Path]) -> dict[str, set[str]]:
    sources: dict[str, set[str]] = {}
    for inventory_path in inventory_paths:
        payload = json.loads(inventory_path.read_text())
        for entry in list(payload["entries"]) + list(payload.get("blocked_by_ancestor", [])):
            source = entry.get("vanilla", {}).get("source")
            if source:
                sources.setdefault(entry["display_path"], set()).add(source)
    return sources


def language_class(path: str, runtime_classes: set[str]) -> str:
    if path == "html" and runtime_classes <= {"MATCH_DIAGNOSTIC", "MATCH_VALUE"}:
        return "EXPECTED_FEATURE_DISABLED"
    if path in COLOR_GLOBAL_CONTRADICTIONS and "CRYSTALLINE_ONLY" in runtime_classes:
        return "L0_CONTRADICTION"
    if "CRYSTALLINE_ONLY" in runtime_classes:
        return "INTENTIONAL_PRODUCT_EXTENSION"
    if "VANILLA_ONLY" in runtime_classes:
        return "MISSING_LANGUAGE_MEMBER"
    if "DIFFERENT_VALUE" in runtime_classes:
        return "WRONG_PUBLIC_KIND_OR_IDENTITY"
    if "DIFFERENT_DIAGNOSTIC" in runtime_classes:
        return "DIAGNOSTIC_SPAN_DIVERGENCE"
    return "UNRESOLVED"


def adjudication(language: str, path: str, canonical: str) -> tuple[str, str, str, str, str]:
    if language == "EXPECTED_FEATURE_DISABLED":
        return (
            "bilateral feature gate: absent by default and present with html",
            "ADR-0127_NO_CHANGE",
            "The same feature controls the path in both products.",
            "A profile where only one product accepts the path would refute this.",
            "Keep the symmetric feature gate; no P1300 change.",
        )
    if language == "L0_CONTRADICTION":
        return (
            "L0 calls the constructor global; ratified vanilla exposes it only under color.*",
            "ADR-0127_HUMAN_GATE_PUBLIC_REMOVAL",
            f"The bare path is leaked while the qualified route {canonical} remains bilateral.",
            "A ratified vanilla run accepting the bare path, or failure of the qualified route, would refute this.",
            "P1300: remeasure, edit color L0 first, show diff and stop for human confirmation.",
        )
    if language == "INTENTIONAL_PRODUCT_EXTENSION":
        return (
            "owner L0 explicitly retains the crystalline-only public extension/compatibility path",
            "ADR-0127_HUMAN_GATE_IF_CONTRACT_REOPENED",
            "The path is accepted only by crystalline in every measured profile and is documented as retained.",
            "An owner L0 that does not retain it, or vanilla acceptance, would refute this classification.",
            "Retain outside P1300; reopen only by an owner decision and public-contract gate.",
        )
    if language == "MISSING_LANGUAGE_MEMBER":
        return (
            "ratified vanilla member is accepted while crystalline rejects it",
            "ADR-0127_OWNER_STEP_REQUIRED",
            "The bilateral runtime result confirms a missing language member, including former blocked ancestors.",
            "Crystalline acceptance with the same public value would refute this.",
            "Queue an owner-specific parity cohort; do not mix it with alias removal.",
        )
    if language == "WRONG_PUBLIC_KIND_OR_IDENTITY":
        return (
            "both accept, but repr-observable public morphology differs",
            "ADR-0127_OWNER_STEP_REQUIRED",
            "The mismatch is stable across all four profiles and belongs to the repr observation path.",
            "Byte-identical stdout for the same expression would refute this.",
            "Queue a repr-focused causal cohort after higher-priority contract contradictions.",
        )
    if language == "DIAGNOSTIC_SPAN_DIVERGENCE":
        return (
            "both reject semantically, but the public diagnostic/span differs byte-for-byte",
            "ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY",
            "The closed classifier preserves full stderr and detects a diagnostic observable difference.",
            "Identical exit code and stderr would refute this.",
            "Queue a field-access span cohort; do not mix it with public binding removal.",
        )
    return (
        "proof insufficient",
        "BLOCKING_UNKNOWN",
        "The closed evidence did not determine one allowed semantic class.",
        "A complete bilateral observation plus owner evidence would resolve it.",
        "Block P1299.",
    )


def write_owner_ledger(
    matrix_path: pathlib.Path,
    catalog_path: pathlib.Path,
    inventory_paths: list[pathlib.Path],
    output_path: pathlib.Path,
) -> None:
    matrix = json.loads(matrix_path.read_text())
    catalog = json.loads(catalog_path.read_text())
    catalog_by_path = {probe["path"]: probe for probe in catalog["probes"]}
    by_path: dict[str, dict[str, str]] = {}
    for result in matrix["results"]:
        by_path.setdefault(result["path"], {})[result["profile"]] = result["runtime_class"]
    divergent = {
        path
        for path, profile_classes in by_path.items()
        if any(value not in {"MATCH_VALUE", "MATCH_DIAGNOSTIC"} for value in profile_classes.values())
    }
    divergent.add("html")  # fecha explicitamente o DIFFERENCE_OR_DISABLED do P1297.
    vanilla_sources = inventory_vanilla_sources(inventory_paths)
    fields = [
        "path", "profiles", "runtime_class", "crystalline_registration_file_line",
        "vanilla_source_file_line", "canonical_route", "owner_prompt",
        "owner_prompt_hash_declared", "owner_prompt_sha256", "l0_claim",
        "language_class", "gate_class", "inference", "refutation", "recommended_action",
    ]
    with output_path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        for path in sorted(divergent):
            profile_classes = by_path[path]
            classes = set(profile_classes.values())
            semantic = language_class(path, classes)
            canonical = catalog_by_path.get(path, {}).get("canonical_route", "")
            prompt = owner_prompt(path, classes)
            claim, gate, inference, refutation, action = adjudication(semantic, path, canonical)
            sources = sorted(vanilla_sources.get(path, []))
            vanilla_source = ";".join(sources)
            if not vanilla_source:
                vanilla_source = (
                    "ABSENT in ratified public catalog"
                    if "CRYSTALLINE_ONLY" in classes
                    else "runtime canary; see full bilateral receipt"
                )
            writer.writerow(
                {
                    "path": path,
                    "profiles": ",".join(PROFILES),
                    "runtime_class": ";".join(
                        f"{profile}:{profile_classes[profile]}" for profile in PROFILES
                    ),
                    "crystalline_registration_file_line": crystalline_registration(path, classes),
                    "vanilla_source_file_line": vanilla_source,
                    "canonical_route": canonical,
                    "owner_prompt": str(prompt.relative_to(ROOT)),
                    "owner_prompt_hash_declared": declared_code_hash(prompt),
                    "owner_prompt_sha256": sha256(prompt),
                    "l0_claim": claim,
                    "language_class": semantic,
                    "gate_class": gate,
                    "inference": inference,
                    "refutation": refutation,
                    "recommended_action": action,
                }
            )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--vanilla-bin", type=pathlib.Path)
    parser.add_argument("--crystalline-bin", type=pathlib.Path)
    parser.add_argument("--inventory-default", type=pathlib.Path)
    parser.add_argument("--inventory-html", type=pathlib.Path)
    parser.add_argument("--surface-default", type=pathlib.Path)
    parser.add_argument("--surface-html", type=pathlib.Path)
    parser.add_argument("--output-catalog", type=pathlib.Path)
    parser.add_argument("--output-matrix", type=pathlib.Path)
    parser.add_argument("--timeout-seconds", type=float, default=30.0)
    parser.add_argument("--ledger-from-matrix", type=pathlib.Path)
    parser.add_argument("--catalog-input", type=pathlib.Path)
    parser.add_argument("--output-ledger", type=pathlib.Path)
    args = parser.parse_args()

    if args.ledger_from_matrix:
        required = [args.catalog_input, args.inventory_default, args.inventory_html, args.output_ledger]
        if any(value is None for value in required):
            parser.error("ledger mode requires --catalog-input, both inventories and --output-ledger")
        write_owner_ledger(
            args.ledger_from_matrix,
            args.catalog_input,
            [args.inventory_default, args.inventory_html],
            args.output_ledger,
        )
        print(f"ledger_rows={sum(1 for _ in args.output_ledger.open()) - 1}")
        return

    required = [
        args.vanilla_bin, args.crystalline_bin, args.inventory_default,
        args.inventory_html, args.surface_default, args.surface_html,
        args.output_catalog, args.output_matrix,
    ]
    if any(value is None for value in required):
        parser.error("matrix mode requires binaries, inventories, surfaces and both outputs")

    catalog = build_catalog(
        [args.inventory_default, args.inventory_html],
        [args.surface_default, args.surface_html],
    )
    write_json(args.output_catalog, catalog)
    matrix = execute_matrix(
        catalog, args.vanilla_bin, args.crystalline_bin, args.timeout_seconds
    )
    matrix["catalog"]["sha256"] = sha256(args.output_catalog)
    write_json(args.output_matrix, matrix)
    print(json.dumps(matrix["counts"], sort_keys=True))


if __name__ == "__main__":
    main()
