import tempfile
import unittest
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from p1240_raster import graph_box, normalized_svg, render
import xml.etree.ElementTree as ET

RENDERER = Path("/home/linuxbrew/.linuxbrew/bin/rsvg-convert")


class RasterLocalP1240(unittest.TestCase):
    def svg(self, text):
        handle = tempfile.NamedTemporaryFile(suffix=".svg", delete=False)
        path = Path(handle.name); handle.write(text.encode()); handle.close()
        self.addCleanup(path.unlink, missing_ok=True)
        return path

    def test_external_translation_is_not_observable_at_three_scales(self):
        left = self.svg('<svg xmlns="http://www.w3.org/2000/svg"><rect x="2" y="3" width="4" height="5" fill="red"/></svg>')
        right = self.svg('<svg xmlns="http://www.w3.org/2000/svg"><rect x="22" y="33" width="4" height="5" fill="red"/></svg>')
        for scale in (1, 2, 4):
            a, b = render(left, RENDERER, scale), render(right, RENDERER, scale)
            self.assertEqual(a["sha256"], b["sha256"])

    def test_linear_radial_and_pattern_controls_ignore_ids_and_defs_order(self):
        controls = [
            ('<linearGradient id="a"><stop stop-color="red"/><stop offset="1" stop-color="blue"/></linearGradient>', 'url(#a)'),
            ('<radialGradient id="a"><stop stop-color="red"/><stop offset="1" stop-color="blue"/></radialGradient>', 'url(#a)'),
            ('<pattern id="a" width="1" height="1" patternContentUnits="objectBoundingBox"><rect width=".5" height=".5" fill="red"/><rect x=".5" width=".5" height=".5" fill="blue"/><rect y=".5" width=".5" height=".5" fill="green"/><rect x=".5" y=".5" width=".5" height=".5" fill="purple"/></pattern>', 'url(#a)'),
        ]
        for definition, paint in controls:
            renamed = definition.replace('id="a"', 'id="z"')
            left = self.svg(f'<svg xmlns="http://www.w3.org/2000/svg"><defs>{definition}<rect id="unused"/></defs><rect x="2" y="3" width="8" height="6" fill="{paint}"/></svg>')
            right = self.svg(f'<svg xmlns="http://www.w3.org/2000/svg"><defs><rect id="unused"/>{renamed}</defs><rect x="22" y="33" width="8" height="6" fill="url(#z)"/></svg>')
            for scale in (1, 2, 4):
                a, b = render(left, RENDERER, scale), render(right, RENDERER, scale)
                self.assertEqual(a["sha256"], b["sha256"])

    def test_ancestor_transforms_are_composed(self):
        root = ET.fromstring('<svg><g transform="translate(10 20)"><g transform="scale(2)"><rect x="1" y="2" width="3" height="4"/></g></g></svg>')
        self.assertEqual(graph_box(root), __import__('p1240_raster').Box(12, 24, 18, 32))

    def test_stroke_expands_declared_geometry(self):
        root = ET.fromstring('<svg><rect x="2" y="3" width="4" height="5" fill="none" stroke="black" stroke-width="2"/></svg>')
        self.assertEqual(graph_box(root), __import__('p1240_raster').Box(1, 2, 7, 9))

    def test_mask_intersects_geometry_without_alpha_crop(self):
        root = ET.fromstring('<svg><defs><mask id="m"><rect x="4" y="5" width="3" height="2" fill="white"/></mask></defs><rect x="0" y="0" width="10" height="10" mask="url(#m)"/></svg>')
        self.assertEqual(graph_box(root), __import__('p1240_raster').Box(4, 5, 7, 7))

    def test_broken_mask_is_unknown_error(self):
        path = self.svg('<svg><rect width="10" height="10" mask="url(#missing)"/></svg>')
        with self.assertRaisesRegex(ValueError, "broken mask"):
            normalized_svg(path)

    def test_receipt_declares_input_and_renderer(self):
        path = self.svg('<svg xmlns="http://www.w3.org/2000/svg"><rect width="2" height="2"/></svg>')
        receipt = render(path, RENDERER, 1)
        self.assertEqual(receipt["declared_inputs"], [str(path.resolve()), str(RENDERER.resolve())])


if __name__ == "__main__":
    unittest.main()
