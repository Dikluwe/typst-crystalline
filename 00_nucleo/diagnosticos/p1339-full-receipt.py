"""Close measurement authorship/provenance only; not a verification certificate."""
import importlib.util
import json
from pathlib import Path
import sys

sys.dont_write_bytecode=True
spec=importlib.util.spec_from_file_location('probe',Path(__file__).with_name('p1339-full-probe.py'))
m=importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

artifacts={str(p.relative_to(m.ROOT)):m.sha(p) for p in sorted(m.D.glob('p1339-full-*')) if p.is_file()}
a0=json.loads((m.D/'p1339-a0.json').read_text())
assert all(m.sha(b['path'])==b['sha256'] for b in a0['binaries'].values())
checks=[]
for stage,runner in [('explore','p1339-full-probe-explore-r0.py'),('focal-r1','p1339-full-probe-focal-r1.py'),('where-early','p1339-full-probe.py'),('final','p1339-full-probe.py'),('boundaries','p1339-full-probe.py')]:
    manifest=json.loads((m.D/('p1339-full-'+stage+'-manifest.json')).read_text())
    matched=manifest['runner_sha256']==m.sha(m.D/runner)
    assert matched,(stage,runner)
    checks.append(dict(stage=stage,runner=runner,hash_matches=matched))
show=json.loads((m.D/'p1339-full-show-final-vanilla-runs.json').read_text())
assert show['runner_sha256']==m.sha(m.D/'p1339-full-show-probe-r0.py')
m.save('receipt',dict(
    at=m.now(),executor='/root/p1339_measure_r1',regime='executado sem atestação de isolamento',capabilities_enforced=False,
    purpose='A.1 bilateral measurement and vanilla A.2 only; no contract, candidate, seal, RED gate or final PASS',
    provenance=m.provenance(),binaries=a0['binaries'],artifacts=artifacts,archived_runner_checks=checks,
    capabilities=dict(read=['skill and references','explicit corrected P1339 step','A0, resume, authority manifests','old focal probes','pinned binaries','four permitted vanilla source files after corresponding exploration'],forbidden_not_read=['crystalline productive source','productive L0','candidate implementation','private oracles'],write=['new p1339-full-* diagnostics','exclusive temporary compiler outputs'],context='fresh delegated task; no parent implementation context'),
    commands=[
        'python3 00_nucleo/diagnosticos/p1339-full-probe.py explore',
        'python3 00_nucleo/diagnosticos/p1339-full-probe.py focal-r1',
        'python3 00_nucleo/diagnosticos/p1339-full-probe.py where-early',
        'python3 00_nucleo/diagnosticos/p1339-full-probe.py final',
        'python3 00_nucleo/diagnosticos/p1339-full-boundaries.py boundaries',
        'python3 00_nucleo/diagnosticos/p1339-full-show-probe.py show-focal (archived r0)',
        'python3 00_nucleo/diagnosticos/p1339-full-show-probe.py show-final (archived r0)',
        'PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1339-full-show-probe.py show-focal-r1',
        'PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1339-full-show-compile.py show-compile-focal',
        'PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1339-full-summarize.py',
        'PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1339-full-receipt.py'],
    corrections=[
        dict(cause='invalid auxiliary API in angle observer',exploratory_cases=40,revision=1,change='calc.is-infinite -> signed-zero signum observable',result='focal vanilla observer executes; no expected values changed'),
        dict(cause='manual minimum i64 literal rejected by parser',exploratory_cases=6,revision=1,change='construct minimum with (-9223372036854775807 - 1)',result='focal reaches version.at diagnostic'),
        dict(cause='show query CLI lacks feature flags on antecedent',original_unknown_cells=108,revision=1,change='eval --in via public warning recommendation',result='antecedent rejects --in; original Unknown retained'),
        dict(cause='show query CLI lacks feature flags on antecedent',revision=2,change='compile identical fixture following actual --help capabilities',result='single focal fixture completed before operator stop message: vanilla compiles, antecedent rejects missing function.where in all four profiles; observes compile acceptance only; no full repetition and no reclassification of original Unknown')],
    cleanup='One generated p1339-full-probe.cpython-312.pyc was removed by exact path; no material input/output was deleted. Subsequent imports disable bytecode writes.',
    limits=['Angle NaN actual receiver remains Unknown; construction behavior differs outside this step scope','108 original show cells Unknown due CLI transport; separate later focal evidence does not rewrite them','plugin-specific receiver optional dimension unmeasured; native non-element representative measured','not technically isolated; source family allowlist honored voluntarily','source-informed boundaries entrypoint hashes are recorded here; its original manifest hashes imported runner base','final comparison catalog is measurement evidence, not sealed oracle expectations'],
    disposition='Measurement delivered, progression inconclusive; operator/human must resolve obligation/observability and architectural gate before L0 or implementation'))
