# L0 — `LayouterRuntimeState` (`entities/layouter_runtime_state.rs`)

## Módulo
`01_core/src/entities/layouter_runtime_state.rs`

**Histórico relevante**:
- 2026-05-04 (P190C): criada como parte do M6 — categoria Page tracking. Pattern arquitectural "Layouter-runtime → struct dedicada" 1ª aplicação.

## Propósito

Struct dedicada para state Layouter-runtime que **não é derivado de Content pre-pass**.

Diferente de `TagIntrospector` (state derivado de walk + from_tags pipeline), este struct é populated **durante o layout** pelo Layouter — campos têm semântica de página (`label_pages`, `known_page_numbers`) que só existem em runtime de render, não em introspecção.

## Campos

```rust
pub struct LayouterRuntimeState {
    /// Mapeia label para número de página onde foi resolvida.
    /// Populated por `references.rs` durante layout (write).
    pub label_pages: HashMap<Label, usize>,

    /// Page numbers conhecidos da iteração anterior do fixpoint.
    pub known_page_numbers: HashMap<Label, usize>,

    /// **P190D** — modo read-only do Layouter (DEBT-13).
    /// Set/unset por `outline.rs` em volta de render TOC entries.
    pub is_readonly: bool,
}
```

3 fields (após P190D). Field `lang: Option<Lang>` deferido — requer walk fn signature change (lido por `compute_labelled` durante walk pre-pass).

## Pattern arquitectural

**"Layouter-runtime → struct dedicada"** estabelecido em P190C.
Replicado em P190D para outros campos Layouter-runtime
(`is_readonly`, `lang`).

Razão: state que não cabe em Introspector (não derivado de Content)
não deve poluir `CounterStateLegacy` ou misturar-se com state
derivado de walk + from_tags.

## Consumers

- `rules/layout/references.rs:30` — escrita: `layouter.runtime.label_pages.insert(label, page)`.
- `rules/layout/outline.rs:51` — leitura: `layouter.runtime.known_page_numbers.get(&label)`.
- `rules/layout/mod.rs:1139` — leitura `label_pages` no fim de `finish()` para preencher `PagedDocument.extracted_label_pages`.
- `rules/layout/mod.rs:1535` — escrita `known_page_numbers` no fixpoint loop entre iterações.

## Cross-references

- **P190A §3 achado crítico** — 4 campos Layouter-runtime identificados
  como não cabendo em Introspector.
- **P190B** — pattern "eliminação write paralelo M5" 1ª aplicação
  (Bibliography); P190C 2ª aplicação introduz padrão Layouter-runtime.
- **P190D** — replicação do padrão para `is_readonly` + `lang`.
- **DEBT-12** — page tracking via `known_page_numbers` (Pass 3 final).
- **DEBT-13** — outline freeze via `is_readonly` (P190D).

## Pureza L1

Struct pura sem I/O. Apenas owns `HashMap`s. Acessível em testes via
`Default::default()`.
