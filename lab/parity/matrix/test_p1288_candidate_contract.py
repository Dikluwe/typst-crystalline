"""Black-box A/B contract tests for the post-gate P1288 candidate."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
CANDIDATE = ROOT / "target/release/typst"
SEAL = ROOT / "00_nucleo/diagnosticos/p1288-contract-seal.json"
ORACLE_MODULE = HERE / "p1288_oracles.py"

SPEC = importlib.util.spec_from_file_location("p1288_frozen_oracles_ab", ORACLE_MODULE)
oracles = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(oracles)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


class P1288CandidateContractTests(unittest.TestCase):
    """Require the candidate to refine the already-frozen oracle contract."""

    @classmethod
    def setUpClass(cls):
        cls.seal = json.loads(SEAL.read_text(encoding="utf-8"))
        cls.baseline = oracles.load_baseline()
        cls.input_drift = oracles.verify_hashes(cls.baseline)
        if not CANDIDATE.is_file():
            raise AssertionError(f"candidate binary missing: {CANDIDATE}")
        cls.candidate_sha256 = sha256(CANDIDATE)
        with tempfile.TemporaryDirectory(prefix="p1288-candidate-ab-") as temp:
            root = Path(temp)
            forward_root = root / "forward"
            reverse_root = root / "reverse"
            forward_root.mkdir()
            reverse_root.mkdir()
            cls.forward = oracles.run_suite(
                str(CANDIDATE), cls.baseline, "forward", forward_root
            )
            cls.reverse = oracles.run_suite(
                str(CANDIDATE), cls.baseline, "reverse", reverse_root
            )
        cls.results = {row["id"]: row for row in cls.forward}

    def assertPreserved(self, ids: list[str]):
        failures = {
            case_id: self.results.get(case_id, {"status": "MISSING"})
            for case_id in ids
            if self.results.get(case_id, {}).get("status") != "Preserved"
        }
        self.assertEqual(failures, {})

    def test_01_frozen_inputs_have_no_drift(self):
        self.assertEqual(self.input_drift, [])
        self.assertEqual(self.seal["seal"]["status"], "SEALED_PRE_CANDIDATE_CONTRACT")
        self.assertEqual(self.seal["seal"]["mutation_score"], 1.0)

    def test_02_default_features_and_active_trio_are_independent(self):
        self.assertPreserved(
            [
                "feature-default-html",
                "feature-html-enabled",
                "feature-default-a11y",
                "feature-a11y-trio",
                "feature-html-does-not-enable-a11y",
                "feature-a11y-does-not-enable-html",
                "feature-both-comma",
                "feature-both-forward",
                "feature-both-reverse",
                "feature-a11y-repeated",
                "feature-unknown",
                "feature-bundle-no-a11y",
            ]
        )

    def test_03_casts_defaults_and_public_diagnostics_match(self):
        self.assertPreserved(
            [
                "summary-missing-table",
                "summary-content-not-table",
                "summary-integer",
                "summary-explicit-none",
                "summary-positional",
                "header-missing-cell",
                "header-integer-cell",
                "header-level-zero",
                "header-level-negative",
                "header-level-float",
                "header-invalid-scope",
                "header-none-scope",
                "data-missing-cell",
                "data-integer-cell",
                "data-named-cell",
            ]
        )

    def test_04_summary_scope_level_and_data_survive_to_pdf(self):
        self.assertPreserved(
            [
                "summary-string",
                "summary-omitted",
                "data-cell-casts",
                "scope-gate",
                "levels",
                "spans",
                "simple-structure",
                "automatic-header",
                "explicit-semantics",
            ]
        )

    def test_05_tags_disabled_remove_structure_without_visual_change(self):
        self.assertPreserved(
            [
                "tags-disabled-summary-string.typ",
                "tags-disabled-explicit-semantics.typ",
                "tags-disabled-multipage.typ",
                "visual-pair-summary-string.typ",
                "visual-pair-explicit-semantics.typ",
                "visual-pair-multipage.typ",
            ]
        )

    def test_06_multipage_ids_parent_tree_and_logical_header_survive(self):
        self.assertPreserved(["multipage"])

    def test_07_pdf_metadata_does_not_leak_into_other_targets(self):
        self.assertPreserved(
            ["non-pdf-pair-html", "non-pdf-pair-svg", "non-pdf-pair-png"]
        )

    def test_08_forward_reverse_candidate_results_are_identical(self):
        self.assertEqual(
            oracles.canonical_results(self.forward),
            oracles.canonical_results(self.reverse),
        )

    def test_09_existing_html_profile_surface_remains_available(self):
        self.assertPreserved(["feature-html-enabled"])

    def test_10_registration_only_stub_is_rejected(self):
        decisive = [
            row for row in self.forward
            if row["category"] != "opaque"
        ]
        self.assertEqual(len(decisive), 46)
        self.assertEqual(
            [row for row in decisive if row["status"] != "Preserved"],
            [],
            "registering the trio is insufficient: every semantic/PDF witness must pass",
        )


if __name__ == "__main__":
    unittest.main()
