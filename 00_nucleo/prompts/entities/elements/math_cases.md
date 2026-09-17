# Prompt L0 — `entities/elements/math_cases` — `MathCasesElem`
Hash do Código: 60ca056b

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

## P1430 — carrier completo (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O carrier vigente já acumulou `delim`, `reverse` e `gap`, mas guarda rows
pré-resolvidas, usa `(char, char)` obrigatório, `Option<Length>` e não conserva
presença explícita. O vanilla guarda children variádicos, par opcional e gap
relativo; a resolução de ramos pertence ao consumer.

### Contrato público proposto

`MathCasesElem` passa a guardar `children: Vec<Content>`,
`delim: (Option<char>, Option<char>)`, `reverse: bool`,
`gap: Rel<Length>` e `MathCasesExplicit { delim, reverse, gap }`. O tipo de
presença é público, `Copy + Eq + Hash`. Elemento, explicitude, delimitadores e
gap participam de igualdade/hash; floats relativos seguem a normalização dos
carriers vigentes. `map_content` desce em cada child; `map_text` continua
terminal; `plain_text` junta ramos com `, ` e deixa `MathAlignPoint` dentro do
child. Toda reconstrução preserva valores, ordem e bits.
