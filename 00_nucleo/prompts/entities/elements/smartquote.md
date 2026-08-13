# Prompt L0 — `entities/elements/smartquote` — `SmartQuoteElem`
Hash do Código: 62c6d0c9

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/smartquote.rs`
**Origem**: modelo D (ADR-0105), **Lote 9 P324** (por largura). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável**. Leaf — `map_*` terminais.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuoteElem {
    pub double: bool,
}
```

`Content::SmartQuote { double }` → `Content::SmartQuote(Arc<SmartQuoteElem>)`.
Construtor ergonómico: `Content::smartquote(double: bool)`. **Deriva `Hash`/`Eq`**
(`bool`).

## `impl Element for SmartQuoteElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `if self.double { "\"".to_string() } else { "'".to_string() }` (`content.rs:1722`) |
| `is_empty` | default `false` (`content.rs:1662`) |
| `map_content`/`map_text` | **terminais** (leaf) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `double` (paridade `content.rs:1992`).

> **Arm `|`-combinado**: `SmartQuote` está no arm terminal combinado de
> `map_content`/`map_text` com outros leaves. Como é terminal (sem binding),
> fica **no mesmo arm** (`{ .. }` → `(_)`) — **sem split**.
