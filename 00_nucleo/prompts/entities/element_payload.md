# Prompt L0 — `entities/element_payload`
Hash do Código: c49a4e16

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/element_payload.rs`
**Criado em**: 2026-04-30 (P161 sub-passo .7)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`ElementPayload` é a forma fechada e tipada dos dados específicos de cada elemento indexado pela introspecção. Uma variante por kind (`Heading`, `Figure`, `Citation`), com os campos exactos que o motor de introspecção precisa para cada um.

P161 sub-passo .1 confirmou os campos das variantes correspondentes em `Content`:

| Variant Content | Campos relevantes confirmados em content.rs |
|-----------------|---------------------------------------------|
| `Content::Heading` | `level: u8`, `body: Box<Content>` |
| `Content::Figure` | `body, caption: Option<Box<Content>>, kind: Option<String>, numbering: Option<String>` |
| `Content::Cite` | `key: String, supplement: Option<Box<Content>>, form: Option<CitationForm>` |

Adicionalmente:
- `body_hash` em `Heading` é populado pela função `hash_content` (em `entities/content_hash.rs`, P162 sub-passo .B) chamada por `extract_payload` (em `rules/introspect/extract_payload.rs`, P162 sub-passo .D). Pendência placeholder de P161 resolvida em P162.

---

## Restrições Estruturais

- Camada **L1**: enum puro.
- Sem referências para `Content` directamente (clones-pesados); apenas hashes (`u128`) e cópias leves (`u8`, `String`, `CounterUpdate`, `Option<…>`).
- `Clone` derivado para passar entre walk → tag → registry sem `Arc`.

---

## Interface pública

```rust
use crate::entities::counter_update::CounterUpdate;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementPayload {
    Heading {
        /// Nível do heading (1..=6 após clamp). Paridade com `Content::Heading.level`.
        depth: u8,

        /// Hash determinístico do `body` do heading (Box<Content>).
        /// Populado por `hash_content(body)` em `extract_payload`
        /// (P162 .D). Identifica univocamente o conteúdo do body
        /// para detecção de mudanças cross-iteration.
        body_hash: u128,

        /// Update implícito do contador "heading" associado a este nó.
        /// Tipicamente `CounterUpdate::Step` (avança ao nível `depth`).
        counter_update: CounterUpdate,
    },

    Figure {
        /// Discriminador do tipo de figura — `"image"` / `"table"` / `"raw"` / etc.
        /// Em paridade com `Content::Figure.kind: Option<String>`.
        /// `None` ↔ Auto (resolver no consumer via `kind.as_deref().unwrap_or("image")`).
        kind: Option<String>,

        /// Update implícito do contador da figura (kind-discriminado).
        counter_update: CounterUpdate,

        /// **P168 (M5 sub-passo 2)**: `true` se figura conta para
        /// numeração — predicado `figure.numbering.is_some() && figure.caption.is_some()`.
        /// Permite a `from_tags` indexar apenas figuras numeradas para
        /// `figure_label_numbers`, em paridade com walk arm `Content::Labelled`
        /// que aplica o mesmo filtro no `CounterStateLegacy.figure_label_numbers`.
        is_counted: bool,
    },

    Citation {
        /// Chave da citação. Paridade com `Content::Cite.key`.
        key: String,
    },

    /// **P169 (M9 sub-passo 1)** — payload de `metadata(value)`.
    /// `Box<Value>` para evitar cycle de tamanhos (Value contém Content
    /// que poderia conter Metadata via `Content::Metadata`).
    /// Consumer: `MetadataStore` populado por `from_tags`.
    Metadata {
        value: Box<Value>,
    },
}
```

**Nota sobre derives** (P169): `ElementPayload` deixou de derivar `Eq, Hash`
porque `Value` (em `Metadata` variant) não impl `Eq` (f64 NaN). `Hash`
é implementado manualmente via `format!("{:?}", self).hash()` (estratégia
consistente com `entities::content_hash::hash_content`). `Eq` é
declarada via `impl Eq for ElementPayload {}` (white-lie consistente
com PartialEq derive de Value, que tem mesma issue).

---

## Semântica

- `Heading`: representa um heading que será indexado pela introspecção. `depth` é o nível clamped (1–6). `body_hash` permite que o registry detecte alterações ao corpo entre iterações sem clonar `Content` inteiro (essencial para o fixpoint M2+). `counter_update` regista que tipo de update foi aplicado ao counter "heading".
- `Figure`: representa uma figura. `kind` é o discriminador para contadores independentes por tipo. `counter_update` regista o step do counter "figure:{kind}".
- `Citation`: representa uma citação. Apenas a `key` é relevante para introspecção (resolução para bib_entry posterior, em M9 ou semelhante).

---

## Invariantes

- Apenas 3 variantes em P161 — coerente com `ElementKind`.
- Cada variante tem **apenas** os campos confirmados em sub-passo .1. Não adicionar campos especulativos (e.g. `Heading.label`, `Heading.numbering`) — esses ficam em `ElementInfo` (label) ou são derivados pelo Layouter (numbering).
- `body_hash` em `Heading` é populado por `hash_content` (P162 .B); em P161 era placeholder `0`, resolvido em P162 .D quando `extract_payload` o chama.
- `Hash` derivado: necessário para o fixpoint detectar convergência.
- `Eq` derivado: igualdade exige todos os campos iguais.

---

## Consumers actuais

Nenhum em P161 — `ElementPayload` é infraestrutura passiva.

## Consumers planeados

- `entities/element_info.rs` (P161 sub-passo .8) — wrap `ElementPayload` + `Option<Label>`.
- `entities/tag.rs::Tag::Start(Location, ElementInfo)` (P161 sub-passo .9).
- `rules/introspect.rs` walk em P162 — constrói `ElementPayload` para cada Heading/Figure/Cite encontrado, emite `Tag::Start(loc, ElementInfo { payload, label })`.

---

## Sobre paridade

Vanilla não tem `ElementPayload` enum. A informação equivalente é distribuída por:
- `HeadingElem` struct com fields `level`, `body`, `numbering`, `supplement`, etc.
- `FigureElem` struct com fields `body`, `caption`, `kind`, `numbering`, `supplement`, etc.
- `CiteElem` struct com fields `key`, `supplement`, `form`, `style`.

Cristalino agrega num enum estreito por dois motivos:

1. Coerência com `Content` (enum) e `ElementKind` (enum) — sem proc-macros vtable.
2. Subset deliberado: só os campos que o motor de introspecção precisa de observar. Outros campos (numbering pattern, supplement) são responsabilidade do Layouter, não do Introspector.

Ver `desenho-introspection-fixpoint.md` §2.1 (referenciado em P161; documento ainda por localizar/produzir — registado como lacuna em `inventario-tipos-introspection-vanilla.md` 2026-04-30) para o contexto de design.

---

## Resultado Esperado

- `01_core/src/entities/element_payload.rs` — enum + tests unitários (construção de cada variante, igualdade, hash, clone).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P161 sub-passo .7: forma fechada por kind para introspecção M1 | `element_payload.rs`, `element_payload.md` |
