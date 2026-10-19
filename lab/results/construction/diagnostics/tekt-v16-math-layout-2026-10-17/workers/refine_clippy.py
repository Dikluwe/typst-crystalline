from pathlib import Path
import datetime, hashlib, json, shutil, subprocess

root = Path.cwd()
out = Path(__file__).resolve().parents[1]
source = '01_core/src/compiler/math/layout/source_transparency_tests.rs'
owner = '00_nucleo/prompts/compiler/math/layout/source-transparency-tests.md'
assert json.load(open(out / 'validation-resources.json'))['exit'] == 1
details = json.load(open(out / 'strict-details.json'))
assert len(details['diagnostics']) == 1
assert details['diagnostics'][0]['code']['code'] == 'clippy::cloned_ref_to_slice_refs'
archive = out / 'attempt-clippy'
archive.mkdir(exist_ok=False)
for p in list(out.iterdir()):
    if p.is_file() and (p.name.startswith(('validation-', 'source-after-', 'probe-green', 'strict-')) or p.name == 'guard-result.json'):
        shutil.move(str(p), archive / p.name)
old_source = Path(source).read_text()
old_owner = Path(owner).read_text()
old = '&[container.clone()]'
new = 'std::slice::from_ref(&container)'
assert old_source.count(old) == 1
note = '''

### Empréstimo no controle de origem — 2026-10-17

Clippy estrito mede `cloned_ref_to_slice_refs` no controle de flatten,
source_transparency_tests.rs:378; recibo e fontes exatas do candidato em
attempt-clippy/ e clippy-refinement.json. O contêiner é apenas emprestado
para a entrada de um item. Prescrever `std::slice::from_ref(&container)`
em vez da fatia temporária com clone, mantendo os mesmos dados, spans,
asserções, observação e contagem de testes. Não introduzir supressão ou API.
'''
record = dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), source=source, owner=owner, old=old, new=new, before_source=old_source, before_owner=old_owner, after_owner=old_owner + note, head=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(), status=subprocess.check_output(['git', 'status', '--porcelain=v1'], text=True), diffstat=subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True))
Path(owner).write_text(record['after_owner'])
record['l0_written_before_code'] = Path(source).read_text() == old_source
assert record['l0_written_before_code']
(out / 'clippy-refinement.json').write_text(json.dumps(record, indent=2) + '\n')
Path(source).write_text(old_source.replace(old, new))
subprocess.run(['python3', str(out / 'workers/seal_owned.py'), source], check=True)
subprocess.run(['python3', str(out / 'workers/guard.py')], check=True)
print('CLIPPY TEST REFINEMENT PASS', flush=True)
