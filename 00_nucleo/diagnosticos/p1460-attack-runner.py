#!/usr/bin/env python3
"""Travas estruturais/modelo P1460; não executa mutação compilada."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ORACLES = ROOT / "00_nucleo/diagnosticos/p1460-oracles.json"
OWNER = ROOT / "01_core/src/compiler/lang/heading_supplement.rs"
HUB = ROOT / "01_core/src/compiler/lang/mod.rs"
CLI = ROOT / "02_shell/src/cli.rs"


def main() -> int:
    oracles = json.loads(ORACLES.read_text(encoding="utf-8"))
    owner = OWNER.read_text(encoding="utf-8")
    hub = HUB.read_text(encoding="utf-8")
    cli = CLI.read_text(encoding="utf-8")
    productive = cli.split("fn content_to_semantic", 1)[1].split(
        "fn content_semantic_func_name", 1
    )[0]
    normalized = re.sub(r"\s+", "", productive)

    direct_none = (
        re.search(
            r"Content::text\(heading_supplement_for_lang\(None,?\)\)",
            normalized,
        )
        is not None
    )
    no_successor = all(
        token not in productive
        for token in ['"Section"', "const NAMES", "Lang::", "fn heading_supplement"]
    )
    narrow_api = all(
        [
            "mod heading_supplement;" in hub,
            "pub mod heading_supplement;" not in hub,
            "pub use heading_supplement::heading_supplement_for_lang;" in hub,
            re.search(r"\bpub fn heading_supplement_for_lang\s*\(", owner)
            is not None,
            re.search(r"\bpub(?:\([^)]*\))?\s+const\s+NAMES", owner) is None,
        ]
    )
    snapshot_first = (
        "Value::LocatedContent(content, _) => match content.fields()" in cli
        and "Some(fields) =>" in cli
        and "None => content_to_semantic(content.content())" in cli
    )
    transcript_tests = all(
        token in cli
        for token in [
            "p1460_raw_heading_delega_owner_e_preserva_transcripts",
            "p1460_snapshot_causal_continua_autoritativo",
            "cannot print content in raw format",
            "Abschnitt",
        ]
    )
    owner_corpus = owner.count("(") >= 45 and 'unwrap_or("Section")' in owner
    frozen_authority = (
        oracles["phase"] == "before_candidate"
        and oracles["sealed_before_candidate"] is True
        and oracles["unknown_policy"].startswith("Unknown is fail-closed")
    )

    conditions = {
        "M01-retain-current-literal": direct_none and no_successor,
        "M02-move-literal-to-local-constant-or-helper": direct_none and no_successor,
        "M03-duplicate-table-or-language-branch-in-L2": no_successor,
        "M04-call-owner-with-Some-English": direct_none,
        "M05-call-owner-with-German-or-process-language": direct_none,
        "M06-call-equation-or-figure-owner": direct_none,
        "M07-owner-not-public-through-required-hub-path": narrow_api,
        "M08-make-physical-module-public-instead-of-narrow-reexport": narrow_api,
        "M09-change-owner-None-fallback-or-ratified-entry": owner_corpus,
        "M10-remove-or-reorder-raw-fields": transcript_tests,
        "M11-infer-language-from-raw-Styled-wrapper": direct_none and no_successor,
        "M12-route-snapshot-present-content-through-raw-fallback": snapshot_first
        and transcript_tests,
        "M13-fix-only-JSON-or-only-YAML": transcript_tests,
        "M14-change-raw-format-error": transcript_tests,
        "M15-derive-expected-output-from-candidate": frozen_authority,
        "M16-promote-Unknown": frozen_authority,
    }
    required = [entry["id"] for entry in oracles["discriminatory_mutants"]]
    killed = [mutant for mutant in required if conditions.get(mutant) is True]
    survivors = [mutant for mutant in required if mutant not in killed]
    result = {
        "schema": "p1460-static-attack-results/v1",
        "evidence_kind": "static_or_model_discriminators",
        "compiled_mutation_execution": False,
        "attestation": "executado_sem_atestacao_de_isolamento",
        "valid_attacks": len(required),
        "rejected": len(killed),
        "survivors": survivors,
        "discriminator_score": len(killed) / len(required),
        "locks": {
            "direct_none_owner_call": direct_none,
            "no_l2_successor": no_successor,
            "narrow_public_api": narrow_api,
            "snapshot_present_bypasses_fallback": snapshot_first,
            "frozen_transcript_tests_present": transcript_tests,
            "owner_corpus_guard_present": owner_corpus,
            "pre_candidate_authority_frozen": frozen_authority,
        },
    }
    json.dump(result, sys.stdout, ensure_ascii=False, indent=2, sort_keys=True)
    sys.stdout.write("\n")
    return 0 if not survivors else 1


if __name__ == "__main__":
    raise SystemExit(main())
