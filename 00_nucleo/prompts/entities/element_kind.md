# Prompt L0 — `entities/element_kind`
Hash do Código: 8502f5bf

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/element_kind.rs`
**Criado em**: 2026-04-30 (P161 sub-passo .5)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`ElementKind` é o discriminador estreito dos tipos de elemento que entram no índice de introspecção. Apenas três variantes em M1: `Heading`, `Figure`, `Citation`. Outras kinds (Equation, Footnote, ListItem, etc.) ficam para M9 ou para os passos correspondentes em que essas features forem ligadas ao motor.

Forma análoga (mas muito reduzida) à hierarquia de elements vanilla. Vanilla usa `Element` (struct dinâmico com vtable proc-macro). Cristalino usa um enum fechado pequeno — coerente com a topologia de `Content` (enum em vez de vtable).

---

## Restrições Estruturais

- Camada **L1**: enum puro, `Copy`, sem alocação.
- Apenas 3 variantes em P161.
- Sem campos por variante — `ElementKind` é só o discriminador. Os detalhes específicos (level, kind string, key) vão em `ElementPayload`.

---

## Interface pública

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementKind {
    Heading,
    Figure,
    Citation,
    /// **P169 (M9 sub-passo 1)** — feature `metadata(value)` vanilla.
    Metadata,
    /// **P171 (M9 sub-passo 3)** — `state(key, init)` runtime state.
    State,
    /// **P171 (M9 sub-passo 3)** — `state.update(key, value)` runtime update.
    StateUpdate,
    /// **P178** — `Content::Outline` indexável; fecha lacuna #7 (`has_outline`).
    Outline,
    /// **P181C** — `Content::Bibliography` promovido a locatable em
    /// P181D. Suporta plano P181 para fechar lacuna #6
    /// (`bib_entries`/`bib_numbers`); `from_tags` arm correspondente
    /// é adicionada em P181E e popula `BibStore` (P181B).
    Bibliography,
    /// **P186B** — `Content::Equation` promovido a locatable em
    /// P186C/D/E. Indexa locations de equações em `kind_index` para
    /// `query_by_kind(Equation)`. Suporta C2 (equation counter)
    /// desbloqueio per ADR-0068; consumer migra em P188. `from_tags`
    /// arm Equation (P186E) gate `block && state.numbering_active:equation`
    /// — dormente em produção até `Content::SetEquationNumbering`
    /// materializar (P186A §11.2).
    Equation,
    /// **P198C** — `Content::CounterUpdate` promovido a locatable
    /// (cenário β-promote ADR-0069). Indexa locations de CounterUpdate
    /// em `kind_index`; `from_tags` arm aplica à CounterRegistry.
    CounterUpdate,
    /// **P461** — `Content::Table` promovido a locatable. Indexa
    /// locations de tables em `kind_index`; counter flat `"table"`
    /// populado quando numbering+caption activos.
    Table,
    /// **P494** — Selector `list`. Cristalino não materializa
    /// `Content::List` (lista é `Sequence` de `ListItem`), por isso
    /// `kind_index` fica vazio; a contagem é feita em L3 por análise
    /// do `Content` (`query_helpers.rs`). O kind existe em L1 para
    /// permitir `Selector::Kind(List)` e paridade de parse.
    List,
    /// **P494** — Selector `enum`. Cristalino não materializa
    /// `Content::Enum` (enum é `Sequence` de `EnumItem`); contagem em
    /// L3 por análise do `Content`.
    Enum,
    /// **P494** — Selector `par`. Cristalino não materializa
    /// `Content::Par` (parágrafo é texto plano numa `Sequence`);
    /// contagem aproximada em L3 (presença de texto no documento).
    Par,
    /// **P494** — Selector `link`. `Content::Link` existe em L1;
    /// contagem por análise do `Content` em L3 (alternativa: tornar
    /// locatable em passo futuro).
    Link,
    /// **P494** — Selector `raw`. `Content::Raw` existe em L1;
    /// contagem por análise do `Content` em L3.
    Raw,
    /// **P494** — Selector `quote`. `Content::Quote` existe em L1;
    /// contagem por análise do `Content` em L3.
    Quote,
    /// **P494** — Selector `footnote`. `Content::Footnote` existe em
    /// L1; contagem por análise do `Content` em L3.
    Footnote,
    /// **P240 (M9d/M7+1)** — `Content::StateDisplay` promovido a
    /// locatable. Indexa locations de StateDisplay em `kind_index`;
    /// valor pre-rendered é produzido em `apply_state_displays`
    /// pós-fixpoint (paralelo `apply_state_funcs` P191B).
    StateDisplay,
    /// **P241 (M9d/M7+2)** — `Content::CounterDisplayCallback`
    /// promovido a locatable paralelo absoluto `StateDisplay` P240.
    /// Indexa locations em `kind_index`; valor pre-rendered produzido
    /// em `apply_counter_displays` pós-fixpoint.
    CounterDisplay,
}

impl ElementKind {
    /// Forma textual estável (para diagnóstico e debug).
    pub fn as_str(self) -> &'static str;
}
```

---

## Semântica

- `as_str()`: retorna `"heading"`, `"figure"`, `"citation"`, etc. Use case: chave em mapas se `ElementKind` for usado como `&str` (e.g. selectores). Não usar para reconstrução do enum — a reconstrução é via `from_name` (P175), não `from_str`.
- `Hash`/`Eq`/`Copy` derivados — usável como chave em `HashMap<ElementKind, T>`.
- **P494**: Kinds `List`, `Enum`, `Par`, `Link`, `Raw`, `Quote`, `Footnote` existem em L1 para permitir `Selector::Kind(...)` e parsing de selectores vanilla, mas a sua contagem em queries de introspecção é realizada em L3 (`query_helpers.rs`) por análise do `Content`, porque cristalino não materializa os containers `List`/`Enum`/`Par` (usa `Sequence` de itens / texto plano).

---

## Invariantes

- **Adicionar variantes nova exige passo dedicado** com inventário do consumer e ADR justificativa quando a feature correspondente for activada.
- Sem variantes catch-all (`Other`, `Unknown`).
- **P494**: Kinds `List`/`Enum`/`Par`/`Link`/`Raw`/`Quote`/`Footnote` são adicionados como discriminadores de selector válidos; a correspondência com elementos concretos do `Content` é resolvida em L3, não obrigatoriamente via `kind_index` em L1.

---

## Consumers actuais

- `entities/selector.rs` — `Selector::Kind(ElementKind)`.
- `entities/introspector.rs` — `query_by_kind`, `query_first`, `query_unique`.
- `rules/stdlib/foundations.rs` — `parse_selector_arg` converte string → `Selector::Kind`.
- `03_infra/query_helpers.rs` — parsing de selectors para comparação vanilla; P494 adiciona contagem por análise do `Content` para os novos kinds.

## Consumers planeados

- `entities/element_payload.rs` — cada variante locatable de `ElementPayload` corresponde a um `ElementKind`.
- `rules/introspect.rs` walk — branches locatable constroem o `ElementKind` apropriado.
- `Introspector` — usar como discriminador para queries por tipo.

---

## Sobre paridade

Vanilla não tem `ElementKind` enum. Tem `Element` (struct + vtable), `Selector::Elem(Element)`, e elementos individuais como `HeadingElem`, `FigureElem`, `CiteElem`. Cristalino simplifica para um enum estreito porque:

1. O número de tipos relevantes para introspecção é pequeno e fechado (não é arbitrário).
2. Coerência com `Content` (enum vs vtable).
3. Selectores futuros (M5+) podem usar `ElementKind` como filtro discreto, evitando o type-erasure vanilla.

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) §3: `Locatable`/`Tagged`/`Unqueriable` traits vanilla são scope-out porque cristalino não usa marker-traits-per-element.

---

## Resultado Esperado

- `01_core/src/entities/element_kind.rs` — enum + 1 método + tests unitários (igualdade, as_str, hash semantics).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P161 sub-passo .5: discriminador estreito de elementos para Introspection M1 | `element_kind.rs`, `element_kind.md` |
| 2026-04-29 | P178: variant `Outline` adicionada; fecha lacuna #7 (`has_outline` via `query("outline")`) | `element_kind.rs`, `element_kind.md` |
| 2026-05-01 | P181C: variant `Bibliography` adicionada; suporte ao plano P181 (lacuna #6); `from_name("bibliography")` round-trip | `element_kind.rs`, `element_kind.md` |
| 2026-05-03 | P186B: variant `Equation` adicionada; suporte ao plano P186 (eixo 2 do bloqueio P183C — C2 equation counter); `from_name("equation")` round-trip; `from_tags` arm Equation populará `kind_index` em P186E. Dormente em produção até `Content::SetEquationNumbering` materializar (passo dedicado fora da série P186-P188). | `element_kind.rs`, `element_kind.md` |
| 2026-06-29 | P494: adicionadas variants `List`, `Enum`, `Par`, `Link`, `Raw`, `Quote`, `Footnote` para paridade de selectores de documento. Kinds `List`/`Enum`/`Par` mapeiam para estruturas implícitas de `Content` (`Sequence` de itens / texto); contagem em L3 `query_helpers.rs`. Kinds `Link`/`Raw`/`Quote`/`Footnote` mapeiam para variantes existentes de `Content`. | `element_kind.rs`, `element_kind.md`, `selector.rs`, `query_helpers.rs`, `foundations.rs` |
