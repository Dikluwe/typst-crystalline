#!/usr/bin/env python3
"""Congela o contrato produtivo P1280 antes da implementação candidata."""

from __future__ import annotations

import csv
import hashlib
import json
import subprocess
from datetime import datetime
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
POLAR = frozenset(
    f"{kind}/{space}"
    for kind in ("linear", "radial")
    for space in ("oklch", "hsl", "hsv")
)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(
            handle, list(rows[0]), delimiter="\t", lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(rows)


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=True, text=True, capture_output=True
    ).stdout.strip()


def accepts(allowlist: frozenset[str], gates: dict[str, bool]) -> bool:
    return allowlist == POLAR and all(gates.values())


def main() -> None:
    contract = [
        ("C01", "allowlist", "exactly Linear/Radial x Oklch/Hsl/Hsv"),
        ("C02", "graph", "resolved local native gradient with correct variant"),
        ("C03", "fill", "fill references the native server"),
        ("C04", "stroke", "stroke references the native server"),
        ("C05", "fallback", "no gradient-color-space fallback on polar pairs"),
        ("C06", "adaptive", "svg_adaptive_stops remains on the productive route"),
        ("C07", "carriers", "offsets order discontinuities and alpha preserved"),
        ("C08", "luma", "Linear/Radial Luma retain explicit fallback"),
        ("C09", "cmyk", "Linear/Radial CMYK retain explicit fallback"),
        ("C10", "scope", "Conic Tiling and unlisted pairs do not move"),
        ("C11", "invalid", "56/56 frozen invalid probes remain rejected"),
        ("C12", "determinism", "384/384 inverse/repeat receipts agree"),
        ("C13", "budgets", "threshold cap and sealed numeric budgets unchanged"),
        ("C14", "claim", "fragment verdict never becomes general SVG equivalence"),
    ]
    write_tsv(
        DIAG / "p1280-contract.tsv",
        [
            {"claim": claim, "observable": observable, "requirement": requirement}
            for claim, observable, requirement in contract
        ],
    )
    write_tsv(
        DIAG / "p1280-unknown-policy.tsv",
        [
            {
                "case": case,
                "classification": classification,
                "counts_as_success": "false",
            }
            for case, classification in (
                ("Linear/Radial Luma", "Unknown-generalization"),
                ("Linear/Radial CMYK", "Unknown-ADR0097"),
                ("unresolved-or-opaque-SVG", "Unknown"),
                ("outside-P1280-fragment", "Unknown"),
            )
        ],
    )
    write_tsv(
        DIAG / "p1280-policy.tsv",
        [
            {
                "pair": f"{kind}/{space}",
                "expected_route": "native-adaptive" if space in ("oklch", "hsl", "hsv") else "fallback",
                "productive_promotion": str(space in ("oklch", "hsl", "hsv")).lower(),
            }
            for kind in ("linear", "radial")
            for space in ("oklch", "hsl", "hsv", "luma", "cmyk")
        ],
    )

    gates = {
        "resolved_variant": True,
        "fill": True,
        "stroke": True,
        "no_polar_fallback": True,
        "adaptive": True,
        "carriers": True,
        "luma_fallback": True,
        "cmyk_fallback": True,
        "no_scope_creep": True,
        "invalid": True,
        "determinism": True,
        "budgets_frozen": True,
        "unknown_preserved": True,
        "fragment_claim": True,
    }
    mutants: list[tuple[str, str, frozenset[str], dict[str, bool]]] = []
    for mutant, witness, removed in (
        ("M01-drop-linear", "one geometry omitted", {p for p in POLAR if p.startswith("linear/")}),
        ("M02-drop-radial", "one geometry omitted", {p for p in POLAR if p.startswith("radial/")}),
        ("M03-drop-oklch", "Oklch omitted", {p for p in POLAR if p.endswith("/oklch")}),
        ("M04-drop-hsl", "Hsl omitted", {p for p in POLAR if p.endswith("/hsl")}),
        ("M05-drop-hsv", "Hsv omitted", {p for p in POLAR if p.endswith("/hsv")}),
    ):
        mutants.append((mutant, witness, POLAR - removed, gates))
    mutants.extend(
        (
            mutant,
            witness,
            allowlist,
            {**gates, **changes},
        )
        for mutant, witness, allowlist, changes in (
            ("M06-promote-luma", "Luma enters allowlist", POLAR | {"linear/luma", "radial/luma"}, {}),
            ("M07-promote-cmyk", "CMYK enters allowlist", POLAR | {"linear/cmyk", "radial/cmyk"}, {}),
            ("M08-wildcard-spaces", "allowlist is wider than six pairs", POLAR | {"linear/srgb"}, {}),
            ("M09-unresolved-reference", "native graph has a pending URL", POLAR, {"resolved_variant": False}),
            ("M10-retain-polar-fallback", "fallback survives beside promotion", POLAR, {"no_polar_fallback": False}),
            ("M11-solid-disguise", "paint is reduced to a solid", POLAR, {"resolved_variant": False}),
            ("M12-bypass-adaptive", "productive route skips adaptive stops", POLAR, {"adaptive": False}),
            ("M13-drop-alpha", "alpha carrier is ignored", POLAR, {"carriers": False}),
            ("M14-fill-only", "stroke has no independent receipt", POLAR, {"stroke": False}),
            ("M15-accept-bytes-or-ids", "language graph is not checked", POLAR, {"fragment_claim": False}),
            ("M16-widen-budget", "sealed threshold or cap changes", POLAR, {"budgets_frozen": False}),
            ("M17-unknown-as-success", "Unknown enters success", POLAR, {"unknown_preserved": False}),
            ("M18-general-equivalence", "fragment claim is overgeneralized", POLAR, {"fragment_claim": False}),
        )
    )
    attacks = []
    for mutant, witness, allowlist, mutant_gates in mutants:
        killed = not accepts(allowlist, mutant_gates)
        attacks.append(
            {
                "mutant_id": mutant,
                "executed": "true",
                "witness": witness,
                "status": "REJECTED" if killed else "SURVIVED",
            }
        )
    if any(row["status"] != "REJECTED" for row in attacks):
        raise RuntimeError("P1280 contract mutant survived")
    write_tsv(DIAG / "p1280-preseal-attacks.tsv", attacks)

    write_tsv(
        DIAG / "p1280-role-capabilities.tsv",
        [
            {"role": "intent-owner", "artifact": "L0 + passo", "write_scope": "specification", "isolation": "shared-checkout"},
            {"role": "contract-author", "artifact": "contract/policy/preseal attacks", "write_scope": "diagnostics only", "isolation": "logical-order-only"},
            {"role": "implementer", "artifact": "svg.rs predicate", "write_scope": "consumer after seal", "isolation": "not isolated"},
            {"role": "verifier", "artifact": "postseal receipts/certificate", "write_scope": "diagnostics after candidate", "isolation": "not isolated"},
        ],
    )

    inputs = (
        ROOT / "typst-passo-1280.md",
        ROOT / "00_nucleo/prompts/infra/export/svg.md",
        ROOT / "00_nucleo/diagnosticos/typst-p1279-offset-identity-certificate.md",
        ROOT / "00_nucleo/diagnosticos/p1279-final-summary.json",
        ROOT / "00_nucleo/diagnosticos/p1279-final-pairs.tsv",
        ROOT / "00_nucleo/diagnosticos/p1279-product-boundary.tsv",
        ROOT / "00_nucleo/diagnosticos/p1279-evidence-manifest.tsv",
        ROOT / "00_nucleo/diagnosticos/p1280-preseal.py",
        Path("/usr/local/bin/typst"),
    )
    write_tsv(
        DIAG / "p1280-preseal-inputs.tsv",
        [{"path": str(path), "sha256": sha(path)} for path in inputs],
    )
    manifest_inputs = (
        DIAG / "p1280-contract.tsv",
        DIAG / "p1280-unknown-policy.tsv",
        DIAG / "p1280-policy.tsv",
        DIAG / "p1280-preseal-attacks.tsv",
        DIAG / "p1280-role-capabilities.tsv",
        DIAG / "p1280-preseal-inputs.tsv",
    )
    write_tsv(
        DIAG / "p1280-preseal-manifest.tsv",
        [{"path": str(path.relative_to(ROOT)), "sha256": sha(path)} for path in manifest_inputs],
    )
    measured_at = datetime.now().astimezone().isoformat(timespec="seconds")
    summary = {
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        "baseline": "upstream/main a51e02804 ratified",
        "head": git("rev-parse", "HEAD"),
        "measured_at": measured_at,
        "mutation_score": 1.0,
        "mutants_rejected": len(attacks),
        "mutants_total": len(attacks),
        "policy_pairs_promoted": len(POLAR),
        "preseal": "ACCEPTED-FOR-IMPLEMENTATION",
        "scope": "Linear/Radial x Oklch/Hsl/Hsv productive route only",
    }
    (DIAG / "p1280-preseal.json").write_text(
        json.dumps(summary, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
