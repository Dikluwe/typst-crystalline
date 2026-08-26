# Prompt L0 — `entities/elements/math_op` — `MathOpElem`
Hash do Código: b301f36d

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_op.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub. Mecanismo vanilla:
`OpElem` (P298; `limits` é discriminador de layout cross-variant — afeta o
`MathAttach` pai; semântica inalterada, em `rules/math/layout`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathOpElem {
    pub text:   Content,   // era Box<Content>
    pub limits: bool,
}
```

`Content::MathOp { text, limits }` → `Content::MathOp(Arc<MathOpElem>)`.
Construtor ergonómico: `Content::math_op(text: Content, limits: bool)`.

> **Fonte de paridade**: documentação `https://typst.app/docs/reference/math/op/`
> (corpus `00_nucleo/corpus-docs/math/op.typ:7-27`); mecanismo vanilla
> `lab/typst-original/crates/typst-library/src/math/op.rs`
> (`OpElem { text, limits }`) e consumo de `limits` em
> `lab/typst-original/crates/typst-layout/src/math/ir/resolve.rs`
> (`resolve_op`).

## `impl Element for MathOpElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.text.plain_text()` (`limits` é layout-only; `content.rs:1655`) |
| `is_empty` | default `false` (`content.rs:1568`) |

> **Nota de verificação**: a afirmação "`limits` é layout-only" refere-se à
> entidade `MathOpElem` — a flag não altera `plain_text`/`is_empty`/
> `map_content`/`map_text`/`eq`. O consumo real de `limits` acontece em
> `layout_attach` (`attach.rs:78`, ver `math/layout/attach.md` §P992).
| `map_content` | **recursivo** em `text`, `limits` preservado (`content.rs:2151`): `Content::MathOp(Arc::new(MathOpElem { text: self.text.map_content(f)?, limits: self.limits }))` |
| `map_text` | **terminal** (math structural; `content.rs:2637`): `Content::MathOp(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `text + limits` (paridade `content.rs:1838`).

## Critério

`plain_text` só o `text`; `map_content` recurse `text` e preserva `limits`;
`map_text` terminal; igualdade estrutural.
