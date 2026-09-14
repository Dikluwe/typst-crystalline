import importlib.util
import itertools
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("p1370_reclassifier.py")
SPEC = importlib.util.spec_from_file_location("p1370_reclassifier", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
reclassifier = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(reclassifier)


def observation(value):
    return {"complete": True, "value": value, "reason_code": None}


def payload():
    rows = []
    for product in reclassifier.PRODUCTS:
        for profile in reclassifier.PROFILES:
            for case in reclassifier.CASES:
                rows.append(
                    {
                        "job": {"product": product, "profile": profile, "case": case},
                        "observation": observation(True),
                    }
                )
    return {
        "schema": "p1369-s02-bilateral/v1",
        "identities": dict(reclassifier.EXPECTED_IDENTITIES),
        "job_count": 24,
        "rows": rows,
        "profiles": [{"bilateral": "DIFFERENCE"}],
    }


class P1370ReclassifierTest(unittest.TestCase):
    def test_retified_vector_mutants_and_structure_are_fail_closed(self):
        complete = [observation(True), observation(True), observation(True)]
        self.assertEqual(reclassifier.classify_vector(complete)["classification"], "Preserved")

        for bits in itertools.product((False, True), repeat=3):
            if bits == (True, True, True):
                continue
            actual = reclassifier.classify_vector([observation(bit) for bit in bits])
            self.assertEqual(actual["classification"], "Violated")

        opaque = [observation(True), {"complete": False, "value": None}, observation(True)]
        self.assertEqual(reclassifier.classify_vector(opaque)["classification"], "Unknown")

        result = reclassifier.reclassify(payload())
        self.assertEqual(result["verdict"], "MATCH")
        self.assertEqual(result["counts"], {"MATCH": 4})

        incomplete = payload()
        incomplete["rows"].pop()
        self.assertEqual(reclassifier.reclassify(incomplete)["verdict"], "Unknown")


if __name__ == "__main__":
    unittest.main()
