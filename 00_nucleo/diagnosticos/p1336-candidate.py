"""Pin the implementation after independent RED, before GREEN is observed."""
import difflib, importlib.util, re, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1336-record.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
m = r.read('manifest')
state = r.state()
r.verify(state)
prompt = (r.ROOT / r.L0).read_text()
normative = re.sub(r'^Hash do Código:.*\n', '', prompt, flags=re.M)
assert normative == m['prompt']['normative_text']
red = r.read('unit-red')
assert red['exit'] == 101
assert (D / 'p1336-review-red.md').is_file()
before = red['after']['product_inventory'][r.SOURCE]
current = (r.ROOT / r.SOURCE).read_text()
assert r.sha(r.ROOT / r.SOURCE) != before
r.save('candidate', dict(
    at=r.now(), manifest_sha256=r.sha(D / 'p1336-manifest.json'),
    tests_freeze_sha256=r.sha(D / 'p1336-tests-freeze.json'),
    red_sha256=r.sha(D / 'p1336-unit-red.json'),
    red_review_sha256=r.sha(D / 'p1336-review-red.md'),
    attacks_plan_sha256=r.sha(D / 'p1336-attacks-plan.md'),
    normative_sha256=r.hashlib.sha256(normative.encode()).hexdigest(),
    source_sha256=r.sha(r.ROOT / r.SOURCE), source=current,
    delta_from_pre_test_manifest=''.join(difflib.unified_diff(
        m['pre_candidate_source'].splitlines(True), current.splitlines(True),
        fromfile='frozen-before-tests', tofile='candidate-with-independent-tests')),
    state=state,
    observation_policy='Implementation applied after independent RED review; GREEN output not yet observed. Subsequent lineage reseal may change metadata only.'
))
