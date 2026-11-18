# Pacotes com ganho adotados no produto e comparação com vanilla

Registro UTC: 2026-11-18T01:51:14.927776+00:00. Árvore de trabalho sem commit, baseada em `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`. Nenhum commit ou staging feito. A lista exata de mudanças (`git diff HEAD --stat`), status dos arquivos novos e hashes por arquivo estão em [final.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/final.json); o estado anterior está em [initial.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/initial.json).

## Escopo adotado

Foi adotado o pacote cumulativo Rust validado: retenção de captura e resultados físicos com certificados/guards; consultas e reúso do SourcePlan; indexação semântica; retenção do grafo builtin; seleção e fallback de fontes, candidates_for_char e produtos de shaping; redução de reconstruções de vetores em link_capture; caches de parágrafos e ciclo de sessão. As dependências necessárias entre esses componentes foram preservadas. A seleção deriva das qualificações anteriores do laboratório; esta rodada mede o pacote cumulativo, não atribui a cada componente os ganhos totais nem soma percentuais históricos.

L0 aplicado antes do código, grafo de owners/núcleos validado antes do reseal, cabeçalhos resealados. Preservados origem, efeitos, invalidação e os ADRs atuais 0029/0030/0107. Removidos os caminhos de diagnóstico/ambiente do laboratório do código produtivo. Não foram adotados padding de código nativo, overlays ELF ou experimentos rejeitados. [Aplicação](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/applied.json) · [Plano L0](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/application-plan.json) · [auditoria dos testes](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/test-policy-audit.json).

![Comparação do pacote com o anterior e vanilla](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/comparison.png)

## CLI: ciclo completo em processo novo

CPU de todos os threads, criação até saída (`wait4`); inclui descoberta de fontes, compile e exportação. Mediana de 16 blocos pareados, ordem Williams balanceada, mais um bloco fixo de aquecimento excluído. Cache de disco não foi esvaziado. A/A usa cópia byte idêntica do produto. Percentuais são medianas das razões pareadas; IC95 por bootstrap de blocos, exploratório, sem remoção de outliers.

| Fixture | Anterior ms | Atual ms | Vanilla ms | Atual/anterior Δ% [IC95] | A/A Δ% |
|---|---:|---:|---:|---:|---:|
| empty | 7.45 | 7.69 | 9.29 | +4.4 [+1.0, +8.4] | -3.8 |
| header_report | 157.13 | 48.30 | 152.87 | -69.3 [-70.0, -68.6] | -2.3 |
| math_1 | 109.95 | 107.94 | 142.61 | -1.8 [-2.8, -0.8] | +2.1 |
| math_10 | 135.29 | 117.80 | 142.94 | -14.6 [-15.1, -13.3] | +1.1 |
| math_60 | 292.22 | 160.11 | 153.22 | -45.1 [-46.5, -43.7] | -0.0 |
| math_supported | 198.18 | 133.95 | 146.81 | -33.1 [-33.9, -30.4] | +0.2 |
| mixed_report | 677.49 | 303.42 | 162.48 | -56.1 [-56.2, -54.7] | +0.4 |
| one_line | 99.22 | 10.18 | 140.06 | -90.2 [-90.4, -89.9] | -2.8 |
| outline_100 | 266.64 | 75.76 | 153.98 | -71.2 [-71.5, -70.7] | -1.0 |
| prose_20 | 104.59 | 13.69 | 142.21 | -86.9 [-87.1, -86.7] | -2.2 |
| prose_500 | 269.77 | 170.81 | 170.52 | -36.1 [-37.0, -35.7] | +0.4 |
| references_100 | 276.69 | 81.58 | 153.41 | -70.8 [-71.5, -70.3] | +0.4 |
| references_500 | 2791.72 | 472.08 | 195.40 | -83.0 [-83.4, -82.7] | -1.0 |
| unicode_repo | 271.15 | 217.32 | 209.61 | -19.5 [-20.4, -18.4] | -0.0 |

`references_500` contém 20 seções/referências e 500 parágrafos; `references_100` contém uma referência e 100 parágrafos. `outline_100` contém um heading e 100 parágrafos: o nome não significa 100 entradas de sumário. O documento vazio mede sobretudo o piso do processo e não possui o mesmo custo dos documentos não vazios. Nele, a razão pareada atual/anterior foi +4,4% (aprox. 0,24 ms entre medianas); o A/A byte idêntico variou −3,8%. Não se atribui ganho ao vazio nem se isola uma regressão de implementação a partir dessa escala sem nova medição dedicada.

## Watch: início e recompilação

Medição em observadores isolados: fonte do produto adotado mais protocolo privado de fronteira; upstream fixado mais seu protocolo verificado. Nenhum observador foi instalado como produto. CPU END→END inclui detectar mudança, reset, compilação, publicação, drop e eviction. O início inclui descoberta eager de fontes upstream. BEGIN→END também foi registrado. Reescrever os mesmos bytes dispara recompilação; não é evento ignorado. Doze blocos, todas as seis ordens duas vezes. Comentários A/B são apresentados separadamente da edição visível.

| Fixture | Estado | Atual ms | Vanilla ms | Atual/vanilla Δ% [IC95] | A/A Δ% |
|---|---|---:|---:|---:|---:|
| empty | comment_a | 0.942 | 0.902 | +3.5 [-5.5, +16.7] | +1.7 |
| empty | comment_b | 0.852 | 0.849 | -2.2 [-8.9, +10.3] | +1.9 |
| empty | same_bytes | 0.807 | 0.863 | -5.5 [-21.3, +3.4] | +6.7 |
| empty | startup | 6.514 | 123.385 | -94.9 [-95.0, -94.5] | +3.7 |
| empty | visible_tail | 3.512 | 3.154 | +11.5 [+8.6, +18.2] | -0.8 |
| math_supported | comment_a | 40.209 | 4.384 | +819.1 [+772.9, +919.2] | +1.1 |
| math_supported | comment_b | 40.458 | 4.128 | +872.0 [+826.9, +995.0] | +0.8 |
| math_supported | same_bytes | 40.044 | 3.546 | +1046.0 [+977.5, +1091.9] | +0.2 |
| math_supported | startup | 166.056 | 129.311 | +28.2 [+26.7, +31.1] | -0.5 |
| math_supported | visible_tail | 40.952 | 5.676 | +596.9 [+573.0, +725.9] | +0.8 |
| mixed_report | comment_a | 161.238 | 9.122 | +1689.3 [+1585.6, +1798.1] | -1.6 |
| mixed_report | comment_b | 160.850 | 4.716 | +3289.1 [+3170.1, +3940.5] | +0.7 |
| mixed_report | same_bytes | 158.572 | 4.127 | +3713.2 [+3619.6, +3937.2] | -0.7 |
| mixed_report | startup | 299.501 | 140.696 | +115.0 [+110.2, +117.6] | -1.3 |
| mixed_report | visible_tail | 162.223 | 10.018 | +1498.7 [+1458.5, +2083.8] | -0.7 |
| outline_100 | comment_a | 43.753 | 5.292 | +740.4 [+693.1, +799.5] | +2.0 |
| outline_100 | comment_b | 43.840 | 2.673 | +1588.9 [+1326.2, +1702.1] | +1.3 |
| outline_100 | same_bytes | 45.190 | 2.601 | +1674.5 [+1375.1, +1808.6] | -1.1 |
| outline_100 | startup | 62.246 | 131.695 | -52.8 [-53.5, -51.5] | -0.7 |
| outline_100 | visible_tail | 46.939 | 7.569 | +505.9 [+495.7, +645.9] | -0.2 |
| references_100 | comment_a | 46.701 | 5.384 | +754.6 [+721.3, +891.4] | -1.6 |
| references_100 | comment_b | 45.386 | 3.172 | +1399.1 [+1290.7, +1552.2] | -0.7 |
| references_100 | same_bytes | 46.081 | 2.527 | +1704.1 [+1576.7, +1968.8] | -1.1 |
| references_100 | startup | 60.528 | 133.093 | -54.4 [-55.2, -53.9] | +0.2 |
| references_100 | visible_tail | 48.088 | 7.315 | +560.2 [+531.0, +683.6] | -1.2 |
| references_500 | comment_a | 369.508 | 20.367 | +1710.3 [+1660.0, +1774.1] | -0.3 |
| references_500 | comment_b | 370.997 | 9.780 | +3679.3 [+3648.6, +4244.5] | -2.1 |
| references_500 | same_bytes | 369.821 | 9.110 | +3943.7 [+3896.5, +4139.9] | +2.2 |
| references_500 | startup | 403.654 | 169.531 | +136.9 [+134.1, +141.9] | +0.8 |
| references_500 | visible_tail | 367.432 | 26.784 | +1287.2 [+1254.3, +1325.5] | -1.7 |
| unicode_repo | comment_a | 72.371 | 14.683 | +392.3 [+382.1, +474.9] | +0.0 |
| unicode_repo | comment_b | 72.794 | 14.465 | +419.5 [+375.2, +509.1] | -0.8 |
| unicode_repo | same_bytes | 89.188 | 14.393 | +542.3 [+506.6, +629.1] | +0.2 |
| unicode_repo | startup | 241.257 | 180.916 | +34.6 [+31.0, +35.5] | -0.1 |
| unicode_repo | visible_tail | 73.096 | 16.710 | +348.4 [+328.7, +391.5] | +0.3 |

O ganho na CLI não elimina a lacuna incremental. O próximo alvo recomendado é o custo de recompilar referências/sumário em uma sessão já aberta, especialmente trabalho repetido após comentários. As medições apontam a prioridade; não identificam ainda a função responsável. Um novo perfil deve separar retenção/reconstrução do plano, avaliação e convergência/layout com os guards atuais preservados. Não há novo experimento de otimização nesta integração.

## Correção e limites de equivalência

2357 pares de PDFs verificados: cada medição contra a compilação fresca do mesmo compilador/input; antes/depois; alterações de texto/estilo/dependência, estado/counter/query, erros e recuperação, além de 40 edições. Verificação cobre caracteres, estilo e geometria por glifo, links, bookmarks e presença de tagging; não exige a mesma segmentação mecânica de spans no PDF. Contra o fresco do mesmo compilador, anotações são comparadas por retângulo/ação/destino/flags/borda; duplicatas exatas são reduzidas preservando a ordem da última ocorrência. Entre anterior e atual, as correções autorizadas de captura/largura recebem uma classificação explícita: render preservado, conjunto de destinos preservado e contagem atual coincidente com o oracle nesses inputs. Contagem isolada não prova correção das regiões; os testes lexicais/geometry/rollback do owner fornecem os controles correspondentes. Todos os mapas de destinos nomeados também são comparados integralmente. Controles de efeitos/invalidação/erro e retenção também passaram por rasterização a 72 dpi. Não é uma prova de todo o grafo de acessibilidade. [Verificação](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/verification.json).

Os controles verificam preservação de caracteres, estilo, geometria, raster e destinos, além das correções de áreas clicáveis já especificadas pelos L0; não se exige navegação fisicamente idêntica à implementação anterior defeituosa. A contagem física e as áreas clicáveis divergem de forma esperada: a correção release de active.pop fora de debug_assert libera o escopo lexical, removendo links vazados para conteúdo não relacionado; text_width corrige a largura horizontal conforme L0. Em references_500, o anterior produziu 28.487 anotações, contra 40 no atual e no vanilla. A lentidão extrema de get_links com essa entrada levou à comparação direta de xrefs, sem retirar a validação dos destinos. As regiões e posições de destino do atual ainda diferem do vanilla; a auditoria registra isso como dívida preexistente, preservada onde os mapas anterior/atual coincidem, não como paridade fechada. [Auditoria de links](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/link-semantic-audit.json) · [L0 de captura](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/core/compiler/layout/link_capture.md:41) · [L0 de largura](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/core/compiler/layout/link.md:71). [Contagem e amostras](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/link-density-observation.json) · [Mapas de destino](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/destination-verification.json). A comparação com vanilla tem diferenças preexistentes de saída e não certifica paridade global de linguagem ou render. Essas diferenças foram auditadas sem transformar bytes/algoritmos em requisito de paridade (ADR-0107):

| Fixture | Páginas atual/vanilla | Texto normalizado igual? | Links iguais? |
|---|---:|---|---|
| empty | 1/1 | True | True |
| header_report | 5/5 | False | True |
| math_1 | 1/1 | True | True |
| math_10 | 2/2 | True | True |
| math_60 | 10/10 | True | True |
| math_supported | 5/5 | True | True |
| mixed_report | 5/5 | False | True |
| one_line | 1/1 | True | True |
| outline_100 | 5/5 | True | True |
| prose_20 | 1/1 | True | True |
| prose_500 | 20/20 | True | True |
| references_100 | 5/5 | True | True |
| references_500 | 23/23 | False | True |
| unicode_repo | 1/1 | False | True |

O grafo builtin compartilhado corrige uma diferença observável de igualdade da linguagem: dez comparações de módulos agora coincidem com o vanilla. A igualdade de módulos de usuário importados continua sendo dívida anterior (probe inicial), e o caso contextual `context_body` continua com resultado diferente do oracle. Não são regressões introduzidas pelo pacote. [Semântica](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/semantic-comparison.json) · [Auditoria das saídas](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/vanilla-output-audit.json).

## Validação e recursos

RED real de retenção: quatro falhas antes da implementação; GREEN: oito testes passaram. Suítes: core 8559 passaram/5 ignorados; infra 1819/2; shell 120; wiring 40/40. Build workspace, documentação com warnings negados, Clippy workspace/all-targets `-D warnings` e crystalline-lint com zero avisos passaram. Dois testes wiring inicialmente encontraram bloqueio de bind local no sandbox; o mesmo ELF passou os 40 com a exceção de loopback autorizada. [Recibos](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/full-unit-suites.json).

Um job Cargo, teto AS 12 GiB, interrupção por RSS da árvore acima de 8 GiB, mínimo disponível 6 GiB, sem swap. Pico de compilação registrado: 4198,3 MiB. Runtime e controlador limitados a 1 GiB AS. Na CLI, o HWM mediano de references_500 caiu de 181,8 MiB para 61,9 MiB (vanilla: 48,6 MiB); o maior HWM observado em todo o corpus atual foi 108,0 MiB. Medianas de wall e HWM por fixture/estado também estão em analysis.json; wall do watch inclui a interação com o observador e não é a métrica primária. Fontes/ambiente/afinidade/flags/inputs congelados nos recibos. Mesmas flags efetivas de release para anterior e atual. Nenhuma mudança de fonte durante as medições.

40 edições, mixed_report: rss_bytes MiB: inicial 45.0, final 45.0, máximo 47.5; hwm_bytes MiB: inicial 46.8, final 47.4, máximo 47.5.
40 edições, references_500: rss_bytes MiB: inicial 57.5, final 70.4, máximo 71.8; hwm_bytes MiB: inicial 64.2, final 71.6, máximo 71.8.

No caso de referências, o RSS cresceu cerca de 13 MiB entre início e fim; a janela de 40 edições não demonstra retenção ilimitada nem prova ausência de crescimento em sessões muito maiores. O máximo observado permaneceu abaixo de 72 MiB nesses dois controles.

Os encerramentos externos de controladores foram preservados nos recibos; causa não identificada. A retomada manteve todas as amostras concluídas e renomeou diretórios incompletos. Não houve descarte seletivo de medições.

## Proveniência e publicação

Oracle CLI: commit `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, tree `483bd6f1d8ac8810db47b65336d402647955fd17`, SHA-256 `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`. Não é identificado apenas pela versão.

Produto instalado localmente em `/repos/Antigravity/typst-crystalline/lab/target/product/release/typst`, SHA-256 `61be8cf9e6b32bb4df09e2efbd98f4c0b3ce4c6aa6d8109b6b5384bee614cbb4`; binário anterior preservado em `/repos/Antigravity/typst-crystalline/lab/experiments/gains-product-vanilla-2026-11-18/installed-before`. O instalado é o ELF produtivo sem marcadores, não o último artefato do target usado para construir o observador.

O smoke test do executável já instalado passou, com texto/estilo/geometria/navegação e raster iguais ao fresco qualificado. [Recibo do instalado](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/installed-smoke.json).

[Build](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/after-build.json) · [Fontes e ELFs congelados](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/after-frozen.json) · [Observadores](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/watch-frozen.json) · [Plano](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/measurement-plan.json) · [CLI bruto](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/cli.json) · [Watch bruto](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/watch.json) · [Análise](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/analysis.json) · [RAM](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/long-ram.json) · [Estado final](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/final.json).
