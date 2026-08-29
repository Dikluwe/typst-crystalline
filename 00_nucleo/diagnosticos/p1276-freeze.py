#!/usr/bin/env python3
"""Congela o pacote pré-candidato P1276 sem executar o produto."""

from __future__ import annotations

import csv
import hashlib
import json
import subprocess
from copy import deepcopy
from datetime import datetime
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
PAIRS = (
    ("linear/oklch", "polar", "P1238 42/42 L1; SVG budget absent"),
    ("radial/oklch", "polar", "P1238 42/42 L1; SVG budget absent"),
    ("linear/hsl", "polar", "P1238 42/42 L1; SVG budget absent"),
    ("radial/hsl", "polar", "P1238 42/42 L1; SVG budget absent"),
    ("linear/hsv", "polar", "P1238 42/42 L1; SVG budget absent"),
    ("radial/hsv", "polar", "P1238 42/42 L1; SVG budget absent"),
    ("linear/luma", "luma", "P1239 L1 preserved; SVG budget absent"),
    ("radial/luma", "luma", "P1239 L1 preserved; SVG budget absent"),
)
POLAR_SCENARIOS = (
    "seam-forward", "seam-reverse", "no-wrap", "low-chroma",
    "alpha-first", "alpha-last",
)
LUMA_SCENARIOS = (
    "black-white", "white-black", "primary-extremes",
    "nonsaturated", "alpha-first", "alpha-last",
)
POSITIONS = "0;0.000001;0.25;0.5;0.75;0.999999;1"


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=True, text=True, capture_output=True
    ).stdout.rstrip()


def write_tsv(name: str, rows: list[dict[str, object]]) -> Path:
    if not rows:
        raise RuntimeError(f"empty artifact: {name}")
    path = DIAG / name
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(
            handle, list(rows[0]), delimiter="\t", lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(rows)
    return path


def contract_rows() -> list[dict[str, str]]:
    obligations = (
        ("C01", "oracle-order", "vanilla budgets masks and costs exist before candidate execution", "candidate-derived oracle violates"),
        ("C02", "pair-independence", "eight geometry/space pairs are adjudicated separately", "cross-pair inheritance violates"),
        ("C03", "general-envelope", "each pair executes all 24 frozen P1266 seed families", "missing seed blocks only its pair"),
        ("C04", "polar-envelope", "each polar pair executes six scenarios at seven positions", "missing seam or alpha scenario blocks the pair"),
        ("C05", "luma-envelope", "each Luma pair executes six scenarios at seven positions", "missing luminance or alpha control blocks the pair"),
        ("C06", "original-stops", "offset order coincidence right-continuity color and alpha of declared stops are preserved", "carrier spelling and adaptive count are mechanical"),
        ("C07", "hue-route", "polar hue follows vanilla short route and modular angular distance", "raw subtraction or forced long route violates"),
        ("C08", "numeric", "premultiplied encoded-sRGB color and alpha pass max and p95 before u8", "one metric cannot mask another"),
        ("C09", "graph", "variant local URL role geometry transform and zero unresolved references are preserved", "SVG bytes IDs and defs order are mechanical"),
        ("C10", "raster", "local mask is frozen from vanilla geometry before candidate", "candidate-guided crop fitting or erosion violates"),
        ("C11", "degenerate", "degenerate stroke preserves painted region and zero-area fill paints no positive area", "one zero-area rule for both violates"),
        ("C12", "cost", "termination and cost are published and limited per original interval", "global stop cap is not a language gate"),
        ("C13", "domain", "invalid offsets cardinality and radial geometry are rejected", "invalid input is not Unknown"),
        ("C14", "determinism", "direct inverse and repeat runs are semantically identical", "order-sensitive verdict violates"),
        ("C15", "product-boundary", "all eight pairs retain explicit gradient-color-space fallback", "native promotion in P1276/P1277 violates"),
        ("C16", "scope", "CMYK stays Unknown-ADR0097 and no general SVG equivalence is claimed", "scope expansion violates"),
    )
    return [
        {"id": item[0], "boundary": item[1], "obligation": item[2], "adjudication": item[3]}
        for item in obligations
    ]


def base_model() -> dict[str, object]:
    return {
        "pairs": {pair for pair, _, _ in PAIRS},
        "cmyk": "Unknown-ADR0097",
        "general_seeds": 24,
        "polar_scenarios": set(POLAR_SCENARIOS),
        "luma_scenarios": set(LUMA_SCENARIOS),
        "positions": 7,
        "metrics": {"color_max", "color_p95", "alpha_max", "alpha_p95"},
        "hue_metric": "modular-shortest-arc",
        "cost": "per-original-interval",
        "degenerate": {"stroke-painted-region", "fill-zero-positive-area"},
        "invalid_domain": True,
        "determinism": {"direct", "inverse", "repeat"},
        "unknown_success": False,
        "product_promotion": False,
        "general_equivalence": False,
        "oracle_order": "vanilla-first",
    }


def validate(model: dict[str, object]) -> None:
    expected_pairs = {pair for pair, _, _ in PAIRS}
    if model["pairs"] != expected_pairs:
        raise ValueError("pair-local population drift")
    if model["cmyk"] != "Unknown-ADR0097":
        raise ValueError("CMYK boundary drift")
    if model["general_seeds"] != 24:
        raise ValueError("general seed cardinality drift")
    if model["polar_scenarios"] != set(POLAR_SCENARIOS):
        raise ValueError("polar envelope drift")
    if model["luma_scenarios"] != set(LUMA_SCENARIOS):
        raise ValueError("Luma envelope drift")
    if model["positions"] != 7:
        raise ValueError("focal position drift")
    if model["metrics"] != {"color_max", "color_p95", "alpha_max", "alpha_p95"}:
        raise ValueError("numeric conjunction drift")
    if model["hue_metric"] != "modular-shortest-arc":
        raise ValueError("hue observable drift")
    if model["cost"] != "per-original-interval":
        raise ValueError("cost contract drift")
    if model["degenerate"] != {"stroke-painted-region", "fill-zero-positive-area"}:
        raise ValueError("degenerate geometry drift")
    if model["invalid_domain"] is not True:
        raise ValueError("invalid domain lost")
    if model["determinism"] != {"direct", "inverse", "repeat"}:
        raise ValueError("determinism phase drift")
    if model["unknown_success"] is not False:
        raise ValueError("Unknown converted to success")
    if model["product_promotion"] is not False:
        raise ValueError("premature productive promotion")
    if model["general_equivalence"] is not False:
        raise ValueError("general SVG equivalence overclaim")
    if model["oracle_order"] != "vanilla-first":
        raise ValueError("candidate-derived oracle")


def attack_rows() -> list[dict[str, str]]:
    mutations = (
        ("D01", "drop-linear-oklch", lambda m: m["pairs"].remove("linear/oklch")),
        ("D02", "merge-radial-with-linear", lambda m: m["pairs"].remove("radial/hsv")),
        ("D03", "admit-cmyk", lambda m: m.update(cmyk="Preserved")),
        ("D04", "drop-general-seed", lambda m: m.update(general_seeds=23)),
        ("D05", "drop-seam-reverse", lambda m: m["polar_scenarios"].remove("seam-reverse")),
        ("D06", "drop-polar-alpha", lambda m: m["polar_scenarios"].remove("alpha-last")),
        ("D07", "drop-luma-extreme", lambda m: m["luma_scenarios"].remove("primary-extremes")),
        ("D08", "drop-endpoint", lambda m: m.update(positions=6)),
        ("D09", "raw-hue-subtraction", lambda m: m.update(hue_metric="raw-subtraction")),
        ("D10", "drop-alpha-p95", lambda m: m["metrics"].remove("alpha_p95")),
        ("D11", "global-cap", lambda m: m.update(cost="global-64-stops")),
        ("D12", "merge-degenerate-rules", lambda m: m["degenerate"].remove("stroke-painted-region")),
        ("D13", "accept-invalid-as-unknown", lambda m: m.update(invalid_domain=False)),
        ("D14", "drop-inverse", lambda m: m["determinism"].remove("inverse")),
        ("D15", "unknown-is-success", lambda m: m.update(unknown_success=True)),
        ("D16", "promote-before-verdict", lambda m: m.update(product_promotion=True)),
        ("D17", "claim-general-equivalence", lambda m: m.update(general_equivalence=True)),
        ("D18", "candidate-first-budget", lambda m: m.update(oracle_order="candidate-first")),
    )
    rows = []
    for mutant_id, mutation, apply in mutations:
        model = deepcopy(base_model())
        apply(model)
        try:
            validate(model)
        except ValueError as error:
            rows.append({
                "mutant_id": mutant_id,
                "mutation": mutation,
                "executed": "true",
                "rejected": "true",
                "witness": str(error),
                "scope": "contract-definition-only",
            })
        else:
            raise RuntimeError(f"surviving definition mutant: {mutant_id}")
    return rows


def main() -> None:
    validate(base_model())
    pair_rows = [
        {
            "pair": pair,
            "family": family,
            "predecessor_evidence": evidence,
            "p1275_route": "FALLBACK-PRESERVED",
            "p1276_state": "FROZEN-NOT-ADJUDICATED",
            "productive_promotion": "false",
        }
        for pair, family, evidence in PAIRS
    ]
    population_rows = []
    for pair, family, _ in PAIRS:
        population_rows.append({
            "pair": pair, "stratum": "general-p1266", "fixture_groups": 24,
            "positions_per_group": "mesh-8192+graph+raster", "source": "p1266-generalization-matrix.tsv",
        })
        scenarios = POLAR_SCENARIOS if family == "polar" else LUMA_SCENARIOS
        population_rows.append({
            "pair": pair, "stratum": f"{family}-focal", "fixture_groups": len(scenarios),
            "positions_per_group": POSITIONS, "source": "P1238" if family == "polar" else "P1239-derived",
        })

    unknown_rows = [
        {"case": "missing-oracle", "classification": "Unknown", "blocks": "affected pair", "counts_as_success": "false"},
        {"case": "opaque-parser-or-SVG", "classification": "Unknown", "blocks": "affected fixture and pair", "counts_as_success": "false"},
        {"case": "budget-exhausted", "classification": "Unknown", "blocks": "affected metric and pair", "counts_as_success": "false"},
        {"case": "ambiguous-identity", "classification": "Unknown", "blocks": "affected graph and pair", "counts_as_success": "false"},
        {"case": "invalid-domain", "classification": "Rejected-by-domain", "blocks": "not admitted to valid population", "counts_as_success": "false"},
        {"case": "CMYK", "classification": "Unknown-ADR0097", "blocks": "both CMYK pairs", "counts_as_success": "false"},
    ]
    role_rows = [
        {"role": "intent-and-contract-author", "executor": "/root", "read_allowlist": "P1238;P1239;P1241;P1266-P1275;L0 SVG;consumer SVG", "write_allowlist": "P1276 diagnostic artifacts", "candidate_read": "not-executed", "attested_isolation": "false"},
        {"role": "oracle-author", "executor": "reserved-for-P1277", "read_allowlist": "ratified vanilla;P1276 frozen pack", "write_allowlist": "P1277 oracle artifacts", "candidate_read": "forbidden-before-oracle-seal", "attested_isolation": "not-yet-run"},
        {"role": "adversary", "executor": "reserved-for-P1277", "read_allowlist": "P1276 contract;vanilla baseline", "write_allowlist": "P1277 semantic mutants", "candidate_read": "forbidden-before-attack-freeze", "attested_isolation": "not-yet-run"},
        {"role": "implementer", "executor": "none-in-P1276", "read_allowlist": "none", "write_allowlist": "none", "candidate_read": "not-applicable", "attested_isolation": "not-applicable"},
        {"role": "verifier", "executor": "reserved-for-P1277", "read_allowlist": "sealed P1276/P1277 artifacts and candidate outputs", "write_allowlist": "P1277 receipts only", "candidate_read": "after-seal", "attested_isolation": "not-yet-run"},
    ]

    artifacts = []
    artifacts.append(write_tsv("p1276-pairs.tsv", pair_rows))
    artifacts.append(write_tsv("p1276-population.tsv", population_rows))
    artifacts.append(write_tsv("p1276-contract.tsv", contract_rows()))
    artifacts.append(write_tsv("p1276-unknown-policy.tsv", unknown_rows))
    attacks = attack_rows()
    artifacts.append(write_tsv("p1276-attacks.tsv", attacks))
    artifacts.append(write_tsv("p1276-role-capabilities.tsv", role_rows))

    input_paths = (
        ("step", ROOT / "typst-passo-1276.md"),
        ("runner", Path(__file__)),
        ("p1275_certificate", DIAG / "typst-p1275-promotion-certificate.md"),
        ("p1275_manifest", DIAG / "p1275-final-manifest.tsv"),
        ("p1241_results", DIAG / "p1241-svg-multispace-results.tsv"),
        ("p1238_summary", DIAG / "p1238-polar-summary.json"),
        ("p1239_summary", DIAG / "p1239-luma-summary.json"),
        ("general_matrix", DIAG / "p1266-generalization-matrix.tsv"),
        ("corrected_contract", DIAG / "p1267-generalization-contract-v2.tsv"),
        ("svg_l0", ROOT / "00_nucleo/prompts/infra/export/svg.md"),
        ("svg_consumer", ROOT / "03_infra/src/export/svg.rs"),
        ("vanilla", Path("/usr/local/bin/typst")),
    )
    missing = [str(path) for _, path in input_paths if not path.is_file()]
    if missing:
        raise RuntimeError(f"missing protected inputs: {missing}")
    input_rows = [
        {"input": label, "path": str(path), "sha256": sha(path), "frozen": "true"}
        for label, path in input_paths
    ]
    input_manifest = write_tsv("p1276-input-manifest.tsv", input_rows)
    artifacts.append(input_manifest)

    receipt_rows = []
    for path in artifacts:
        receipt_rows.append({
            "artifact": path.name, "sha256": sha(path), "status": "FROZEN"
        })
    receipt_rows.extend((
        {"artifact": "definition-mutants", "sha256": sha(DIAG / "p1276-attacks.tsv"), "status": f"{len(attacks)}/{len(attacks)} REJECTED"},
        {"artifact": "candidate-execution", "sha256": "not-applicable", "status": "NOT-RUN"},
        {"artifact": "productive-change", "sha256": sha(ROOT / "03_infra/src/export/svg.rs"), "status": "UNCHANGED"},
    ))
    preseal = write_tsv("p1276-preseal-receipt.tsv", receipt_rows)

    measured_at = datetime.now().astimezone().isoformat(timespec="seconds")
    diff_stat = git("diff", "HEAD", "--stat")
    snapshot = (
        f"measured_at={measured_at}\nhead={git('rev-parse', 'HEAD')}\n"
        "working_tree=uncommitted;P1276-diagnostic-pack\n\n"
        f"git status --short\n{git('status', '--short')}\n\n"
        f"git diff HEAD --stat\n{diff_stat}"
        + ("\n" if diff_stat else "")
    )
    (DIAG / "p1276-working-tree-snapshot.txt").write_text(snapshot, encoding="utf-8")
    summary = {
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        "baseline_commit": git("rev-parse", "HEAD"),
        "candidate_executed": False,
        "definition_mutation_score": f"{len(attacks)}/{len(attacks)}",
        "fixture_groups": 240,
        "general_groups": 192,
        "focal_observations": 336,
        "pairs": 8,
        "productive_change": False,
        "semantic_mutation_score": None,
        "state": "CONTRACT-CANDIDATE-FROZEN",
        "next_route": "P1277 vanilla-first oracles and semantic adjudication",
        "measured_at": measured_at,
    }
    (DIAG / "p1276-summary.json").write_text(
        json.dumps(summary, ensure_ascii=False, sort_keys=True, indent=2) + "\n",
        encoding="utf-8",
    )
    report = f"""# P1276 — preseal da fronteira SVG multi-space restante

**Veredito:** `CONTRACT-CANDIDATE-FROZEN`.

Foram congelados oito pares, 240 grupos de fixture e 336 observações focais.
Os {len(attacks)} mutantes da definição foram executados e rejeitados. Esse
score é limitado à estrutura do contrato; nenhum mutante semântico, oracle ou
candidato produtivo foi executado neste passo.

CMYK permanece `Unknown-ADR0097`. O owner SVG e seu L0 não foram alterados; os
oito pares continuam com fallback explícito. P1277 deve materializar primeiro
os oráculos vanilla, congelá-los e somente então medir o candidato.

Proveniência: HEAD `{git('rev-parse', 'HEAD')}`, working tree não commitada,
medição `{measured_at}`. Manifesto de entradas:
`p1276-input-manifest.tsv` (`sha256:{sha(input_manifest)}`). Recibo do pacote:
`p1276-preseal-receipt.tsv` (`sha256:{sha(preseal)}`).

**Atestação:** EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
"""
    (DIAG / "typst-p1276-preseal.md").write_text(report, encoding="utf-8")


if __name__ == "__main__":
    main()
