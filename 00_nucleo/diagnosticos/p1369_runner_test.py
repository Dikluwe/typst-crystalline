import importlib.util
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("p1369_runner.py")
SPEC = importlib.util.spec_from_file_location("p1369_runner", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
runner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runner)


class P1369RunnerTest(unittest.TestCase):
    def test_exact_matrix_limits_and_fail_closed_bilateral_classification(self):
        jobs = runner.planned_jobs()
        identities = [(j["product"], j["profile"], j["case"]) for j in jobs]
        self.assertEqual(len(identities), 24)
        self.assertEqual(len(set(identities)), 24)
        self.assertEqual(set(p for p, _, _ in identities), {"vanila", "cristalino"})
        self.assertEqual(
            set(profile for _, profile, _ in identities),
            {"default", "html", "a11y-extras", "html+a11y"},
        )
        self.assertEqual(
            set(case for _, _, case in identities),
            {"top_level", "document_over_cli", "cli_only"},
        )
        self.assertEqual(runner.TIMEOUT_SECONDS, 45)
        self.assertEqual(runner.ADDRESS_SPACE_BYTES, 2 * 1024**3)
        self.assertEqual(runner.FILE_SIZE_BYTES, 16 * 1024**2)
        self.assertEqual(runner.CHANNEL_READ_BYTES, 1024**2)
        self.assertEqual(runner.MAX_PARALLEL_PROCESSES, 1)

        preserved = {"classification": "Preserved", "vector": [True, False, True]}
        violated = {"classification": "Violated", "vector": [False, False, True]}
        unknown = {"classification": "Unknown", "vector": None}
        self.assertEqual(runner.classify_bilateral(preserved, preserved), "MATCH")
        self.assertEqual(runner.classify_bilateral(preserved, violated), "DIFFERENCE")
        self.assertEqual(runner.classify_bilateral(preserved, unknown), "Unknown")


if __name__ == "__main__":
    unittest.main()
