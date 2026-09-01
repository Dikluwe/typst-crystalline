# Prompt L0 — `compiler/math/layout/vec` — vetor coluna

**Estado:** RASCUNHO NORMATIVO NO GATE ADR-0127 — sem consumer e sem Hash do
Código até selo humano.

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

## Decisão proposta

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

### P1291.vec-region-gap — contexto selado no mesmo gate

A parcela percentual de `Rel<Length>` resolve contra a altura disponível da
região. O caller `compiler/layout/equation.rs` passa
`Regions::effective().height` como `Pt` obrigatório ao `MathLayouter`; este
owner resolve `gap.rel * region_height + gap.abs.resolve_pt(style.size)`.
Altura da grade, tamanho da fonte e zero não são substitutos. Callers de teste
fornecem região explícita, e o caminho produtivo não possui constructor que
omita a base percentual.

**Altura não-finita medida antes da decisão:** em
`2026-08-31T15:11:48-03:00`, vanilla ratificado binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`
compilou com sucesso `page(height: auto)` + `vec(1, 2, gap: #10%)`; a fonte
vanilla aplica `Rel::relative_to(ctx.region.size)` sem clamp ou erro prévio
(`typst-layout/src/math/table.rs:24-34`, SHA-256
`77e1f383e1b252af5331803c9e88ba64d079f1528183083896b9d9f7501e7e04`;
`typst-library/src/layout/rel.rs:112-113`, SHA-256
`2c9b969e8fa9d8f007b87f4af7d959f2a65590e5c7bc595b69ca5f6c8bab3aa5`).
Portanto altura efetiva infinita é dado legítimo: o cristalino conserva o valor e a aritmética relativa, sem
substituir zero, altura final da página ou font-size e sem introduzir erro que
o vanilla não produz. O resultado final auto-page deve coincidir em morfologia
e geometria; valores não-finitos não podem vazar para o exporter se o processo
de finalização da região não os conservar no vanilla.

Testes discriminatórios usam simultaneamente altura de página, altura da
região efetiva, altura da grade e font-size distintos e alteram somente a
região efetiva para provar a base escolhida. Um caso bilateral adicional usa
`page(height: auto)` e gap percentual para guardar a política não-finita.

## Aceitação linguística

Filhos, alinhamento, gap e delimitadores determinam a morfologia do vetor sem
convertê-lo em matrix. A aceitação compara estrutura/semântica e geometria
derivada da fonte, não bytes ou ordem mecânica de items.
