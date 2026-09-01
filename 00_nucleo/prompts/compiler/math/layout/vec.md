# Prompt L0 — `compiler/math/layout/vec` — vetor coluna

**Estado:** CONTRATO P1292 AGUARDA SELO ADR-0127 — sem consumer e sem
`Hash do Código` até a materialização posterior ao gate humano.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c

**Camada:** L1
**Alvo planejado:** `01_core/src/compiler/math/layout/vec.rs`
**Contrato compartilhado:** `compiler/math/layout/_comum.md`
**Entidade:** `entities/elements/math_vec.md`

## Medição anterior à decisão

No vanilla ratificado, `VecElem` resolve cada filho como uma linha de uma
célula com alinhamento explícito e gap somente no eixo vertical
(`lab/typst-original/crates/typst-library/src/math/ir/resolve.rs:1002-1026`).
O table layout usa estilo denominador, centralização no eixo e delimitadores
esticados (`lab/typst-original/crates/typst-layout/src/math/table.rs:16-191`).
O cristalino atual reutiliza `MathMatrix` e perde identidade/alinhamento/gap.

## Decisão P1292

Uma free function `vec::layout(layouter, elem, style)`:

1. dispõe cada filho como uma linha de uma célula em estilo denominador;
2. aplica `Left|Center|Right`; enquanto direção não for modelada,
   `Start→Left` e `End→Right`, preservando o valor original no payload;
3. usa `gap` somente entre linhas;
4. centra a grade no eixo matemático e estica os delimitadores com o mecanismo
   comum já usado por matrix;
5. devolve caixa de tamanho zero para zero filhos, antes dos delimitadores,
   conforme o observável vanilla.

Reutilizar helpers de grid/delimiter não transforma `matrix.rs` no owner de
vec: a função e decisões de vetor permanecem neste arquivo (ADR-0109). `&`
dentro de um filho pode subdividir/alinha-lo conforme a semântica matemática
vigente, sem criar linhas extras.

### P1292.vec-region-gap — região efetiva, inclusive `height:auto`

A parcela percentual de `Rel<Length>` resolve contra a altura disponível da
região. O caller `compiler/layout/equation.rs` passa
`Regions::effective().height` como `Pt` obrigatório ao `MathLayouter`; este
owner resolve, para altura finita,
`gap.rel * region_height + gap.abs.resolve_pt(style.size)`; a regra não-finita
medida fica no parágrafo seguinte.
Altura da grade, tamanho da fonte e zero não são substitutos. Callers de teste
fornecem região explícita, e o caminho produtivo não possui constructor que
omita a base percentual.

**Altura não-finita medida antes da decisão:** o recibo fresco P1292 mediu
`height:auto` bilateralmente. `gap:10%` produziu a mesma geometria que `0%`;
`gap:1em` continuou ativo; `10% + 1em` produziu a mesma geometria que a parcela
absoluta. Logo, quando `Regions::effective().height` não é finita, este owner
resolve a parcela relativa como zero e preserva `gap.abs.resolve_pt(style.size)`.
Não propaga infinito, não usa altura final da página, fonte ou grade como base
substituta e não descarta a parcela absoluta. Em região finita, usa a equação
normal contra a altura efetiva.

Testes discriminatórios usam simultaneamente altura de página, altura da
região efetiva, altura da grade e font-size distintos e alteram somente a
região efetiva para provar a base escolhida. Um caso bilateral adicional usa
`page(height: auto)` e gap percentual, absoluto e misto para guardar a política
não-finita.

## Aceitação linguística

Filhos, alinhamento, gap e delimitadores determinam a morfologia do vetor sem
convertê-lo em matrix. A aceitação compara estrutura/semântica e geometria
derivada da fonte, não bytes ou ordem mecânica de items.
