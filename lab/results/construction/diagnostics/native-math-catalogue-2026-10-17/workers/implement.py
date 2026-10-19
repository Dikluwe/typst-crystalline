from pathlib import Path
import json,datetime,hashlib,collections
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));cases=json.load(open(out/'batch-schema.json'))['cases'];log=(out/'red.log').read_text();assert '26 failed' in log and 'existing math element missing' in log;assert json.load(open(out/'red-resources.json'))['exit']==101
p=Path(scope['sources'][0]);s=p.read_text();rows=['    element!('+c['callback']+', '+json.dumps(c['name'])+', false, ['+', '.join(map(json.dumps,c['fields']))+']);' for c in cases];anchor='    element!(native_title,';assert s.count(anchor)==1;s=s.replace(anchor,'\n'.join(rows)+'\n'+anchor);p.write_text(s)
branches=[c for c in cases if c['new_bridge']];groups=collections.defaultdict(list)
for c in branches:groups[c['module']].append(c['actual_callback']+(' as '+c['callback'] if c['actual_callback']!=c['callback'] else ''))
p=Path('01_core/src/compiler/stdlib/structural/math.rs');s=p.read_text();anchor='use casts::vanilla_type_name_class;';assert s.count(anchor)==1;s=s.replace(anchor,anchor+'\n\n'+'\n'.join('pub(in crate::compiler) use '+m+'::{'+', '.join(ns)+'};' for m,ns in sorted(groups.items())));p.write_text(s)
for source,module in [('01_core/src/compiler/stdlib/structural/mod.rs','math'),('01_core/src/compiler/stdlib/mod.rs','structural')]:
 p=Path(source);s=p.read_text();block='pub(in crate::compiler) use '+module+'::{'+', '.join(c['callback'] for c in branches)+'};';anchor='#[cfg(test)]\nmod tests';assert anchor in s;s=s.replace(anchor,block+'\n\n'+anchor,1);p.write_text(s)
p=Path('01_core/src/compiler/stdlib/structural/math/limits.rs');s=p.read_text()
for name in ['native_math_limits','native_math_scripts']:
 old='pub(super) fn '+name;assert s.count(old)==1;s=s.replace(old,'pub(in crate::compiler) fn '+name)
p.write_text(s)
(out/'implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in scope['sources']},new_metadata=26,new_internal_bridges=len(branches),red_functional_failures=26,red_controls_passed=1),indent=2)+'\n')
