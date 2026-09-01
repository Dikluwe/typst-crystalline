# Prompt L0 — `entities/elements/math_vec` — `MathVecElem`

**Estado:** CONTRATO P1292 AGUARDA SELO ADR-0127 — sem consumer e sem
`Hash do Código` até a materialização posterior ao gate humano.

**Camada:** L1
**Alvo planejado:** `01_core/src/entities/elements/math_vec.rs`
**Regras comuns:** `entities/elements/_comum.md`
**Vanilla ratificado:** `a51e02804`

## Medição anterior à decisão

O cristalino degrada `vec` a `MathMatrix` em
`01_core/src/compiler/eval/math.rs:1030-1062`. O vanilla declara `VecElem`
distinto em `lab/typst-original/crates/typst-library/src/math/matrix.rs:18-68`,
com delimitador, alinhamento horizontal, gap relativo e filhos variádicos.
Probes de 2026-08-31 confirmam que zero filhos é válido e conserva identidade
`vec(children: ())`.

## Contrato público P1292

```rust
pub struct MathVecElem {
    pub children: Vec<Content>,
    pub delim: (char, char),
    pub align: HAlign,
    pub gap: Rel<Length>,
    pub explicit: MathVecExplicit,
}
```

Defaults: `delim = ('(', ')')`, `align = HAlign::Center`,
`gap = Rel { rel: 0, abs: 0.2em }`. `MathVecExplicit` conserva um bit para
cada named `delim`, `align` e `gap`; os bits não mudam layout, mas participam
da morfologia/identidade para que um default explicitamente escrito continue
no `repr`. O sentinel de delimitador ausente segue a
convenção tipada já usada por matrix/cases; não se converte `none` em glifo.
`Start` e `End` permanecem valores distintos no payload.

Elemento math estrutural não-locatável. `plain_text` concatena os filhos em
ordem; `map_content` recursa em cada filho e preserva `delim/align/gap`;
`map_text` é terminal; igualdade/hash incluem todos os campos pela convenção
canónica para escalares, inclusive presença. Zero filhos não é normalizado
para conteúdo vazio nem para matrix. O constructor canónico transporta
`children`, `delim`, `align`, `gap` e presença; uma conveniência Rust sem named
inicializa os três bits como omitidos.

O percentual de `gap` permanece dado não resolvido. A resolução geométrica é
obrigação de `compiler/math/layout/vec.md` e é fechada pelo contrato
`P1292.vec-region-gap` de `compiler/math/layout/_comum.md`; usar `em` como base
do percentual é proibido.

## Aceitação linguística

A entidade preserva identidade vetorial, cardinalidade/ordem dos filhos,
delimitadores inferidos, alinhamento e gap. Os casts e erros pertencem a
`compiler/stdlib/structural/math.md` e `compiler/eval/math.md`.

A forma default é `vec(children: ())` ou
`vec(children: ([a], [b]))`. Named presentes aparecem antes de `children`, em
ordem `delim`, `align`, `gap`; delimiter é normalizado como par, `gap` como
relative length. Assim `delim: "["` aparece `delim: ("[", "]")` e
`gap: 1em` aparece `gap: 0% + 1em`. Singleton em `children` conserva vírgula.
