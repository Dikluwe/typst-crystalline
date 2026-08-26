import pathlib
import tempfile
import unittest

from svg_morphology import compare, morphology


class SvgMorphologyTests(unittest.TestCase):
    def pair(self, left, right):
        tmp = tempfile.TemporaryDirectory(); self.addCleanup(tmp.cleanup)
        a = pathlib.Path(tmp.name) / "a.svg"; b = pathlib.Path(tmp.name) / "b.svg"
        a.write_text(left); b.write_text(right)
        return a, b

    def test_alpha_renames_ids_and_resolves_references(self):
        a, b = self.pair(
            '<svg width="10pt" height="10pt"><g><use href="#a" x="1" y="2" fill="#000000"/></g><defs><symbol id="a"><path d="M0 0L1 1"/></symbol></defs></svg>',
            '<svg width="10pt" height="10pt"><g><use href="#z" x="1" y="2" fill="#000000"/></g><defs><symbol id="z"><path d="M 0 0 L 1 1"/></symbol></defs></svg>')
        self.assertEqual(compare(a, b)["verdict"], "Preserved")

    def test_broken_reference_is_unknown(self):
        a, _ = self.pair('<svg width="10pt" height="10pt"><use href="#missing"/></svg>', '<svg width="10pt" height="10pt"/>')
        self.assertEqual(morphology(a)["state"], "Unknown")

    def test_child_order_is_observable(self):
        one = '<symbol id="a"><path d="M0 0"/></symbol>'; two = '<symbol id="b"><path d="M1 1"/></symbol>'
        a, b = self.pair(f'<svg width="10pt" height="10pt"><use href="#a"/><use href="#b"/><defs>{one}{two}</defs></svg>', f'<svg width="10pt" height="10pt"><use href="#b"/><use href="#a"/><defs>{one}{two}</defs></svg>')
        self.assertEqual(compare(a, b)["verdict"], "Violated")

    def test_neutral_group_split_is_preserved(self):
        defs = '<defs><symbol id="a"><path d="M0 0"/></symbol></defs>'
        a, b = self.pair(f'<svg width="10pt" height="10pt"><g transform="matrix(1 0 0 -1 2 3)"><use href="#a" x="4"/></g>{defs}</svg>', f'<svg width="10pt" height="10pt"><g transform="matrix(1 0 0 -1 6 3)"><use href="#a" x="0"/></g>{defs}</svg>')
        self.assertEqual(compare(a, b)["verdict"], "Preserved")

    def test_transform_difference_is_violated(self):
        defs = '<defs><symbol id="a"><path d="M0 0"/></symbol></defs>'
        a, b = self.pair(f'<svg width="10pt" height="10pt"><g transform="matrix(1 0 0 1 1 0)"><use href="#a"/></g>{defs}</svg>', f'<svg width="10pt" height="10pt"><g transform="matrix(1 0 0 1 2 0)"><use href="#a"/></g>{defs}</svg>')
        self.assertEqual(compare(a, b)["verdict"], "Violated")

    def test_unsupported_paint_is_unknown(self):
        a, _ = self.pair('<svg width="10pt" height="10pt"><circle r="1"/></svg>', '<svg width="10pt" height="10pt"/>')
        self.assertEqual(morphology(a)["state"], "Unknown")

    def test_rect_path_and_rect_element_are_preserved(self):
        a, b = self.pair(
            '<svg width="10pt" height="10pt"><path fill="red" transform="translate(2 3)" d="M 0 0v 4h 5v -4Z"/></svg>',
            '<svg width="10pt" height="10pt"><rect x="2" y="3" width="5" height="4" fill="red"/></svg>')
        self.assertEqual(compare(a, b)["verdict"], "Preserved")

    def test_stroke_width_is_observable(self):
        base = '<svg width="10pt" height="10pt"><rect width="5" height="4" fill="none" stroke="red" stroke-width="{}"/></svg>'
        a, b = self.pair(base.format(1), base.format(2))
        self.assertEqual(compare(a, b)["verdict"], "Violated")

    def test_fill_rule_is_observable(self):
        base = '<svg width="10pt" height="10pt"><rect width="5" height="4" fill="red" fill-rule="{}"/></svg>'
        a, b = self.pair(base.format("nonzero"), base.format("evenodd"))
        self.assertEqual(compare(a, b)["verdict"], "Violated")

    def test_shape_order_is_observable(self):
        red = '<rect width="5" height="4" fill="red"/>'; blue = '<rect width="4" height="5" fill="blue"/>'
        a, b = self.pair(f'<svg width="10pt" height="10pt">{red}{blue}</svg>', f'<svg width="10pt" height="10pt">{blue}{red}</svg>')
        self.assertEqual(compare(a, b)["verdict"], "Violated")

    def test_malformed_rect_path_is_unknown(self):
        a, _ = self.pair('<svg width="10pt" height="10pt"><path d="M 0 0v 4h 5v -3Z"/></svg>', '<svg width="10pt" height="10pt"/>')
        self.assertEqual(morphology(a)["state"], "Unknown")


if __name__ == "__main__":
    unittest.main()
