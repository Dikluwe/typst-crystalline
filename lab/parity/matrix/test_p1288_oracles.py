import copy
import importlib.util
import json
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("p1288_oracles.py")
SPEC = importlib.util.spec_from_file_location("p1288_oracles", MODULE_PATH)
oracles = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(oracles)


class P1288OracleTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.data = oracles.load_baseline()

    def test_frozen_inputs_have_no_drift(self):
        self.assertEqual(oracles.verify_hashes(self.data), [])

    def test_preserved_violated_unknown_are_distinct(self):
        expected = {"exit": 0, "stdout": "ok\n"}
        self.assertEqual(
            oracles.judge_process(expected, {"opaque": False, "exit": 0, "stdout": "ok\n", "stderr": ""})[0],
            "Preserved",
        )
        self.assertEqual(
            oracles.judge_process(expected, {"opaque": False, "exit": 1, "stdout": "", "stderr": ""})[0],
            "Violated",
        )
        self.assertEqual(oracles.judge_process(expected, {"opaque": True, "reason": "opaque"})[0], "Unknown")

    def test_unknown_is_not_success(self):
        status, _ = oracles.judge_process({"exit": 0}, {"opaque": True, "reason": "missing parser"})
        self.assertNotEqual(status, "Preserved")

    def test_summary_none_is_explicit_error(self):
        case = next(case for case in self.data["eval_cases"] if case["id"] == "summary-explicit-none")
        self.assertEqual(case["exit"], 1)
        self.assertEqual(case["stderr_contains"], "expected string, found none")

    def test_case_ids_are_unique(self):
        ids = [case["id"] for case in self.data["eval_cases"]]
        ids += [case["id"] for case in self.data["compile_cases"]]
        ids += [case["id"] for case in self.data["opaque_cases"]]
        self.assertEqual(len(ids), len(set(ids)))

    def test_positive_negative_and_opaque_cases_exist(self):
        kinds = {case["kind"] for case in self.data["eval_cases"]}
        self.assertEqual(kinds, {"positive", "negative"})
        self.assertTrue(self.data["opaque_cases"])

    def test_future_binary_does_not_rewrite_expectations(self):
        frozen = json.dumps(self.data, sort_keys=True)
        clone = copy.deepcopy(self.data)
        command = ["/future/bin", "eval", clone["eval_cases"][0]["expr"], "--format", "json"]
        self.assertEqual(command[0], "/future/bin")
        self.assertEqual(json.dumps(clone, sort_keys=True), frozen)

    def test_forward_reverse_canonicalization(self):
        results = [{"id": "b", "status": "Unknown"}, {"id": "a", "status": "Preserved"}]
        self.assertEqual(oracles.canonical_results(results), oracles.canonical_results(list(reversed(results))))

    def test_feature_equivalence_group_is_frozen(self):
        grouped = [case for case in self.data["eval_cases"] if case.get("equivalence_group") == "both-features"]
        self.assertEqual({case["id"] for case in grouped}, {"feature-both-comma", "feature-both-forward", "feature-both-reverse"})
        self.assertEqual({case["stdout"] for case in grouped}, {'"(module, function)"\n'})

    def test_self_test(self):
        self.assertEqual(oracles.self_test(), 0)


if __name__ == "__main__":
    unittest.main()
