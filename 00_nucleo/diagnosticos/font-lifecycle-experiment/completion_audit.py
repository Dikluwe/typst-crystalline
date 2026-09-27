#!/usr/bin/env python3
"""Check preserved evidence without editing production or the original baseline."""
import difflib, json, pathlib
import experiment as e
p=e.OUT
initial=json.loads((p/'initial.json').read_text())
owned=[]
patch=[]
for name,old_sha in initial['originals'].items():
    backup=p/'originals'/(name+'.snapshot')
    assert e.c.sha(backup)==old_sha, 'Original byte backup changed: '+name
    current=e.c.ROOT/name
    if e.c.sha(current)!=old_sha:
        owned.append({'path':name,'incoming_sha256':old_sha,'final_sha256':e.c.sha(current)})
        patch.extend(difflib.unified_diff(backup.read_text().splitlines(True),current.read_text().splitlines(True),fromfile='incoming/'+name,tofile='final/'+name))
assert len(owned)==20
(p/'state/task-only.patch').write_text(''.join(patch))
e.c.save(pathlib.Path('task-files.json'),{'utc':e.c.utc(),'head':initial['head'],'files':owned,'note':'Difference from incoming bytes, not difference from HEAD; preserves existing edits.'})

def is_consumer(k):
    return k.startswith(('01_core/','02_shell/','03_infra/','04_wiring/','benches/','.cargo/')) or k in ['Cargo.toml','Cargo.lock','rust-toolchain','rust-toolchain.toml','build.rs']
before=json.loads((p/'before-build-state.json').read_text())['files']
lazy=json.loads((p/'state/lazy-built.json').read_text())['files']
keys={k for k in before if is_consumer(k)}|{k for k in lazy if is_consumer(k)}
reconstructed={k:initial['originals'].get(k,lazy.get(k)) for k in keys}
differences=[{'path':k,'before':before.get(k),'incoming':reconstructed[k]} for k in sorted(keys) if before.get(k)!=reconstructed[k]]
assert not differences
e.c.save(pathlib.Path('before-consumer-control.json'),{'comparison':'Incoming task bytes (initial originals for owned files, lazy-built snapshot for unchanged files) vs preserved before executable build state','union_keys_checked':len(keys),'added_or_removed_keys':[k for k in sorted(keys) if k not in before or k not in lazy],'differences':differences})

fp=e.c.snapshot('final'); e.c.require_built_state(fp); e.identities()
expected=json.loads((p/'build-state.json').read_text())
final_consumer_changes=[k for k in sorted(keys) if fp['files'].get(k)!=before.get(k)]
assert set(final_consumer_changes)=={r['path'] for r in owned if is_consumer(r['path'])}, final_consumer_changes
assert fp['head']==initial['head']
receipts=[]
for f in sorted((p/'receipts').glob('*.json')):
    if not f.stem.startswith(('plain-r','traced-r','catalog-r')): continue
    r=json.loads(f.read_text())
    assert r['valid'] and not r['contamination']
    assert r['before']==r['after']==expected['sha256'] and r['head']==expected['head']
    receipts.append(f.name)
assert len(receipts)==33
probes=json.loads((p/'probes.json').read_text())
for r in probes.values(): assert e.c.sha(r['path'])==r['sha256']
profiles=json.loads((p/'benchmark-build-profiles.json').read_text())
assert len(profiles)==8
for r in profiles:
    assert r['profile']['opt_level']=='3' and not r['profile']['debug_assertions']
    r['sha256_at_completion']=e.c.sha(r['executable'])
for name in ['memory-results','control-results']:
    assert all(r['exit_code']==0 for r in json.loads((p/(name+'.json')).read_text())['rows'])
validation=json.loads((p/'output-validation.json').read_text())
for r in validation['comparisons']:
    assert all(r[k] for k in ['pages_match','geometry_match','normalized_text_match','text_font_control_match']),r
    # Empty documents retain standard unused font resources in all Tekt variants; vanilla omits them.
    if not r['font_resource_names_match']:
        assert r['fixture'] in ['image','shapes'] and r['comparison']=='interval-vanilla'
fallback=json.loads((p/'fallback-validation.json').read_text())
assert all(r['text_matches_before'] and r['fonts_match_before'] for r in fallback['comparisons'])

# Read-only verification of the original immutable dataset. Never import its runner.
baseline=e.c.ROOT/'00_nucleo/diagnosticos/performance-baseline'
manifest=baseline/'MANIFEST.sha256'
verified=[]
for entry in manifest.read_text().splitlines():
    digest,name=entry.split('  ',1)
    assert e.c.sha(baseline/name)==digest, 'Original baseline changed: '+name
    verified.append(name)

samples=[]
counts={}
for directory in ['samples','catalog']:
    for f in (p/directory).glob('*/samples.json'):
        rows=json.loads(f.read_text())['samples'];samples.extend(rows)
        for r in rows:
            key=('/'.join([directory,'traced' if r.get('instrumented') else 'plain',r.get('fixture','system-catalog'),r['variant']]))
            counts[key]=counts.get(key,0)+1
assert all(n==(18 if '/traced/' in k else 36) for k,n in counts.items())
report=e.c.ROOT/'00_nucleo/diagnosticos/font-lifecycle-experiment.md'
e.c.save(pathlib.Path('completion-audit.json'),{'utc':e.c.utc(),'head':fp['head'],'frozen_fingerprint':fp['sha256'],'stable_at_completion':True,'admitted_blocks':receipts,'sample_counts':counts,'sample_utc_start':min(r['utc_start'] for r in samples),'sample_utc_end':max(r['utc_end'] for r in samples),'original_baseline_manifest_sha256':e.c.sha(manifest),'original_baseline_entries_verified':len(verified),'original_baseline_unchanged':True,'incoming_backup_hashes_verified':True,'final_consumer_changes':final_consumer_changes,'unmodified_fonts_consumer_and_owner':True,'all_qualifying_pdf_controls_passed':True,'pdf_byte_identity_is_not_claimed':True,'memory_512_mib_cases_passed':True,'fallback_controls_passed':True,'benchmark_profiles_and_current_hashes':profiles,'report_sha256':e.c.sha(report),'scope_note':'Production changes limited to the ten listed pairs. New experiment diagnostics only; no edit to the original baseline, ADRs, shared nuclei, export sources or existing benchmarks.'})
print('Completion audit passed; original baseline manifest verified; sources/executables unchanged; 33 admitted blocks.')
