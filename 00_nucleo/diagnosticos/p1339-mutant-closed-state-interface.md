# P1339 — interface mecânica prospectiva do adaptador closed_state

Autor: `/root/p1336_tests`, somente tradução de fixtures independentes para chamadas reais. Sem expectativas, algoritmo substituto, candidato ou veredito. Isolamento apenas procedural; contexto herdado P1336. Fonte baseline examinada em 2026-09-10, HEAD `2f42d64253547734564513a1159ee6b584c1c4b4` com mudanças L0/header não commitadas. Nenhuma implementação P1339 existe.

Autoridade: `p1339-closed-harness-authority.json`, SHA-256 `08d0be7f19894c111130d0c791df6804cd1e7d89d2b8d6f418702be88af2125d`; manifesto r2 `842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`; L0 freeze `397c136fc8710d44b2f7537193fe5b9c44ab89d40a99bf6b994296e05e7c4d04`. O contrato r2 em preparação será pinado antes de congelar o adaptador final.

## Fronteira test-only, não nova API produtiva

Um módulo de teste descendente de `compiler::eval` pode chamar o entrypoint crate-private abaixo e construir os carriers existentes. Não usar o entrypoint público `eval_expression` como substituto: ele não devolve o `EvalContext` que precisa ser observado. A inclusão futura é somente `#[cfg(test)]`, sem alterar visibilidade produtiva.

```rust
pub(crate) fn eval_expr(
    expr: Expr<'_>, scopes: &mut Scopes<'_>, ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value>;

impl EvalContext {
    pub fn new() -> Self;
    // Baseline já dispõe destes campos:
    // pub introspector: TagIntrospector;
    // pub current_location: Option<Location>;
    // pub in_context: bool;
    // pub target: EvalTarget;
    // pub features: Features;
}
```

Declarações futuras autorizadas pelo L0 `compiler/eval.md:415-430`, ainda **NOT_EXECUTED_PRESEAL**:

```rust
impl EvalContext {
    pub fn has_filtered_counter_reads(&self) -> bool;
    pub fn context_reads_valid_for(&self, candidate: &TagIntrospector,
        engine: &mut Engine<'_>) -> SourceResult<bool>;
    pub fn context_nonconvergence_diagnostics(&self, history: &[TagIntrospector],
        engine: &mut Engine<'_>) -> SourceResult<Vec<SourceDiagnostic>>;
}
```

O adaptador conservará o `Result` do corpo mesmo quando for `Err`, chamará os três métodos reais conforme a fixture e retornará observações separadas. Não transforma `Ok(true)` da validação em sucesso do corpo, nem decide Same/Different/Unproven por um comparador próprio.

## Construção real disponível no baseline

`Engine<'a>` tem exatamente estes campos de construção: `world: &'a dyn World`, `font_metrics: &'a dyn FontMetrics`, `route: Tracked<'a, Route<'a>>`, `styles: &'a mut StyleChain`, `show_rules: &'a mut Arc<[ShowRule]>`, `active_guards: &'a mut Vec<RuleId>`, `current_file: FileId`, `sink: &'a mut TrackedMut<'a, Sink>`. Pode usar World de memória que conserva todas as Sources da fixture, `FixedMetrics`, `Route::root().with_id(file)`, chain declarada e sinks distintos. Não substituir o World lógico entre corpo e validação. Chain não declarada usa a construção baseline `StyleChain::default_chain()`, e esse input fica explícito no recibo.

Fonte de código: `Source::new_with_parser(file_id, text, crate::compiler::parse::parse_code)`, `Expr::from_untyped` sobre filhos reais e chamadas `eval_expr`. Setup e expressões de valores usam o mesmo scope de fixture, para permitir reutilizar uma função/valor já criado; não reevaluar um binding compartilhado quando a fixture pedir retenção.

Carriers e entradas existentes:

```rust
TagIntrospector::empty() -> TagIntrospector
TagIntrospector: Clone
crate::compiler::introspect::introspect_with_introspector(&Content) -> TagIntrospector
crate::compiler::introspect::introspect_with_runtime(
    &Content, &mut Engine<'_>, &mut EvalContext
) -> SourceResult<TagIntrospector>
IntrospectedContent::new(Content, Option<IndexMap<EcoString, Value, FxBuildHasher>>)
Locator::new() -> Locator
Locator::next(&mut self) -> Location
TagIntrospector::inject_positions(&mut self, SealedPositions)
SealedPositions::from_runtime(HashMap<Location, Position>) -> SealedPositions
TagIntrospector::inject_pages(&mut self, PageStore)
PageStore::from_total_pages(NonZeroUsize) -> PageStore
PageStore::from_runtime(NonZeroUsize, Vec<Option<Numbering>>, Vec<Content>) -> PageStore
```

`TagIntrospector` expõe `elements: HashMap<Location, IntrospectedContent>`, `kind_index: HashMap<ElementKind, Vec<Location>>`, `labels`, `counters`, `metadata`, `state`, `positions`, `page_store`, `parent_locations`, `context_block_locations` e outros stores históricos. `IntrospectedContent` não expõe seus campos privados, mas `new` preserva `None` versus `Some(empty)`; `Value::LocatedContent(IntrospectedContent, Location)` é carrier real. `Position` contém `page: NonZeroUsize` e `point: Point`.

Para um descendente test-only dentro do crate, os mutadores já existentes `StateRegistry::init(key: String, init: Value, location: Location)` e `update(key: String, value: Value, location: Location)`, `LabelRegistry::add(Label, Location)` e `MetadataStore::add(Value)` são acessíveis como `pub(crate)`. Não se propõe expô-los ao produto. Raw `Location::from_raw` também é crate-private; preferir `Locator` e referências simbólicas compartilhadas na fixture.

**Limite importante:** `introspect_with_introspector` e `introspect_with_runtime` são entradas distintas. A primeira não realiza os pós-processadores runtime de callbacks. A fixture deve escolher explicitamente a entrada; não inferir que snapshot vazio ou um JSON de tags já equivale à construção produtiva. Não existe hoje `from_tags(tags) -> TagIntrospector` público com esse nome, apesar de comentários históricos o citarem.

## Protocolo de fixture proposto ao autor independente

O autor do oráculo pode congelar JSON contendo `id`, `setup_code` (código Typst), `value_bindings` ordenados (nome, expressão), `snapshots` ordenados, `body_code`, `body_snapshot`, `current_location`, `in_context`, `features`, `target`, `history_ids`, `validate_candidate_ids` e **seus próprios** predicados. Cada expressão recebe identidade de Source estável e span numérico real.

Um snapshot deve declarar uma das origens: `empty`, `clone_of` (retenção causal literal) ou `content_expression` mais `introspection_entrypoint` explícito (`pure`/`runtime`). Pode ainda declarar overlays ordenados que se traduzam diretamente nos carriers listados: associação label→Location; entrada de `elements` com expressão de Content e fields `null` ou lista ordenada nome/expressão; índice kind→Locations; valores iniciais/updates de state; metadata; posições; page store. Referências a valores compartilhados resolvem o binding já avaliado, não código reexecutado. Localizações são tokens simbólicos mapeados uma vez por Locator ou labels realmente encontrados no snapshot.

Não há ainda parser/harness congelado dessa proposta. O autor deve especificar somente as operações necessárias aos seus cenários; operação ausente ou carrier opaco não recebe fallback silencioso. A seleção de cenários, valores esperados, diagnósticos, predicados e limites é exclusivamente do oráculo. Após recebê-los com hash, o adaptador será uma tradução mecânica deles e estes limites serão revisados pelo verificador.

## Pins de assinaturas/carriers consultados

| Fonte baseline | SHA-256 |
|---|---|
| `01_core/src/compiler/eval/mod.rs` (124,223,386,969) | `aa423c6fbc0fc66ebda70e102118313d313a5c81e1335d3c3bc73197562bfd74` |
| `01_core/src/entities/engine.rs` (43) | `8939ec5b36ea682318c3971c12459989114181356d7948607f2be26cf7562853` |
| `01_core/src/entities/introspector.rs` (364,510,530,554) | `176bc6ba1304c499a302345b3ada341c49fa101e374c1298772cc7f9e9d142df` |
| `01_core/src/compiler/introspect.rs` (360,391) | `aa8b05c24f672783ef1bdaf5eea2ce46e8ee1e9e098b8cee8ea2e7cc52787974` |
| `01_core/src/entities/value.rs` (26,32) | `f792590f48799067f2f00767319c97aa498a519f207a154332ffc4deb6c8712b` |
| `01_core/src/entities/state_registry.rs` (47,55) | `51a0423d0ff5c1cc18a66efc76b5ab3e07c0fdf9582855db42f3897488c4a536` |
| `01_core/src/entities/page_store.rs` (55,71) | `ed33967f0de02648d0dd5c57755fde974c6e3c5ab7862ecab42923c2fe69586d` |
| `01_core/src/entities/sealed_positions.rs` (44) | `d8b98e705179982ceece2c61d504715a94f49b0f13720d89280076bcd3c30338` |

Esses hashes identificam a fonte medida, não autoridade de implementação e não evidência de sucesso das APIs futuras. O verificador deve exigir compilação/execução real FINAL e invalidar qualquer cobertura fictícia.
