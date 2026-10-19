# Relatório — Passo 972 (parênteses desalinhados da baseline em numerador de fracção)

**Data:** 2026-08-05
**Proveniência**: HEAD no início = `f9657f6f4` (P971 Fase A). Alterações de
P972 no working tree durante as medições: `frac.md` (L0 §P972),
`01_core/src/engine/math/layout/frac.rs`,
`03_infra/src/integration_tests.rs`. Binários: debug reconstruído ~15:40;
release "antes" = binário de P970 copiado para `temp/p972/typst-antes`
antes do rebuild.

## Fase A — causa real (diferente da hipótese inicial do passo)

O passo suspeitava de `delimited.rs`/`stretchy.rs` (convenção de baseline do
delimitador). A investigação refutou essa hipótese por medição e encontrou
a causa noutro lugar:

1. **Reprodução isolada**: `$ n(n+1) $` (inline e bloco) — alinhado;
   `$ (n(n+1)) / 2 $` — parênteses do numerador **3.52pt abaixo** dos
   dígitos (pdf + content stream: mesmo glifo gid=9/10, mesmo tamanho 11pt,
   cm y 737.71 vs 741.23). Vanilla alinhado. Só o contexto de fracção.
2. **`layout_stretchy_delimiter` inocente**: instrumentação temporária
   (eprintln, removida depois) mostrou shift=0.000 idêntico nos dois
   contextos (Display e Text) — a caixa do delimitador sai correcta.
3. **Sondas por camada** (lição operacional registada abaixo): com
   `FixedMetrics` o bug **não se manifesta** — o parêntese sai como
   `FrameItem::Text` e é deslocado correctamente. Com as fontes reais
   (`with_fonts_and_system(&[])`, como o CLI) a variante do parêntese não
   tem mapeamento Unicode (`glyph_to_char` → None) e sai como
   **`FrameItem::Glyph`** (braço P906/P952b de `stretchy.rs`).
4. **Causa** (`frac.rs`, ciclos de posicionamento do numerador e do
   denominador): o `match` só deslocava `Text`/`TextShaped`; o braço
   `_ => {}` deixava `Glyph` (e `Line` — ex.: a barra de um `sqrt`
   aninhado) **sem o offset** `num_y`/`den_y`. Os dígitos subiam
   `num_y = −(descent + num_gap + thickness/2) = −3.52pt`; os parênteses
   ficavam — daí os 3.52pt exactos. No denominador o delta era −9.02pt
   (medido pelo teste RED).

## Fase B — correcção (fluxo contínuo ADR-0127: fórmula interna)

L0 primeiro: `frac.md` §P972. Os dois ciclos passam a usar
`offset_item(item, dx, dy)` (cobre todos os tipos — já usado no map final
do eixo, P919). Nenhuma assinatura muda; comportamento para
Text/TextShaped bit-a-bit igual.

**Testes** (`p972_tests` em `03_infra/src/integration_tests.rs` — têm de
ser de integração: só com a fonte real o parêntese é `Glyph`; RED
confirmado nos dois):
- `p972_parenteses_do_numerador_na_baseline` — Δy era **+3.520pt** (o caso
  do achado 9.3), agora < 0.01pt.
- `p972_parenteses_do_denominador_na_baseline` — Δy era **−9.020pt**
  (mesmo braço `_` no ciclo do denominador), agora < 0.01pt.

Suite completa: **5729 testes, 0 falhas** (+2). Delimitadores esticados
(assembly, P957): o fix toca só os ciclos de num/den de `layout_frac`;
delimitadores esticados fora de fracções não passam por estes ciclos, e
dentro de fracções passavam mal — agora passam bem (verificado visualmente
em `$ (1/2)/(3/4) $`, imagens lado a lado indistinguíveis do vanilla).

## Fase C — Revalidação

- **End-to-end** (`temp/p972/min3.typ`): as 6 posições da linha do
  numerador (`n ( n + 1 )`) ficam exactamente no mesmo y; render a 150 DPI
  idêntico ao vanilla nos dois casos.
- **compare.py** (doc de 30 secções vs vanilla, `temp/p957/doc-vanilla.pdf`):
  - Secção 8 (a do achado): max|dy| **8.327 → 1.936**, max|dx|
    **174.665 → 24.008**, med|dx| 4.214 → 2.415, emparelhados 123 → 125.
    Os outliers de dy eram os parênteses deslocados — resolvidos.
  - Secções 5/13/14/21: inalteradas. Total: 1735→1736 acima do limiar
    (dentro do ruído de emparelhamento; a contagem total de glifos mudou
    2722→2739 pelo efeito de P970 no tamanho do índice de raiz, que altera
    a segmentação de palavras do `pdftotext`).
- **Benchmark** (`benchmark-p972-canonical.py`, 7 cenários,
  `tools/perf/results/p972-canonical/`): 01-hello 1.032 · 02-lorem 1.005 ·
  03-images 0.999 · 04-math 1.015 · 05-tables 0.998 · 06-long 0.997 ·
  07-context 0.991 — rácio médio **1.005**, zero regressão (a alteração é
  uma troca de helper no caminho de fracções; o 1.032 no cenário mais
  pequeno é ruído de arranque, dentro do spread habitual desta máquina).
- **Linter**: resselo de `frac.rs` e `integration_tests.rs`;
  `crystalline-lint .` → 0 violations (só o V7 órfão pré-existente).

## Lição operacional (a registar)

Dois níveis de armadilha de medição encontrados neste passo:

1. **O bug só existe com a fonte real.** `FixedMetrics` (testes de unidade
   de L1) e o mundo sem fontes (`SystemWorld::new` sem `with_fonts_*`)
   emitem o parêntese como `Text` e o caminho defeituoso nunca corre. O
   teste de regressão teve de ser de integração com
   `with_fonts_and_system(&[])`. Futuros bugs de geometria que "não
   reproduzem" em testes de unidade: verificar primeiro se o item é
   `Text` ou `Glyph` no caminho real.
2. **A sonda tem de usar o entry point da produção**: `layout()` (P544)
   usa `FixedMetrics`; a produção usa
   `layout_with_introspector_and_metrics(FallbackFontMetrics::new(world))`.
   A primeira sonda (com `layout()`) mostrou tudo alinhado e apontava
   falsamente para o export — só a sonda com o entry point da produção
   localizou a divergência em L1.

## Resultado

- Causa exacta confirmada em código (`frac.rs`, braço `_ => {}` dos ciclos
  de num/den) — não a hipótese inicial do passo (delimitador), refutada por
  instrumentação.
- Parênteses de altura normal alinhados à baseline do conteúdo dentro e
  fora de fracções (medido); barras de fracções aninhadas (`Line`) e
  radicais aninhados em fracções ficam cobertos pela mesma correcção.
- Delimitadores esticados (assembly) não afectados; suíte verde; benchmark
  sem regressão; linter limpo.
