import importlib.util
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("p1371_runner.py")
SPEC = importlib.util.spec_from_file_location("p1371_runner", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
runner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runner)


class P1371RunnerTest(unittest.TestCase):
    def test_exact_plan_and_fail_closed_receipt_classifier(self):
        self.assertEqual(
            runner.command(Path("/frozen/test")),
            ["/frozen/test", runner.EXACT_TEST, "--exact"],
        )
        self.assertEqual(runner.ADDRESS_SPACE_BYTES, 3 * 1024**3)
        self.assertEqual(runner.MAX_PROCESSES, 1)

        success = {"exit_code": 0, "reason_code": None, "passed": 1, "failed": 0}
        self.assertEqual(runner.classify_receipt(success), "Preserved")
        self.assertEqual(runner.classify_receipt({**success, "exit_code": 101, "failed": 1}), "Violated")
        self.assertEqual(runner.classify_receipt({**success, "passed": 0}), "Unknown")
        self.assertEqual(runner.classify_receipt({**success, "passed": 2}), "Unknown")
        self.assertEqual(runner.classify_receipt({**success, "reason_code": "TIMEOUT"}), "Unknown")


if __name__ == "__main__":
    unittest.main()
