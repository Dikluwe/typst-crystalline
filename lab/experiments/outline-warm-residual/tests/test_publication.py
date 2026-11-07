"""Protocol controls only: EOF/identity fixtures are not certified PDFs."""
import importlib.util
import os
from pathlib import Path
import tempfile
import unittest

TASK_RAM = Path('/dev/shm/tekt-outline-warm-20261106')
DRIVER = Path(os.environ.get('TEKT_OUTLINE_MEASURE_DRIVER',
                            str(Path(__file__).resolve().parents[1] / 'measure.py')))
spec = importlib.util.spec_from_file_location('outline_publication_driver_contract', DRIVER)
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)

COMPLETE = b'%PDF-1.7\nprotocol-fixture-only\n%%EOF\n'


class PublicationProtocol(unittest.TestCase):
    def setUp(self):
        base = Path(os.environ.get('TMPDIR', str(TASK_RAM)))
        self.assertTrue(base.is_absolute() and base.is_relative_to(TASK_RAM))
        base.mkdir(parents=True, exist_ok=True)
        self.assertTrue(base.resolve().is_relative_to(TASK_RAM))
        self.temporary = tempfile.TemporaryDirectory(prefix='publication-b-', dir=base)
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.pdf = self.root / 'output.pdf'

    def test_signature_matches_all_four_identity_fields(self):
        self.pdf.write_bytes(COMPLETE)
        stat = self.pdf.stat()
        self.assertEqual(driver.signature(stat),
                         (stat.st_dev, stat.st_ino, stat.st_mtime_ns, stat.st_size))

    def test_complete_new_publication_returns_its_signature(self):
        self.pdf.write_bytes(COMPLETE)
        self.assertEqual(driver.published(self.pdf, None), driver.signature(self.pdf.stat()))

    def test_same_previous_signature_is_rejected_even_with_complete_eof(self):
        self.pdf.write_bytes(COMPLETE)
        previous = driver.signature(self.pdf.stat())
        self.assertEqual(driver.published(self.pdf, None), previous)
        self.assertIsNone(driver.published(self.pdf, previous))

    def test_incomplete_file_is_not_a_publication(self):
        self.pdf.write_bytes(b'%PDF-1.7\nincomplete-protocol-fixture\n')
        self.assertIsNone(driver.published(self.pdf, None))

    def test_atomic_same_content_replacement_is_a_new_publication(self):
        self.pdf.write_bytes(COMPLETE)
        previous = driver.signature(self.pdf.stat())
        replacement = self.root / 'replacement.pdf'
        replacement.write_bytes(COMPLETE)
        os.replace(replacement, self.pdf)
        current = driver.signature(self.pdf.stat())
        self.assertNotEqual(current, previous)
        self.assertEqual(self.pdf.read_bytes(), COMPLETE)
        self.assertEqual(driver.published(self.pdf, previous), current)

    def test_absent_file_returns_none(self):
        self.assertFalse(self.pdf.exists())
        self.assertIsNone(driver.published(self.pdf, None))


if __name__ == '__main__':
    unittest.main()
