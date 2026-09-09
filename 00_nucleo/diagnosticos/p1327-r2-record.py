"""Additive P1327 scope succession: only run_eval diagnostic ordering in wiring."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('base', Path(__file__).with_name('p1327-record.py'))
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
base.PAIRS += ('04_wiring/src/main.rs', '00_nucleo/prompts/wiring.md')
original_save = base.save
def save(name, data):
    manifest = base.D / 'p1327-r2-manifest.json'
    if manifest.exists():
        data['manifest_sha256'] = base.sha(manifest)
    original_save('r2-' + name, data)
base.save = save
def __getattr__(name):
    return getattr(base, name)
if __name__ == '__main__':
    if sys.argv[1] == 'init':
        s = base.state()
        base.verify(s)
        prior = json.loads((base.D / 'p1327-workspace-tests.json').read_text())
        assert s['product_inventory'] == prior['after']['product_inventory']
        save('baseline', dict(at=base.now(), state=s,
            previous_manifest_sha256=base.sha(base.D / 'p1327-manifest.json'),
            failed_cli_sha256=base.sha(base.D / 'p1327-ab-cli-candidate.json'),
            review_sha256=base.sha(base.D / 'p1327-review-cli-failure.md'),
            original_wiring=(base.ROOT / base.PAIRS[4]).read_text(),
            original_wiring_prompt=(base.ROOT / base.PAIRS[5]).read_text(),
            owner_set=list(base.PAIRS),
            succession='Additional productive owner wiring, only run_eval error-before-warnings presentation. Frozen original oracles unchanged; C1 failures retained.',
            binary_sha256=base.sha(base.TARGET + '/release/typst')))
    else:
        base.command(sys.argv[1], sys.argv[2:])
