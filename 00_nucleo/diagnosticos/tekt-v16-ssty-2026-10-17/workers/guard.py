from pathlib import Path
import json,datetime,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];r=json.load(open(out/(phase+'-probe-result.json')));ok=r['structural_guarded_sites']==2;record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),phase=phase,expected_guarded_sites=2,observed_guarded_sites=r['structural_guarded_sites'],pass_guard=ok,source_hashes=r['source_hashes']);(out/(phase+'-guard-result.json')).write_text(json.dumps(record,indent=2)+'\n');print('PASS' if ok else 'FAIL: future MathSize accepted without owner review');sys.exit(0 if ok else 1)
