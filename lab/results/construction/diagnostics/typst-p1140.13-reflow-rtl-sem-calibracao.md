# Diagnóstico P1140.13 — Reflow RTL sem calibração empírica

**Estado:** concluído  
**Data:** 2026-08-24

## Resultado

O reflow RTL deixou de inferir continuidade por largura textual aproximada,
folga fixa ou múltiplo arbitrário da altura. A causa de `parbreak` passou a ser
transportada por `SemanticKind::ParbreakBoundary`, e a continuidade de linhas
sem marcador é derivada do avanço tipográfico usado pelo layout.

Não foi criado marcador para wrapping automático: os controles provaram que
ele já possuía paridade e deve continuar sendo reconhecido pelas métricas.

## Proveniência

Medição final realizada em **2026-08-24T12:38:39-03:00**.

- base Git: `ca28f4ab74ae66985cdc66805c16c2ddc8f08366`;
- estado: working tree não commitado;
- `git diff HEAD --stat`: **56 ficheiros alterados, 1419 inserções, 266 remoções**;
- `git status --short | wc -l`: **68 entradas**;
- vanilla: `/usr/local/bin/typst`, build ratificado;
- cristalino: `./target/debug/typst`, reconstruído depois da implementação;
- extração geométrica: `pdftotext -bbox`.

Os números descrevem a árvore acumulada da sessão, não apenas P1140.13.

## Mudanças

### L0 e contrato

- `entities/layout_types.md`: especifica `ParbreakBoundary`;
- `compiler/layout.md`: especifica a emissão transparente antes do flush;
- `infra/layout_bidi.md`: proíbe as calibrações e deriva o avanço vertical;
- hashes ressellados por `crystalline-lint --fix-hashes .` sem drift restante.

### L1

- `SemanticKind::ParbreakBoundary` adicionado ao contrato L1→L3;
- `compiler/layout/parbreak.rs` atomiza a emissão da fronteira;
- o braço `Content::Parbreak` emite o marcador somente quando há linha para
  fechar e preserva todo o cálculo vigente de spacing/margin collapse;
- teste confirma exatamente um marcador entre dois parágrafos.

### L3

`same_paragraph` agora:

- bloqueia `ExplicitLinebreakBoundary` e `ParbreakBoundary`;
- rejeita conservadoramente linhas com items cuja extensão vertical não possa
  reconstruir;
- calcula `top + descent + leading` por `FontMetrics::text_edges`, pelo leading
  resolvido do último item e pelo default normativo `PAR_LEADING`;
- usa somente `Y_TOLERANCE_PT`, tolerância numérica já ligada ao agrupamento de
  baselines.

Foram removidos do caminho de produção:

- `text.len() × style.size × 0.5`;
- `word_w + 30.0`;
- `1.5 × max_height`;
- `item_height`, que existia somente para sustentar o último limiar.

## RED → GREEN

O primeiro teste L1 não compilou porque `SemanticKind::ParbreakBoundary` ainda
não existia, confirmando o RED do contrato. Depois da implementação:

- `p1140_13_parbreak_preserva_fronteira_semantica`: GREEN;
- `p1140_13_parbreak_impede_reflow_rtl`: GREEN;
- `p1140_13_wrapping_segue_avanco_tipografico_real`: GREEN;
- controles `p565_reflow_merges_rtl_lines_when_fits` e
  `p565_reflow_does_not_merge_when_too_wide`: GREEN.

O primeiro fixture do controle vertical não oferecia largura suficiente para
fusão e foi corrigido para isolar a variável medida; não houve ajuste de
constante na implementação.

## Diferencial vanilla × cristalino

### `parbreak` RTL

Ambos mantiveram duas baselines: **7.64 pt** e **26.22 pt**. As caixas de todas
as palavras coincidiram; diferenças máximas observadas foram de
**0.000002 pt**, arredondamento textual do extrator.

### wrapping automático RTL

Ambos mantiveram três baselines: **7.64 pt**, **20.72 pt** e **33.80 pt**. As
oito caixas de palavra coincidiram; diferenças máximas observadas foram de
**0.000005 pt**, também no arredondamento do extrator.

Esses valores são oracles de teste e não foram introduzidos no código.

## Validação final

- `cargo test -p typst-core`: **5156 passed, 0 failed**;
- `cargo test -p typst-infra`: **832 passed, 0 failed**;
- `cargo build`: concluído;
- `crystalline-lint .`: zero violações;
- `git diff --check`: sem erros;
- busca estrutural no código de produção: nenhuma ocorrência das três
  calibrações removidas.

Warnings preexistentes de Rust e informações V19/V20 do linter permanecem
informativos e não constituem violações.

## Estado remanescente

P1140.13 fecha a classe conhecida de calibração empírica em
`same_paragraph`. A próxima frente deve medir outras decisões geométricas de
reflow/bidi antes de generalizar esta correção; nenhuma falha conhecida nova
foi revelada por este passo.
