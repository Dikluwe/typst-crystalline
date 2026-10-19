from pathlib import Path
import json,gzip,hashlib,datetime,shutil,subprocess
out=Path('00_nucleo/diagnosticos/clippy-enums-2026-10-16');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=datetime.datetime.now(datetime.timezone.utc).isoformat()
assert json.loads((out/'validation-result.json').read_text())['exit']==1
archive=out/'first-validation';archive.mkdir()
for pattern in ['validation*','clippy-inventory.json','code-guard.json']:
 for p in out.glob(pattern):
  if p.is_file():shutil.move(str(p),archive/p.name)
before=json.load(gzip.open(out/'before.json.gz'));scope=json.loads((out/'scope.json').read_text())
source='01_core/src/compiler/eval/repr/value.rs';owner='00_nucleo/prompts/compiler/eval/repr/value.md'
for p in [source,owner]:
 assert sha(p)==before['all_source_hashes'][p],p
 before['files'][p]=Path(p).read_text()
scope['sources'].append(source);scope['owners'].append(owner)
with gzip.open(out/'before.json.gz','wt') as f:json.dump(before,f)
(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
style_owner=Path('00_nucleo/prompts/entities/style.md');s=style_owner.read_text();s+='''
## Adaptador inerente — previsão corrigida após medição 2026-10-16

### Medição anterior à decisão

A primeira validação deste lote, após ratificação e implementação, terminou no gate do inventário Clippy: first-validation/validation-clippy.log em 00_nucleo/diagnosticos/clippy-enums-2026-10-16 contém should_implement_trait em Styles::from_iter mesmo com FromIterator<Style> implementado. Os quatro testes approved_traits passaram, incluindo collect::<Styles> vazio e a dobra ordenada de Weight. traits-green-result.json registra comando, UTC, cgroup e retorno; first-validation/validation-details.json preserva hashes, HEAD, working tree e diffstat. Portanto a previsão anterior de que apenas implementar o trait eliminaria este aviso foi refutada na ferramenta instalada (Clippy/Rust 1.92). Não é ausência do trait nem evidência de ambiguidade na linguagem Typst.

### Decisão e aceitação

Manter o adaptador inerente e sua assinatura exatamente como ratificados. Corrigir somente a previsão anterior de ausência de expect: prescrever #[expect(clippy::should_implement_trait, reason="S005: adapter inerente de compatibilidade; FromIterator<Style> implementado e validado por collect")] no adaptador. O corpo continua delegando integralmente ao trait; nenhum fold duplicado ou supressão no trait/impl/módulo/crate. O atributo deve ser cumprido com unfulfilled_lint_expectations como erro; collect e chamadas inerentes continuam validados. É adendo interno de documentação/metadados em fluxo contínuo ADR-0127; nenhum contrato público ratificado é alterado. A cláusula anterior sem expect e a previsão de eliminação automática ficam substituídas apenas por esta medição/decisão. L0 primeiro e resselo restrito; suíte completa e gates sob 12 GiB, swap zero e um job.
''';style_owner.write_text(s)
p=Path(owner);p.write_text(p.read_text()+'''
## Uso do Display decimal — correção interna medida 2026-10-16

### Medição anterior à decisão

first-validation/validation-clippy.log em 00_nucleo/diagnosticos/clippy-enums-2026-10-16 registra to_string_in_format_args em repr/value.rs no braço Value::Decimal após a implementação Display ratificada. O texto vigente é format!("decimal(\\\"{}\\\")", d.to_string()); o Display padrão do novo contrato delega ao mesmo InnerDecimal. Hashes, HEAD, working tree, UTC e diffstat estão em first-validation/validation-details.json. É construção de string interna, não nova medição de paridade.

### Responsabilidade e aceitação

Passar d diretamente ao format!, preservando decimal("...") e o texto decimal/escala/zeros/sinal. Não alterar nenhuma outra variante, regra de quoting, assinatura ou fase. Correção mecânica em fluxo contínuo ADR-0127, sem supressão ou contrato novo; preservar todos os testes de repr e rodar suíte, Clippy, build, rustdoc e linhagem com teto de 12 GiB. O diagnóstico anterior constitui RED do lint; GREEN exige ausência de novos avisos e saídas existentes preservadas.
''')
record=dict(utc=now,owners_before_code={str(style_owner):sha(style_owner),owner:sha(owner)},before={},edits=[],unformatted={})
def edit(p,old,new):
 t=Path(p).read_text();record['before'][p]=t;assert t.count(old)==1;Path(p).write_text(t.replace(old,new,1));record['edits'].append(dict(path=p,old=old,new=new));record['unformatted'][p]=Path(p).read_text()
edit('01_core/src/entities/style.rs','    pub fn from_iter<I: IntoIterator<Item = Style>>(iter: I) -> Self {','    #[expect(\n        clippy::should_implement_trait,\n        reason = "S005: adapter inerente de compatibilidade; FromIterator<Style> implementado e validado por collect"\n    )]\n    pub fn from_iter<I: IntoIterator<Item = Style>>(iter: I) -> Self {')
edit(source,'Value::Decimal(d) => format!("decimal(\\\"{}\\\")", d.to_string()),','Value::Decimal(d) => format!("decimal(\\\"{}\\\")", d),')
(out/'traits-followup-materialization.json').write_text(json.dumps(record,indent=2)+'\n')
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*record['before']],check=True)
print('First validation archived; two followups prescribed before edits; contracts preserved.')
