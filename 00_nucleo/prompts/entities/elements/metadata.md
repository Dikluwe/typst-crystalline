# Prompt L0 — `entities/elements/metadata` — `MetadataElem`
Hash do Código: 6d0f065d

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/metadata.rs`
**Origem**: modelo D (ADR-0105), **Lote 6 P321** (família state/counter). Trait e glossário (§A.0):
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

---

## P844 (achado #47 de P831) — campo `value` acessível

- O campo `value` do elemento é exposto via `Content::get_field` (braço em `entities/content.rs`) — paridade vanilla `MetadataElem.value`; consumido por `query(<meta>).first().value`.

---

## P1031 — fonte de paridade (quatro achados de Bloco 3)

Doc comment `#[elem]` do vanilla ratificado (`e0e8ca4d`),
`crates/typst-library/src/introspection/metadata.rs:3-28`, publicado em
`typst.app/docs/reference/introspection/metadata/`:

```rust
/// Exposes a value to the query system without producing visible content.
///
/// This element can be retrieved with the `query` function and from the command
/// line with `typst query`. Its purpose is to expose an arbitrary value to the
/// introspection system. To identify a metadata value among others, you can
/// attach a `label` to it and query for that label.
/// …
/// ```example
/// #metadata("This is a note") <note>
/// #context { query(<note>).first().value }
/// ```
#[elem(since = "0.7.0", Locatable)]
pub struct MetadataElem {
    /// The value to embed into the document.
    #[required]
    pub value: Value,
}
```

Isto fecha os quatro achados sobre este L0:

1. **Struct/construtor** (`value` único, obrigatório) — **citação literal**:
   `metadata.rs:24-28`, `#[required] pub value: Value`. O `Box<Value>` do cristalino é
   indireção de memória, mecânica, diverge de propósito (ADR-0107/ADR-0030).
2. **`plain_text` → `String::new()`** — **sustentado pela frase de propósito**, não por uma
   citação da mesma forma: `metadata.rs:3` diz *"Exposes a value to the query system
   **without producing visible content**."* Um elemento sem conteúdo visível não tem texto
   simples a devolver. Citação **contextual**: a documentação afirma a ausência de saída
   visível; que a representação disso seja a string vazia é decisão do cristalino
   (`content.rs:1655`).
3. **`eq` cai em `_ => false`** — **não tem fonte na linguagem, e é assumidamente um quirk.**
   O vanilla deriva `PartialEq` normalmente para `MetadataElem`; a desigualdade sistemática
   do hub é mecânica do cristalino. A igualdade do Rust é explicitamente terreno de
   divergência (ADR-0107), pelo que não há paridade a provar aqui — só o registo de que a
   afirmação **não** é sobre a linguagem. Reclassificado de "paridade" para "quirk interno
   preservado".
4. **`value` acessível via `query(...).first().value`** — **citação literal**: é exactamente
   o exemplo do doc comment (`metadata.rs:14-22`), incluindo o acesso ao campo `.value`
   sobre o resultado de `query`. ✅ a alegação de paridade de P844 confirma-se.
