from pathlib import Path
import json,subprocess,re,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');scope=json.load(open(out/'scope.json'));walk=scope['sources'][0];outline='01_core/src/compiler/introspect/outline_entries.rs';registries=['01_core/src/entities/counter_registry.rs','01_core/src/entities/state_registry.rs']
def run(label,cmd,expected):
 print(label,flush=True)
 with (out/('order-refinement-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode==expected,(label,r.returncode)
assert '10 passed; 4 failed;' in (out/'order-refinement-functional-red.log').read_text()
log=(out/'order-refinement-contract-red.log').read_text();assert re.findall(r'error\[([^]]+)\]',log)==['E0599']*6
assert 'CounterRegistry' in log and 'StateRegistry' in log
script=(out/'workers/apply_order_refinement.py').read_text();exec(compile(script[script.index("method=\'\'\'"):],str(out/'workers/apply_order_refinement.py'),'exec'))
