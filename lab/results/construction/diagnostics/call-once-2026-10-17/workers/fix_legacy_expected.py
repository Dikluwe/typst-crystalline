from pathlib import Path
import json,gzip,shutil,datetime
out=Path(__file__).resolve().parents[1];v=json.load(open(out/'workspace/workspace-result.json'));assert v['exit']!=0 and v['failed']==1;scope=json.load(open(out/'scope.json'));a=out/'candidate-5';a.mkdir()
for p in list(out.glob('validation-*'))+list(out.glob('*-resources.json')):
 if p.is_file() and p.name not in ['red-resources.json','before-probe-resources.json','before-math-probe-resources.json','value-fixtures-resources.json']:shutil.copy(p,a/p.name)
shutil.move(str(out/'workspace'),str(a/'workspace'))
with gzip.open(a/'sources.json.gz','wt') as f:json.dump({p:Path(p).read_text() for p in scope['sources']+list(scope['owners'].values())},f)
p=Path('00_nucleo/prompts/compiler/eval/bindings/field_access.md');p.write_text(p.read_text()+'''\n\n### Atualização do controle Type::Array para o pin vigente

candidate-5/workspace/workspace.log registra uma falha, no controle antigo
p1338_preserve_other_categories_diagnostics: #array.nope ainda exige aspas
duplas e a âncora array.nope inteira. O pin 9f2b6e871 na sonda missing/array
emite type array does not contain field `__missing`, no field-only span, e a
fonte code.rs:347–368 usa field.span. Os REDs de chamadas e a comparação
bilateral integral fixaram esse contrato antes do código. Atualizar somente
a linha #array.nope daquele teste para mensagem com backticks e âncora nope.
Manter os casos length/boolean/none/auto/integer/string, sua severidade, spans,
hints, traces e perfis anteriores, sem retirar assertivas ou ignorar o teste.
''')
p=Path('01_core/src/compiler/eval/bindings/field_access.rs');s=p.read_text();a='("#array.nope", "array.nope", "type array does not contain field \\\"nope\\\"")';assert s.count(a)==1;s=s.replace(a,'("#array.nope", "nope", "type array does not contain field `nope`")');p.write_text(s)
(out/'legacy-expected-fix.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),changed_only_fixture_for_corrected_type_array_diagnostic=True),indent=2)+'\n')
