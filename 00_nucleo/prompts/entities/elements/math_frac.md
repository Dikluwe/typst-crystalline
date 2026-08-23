# Prompt L0 — `entities/elements/math_frac` — `MathFracElem`
Hash do Código: 3e637d4e

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_frac.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathFracElem {
    pub num: Content,   // era Box<Content>
    pub den: Content,   // era Box<Content>
    pub line: bool,
}
```

`Content::MathFrac { num, den }` → `Content::MathFrac(Arc<MathFracElem>)`.
`Content::math_frac(num, den)` fixa `line: true`; o construtor interno
`Content::math_frac_unlined(num, den)` fixa `line: false` para `binom`.

## `impl Element for MathFracElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("({})/({})", self.num.plain_text(), self.den.plain_text())` (`content.rs:1613`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em `num`/`den` (`content.rs:2096`): `Content::MathFrac(Arc::new(MathFracElem { num: self.num.map_content(f)?, den: self.den.map_content(f)? }))` |
| `map_text` | **terminal** (math structural; `content.rs:2621`): `Content::MathFrac(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `num + den + line`.

## Critério

`plain_text` formato `(num)/(den)`; `map_content` recurse ambos; `map_text`
terminal; igualdade estrutural.

## P1132f — modo frac-like sem barra para `binom`

**Medição antes da decisão** (secção 7, `HEAD 781b207b4a5d`, working tree
não commitado, 2026-08-22): `binom(n,k)` cristalino usa `MathMatrix` e mede
13,156pt entre as baselines de `n` e `k`; o vanilla mede 14,993pt e resolve
`BinomElem` pela mesma `FractionItem` de `frac`, com `line = false`
(`ir/resolve.rs:715-759`). A diferença desloca as equações posteriores e não
pode ser corrigida por padding empírico de matriz.

**Decisão de contrato**: `MathFracElem` passa a transportar `pub line: bool`.
O valor integra a morfologia e a igualdade estrutural. Frações existentes
continuam com `true`; `binom` usa `false` e envolve a caixa em
`MathDelimited('(', ..., ')')`. Campo público novo: gate humano ADR-0127
obrigatório antes do código.
