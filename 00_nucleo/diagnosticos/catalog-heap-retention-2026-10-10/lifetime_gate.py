"""Diagnostic resource gate; language acceptance remains in inline/CLI controls."""
from pathlib import Path
import datetime,gzip,hashlib,json,sys
D=Path(__file__).resolve().parent
mode=sys.argv[1]
name={'red':'phase-catalog.json.gz','green':'phase-catalog-materialized.json.gz'}[mode]
data=json.load(gzip.open(D/name,'rt'))
rows=[]
for sample in data['samples']:
 if sample['mode']!='new' or sample['trim']!='no-trim':continue
 phases={r['phase']:r for r in sample['phases']}
 before=phases['math_cloned']['tracked_requested_live']
 retained=phases['catalog_alive_temporaries_dropped']['tracked_requested_live']
 # Fixed audited fixture: project metadata follows. Allow16KiB of diagnostic/slack.
 rows.append({'rep':sample['rep'],'before_project_live':before,'retained_catalog_live':retained,
              'limit':retained+16384,'passed':before<=retained+16384})
receipt={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':data['head'],
 'working_tree':data['working_tree'],'git_status':data['git_status'],'git_diff_stat':data['git_diff_stat'],
 'source_receipt':name,'source_receipt_sha256':hashlib.sha256((D/name).read_bytes()).hexdigest(),
 'driver_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'mode':mode,'rows':rows,
 'passed':all(r['passed'] for r in rows)}
(D/('lifetime-gate-'+mode+'.json')).write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt['rows'],indent=2));print('passed',receipt['passed'])
if not receipt['passed']:sys.exit(1)
