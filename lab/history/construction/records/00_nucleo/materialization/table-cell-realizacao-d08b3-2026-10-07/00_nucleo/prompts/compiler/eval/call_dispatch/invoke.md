# L0 — compiler/eval/call_dispatch/invoke
Hash do Código: ebccad03

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/invoke.rs`. No HEAD `1492976302212e007282875aef066f84a3cf13aa` e árvore limpa, `01_core/src/compiler/eval/call_dispatch/call.rs:638-827` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-call-core-2026-09-30-before.json.gz`.

## Contrato

Resolver o callee preservado, avaliar argumentos e executar o match fechado de Func/Type/Plugin/Content com spans e traces atuais. Preservar `00_nucleo/prompts/compiler/eval/call_dispatch.md` e o owner `call.md`, sem mudar assinatura pública, defaults, fase do pipeline ou compatibilidade. A extração fica em L1, com despacho estático.

## Aceitação

Preservar a suíte L1 e os testes de call_dispatch; `crystalline-lint .` valida ownership/hashes. Não inferir ganho de desempenho.

## Origem de falhas na realização de conteúdo chamado — 2026-10-06

Medição: controls-first.json mostra maximum show rule depth exceeded sem fonte causal; invoke.rs:145 aplica a intercepção depois do trace de apply_func e perde o span da construção. Ao propagar erros de intercept_content, conservar spans conhecidos e ancorar somente os detached no call.span real. Não reconstruir origem pelo texto do diagnóstico nem sobrescrever origens internas. Mantém assinatura/fase e o trace de apply_func.

## D06 — origem causal de receita diferida matemática

Medição after.json.gz show-clone-body tem erro/hints corretos mas sem span da
inserção; invoke.rs:145–155 só ancora erros eager. Ao capturar ShowRecipe de
conteúdo que contém inserção Mathy externa, guardar call.span no Styles privado
eval.math.origin usando bits de Span::into_raw (Value::Int), sem alterar o
constructor/valor cru quando não há receita. Contexto existe somente na receita.
A realização reconstrói com Span::from_raw e ancora somente detached; mantém
spans conhecidos, Source real e ordem de hints/traces. O escopo de capture é
restaurado antes de retornar, inclusive em erro. Não derivar span da mensagem
ou recriar Source pelo texto. Nenhum contrato público ou fase além do gate D06.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Result Content::Table/Content::TableCell de constructor ou closure não é inserção por si só: conservar valor cru e não assar recipe global sobre valor guardado em let. Demais resultados conservam intercepção existente. Spans específicos e traces de argumento permanecem; a inserção de Table/Cell pertence a markup/join/produto efetivamente emitido, não ao nome da função.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.
