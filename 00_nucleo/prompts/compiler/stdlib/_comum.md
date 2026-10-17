# L0 — stdlib/_comum
Hash do Código: 5a357d7b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:a922bbaa9c3b4c8435708af8e68b4557801f93ba09188768d48c711ba35a0c15
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:79283c9047419768139e0227b9ed7e34146a85706ed47731655736545c95306b
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:32890de1247262b54fa39d7ae1d640a068774a28cf1fa1c8890d0508d999af51
- 00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml sha256:19eb838b3e56711fa151d416bd24394a0a6f5c8286bb9ab2d0dd9bc542b4aee7

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/mod.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/mod.rs:212` (pubsuper) fn errmsg: impl Into<String>) -> SourceResult<Value> {); `01_core/src/compiler/stdlib/mod.rs:221` (pubsuper) fn expect_no_named) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own the stdlib façade: module declarations, explicit reexports, local err/expect_no_named helpers and cfg(test) test-module composition. Preserve established Rust public/internal paths and their specified visibility, with discrepancies explicitly retained below; do not create wrappers, duplicate namespace factories or move algorithms into the hub.

`err` produces one Error diagnostic with a detached span. `expect_no_named` rejects the first unknown named key in insertion order with the existing literal `argumento nomeado inesperado: 'KEY'` and a detached span. These helpers do not replace owner-specific origin-sensitive validation.

The actual native ABIs are the current Func native/Engine interfaces; the historical shortened signature is not an API specification. Per-native casts, diagnostics, return values and World use belong to their owners. Numeric helpers and promotion rules are not implemented here.

Registry composition belongs to `00_nucleo/prompts/compiler/stdlib/registry/mod.md`; eval keeps its own façade/call dispatch. Preserve the direct linkage of make_stdlib_with_features, calc_abs, format factory, collection call-span metadata, float/int/version dispatch, array-from-bytes, codec encoders, outline entry operations, spot construction and all existing native reexports. Do not globally register aliases from this façade.

Constructor hubs `00_nucleo/prompts/compiler/stdlib/foundations.md`, `00_nucleo/prompts/compiler/stdlib/text.md`, `00_nucleo/prompts/compiler/stdlib/structural.md` and `00_nucleo/prompts/compiler/stdlib/primitives-constructors.md` own only their composition. Production behavior belongs to the respective descendants; the atomized test suite is owned by `00_nucleo/prompts/compiler/stdlib/tests/mod.md` and its test owners.

The contract requires the native_flush façade reexport to be pub(crate), not an externally public API. The current public reexport conflicts with that requirement. Keep this obligation explicit; the observed code does not supersede it. No Rust visibility change is made or authorized during sanitation, and the discrepancy remains for a separate contract/code resolution.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

## D05 — equation.where, correção interna de paridade

### Medição antes da decisão

Coleta equation-where-2026-10-06/before.json.gz registra pin
9f2b6e8715237cb086899a42873660fe744622e8, binários SHA-256, UTC, HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b e diff HEAD --stat/estado exato.
Tekt rejeita equation.where como função não-elemento; vanilla aceita catálogo
público, filtros block/label/body/numbering/alt, alias, composição e show.
Vanilla rejeita campos size/missing, posição, closure e aplicação parcial.
Fonte upstream: foundations/func.rs:433–462 valida field_id e mantém grupo;
math/equation.rs:58–170 separa campos públicos dos internos.
Tekt element_where.rs:63–490 omite equation; selector_matching.rs:269–310
omite Equation no matching nativo. O getter block/body e snapshot já existem.

### Decisão e aceitação

A fachada reexporta o bridge native_math_equation apenas com
pub(in crate::compiler) para as tabelas de eval. Preservar todos os reexports
públicos, namespaces, signatures e fábricas existentes. A façade permanece
composição sem lógica. Nenhuma API pública nova ou flag.

Fluxo contínuo ADR-0127: correção de tabela interna, sem novo contrato público
ou fase. RED→GREEN dos consumers e revalidação bilateral dos controles;
preservar D04 e não realizar math bare/D06 ou morfologia de matriz/D07.

## D07 — correção de morfologia de matriz, fluxo ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado;
00_nucleo/diagnosticos/matrix-morfologia-2026-10-06/before.json.gz guarda
UTC/diff/fontes/binários e casos contra o pin 9f2b6e8715237cb086899a42873660fe744622e8.
Mat raw/query sai como {func:math.matrix} sem rows; getter/métodos perdem campos
e func-equality retorna false. Fonte upstream math/matrix.rs:91–221 declara
propriedades e rows. Tekt MathMatrixElem conserva rows/explicit em math_matrix.rs:45–52,
mas Element.get_field é default; content_methods.rs:151–191 não enumera mat;
cli.rs:1154–1348 cai no fallback body. É morfologia/semântica da linguagem,
não exigência de estrutura Rust ou bytes PDF; nenhuma intenção foi inferida.

### Responsabilidade exclusiva

Reexport privado pub(in crate::compiler) de structural::native_math_mat. A façade não valida argumentos ou identifica por strings; disponibiliza apenas a mesma nativa existente ao getter func.

### Aceitação

RED→GREEN e coleta bilateral de fields/has/at/func, JSON/query, padding e
presença; build/lint e regressões D05/D06/query. Gate contínuo: contratos
públicos de tipos/traits/assinaturas e fase ficam iguais. Não certificar
mat selectors, render, Styled geral ou outros elementos não medidos.

## Imports de produção e teste — 2026-10-13

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. 01_core/src/compiler/stdlib/mod.rs:123; 01_core/src/compiler/stdlib/structural/math/matrix.rs:81,163,406 registra a composição e os usos existentes. O inventário Clippy distingue imports sem uso na compilação produtiva; os usos de teste foram conferidos separadamente. Fontes, L0, UTC, status e diffstat estão em `00_nucleo/diagnosticos/imports-ci-2026-10-13/before.json.gz`. Não é medição de paridade ou desempenho.

### Decisão e aceitação

Remover somente o alias crate-interno math_matrix_augment do hub stdlib; seus usos estão confinados ao owner matrix. A obrigação anterior de preservar os paths internos deixa de abranger esse alias sem consumidores. Preservar todas as nativas públicas, MathMatrixOptions, apply_math_matrix_option e a implementação da função no filho matrix.

Correção de composição interna, sem mudança de assinatura pública, defaults, fase ou comportamento da linguagem. Preservar cenários/assertivas, executar a suíte core, build, Clippy e linhagem. Não mascarar avisos com allow/expect.


## Identidade matemática nativa — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado; pin `9f2b6e8715237cb086899a42873660fe744622e8`.
`00_nucleo/diagnosticos/native-math-catalogue-2026-10-17/` fixa o estado
exato em before.json.gz: UTC, diff HEAD --stat e hashes de todos os consumers,
L0 e configuração protegidos. before-measurement.json.gz é reutilização
verificada da matriz final da etapa anterior: fontes e binário Tekt têm
identidade de hash com o baseline atual (baseline-reuse.json), não uma nova
execução. A matriz cobre os oito perfis de features e 3956 casos de funções;
26 rotas math ainda são recusadas em where/kind pelo Tekt e aceitas pelo pin.
before-batch.json.gz mede 227 casos detalhados de campos, exclusões, alias
parcial e counter, com comandos/binários/saídas e seus hashes.

batch-schema.json conserva blocos e SHA-256 das fontes primárias do pin,
file:line e declarações de ABI do Tekt; typst-macros/src/elem.rs:410–421
exclui internal/external de field_id. As anotações elem destas 26 funções
contêm Mathy e não Locatable. Isto é semântica/forma pública da linguagem,
não igualdade mecânica de Func nem inferência de intenção por render.

### Responsabilidade exclusiva e decisão

A fachada propaga diretamente as 23 pontes internas declaradas no owner
`00_nucleo/prompts/compiler/stdlib/structural/math.md`, de `structural` a
`compiler::stdlib`, com `pub(in crate::compiler)`. Não copiar schema/validação,
criar wrapper, alterar bindings, mudar API externa ou resolver a divergência
histórica de visibilidade de native_flush neste lote.

### Aceitação e limites

Invariantes compartilhadas: consumir o Núcleo Tekt pinado
`00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml`; as
responsabilidades exclusivas acima definem o segmento de cada owner.

Correção interna de tabela/composição em fluxo contínuo ADR-0127. Nenhum campo,
trait, assinatura Rust externa, compatibilidade, default ou fase muda. L0
antes dos testes/código; RED funcional por identidade ausente, GREEN dos 26
casos com filtros e exclusões; controles de With, homônimo e helpers que não
são elementos. Validar antes/depois toda a matriz, preservando observáveis
fora das identidades autorizadas e aliases efetivos (accent/cancel globais
cristalinos incluídos, sem crédito de paridade se não existem no pin).

O sucesso de where/kind não certifica getters, síntese, indexação, show,
layout ou counter. Os controles counter permanecem explícitos e a sua
admissão separada não é alterada neste lote. As quinze rotas restantes de
formatos/HTML/PDF têm investigação própria; a colisão de callback dos formatos
não é resolvida por nomes. FG1/RM1/SY1/RG1 preservam escopos e gates.

Verificar ownership/V26 antes do resselo restrito aos cinco pares; build,
suíte completa, fmt, Clippy/Rustdoc estritos, linhagem e diagnóstico global
sem nova violação, sob MemoryMax12GiB/High10GiB/swap0/jobs1 sequenciais.


### Posição das pontes no hub — correção Clippy

Medição anterior à decisão: candidate-1/strict-details.json em
00_nucleo/diagnosticos/native-math-catalogue-2026-10-17 registra
clippy::items_after_test_module no hub estrutural:137/1199. Fontes, hashes
e comandos congelados dessa tentativa estão em candidate-1/state.json.gz.
O conjunto de callbacks e seus corpos já passou os testes focados e a
matriz; a posição do item após o módulo inline de testes é a falha nova.

Manter o reexport interno deste lote antes da declaração de módulo de testes,
junto dos itens de composição. Mover apenas o item, preservando seus símbolos,
ABI, visibilidade e todos os itens preexistentes. Não adicionar allow/expect.
Revalidar a versão final ressela­da: fonte mudou, portanto as medições da
tentativa anterior são histórico e não substituem os gates da versão final.


## Formatos como funções de elemento — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado. `00_nucleo/diagnosticos/native-format-catalogue-2026-10-17/`
conserva before.json.gz com UTC, status, diff HEAD --stat e hashes exatos;
baseline-reuse.json prova igualdade de fontes/configuração/binário com a
matriz final anterior de 3956 casos em oito perfis. before-batch.json.gz
mede 144 casos detalhados; before-compat.json.gz mede 456 controles de
nomes, aliases e namespace. Cada sonda conserva argv, binários e saídas.
O pin é `9f2b6e8715237cb086899a42873660fe744622e8`, executável SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

As quinze rotas restantes são rejeitadas por where/kind no Tekt e aceitas
no pin. format.rs:17–27/96–146 compartilha native_format entre os cinco
formatos, logo só ponteiro não distingue schemas. pinned-schema.json.gz
conserva nove declarações upstream com blobs/hash/linhas e atributos elem.
FuncRepr é pub(crate) em entities/func.md:21 e entities/func.rs:24; a nota
anterior de que o L0 o mostrava público foi refutada em retificacao-func-repr.md.
public-contract-before.json fixa assinaturas, reexports e structs públicos
antes da mudança; a classificação é representação interna e paridade, não
novo contrato público ou inferência de intenção por render.

### Responsabilidade exclusiva

Disponibilizar reexports diretos pub(in crate::compiler) de
html::native_html_frame e pdf::{native_pdf_attach,native_pdf_artifact},
antes do módulo de testes. native_html_elem já é público e não é duplicado.
Preservar todos os exports externos, factories, registry e helpers.

### Aceitação e fronteiras

Consumir as invariantes compartilhadas do Núcleo Tekt pinado
`00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml`. As novas
alternativas Native/Format são agrupáveis porque usam o mesmo payload/ABI
para projeção e execução; cada uma deve ser exercitada nos testes de nome,
namespace, ponteiro, Eq/Hash, clone/With e despacho. V19 novo nessas uniões
deve ser inventariado e ligado aos testes; não adicionar suppressions.
A nova variante deve ser explicitamente nomeada nos matches fechados.

Fluxo contínuo ADR-0127: nenhuma definição, assinatura, campo ou trait
externo novo, compatibilidade, default ou fase muda. Antes do código, RED
funcional dos quinze casos por factories atuais e controle de observação
real contra Native não marcada. GREEN exige corpus completo, 144 casos de
campos/chamadas e 456 controles de compatibilidade preservados fora de
where/kind. Comparar public-contract-before/after e sizeof antes/depois.

L0 antes do código; ownership/V26 antes de resselo restrito aos onze pares.
Build, suíte workspace, Clippy/Rustdoc estritos, fmt, V1/V5/V15/V26, reverse
e diferencial global sem novo erro/aviso. Comandos pesados sequenciais sob
MemoryMax12GiB/High10GiB/swap0/jobs1. Rastrear notas V19 legítimas com cobertura.

Esta etapa fecha admissão where/kind, não todas as operações de seleção,
show/query, síntese, morfologia dos carriers ou counter. As frentes FG1, RM1,
SY1, RG1 e V16 anteriores preservam seus escopos/gates; não aplicar candidatos
antigos por cópia nem publicar.


## Avaliação única em chamadas — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/call-once-2026-10-17/before.json.gz registra UTC,
status, diff HEAD --stat, fontes/L0/config e hashes. Os 2124 hashes da etapa
anterior foram verificados exatamente antes do baseline. before-probe.json.gz
e before-math-probe.json.gz conservam binários/argv/fixtures/saídas bilaterais.
Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 do binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
calc.abs/array.len/str.from-unicode medem receiver=3/args=2 contra 1/1;
module.measure/layout/within/before/after/func/has/body chegam a receiver=4;
fallback mutante mede args=3 contra 1. Métodos ausentes executam panic de
argumento antes do erro causal. math.mat soma duas vezes; array prévio (1,)
fica (1,1,1) contra (1,1). Array vazio mascara o defeito com truncate.
call.rs:70–118 perde receiver; collection_methods.rs:43 avalia args antes de
reconhecer operação; contextual/content/selector reavaliam AST no fallthrough;
method_dispatch.rs:247 descarta args; invoke.rs:66–120 compensa por nome.
O L0 call_dispatch.md já exige receiver/args uma vez. pinned-source.json.gz
fixa call.rs/methods.rs upstream: interesse é ordem/efeito/diagnóstico da
linguagem, não copiar Vm, storage ou igualdade Rust.

A primeira fixture declarava palavra reservada or como função: é inválida e
está arquivada em invalid-fixture-attempt, sem crédito para efeitos de chamada.
Ela revelou panic 101 em import inválido; manter aberto para correção causal
própria no owner de parsing/import. O baseline válido substitui só a fixture.

### Decisão e responsabilidade exclusiva

Reexportar CollectionDispatch e try_dispatch_collection_method_with somente no crate e preservar os exports existentes, sem wrapper ou lógica.

Consumir o núcleo eval/call-once.toml pinado. Mudança mecânica interna e
correção de paridade ADR-0127 em fluxo contínuo. Assinaturas pub fn/pub use e
structs públicos não mudam; os parâmetros novos/retornos/helpers acima têm
visibilidade crate/eval, sem novo trait público, campo público, default ou
fase. Esta cláusula supersede somente restrições históricas conflitantes de
assinatura interna e compensação por nome; manter extensões Tekt existentes.

### Aceitação e limites

Testes funcionais co-localizados antes do código: efeitos de receiver/args,
fallback mutante, erro antes de args, where/with/selector/coleções preservados
e matriz sem truncar efeito legítimo. Confirmar RED funcional e GREEN.
Comparar casos bilaterais válidos e controles; hits e misses de todos os
métodos de coleção, erros, spans/occurrences e complementos de Value. Não
mascarar dívida residual de diagnóstico como igualdade. Testar dispatch
Unhandled sem perder receiver ou executar callback de args; contar avaliações
pelo efeito observado, não por bytes de PDF, Rust Eq ou microbenchmark.

Ownership/V26 antes de resselo só dos owners do lote. Build, suíte workspace,
Clippy all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito,
fmt, V1/V5/V15/V26 e reverse, SARIF diferencial sem erro novo. V19 uniforme
continua informativo, com matriz de alternativas e relação de comportamento;
sem supressão nem mudança de configuração. Comandos pesados sequenciais com
12 GiB/high10GiB/swap0/jobs1. Não publicar/commit/merge/release nem declarar o
objetivo global fechado por esta etapa.
