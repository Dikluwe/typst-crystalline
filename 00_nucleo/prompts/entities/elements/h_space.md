# Prompt L0 — `entities/elements/h_space` — `HSpaceElem`
Hash do Código: e7c196ce

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/h_space.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait e glossário (§A.0): ver
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
    pub amount:        Spacing,
    pub weak:          bool,
    pub weak_explicit: bool,
}
```

`Content::HSpace { amount, weak }` → `Content::HSpace(Arc<HSpaceElem>)`.
Construtores ergonómicos: `Content::h_space(amount: Length, weak: bool)`
(→ `Absolute`) e `Content::h_space_fraction(fr: f64, weak: bool)`
(→ `Fractional`, P842). `Spacing::is_zero`: zero absoluto ou fração zero.

**P1140.9 — contrato público e presença.** `weak_explicit` preserva se o named
`weak` apareceu na chamada Typst, inclusive `weak: false`, para que `repr` não
perca informação observável. Os construtores ergonómicos existentes preservam
a assinatura: tratam `weak = true` como explícito e `weak = false` como default
omitido. Um construtor adicional, nomeado explicitamente como
`h_space_with_weak_presence` (e equivalente fracionário), recebe o bit usado
pelo caminho de eval. Layout consulta somente `weak`.

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

`#[derive(PartialEq)]` compara `amount + weak + weak_explicit`: valores com
`repr` diferente são distinguíveis na linguagem. O `Hash` manual inclui o novo
campo por derivar da representação `Debug` completa.

## Critério

`plain_text` vazio; `is_empty` quando `amount` zero (absoluto ou fração);
map_* terminais; `Hash` manual via Debug; igualdade por
`amount+weak+weak_explicit`;
`Spacing::Fractional` propagável até ao layout (P842).
