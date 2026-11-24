import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

exp = Path(__file__).resolve().parent
out = exp.parents[2] / 'lab/results/construction/diagnostics' / exp.name
previous = out.parent / 'page-three-position-remap-2026-11-21'
source = Path(sys.argv[1])
rust = source / '01_core/src/compiler/layout/page_running_requests.rs'
prompt = source / '00_nucleo/prompts/core/compiler/layout/page_running_requests.md'
manifest = {(p.removeprefix('./')): h for h,p in
            (row.split(maxsplit=1) for row in (previous/'candidate-source.sha256').read_text().splitlines())}

def replace_once(s, old, new):
    assert s.count(old) == 1, old
    return s.replace(old, new, 1)

for path, digest in manifest.items():
    assert hashlib.sha256((source/path).read_bytes()).hexdigest() == digest, path
pairs = [('trace.rs', rust), ('trace.md', prompt)]
for name, path in pairs:
    (out/f'before-{name}').write_bytes(path.read_bytes())
with prompt.open('a') as f:
    f.write((exp/'contract.md').read_text())
seal = subprocess.run(['crystalline-lint','--fix-hashes','--dry-run',str(source)],
                      stdout=subprocess.PIPE, stderr=subprocess.STDOUT,text=True)
(out/'dry-seal.log').write_text(seal.stdout)
row = next(row for row in seal.stdout.splitlines() if f'{rust} prompt={prompt.relative_to(source)} ' in row)
header, code = re.search(r'hash-a=(\w+) hash-b=(\w+)', row).groups()
prompt.write_text(re.sub(r'Hash do Código: \w+',f'Hash do Código: {code}',prompt.read_text(),count=1))
s = re.sub(r'@prompt-hash \w+',f'@prompt-hash {header}',rust.read_text(),count=1)
s = replace_once(s, '            .filter_map(|(index, event)| (event.page.get() >= 3).then_some(index))',
'''            .filter_map(|(index, event)| {
                if !Arc::ptr_eq(&event.plan, &self.data.source) || event.deferred_anchor {
                    return Some(index);
                }
                let Some(node) = event.plan.node(event.packet) else {
                    return Some(index);
                };
                node.location.iter().chain(&node.auxiliary_locations)
                    .any(|&location| self.data.positions.position_of(location)
                        .is_some_and(|position| position.page.get() >= 3))
                    .then_some(index)
            })''')
s = replace_once(s, '        let mut rebound = Vec::with_capacity(new_indices.len());','        let mut matched = 0;')
start = s.index('            rebound.push((new_index, Admission {')
end = s.index('            }));',start)+len('            }));')
s = s[:start] + '            matched += 1;' + s[end:]
s = replace_once(s, '''        for (index, admission) in rebound {
            self.data.admissions[index] = admission;
            receipt[11] += 1;
        }''','        receipt[11] = matched;')
s = replace_once(s, '        let mut new = trace(2, current, 48.0);', '''        let mut new = trace(2, current, 48.0);
        new.data.admissions.push(Admission {
            plan: new.data.source.clone(), packet: PacketId(0), phase: PlanPhase::Root,
            page: NonZeroUsize::new(3).unwrap(), section: TraceSection::Body,
            causal_order: vec![1], deferred_anchor: false,
        });''')
s = replace_once(s, '        assert_eq!(event.causal_order, vec![2]);', '''        assert_eq!(event.causal_order, vec![2]);
        assert_eq!(new.data.admissions[1].packet, PacketId(0));
        assert_eq!(new.data.admissions[1].causal_order, vec![1]);''')
rust.write_text(s)
modified = {p: h for p in manifest if (h := hashlib.sha256((source/p).read_bytes()).hexdigest()) != manifest[p]}
assert set(modified) == {str(p.relative_to(source)) for _,p in pairs},modified
(out/'freeze.json').write_text(json.dumps(modified,indent=2)+'\n')
patch = []
for name, path in pairs:
    frozen = out/f'candidate-{name}'
    frozen.write_bytes(path.read_bytes())
    rel = str(path.relative_to(source))
    diff = subprocess.run(['diff','-u','--label','a/'+rel,'--label','b/'+rel,
                           str(out/f'before-{name}'),str(frozen)],capture_output=True)
    assert diff.returncode == 1
    patch.append(diff.stdout)
(out/'candidate.patch').write_bytes(b''.join(patch))
