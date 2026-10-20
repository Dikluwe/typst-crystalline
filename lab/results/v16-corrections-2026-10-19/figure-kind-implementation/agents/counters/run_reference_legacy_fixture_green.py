from pathlib import Path
import subprocess,json,datetime
root=Path.cwd()
base=root/'lab/results/v16-corrections-2026-10-19/figure-kind-implementation'
results=[]
for name,package,filter_ in [('core','typst-core','eval_ref_gera_content_ref'),('infra-default','typst-infra','p788_ref_happy_path_section_en'),('infra-explicit','typst-infra','p788_ref_suplemento_explicito')]:
 command=['python3',str(base/'run_tests.py'),'--output',str(base/'agents/counters'/('legacy-fixture-green-'+name)),'--filter',filter_,'--package',package]
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 run=subprocess.run(command)
 results.append({'command':command,'utc_start':start,'utc_end':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':run.returncode})
 (base/'agents/counters/legacy-fixture-green-commands.json').write_text(json.dumps(results,indent=2)+'\n')
raise SystemExit(int(any(x['exit'] for x in results)))
