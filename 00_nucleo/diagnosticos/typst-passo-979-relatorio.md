# Relatório — Passo 979 (agrupamento de runs de texto num único `BT…ET`)

**Data:** 2026-08-05 · **Gate:** confirmado pelo dono em 2026-08-05
("Continue" após `typst-passo-979-faseA.md`).
**Proveniência**: HEAD no início = `c909e07ec` (P979 Fase A). Benchmark:
`temp/p979/typst-antes` = release de P978.

## Fase B — implementação (resumo da Fase A em `typst-passo-979-faseA.md`)

Em `stream.rs` (modo verbose apenas — o compacto fica byte-inalterado):

- `build_page_stream` detecta runs: itens `TextShaped` **consecutivos**
  com envelope idêntico (fonte, tamanho, fill, weight/faux-bold, tracking,
  eixos de variação, direcção, math/math_size, `units_per_em`) e **mesma
  baseline** fundem-se num só bloco. Cenários suportados: Cidfont e
  Multifont sem bitmap; Type1/bitmap nunca formam run (caminho antigo,
  item a item).
- `emit_verbose_text_run` + `push_run_tj_entries`: um só envelope e um só
  array `TJ`; dentro de cada item, o delta model de P486/P520/P548
  inalterado; na fronteira entre itens, um ajuste `TJ` extra computado
  das **posições**, com o cursor a seguir a aritmética arredondada do
  renderer (inteiros de 1/1000 em) — a deriva de arredondamento nunca
  acumula além de um quantum (~0.005pt) por fronteira.

**Testes** (`p979_tests` em `export/tests.rs`; RED confirmado nos dois de
fusão — antes: 2 blocos): fusão básica; não-fusão por fill/size/baseline;
quebra por item não-texto; ajuste de fronteira exacto (`-233` no caso
canónico 100→110pt com advance 600du@12pt). Suite: **5759 testes,
0 falhas** (+6).

## Fase C — Revalidação

- **Contagem de blocos**: 02-lorem **503 → 36 `BT`** — exactamente os 36
  do vanilla (stream 82.5KB → 33.0KB; vanilla 30.8KB). Documento de 30
  secções: 2074 → **1293** (stream 259KB → 175KB) — abaixo dos 1919 do
  vanilla: fundimos mais que os TextItems de math do vanilla (as fronteiras
  de fragmento internas do vanilla não são observáveis dos nossos itens) —
  registado; a direcção da convergência é a pretendida pelo passo (menos
  operadores, posições idênticas).
- **Posições**: `compare.py` depois-vs-antes no documento completo —
  **0/2997 glifos acima do limiar de 0.5pt** (max|dx| = 0.004pt = o
  quantum de arredondamento TJ). Pixel-diff a 150/300 DPI: lorem
  **pixel-idêntico**; doc de 30 secções com ~1200 pixels de cobertura de
  antialiasing em 10 linhas (desvios ≤0.006pt, invisíveis a olho — o preço
  medido do fluxo por advances arredondados em vez de um `cm` por item).
- **Benchmark** (`benchmark-p979-canonical.py`, 7 cenários,
  `tools/perf/results/p979-canonical/`): 01-hello 1.012 · 02-lorem 1.000 ·
  03-images 1.005 · 04-math 1.017 · 05-tables 1.005 · 06-long 1.007 ·
  07-context 0.994 — rácio médio **1.006**, zero regressão (menos
  operadores a escrever, mesma aritmética).
- **Linter**: resselo de `stream.rs`; `crystalline-lint .` → 0 violations
  (só o V7 órfão pré-existente).

## Resultado

- Runs consecutivos sem mudança de estado gráfico fundidos num único
  `BT…ET`, como o vanilla — o residual de P956 §7 fica fechado.
- Prosa: contagem de blocos igual à do vanilla (36 = 36); stream 2.5×
  menor.
- Posições de glifo inalteradas (0 acima de 0.5pt; máx 0.004pt).
- Benchmark sem regressão; suíte verde; linter limpo.
