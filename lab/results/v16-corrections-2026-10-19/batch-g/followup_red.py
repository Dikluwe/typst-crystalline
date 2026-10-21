"""Seal the added invariant owner before actual depth/footer regression RED."""
from pathlib import Path
import datetime,json,subprocess
P=Path(__file__).resolve().parent;out=P/'followup-red';out.mkdir(exist_ok=False);records=[]
def run(name,argv):
 start=datetime.datetime.now(datetime.timezone.utc).isoformat();print(start,'START',name,flush=True)
 with (out/(name+'.log')).open('w') as f:r=subprocess.run(argv,stdout=f,stderr=subprocess.STDOUT)
 records.append({'name':name,'argv':argv,'utc_start':start,'utc_end':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':r.returncode});(out/'commands.json').write_text(json.dumps(records,indent=2)+'\n');print(name,r.returncode,flush=True)
 if r.returncode:print((out/(name+'.log')).read_text()[-7000:],flush=True);raise SystemExit(r.returncode)
run('seal',['python3',str(P/'reseal.py'),'--output',str(P/'reseal-followup-before-implementation')])
args=['python3',str(P/'run_tests.py'),'--output','focused-followup-red-core','--package','typst-core','--expected-total','20']
for name in ['carrier_debug_transparency_reaches_the_same_visible_depth_boundary','carrier_hash_compatibility_covers_selected_old_and_new_content_families','debug_retains_visible_max_depth_and_iterates_carriers_without_consuming_originals']:args+=['--expected-fail',name]
run('core-depth',args)
run('infra-footer',['python3',str(P/'run_tests.py'),'--output','focused-followup-red-infra','--package','typst-infra','--expected-total','32','--expected-fail','batch_g_html_raw_footer_projection_has_no_unemitted_warning'])
