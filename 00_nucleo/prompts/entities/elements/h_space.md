# Prompt L0 — `entities/elements/h_space` — `HSpaceElem`
Hash do Código: c66dbb77

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/h_space.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait: ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P320). Espaço
horizontal (`h(amount)`). Comportamento idêntico ao braço atual.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct HSpaceElem {
    pub amount: Length,
    pub weak:   bool,
}
```

`Content::HSpace { amount, weak }` → `Content::HSpace(Arc<HSpaceElem>)`.
Construtor ergonómico preservado: `Content::h_space(amount: Length, weak: bool)`.

> **`Hash` manual** (precedente Lote 4 P319): `Length` carrega `f64` e não
> implementa `Hash` → `impl Hash { format!("{self:?}").hash(state) }` (paridade
> `content_hash`). Ressalva `-0.0` vs `0.0` em comentário no código; aceitável
> (`…Elem` não são chaves de mapa). Teste relacional de hash.

## `impl Element for HSpaceElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1786`) |
| `is_empty` | **override**: `self.amount.is_zero()` (`content.rs:1610`) |
| `map_content` | **terminal** |
| `map_text` | **terminal** |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq`

`#[derive(PartialEq)]` compara `amount + weak` (paridade `content.rs:1964`).

## Critério

`plain_text` vazio; `is_empty` quando `amount` zero; map_* terminais; `Hash`
manual via Debug; igualdade por `amount+weak`.
