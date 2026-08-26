# Prompt L0 — `entities/elements/math_cases` — `MathCasesElem`
Hash do Código: 8310df66

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_cases.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathCasesElem {
    pub rows: Vec<Vec<Content>>,
}
```

`Content::MathCases { rows }` → `Content::MathCases(Arc<MathCasesElem>)`.
Construtor ergonómico: `Content::math_cases(rows: Vec<Vec<Content>>)`.
Delimitador esquerdo `{` fixo; sem delimitador direito (no layout, inalterado).

## `impl Element for MathCasesElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | linhas → células `join(" & ")`, linhas `join(", ")` (`content.rs:1639`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em cada célula de cada linha (`content.rs:2123`): `Content::MathCases(Arc::new(MathCasesElem { rows: new_rows? }))` |
| `map_text` | **terminal** (math structural; `content.rs:2626`): `Content::MathCases(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `rows` (paridade `content.rs:1828`).

## Critério

`plain_text` células `& ` e linhas `, `; `map_content` recurse na grelha;
`map_text` terminal; igualdade estrutural.
