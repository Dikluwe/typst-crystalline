from pathlib import Path
import json,gzip,datetime,hashlib,re
out=Path(__file__).resolve().parents[1];scope=json.load(open(out/'scope.json'));p=Path(scope['sources'][0]);s=p.read_text();log=(out/'red.log').read_text();assert '15 failed' in log and 'existing native element missing' in log;assert json.load(open(out/'red-resources.json'))['exit']==101
s=s.replace('/// Fonte: typst-library e field_id de typst-macros::elem (a51e02804).','/// Fonte: field_id de typst-macros::elem; lotes auditados no pin 9f2b6e871.\n/// As entradas históricas preservadas ainda têm auditoria própria pendente.')
engine='''        if std::ptr::fn_addr_eq(native.call, native_op as fn(_, _, _, _, _, _) -> _) {
            return Some(("op", &["text", "limits"], false));
        }
'''
anchor='    if let crate::entities::func::FuncRepr::NativeWithEngine(native) = function.repr() {\n';assert s.count(anchor)==1;s=s.replace(anchor,anchor+engine)
rows=[]
for c in json.load(open(out/'batch-schema.json'))['cases']:
 if c['abi']==6:continue
 rows.append('    element!('+c['callback']+', '+json.dumps(c['name'])+', '+str(c['locatable']).lower()+', ['+', '.join(json.dumps(x) for x in c['fields'])+']);')
anchor='    element!(\n        native_math_equation,';assert s.count(anchor)==1;s=s.replace(anchor,'\n'.join(rows)+'\n'+anchor)
p.write_text(s)
(out/'implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),red_functional_failures=15,red_controls_passed=1),indent=2)+'\n')
