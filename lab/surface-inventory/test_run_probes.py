import pathlib
import sys
import unittest

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

import run_probes


class ProbeContractTests(unittest.TestCase):
    def test_default_command_has_no_features(self):
        command = run_probes.command(pathlib.Path("typst"), "1", "default")
        self.assertNotIn("--features", command)

    def test_html_command_has_exact_feature(self):
        command = run_probes.command(pathlib.Path("typst"), "1", "html")
        self.assertEqual(command[-2:], ["--features", "html"])

    def test_both_fail_are_not_match(self):
        receipt = {"exit_code": 1, "stdout": ""}
        self.assertFalse(run_probes.receipts_match(receipt, receipt))

    def test_one_failure_is_not_match(self):
        ok = {"exit_code": 0, "stdout": "1\n"}
        failed = {"exit_code": 1, "stdout": ""}
        self.assertFalse(run_probes.receipts_match(ok, failed))

    def test_both_success_must_have_equal_observable(self):
        first = {"exit_code": 0, "stdout": "1\n"}
        second = {"exit_code": 0, "stdout": "2\n"}
        self.assertFalse(run_probes.receipts_match(first, second))
        self.assertTrue(run_probes.receipts_match(first, dict(first)))


if __name__ == "__main__":
    unittest.main()
