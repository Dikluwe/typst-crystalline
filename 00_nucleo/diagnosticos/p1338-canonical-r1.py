"""Pre-C canonical-reference correction, preserving the original manifest."""
import copy, importlib.util, re, subprocess, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1338-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
m = r.read('manifest'); prompt = (r.ROOT / r.L0).read_text()
norm = re.sub(r'^Hash do Código:.*\n', '', prompt, flags=re.M)
assert norm == m['prompt']['normative_text'].replace('`bindings/access.md`', '`compiler/eval/bindings/access.md`')
source = (r.ROOT / r.SOURCE).read_text()
assert source == m['pre_candidate_source'], 'successor must precede test integration'
h = r.load('p1334-lineage-lib').hashes(r.L0, r.SOURCE)
old, new = h['recorded_a'], h['effective_a'][:8]
patch = f'*** Begin Patch\n*** Update File: {r.ROOT / r.SOURCE}\n@@\n-//! @prompt-hash {old}\n+//! @prompt-hash {new}\n*** End Patch\n'
subprocess.run(['apply_patch'], input=patch, text=True, check=True)
successor = copy.deepcopy(m)
successor.update(at=r.now(), supersedes_manifest_sha256=r.sha(D / 'p1338-manifest.json'),
    succession='Only inherited L0 reference bindings/access.md -> compiler/eval/bindings/access.md (ADR0130), and causal forward hash. Array obligation, roles, scope, methods, budget and policy unchanged; no candidate/test integration yet.',
    prompt=dict(path=r.L0, sha256=r.sha(r.ROOT/r.L0), normative_sha256=h['norm_sha256'], normative_text=norm),
    source_sha256=r.sha(r.ROOT/r.SOURCE), pre_candidate_source=(r.ROOT/r.SOURCE).read_text(), state=r.state())
r.verify(successor['state'])
r.save('manifest-r1', successor)
