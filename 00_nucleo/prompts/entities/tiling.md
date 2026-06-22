# Prompt L0 — `entities/tiling` — padrão de azulejos (Tiling)
Hash do Código: e3da60bf

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/tiling.rs`
**Origem**: Passo 395 — modelagem de tipos (M); abertura do portão ADR-0017.
**ADRs**: ADR-0017 (enum fechado; variant novo exige tipo migrado), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (graded scope-out).

---

## 1. Contexto

O vanilla expõe `tiling(...)` como construtor de padrão de preenchimento (pattern fill) para gradientes e imagens. A morfologia linguagem é:

```typst
#let t = tiling(image("pat.png"), size: auto, relative: "self")
#box(fill: t)
```

Este passo **modela apenas o tipo** `Tiling` em L1; o construtor `tiling()` é P396.

## 2. Estrutura

```rust
pub struct Tiling {
    pub body: TilingBody,
    pub size: Option<Size>,
    pub relative: TilingRelative,
    pub spacing: Option<Size>,
}

pub enum TilingBody {
    Image(ImageElem),
    Gradient(Gradient),
    Color(Color),
}

pub enum TilingRelative {
    Self,
    Parent,
}
```

- `body`: corpo do padrão — imagem, gradiente ou cor sólida.
- `size`: tamanho da célula do padrão; `None` ↔ `auto` (bounds do body).
- `relative`: `"self"` (default) ou `"parent"`.
- `spacing`: gap entre repetições; `None` ↔ zero.

## 3. Decisões

- `TilingBody::Gradient` é **placeholder** — o tipo `Gradient` existe (P262) mas o consumer real de Tiling+Gradient é scope-out ADR-0054. O variant existe para paridade futura.
- `TilingBody::Image` usa `ImageElem` já existente em L1.
- `TilingBody::Color` é o fallback simples e único consumer inicial.
- Derives: `Debug`, `Clone`, `PartialEq`.
- Constructor: `Tiling::new(body: TilingBody) -> Self` com defaults (`size: None`, `relative: Self`, `spacing: None`).

## 4. Integração Paint / Fill

`Tiling` integra no enum `Paint` de `entities/paint.rs` como `Paint::Tiling(Tiling)`, lado a lado com `Solid` e `Gradient`. O fallback de cor (`Paint::to_color`) resolve:

- `TilingBody::Color(c)` → `c`.
- `TilingBody::Gradient(g)` → `g.first_stop_color()`.
- `TilingBody::Image(_)` → cor preta (scope-out graded — pattern fill de imagem ainda não renderizado).

## 5. Scope-out

- `tiling()` função nativa — P396.
- Render PDF de pattern fill — scope-out ADR-0054 graded.
- `TilingBody::Gradient` consumer real — scope-out.
- `TilingBody::Image` consumer real — scope-out.

## 6. Testes

- `tiling_new_color` — construção com cor.
- `tiling_new_image` — construção com `ImageElem` mock.
- `tiling_equality` — PartialEq.
- `tiling_clone` — Arc clone O(1) por body image compartilhado.
- `tiling_relative_default` — `Self` default.
- `tiling_size_none` — auto default.
- `paint_tiling_to_color` — fallback Color via `Paint::to_color`.
