# Prompt L0 — `entities/elements/label` — `LabelElem`
Hash do Código: 00000000

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/label.rs`
**Origem**: P460 (destino nomeado para referências cruzadas). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável** (sem `element_kind`/`to_payload`);
`locatable.rs` lista `Content::Label` no bloco não-locatável.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LabelElem {
    pub name: EcoString,       // nome do destino: "sec1", "fig1", ...
    pub body: Content,         // conteúdo ao qual o label está associado
}
```

`Content::Label { name, body }` → `Content::Label(Arc<LabelElem>)`.
Construtor ergonómico: `Content::label(name, body)`. **Deriva `Hash`**
(`EcoString` deriva `Hash`; `Content` tem `impl Hash` manual).

## P1140.2 — distinção do valor `label`

Medição: o vanilla rejeita `label("x", [body])`; o cristalino aceitava essa
extensão. `LabelElem` não é o resultado do construtor público `label(name)`:
esse resultado é `Value::Label`. `Content::Label`, `Content::label` e
`Content::label_auto` permanecem mecanismos internos de associação,
introspecção e fixtures Rust. Nenhum deles autoriza uma função pública de
dois argumentos.

## `impl Element for LabelElem`

| método | comportamento |
|---|---|
| `plain_text` | `self.body.plain_text()` |
| `is_empty` | `self.body.is_empty()` |
| `map_content` | **recursivo** no `body`, preserva `name` |
| `map_text` | **recursivo** no `body`, preserva `name` |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara `name`/`body`.

## Semântica de layout

`Content::Label` é um wrapper transparente: o `body` é renderizado
normalmente. O layout regista o destino (página + posição) em
`LayouterRuntimeState`, que `Layouter::finish()` expõe via
`PagedDocument.extracted_label_pages` e `extracted_label_positions`.
