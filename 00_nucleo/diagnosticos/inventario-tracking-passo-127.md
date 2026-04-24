# Passo 127.A — Inventário `text.tracking` (DEBT-1 subset)

**Data**: 2026-04-24

---

## Parte 1 — Tipo `Length` em L1

**Ficheiro**: `01_core/src/entities/layout_types.rs:527-545`.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    pub abs: Abs,    // componente absoluta (f64 em pt)
    pub em:  f64,    // componente relativa em múltiplos de font-size
}

impl Length {
    pub const ZERO: Self;
    pub fn pt(v: f64) -> Self;
    pub fn em(v: f64) -> Self;
    pub fn resolve_pt(&self, font_size_pt: f64) -> f64;
}
```

- **Copy** — pode viver em `Option<Length>` sem custo.
- Fiel ao vanilla (`struct Length { abs: Abs, em: f64 }`).
- ADR-0029 documenta decisão.

## Parte 2 — `Value::Length`

**Ficheiro**: `01_core/src/entities/value.rs:53`.

```rust
Length(crate::entities::layout_types::Length),
```

Variante directa — `if let Value::Length(l) = val { ... }` sem
cast intermédio. `impl From<Length> for Value` disponível.

## Parte 3 — Como outros arms usam `Length`

`size` em `rules/eval/rules.rs:284-288`:
```rust
"size" => {
    if let Value::Length(l) = val {
        delta.size = Some(l.abs.to_pt());  // só abs, perde em
    }
}
```

`size` perde componente `em` — decisão legada (DEBT-1 original).
Este passo **não** replica essa perda — preserva `Length` inteiro
em `StyleDelta.tracking: Option<Length>`. Quando consumer chegar,
`resolve_pt(font_size)` recupera valor final.

## Parte 4 — Vanilla tracking

`lab/.../typst-library/src/text/mod.rs:303`:
```rust
pub tracking: Length,
```

Default não visível nesta linha mas provavelmente `Length::ZERO`.
Semântica: espaçamento adicional entre glyphs; negativo aperta,
positivo espaça.

## Parte 5 — Teste DEBT-49 actual (L3)

`03_infra/src/integration_tests.rs:2222` — input actual:
```rust
#"#set text(font: \"A\", lang: \"pt\", stroke: 1pt)"#
```

**Tracking **não aparece**. Zero rotação necessária.

---

## Decisões

| Dimensão | Escolha | Razão |
|----------|---------|-------|
| Tipo T | **`Option<Length>`** | Preserva `abs + em`; `Length: Copy`; zero lifetime; matches vanilla |
| Arm cast | `Value::Length(l) => delta.tracking = Some(l)` | Sem `.abs.to_pt()` — não perder precisão |
| Range / default | Sem validação / sem default | Coerente com pattern 126 (aceita raw) |
| ADR-0038 | **Anotar** | Primeira propriedade a preservar `Length` inteiro; pattern extende do 126 (numérico simples) para semântico |
| Rotação DEBT-49 | **Não necessária** | Input já usa font/lang/stroke |

## Gate 127.A

**Passa**. 2 ficheiros L1:
- `entities/style_chain.rs` (+1 campo).
- `rules/eval/rules.rs` (+1 arm).

Sem ripple em `StyleChain` resolvers, `push_styles`, consumers,
layout ou export. XS confirmado.
