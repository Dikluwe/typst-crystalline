# Prompt L0 — `entities/elements/math_root` — `MathRootElem`
Hash do Código: f10a8e26

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_root.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathRootElem {
    pub index:    Option<Content>,   // era Option<Box<Content>>; None = raiz quadrada
    pub radicand: Content,           // era Box<Content>
}
```

`Content::MathRoot { index, radicand }` → `Content::MathRoot(Arc<MathRootElem>)`.
Construtor ergonómico: `Content::math_root(index: Option<Content>, radicand: Content)`.

## `impl Element for MathRootElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `None => format!("sqrt({})", radicand…)`; `Some(i) => format!("root({}, {})", i…, radicand…)` (`content.rs:1625`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** (`content.rs:2107`): `index` via `.as_ref().map(map_content).transpose()?`; `radicand.map_content(f)?` |
| `map_text` | **terminal** (math structural; `content.rs:2623`): `Content::MathRoot(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `index + radicand` (paridade `content.rs:1820`).

## Critério

`plain_text` `sqrt(...)`/`root(i, ...)`; `map_content` recurse preservando `None`
no index; `map_text` terminal; igualdade estrutural.
