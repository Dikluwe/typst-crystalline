# Prompt L0 — `entities/elements/overline` — `OverlineElem`
Hash do Código: 051c0a83

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/overline.rs`
**Origem**: modelo D (ADR-0105), **Lote 4 P319** (decorações de texto). Trait e
regras partilhadas: ver `entities/elements/_comum.md`. **Não-locatável**
(confirmado P319: na lista exaustiva não-locatável de `introspect/locatable.rs`).
Comportamento idêntico ao braço atual do hub (P284, ADR-0054 graded).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct OverlineElem {
    pub body:   Content,           // era Box<Content>
    pub stroke: Option<Color>,
    pub offset: Option<Length>,
    pub extent: Option<Length>,
}
```

`Content::Overline { body, stroke, offset, extent }` →
`Content::Overline(Arc<OverlineElem>)`. Construtor ergonómico:
`Content::overline(body, stroke, offset, extent)`.
`Color`/`Length` de `entities::layout_types`.

> **`Hash` manual (1ª vez na família D)** — **causa**: `Length` carrega `f64` e
> **não** implementa `Hash` (deriva só `Debug, Clone, Copy, PartialEq`). O trait
> `Element` exige `Hash`, logo `#[derive(Hash)]` falha → implementar `Hash` à mão
> **via `Debug`**, paridade com `content_hash::hash_content` (Debug-based):
> `impl Hash { fn hash(..) { format!("{self:?}").hash(state) } }`.
>
> **Ressalva conhecida** (documentar também em comentário no código de cada
> `impl`): hash-via-`Debug` pode **violar o contrato `Hash`/`Eq`** para
> `-0.0` vs `0.0` (são `PartialEq`-iguais mas têm `Debug` distinto → hashes
> distintos). **Aceitável** porque (1) é exatamente o regime do `content_hash`
> pré-existente, e (2) os `…Elem` **não são chaves de mapa**. **Se algum vier a
> ser usado como chave** (`HashMap`/`HashSet`), **revisitar** (hash canónico dos
> floats por bits, ou normalizar `-0.0`). Não é hot-path.

## `impl Element for OverlineElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` — transparente (`content.rs:1660`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1599`) |
| `map_content` | **recursivo** no body, preserva `stroke`/`offset`/`extent` (Copy; `content.rs:2146`) |
| `map_text` | **recursivo** no body (container; `content.rs:2464`), preserva cosméticos |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body + stroke + offset + extent`
(paridade `content.rs:1956`).

## Critério

`plain_text` transparente; `is_empty` delega ao body; `map_*` recursam no body
preservando cosméticos; `Hash` manual via Debug; igualdade estrutural.
