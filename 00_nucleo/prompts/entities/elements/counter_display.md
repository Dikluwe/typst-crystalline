# Prompt L0 — `entities/elements/counter_display` — `CounterDisplayElem`
Hash do Código: db3bf8c7

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/counter_display.rs`
**Origem**: modelo D (ADR-0105), **Lote 6 P321** (família state/counter). Trait e glossário (§A.0):
ver `entities/elements/_comum.md`. Comportamento idêntico ao braço atual.

> **Fronteira: NÃO-locatável** (legacy single-pass). É a única não-locatável do
> lote — `CounterDisplay { kind }` resolve no Layouter directo (DEBT-10), sem
> tag de introspecção. `element_kind`/`to_payload` ficam no default `None`.
> (Distinta de `CounterDisplayCallback`, locatável.)

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CounterDisplayElem {
    pub kind: String,
}
```

`Content::CounterDisplay { kind }` → `Content::CounterDisplay(Arc<CounterDisplayElem>)`.
Construtor ergonómico: `Content::counter_display(kind: impl Into<String>)`.
**Deriva `Hash`** (só `String`).

## `impl Element for CounterDisplayElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1703`) |
| `is_empty` | default `false` |
| `map_content`/`map_text` | **terminais** |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (**não-locatável**) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `kind` (paridade `content.rs:1867`).
