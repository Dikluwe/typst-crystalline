from pathlib import Path
import json,gzip,hashlib,datetime,shutil
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'working-baseline.json.gz'));scope=json.load(open(out/'scope.json'));new={
'01_core/src/entities/elements/figure_caption.rs':'00_nucleo/prompts/entities/elements/figure_caption.md',
'01_core/src/compiler/eval/bindings/field_access/content_methods.rs':'00_nucleo/prompts/compiler/eval/bindings/field_access/content_methods.md',
'01_core/src/compiler/eval/bindings/value_methods/element_where.rs':'00_nucleo/prompts/compiler/eval/bindings/value_methods/element_where.md',
'01_core/src/compiler/stdlib/loading/encoding.rs':'00_nucleo/prompts/compiler/stdlib/loading/encoding.md',
'02_shell/src/cli.rs':'00_nucleo/prompts/shell/cli.md'};sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
for source,owner in new.items():
 for p in [source,owner]:assert sha(p)==b['hashes'][p];b['files'][p]=Path(p).read_text()
 scope['sources'].append(source);scope['owners'][source]=owner
with gzip.open(out/'working-baseline.json.gz','wt') as f:json.dump(b,f)
(out/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
evidence='''
## Completar projeção da caption existente — 2026-10-17

Medição anterior à decisão: caption-shape-before.json e caption-after.json em
00_nucleo/diagnosticos/tekt-v16-figure-casts-2026-10-17 fixam fontes, UTC,
binários, HEAD/working tree de validation-result.json e pin
9f2b6e8715237cb086899a42873660fe744622e8. Projeção de caption já produz o
carrier, mas sua CLI imprime só func:figure.caption; body/getters/fields estão
ausentes. json.encode tem o mesmo defeito. Metadata de kind rejeita a função
de caption válida. O pin em model/figure.rs:510 declara caption Locatable e
em :511-594 declara position,separator,body e kind/supplement/numbering/counter
sintetizados públicos, com figure_location interno. Não sintetizar esses dados
nesta correção; a função de elemento já existe desde a aprovação P1473.
'''
contracts={
new['01_core/src/entities/elements/figure_caption.rs']:'''Sobrescrever apenas Element::get_field existente: body sempre conhecido;
position Some Top/Bottom retorna alinhamento vertical, separator Some retorna
Content original; Option None continua desconhecido, sem default assado. Não
mudar struct, métodos do trait, construtor, síntese ou phase; não reabrir os
não-objetivos de síntese/location desta entidade. É completar a reflexão dos
campos que o contrato aprovado já conserva. Content delega ao owner.
''',
new['01_core/src/compiler/eval/bindings/field_access/content_methods.rs']:'''Listar campos candidatos de FigureCaption na ordem position,separator,body;
expor somente getters conhecidos. content_elem_func retorna o callback nativo
native_figure_caption para FigureCaption, conservando nome interno existente;
não usar o callback genérico não chamável nem igualdade por nome como prova.
Não alterar política de Unset/síntese dos campos ausentes nesta fatia.
''',
new['01_core/src/compiler/eval/bindings/value_methods/element_where.rs']:'''Reconhecer native_figure_caption por fn_addr_eq na tabela de elemento: nome
caption; campos públicos position,separator,body,kind,supplement,numbering,counter;
locatable=true conforme a fonte pinada. Excluir figure_location interno. Isso
corrige a classificação da função em kind e where; não materializa síntese nem
índice de captions. FG1 ainda completa esses mecanismos e audita a totalidade
do catálogo; esta correção não certifica todos os callbacks do registry.
''',
new['01_core/src/compiler/stdlib/loading/encoding.rs']:'''Na projeção textual, mapear somente o nome interno FigureCaption para func
público caption. Usar os campos conhecidos fornecidos pelos owners de getters/
fields; Source transparente e snapshot Some autoritativo, inclusive vazio,
continuam exatos. Não materializar defaults/síntese de Figure ao serializar
uma FigureCaption solta. Não mudar formatos nem normalização numérica.
''',
new['02_shell/src/cli.rs']:'''Mapear FigureCaption para func público caption; projetar position,separator,
body conhecidos pelo getter na ordem declarada, recursivamente com
value_to_semantic. Não usar figure_caption_to_semantic, que sintetiza defaults
do objeto Figure e tem outro contrato. Não alterar a projeção legada de Figure
nem snapshots Some. JSON/YAML partilham a projeção; não duplicar validação L1.
''',
'00_nucleo/prompts/entities/content/fields.md':'''Delegar também FigureCaption ao get_field do owner, como Figure. Sem lógica
de montagem/validação no dispatcher. Conservar todos os outros braços/pins.
'''}
for owner,contract in contracts.items():
 p=Path(owner);p.write_text(p.read_text()+evidence+'\nResponsabilidade exclusiva e aceitação:\n\n'+contract+'\nCorreção interna/paridade em fluxo contínuo ADR-0127. Não há novo campo/trait/\nassinatura Rust pública, fase, modo ou default. L0 antes de testes/código, RED\nreal de getters/metadados/encoder/CLI, GREEN e corpus externo sem relaxar\nexpectativas de corpo/nome. Revalidar todos os gates da etapa sob 12 GiB, swap\nzero, jobs1. Resselo só dos owners após V15/V26. FG1/SY1 permanecem incompletos\nconforme follow-ups.md; nenhuma API pendente está autorizada por esta seção.\n')
dest=out/'candidate-3';dest.mkdir(exist_ok=False)
for p in [*out.glob('*resources.json'),*out.glob('validation-*.log'),out/'validation-result.json',out/'caption-after.json']:
 if p.is_file():shutil.copy(p,dest/p.name)
(out/'caption-shape-scope.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),added=new,source_sha256={p:sha(p) for p in new}),indent=2)+'\n')
