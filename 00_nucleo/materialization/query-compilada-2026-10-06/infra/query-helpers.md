# L0 — Query parsing, summaries and located results
Hash do Código: 1adf762e

**Layer:** L3
**Exclusive consumer:** `03_infra/src/query_helpers.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24
```

## Current evidence

- `03_infra/src/query_helpers.rs:110` — `parse_selector`.
- `03_infra/src/query_helpers.rs:154` — `summarize_query`.
- `03_infra/src/query_helpers.rs:225` — approximate content counts.
- `03_infra/src/query_helpers.rs:274` and `:357` — text/variant walkers.
- `03_infra/src/query_helpers.rs:455` and `:475` — summary and located-result entrypoints.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own selector parsing, the existing approximate summary projection and transport of located query values. Do not implement JSON/YAML/repr formatting, field realization, CLI --field/--one selection or a new L1 selector model. Presentation belongs to `00_nucleo/prompts/shell/cli.md`; command composition belongs to `00_nucleo/prompts/wiring/query.md`.

`parse_selector` trims input and recognizes simple ElementKind names, nonempty angle-bracket labels and the `math.equation` alias. Preserve the internal standalone equation spelling. Quoted-text/regex selectors produce the distinct `text is not locatable` error. Empty/unknown/complex selectors preserve their current QueryError messages; dots/parentheses are rejected after the alias check. Diagnostic literals containing a historical case identifier remain exact output, not chronology authorizing a contract.

`query_to_summary(world: &dyn World, source: &Source, selector: &str)` returns `Result<QuerySummary, QueryError>`. Parse before eval; reuse the pipeline eval adapter, obtain module content, and introspect the pre-show tree when supplied. Preserve EvalFailed, NoContent, InvalidSelector and WorldError. This legacy entrypoint discards evaluation warnings. `summarize_query` also takes the raw selector string and retains it in the result. QuerySummary keeps selector, SelectorKind, count, kind_name, label_found and metadata_values without Serialize. Label summaries count zero/one and retain a matched label. Kind summaries count introspector matches except the existing approximate document-element projection. Metadata uses its current plain-text/debug-string projection, not generic language serialization.

`count_element_in_content` retains its distinct List/Enum group walkers, boolean paragraph-text approximation and Link/Raw/Quote/Footnote variant counts; other kinds return zero. Do not promote approximate matches into located values. The list/enum walkers count all-item Sequences and recurse as currently implemented; the singleton discrepancy is recorded in `00_nucleo/diagnosticos/sanitization-infra-wiring-history.md`. The text/variant walkers descend through their explicit content containers, including PageRun.body without interpreting PageConfig. OutlineEntry text consults element; variant counting descends element/fill after the predicate precheck. A matching predicate returns once without counting descendants again.

Mathematical carriers are terminal in this approximate projection, including limits/class/style wrappers, root/fraction/sequence, lr/mid/stretch, primes, accents and over/under decorations. MathUnderline, MathVec and Flush do not invent text/located occurrences or descend into math internals. Space and SpaceFromNewline remain neutral, without guessing newline origin or removing CJK spacing. Public structural traversal/repr/layout semantics remain with their L1 owners; these approximate walkers do not redefine them.

Preserve the current public located-result signature:

```rust
pub fn query_elements(
    world: &dyn World,
    source: &Source,
    selector: &str,
) -> (Result<Vec<Value>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

Invalid selectors become detached SourceDiagnostics without panic. Evaluation errors and warnings remain separate; a module without content returns an empty successful vector. Build original/pre-show and realized introspectors, prefer the realized index only when it has more matches, otherwise retain the original index. Preserve selected location order and cardinality for every recognized locatable kind and labels, without a heading-only filter.

For an entry with captured fields (`Some`), clone the complete IntrospectedContent and transport its exact Location as LocatedContent. Preserve the captured label; never rebuild a captured-field entry from a selector/registry, discard its snapshot or replace Some by None. For an entry with no captured fields, preserve the legacy labeled-content projection by constructing a separate None entry when required, without modifying the original entry. A location without an element fails with `query result has no locatable element` instead of silently dropping it. No output serialization or default-field fabrication belongs to L3.

## Verification boundary

Existing tests cover selectors, summaries, match ordering, label transport and neutral math/newline-space projections. Located values preserve public fields/labels through the L2 serializer. This prompt records the current consumer; it does not resolve archived approval/provenance gaps or certify fresh upstream parity.


## Realização com avaliação única

Query consulta um único índice com origens e produtos causais da realização. Revoga escolha entre árvores por cardinalidade; elementos semanticamente distintos continuam distintos mesmo com campos iguais.


## Realização com avaliação única

Contagens/visibilidade legadas projetam carriers internos para o visual; query semântico permanece no índice realizado com proveniência.


## Realização com avaliação única

Contagem visual de variantes atravessa carriers internos sem contar a origem consumida como conteúdo emitido.


## Reaproveitamento do índice realizado — 2026-10-05

A inspeção em `03_infra/src/query_helpers.rs:478,519` revelou reconstrução do índice semântico inicial já calculado pela realização. Nos dois caminhos de consulta existentes, consultar Module::realization_introspector segundo `00_nucleo/prompts/entities/module.md`; utilizar o índice emprestado enquanto o Module estiver vivo, construindo o fallback apenas quando estiver ausente. Não clonar stores para consulta só de leitura e não consultar árvore visual por cardinalidade. Preservar ordenação causal, campos, labels, originais e gerados. Isso não autoriza persistir índices entre avaliações distintas.


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

Este owner adiciona:
```rust
pub enum CompiledQueryError {
    Compilation(Vec<SourceDiagnostic>),
    Retrieval(String),
}
pub fn query_compiled_elements(
    world: &dyn World,
    source: &Source,
    selector: &str,
) -> (Result<Vec<Value>, CompiledQueryError>, Vec<SourceDiagnostic>);
```
Compilar primeiro via compile_query_context; falha conserva diagnostics crus
como Compilation. Após sucesso avaliar selector com o avaliador code existente
em scope stdlib separado do documento, introspector vazio para a expressão,
Features default e Source code transitória. Não interpolar o selector numa
expressão query(...) nem executar o programa de novo. A Source transitória é
usada internamente pelo avaliador; erros da expressão são projetados como texto
retrieval, nunca enviados ao formatter contra a Source documental.
Erro de avaliação vira Retrieval("failed to evaluate selector: " + mensagens
em ordem, separadas por ", "). Warnings da expressão não são publicados, como
upstream retrieve; os warnings de compilação permanecem separados.
Para o valor avaliado, reutilizar native_query público com Args de um único
valor e EvalContext cujo introspector é o final. Str/Regex de texto recebem
Retrieval("text is not locatable") antes da conversão legada por nome de string;
não reinterpretar texto como kind. Composições e locatability pertencem ao L1
existente; não criar parser .where, match de nomes de função ou modelo Selector
em L3. Falhas da conversão locatable viram Retrieval com mensagem original.
O resultado Array transporta Vec<Value> com LocatedContent e campos capturados;
não reconstruir labels, fabricar campos ou selecionar índices por cardinalidade.
Manter parse_selector, query_to_summary e query_elements como APIs legadas com
seus contratos, sem trocar seus retornos públicos nem promover a aproximação
para o caminho compilado. CLI abandona o helper legado; ele mantém consumers
históricos. Uma assinatura pública nova, sem remover as antigas.
