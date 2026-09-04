import importlib.util
import pathlib
import unittest


MODULE_PATH = pathlib.Path(__file__).with_name("p1299-run-matrix.py")
SPEC = importlib.util.spec_from_file_location("p1299_run_matrix", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
RUNNER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RUNNER)


def receipt(exit_code: int, stdout: str = "", stderr: str = "") -> dict:
    return {
        "exit_code": exit_code,
        "stdout": stdout,
        "stderr": stderr,
        "complete": True,
    }


class CommandContractTests(unittest.TestCase):
    def test_profiles_have_exact_feature_argv(self):
        expected = {
            "default": [],
            "html": ["--features", "html"],
            "a11y": ["--features", "a11y-extras"],
            "html+a11y": ["--features", "html,a11y-extras"],
        }
        for profile, suffix in expected.items():
            command = RUNNER.command(pathlib.Path("typst"), "1", profile)
            self.assertEqual(command[5:], suffix)

    def test_command_keeps_literal_expression_and_json_format(self):
        command = RUNNER.command(pathlib.Path("typst"), "repr(type(html))", "default")
        self.assertEqual(
            command,
            ["typst", "eval", "repr(type(html))", "--format", "json"],
        )


class ClosedClassifierTests(unittest.TestCase):
    def test_all_closed_classes(self):
        cases = [
            (receipt(0, "1\n"), receipt(0, "1\n"), "MATCH_VALUE"),
            (receipt(1, stderr="same"), receipt(1, stderr="same"), "MATCH_DIAGNOSTIC"),
            (receipt(1, stderr="no"), receipt(0, "1\n"), "CRYSTALLINE_ONLY"),
            (receipt(0, "1\n"), receipt(1, stderr="no"), "VANILLA_ONLY"),
            (receipt(0, "1\n"), receipt(0, "2\n"), "DIFFERENT_VALUE"),
            (receipt(1, stderr="left"), receipt(1, stderr="right"), "DIFFERENT_DIAGNOSTIC"),
        ]
        for vanilla, crystalline, expected in cases:
            with self.subTest(expected=expected):
                self.assertEqual(RUNNER.classify(vanilla, crystalline), expected)

    def test_incomplete_signal_or_io_is_unknown(self):
        ok = receipt(0, "1\n")
        for failure in ("timeout", "signal", "io_error"):
            incomplete = {
                "exit_code": None,
                "stdout": "",
                "stderr": "",
                "complete": False,
                "failure": failure,
            }
            self.assertEqual(RUNNER.classify(incomplete, ok), "EXECUTION_UNKNOWN")
            self.assertEqual(RUNNER.classify(ok, incomplete), "EXECUTION_UNKNOWN")

    def test_kills_swapped_crystalline_only_classifier(self):
        vanilla = receipt(1, stderr="missing")
        crystalline = receipt(0, "function\n")
        mutant = RUNNER.classify(crystalline, vanilla)
        self.assertEqual(RUNNER.classify(vanilla, crystalline), "CRYSTALLINE_ONLY")
        self.assertNotEqual(mutant, "CRYSTALLINE_ONLY")

    def test_kills_swapped_match_diagnostic_classifier(self):
        vanilla = receipt(1, stderr="same")
        crystalline = receipt(1, stderr="same")
        mutant = "DIFFERENT_DIAGNOSTIC"
        self.assertEqual(RUNNER.classify(vanilla, crystalline), "MATCH_DIAGNOSTIC")
        self.assertNotEqual(mutant, RUNNER.classify(vanilla, crystalline))

    def test_kills_swapped_different_diagnostic_classifier(self):
        vanilla = receipt(1, stderr="span 1")
        crystalline = receipt(1, stderr="span 2")
        mutant = "MATCH_DIAGNOSTIC"
        self.assertEqual(RUNNER.classify(vanilla, crystalline), "DIFFERENT_DIAGNOSTIC")
        self.assertNotEqual(mutant, RUNNER.classify(vanilla, crystalline))


if __name__ == "__main__":
    unittest.main()
