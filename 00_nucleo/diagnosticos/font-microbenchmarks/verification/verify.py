#!/usr/bin/env python3
"""Read-only verification of frozen measurement inputs and evidence manifests."""
import hashlib,json,pathlib,sys
P=pathlib.Path(__file__).resolve().parents[1];sys.path.insert(0,str(P));import measure as m
frozen=json.loads((P/'frozen-state.json').read_text());current=m.fingerprint()
assert (current['sha256'],current['head'])==(frozen['sha256'],frozen['head'])
build=json.loads((P/'build-provenance.json').read_text());assert m.sha(build['executable'])==build['sha256']
for n in ['r1','r2','r3']:
 r=json.loads((P/'rounds'/n/'receipt.json').read_text());assert r['valid'] and not r['contamination']
 assert r['before']==r['after']==frozen['sha256'] and r['head']==frozen['head'] and r['executable_sha256']==build['sha256']
controls=json.loads((P/'fixture-controls.json').read_text())
assert all(r['historical_equals_current_and_set_oracle'] for r in controls['fixtures'])
h=json.loads((P/'historical-provenance.json').read_text());source=m.ROOT/h['source'];assert m.sha(source)==h['source_sha256']
s=source.read_text();body=s[s.index('#[derive(Clone, PartialEq, Eq, Hash)]\npub struct Coverage'):s.index('/// Metadados de uma face de fonte')]
assert hashlib.sha256(body.encode()).hexdigest()==h['extracted_body_sha256']
assert (P/'src/historical.rs').read_text()=='// Diagnostic extraction of historical Coverage; not a production module.\n'+body
prior=m.ROOT/'00_nucleo/diagnosticos/font-lifecycle-experiment'
old=json.loads((prior/'build-state.json').read_text())
roots=('01_core/','02_shell/','03_infra/','04_wiring/','benches/','.cargo/','00_nucleo/prompts/')
for name,sha in old['files'].items():
 if name.startswith(roots) or name in ['Cargo.toml','Cargo.lock']:assert m.sha(m.ROOT/name)==sha,'Prior consumer/owner changed: '+name
for dataset in [prior,m.ROOT/'00_nucleo/diagnosticos/performance-baseline',P]:
 manifest=dataset/'MANIFEST.sha256'
 if not manifest.exists():continue
 for line in manifest.read_text().splitlines():
  digest,name=line.split('  ',1);assert m.sha(dataset/name)==digest,'Sealed dataset changed: '+str(dataset/name)
oldreport=json.loads((prior/'completion-audit.json').read_text());assert m.sha(prior.parent/'font-lifecycle-experiment.md')==oldreport['report_sha256']
report=P.parent/'font-microbenchmarks.md'
if (P/'completion-audit.json').exists():assert m.sha(report)==json.loads((P/'completion-audit.json').read_text())['report_sha256']
print('Verified: frozen sources/inputs/executable, all three receipts, historical extraction, prior consumers/owners and sealed datasets.')
