# Prompt L0 — `entities/elements/math_matrix` — `MathMatrixElem`
Hash do Código: 687e1a34

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_matrix.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathMatrixElem {
    pub rows:  Vec<Vec<Content>>,
    pub delim: (char, char),
}
```

`Content::MathMatrix { rows, delim }` → `Content::MathMatrix(Arc<MathMatrixElem>)`.
Construtor ergonómico: `Content::math_matrix(rows: Vec<Vec<Content>>, delim: (char, char))`.

## `impl Element for MathMatrixElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | linhas → células `join(", ")`, linhas `join("; ")` (`content.rs:1634`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em cada célula de cada linha, `delim` preservado (`content.rs:2116`): `Content::MathMatrix(Arc::new(MathMatrixElem { rows: new_rows?, delim: self.delim }))` |
| `map_text` | **terminal** (math structural; `content.rs:2625`): `Content::MathMatrix(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `rows + delim` (paridade `content.rs:1826`).

## Critério

`plain_text` células `, ` e linhas `; `; `map_content` recurse na grelha; `delim`
preservado; `map_text` terminal; igualdade estrutural.
