# Prompt L0 — `entities/element_kind`
Hash do Código: 4421c65e

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
}

impl ElementKind {
    /// Forma textual estável (para diagnóstico e debug).
    pub fn as_str(self) -> &'static str;
}
```

---

## Semântica

- `as_str()`: retorna `"heading"`, `"figure"`, ou `"citation"`. Use case: chave em mapas se `ElementKind` for usado como `&str` (e.g. selectores futuros). Não usar para reconstrução do enum — sem `from_str`.
- `Hash`/`Eq`/`Copy` derivados — usável como chave em `HashMap<ElementKind, T>`.

---

## Invariantes

- Apenas 3 variantes em P161. **Adicionar variantes nova exige passo dedicado** com inventário do consumer e ADR justificativa quando a feature correspondente for activada.
- Sem variantes catch-all (`Other`, `Unknown`).

---

## Consumers actuais

Nenhum em P161. Apenas listado em re-exports.

## Consumers planeados

- `entities/element_payload.rs` (P161 sub-passo .7) — cada variante de `ElementPayload` corresponde a um `ElementKind`.
- `rules/introspect.rs` walk em P162 — branches `Content::Heading` / `Content::Figure` / `Content::Cite` constroem o `ElementKind` apropriado.
- `Introspector` em M3 — usar como discriminador para queries por tipo.

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
