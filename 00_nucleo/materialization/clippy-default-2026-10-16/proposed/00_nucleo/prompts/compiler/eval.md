# L0 — compiler/eval
Hash do Código: 5c7a9d91

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:6eec17984449d21b626379651b9b0a511a2dfe8e1a1f0a0af230cce52a47b827
- 00_nucleo/prompts/_nuclei/show-occurrence.toml sha256:3c1058d01f7ca34817e561ef210e78eda38dcb993028414d0b30f296d35535d7

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/mod.rs:81` — `pub fn realize_outlines`.
- `01_core/src/compiler/eval/mod.rs:92` — `pub fn repr_stroke_value`.
- `01_core/src/compiler/eval/mod.rs:100` — `pub fn repr_value_for_serialization`.
- `01_core/src/compiler/eval/mod.rs:110` — `pub fn pdf_standards_conflict_diagnostic`.
- `01_core/src/compiler/eval/mod.rs:252` — `enum ContextReadRequest`.
- `01_core/src/compiler/eval/mod.rs:331` — `struct ContextRead {`.
- `01_core/src/compiler/eval/mod.rs:345` — `struct ContextReads {`.
- `01_core/src/compiler/eval/mod.rs:738` — `pub struct EvalContext {`.
- `01_core/src/compiler/eval/mod.rs:905` — `pub enum EvalTarget {`.
- `01_core/src/compiler/eval/mod.rs:915` — `pub fn eval`.
- `01_core/src/compiler/eval/mod.rs:928` — `pub fn eval_expression`.
- `01_core/src/compiler/eval/mod.rs:940` — `struct ExpressionWorld<'a> {`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/value.rs:210–214`.
- `lab/typst-original/crates/typst-library/src/foundations/value.rs:343-362`.
- `lab/typst-original/crates/typst-library/src/lib.rs:374`.
- `lab/typst-original/crates/typst-library/src/lib.rs:397-401`.
- `layout/grid/mod.rs:422-438,574-677,767-768`.
- `model/table.rs:289-305,494-613,732-733`.
- `typst-bundle/src/lib.rs:67-76`.
- `typst-eval/src/call.rs:638`.
- `typst-eval/src/code.rs:148`.
- `typst-library/src/lib.rs:430`.
- `typst-realize/src/lib.rs:1261-1325`.

## Contract

Own EvalContext storage, public evaluation entrypoints, base-scope construction and one document evaluation. Context-read capture/replay, markup-stream coordination and the exhaustive Expr dispatcher belong to `00_nucleo/prompts/compiler/eval/context_reads.md`, `markup_stream.md` and `expression_dispatch.md` respectively. Retain explicit Engine/Scopes/context state and the pinned core and
feature-gate invariants. No child behavior becomes hub-owned merely because
the hub calls or reexports it.

## Entrypoints and scope

Keep eval, eval_expression, eval_expression_with_features,
eval_expression_with_features_and_introspector, eval_with_full_error,
eval_with_full_error_and_target and eval_with_full_error_target_and_features with
their existing signatures, return values, sink behavior and defaults. Existing
default target is Paged; full_error defaults false and Features default empty.
Target Paged/Html/Bundle is explicit and does not enable a feature. Eval receives
resolved configuration, never reads environment or imports infra.

Acquire World inputs once for the single document evaluation. Build a fresh read-only Library base containing the composed stdlib,
predefined colors, text and supplied element-registry constructors. Local scopes
remain mutable/shadowable. The lexical binding std holds an independent clone of
the unshadowed stdlib in a Module publicly named global; do not create a global
binding called global. Ordinary local std bindings can shadow it normally.
Private make_stdlib wrappers delegate composition to
00_nucleo/prompts/compiler/stdlib/registry/mod.md and its family owners. Global
registration, namespace membership, color aliases, native PDF/HTML/asset identity
and compatibility math globals remain normative there, not copied into this hub.

The legacy string expression facades build a numbered parse_code Source from
world.main() and the complete expression; the Source-based facade below preserves
the caller-supplied identity. The private per-invocation World overlay returns that
identical Source for its id and delegates every other World service/id unchanged,
including defaults, inputs, plugin/path/package/font/date services. Do not mutate
the caller's World or recreate its resources. The injected-introspector sibling
clones the supplied snapshot before evaluation; the ordinary feature-aware
entrypoint delegates with an empty snapshot and preserves its default behavior.
Keep parser diagnostics, scopes, route, sink, limits and feature transport.

A produced Func with detached private origin may acquire the producer-expression
span. Never overwrite a known closure/alias/With/factory/contextual origin, change
other Values, execute a function or make origin part of public equality. A Span
without its Source is insufficient for a resolvable call trace.

## Document coordination and dispatch

Propagate syntax errors with hints; markup empty-stars/underscores warnings remain
nonfatal sink diagnostics. Preserve the existing empty-source warning literal.
Run one fresh-scope/context document evaluation, capturing show recipes without executing them. Keep its scope, bibliography styles and metadata. After common code executes, call the pure realization facade; retain the resulting provenance graph for visual layout and semantic introspection. Preserve diagnostics, warning hints, page-column wrapping and forbidden FlowEvent handling. The previous requirement for two fresh evaluations is revoked by the confirmed realization gate.

Expr dispatch remains static and exhaustive. Preserve literal/unit construction
(Percent -> Ratio), lexical lookup, arrays/dictionaries/spreads, blocks, contextual
content and first-error order. Code blocks accumulate through operators::join.
Boolean And/Or evaluate the left operand first, type-check as currently required,
and do not evaluate a short-circuited right operand. Assignment, calls, closures,
control flow, imports, markup and math delegate to their owners below. Retain the
first flow event until the responsible loop/function consumer handles it.

Markup captures SpaceFromNewline only when the whitespace token contains newline,
otherwise Space. Comments/trivia do not erase adjacent origin; Parbreak/Linebreak
remain distinct. Defer eager per-leaf textual show to the rules owner with the
causal lexical style/rule snapshot, so later declarations do not retroactively
affect earlier groups. A ContentBlock held as a value retains raw whitespace
until insertion; local textual-show matching preserves unmatched separators.
Normalization and realized-group matching are specified only by
00_nucleo/prompts/compiler/eval/rules/textual.md. Smartquote markup reads enabled,
alternative and canonical quotes from styles and delegates to the shared pure
quote resolver; it still emits Text and does not migrate to layout.

## Contextual observation coordination

Keep a private empty-per-evaluation log of the operations actually observed,
independent of introspector state. Store the effective request/arguments, span,
current file, location, in-context state, target/features, immutable styles/rules/
guards and complete result or error. Do not reconstruct requests from AST,
diagnostic strings, equal Values or a final global stack. Preserve query/locate/
here, state/counter (including label resolution and legacy requests), Location
and outline helper requests as their typed existing operations.

A counter read whose selector tree contains Element selects the block before
location/label/callback resolution, including failing reads. Traverse Where,
And/Or, Within and both Before/After branches without flattening them. Once
selected, every already-observed introspective entry participates in validation.
has_filtered_counter_reads remains a pure getter. The doc-hidden public
has_location_position_reads getter reports actual LocationPosition requests even
if their result is None/error; page, page-numbering/query/state/counter do not
select that demand. L3 owns the retry/fixpoint choice, not this getter.

context_reads_valid_for replays only recorded requests against the candidate
TagIntrospector with captured causal context and current supplied Engine resources,
delegating operation semantics to their owners. Do not reexecute the block body,
callbacks never demanded or append new observations while replaying. The private
relation comes exclusively from
00_nucleo/prompts/compiler/eval/observation.md: Same certifies preservation;
Different/Unproven, success/error transitions or changed full diagnostics invalidate
reuse. Mechanism failures propagate SourceDiagnostic; a stable observed error
does not turn the body into success.

context_nonconvergence_diagnostics uses the orchestrator's real ordered candidate
history and final-attempt requests projected onto it; it does not claim that the
body ran those requests in every round or invent details for opaque/alien reads.
OutlineEntryPage and OutlineEntryIndented remain distinct causal reads, not
counter/state substitutes; indented carries the actual entry/prefix/inner/gap and
private parent styles, while absent parent fails before registration.

## Delegation boundary

- 00_nucleo/prompts/compiler/eval/bindings.md owns the reexport hub and its children.
- 00_nucleo/prompts/compiler/eval/call_dispatch.md owns argument/call routing/traces;
  00_nucleo/prompts/compiler/eval/closures.md owns closure capture/application.
- 00_nucleo/prompts/compiler/eval/control_flow.md owns loops/conditionals;
  00_nucleo/prompts/compiler/eval/flow.md owns event representation/forbidden text.
- 00_nucleo/prompts/compiler/eval/operators.md delegates pure operator semantics.
- 00_nucleo/prompts/compiler/eval/modules.md owns imports/includes;
  00_nucleo/prompts/compiler/eval/markup.md owns specialized markup adapters;
  00_nucleo/prompts/compiler/eval/math.md owns mathematical syntax adapters.
- 00_nucleo/prompts/compiler/eval/rules.md owns set/show/realization, with
  00_nucleo/prompts/compiler/eval/selector_matching.md and
  00_nucleo/prompts/compiler/eval/show_rule_termination.md as independent owners.
- 00_nucleo/prompts/compiler/eval/font_dict.md, bibliography.md, table.md and cast.md
  own their exclusive helpers (their full canonical paths are in the owner map).
- 00_nucleo/prompts/compiler/eval/repr.md owns morphology formatting;
  00_nucleo/prompts/compiler/eval/observation.md and its children own internal
  observation comparison, distinct from both public equality and public repr.
- 00_nucleo/prompts/compiler/eval/tests.md and the independent test prompts own
  their exclusive test consumers, not this productive module.

The pure public repr_stroke_value and repr_value_for_serialization facades delegate
to repr without Debug, I/O or deciding structured serialization classes. Keep their
signatures and caller-owned classification. Public realize_outlines and
pdf_standards_conflict_diagnostic facades delegate to rules; they do not transfer
rule or PDF-schema ownership to this hub.

Preserve public contracts, defaults, compatibility and phases. Any ambiguous
change affecting these requires ADR-0127 direction before implementation.

### Refinamento de precedência medido antes da implementação

No estado precedence-state.json, regression-acceptance.json registra 191 verificações passando e duas falhas no controle show-par-sets-false. O Tekt emite extraordi-/interna- onde vanilla conserva palavras sem hifenização. show-par-full-tree.log mostra SetPage e o escopo global dentro de um Par sintetizado; a recipe fica exterior ao text.hyphenate true. eval/mod.rs:1532-1560 possui a proveniência exata dos escopos #set; rules.rs:660-727 apagou essa distinção ao tratar Styled sempre como inline. Não inferir origem pelo conteúdo do Styles.

Decisão interna: o fold dos escopos #set em eval_markup transporta marcador privado eval.flow.scope=true no Styles existente. Antes de formar o primeiro parágrafo material, realize_flow reconhece esse marcador e mantém o escopo exterior à realização dos seus parágrafos, preservando os carriers prévios SetPage/Empty/Space/SpaceFromNewline em ordem, sem os embrulhar em Par. Após conteúdo inline material o escopo permanece inline dentro do parágrafo corrente; wrappers de constructors/estilos sem marcador permanecem inline. Nenhuma entidade, assinatura, comportamento válido por defeito ou fase pública muda. Pin no núcleo compartilhado paragraph-composition documenta a distinção; não recorrer a heurística por chave de estilo. O consumer do hub só produz o marcador no fold; preparação e reconhecimento são do owner rules.


## Realização com avaliação única

A avaliação do documento executa o programa comum uma vez com defer_show_rules interno ativo. A realização possui um EvalContext independente, realizing_show_rules ativo e Locator por documento; não usar flags de captura de P498 para reexecutar o programa. Capturar receitas preserva estilos, guards e arquivo, sem aplicá-las no programa comum. Revoga a exigência histórica de duas passagens de eval. A fachada eval existente conserva compatibilidade e chama a fase pura de realização após executar o programa; a fachada de realização também fica disponível para coordenação de L3.


## Realização com avaliação única

Inicializar os estados de realização por instância de EvalContext, sem estado global.


## Realização com avaliação única

No único run_pass do documento, deferir receitas, mantendo execução de código, scope e metadados causais.


## Realização com avaliação única

Module conserva a árvore realizada com proveniência em content e introspection_content; introspecção percorre original e produto sem escolher por cardinalidade. Warnings de realização são encaminhados ao sink existente.


## Realização com avaliação única

A aplicação de receitas é reutilizável pelo consumer irmão compiler/realize, por fronteira interna de crate.


## Realização com avaliação única

Conservar flag local has_deferred_show_rules apenas como evidência de captura real; um documento sem receitas capturadas entrega conteúdo diretamente sem travessia de realização nem nova avaliação.


## Realização com avaliação única

Inicializar evidência de captura por avaliação; não deduzir ausência de receitas pelo texto do arquivo.


## Realização com avaliação única

O resultado interno de avaliação transporta a evidência de captura real junto com conteúdo, scope e metadados, sem alterar assinatura pública.


## Realização com avaliação única

Transportar flag local de receitas capturadas no resultado interno.


## Realização com avaliação única

Usar evidência de captura para selecionar realização, não sintaxe nem heurística de elementos.


## Realização com avaliação única

Sem receitas capturadas, o conteúdo comum segue diretamente para page-column wrapping; não construir árvore equivalente por conveniência.


## Realização com avaliação única

Preservar page-column wrapping e compatibilidade no caminho sem receitas.


## Realização com avaliação única

Encaminhar o diagnóstico de realização completo ao sink, sem descartar hints/traces nem mudar dedup.


## Reaproveitamento do índice realizado — 2026-10-05

A inspeção em `01_core/src/compiler/eval/mod.rs:888–896` mostrou descarte do campo introspector de RealizedDocument e publicação apenas de visual. Após o gate confirmado de `00_nucleo/prompts/entities/module.md`, conservar ambos os resultados. O visual do caminho realizado já recebeu wrap_page_columns da fachada; o caminho sem receitas conserva seu helper atual. Publicar os carriers de Content antes de chamar o setter interno do índice no Module. Delegar validade, invalidação e lifetime ao owner `00_nucleo/prompts/entities/module.md`, sem nova avaliação e sem índice sintético no caminho sem receitas. Exigir teste de Module avaliado com show dispondo de original e gerados, além de equivalência de consultas/ordem com reconstrução fresca.

## Estado privado de receitas por ocorrência — 2026-10-06

Medição: apply.rs:110-136 guarda apenas RuleId durante callback; EvalContext:285-405 não conserva pares de ocorrência. A auditoria show-recursao-auditoria-2026-10-06 mede quatro gerações infinitas indevidamente aceitas; o owner apply recebeu confirmação humana para corrigi-las. Este owner armazena e inicializa mapa privado de pares Location/RuleId com expansão em andamento ou produto concluído e profundidade privada zero. O algoritmo pertence ao owner apply e obedece ao núcleo show-occurrence. Não altera campos ou métodos públicos.

A profundidade privada acompanha somente transformações ativas. full_error conserva vetor privado de formas ancestrais inicialmente vazio, para classificar o erro de profundidade sem encerrar por morfologia. O owner apply avança next_rule_id acima de snapshots capturados antes de executar receitas locais.

## Identidade de documento e expressão — gate confirmado 2026-10-06

### Evidência anterior à decisão

`00_nucleo/diagnosticos/diagnostic-paths-2026-10-06/identity-next-audit.json`
conserva comandos, fontes, UTC, HEAD, árvore alterada e hashes dos binários do
Tekt e do vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8. Os casos
separam erro/warning do documento de erro da expressão: o documento perde sua
fonte, enquanto a expressão recebe indevidamente o nome do documento.
`01_core/src/compiler/eval/mod.rs:609-615` usa world.main() para Source e base;
`04_wiring/src/eval.rs:66-95` repete esse id ao fornecer o primary ao formatter.
`03_infra/src/world/cache.rs:58-78` registra arquivos e recicla o contador;
`lab/typst-original/crates/typst-cli/src/eval.rs:159-179` e
`lab/typst-original/crates/typst-cli/src/world.rs:176-190` distinguem expressão,
stdin e ausência de entrada por identidade. A igualdade dos inteiros e o
interner global upstream são mecânica, não requisito (ADR-0107). A origem
mostrada, fonte, posição e resolução relativa são observáveis de linguagem.

### Decisão e escopo do gate

Adicionar a fachada pública pura:

```rust
pub fn eval_source_with_features_and_introspector(
    world: &dyn World,
    source: &Source,
    features: Features,
    introspector: &TagIntrospector,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>);
```

A Source fornecida já é numerada e analisada em parse_code pelo caller. Usar
sua árvore e id para sintaxe/spans, sem reparse, substituição de id ou alocação.
Conservar o overlay privado somente para source.id(); current_file e Route
iniciais continuam world.main(). Delegar os demais serviços/ids ao World.
Conservar setup, scopes, feature transport, introspector clonado e sink.
As fachadas públicas string existentes preservam assinatura e compatibilidade:
constroem sua Source legada com world.main() e delegam à fachada Source-based.
Elas não prometem identidade separada; callers contextuais que precisam de
fontes independentes devem usar a nova entrada. A CLI migra neste pacote.
Não mudar World, FileId, Source ou fases de realização/layout.

Gate ADR-0127 confirmado pelo dono em 2026-10-06 (resposta: confirmo), conforme
00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json.
Consumer exclusivo preservado; Hash do Código acompanha a materialização.

## Alcance de warnings de markup vazio — 2026-10-06

Medição anterior à decisão: empty-markup-warning-2026-10-06/before.json conserva
HEAD/árvore alterada/UTC/binários/fontes e controles bilaterais de **, __,
prefixo Unicode, adjacência, conteúdo não vazio e delimitador aberto. O pin
9f2b6e871 marca os dois delimitadores; Tekt marca um caractere. parse/markup.rs:143-177
insere warn_at antes de wrap; parser/diagnostics.rs:99-102 usa texto vazio.
Este é CST privado lossless permitido; eval/mod.rs:765-778 publica esse span
vazio diretamente. No upstream typst-syntax/src/parser.rs:145-169, o warning
pertence ao construct embrulhado. Classificação: alcance do diagnóstico é
observável de linguagem; não exigir a mesma representação de warnings da CST.

Correção interna de paridade em fluxo contínuo ADR-0127: ao publicar somente
NoTextWithinStars/NoTextWithinUnderscores no sink, localizar pela identidade do
span seu nó Error vazio na árvore numerada. Usar o span do pai direto se ele
for respectivamente Strong/Emph com comprimento textual dois. A relação é da
árvore efetiva, sem inferência pelo texto da mensagem, soma aproximada de offsets
ou mudança da Source/CST. Se o vínculo não existir, conservar o span original.
Conservar severity, hint, ordem, número de warnings, demais erros e defaults.
Não adicionar API pública, campo, trait, nova fase ou unsafe.

## D06 — ligação do pacote ratificado

Medição: math-bare-2026-10-06/before.json.gz, frac-query retorna zero versus um;
valid-red.log prova índice sem Equation. eval/mod.rs:908–925 realiza somente se
has_recipes, omitindo preparação math sem show. Depois da avaliação única,
selecionar a mesma realização também quando o helper privado bare_math encontra
inserção estrutural externa, parando em Equation/Located. Conteúdo sem receitas
e sem math externo conserva o caminho direto. Scope/valor cru não muda; usar
realize_document existente, mesmo target/features/sink/índice, sem reavaliar
fonte nem criar outro contrato público. A exceção à cláusula sem receitas é
estritamente a fronteira matemática já ratificada no pacote D06.

## Integração condicional dos módulos de teste — 2026-10-13

Medição antes da decisão: HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b,
working tree não commitado. A duplicação cfg(test) ocorre em `01_core/src/compiler/eval/p1425_tests.rs:5` / pai :972; `01_core/src/compiler/eval/p1426_tests.rs:5` / pai :975; `01_core/src/compiler/eval/p1427_tests.rs:5` / pai :978; `01_core/src/compiler/eval/p1429_observation_tests.rs:5` / pai :981; `01_core/src/compiler/eval/p1430_tests.rs:5` / pai :985; `01_core/src/compiler/eval/p1431_tests.rs:5` / pai :989.
Fontes, L0, UTC, status e diffstat estão em
`00_nucleo/diagnosticos/cfg-test-ci-2026-10-13/before.json.gz` e
`refinement-before.json.gz`. O Clippy pinado reproduz os atributos duplicados.
A tentativa de retirar a condição interna mantém os testes Rust, mas o linter
arquitetural pinado não propaga a condição do pai e acusa V2 nos helpers.

Neste owner, retirar apenas o atributo externo cfg(test) das declarações
listadas acima. Manter os caminhos, nomes e visibilidades dos módulos e seus
atributos path. A condição interna `#![cfg(test)]` de cada arquivo listado
continua a excluir integralmente o módulo do build de produção, inclusive
seus imports e helpers. Os consumers de teste e seus L0 permanecem intactos.
Não acrescentar dependências, lógica, API, modo padrão ou fase. Validar a
mesma lista de testes, suíte core, build sem cfg(test), Clippy e arquitetura.

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/eval/mod.rs` nas linhas 958 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `00_nucleo/diagnosticos/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Compilar o helper interno `make_stdlib` somente sob `#[cfg(test)]`, conservando assinatura e corpo para os testes existentes. make_stdlib_with_features permanece produtivo; o wrapper de Features::default é utilizado somente pelos testes.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.


## Aridade das fronteiras — `clippy::too_many_arguments`

### Evidência anterior à decisão

A leitura estrutural abaixo usa HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
capturado em 2026-10-15T16:39:18.873477+00:00. Fontes completas, L0 anteriores, assinaturas,
hashes, UTC, status e `git diff HEAD --stat` estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/before.json.gz`.
A classificação individual e as cláusulas deste L0 lidas antes da decisão estão em
`00_nucleo/diagnosticos/too-many-arguments-2026-10-15/classification.json`.
Os spans Clippy foram reconfirmados contra as fontes do inventário anterior.
É medição de estrutura Rust; não é nova prova de paridade ou de falha funcional.

- `C003`: `01_core/src/compiler/eval/mod.rs:682`, `eval_with_full_error`, 8 argumentos contados pelo Clippy (limiar 7).
- `C004`: `01_core/src/compiler/eval/mod.rs:709`, `eval_with_full_error_and_target`, 9 argumentos contados pelo Clippy (limiar 7).
- `C005`: `01_core/src/compiler/eval/mod.rs:734`, `eval_with_full_error_target_and_features`, 10 argumentos contados pelo Clippy (limiar 7).

### Prescrição exclusiva por função

As correções abaixo são alvos internos de manutenção, ainda não materializados nesta auditoria.
Preservar as assinaturas públicas vigentes e as decisões de linguagem deste owner.
A necessidade de corrigir estrutura é julgamento de manutenção apoiado nos dados
explicitados; o aviso de aridade, sozinho, não demonstra um defeito funcional.

#### `C003` — `eval_with_full_error`: Exceção pontual justificada

Fachada pública de compatibilidade: registry e full_error são entradas explícitas; _routines/_traced/_route são transportados ao próximo wrapper, mas não consumidos na entrada completa atual. A exceção destes parâmetros é por compatibilidade prescrita no L0, não por necessidade operacional demonstrada.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C003: Fachada pública de compatibilidade: registry e full_error são entradas explícitas; _routines/_traced/_route são transportados ao próximo wrapper, mas não consumidos na entrada completa atual")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C004` — `eval_with_full_error_and_target`: Exceção pontual justificada

Fachada pública que acrescenta target à anterior sem ativar features. Preservar a distinção e os defaults declarados no L0, sem substituir a assinatura para atender ao limiar.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C004: Fachada pública que acrescenta target à anterior sem ativar features")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

#### `C005` — `eval_with_full_error_target_and_features`: Exceção pontual justificada

Entrada pública completa de avaliação: World/Source/registry/sink e configuração são utilizados; _routines/_traced/_route não são lidos pelo corpo atual. A cláusula Entrypoints and scope manda preservar a assinatura; esses três permanecem por compatibilidade, sem alegar que são necessários ao algoritmo.

Preservar assinatura, ordem e semântica. Na materialização, usar #[expect(clippy::too_many_arguments, reason="C005: Entrada pública completa de avaliação: World/Source/registry/sink e configuração são utilizados; _routines/_traced/_route não são lidos pelo corpo atual")], somente nesta função.

Aceitação: Confirmar o aviso no Clippy workspace/all-targets e a ausência de unfulfilled_lint_expectations após a anotação; preservar chamadas e controles funcionais do owner. Não elevar limiar global nem silenciar outros lints.

### Limite da exceção

Autorizar somente `#[expect(clippy::too_many_arguments, reason = "...")]` no item
nominal acima, com o motivo arquitetural indicado. Não aplicar no módulo, impl,
crate ou configuração global; não elevar o limiar nem abranger outro lint.
Nos casos com remoção de argumento morto, a exceção residual depende dessa correção.
Se a função deixar de exceder o limiar, retirar/revisar a expectativa; não silenciar
`unfulfilled_lint_expectations`. Esta permissão específica supera proibições
genéricas de supressão em seções anteriores somente para este lint e os itens
nominais justificados aqui. As demais obrigações de correção permanecem.

### Fronteira de materialização

Nesta auditoria, somente o L0 e seu selo de linhagem são atualizados; atributos de
supressão e mudanças de assinatura/corpo ficam pendentes da etapa de materialização.
As entradas com correção exigem testes de linguagem/integração do owner e controle
de chamadas antes/depois, sem relaxar fixtures ou tolerâncias. Manter as camadas,
a fase, os defaults, o despacho estático e a ausência de estado global mutável.
Correções de assinatura privada devem migrar também os chamadores e seus L0 antes
do código. Qualquer necessidade nova de contrato público ou compatibilidade externa
exige proposta concreta e confirmação ADR-0127; esta seção não a antecipa.


## Tipos explícitos e leitura local — type_complexity (2026-10-15)

### Medição anterior à decisão

01_core/src/compiler/eval/mod.rs:811 (C006).
O snapshot before.json.gz em 00_nucleo/diagnosticos/type-complexity-2026-10-15
conserva HEAD, working tree não commitado, UTC, diffstat, fontes/owners e hashes.
O gate red-clippy.log falha com este lint promovido a erro antes da edição.
A inspeção distingue tuplas posicionais, repetição do alias FontKey e contratos
estáticos já explícitos. É manutenção de representação Rust; não medição de
benefício da IA, bug funcional ou paridade nova.

### Responsabilidade e aceitação

Substituir o resultado privado da closure run_pass por EvaluatedPass, com campos value, scope, bibliography_styles, document_info e has_recipes. Construir e consumir pelos nomes, preservando a execução única, o erro de flow, sinks, metadados e realização. Nenhuma fachada pública muda.

Correção interna em fluxo contínuo ADR-0127: nenhuma entidade, assinatura pública
resolvida, default ou fase é alterada. Uma abreviação de tipo só é permitida com
expansão comprovadamente idêntica; incompatibilidade ou tipo público novo exige
outro gate. As exceções são nominais apenas para type_complexity; não autorizar
supressão em módulo/impl/crate nem elevar limiar. Esta permissão pontual supera
proibições genéricas anteriores somente para os três itens justificados neste lote;
continuam proibidas supressões dos demais casos destinados à correção.
Usar os controles funcionais existentes sem mudar seus esperados. O gate selecionado
deve passar RED→GREEN, incluindo unfulfilled_lint_expectations como erro; revalidar
build, Clippy all-targets, suíte workspace, formato, Rustdoc e linhagem com resselo
restrito. Não criar testes que apenas espelhem aliases ou nomes de campos.


## Default público — proposta sujeita à confirmação ADR-0127

### Medição anterior à decisão

`01_core/src/compiler/eval/mod.rs:397` declara pub fn new() -> Self sem argumentos. O inventário em `00_nucleo/diagnosticos/field-reassign-2026-10-15/clippy-inventory.json` registra new_without_default; fonte/L0/hashes atuais são preservados no pacote clippy-default-2026-10-16. EvalContext::new inicializa target Paged, features vazias, full_error false, apply_show_rules true, orçamento de loop 1_000_000 e estados privados novos. Derivar Default dos campos daria defaults incorretos para orçamento e recipes; a única implementação autorizada delega a Self::new(). Isto é contrato Rust de construção, sem nova medição de paridade da linguagem.

### Contrato proposto

Adicionar somente `impl std::default::Default for EvalContext` com `fn default() -> Self` que devolve `Self::new()`. Preservar integralmente new(), sua visibilidade, campos, seus valores, todos os demais métodos e semântica. Não criar outro trait, novo modo/flag, estado compartilhado global, I/O no construtor ou mudança de fase. Nenhum caller existente precisa migrar; novo código pode usar `<EvalContext as Default>::default()` e bounds `T: Default`.

### Gate, testes e aceitação

Nenhuma materialização antes da confirmação humana do manifesto deste pacote (ADR-0127 §2.1/§2.4). Os impls são acréscimos públicos: sua remoção futura quebraria consumidores do novo contrato. Após ratificação, a cláusula só supersede proibições genéricas de nova API para estes três impls expressos; os demais contratos permanecem. Testes de uso por bound Default devem falhar por ausência de implementação antes do código e passar após delegação. Verificar valores/isolamento de contextos e sessões novas, preguiça de recursos e conservação dos controles existentes; suíte workspace, Clippy all-targets, build, Rustdoc, formato e linhagem com resselo restrito. Não alterar valores/defaults nem suprimir new_without_default.

Os novos testes de EvalContext pertencem exclusivamente a `00_nucleo/prompts/compiler/eval/tests.md`, sem inserir uma suíte paralela no módulo produtivo.
