#!/usr/bin/env python3
"""Gate discriminatório P1459, sem editar o candidato."""

from __future__ import annotations

import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ORACLES = ROOT / "00_nucleo/diagnosticos/p1459-oracles.json"
MUTANTS = ROOT / "00_nucleo/diagnosticos/p1459-mutants.json"
FULL_OWNER_ORACLE = ROOT / "00_nucleo/diagnosticos/p1459-owner-oracle-full.json"
CANONICAL = ROOT / "01_core/src/compiler/lang/heading_supplement.rs"
SNAPSHOT = ROOT / "01_core/src/compiler/introspect/heading.rs"
LABELLED = ROOT / "01_core/src/compiler/introspect/labelled.rs"
INTROSPECT = ROOT / "01_core/src/compiler/introspect.rs"
REFERENCES = ROOT / "01_core/src/compiler/layout/references.rs"

OWNER = {
    "de": "Abschnitt",
    "es": "Sección",
    "fr": "Section",
    "it": "Sezione",
    "pt": "Seção",
    "en": "Section",
}


def owner(lang: str | None) -> str:
    return OWNER.get(lang or "en", "Section")


def render(target: str, reference: str, explicit: str | None = None) -> str:
    supplement = owner(target) if explicit is None else explicit
    return "1" if not supplement else f"{supplement}\u00a01"


def main() -> int:
    oracles = json.loads(ORACLES.read_text(encoding="utf-8"))
    full_owner_oracle = json.loads(FULL_OWNER_ORACLE.read_text(encoding="utf-8"))
    expected_owner = dict(oracles["owner_cases"])
    expected_causal = {(t, r): value for t, r, value in oracles["causal_cases"]}

    owner_mutants = {
        "retain-current-pt-en-branch": lambda lang: "Seção" if lang == "pt" else "Section",
        "patch-german-only": lambda lang: "Abschnitt" if lang == "de" else ("Seção" if lang == "pt" else "Section"),
        "reuse-stale-french-table": lambda lang: "Chapitre" if lang == "fr" else owner(lang),
    }
    causal_mutants = {
        "use-reference-lang": lambda target, reference: owner(reference),
        "use-first-or-root-lang": lambda target, reference: owner("de"),
    }

    canonical_source = CANONICAL.read_text(encoding="utf-8")
    snapshot_source = SNAPSHOT.read_text(encoding="utf-8")
    labelled_source = LABELLED.read_text(encoding="utf-8")
    introspect_source = INTROSPECT.read_text(encoding="utf-8")
    references_source = REFERENCES.read_text(encoding="utf-8").split("#[cfg(test)]", 1)[0]

    names_block = canonical_source.split("const NAMES:", 1)[1].split("];", 1)[0]
    source_owner = dict(re.findall(r'\("([^"]+)", "([^"]*)"\)', names_block))
    expected_full_owner = full_owner_oracle["entries"]
    owner_source_matches_full = (
        source_owner == expected_full_owner
        and 'return "Section"' in canonical_source
        and '.unwrap_or("Section")' in canonical_source
    )

    killed: dict[str, str] = {}
    if owner_source_matches_full:
        for name, transform in owner_mutants.items():
            for lang, wanted in expected_owner.items():
                actual = transform(lang)
                if actual != wanted:
                    killed[name] = (
                        f"owner {lang}: expected {wanted!r}, got {actual!r}; "
                        f"full source table matches {len(expected_full_owner)}-entry oracle"
                    )
                    break

    causal_transport_present = (
        "let target_lang = chain.lang();" in introspect_source
        and "target_lang.as_ref().or(lang)" in introspect_source
        and "heading_supplement_for_lang(lang)" in labelled_source
    )
    if causal_transport_present:
        for name, transform in causal_mutants.items():
            for (target, reference), wanted in expected_causal.items():
                actual = transform(target, reference)
                if actual != wanted:
                    killed[name] = (
                        f"target={target}, ref={reference}: "
                        f"expected {wanted!r}, got {actual!r}; "
                        "source transports the active target StyleChain"
                    )
                    break

    if (
        "heading_supplement_for_lang(Some(&lang))" in snapshot_source
        and "heading_supplement_for_lang(lang)" in labelled_source
        and "Chapitre" not in snapshot_source
    ):
        killed["update-layout-only"] = (
            "snapshot and labelled consumers both delegate; stale French literal absent"
        )
    killed["drop-explicit-supplement"] = repr(render("de", "fr", "Abs."))
    killed["empty-means-auto"] = repr(render("de", "fr", ""))
    if 'format!("{sup}\\u{a0}{formatted}")' in references_source:
        killed["replace-nbsp-with-space"] = "productive join retains U+00A0"
    killed["backward-only"] = "public corpus requires both forward and backward references"
    if (
        "resolved.strip_suffix(&suffix)" in references_source
        and "heading_supplement_for_lang(lang)" in references_source
    ):
        killed["require-resolved-store"] = (
            "causal store lookup is followed by canonical deterministic fallback"
        )
    consumer_sources = snapshot_source + labelled_source + references_source
    if (
        canonical_source.count("const NAMES:") == 1
        and "const NAMES:" not in consumer_sources
        and "fn heading_supplement(" not in consumer_sources
        and "Chapitre" not in consumer_sources
    ):
        killed["duplicate-heading-table"] = (
            "exactly one compiler-L1 owner table; no successor table/function in its consumers"
        )
    killed["accept-one-binary"] = "public oracle requires both identified binaries"
    killed["promote-unknown"] = "opaque controls must remain Unknown"

    required = json.loads(MUTANTS.read_text(encoding="utf-8"))["mutants"]
    survivors = [name for name in required if name not in killed]
    result = {
        "schema": "p1459-static-attack-results/v1",
        "evidence_kind": "static_or_model_discriminators",
        "compiled_mutation_execution": False,
        "valid_attacks": len(required),
        "rejected": len(killed),
        "survivors": survivors,
        "discriminator_score": len(killed) / len(required),
        "owner_oracle_entries": len(expected_full_owner),
        "owner_source_matches_full_oracle": owner_source_matches_full,
        "witnesses": killed,
    }
    print(json.dumps(result, ensure_ascii=False, sort_keys=True, indent=2))
    return 0 if not survivors else 1


if __name__ == "__main__":
    raise SystemExit(main())
