# Prompt L0 — `entities/elements/linebreak` — `LinebreakElem`
Hash do Código: 1b7a8f34

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/linebreak.rs`
**Origem**: modelo D (ADR-0105), **Lote 5 P320** (quebras/espaços + grid/table
header/footer). Trait: ver `entities/elements/_comum.md`. **Não-locatável**
(confirmado P320). **Comando unit** — precedente `Divider`. Comportamento
idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LinebreakElem;
```

`Content::Linebreak` (unit) → `Content::Linebreak(Arc<LinebreakElem>)`.
Construtor ergonómico: `Content::linebreak()`.

## `impl Element for LinebreakElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `"\n".to_string()` (`content.rs:1683`) |
| `is_empty` | default `false` (não está no match de `is_empty`) |
| `map_content` | **terminal** (`Content::Linebreak(Arc::new(self.clone()))`) |
| `map_text` | **terminal** (`content.rs` bloco terminal) |
| `get_field`/`element_kind`/`to_payload` | default `None` |

## `eq`

`#[derive(PartialEq)]` em struct unit → sempre igual (paridade `content.rs:1847`
`(Linebreak, Linebreak) => true`).

## Critério

`plain_text` `"\n"`; map_* terminais; igualdade trivial.
