from pathlib import Path
import subprocess,json,datetime
base=Path.cwd()/'lab/results/v16-corrections-2026-10-19/figure-kind-implementation';commands=[['python3',str(base/'run_tests.py'),'--output',str(base/'agents/counters/counter-final-capability-green'),'--filter','counter_final_owned'],['python3',str(base/'run_tests.py'),'--output',str(base/'agents/counters/counter-final-existing-core'),'--filter','counter'],['python3',str(base/'build.py'),'--output',str(base/'counter-final-capability-build-1')]];results=[]
for cmd in commands:
 start=datetime.datetime.now(datetime.timezone.utc).isoformat();r=subprocess.run(cmd);results.append({'command':cmd,'utc_start':start,'utc_end':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':r.returncode});(base/'agents/counters/counter-final-green-commands.json').write_text(json.dumps(results,indent=2)+'\n')
 if r.returncode:break
raise SystemExit(int(len(results)!=len(commands) or any(r['exit'] for r in results)))
