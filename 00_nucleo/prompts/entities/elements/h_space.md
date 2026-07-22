# Prompt L0 — `entities/elements/h_space` — `HSpaceElem`
Hash do Código: b0dc0d79

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/h_space.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P320). Espaço
horizontal (`h(amount)`). Comportamento idêntico ao braço atual.

---

## Struct

**P842 (#38)** — `amount` passou de `Length` para o enum `Spacing`
(paridade vanilla `layout/spacing.rs`: `Absolute`/`Fractional`):

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spacing {
    Absolute(Length),   // h(10pt), h(2em)
    Fractional(f64),    // h(1fr) — expandida no flush_line/finish do Layouter
}

#[derive(Debug, Clone, PartialEq)]
pub struct HSpaceElem {
    pub amount: Spacing,
    pub weak:   bool,
}
```

`Content::HSpace { amount, weak }` → `Content::HSpace(Arc<HSpaceElem>)`.
Construtores ergonómicos: `Content::h_space(amount: Length, weak: bool)`
(→ `Absolute`) e `Content::h_space_fraction(fr: f64, weak: bool)`
(→ `Fractional`, P842). `Spacing::is_zero`: zero absoluto ou fração zero.

> **`Hash` manual** (precedente Lote 4 P319): `Length` carrega `f64` e não
> implementa `Hash` → `impl Hash { format!("{self:?}").hash(state) }` (paridade
> `content_hash`). Idem para `Spacing::Fractional` (`f64`, P842). Ressalva
> `-0.0` vs `0.0` em comentário no código; aceitável (`…Elem` não são chaves
> de mapa). Teste relacional de hash.

## `impl Element for HSpaceElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1786`) |
| `is_empty` | **override**: `self.amount.is_zero()` (P842: via `Spacing::is_zero`) |
| `map_content` | **terminal** |
| `map_text` | **terminal** |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq`

`#[derive(PartialEq)]` compara `amount + weak` (paridade `content.rs:1964`).

## Critério

`plain_text` vazio; `is_empty` quando `amount` zero (absoluto ou fração);
map_* terminais; `Hash` manual via Debug; igualdade por `amount+weak`;
`Spacing::Fractional` propagável até ao layout (P842).
