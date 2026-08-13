# Prompt L0 — `entities/elements/colbreak` — `ColbreakElem`
Hash do Código: 61cd4f53

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/colbreak.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320**. Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (confirmado P320). **Comando
unit** (com flag `weak`) — precedente `Divider`/`Pagebreak`. Comportamento
idêntico ao braço atual.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ColbreakElem {
    pub weak: bool,
}
```

`Content::Colbreak { weak }` → `Content::Colbreak(Arc<ColbreakElem>)`.
Construtor ergonómico preservado: `Content::colbreak(weak: bool)`.

## `impl Element for ColbreakElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1790`) |
| `is_empty` | default `false` — **nunca vazio** (event observable; `content.rs:1617`) |
| `map_content` | **terminal** |
| `map_text` | **terminal** |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq`

`#[derive(PartialEq)]` compara `weak` (paridade `content.rs:1972`).

## Critério

`plain_text` vazio; `is_empty` sempre `false`; map_* terminais; igualdade por `weak`.
