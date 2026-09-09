"""Freeze A/B intention and capabilities before private tests or implementation."""
import importlib.util, re, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('r', D / 'p1337-record.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
s = r.state(); r.verify(s)
lineage = r.load('p1334-lineage-lib').hashes(r.L0, r.SOURCE)
assert lineage['recorded_a'] == lineage['effective_a'][:8]
assert lineage['recorded_b'] == lineage['code_b'][:8]
prompt = (r.ROOT / r.L0).read_text()
r.save('manifest', dict(at=r.now(), regime='A/B with separate authors; executed without technical isolation attestation; no refinement seal',
    baseline_sha256=r.sha(D / 'p1337-baseline.json'), measurement_sha256=r.sha(D / 'p1337-measurement.json'),
    prompt=dict(path=r.L0, sha256=r.sha(r.ROOT / r.L0), normative_sha256=lineage['norm_sha256'], normative_text=re.sub(r'^Hash do Código:.*\n', '', prompt, flags=re.M)),
    source=r.SOURCE, source_sha256=r.sha(r.ROOT / r.SOURCE), pre_candidate_source=(r.ROOT / r.SOURCE).read_text(),
    state=s, target=r.TARGET, baseline_binary=r.read('baseline')['predecessor'], vanilla=r.read('baseline')['vanilla'],
    roles=dict(operator=dict(executor='/root', context='conversation', read='baseline, L0, evidence; private tests only after C', write='step, L0, implementation pair, p1337 orchestration/report'),
        tests=dict(executor='/root/p1337_tests', context='fresh bounded prompt', read='L0, baseline source for integration, vanilla/baseline CLI; no candidate source', write='p1337-tests-* and pre-C cfg(test) module; only three explicitly superseded P1336 expectations and test rename'),
        adversary=dict(executor='/root/p1337_attacks', context='fresh bounded prompt', read='frozen intention/baseline before attack plan; C only after plan freeze', write='p1337-attacks-* and exclusive temporary mutant workspace/target'),
        reviewer=dict(executor='/root/p1337_review', context='fresh bounded prompt', read='judged artifacts, no prohibited historical materialization/context', write='p1337-review-* only; cannot fix judged inputs')),
    policy=dict(allowed_product_changes=[r.L0,r.SOURCE], metadata='Hash do Código and @prompt-hash may be mechanically resealed with receipts; normative text cannot change without reopening',
        test_succession='Only three Bool/None/Auto tuples in p1336_preserve_other_fallback_names_and_ast_span and honest successor rename may migrate before C; protect all other tests',
        profiles=['default','html','a11y','html+a11y'], orders=['normal','repeat','reverse'], max_cli_workers=4,
        mutant_families=5, cli_timeout_seconds=30, command_timeout_seconds=2700,
        unknown='Mandatory Unknown blocks; only deliberately opaque harness controls expect Unknown; baseline debt preservation is not parity credit',
        budget='Five productive families; two revisions without gain on same cause require method review before another full run. Freeze mutation build profile before C; compile failures or stale cache never count as kills.',
        stage=False, commit=False, global_parity_claim=False)))
