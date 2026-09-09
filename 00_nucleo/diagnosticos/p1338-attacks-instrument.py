"""Create P1338 runner by explicit reuse; never execute preparation/Cargo here."""
import ast, runpy
from pathlib import Path
D=Path('/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos')
prior=D/'p1337-attacks-runner.py'; x=runpy.run_path(str(prior))
assert x['sha'](prior.read_bytes())=='29e3abd140c830ed44886792f3148bcd325abbed84dcb793fd21b7c0e074a62c'
body=prior.read_text().replace('P1337','P1338').replace('p1337','p1338')
start=body.index(" marker='        other @ ('")
end=body.index(" record={'at':utc()",start)
replacement=''' marker='        Value::Array(arr) => match field {'
 assert candidate.count(marker)==1
 insert='        Value::Array(_) if !matches!(field, "len" | "first" | "last") => Err(vec![SourceDiagnostic::error(\\n            span,\\n            "array field access forbidden".to_string(),\\n        )]),\\n'
 cases=[('C',candidate),('M1',candidate.replace(marker,insert+marker,1))]
 needle='            | Value::Array(_)\\n'; assert candidate.count(needle)==1
 cases.append(('M2',candidate.replace(needle,'',1)))
 needle='            "first" => Ok(arr.first().cloned().unwrap_or(Value::None)),\\n'; assert candidate.count(needle)==1
 cases.append(('M3',candidate.replace(needle,needle.replace('arr.first()','arr.last()'),1)))
 needle='            | Value::Array(_)\\n'
 cases.append(('M4',candidate.replace(needle,needle+'            | Value::Length(_)\\n',1)))
'''
body=body[:start]+replacement+body[end:]
body=body.replace("'complete':len(runs)==6", "'complete':len(runs)==5")
ast.parse(body)
x['publish'](D/'p1338-attacks-runner.py',body)
print(x['sha']((D/'p1338-attacks-runner.py').read_bytes()))
