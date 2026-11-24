import unittest, tempfile, pathlib, hashlib, os
import benchmark as b

class GuardTests(unittest.TestCase):
    def test_digest_mismatch(self):
        with tempfile.TemporaryDirectory() as d:
            p=pathlib.Path(d)/'x';p.write_bytes(b'x')
            with self.assertRaises(ValueError): b.verify(p,'0'*64)
    def test_environment_rejects_allocator_override(self):
        for key in ['LD_PRELOAD','LD_AUDIT','GLIBC_TUNABLES','MALLOC_ARENA_MAX']:
            with self.assertRaises(ValueError): b.environment({'PATH':'/usr/bin',key:'1'})
    def test_environment_canonical(self):
        self.assertEqual(list(b.environment({'Z':'1','A':'2'})),['A','Z'])
    def test_balancing(self):
        orders=b.orders(['a','b','c'],12,7)
        self.assertEqual(len(orders),12)
        for arm in ['a','b','c']:
            for pos in range(3):self.assertEqual(sum(o[pos]==arm for o in orders),4)
        with self.assertRaises(ValueError):b.orders(['a','b','c'],7,7)
    def test_lock_collision(self):
        with tempfile.TemporaryDirectory() as d:
            with b.lock(pathlib.Path(d)):
                with self.assertRaises(BlockingIOError):
                    with b.lock(pathlib.Path(d)):pass
    def test_stage_tamper(self):
        with tempfile.TemporaryDirectory() as d:
            p=pathlib.Path(d)/'x';p.write_bytes(b'x');expected=b.digest(p)
            p.write_bytes(b'y')
            with self.assertRaises(ValueError):b.verify(p,expected)
    def test_nonzero(self):
        with tempfile.TemporaryDirectory() as d:
            with self.assertRaises(RuntimeError):b.measure(['/bin/false'],pathlib.Path(d),b.environment({'PATH':'/usr/bin'}),'time')
    def test_common_argv(self):
        with tempfile.TemporaryDirectory() as d:
            a=b.argv(pathlib.Path(d))
            self.assertEqual(a,[d+'/typst','compile','--color','never',d+'/input.typ',d+'/output.pdf'])

class CampaignTests(unittest.TestCase):
    def setup_case(self,d):
        import json
        p=pathlib.Path(d);exe=p/'source';exe.write_bytes(b'ELF test double')
        fixture=p/'fixture.typ';fixture.write_text('hello')
        m={'self_contained_fixtures':True,'arms':{k:{'path':str(exe),'sha256':b.digest(exe)} for k in ['a','long-label']},'fixtures':{'case':{'path':str(fixture),'sha256':b.digest(fixture)}}}
        manifest=p/'manifest.json';manifest.write_text(json.dumps(m));return manifest,p/'out',p/'run'
    def test_campaign_identity_and_count(self):
        import json
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as d:
            args=self.setup_case(d);seen=[]
            def measure(argv,root,env,mode):
                seen.append((argv.copy(),root,env.copy()))
                return {'stdout_sha256':'same','stderr_sha256':'same',('rss_kib' if mode=='memory' else 'wall_ns'):100}
            def output(cmd,**kw):return b'<body>same</body>' if cmd[0]=='pdftotext' else 'fixed provenance'
            with patch.object(b,'measure',side_effect=measure),patch.object(b.subprocess,'check_output',side_effect=output):b.campaign(*args,rounds=2)
            self.assertTrue(all(x==seen[0] for x in seen));self.assertEqual(len(seen),12)
            state=json.loads((args[1]/'state.json').read_text());self.assertEqual(state['status'],'complete')
            rows=[json.loads(l) for l in (args[1]/'samples.jsonl').read_text().splitlines()]
            self.assertEqual(sum(x['warmup'] for x in rows),4)
    def test_tamper_marks_failed(self):
        import json
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as d:
            args=self.setup_case(d)
            def measure(argv,root,env,mode):
                (root/'input.typ').write_text('changed during execution')
                return {}
            with patch.object(b,'measure',side_effect=measure),self.assertRaises(ValueError):b.campaign(*args,rounds=2)
            self.assertEqual(json.loads((args[1]/'state.json').read_text())['status'],'failed')
            self.assertFalse((args[1]/'summary.json').exists())

class ClockTests(unittest.TestCase):
    def test_short_process_is_not_rounded_by_timeout_polling(self):
        import statistics
        with tempfile.TemporaryDirectory() as d:
            readings=[b.measure(['/bin/sleep','0.02'],pathlib.Path(d),{'PATH':'/usr/bin'},'time')['wall_ns'] for _ in range(5)]
            self.assertLess(statistics.median(readings),28_000_000,readings)

if __name__=='__main__':unittest.main()
