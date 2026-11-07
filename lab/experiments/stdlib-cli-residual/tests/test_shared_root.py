"""Black-box shared-root contract; C supplies the completed driver at execution.

No import or source inspection of the driver. The child is an observable fake
compiler process; these tests qualify transport, never Typst compilation.
"""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

TASK_RAM = Path('/dev/shm/tekt-stdlib-cli-residual-20261106')
DRIVER = os.environ.get('TEKT_CLI_SHARED_DRIVER')

FAKE = '''#!/usr/bin/env python3
import hashlib,json,os,sys
from pathlib import Path
assert sys.argv[1] == 'compile'
source=Path(sys.argv[2]);output=Path(sys.argv[3]);root=Path(sys.argv[sys.argv.index('--root')+1])
payload=source.read_bytes()
observation=dict(source=str(source),root=str(root),cwd=os.getcwd(),output=str(output),
                 source_hash=hashlib.sha256(payload).hexdigest(),source_mtime_ns=source.stat().st_mtime_ns,
                 source_payload=payload.decode('utf-8'))
(output.parent/'observed.json').write_text(json.dumps(observation))
output.write_bytes(b'%PDF-1.7\\nTEKT_EPOCH_0\\n%%EOF\\n')
'''


@unittest.skipUnless(DRIVER, 'C must set TEKT_CLI_SHARED_DRIVER after A/B complete')
class SharedInputContract(unittest.TestCase):
    def setUp(self):
        self.assertTrue(TASK_RAM.is_dir(), 'exclusive task RAM must exist')
        self.temporary = tempfile.TemporaryDirectory(prefix='shared-test-', dir=TASK_RAM)
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.fixtures = self.root / 'fixtures'; self.fixtures.mkdir()
        (self.fixtures / 'no_import.typ').write_text('#let observed = sys.inputs\n')
        self.binary = self.root / 'fake'; self.binary.write_text(FAKE); self.binary.chmod(0o700)
        self.cache = self.root / 'cache'; (self.cache / 'typst/packages').mkdir(parents=True)
        self.data_home = self.root / 'data'; self.data_home.mkdir()
        self.shared = self.root / 'shared'
        self.batch = self.root / 'batch'
        self.env = dict(os.environ, XDG_DATA_HOME=str(self.data_home), PYTHONDONTWRITEBYTECODE='1')

    def command(self, shared=None):
        args = [sys.executable, '-B', DRIVER, '--fixtures', str(self.fixtures),
                '--fixture', 'no_import.typ', '--ram-dir', str(self.batch),
                '--package-cache', str(self.cache), '--inherit-data-home',
                '--rounds', '2', '--repeats', '1', '--cpu', str(min(os.sched_getaffinity(0))),
                '--variant', 'bas=' + str(self.binary), '--variant', 'dup=' + str(self.binary)]
        if shared is not None:
            args += ['--shared-input-root', str(shared)]
        return args

    def run_driver(self, shared=None):
        return subprocess.run(self.command(shared), env=self.env, stdout=subprocess.PIPE,
                              stderr=subprocess.STDOUT, text=True, timeout=60)

    def observations(self):
        return [json.loads(path.read_text()) for path in self.batch.rglob('observed.json')]

    def test_all_launches_share_source_root_cwd_without_output_or_source_mutation(self):
        result = self.run_driver(self.shared)
        self.assertEqual(result.returncode, 0, result.stdout)
        rows = self.observations()
        self.assertGreaterEqual(len(rows), 4)
        self.assertEqual({r['source'] for r in rows}, {str(self.shared / 'no_import.typ')})
        self.assertEqual({r['root'] for r in rows}, {str(self.shared)})
        self.assertEqual({r['cwd'] for r in rows}, {str(self.shared)})
        self.assertEqual(len({r['output'] for r in rows}), len(rows))
        self.assertEqual(len({r['source_hash'] for r in rows}), 1)
        self.assertEqual(len({r['source_mtime_ns'] for r in rows}), 1)
        payload = (self.shared / 'no_import.typ').read_bytes()
        self.assertEqual(payload.count(b'TEKT_EPOCH_0'), 1)
        self.assertEqual(hashlib.sha256(payload).hexdigest(), rows[0]['source_hash'])
        self.assertEqual((self.shared / 'no_import.typ').stat().st_mtime_ns, rows[0]['source_mtime_ns'])
        self.assertEqual((self.fixtures / 'no_import.typ').read_text(), '#let observed = sys.inputs\n')
        for row in rows:
            self.assertTrue(Path(row['output']).is_relative_to(self.batch))

    def test_absent_flag_keeps_independent_case_input_roots(self):
        result = self.run_driver()
        self.assertEqual(result.returncode, 0, result.stdout)
        rows = self.observations()
        self.assertGreaterEqual(len(rows), 4)
        self.assertEqual(len({r['source'] for r in rows}), len(rows))
        self.assertEqual(len({r['root'] for r in rows}), len(rows))
        self.assertFalse(self.shared.exists())
        for row in rows:
            self.assertEqual(row['root'], row['cwd'])
            self.assertEqual(Path(row['source']).parent, Path(row['root']))
            self.assertTrue(Path(row['source']).is_relative_to(self.batch))

    def test_existing_shared_root_refused_before_any_child(self):
        self.shared.mkdir()
        sentinel = self.shared / 'preserve'; sentinel.write_text('keep')
        result = self.run_driver(self.shared)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(sentinel.read_text(), 'keep')
        self.assertEqual(self.observations(), [])

    def test_symlink_shared_root_refused_before_any_child(self):
        target = self.root / 'target'; target.mkdir()
        self.shared.symlink_to(target, target_is_directory=True)
        result = self.run_driver(self.shared)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(list(target.iterdir()), [])
        self.assertEqual(self.observations(), [])

    def test_fixture_overlap_refused_before_any_child(self):
        result = self.run_driver(self.fixtures / 'shared-child')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(list(self.fixtures.iterdir()), [self.fixtures / 'no_import.typ'])
        self.assertEqual(self.observations(), [])

    def test_output_overlap_refused_before_any_child(self):
        result = self.run_driver(self.batch / 'shared-child')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(self.observations(), [])


if __name__ == '__main__':
    unittest.main()
