"""Regression checks for fail-closed core benchmark artifact selection."""

import importlib.util
import json
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location(
    "core_runner", Path(__file__).resolve().parents[1] / "run_core_benchmark.py"
)
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def artifact(opt_level="3", debug_assertions=False):
    return {
        "reason": "compiler-artifact",
        "target": {"name": "typst_core"},
        "profile": {
            "test": True,
            "opt_level": opt_level,
            "debug_assertions": debug_assertions,
        },
        "executable": "/tmp/typst-core-test",
    }


class BenchmarkGuards(unittest.TestCase):
    def test_rejects_debug_and_unoptimized_artifacts(self):
        for opt_level, assertions in [("0", True), ("0", False), ("3", True)]:
            with self.subTest(opt_level=opt_level, assertions=assertions):
                with self.assertRaises(ValueError):
                    runner.executable_from_messages(json.dumps(artifact(opt_level, assertions)))

    def test_accepts_optimized_test_among_unrelated_artifacts(self):
        unrelated = artifact("0", True)
        unrelated["target"]["name"] = "other"
        messages = "\n".join(map(json.dumps, [unrelated, artifact()]))
        self.assertEqual(
            runner.executable_from_messages(messages), Path("/tmp/typst-core-test")
        )

    def test_rejects_missing_malformed_and_ambiguous_artifacts(self):
        for messages in ["", "not JSON", "\n".join([json.dumps(artifact())] * 2)]:
            with self.subTest(messages=messages):
                with self.assertRaises(ValueError):
                    runner.executable_from_messages(messages)

    def test_rejects_incomplete_profile_and_executable(self):
        for field in ["opt_level", "debug_assertions"]:
            incomplete = artifact()
            del incomplete["profile"][field]
            with self.assertRaises(ValueError):
                runner.executable_from_messages(json.dumps(incomplete))
        for executable in [None, "relative/path"]:
            incomplete = artifact()
            incomplete["executable"] = executable
            with self.assertRaises(ValueError):
                runner.executable_from_messages(json.dumps(incomplete))

    def test_debug_artifact_is_rejected_before_listing_or_sampling(self):
        import subprocess
        from unittest.mock import patch
        build = subprocess.CompletedProcess([], 0, stdout=json.dumps(artifact("0", True)))
        with patch.object(runner.sys, "argv", ["runner", sorted(runner.TESTS)[0]]):
            with patch.object(runner.subprocess, "run", return_value=build) as run:
                with self.assertRaisesRegex(ValueError, "refusing core benchmark"):
                    runner.main()
                self.assertEqual(run.call_count, 1)

    def test_debug_rust_launchers_stop_before_starting_python(self):
        import os
        import subprocess
        import tempfile
        benches = Path(__file__).resolve().parents[1]
        for name in ["observation_bench", "stdlib_bench"]:
            for opt_level, assertions in [("0", "no"), ("3", "yes")]:
                with self.subTest(name=name, opt_level=opt_level, assertions=assertions):
                    with tempfile.TemporaryDirectory(prefix="bench-guard-") as directory:
                        executable = Path(directory) / name
                        env = dict(os.environ, CARGO_MANIFEST_DIR=str(benches),
                                   TYPST_BENCH_OPT_LEVEL=opt_level)
                        subprocess.run(
                            ["rustc", "--edition=2021", str(benches / (name + ".rs")),
                             "-o", str(executable), "-C", "opt-level=" + opt_level,
                             "-C", "debug-assertions=" + assertions],
                            env=env, capture_output=True, text=True, check=True,
                        )
                        result = subprocess.run(
                            [str(executable), "--test"], capture_output=True, text=True,
                        )
                        self.assertNotEqual(result.returncode, 0)
                        self.assertIn("benchmarks require optimization", result.stderr)
                        self.assertNotIn("Verified optimized core", result.stderr)


    def test_smoke_checks_exact_ignored_test_without_sampling(self):
        import subprocess
        from unittest.mock import patch
        test = sorted(runner.TESTS)[0]
        build = subprocess.CompletedProcess([], 0, stdout=json.dumps(artifact()))
        listing = subprocess.CompletedProcess([], 0, stdout=f"{test}: test\n")
        with patch.object(runner.sys, "argv", ["runner", test, "--bench", "--test"]):
            with patch.object(runner.subprocess, "run", side_effect=[build, listing]) as run:
                runner.main()
                self.assertEqual(run.call_count, 2)
                self.assertEqual(run.call_args.args[0],
                                 ["/tmp/typst-core-test", test, "--exact", "--ignored", "--list"])

    def test_missing_test_never_starts_sampling(self):
        import subprocess
        from unittest.mock import patch
        build = subprocess.CompletedProcess([], 0, stdout=json.dumps(artifact()))
        listing = subprocess.CompletedProcess([], 0, stdout="0 tests, 0 benchmarks\n")
        with patch.object(runner.sys, "argv", ["runner", sorted(runner.TESTS)[0]]):
            with patch.object(runner.subprocess, "run", side_effect=[build, listing]) as run:
                with self.assertRaisesRegex(ValueError, "ignored benchmark test not found"):
                    runner.main()
                self.assertEqual(run.call_count, 2)


if __name__ == "__main__":
    unittest.main()
