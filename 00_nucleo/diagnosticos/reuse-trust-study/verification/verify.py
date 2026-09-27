#!/usr/bin/env python3
"""Read-only verification of this study and preceding evidence."""
import hashlib,json,pathlib,sys,importlib.util
P=pathlib.Path(__file__).resolve().parents[1];ROOT=P.parents[2]
def sha(f):return hashlib.sha256(f.read_bytes()).hexdigest()
v=json.loads((P/'provenance.json').read_text())
for name,digest in v['source_inputs'].items():assert sha(ROOT/name)==digest
before=json.loads((P/'source-before.json').read_text());after=json.loads((P/'source-after.json').read_text());assert before['sha256']==after['sha256']
for name,digest in after['files'].items():
 path=pathlib.Path(name);assert sha(path if path.is_absolute() else ROOT/path)==digest
assert sha(P/'bin/reuse-trust-study')==v['executable_sha256']
assert (P/'runtime/main.typ').read_text()=='First snapshot'
assert not (P/'runtime/reappearing.otf').exists()
r=json.loads((P/'results.json').read_text());assert r['source_snapshot']['retained_reads_previous_snapshot'];assert r['catalog_replacement']['matches_fresh_catalog'];assert r['ttc_acquisition_lifetime']['contents_equal']
for dataset in ['performance-baseline','font-lifecycle-experiment','font-microbenchmarks','font-microbenchmarks-vanilla','vanilla-font-mechanics','reuse-trust-study']:
 d=P.parent/dataset
 for line in (d/'MANIFEST.sha256').read_text().splitlines():
  digest,name=line.split('  ',1);assert sha(d/name)==digest
for name in ['font-lifecycle-experiment','font-microbenchmarks','font-microbenchmarks-vanilla','vanilla-font-mechanics','reuse-trust-study']:
 a=json.loads((P.parent/name/'completion-audit.json').read_text());assert sha(P.parent/(name+'.md'))==a['report_sha256']
print('Verified study inputs/consumer fingerprints, archived executable, restored scratch fixtures, all prior sealed evidence and reports.')
