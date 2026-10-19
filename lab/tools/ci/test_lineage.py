import hashlib
from pathlib import Path
import tempfile
import unittest

from check_reverse_lineage import check_pair


class ReverseLineageTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.source = self.root / '01_core/unit.rs'
        self.prompt = self.root / '00_nucleo/prompts/unit.md'
        self.source.parent.mkdir(parents=True)
        self.prompt.parent.mkdir(parents=True)
        self.body = b'//! @prompt 00_nucleo/prompts/unit.md\n//! @prompt-hash 12345678\n\nfn unit() {}\n'
        self.source.write_bytes(self.body)
        digest = hashlib.sha256(self.body.replace(b'//! @prompt-hash 12345678\n', b'')).hexdigest()[:8]
        self.prompt.write_text(f'# Unit\nHash do Código: {digest}\n\nContract.\n')

    def test_valid_pair_and_header_only_reseal(self):
        self.assertEqual(check_pair(self.root, self.source), [])
        self.source.write_bytes(self.body.replace(b'12345678', b'87654321'))
        self.assertEqual(check_pair(self.root, self.source), [])

    def test_source_edit_is_not_hidden_by_unchanged_forward_seal(self):
        self.source.write_bytes(self.body + b'fn extra() {}\n')
        self.assertIn('reverse hash mismatch', ' '.join(check_pair(self.root, self.source)))

    def test_metadata_in_body_cannot_replace_preamble(self):
        self.prompt.write_text(self.prompt.read_text().replace('# Unit\n', '# Unit\n\n'))
        self.assertIn('preamble', ' '.join(check_pair(self.root, self.source)))

    def test_duplicate_metadata_fails(self):
        line = self.prompt.read_text().splitlines()[1]
        self.prompt.write_text(f'# Unit\n{line}\n{line}\n\nContract.\n')
        self.assertTrue(check_pair(self.root, self.source))

    def test_missing_owner_and_missing_header_fail(self):
        self.prompt.unlink()
        self.assertTrue(check_pair(self.root, self.source))
        self.source.write_text('fn unit() {}\n')
        self.assertTrue(check_pair(self.root, self.source))

    def test_parent_traversal_and_symlinks_fail(self):
        self.source.write_bytes(self.body.replace(b'00_nucleo/prompts/unit.md', b'../outside.md'))
        self.assertTrue(check_pair(self.root, self.source))
        self.source.write_bytes(self.body)
        self.prompt.unlink()
        try:
            self.prompt.symlink_to(self.source)
        except OSError:
            self.skipTest('symlinks unavailable')
        self.assertTrue(check_pair(self.root, self.source))

    def test_body_hash_marker_is_part_of_source_hash(self):
        self.source.write_bytes(self.body + b'//! @prompt-hash 12345678\n')
        self.assertTrue(check_pair(self.root, self.source))


if __name__ == '__main__':
    unittest.main()
