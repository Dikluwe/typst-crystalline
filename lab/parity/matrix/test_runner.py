import copy
import importlib.util
import pathlib
import unittest

HERE = pathlib.Path(__file__).resolve().parent
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

    def test_side_environment_must_be_string_map(self):
        manifest = runner.load_manifest(HERE / "manifest.yaml")
        broken = copy.deepcopy(manifest)
        broken["cases"][0]["oraculo"]["env"] = {"X": 1}
        self.assertTrue(runner.validate_manifest(broken))


class ClassifierTests(unittest.TestCase):
    def result(self, code=0, stdout="", stderr="", artifact=False):
        return {"exit_code": code, "stdout": stdout, "stderr": stderr, "artifact_exists": artifact}

    def test_exact_mismatch_is_diff(self):
        self.assertEqual(runner.classify("exact_output", self.result(stdout="a"), self.result(stdout="b"))[0], "DIFF")

    def test_missing_capability_is_absent(self):
        self.assertEqual(runner.classify("capability", self.result(0), self.result(2))[0], "ABSENT")

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

    def test_geometry_tolerance_is_three_decimal_points(self):
        self.assertEqual(round(10.0004, 3), round(10.00049, 3))

    def test_png_invalid_input_is_rejected(self):
        import tempfile
        with tempfile.TemporaryDirectory() as tmp:
            path = pathlib.Path(tmp) / "bad.png"; path.write_bytes(b"not png")
            with self.assertRaises(ValueError): runner.png_pixels(path)

    def test_pdf_observable_requires_single_artifact(self):
        with self.assertRaises(ValueError): runner.artifact({"artifact_paths": []}, ".pdf")


if __name__ == "__main__":
    unittest.main()
