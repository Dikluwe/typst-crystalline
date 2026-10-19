from pathlib import Path
import subprocess
import tempfile
import unittest

from build_oracle import verify_source


class OracleSourceTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()
        self.git('init', '-q')
        (self.root / 'source.rs').write_text('// source\n')
        self.git('add', '.')
        self.git('-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                 'commit', '--no-gpg-sign', '-qm', 'fixture')
        self.pin = self.git('rev-parse', 'HEAD')
        self.tree = self.git('rev-parse', 'HEAD^{tree}')

    def git(self, *args):
        return subprocess.check_output(['git', '-C', str(self.root), *args], text=True).strip()

    def test_clean_pinned_source_is_accepted(self):
        verify_source(self.root, self.pin, self.tree)

    def test_wrong_commit_or_tree_is_rejected(self):
        for pin, tree in [('0' * 40, self.tree), (self.pin, '0' * 40)]:
            with self.assertRaisesRegex(ValueError, 'commit/tree'):
                verify_source(self.root, pin, tree)

    def test_dirty_tracked_and_untracked_sources_are_rejected(self):
        (self.root / 'source.rs').write_text('// modified\n')
        with self.assertRaisesRegex(ValueError, 'dirty'):
            verify_source(self.root, self.pin, self.tree)
        self.git('checkout', '--', 'source.rs')
        (self.root / 'extra.rs').write_text('// extra\n')
        with self.assertRaisesRegex(ValueError, 'dirty'):
            verify_source(self.root, self.pin, self.tree)

    def test_subdirectory_is_not_an_independent_checkout(self):
        nested = self.root / 'nested'
        nested.mkdir()
        with self.assertRaisesRegex(ValueError, 'own upstream checkout'):
            verify_source(nested, self.pin, self.tree)
