"""Metadata successor: distinguish actual routes from negative-control probes."""
import importlib.util, json, sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('receipt',D/'p1335-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
raw=D/'p1335-aggregate.json'
data=json.loads(raw.read_text())
old=data.pop('paths')
data['probe_profile_coverage']=dict(probes=old['observed'],all_profiles_raw_equal=old['all_profiles_raw_equal'],not_all_profiles_raw_equal=old['not_all_profiles_raw_equal'],
    limits='4718 probes includes 4708 enumerated routes and 10 negative historical controls; not a count of unique real public routes or complete features.')
data['metadata_successor']=dict(at=r.now(),predecessor=dict(path=str(raw),sha256=r.sha(raw)),script_sha256=r.sha(__file__),
    change='Renamed the ambiguous paths block to probe_profile_coverage. No observations, counts, order comparisons or denominator changed; no product rerun.')
r.verify(r.state())
r.save('aggregate-r1',data)
