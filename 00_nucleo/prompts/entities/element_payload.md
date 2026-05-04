# Prompt L0 — `entities/element_payload`
Hash do Código: 2b440e36

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

    /// **P171 (M9 sub-passo 3)** — payload de `state(key, init)`.
    /// Init value para state runtime; populado em `StateRegistry::init`.
    State {
        key:  String,
        init: Box<Value>,
    },

    /// **P171 (M9 sub-passo 3)** — payload de `state.update(key, value)`.
    /// Aplicado em `StateRegistry::apply_update`. Apenas Set variant
    /// em P171; Func adiada.
    StateUpdate {
        key:    String,
        update: StateUpdate,
    },

    /// **P178** — payload de `Content::Outline`. Variant unit em P178
    /// (Opção α): `Content::Outline` é unit, e `query("outline")`
    /// minimal só precisa contar locations. Refino futuro pode capturar
    /// `depth: Option<usize>` e `title: hash` para queries mais ricas.
    Outline,

    /// **P181C** — payload de `Content::Bibliography`. Carrega
    /// entries completos (decisão P181A cláusula 2 — captura full por
    /// simetria com walk arm actual `state.bib_entries.extend(...)`);
    /// `from_tags` arm Bibliography (P181E pendente) extrai `entries`
    /// e popula `BibStore` via `add_bibliography(entries) +
    /// assign_number(key, n)` em loop. `BibEntry` deriva `Debug` —
    /// `impl Hash` manual de `ElementPayload` via `format!("{:?}", ...)`
    /// cobre a variant sem alteração de código.
    Bibliography {
        entries: Vec<BibEntry>,
    },

    /// **P186B** — payload de `Content::Equation`. Forma paralela a
    /// `Figure` (P184B) com `block` (display vs inline) +
    /// `counter_update` (sempre `Step` enquanto não houver equation
    /// set rule). `from_tags` arm Equation (P186E) popula
    /// `CounterRegistry` sob chave `"equation"` quando
    /// `block && state.value_at("numbering_active:equation", loc)
    /// == Some(Bool(true))`. Sem cláusula `is_counted` — equations
    /// não têm o predicado caption-based de figures; numbering
    /// activo é controlado externamente via state. Em produção
    /// (sem `Content::SetEquationNumbering`), gate nunca dispara →
    /// counter introspector vazio → P188 substitution-with-fallback
    /// cobre via legacy.
    Equation {
        block:          bool,
        counter_update: CounterUpdate,
    },

    /// **P195B** — payload de `Content::Labelled` emitido em
    /// **post-recursion** pelo walk arm (per ADR-0069). Pattern
    /// arquitectural novo distinto dos outros variants: estes vêm
    /// de `extract_payload` puro pre-recursion; `Labelled` é
    /// produzido directamente pelo walk arm após recursão no target
    /// porque `resolved_text` depende de state mutado durante walk
    /// recursivo (counter formatting via `state.format_hierarchical`,
    /// `state.get_flat`, `state.figure_numbers`, `state.lang`).
    ///
    /// Campos:
    /// - `label: Label`: chave para `intr.resolved_labels` populate.
    /// - `resolved_text: Option<String>`: texto pré-computed
    ///   ("Secção 1.2", "Equação (3)", "Figura 5"); `None` para
    ///   target types não-resolvíveis (catch-all `_ => None` em
    ///   walk arm legacy).
    /// - `figure_number: Option<usize>`: `Some(n)` apenas quando
    ///   target é Figure numerada+captioned. Permite popular
    ///   `intr.figure_label_numbers` em paralelo com P168 arm
    ///   Figure (write redundante mas inofensivo).
    ///
    /// `from_tags` arm Labelled (P195C) popula ambos sub-stores.
    /// Walk arm legacy (E4 P189B) **mantém** mutação directa em
    /// `state.resolved_labels` + `state.figure_label_numbers`
    /// durante janela compat M5; E4 fecha estruturalmente em P195;
    /// funcionalmente em M6.
    Labelled {
        label:         Label,
        resolved_text: Option<String>,
        figure_number: Option<usize>,
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
| 2026-04-29 | P178: variant `Outline` unit adicionada para suporte de `query("outline")` | `element_payload.rs`, `element_payload.md` |
| 2026-05-03 | P186B: variant `Equation { block, counter_update }` adicionada (forma paralela a `Figure` P184B); suporta P186 plano (eixo 2 P183C); P186D adiciona arm em `extract_payload`, P186E adiciona arm em `from_tags` com gate `block && state numbering_active:equation`. | `element_payload.rs`, `element_payload.md` |
| 2026-05-01 | P181C: variant `Bibliography { entries: Vec<BibEntry> }` adicionada; suporta P181D (`extract_payload` arm Bibliography) e P181E (`from_tags` popula `BibStore`) | `element_payload.rs`, `element_payload.md` |
| 2026-05-04 | P195B: variant `Labelled { label, resolved_text, figure_number }` adicionada com pattern arquitectural novo "post-recursion tag emission for state-dependent payload" (ADR-0069 PROPOSTO). **Sem** `extract_payload` arm — payload depende de state mutado durante walk recursivo, impossível em função pura. Walk arm Labelled (P195D) emite Tag manualmente após recursão. `from_tags` arm popula `intr.resolved_labels` + `intr.figure_label_numbers`. P195B = stub no-op em from_tags; P195C estende. | `element_payload.rs`, `element_payload.md`, `from_tags.rs`, `from_tags.md`, `typst-adr-0069-post-recursion-tag-emission.md` |
