#!/usr/bin/env python3
"""Read-only verification; never rewrites earlier evidence."""
import json,pathlib,sys
P=pathlib.Path(__file__).resolve().parents[1];sys.path.insert(0,str(P));import measure as m
frozen=json.loads((P/'frozen-state.json').read_text());current=m.fingerprint()
assert (current['sha256'],current['head'])==(frozen['sha256'],frozen['head'])
build=json.loads((P/'build-provenance.json').read_text());assert m.sha(P/'bin/font-vanilla-comparison')==build['sha256']
for n in ['r1','r2','r3']:
 r=json.loads((P/'rounds'/n/'receipt.json').read_text());assert r['valid'] and not r['contamination']
 assert r['before']==r['after']==frozen['sha256'] and r['head']==frozen['head'] and r['executable_sha256']==build['sha256']
controls=json.loads((P/'fixture-controls.json').read_text())
assert all(r['coverage_set_equal_across_generic_and_full_paths'] and r['sample_glyph_ids_equal'] for r in controls['fixtures'])
prior=m.ROOT/'00_nucleo/diagnosticos/font-lifecycle-experiment'
old=json.loads((prior/'build-state.json').read_text())
roots=('01_core/','02_shell/','03_infra/','04_wiring/','benches/','.cargo/','00_nucleo/prompts/')
for name,digest in old['files'].items():
 if name.startswith(roots) or name in ['Cargo.toml','Cargo.lock']:assert m.sha(m.ROOT/name)==digest,'Prior consumer/owner changed: '+name
for dataset in [prior,prior.with_name('performance-baseline'),prior.with_name('font-microbenchmarks'),P]:
 manifest=dataset/'MANIFEST.sha256'
 if not manifest.exists():continue
 for line in manifest.read_text().splitlines():
  digest,name=line.split('  ',1);assert m.sha(dataset/name)==digest,'Sealed evidence changed: '+str(dataset/name)
for name in ['font-lifecycle-experiment','font-microbenchmarks']:
 audit=json.loads((prior.parent/name/'completion-audit.json').read_text());assert m.sha(prior.parent/(name+'.md'))==audit['report_sha256']
assert m.sha(m.ROOT/'lab/typst-original/target/release/typst')=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
if (P/'completion-audit.json').exists():assert m.sha(P.parent/'font-microbenchmarks-vanilla.md')==json.loads((P/'completion-audit.json').read_text())['report_sha256']
print('Verified: frozen source/inputs, three admitted receipts, archived executable, prior consumers/owners, sealed prior datasets and unchanged ratified vanilla CLI.')
