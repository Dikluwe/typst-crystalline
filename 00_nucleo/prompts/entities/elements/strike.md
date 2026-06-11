# Prompt L0 — `entities/elements/strike` — `StrikeElem`
Hash do Código: f9c052e0

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/strike.rs`
**Origem**: modelo D (ADR-0105), **Lote 4 P319** (decorações de texto). Trait e
regras partilhadas: ver `entities/elements/_comum.md`. **Não-locatável**
(confirmado P319). Comportamento idêntico ao braço atual do hub (P284).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct StrikeElem {
    pub body:   Content,           // era Box<Content>
    pub stroke: Option<Color>,
    pub offset: Option<Length>,
    pub extent: Option<Length>,
}
```

`Content::Strike { body, stroke, offset, extent }` →
`Content::Strike(Arc<StrikeElem>)`. Construtor ergonómico:
`Content::strike(body, stroke, offset, extent)`.

> **`Hash` manual** (causa + ressalva completas em `overline.md`): `Length`
> carrega `f64` e não implementa `Hash` → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `hash_content`). Ressalva `-0.0` vs `0.0` documentada em comentário
> no código; aceitável (regime do `content_hash`; `…Elem` não são chaves de mapa;
> revisitar se vier a sê-lo).

## `impl Element for StrikeElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1659`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1598`) |
| `map_content` | **recursivo** no body, preserva cosméticos (`content.rs:2140`) |
| `map_text` | **recursivo** no body (`content.rs:2458`), preserva cosméticos |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body + stroke + offset + extent`
(paridade `content.rs:1954`).

## Critério

`plain_text` transparente; `is_empty` delega ao body; `map_*` recursam preservando
cosméticos; `Hash` manual via Debug.
