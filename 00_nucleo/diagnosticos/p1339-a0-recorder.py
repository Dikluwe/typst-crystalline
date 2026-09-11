import datetime
import hashlib
import json
import pathlib
import re
import subprocess

ROOT = pathlib.Path('/repos/Antigravity/typst-crystalline')
D = ROOT / '00_nucleo/diagnosticos'

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(p):
    with pathlib.Path(p).open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

commands = []
def run(argv):
    start = now()
    p = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
    commands.append(dict(argv=argv, cwd=str(ROOT), start=start, end=now(), exit=p.returncode, stdout=p.stdout, stderr=p.stderr))
    return p.stdout

start = now()
head = run(['git', 'rev-parse', 'HEAD']).strip()
branch = run(['git', 'symbolic-ref', '--short', 'HEAD']).strip()
status = run(['git', 'status', '--short'])
diffstat = run(['git', 'diff', 'HEAD', '--stat'])
modified = run(['git', 'diff', 'HEAD', '--name-only', '-z']).strip('\0').split('\0')
modified = {p: sha(ROOT/p) for p in modified if p}
closure = json.loads((D/'p1338-closure.json').read_text())
inventory = {p: sha(ROOT/p) for p in closure['state']['product_inventory']}
mismatches = {p: dict(expected=h, actual=inventory[p]) for p,h in closure['state']['product_inventory'].items() if inventory[p] != h}
binaries = {name: dict(path=closure['binaries'][name]['path'], expected_sha256=closure['binaries'][name]['sha256'], sha256=sha(closure['binaries'][name]['path'])) for name in ('candidate','vanilla')}
antecedents = {str(p.relative_to(ROOT)): sha(p) for p in [D/'p1335-matrix-normal.json', D/'p1338-closure.json', D/'p1338-review-final.json', D/'p1338-review-postclosure.json', D/'p1338-final-report.md']}
sources = ['01_core/src/compiler/stdlib/foundations/float.rs', '01_core/src/compiler/eval/bindings/field_access.rs', '01_core/src/compiler/eval/call_dispatch.rs', '01_core/src/compiler/eval/bindings/value_methods.rs', '01_core/src/compiler/stdlib/primitives_constructors/version.rs', '01_core/src/entities/version.rs']
owners = []
for source in sources:
    text = (ROOT/source).read_text()
    prompt = re.search(r'^//! @prompt (.+)$', text, re.M)[1]
    owners.append(dict(consumer=source, source_sha256=sha(ROOT/source), prompt=prompt, prompt_sha256=sha(ROOT/prompt), prompt_header=re.search(r'^//! @prompt-hash (.+)$',text,re.M)[1], reverse_metadata=[l for l in (ROOT/prompt).read_text().splitlines() if 'Hash do C' in l]))
run(['rg', '-n', 'float_type_field|is_float_instance_method|dispatch_float_method|Value::Type|Value::Angle|Value::Float|Value::Version|version_method|"with"|"where"|"at"|pub fn at', *sources])
run(['rg', '-n', 'Angle|"deg"|"rad"', '01_core/src/compiler/eval/bindings', '01_core/src/compiler/stdlib/foundations'])
run(['crystalline-lint', '--checks', 'v5,v15,v26', '--fail-on', 'warning', '.'])
assert commands[-1]['exit'] == 0
status_end = run(['git', 'status', '--short'])
assert status_end == status
okay = not mismatches and not modified and all(b['sha256']==b['expected_sha256'] for b in binaries.values()) and head == '2f42d64253547734564513a1159ee6b584c1c4b4'
data = dict(start=start,end=now(),head=head,branch=branch,status=status,status_end=status_end,diff_stat=diffstat,modified_tracked=modified,product_inventory=inventory,baseline_mismatches=mismatches,baseline_matches_p1338=okay,baseline_explanation='P1338 certified product bytes preserved exactly; original dirty HEAD d31047d7 is now organized as P1319-P1338 commits ending at current HEAD. The sole initial untracked file is the user-supplied P1339 step.',binaries=binaries,antecedents=antecedents,owners=owners,commands=commands,step=dict(path='00_nucleo/materialization/typst-passo-1339.md',sha256=sha(ROOT/'00_nucleo/materialization/typst-passo-1339.md')),script_sha256=sha(__file__),regime='Protocolo completo solicitado; ambiente compartilhado sem atestação de isolamento; nenhum contrato, selo ou candidato criado.')
payload=json.dumps(data,ensure_ascii=True,indent=2)+'\n'
patch='*** Begin Patch\n*** Add File: '+str(D/'p1339-a0.json')+'\n'+''.join('+'+line+'\n' for line in payload.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,cwd=ROOT,check=True)
print(json.dumps(dict(baseline_matches_p1338=okay,product_files=len(inventory),head=head,a0_sha256=sha(D/'p1339-a0.json'))))
