import importlib.util
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("p1373_runner.py")
SPEC = importlib.util.spec_from_file_location("p1373_runner", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
runner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runner)


class P1373RunnerTest(unittest.TestCase):
    def test_preserves_the_frozen_probe_and_changes_only_cristalino_identity(self):
        self.assertEqual(runner.P1372.EXPECTED_TEXT, "ProbeAfter")
        self.assertEqual(runner.P1372.PROFILES, ("a11y-extras", "html+a11y"))
        self.assertEqual(
            runner.P1372.BOUNDED.sha256_path(Path(__file__).with_name("p1372_runner.py")),
            runner.P1372_RUNNER_SHA256,
        )
        self.assertEqual(len(runner.CRISTALINO_SHA256), 64)


if __name__ == "__main__":
    unittest.main()
