# Prompt L0 — `entities/elements/cite` — `CiteElem`
Hash do Código: c74c52e4

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/cite.rs`
**Origem**: modelo D (ADR-0105), **Lote 9 P324** (por largura). Trait: ver
`entities/elements/_comum.md`. Contentor parcial — `map_*` recursam no
`supplement`.

> **Fronteira: LOCATÁVEL** (M1, junto de Heading/Figure). Absorve o braço de
> `extract_payload` no trait (precedente Heading/Lote 6): `element_kind` →
> `ElementKind::Citation`, `to_payload` → `ElementPayload::Citation { key }`. O
> consumo por `ElementPayload` (`from_tags`/walk) é **inalterado** (matcheia o
> payload, não o `Content`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CiteElem {
    pub key:        String,
    pub supplement: Option<Content>,           // era Option<Box<Content>>
    pub form:       Option<CitationForm>,
}
```

`Content::Cite { key, supplement, form }` → `Content::Cite(Arc<CiteElem>)`.
Construtor ergonómico: `Content::cite(key, supplement, form)`.

> **`Hash` por derive — dependência do lote** (precedente `Parity` P320). Os
> campos: `String` (`Hash`), `Option<Content>` (`Content` tem `impl Hash`
> manual), `Option<CitationForm>` (precisa `Hash`). `CitationForm` é
> `Copy + Eq` com 4 variantes unit (`Normal`/`Prose`/`Author`/`Year`) — **sem
> floats** → seguro para `Hash` por derive (regra do modelo: derive quando o
> `Hash` canónico é seguro). **Ação**: adicionar `Hash` ao derive de
> `CitationForm` (`citation_form.rs:36`) e atualizar o L0
> `entities/citation_form.md` (linha 37 especifica os derives). Registado como
> **dependência do lote**, não conserto oportunista.

## `impl Element for CiteElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `let mut out = format!("[{}]", self.key); if let Some(s) = &self.supplement { out.push_str(&s.plain_text()); } out` (`content.rs:1811`) |
| `is_empty` | default `false` (`content.rs:1643`) |
| `map_content` | **recursivo** no `supplement`, preserva `key`/`form` (`content.rs:2332`) |
| `map_text` | **recursivo** no `supplement`, preserva `key`/`form` (`content.rs:2601`) |
| `get_field` | default `None` |
| `element_kind` | `Some(ElementKind::Citation)` |
| `to_payload` | `Some(ElementPayload::Citation { key: self.key.clone() })` (absorve `extract_payload.rs:33`) |

## `eq`

`#[derive(PartialEq)]` compara `key`/`supplement`/`form` (paridade
`content.rs:1963`).

---

## P418 (XL) — Renderização CSL real

**Decisão arquitetural (ADR-0107 / ADR-0108 / ADR-0109):**
- `CiteElem` mantém `key`/`supplement`/`form`; a formatação real via hayagriva CSL vive em `rules/layout/cite.rs` (forma B).
- `Introspector::bib_entry_for_key` / `bib_number_for_key` continuam como lookup; P418 pode enriquecer com dados hayagriva se necessário.
- Forward references funcionam porque o `Introspector` é populado durante o walk (`from_tags`) antes do layout.

**Scope-out P418:**
- `form` avançado beyond `Normal`/`Prose`/`Author`/`Year`; supplement com formatação CSL nativa (render simples `, supp`).

## Histórico de revisões

| Data | Motivo | Arquivos |
|------|--------|----------|
| 2026-06-23 | P418 (XL): adicionar seção de renderização CSL real e scope-out. | `cite.md`, `cite.rs`, `bibliography.md`, `bibliography.rs`, `loading.md` |
