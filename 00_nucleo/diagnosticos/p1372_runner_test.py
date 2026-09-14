import importlib.util
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("p1372_runner.py")
SPEC = importlib.util.spec_from_file_location("p1372_runner", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
runner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runner)


XML = b'''<doc><page width="200" height="240"><word xMin="10" yMin="12" xMax="30" yMax="20">Probe</word><word xMin="10" yMin="220.1254" xMax="35" yMax="228">After</word></page></doc>'''


class P1372RunnerTest(unittest.TestCase):
    def test_bbox_parser_relation_and_bilateral_classifier_fail_closed(self):
        observed = runner.parse_bbox(XML)
        self.assertEqual(observed["complete"], True)
        self.assertEqual(observed["text"], "ProbeAfter")
        self.assertEqual(observed["after"], {"page": 0, "y_min": "220.125"})

        relation = runner.vertical_relation(observed, observed)
        self.assertEqual(relation["complete"], True)
        self.assertEqual(relation["value"], {"page_delta": 0, "y_delta": "0.000"})
        self.assertEqual(runner.classify_bilateral(relation, relation), "MATCH")

        signature = runner.vertical_signature(observed, observed)
        self.assertEqual(signature["complete"], True)
        self.assertEqual(signature["value"]["lone"], {"page": 0, "y_min": "220.125"})
        shifted_observation = {
            "complete": True,
            "text": "ProbeAfter",
            "after": {"page": 1, "y_min": "220.125"},
        }
        shifted_signature = runner.vertical_signature(shifted_observation, shifted_observation)
        self.assertEqual(runner.classify_bilateral(signature, shifted_signature), "DIFFERENCE")

        shifted = {"complete": True, "value": {"page_delta": 1, "y_delta": "0.000"}}
        self.assertEqual(runner.classify_bilateral(relation, shifted), "DIFFERENCE")
        self.assertEqual(runner.parse_bbox(b"<doc><page><word>Probe</word></page></doc>")["complete"], False)
        duplicated = XML.replace(b"</page>", b'<word xMin="1" yMin="2">After</word></page>')
        self.assertEqual(runner.parse_bbox(duplicated)["complete"], False)
        self.assertEqual(
            runner.classify_bilateral(relation, {"complete": False, "value": None}),
            "Unknown",
        )


if __name__ == "__main__":
    unittest.main()
