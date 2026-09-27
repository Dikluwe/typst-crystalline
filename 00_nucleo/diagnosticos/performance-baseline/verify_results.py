"""Validate coverage, admission identities and additive phase arithmetic on raw results."""
import collections, json, pathlib
import run as c
OUT=c.OUT
m=json.loads((OUT/'micro-summary.json').read_text());e=json.loads((OUT/'e2e-summary.json').read_text());p=json.loads((OUT/'phase-summary.json').read_text());expected=json.loads((OUT/'build-state.json').read_text())
c.identity_check();current=c.snapshot('final');c.require_built_state(current)
assert current['head']==expected['head']
by_target=collections.defaultdict(list)
for name,row in m.items():
    assert row['three_rounds'] and {r['round'] for r in row['rounds']}=={'r1','r2','r3'},name
    by_target[row['target']].append((name,row))
assert set(by_target)==set(c.TARGETS)
for target in ['pipeline','content']:
    plan=json.loads((OUT/(target+'-filter-plan.json')).read_text())
    assert {name for name,row in by_target[target+'_bench']}==set(plan['cases']),target+' inventory incomplete'
    assert plan['executable_sha256']==json.loads((OUT/'artifacts.json').read_text())[target+'_bench']['sha256']
for key,row in e.items():
    count=36 if key.startswith('plain/') else 18
    assert set(row['engine'])=={'tekt','vanilla'}
    assert row['paired_difference_ms']['n']==count
    for v in row['engine'].values():assert v['wall_ms']['n']==count and set(v['round_means_ms'])=={'1','2','3'}
assert len(e)==10 and len(p)==5
for row in p.values():assert abs(sum(row['mean_phase_delta_ms'].values())-row['mean_wall_delta_ms'])<1e-4
for row in json.loads((OUT/'output-validation.json').read_text())['comparisons']:
    assert all(row[k] for k in ['pages_match','geometry_match','normalized_text_match','text_font_control_match'])
used=[]
for path in (OUT/'receipts').glob('*.json'):
    row=json.loads(path.read_text())
    if not row.get('valid') or not path.name.startswith(('micro-','e2e-')):continue
    assert row['fingerprint_before']==row['fingerprint_after']==expected['sha256'],path.name
    assert not row['contamination']
    extra=row.get('additional_driver_input')
    if extra:
        assert extra['sha256_before']==extra['sha256_after']
        archive=OUT/'driver-revisions'/(extra['sha256_before']+'.py');assert c.sha(archive)==extra['sha256_before']
    used.append(path.name)
summary={'utc':c.utc(),'head':current['head'],'consumer_input_fingerprint':current['sha256'],'all_executable_identities_unchanged':True,'coverage_complete':True,'micro_cases':len(m),'targets':{target:{'cases':len(rows),'adr0115_pass':sum(r['adr0115_mean_median_pass'] for _,r in rows),'strict_range_pass':sum(r['strict_range_pass'] for _,r in rows)} for target,rows in sorted(by_target.items())},'cli_populations':len(e),'phase_fixtures':len(p),'used_receipts':sorted(used),'output_controls_pass':True,'scope':'Existing source/benchmark files unchanged; diagnostic driver revisions recorded separately; no global parity or sanitization claim.'}
c.save(pathlib.Path('completion-audit.json'),summary);print(json.dumps({k:v for k,v in summary.items() if k!='used_receipts'},indent=2))
