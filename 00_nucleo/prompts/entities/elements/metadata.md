# Prompt L0 — `entities/elements/metadata` — `MetadataElem`
Hash do Código: 5b8f4129

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/metadata.rs`
**Origem**: modelo D (ADR-0105), **Lote 6 P321** (família state/counter). Trait:
ver `entities/elements/_comum.md`. Comportamento idêntico ao braço atual.

> **Fronteira: LOCATÁVEL** (queryable, M9 P169). Absorve o braço de
> `extract_payload` no trait (precedente Heading): `element_kind` →
> `ElementKind::Metadata`, `to_payload` → `ElementPayload::Metadata`. O consumo
> por `ElementPayload` (`from_tags`/walk) é **inalterado** (matcheia o payload,
> não o `Content`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct MetadataElem {
    pub value: Box<Value>,
}
```

`Content::Metadata { value }` → `Content::Metadata(Arc<MetadataElem>)`.
Construtor ergonómico: `Content::metadata(value: Value)`.

> **`Hash` manual via Debug** (precedente Lote 4/5): `Value` carrega `f64`
> (`Value::Float`) e não implementa `Hash` → `impl Hash { format!("{self:?}")… }`
> (paridade `content_hash`). `PartialEq` deriva (`Value: PartialEq`).

## `impl Element for MetadataElem`

| método | comportamento |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1655`) |
| `is_empty` | default `false` |
| `map_content`/`map_text` | **terminais** |
| `get_field` | default `None` |
| `element_kind` | `Some(ElementKind::Metadata)` |
| `to_payload` | `Some(ElementPayload::Metadata { value: self.value.clone() })` (absorve `extract_payload.rs:38`) |

## `eq` — quirk pré-existente preservado

**`Metadata` NÃO tem arm de `eq` no hub** (`content.rs` cai em `_ => false`:
sempre desigual — quirk pré-existente de marcadores efectivos). **Preservar**:
não adicionar arm de dispatch. O `derive(PartialEq)` no `…Elem` existe pelo
contrato do trait mas o hub mantém `Content::Metadata` em `_ => false`.
