# Pacote de desempenho validado e próximo alvo — 2026-11-17

Pacote cumulativo adotado no projeto com L0 primeiro: captura/SourcePlan, indexação, grafo builtin, fontes e shaping, produtos de parágrafo e redução de reconstruções em link_capture. Preservados origem, efeitos, invalidação, limites de memória e os contratos atuais. Instrumentação de laboratório, overlays ELF e candidatas rejeitadas não foram incorporados ao produto.

As medições ocorreram na árvore de trabalho não commitada baseada em `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`. Os relatórios congelados conservam essa descrição histórica. O manifesto `after-frozen.json` registra os hashes exatos das fontes usadas; o estado produtivo foi reconferido antes deste commit.

## Resultado do ciclo completo

CPU mediana até a saída em processo novo, 16 blocos com ordens balanceadas; o ganho percentual vem das razões pareadas por bloco, não da soma de ganhos históricos.

| Caso | Anterior ms | Atual ms | Vanilla ms | Ganho pareado |
|---|---:|---:|---:|---:|
| Referências / 500 parágrafos | 2791,72 | 472,08 | 195,40 | 83,0% |
| Referências / 100 parágrafos | 276,69 | 81,58 | 153,41 | 70,8% |
| Sumário / 100 parágrafos | 266,64 | 75,76 | 153,98 | 71,2% |
| Unicode | 271,15 | 217,32 | 209,61 | 19,5% |

Referências/500 tem 20 seções/referências e 500 parágrafos; sumário/100 tem um heading e 100 parágrafos. Vazio: 7,45→7,69 ms; não foi qualificado como ganho, com variação A/A de escala semelhante. O pico mediano de RAM em referências/500 caiu de 181,8 para 61,9 MiB. Em 40 edições, o RSS de referências foi de 57,5 para 70,4 MiB: essa janela não prova estabilidade ilimitada.

Vanilla fixado no commit `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, tree `483bd6f1d8ac8810db47b65336d402647955fd17`, ELF SHA-256 `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`. Produto qualificado SHA-256 `61be8cf9e6b32bb4df09e2efbd98f4c0b3ce4c6aa6d8109b6b5384bee614cbb4`.

A comparação com vanilla conserva diferenças preexistentes de saída; não certifica paridade global. A correção de captura elimina links indevidos do código anterior, além de duplicatas, e a correção de largura segue text_width. Em referências/500, 28.487 anotações anteriores passaram a 40, como no oracle; regiões e posições de destino ainda têm dívidas documentadas. Dez comparações de identidade de módulos builtin agora coincidem com o oracle. Igualdade de módulos de usuário e context_body continuam abertos.

## Validação

RED real de captura: quatro falhas antes da implementação; GREEN: oito testes. Core: 8559 passaram/5 ignorados; infra: 1819/2; shell: 120; wiring: 40/40. Build workspace, Clippy de todos os targets com warnings negados, Rustdoc e crystalline-lint passaram. Os dois testes de loopback limitados inicialmente pelo sandbox passaram com o mesmo ELF após a exceção autorizada.

2.357 pares de PDFs e mapas de destino foram verificados na integração, com classificação explícita das correções de links. Mais 235 pares passaram no perfil posterior e nos controles do censo. Foram preservados caracteres, estilos/geometria por glifo, navegação sob seus contratos e publicação/recuperação; presença de tagging não equivale a auditoria integral de acessibilidade.

## Próximo alvo medido

O perfil do produto aplicado reuniu 35.968 amostras em duas frequências. Em sessão aberta, layout físico ocupa 65–66% em referências, 71–72% no sumário e 74–75% no relatório misto. Em referências, links e posições finais juntos ficam perto de 11%; shaping+Bidi final perto de 2%. As frações são custo observado, não ganho recuperável prometido.

O censo da biblioteca registrou 0 hits, 0 misses e 0 entradas no memo de parágrafos nos cinco casos nominais, em quatro compilações por caso. Controles textuais independentes preencheram o memo e geraram hits. Recusas anteriores ao lookup não contam como misses; o motivo exato ainda precisa ser isolado entre estrutura, opções, prova de métricas e caminho de execução. Matemática e Unicode podem legitimamente permanecer fora do subconjunto admitido.

Próximo experimento: L0 primeiro, rastrear entrada/recusas/hits de try_parts, try_layout e ParagraphSession::request no caminho real de referências e sumário. Testar apenas reúso certificado de trechos textuais, conservando origem/SourcePlan, efeitos, estilos, largura, fontes e invalidação; medir o ciclo completo e rejeitar regressões. Aumentar o cache com zero entradas não tem fundamento. Nenhuma alteração produtiva veio desse perfil posterior.

## Evidência versionada e local

A seleção versionada abaixo contém medições, planos, fontes dos inputs, manifestos e resumos de validação. PDFs, executáveis, traces volumosos, snapshots de laboratório e a limpeza histórica anterior permanecem locais; não foram incluídos neste commit.

- [Integração e comparação](../../lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/report.md), [análise](../../lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/analysis.json) e [fontes/ELFs](../../lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/after-frozen.json).
- [Perfil posterior](../../lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/report.md), [resumo](../../lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/summary.json), [censo](../../lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/paragraph-census.json) e [controles positivos](../../lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/paragraph-positive.json).

Os relatórios originais usam caminhos absolutos da execução e apontam também para artefatos locais não publicados. Sua proveniência foi conservada, sem reescrever os arquivos já selados.
