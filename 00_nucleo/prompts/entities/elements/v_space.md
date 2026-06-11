# Prompt L0 — `entities/elements/v_space` — `VSpaceElem`
Hash do Código: de4e9edc

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/v_space.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P320). Espaço
vertical (`v(amount)`). Comportamento idêntico ao braço atual.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct VSpaceElem {
    pub amount: Length,
    pub weak:   bool,
}
```

`Content::VSpace { amount, weak }` → `Content::VSpace(Arc<VSpaceElem>)`.
Construtor ergonómico preservado: `Content::v_space(amount: Length, weak: bool)`.

> **`Hash` manual** (igual a `h_space.md` / precedente Lote 4): `Length` carrega
> `f64` → `impl Hash { format!("{self:?}").hash(state) }`. Ressalva em comentário;
> teste relacional de hash.

## `impl Element for VSpaceElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1786`) |
| `is_empty` | **override**: `self.amount.is_zero()` (`content.rs:1611`) |
| `map_content` | **terminal** |
| `map_text` | **terminal** |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq`

`#[derive(PartialEq)]` compara `amount + weak` (paridade `content.rs:1966`).

## Critério

`plain_text` vazio; `is_empty` quando `amount` zero; map_* terminais; `Hash`
manual via Debug; igualdade por `amount+weak`.
