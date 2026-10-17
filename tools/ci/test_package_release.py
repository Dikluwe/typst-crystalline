import hashlib
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest
import zipfile

from package_release import package


class PackageTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        for name in ['README.md', 'LICENSE', 'NOTICE', 'Cargo.lock', 'rust-toolchain.toml']:
            (self.root / name).write_text('fixture\n')
        (self.root / '.gitignore').write_text('/target/\n/dist/\n')
        subprocess.run(['git', 'init', '-q', str(self.root)], check=True)
        self.git('add', '.')
        self.git('-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid', 'commit', '--no-gpg-sign', '-qm', 'fixture')

    def git(self, *args):
        return subprocess.check_output(['git', '-C', str(self.root), *args], text=True).strip()

    def binary(self, target, windows=False):
        p = self.root / 'target' / target / 'release' / ('typst.exe' if windows else 'typst')
        p.parent.mkdir(parents=True)
        p.write_bytes(b'fixture binary')
        p.chmod(0o755)
        return p

    def test_tar_has_binary_and_provenance_with_verified_digest(self):
        target = 'x86_64-unknown-linux-musl'
        binary = self.binary(target)
        archive = package(self.root, target, 'cross', rustc='fixture rustc')
        with tarfile.open(archive) as f:
            meta = json.load(f.extractfile(f'typst-{target}/BUILD.json'))
            self.assertEqual(meta['binary_sha256'], hashlib.sha256(binary.read_bytes()).hexdigest())
            self.assertEqual(meta['commit'], self.git('rev-parse', 'HEAD'))
            self.assertEqual(meta['tree'], self.git('rev-parse', 'HEAD^{tree}'))
            self.assertIn('--locked', meta['build_command'])
        self.assertIn(hashlib.sha256(archive.read_bytes()).hexdigest(), archive.with_suffix(archive.suffix + '.sha256').read_text())

    def test_windows_archive(self):
        target = 'x86_64-pc-windows-msvc'
        self.binary(target, windows=True)
        archive = package(self.root, target, 'cargo', rustc='fixture rustc')
        with zipfile.ZipFile(archive) as f:
            self.assertEqual(f.read(f'typst-{target}/typst.exe'), b'fixture binary')

    def test_dirty_source_is_rejected(self):
        self.binary('test-target')
        (self.root / 'README.md').write_text('dirty')
        with self.assertRaisesRegex(ValueError, 'dirty'):
            package(self.root, 'test-target', 'cargo', rustc='fixture rustc')

    def test_missing_binary_and_invalid_target_are_rejected(self):
        with self.assertRaises(FileNotFoundError):
            package(self.root, 'test-target', 'cargo', rustc='fixture rustc')
        with self.assertRaises(ValueError):
            package(self.root, '../escape', 'cargo', rustc='fixture rustc')

    def test_untracked_source_is_rejected(self):
        self.binary('test-target')
        (self.root / 'untracked.rs').write_text('fn uncommitted() {}\n')
        with self.assertRaisesRegex(ValueError, 'dirty'):
            package(self.root, 'test-target', 'cargo', rustc='fixture rustc')
