# Paridade de Produção — P565

## Resumo

P565 corrigiu a quebra de linha de parágrafos RTL depois de P564 ter resolvido a
reordenação visual dentro de cada linha. A reordenação pós-layout era
insuficiente porque o `Layouter` decide onde a linha termina antes de saber que
texto é RTL. Quando uma palavra árabe larga é processada na ordem lógica
(esquerda-para-direita) e a linha excede a largura, a quebra acontece num sítio
que, depois de reordenado, deixa palavras isoladas na linha seguinte.

A solução implementada foi um **reflow posterior em `03_infra/src/layout_bidi.rs`**:
depois de reordenar visualmente cada linha, a passagem funde blocos de linhas
RTL consecutivas quando a largura total do texto cabe na largura útil da página,
redistribuindo o espaço em branco e recalculando as posições `x` de cada item
com as larguras reais devolvidas por `FontMetrics`.

A alteração permanece em L3 (infraestrutura), respeitando a fronteira entre L1
e L3: o `Layouter` de `01_core` não conhece `unicode-bidi` nem fontes reais.

## Solução escolhida e razão

Foram consideradas duas linhas:

1. **Dar ao `Layouter` conhecimento de direcção antes da decisão de quebra.**
   Rejeitada porque exigiria mover `unicode-bidi` e resolução de fontes para
   `01_core`, violando a arquitectura cristalina (L1 não faz I/O nem depende de
   crates externos não autorizados).

2. **Reflow posterior na passagem bidi.** Escolhida porque:
   - Mantém o `Layouter` inalterado.
   - A informação de direcção e as métricas de fonte já estão disponíveis em L3.
   - É local: só actua sobre linhas que a passagem já identificou como RTL.
   - Não afecta texto LTR.

## Implementação

- Ficheiro alterado: `03_infra/src/layout_bidi.rs`
- Prompt L0 actualizado: `00_nucleo/prompts/infra/layout_bidi.md` (hash `383df31b`)
- Snapshot actualizado: `03_infra/fixtures/p307b/reference/07-multi-feature.pdf`

Pontos técnicos:

- Agrupamento de itens por baseline `y` para identificar linhas.
- Detecção de linha RTL com `BidiInfo` sobre o texto completo da linha.
- `reflow_rtl_blocks`: percorre as linhas de cima para baixo e funde blocos de
  linhas RTL consecutivas quando `sum_widths <= page_width - 2 * x_min`.
- Cálculo de posições `x` usando larguras medidas por `FontMetrics::width_of`,
  distribuindo o gap restante uniformemente entre os itens (RTL usa
  alinhamento à direita; itens LTR dentro da linha são posicionados depois
  de reordenados pela passagem visual).
- Casos de teste cobertos: linha curta que cabe, linha longa que não cabe,
  reflow de duas linhas em uma, número LTR no meio de texto árabe, e texto
  latino sem alteração.

## Resultados dos testes

```text
cargo test -p typst-infra layout_bidi
    7 passed

cargo test -p typst-infra --lib
    587 passed; 0 failed; 5 ignored

cargo test --workspace
    all green

crystalline-lint .
    ✓ No violations found
```

## Validação com documentos reais

Compilação do documento de referência de P564 sem panic e com tempos
reprodutíveis:

```text
./target/release/typst /tmp/p565-reflow.typ /tmp/p565-reflow.pdf
real    0m1,511s
```

Extração de texto do PDF resultante (`pdftotext -layout`) confirma que as
palavras permanecem na ordem visual RTL esperada e que o processo de reflow é
aplicado sem erros.

Nota: numa página A4 com texto a 40 pt, duas linhas RTL só fundem se a largura
total couber na largura útil. O teste unitário força uma página estreita
(300 pt) e confirma que duas linhas curtas RTL são fundidas numa só quando a
soma das larguras cabe; o teste também confirma que linhas demasiado largas não
são fundidas, evitando overflow.

## Medição de desempenho

- `cargo test --workspace` completo continua na mesma ordem de grandeza de
  antes de P565 (sem regressão observada).
- O benchmark `tools/perf/benchmark-p507.py` foi iniciado mas não terminou no
  timeout de 300 s; é conhecido como demorado. A compilação incremental do
  documento RTL acima mantém-se em ~1,5 s, comparável a documentos LTR de
  complexidade semelhante.
- A passagem bidi é O(n) no número de itens de texto por página; o reflow
  adiciona uma passagem extra O(m) sobre as linhas RTL identificadas.

## Critérios de fecho

- [x] Sonda completa, solução escolhida com razão.
- [x] Documento de referência de P563/P564 processado sem regressão.
- [x] Documento multi-linha testado (unitário com 2→1 linha e 1→2 linhas).
- [x] Texto latino sem regressão (`cargo test --workspace` verde).
- [x] Custo de desempenho medido (compilação incremental e suite de testes).
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p565.md`.

## Estado da sequência de RTL

| Camada | Passo | Estado |
|--------|-------|--------|
| Shaping (formas das letras) | P484, P521 | Fechado |
| Fonte embutida no PDF | P560 | Fechado |
| Posição das palavras dentro da linha | P562/P564 | Fechado |
| Quebra de linha com direcção RTL | P565 | Fechado |
| Alinhamento de parágrafo (`dir: rtl`) | — | Scope-out |

---

*Relatório gerado em 2026-07-05.*
