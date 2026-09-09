"""Freeze normative intention and role capabilities before independent tests/candidate."""
import importlib.util,re,sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('r',D/'p1336-record.py');r=importlib.util.module_from_spec(s);s.loader.exec_module(r)
state=r.state();r.verify(state)
prompt=(r.ROOT/r.L0).read_text();semantic=re.sub(r'^Hash do Código:.*\n','',prompt,flags=re.M)
r.save('manifest',dict(at=r.now(),regime='A/B with separate oracle/test, implementation, adversarial and review authors; executed without technical isolation attestation; no refinement seal',
    baseline_sha256=r.sha(D/'p1336-baseline.json'),measurement_sha256=r.sha(D/'p1336-measurement.json'),
    prompt=dict(path=r.L0,sha256=r.sha(r.ROOT/r.L0),normative_text=semantic,normative_sha256=r.hashlib.sha256(semantic.encode()).hexdigest(),
                metadata_policy='Only Hash do Código and source @prompt-hash may be resealed mechanically; every full revision is receipted. Any normative change invalidates the A/B intention freeze.'),
    source=r.SOURCE,pre_candidate_source_sha256=r.sha(r.ROOT/r.SOURCE),pre_candidate_source=(r.ROOT/r.SOURCE).read_text(),
    state=state,baseline_binary=r.read('baseline')['predecessor'],vanilla=r.read('baseline')['vanilla'],target=r.TARGET,
    roles=dict(operator=dict(executor='/root',context='conversation',write='step, L0 intention, productive implementation only field_access, orchestration p1336-*; never edits frozen oracles or verdict'),
               tests=dict(executor='/root/p1336_tests',context='fresh bounded prompt',read='frozen L0, vanilla/baseline CLI, baseline source only for integrating local tests; no candidate implementation',write='p1336-tests-*; cfg(test) module p1336_tests only before implementation'),
               adversary=dict(executor='/root/p1336_attacks',context='fresh bounded prompt',read='frozen intention/baseline; candidate only after attack plan freeze',write='p1336-attacks-* and isolated temporary mutant copies; never main product'),
               reviewer=dict(executor='/root/p1336_review',context='fresh bounded prompt',read='frozen intention/baseline, judged artifacts and outputs',write='p1336-review-* only; never corrects judged inputs')),
    policy=dict(unknown='mandatory missing observation, timeout, crash, opaque origin or identity mismatch blocks closure; planned opaque controls expect Unknown only',
        orders=['normal','repeat','reverse'],profiles=['default','html','a11y','html+a11y'],max_mutant_families=8,
        max_cli_workers=4,cli_timeout_seconds=30,command_timeout_seconds=2700,
        method_review='Two unsuccessful revisions on same cause without discriminatory gain require method review before any third full run',
        allowed_product_changes=[r.L0,r.SOURCE],commit=False,global_parity_claim=False)))
