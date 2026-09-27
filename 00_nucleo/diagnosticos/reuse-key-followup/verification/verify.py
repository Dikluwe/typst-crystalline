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
assert sha(P/'bin/reuse-key-followup')==v['executable_sha256']
assert (P/'runtime/main.typ').read_text()=='First snapshot'

r=json.loads((P/'results.json').read_text())
for row in json.loads((P/'inputs.json').read_text()):assert sha(pathlib.Path(row['path']))==row['sha256']
for case in r['axis_cases']:
 c=case['counterexample'];assert c and c['fresh_a']!=c['fresh_b'];assert not c['ab_matches_fresh'] and not c['ba_matches_fresh'];assert c['mixed_ab']==[c['fresh_a'][0]]*2;assert c['mixed_ba']==[c['fresh_b'][0]]*2
assert r['split_lifecycle']['same_font_buffer'];assert r['split_lifecycle']['fresh_source_with_retained_pool']=='Edited snapshot'
for i in [1,2]:assert json.loads((P/f'repetition-{i}.json').read_text())==r
for dataset in ['performance-baseline','font-lifecycle-experiment','font-microbenchmarks','font-microbenchmarks-vanilla','vanilla-font-mechanics','reuse-trust-study','reuse-key-followup']:
 d=P.parent/dataset
 for line in (d/'MANIFEST.sha256').read_text().splitlines():
  digest,name=line.split('  ',1);assert sha(d/name)==digest
for name in ['font-lifecycle-experiment','font-microbenchmarks','font-microbenchmarks-vanilla','vanilla-font-mechanics','reuse-trust-study','reuse-key-followup']:
 a=json.loads((P.parent/name/'completion-audit.json').read_text());assert sha(P.parent/(name+'.md'))==a['report_sha256']
print('Verified study inputs/consumer fingerprints, archived executable, restored scratch fixtures, all prior sealed evidence and reports.')
