"""Pre-C capability correction, without changing frozen intention or oracle inputs."""
import importlib.util, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1337-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
m = r.read('manifest')
m['supersedes'] = dict(path=str(D / 'p1337-manifest.json'), sha256=r.sha(D / 'p1337-manifest.json'))
m['at'] = r.now()
m['roles']['reviewer']['executor'] = '/root/p1319_review'
m['roles']['reviewer']['context'] = 'Retained prior independent review context, not a fresh agent. Prior baseline/review evidence is allowed to this role; no authorship of P1337 tests or implementation.'
m['capability_revision'] = dict(reason='New reviewer spawn failed: agent thread limit reached. Reuse an existing independent reviewer before candidate or test/attack freezes.',
    changed=['reviewer executor and prior-review context'], unchanged=['normative L0', 'scope', 'baseline', 'implementation/test/attack authorities', 'oracles'],
    legacy_receipts='R0 and early observations remain immutable; R1 is the effective manifest for subsequent gates. No technical isolation attestation claimed.')
r.save('manifest-r1', m)
