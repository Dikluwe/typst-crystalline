# Prompt L0 — `entities/elements/outline` — `OutlineElem`
Hash do Código: be5d94de

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/outline.rs`
**Origem**: modelo D (ADR-0105), **Lote 8 P323** (por largura). Trait: ver
`entities/elements/_comum.md`. Comportamento idêntico ao braço atual.

> **Variante unit** — `Content::Outline` não tem campos no estado atual. A
> migração encapsula um marcador **sem estado** em `Arc<OutlineElem>` (struct
> vazio). **Não é inédito**: `DividerElem;` (piloto P316) e `LinebreakElem;`
> (Lote 5) já são structs unit — mesmo padrão. Confirmado opção (a) no
> checkpoint P323 (migrar uniforme).
>
> **Campos do vanilla pendentes de migração — ver cobertura.** O vanilla
> `OutlineElem` tem campos settable (`title`/`depth`/`indent`/…) ainda não
> portados; quando a cobertura os trouxer, aterram **neste módulo** sem tocar o
> hub (a vantagem estrutural do modelo D). `Outline` é elemento de utilizador
> (fora da classe DEBT-58).

> **Fronteira: LOCATÁVEL** (queryable desde P178, fecha lacuna #7). Absorve o
> braço de `extract_payload` no trait (precedente Heading/Lote 6): `element_kind`
> → `ElementKind::Outline`, `to_payload` → `ElementPayload::Outline` (payload
> **unit**). O consumo por `ElementPayload` (`introspect.rs:645`/`from_tags`) é
> **inalterado** (matcheia o payload, não o `Content`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutlineElem;
```

`Content::Outline` → `Content::Outline(Arc<OutlineElem>)`.
Construtor ergonómico: `Content::outline()`. **Deriva `Hash`/`Eq`**
(struct unit — trivial).

## `impl Element for OutlineElem`

| método | comportamento |
|---|---|
| `plain_text` | `String::new()` (`content.rs:1729`) |
| `is_empty` | default `false` |
| `map_content`/`map_text` | **terminais** (leaf) |
| `get_field` | default `None` |
| `element_kind` | `Some(ElementKind::Outline)` |
| `to_payload` | `Some(ElementPayload::Outline)` (absorve `extract_payload.rs:71`; payload unit) |

## `eq`

`#[derive(PartialEq)]` — struct unit sempre igual a si (paridade
`content.rs:1894`: `(Outline, Outline) => true`).
