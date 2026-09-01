import importlib.util
from pathlib import Path
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("p1287_global", HERE / "p1287_global.py")
P = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(P)


class P1287OracleTests(unittest.TestCase):
    def test_exact_case_inventory(self):
        self.assertEqual([c["id"] for c in P.CASES],
                         [f"C{i:02}" for i in range(1, 13)] + [f"E{i:02}" for i in range(1, 12)])
        self.assertEqual(sorted(P.fixture_hashes()), [f"C{i:02}.typ" for i in range(1, 13)])

    def test_classification_lattice(self):
        valid = {"state": "valid", "semantic": {"x": 1}}
        self.assertEqual(P.classify(valid, valid)[0], "Preserved")
        self.assertEqual(P.classify(valid, {"state": "valid", "semantic": {"x": 2}})[0], "Violated")
        self.assertEqual(P.classify(valid, {"state": "rejected", "semantic": {}})[0], "Violated")
        self.assertEqual(P.classify({"state": "unknown"}, valid)[0], "Unknown")
        self.assertEqual(P.classify(valid, {"state": "unknown"})[0], "Unknown")
        self.assertEqual(P.classify({"state": "not_applicable"}, valid)[0], "NotApplicable")

    def test_png_is_exact_rgba_without_tolerance(self):
        from PIL import Image
        with tempfile.TemporaryDirectory() as tmp:
            a, b = Path(tmp) / "a.png", Path(tmp) / "b.png"
            Image.new("RGBA", (1, 1), (1, 2, 3, 4)).save(a)
            Image.new("RGBA", (1, 1), (1, 2, 3, 5)).save(b)
            oa, ob = P.png_observe(a), P.png_observe(b)
            self.assertEqual(oa["state"], "valid")
            self.assertNotEqual(oa["semantic"]["rgba_sha256"], ob["semantic"]["rgba_sha256"])
            self.assertEqual(P.classify(oa, ob)[0], "Violated")

    def test_repetition_never_hides_instability(self):
        a = {"state": "valid", "semantic": 1, "artifact_sha256": "a"}
        b = {"state": "valid", "semantic": 1, "artifact_sha256": "b"}
        self.assertEqual(P.stable_runs([a, b])["state"], "unknown")
        self.assertEqual(P.stable_runs([a, a])["state"], "valid")

    def test_opaque_exporters_remain_unknown(self):
        opaque = {c["id"] for c in P.CASES if c.get("opaque")}
        self.assertEqual(opaque, {"E04", "E05", "E06", "E07", "E08", "E09", "E10"})

    def test_protected_hashes(self):
        self.assertEqual(P.sha_file(P.MANIFEST), P.MANIFEST_SHA)
        self.assertEqual(len(P.fixture_manifest_hash()), 64)
        self.assertEqual(len(P.suite_hash()), 64)


if __name__ == "__main__":
    unittest.main()
