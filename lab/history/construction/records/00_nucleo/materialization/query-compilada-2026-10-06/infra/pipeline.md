# L0 — infra/pipeline
Hash do Código: 474541e7

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:fb383f227b0bcd8b1e61899942fa0383a4cda66093da58bda2e87844e2895d2a
```

## Current evidence

- `03_infra/src/pipeline.rs:174` — `eval_expression_with_document_features`.
- `03_infra/src/pipeline.rs:244` — `eval_to_module_with_sink_target_features`.
- `03_infra/src/pipeline.rs:355` — `collect_context_blocks_into`.
- `03_infra/src/pipeline.rs:409` — `substitute_context_blocks`.
- `03_infra/src/pipeline.rs:438` — `Timings`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline.rs`.

## Owned contract

- Own the evaluation adapters, comemo setup (`Routines`, `Traced`, `Sink`, `Route`), causal context walkers, public expansion facades and `Timings` serialization. Return raw diagnostics; never format them or write output.
- Isolated expression evaluation must not manufacture markup content. Document-context evaluation compiles through the paged owner, evaluates against its final introspector, and appends expression warnings after compilation warnings without exporting.
- Keep compatibility defaults (`full_error=false`, empty features, Paged evaluation) and explicit target/features unchanged. Reject assets outside Bundle with `assets are only supported in the bundle target` and hint `try enabling the bundle target`.
- Discover context blocks through the exhaustive L1 content traversal, accumulating the effective ancestor `StyleChain`. Substitute through the same topology; delegate attempt coordination to its child rather than implement a second stabilization algorithm.
- Preserve public import paths through explicit reexports. Private helper access stays within this module family. Target compilation, resources and stabilization belong to the child owners listed below; this prompt does not own their bodies.
- Timing fields describe existing production phases, including shaping and subsetting. Do not add boundary callbacks, an observation ledger, extra replay or an instrumented default.

## Child owners

- `00_nucleo/prompts/infra/pipeline/bundle.md` owns only `03_infra/src/pipeline/bundle.rs`.
- `00_nucleo/prompts/infra/pipeline/context_stabilization.md` owns only `03_infra/src/pipeline/context_stabilization.rs`.
- `00_nucleo/prompts/infra/pipeline/html.md` owns only `03_infra/src/pipeline/html.rs`.
- `00_nucleo/prompts/infra/pipeline/paged.md` owns only `03_infra/src/pipeline/paged.rs`.
- `00_nucleo/prompts/infra/pipeline/pdf.md` owns only `03_infra/src/pipeline/pdf.rs`.
- `00_nucleo/prompts/infra/pipeline/raster.md` owns only `03_infra/src/pipeline/raster.rs`.
- `00_nucleo/prompts/infra/pipeline/resources.md` owns only `03_infra/src/pipeline/resources.rs`.
- `00_nucleo/prompts/infra/pipeline/svg.md` owns only `03_infra/src/pipeline/svg.rs`.
- `00_nucleo/prompts/infra/pipeline/tests.md` owns only `03_infra/src/pipeline/tests.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Realização com avaliação única

Timings.eval_ms cobre avaliação única e realização de receitas da fachada compatível; não descrever a métrica como duas execuções do programa.


## Realização com avaliação única

Coleta contextual atravessa raiz de carriers explicitamente e somente o visual de Realized; o contexto de Located não desaparece porque seu corpo raiz não recebe callback duplicado.


## Realização com avaliação única

Visitante bottom-up reentra nos carriers com a cadeia causal correta, coletando apenas contextos que participam do visual atual.


## Realização com avaliação única

Substituição contextual conserva o Located do marcador original antes do resultado resolvido, preservando here/location e registro de Position. Não trocar a raiz locatable por Sequence dentro do mesmo Located.

## Remoção de wrapper privado sem consumidor — 2026-10-05

Depois de a sessão usar o helper com métricas explícitas, `rg -n "layout_with_realized_math_callbacks\(" 03_infra -g "*.rs"` encontra apenas a definição em paged.rs:592. O import privado em pipeline.rs:90 fica sem uso. Remover essa fachada privada sem consumer e seu import; preservar os helpers com métricas explícitas e com seed usado por bundles. Não alterar reexports públicos, assinaturas públicas ou defaults dos callers remanescentes. Correção interna e reversível no mesmo domínio, fluxo contínuo ADR-0127.


## Preparação de bindings de exportação — 2026-10-05

Medição: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/probe.txt`. Consumir helper privado resolve_export_fonts: fontes originais vinculadas, com gate de instanciação existente. Instanciação fica no consumidor exportador, na mesma fase de render/export, preservando diagnósticos/API/seleção de páginas. O nome antigo prometia bytes já instanciados e não descreve este contrato.

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

Adicionar fachadas públicas sem serialização/export:

```rust
pub fn eval_source_with_sink_features(
    world: &dyn World, expression: &Source, features: Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>);
pub fn eval_source_with_document_features(
    world: &dyn World, document: &Source, expression: &Source, features: Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>);
```

A primeira delega ao novo entrypoint L1 com introspector vazio. A segunda usa
exatamente o compilador paginado existente, recebe seu introspector final e
delega ao entrypoint Source-based. Em falha do documento, retorna suas fontes
cruas e não avalia expressão; em sucesso acrescenta warnings da expressão aos
do documento, sem perder spans/traces. Não reexecutar programa para localizar
erros, reconstruir índice ou transportar bool inferindo a fase do diagnóstico.
Preservar as fachadas string e suas assinaturas/compatibilidade existentes.
Nenhuma função privada do paged torna-se pública por conveniência de wiring.

Gate ADR-0127 confirmado pelo dono em 2026-10-06 (resposta: confirmo), conforme
00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json.
Consumer exclusivo preservado; Hash do Código acompanha a materialização.


## Query CLI compilada — proposta aguardando gate ADR-0127

### Medição anterior à decisão

query-residuos-2026-10-06/before.json.gz registra HEAD/working tree, UTC,
pin/binary hashes, fontes, comandos e stdout/stderr/status. query-stdin-callback
retorna sucesso onde vanilla reporta erro e trace. query-context-success perde
metadata produzido; nested-error também desaparece. query-selector-quotes
apresenta rejeição P206C antes do erro do documento. where-level é rejeitado,
mas vanilla retorna [2]. Document-error-invalid-selector demonstra precedência
da compilação; valid-document-invalid/text-selector demonstra retrieval error
sem depreciação. eval-context-reference e eval-where-reference já coincidem
usando a compilação paginada atual. Upstream typst-cli/src/query.rs:34–66
compila antes de retrieve; :86–105 avalia selector em Scope default e depois
converte para LocatableSelector. Fontes locais: query_helpers.rs:488–522 consulta
somente Module; pipeline.rs:182–214 já usa compilação com introspector final.

### Decisão proposta

Correção da linguagem, não cópia da mecânica upstream. CLI query passa a
compilar documento paginado completo antes da avaliação de selector, sem
exportar arquivo. A execução comum do programa permanece única, conforme os
owners vigentes de realização/contexto. Reutilizar o pipeline paged completo:
nenhuma expansão avulsa, índice pré-contexto ou ciclo de estabilização paralelo.
Distinguir falha de compilação e de retrieval por enum explícito, nunca por
mensagens, spans destacados ou busca de substrings. Há nova API pública e
mudança do caminho de query de eval-only para compilação paginada; PARAR até
confirmação ADR-0127. Candidatos são cópias de revisão, não L0s vigentes.

Este owner adiciona a fachada pública:
```rust
pub fn compile_query_context(
    world: &dyn World,
    source: &Source,
) -> (SourceResult<typst_core::entities::introspector::TagIntrospector>, Vec<SourceDiagnostic>);
```
Com Timings local, chamar compile_to_paged_document_full_error_and_features_with_introspector
existente (full_error=false, Features::default()), projetando somente o
introspector final do par e descartando PagedDocument sem exportar. Preservar
Source causal e warnings crus de compilação. Sem segunda avaliação, novos
flags, observador, caches persistentes ou mudança do pipeline para outros callers.
O algoritmo de paginação/contexto continua com os child owners vigentes.
