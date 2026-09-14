import importlib.util
import itertools
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("p1368_s02_observer.py")
SPEC = importlib.util.spec_from_file_location("p1368_s02_observer", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
observer = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(observer)


class S02ObserverTest(unittest.TestCase):
    def test_p1368_parser_and_classifier_are_fail_closed(self):
        yes = observer.parse_tagged_field(b"Pages: 1\nTagged: YES\n")
        no = observer.parse_tagged_field(b" Tagged: no  \n")
        self.assertEqual(yes, {"complete": True, "value": True, "reason_code": None})
        self.assertEqual(no, {"complete": True, "value": False, "reason_code": None})

        expected = observer.classify_s02(yes, no, yes)
        self.assertEqual(expected["classification"], "Preserved")
        self.assertEqual(expected["vector"], [True, False, True])

        by_value = {True: yes, False: no}
        for bits in itertools.product((False, True), repeat=3):
            if bits == (True, False, True):
                continue
            vector = tuple(by_value[bit] for bit in bits)
            self.assertEqual(observer.classify_s02(*vector)["classification"], "Violated")

        opaque_payloads = (
            b"Pages: 1\n",
            b"Tagged: yes\nTagged: no\n",
            b"Tagged: perhaps\n",
            b"Tagged: yes\xff\n",
        )
        for payload in opaque_payloads:
            unknown = observer.parse_tagged_field(payload)
            self.assertEqual(unknown["complete"], False)
            self.assertEqual(observer.classify_s02(yes, no, unknown)["classification"], "Unknown")


if __name__ == "__main__":
    unittest.main()
