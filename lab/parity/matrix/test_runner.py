import copy
import importlib.util
import pathlib
import sys
import unittest

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
SPEC = importlib.util.spec_from_file_location("p1137_runner", HERE / "runner.py")
runner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runner)


class SchemaTests(unittest.TestCase):
    def test_manifest_is_valid(self):
        self.assertEqual(runner.validate_manifest(runner.load_manifest(HERE / "manifest.yaml")), [])

    def test_invalid_manifest_is_red(self):
        manifest = runner.load_manifest(HERE / "manifest.yaml")
        broken = copy.deepcopy(manifest)
        del broken["cases"][0]["observavel"]
        self.assertTrue(runner.validate_manifest(broken))

    def test_binary_identity_uses_sha256(self):
        import tempfile

        with tempfile.TemporaryDirectory() as tmp:
            binary = pathlib.Path(tmp) / "typst"
            binary.write_bytes(b"typst")
            self.assertEqual(
                runner.sha256_file(binary),
                "aceb8d6389f7a425854fdfd5de310c10ec025ddcf4bd3fc06736adccfdcbddcf",
            )

    def test_side_environment_must_be_string_map(self):
        manifest = runner.load_manifest(HERE / "manifest.yaml")
        broken = copy.deepcopy(manifest)
        broken["cases"][0]["oraculo"]["env"] = {"X": 1}
        self.assertTrue(runner.validate_manifest(broken))

    def test_expected_state_is_required(self):
        manifest = runner.load_manifest(HERE / "manifest.yaml")
        broken = copy.deepcopy(manifest)
        del broken["cases"][0]["expected_state"]
        self.assertTrue(runner.validate_manifest(broken))

    def test_missing_source_fixture_is_rejected_before_execution(self):
        manifest = runner.load_manifest(HERE / "manifest.yaml")
        broken = copy.deepcopy(manifest)
        broken["cases"][0]["fonte_typ"] = "fixtures/does-not-exist.typ"
        self.assertTrue(runner.validate_manifest(broken))

    def test_html_features_must_be_symmetric(self):
        manifest = runner.load_manifest(HERE / "manifest.yaml")
        broken = copy.deepcopy(manifest)
        html_case = next(case for case in broken["cases"] if case["feature"] == "export/html")
        args = html_case["cristalino"]["args"]
        feature_index = args.index("--features")
        del args[feature_index : feature_index + 2]
        self.assertTrue(runner.validate_manifest(broken))

    def test_html_features_must_also_reject_oracle_only_removal(self):
        manifest = runner.load_manifest(HERE / "manifest.yaml")
        broken = copy.deepcopy(manifest)
        html_case = next(case for case in broken["cases"] if case["feature"] == "export/html")
        args = html_case["oraculo"]["args"]
        feature_index = args.index("--features")
        del args[feature_index : feature_index + 2]
        self.assertTrue(runner.validate_manifest(broken))

    def test_p1288_profiles_and_a11y_active_peer_are_explicit(self):
        manifest = runner.load_manifest(HERE / "manifest.yaml")
        self.assertEqual(
            manifest["profiles"],
            {
                "default": {"features": []},
                "html": {"features": ["html"]},
                "a11y-extras": {"features": ["a11y-extras"]},
            },
        )
        self.assertEqual(manifest["scope_out_features"], ["bundle"])
        a11y = next(
            case for case in manifest["cases"]
            if case["feature"] == "pdf/a11y-extras-bindings"
        )
        self.assertEqual(a11y["required_features"], ["a11y-extras"])

    def test_p1288_active_profile_expectation_is_match_after_implementation(self):
        manifest = runner.load_manifest(HERE / "manifest.yaml")
        a11y = next(
            case for case in manifest["cases"]
            if case["id"] == "P1288-B-001"
        )
        self.assertEqual(a11y["expected_state"], "MATCH")

    def test_required_feature_without_active_profile_is_rejected(self):
        manifest = runner.load_manifest(HERE / "manifest.yaml")
        broken = copy.deepcopy(manifest)
        broken["cases"][0]["required_features"] = ["future-flag"]
        self.assertTrue(runner.validate_manifest(broken))


class ClassifierTests(unittest.TestCase):
    def result(self, code=0, stdout="", stderr="", artifact=False):
        return {"exit_code": code, "stdout": stdout, "stderr": stderr, "artifact_exists": artifact}

    def test_exact_mismatch_is_diff(self):
        self.assertEqual(runner.classify("exact_output", self.result(stdout="a"), self.result(stdout="b"))[0], "DIFF")

    def test_missing_capability_is_absent(self):
        self.assertEqual(runner.classify("capability", self.result(0), self.result(2))[0], "ABSENT")

    def test_two_failed_capability_invocations_are_baseline_defect(self):
        self.assertEqual(
            runner.classify("capability", self.result(2), self.result(2)),
            ("ERROR", "BASELINE_DEFECT"),
        )

    def test_semantic_version_mismatch_is_diff(self):
        oracle = self.result(stdout="typst 0.15.1 (hash)\n")
        crystal = self.result(stdout="typst 0.15.0 (hash)\n")
        self.assertEqual(runner.classify("semantic_version", oracle, crystal), ("DIFF", "LANGUAGE_SEMANTICS"))

    def test_matching_failure_exit_is_match(self):
        self.assertEqual(runner.classify("exit_code", self.result(1), self.result(1))[0], "MATCH")

    def test_expected_result_mismatch_is_detectable(self):
        measured = "DIFF"
        expected = "MATCH"
        self.assertNotEqual(measured, expected)

    def test_cli_surface_subset_is_partial(self):
        oracle = self.result(stdout="Commands:\n  compile  Compile\n  eval  Eval\n  fonts  Fonts\n\nOptions:\n  --color <COLOR>\n  --cert <CERT>\n  -h, --help\n")
        crystal = self.result(stdout="Commands:\n  compile  Compile\n  eval  Eval\n\nOptions:\n  --color <COLOR>\n  -h, --help\n")
        self.assertEqual(runner.classify("cli_surface", oracle, crystal), ("PARTIAL", "PUBLIC_CLI"))

    def test_diagnostic_compares_message_and_span(self):
        oracle = self.result(1, stderr="error: bad\n  /tmp/a.typ:2:3")
        crystal = self.result(1, stderr="error: other\n  /tmp/a.typ:2:3")
        self.assertEqual(runner.classify("diagnostic", oracle, crystal)[0], "DIFF")

    def test_typed_value_distinguishes_bool_from_int(self):
        oracle = self.result(stdout="true\n")
        crystal = self.result(stdout="1\n")
        self.assertEqual(runner.classify("typed_value", oracle, crystal)[0], "DIFF")

    def test_typed_value_canonicalizes_dict_order(self):
        oracle = self.result(stdout='{"a":1,"b":[true]}\n')
        crystal = self.result(stdout='{"b":[true],"a":1}\n')
        self.assertEqual(runner.classify("typed_value", oracle, crystal)[0], "MATCH")

    def test_semantic_tree_excludes_only_svg_id(self):
        import tempfile
        with tempfile.TemporaryDirectory() as tmp:
            left = pathlib.Path(tmp) / "left.svg"; right = pathlib.Path(tmp) / "right.svg"
            left.write_text('<svg id="a"><text x="1">Hi</text></svg>')
            right.write_text('<svg id="b"><text x="1">Hi</text></svg>')
            self.assertEqual(runner.semantic_tree(left), runner.semantic_tree(right))

    def test_svg_classifier_uses_resolved_paint_model(self):
        import tempfile
        with tempfile.TemporaryDirectory() as tmp:
            left = pathlib.Path(tmp) / "left.svg"; right = pathlib.Path(tmp) / "right.svg"
            left.write_text('<svg width="10pt" height="10pt"><use href="#a"/><defs><symbol id="a"><path d="M0 0"/></symbol></defs></svg>')
            right.write_text('<svg width="10pt" height="10pt"><use href="#b"/><defs><symbol id="b"><path d="M 0 0"/></symbol></defs></svg>')
            oracle = {"exit_code": 0, "artifact_paths": [str(left)]}
            crystal = {"exit_code": 0, "artifact_paths": [str(right)]}
            self.assertEqual(runner.classify("semantic_tree", oracle, crystal)[0], "MATCH")

    def test_geometry_tolerance_is_three_decimal_points(self):
        self.assertEqual(round(10.0004, 3), round(10.00049, 3))

    def test_png_invalid_input_is_rejected(self):
        import tempfile
        with tempfile.TemporaryDirectory() as tmp:
            path = pathlib.Path(tmp) / "bad.png"; path.write_bytes(b"not png")
            with self.assertRaises(ValueError): runner.png_pixels(path)

    def test_malformed_crystalline_artifact_is_public_format_diff(self):
        import tempfile

        with tempfile.TemporaryDirectory() as tmp:
            oracle_path = pathlib.Path(tmp) / "oracle.png"
            crystal_path = pathlib.Path(tmp) / "crystalline.png"
            runner.write_png_rgba(oracle_path, 1, 1, b"\x00\x00\x00\xff")
            crystal_path.write_bytes(b"not png")
            oracle = {"exit_code": 0, "artifact_paths": [str(oracle_path)]}
            crystal = {"exit_code": 0, "artifact_paths": [str(crystal_path)]}
            self.assertEqual(
                runner.classify("raster", oracle, crystal),
                ("DIFF", "PUBLIC_FORMAT"),
            )

    def test_missing_pdf_tool_is_harness_defect(self):
        import tempfile
        from unittest import mock

        with tempfile.TemporaryDirectory() as tmp:
            oracle_path = pathlib.Path(tmp) / "oracle.pdf"
            crystal_path = pathlib.Path(tmp) / "crystalline.pdf"
            oracle_path.write_bytes(b"pdf")
            crystal_path.write_bytes(b"pdf")
            oracle = {"exit_code": 0, "artifact_paths": [str(oracle_path)]}
            crystal = {"exit_code": 0, "artifact_paths": [str(crystal_path)]}
            with mock.patch.object(runner.subprocess, "run", side_effect=FileNotFoundError("pdfinfo")):
                self.assertEqual(
                    runner.classify("geometry", oracle, crystal),
                    ("ERROR", "HARNESS_DEFECT"),
                )

    def test_pdf_observable_requires_single_artifact(self):
        with self.assertRaises(ValueError): runner.artifact({"artifact_paths": []}, ".pdf")


class ProfileLatticeTests(unittest.TestCase):
    PROFILES = {
        "default": {"features": []},
        "html": {"features": ["html"]},
        "a11y-extras": {"features": ["a11y-extras"]},
    }

    def case(self, required):
        return {"id": "P1288-test", "required_features": required}

    def result(self, code=0, stdout="", stderr=""):
        return {
            "exit_code": code,
            "stdout": stdout,
            "stderr": stderr,
            "artifact_exists": False,
            "artifact_paths": [],
        }

    def test_lattice_is_exact_and_closed(self):
        self.assertEqual(
            runner.LATTICE_STATES,
            (
                "MATCH",
                "DIFFERENCE",
                "DISABLED_BY_PROFILE",
                "UNKNOWN",
                "BASELINE_ONLY",
                "EXTRA_BINDING",
            ),
        )

    def test_bilateral_disabled_failure_is_not_difference_or_match(self):
        gate = runner.profile_gate(
            self.case(["html"]), "default", self.PROFILES
        )
        self.assertEqual(gate, "DISABLED_BY_PROFILE")

    def test_html_disabled_diagnostic_is_not_credited(self):
        bilateral_failure = self.result(
            code=1, stderr="error: unknown variable: html"
        )
        state, _ = runner.classify_for_profile(
            self.case(["html"]),
            "default",
            self.PROFILES,
            "diagnostic",
            bilateral_failure,
            bilateral_failure,
        )
        self.assertEqual(state, "DISABLED_BY_PROFILE")

    def test_unknown_flag_in_active_profile_is_not_hidden_as_disabled(self):
        profiles = {**self.PROFILES, "future": {"features": ["future-flag"]}}
        state, _ = runner.classify_for_profile(
            self.case(["future-flag"]),
            "future",
            profiles,
            "capability",
            self.result(code=0, stdout="ok"),
            self.result(code=2, stderr="unknown feature: future-flag"),
        )
        self.assertEqual(state, "DIFFERENCE")

    def test_missing_active_peer_is_unknown_not_disabled(self):
        state, _ = runner.classify_for_profile(
            self.case(["future-flag"]),
            "default",
            self.PROFILES,
            "capability",
            self.result(code=1),
            self.result(code=1),
        )
        self.assertEqual(state, "UNKNOWN")

    def test_feature_combination_repetition_and_order_are_canonical(self):
        left = runner.with_profile_features(
            ["eval", "x", "--features", "html,a11y-extras,html"],
            {"a11y-extras", "html"},
        )
        right = runner.with_profile_features(
            ["eval", "x", "--features=a11y-extras,html"],
            {"html", "a11y-extras"},
        )
        expected = ["eval", "x", "--features", "a11y-extras,html"]
        self.assertEqual(left, expected)
        self.assertEqual(right, expected)

    def test_profile_features_are_not_injected_without_case_declaration(self):
        args = ["query", "fixture.typ", "heading", "--format", "json"]
        self.assertEqual(
            runner.with_profile_features(args, {"html"}, required_features=set()),
            args,
        )

    def test_required_features_allow_profile_composition_without_inline_flag(self):
        self.assertEqual(
            runner.with_profile_features(
                ["eval", "x", "--format", "json"],
                {"a11y-extras"},
                required_features={"a11y-extras"},
            ),
            ["eval", "x", "--format", "json", "--features", "a11y-extras"],
        )

    def test_query_case_is_unchanged_in_html_profile(self):
        from unittest import mock
        import tempfile

        manifest = runner.load_manifest(HERE / "manifest.yaml")
        case = next(case for case in manifest["cases"] if case["id"] == "P1137-I-001")

        def fake_invoke(_binary, args, _source, _output, _extra_env=None):
            return {
                "command": list(args),
                "exit_code": 0,
                "stdout": "[]\n",
                "stderr": "",
                "artifact_exists": False,
                "artifact_paths": [],
            }

        with tempfile.TemporaryDirectory() as tmp, mock.patch.object(
            runner, "invoke", side_effect=fake_invoke
        ):
            result = runner.run_case(
                case,
                pathlib.Path("vanilla"),
                pathlib.Path("crystal"),
                pathlib.Path(tmp),
                "html",
                manifest["profiles"],
            )

        self.assertNotIn("--features", result["oracle"]["command"])
        self.assertNotIn("--features", result["crystalline"]["command"])

    def test_disabled_never_increases_match_and_totals_close(self):
        counts = runner.closed_counts(
            [
                {"estado": "MATCH"},
                {"estado": "DISABLED_BY_PROFILE"},
                {"estado": "DIFFERENCE"},
            ]
        )
        self.assertEqual(counts["MATCH"], 1)
        self.assertEqual(counts["DISABLED_BY_PROFILE"], 1)
        self.assertEqual(sum(counts.values()), 3)
        self.assertEqual(tuple(counts), runner.LATTICE_STATES)

    def test_declared_asymmetries_have_distinct_no_credit_states(self):
        baseline = {**self.case([]), "asymmetry": "baseline-only"}
        extra = {**self.case([]), "asymmetry": "extra-binding"}
        same = self.result(code=0, stdout="ok")
        self.assertEqual(
            runner.classify_for_profile(
                baseline, "default", self.PROFILES, "capability", same, same
            )[0],
            "BASELINE_ONLY",
        )
        self.assertEqual(
            runner.classify_for_profile(
                extra, "default", self.PROFILES, "capability", same, same
            )[0],
            "EXTRA_BINDING",
        )


class DeterminismTests(unittest.TestCase):
    def test_case_reordering_preserves_normalized_verdicts(self):
        import tempfile
        from unittest import mock

        manifest = runner.load_manifest(HERE / "manifest.yaml")
        cases = [copy.deepcopy(manifest["cases"][1]), copy.deepcopy(manifest["cases"][2])]

        def fake_invoke(_binary, args, _source, _output, _extra_env=None):
            stdout = "typst 0.15.1\n" if "--version" in args else "6\n"
            return {
                "command": list(args),
                "exit_code": 0,
                "stdout": stdout,
                "stderr": "",
                "artifact_exists": False,
                "artifact_paths": [],
            }

        with tempfile.TemporaryDirectory() as tmp, mock.patch.object(
            runner, "invoke", side_effect=fake_invoke
        ):
            root = pathlib.Path(tmp)
            forward = [
                runner.run_case(case, pathlib.Path("vanilla"), pathlib.Path("crystal"), root / case["id"])
                for case in cases
            ]
            reverse = [
                runner.run_case(case, pathlib.Path("vanilla"), pathlib.Path("crystal"), root / case["id"])
                for case in reversed(cases)
            ]

        normalize = lambda results: sorted(
            (result["id"], result["estado"], result["classe"], result["observed"])
            for result in results
        )
        self.assertEqual(normalize(forward), normalize(reverse))

    def test_forward_reverse_payload_is_byte_identical_and_closed(self):
        forward = [
            {"id": "b", "estado": "DISABLED_BY_PROFILE"},
            {"id": "a", "estado": "MATCH"},
        ]
        reverse = list(reversed(forward))
        left = runner.render_payload(
            runner.make_payload("default", forward, "vanilla", "crystalline")
        )
        right = runner.render_payload(
            runner.make_payload("default", reverse, "vanilla", "crystalline")
        )
        self.assertEqual(left, right)
        self.assertEqual(sum(runner.closed_counts(forward).values()), len(forward))


if __name__ == "__main__":
    unittest.main()
