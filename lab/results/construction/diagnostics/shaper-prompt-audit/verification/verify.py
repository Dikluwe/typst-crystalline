#!/usr/bin/env python3
import hashlib,json,pathlib,subprocess
P=pathlib.Path(__file__).resolve().parents[1];ROOT=P.parents[2]
def sha(f):return hashlib.sha256(f.read_bytes()).hexdigest()
v=json.loads((P/'provenance.json').read_text())
for f,h in v['source_inputs'].items():assert sha(ROOT/f)==h
before=json.loads((P/'source-before.json').read_text());after=json.loads((P/'source-after.json').read_text());assert before['sha256']==after['sha256']
for name,h in after['files'].items():
 f=pathlib.Path(name);assert sha(f if f.is_absolute() else ROOT/f)==h
for row in json.loads((P/'inputs.json').read_text()):assert sha(pathlib.Path(row['path']))==row['sha256']
assert sha(P/'bin/shaper-prompt-audit')==v['executable_sha256']
r=json.loads((P/'results.json').read_text())
for k in ['tracking_size','tracking_precision']:
 c=r[k]['result'];assert c['fresh_differ'];assert not c['ab_matches_fresh'] and not c['ba_matches_fresh'];assert c['mixed_ab']==[c['fresh_a'][0]]*2 and c['mixed_ba']==[c['fresh_b'][0]]*2
assert r['sparse_fallback_scoring']['expected_global_slot']==6
assert 'far fallback with long name' in r['sparse_fallback_scoring']['actual_emitted_font_list']
assert r['same_font_script_segmentation']['actual_subrun_texts']==['abαβ']
for i in [1,2]:assert json.loads((P/f'repetition-{i}.json').read_text())==r
assert subprocess.check_output(['git','diff','HEAD','--binary'],cwd=ROOT)==(P/'incoming.patch').read_bytes()
for name in ['performance-baseline','font-lifecycle-experiment','font-microbenchmarks','font-microbenchmarks-vanilla','vanilla-font-mechanics','reuse-trust-study','reuse-key-followup','shaping-axis-key-fix','shaper-prompt-audit']:
 d=P.parent/name
 for line in (d/'MANIFEST.sha256').read_text().splitlines():
  h,f=line.split('  ',1);assert sha(d/f)==h
 if (d/'completion-audit.json').exists():
  a=json.loads((d/'completion-audit.json').read_text())
  if 'report_sha256' in a:assert sha(P.parent/(name+'.md'))==a['report_sha256']
print('Verified controlled counterexamples, repetitions, stable consumers/inputs, incoming patch, and all sealed evidence.')
