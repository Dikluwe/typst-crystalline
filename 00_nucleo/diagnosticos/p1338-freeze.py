"""Freeze P1338 intention, baseline and role capabilities before tests/C."""
import importlib.util, re, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1338-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
s = r.state(); r.verify(s)
lineage = r.load('p1334-lineage-lib').hashes(r.L0, r.SOURCE)
assert lineage['recorded_a'] == lineage['effective_a'][:8]
assert lineage['recorded_b'] == lineage['code_b'][:8]
prompt = (r.ROOT / r.L0).read_text()
r.save('manifest', dict(at=r.now(), regime='A/B with separate authors; no technical isolation attestation; no refinement seal; prior role contexts retained due existing total agent limit',
    baseline_sha256=r.sha(D / 'p1338-baseline.json'), measurement_sha256=r.sha(D / 'p1338-measurement.json'),
    prompt=dict(path=r.L0, sha256=r.sha(r.ROOT / r.L0), normative_sha256=lineage['norm_sha256'], normative_text=re.sub(r'^Hash do Código:.*\n', '', prompt, flags=re.M)),
    source=r.SOURCE, source_sha256=r.sha(r.ROOT / r.SOURCE), pre_candidate_source=(r.ROOT / r.SOURCE).read_text(),
    state=s, target=r.TARGET, baseline_binary=r.read('baseline')['predecessor'], vanilla=r.read('baseline')['vanilla'],
    roles=dict(operator=dict(executor='/root', context='conversation and prior implementation', read='L0, baseline and public evidence; new private tests/oracles only after C', write='new step, L0/source implementation pair, p1338 orchestration/report; not tests/oracles/verdict'),
        tests=dict(executor='/root/p1337_tests', context='prior P1337 test-author context retained; no P1338 candidate exists', read='frozen L0, baseline source for integration, vanilla/baseline CLI; no candidate source', write='p1338-tests-* and pre-C local cfg(test) module; exact single authorized P1337 Array sentinel migration'),
        adversary=dict(executor='/root/p1337_attacks', context='prior P1337 adversary context retained; former candidate now baseline', read='frozen L0/baseline then frozen tests; C only after attack-plan freeze', write='p1338-attacks-* and exclusive temporary copies; not main/tests/oracles'),
        reviewer=dict(executor='/root/p1319_review', context='prior review context retained; no implementation/test authorship', read='judged artifacts; never historical materialization/context', write='p1338-review-* only; no fixes to judged artifacts')),
    policy=dict(allowed_product_changes=[r.L0,r.SOURCE], metadata='Hash do Código and @prompt-hash may be mechanically resealed with receipts; normative text frozen',
        test_succession='Only p1337_preserve_array_ast_message_and_total_span_debt name and expected anchor/message may migrate before C; keep case and comparators, all other prior tests byte-identical',
        profiles=['default','html','a11y','html+a11y'], orders=['normal','repeat','reverse'], max_cli_workers=4,
        mutant_families=4, cli_timeout_seconds=30, command_timeout_seconds=2700,
        unknown='Mandatory Unknown blocks; only planned opaque harness controls expect Unknown; debt preservation not parity',
        budget='Four productive mutation families; paired build profile frozen pre-C; two revisions without gain same cause require method review. No full gates while focal failures remain. Compile failure or stale cache not kill.',
        stage=False, commit=False, global_parity_claim=False)))
