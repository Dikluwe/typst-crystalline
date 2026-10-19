# Contexto integral de bidi — materialização e medição, 2026-10-05

O contrato público confirmado foi materializado: L1 conserva a entrada lógica imutável e a origem UTF-8 das folhas; L3 resolve o input completo, conserva embeddings explícitos de direção e usa a mesma resolução na ordenação e no shaping. Nenhum unsafe novo, método de FontMetrics novo, dependência Unicode em L1, modo/flag ou mudança de fase.

O resultado demonstrado é de correção e recuperação da arquitetura. **Não houve ganho de tempo da CLI demonstrado nesta frente.** Não fecha a paridade universal nem todas as heranças de construção.

## Aprovação, estado e proveniência

Humano confirmou “confirmo” para o manifesto SHA-256 `1ae7513eb3af426754909954aef165c97257a9f62d3911fbb111e2002fac4043`. `bidi-contexto-aprovacao-2026-10-05.json` registra a ratificação separadamente; o manifesto original permanece congelado. Os três SHA-256 públicos conferem com os textos congelados em before.json.gz; CodeHash/status atuais não substituem a prova da aprovação.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**, com modificações anteriores preservadas. execution.json.gz contém o `git diff HEAD --stat` integral, status/untracked, fontes, hashes, owners e logs com UTC. A medição CLI de benchmark.json começou em `2026-10-06T01:19:14.238919+00:00`; verification.json.gz guarda UTC, entrada completa, comandos, stdout/extração, caixas por página e hashes dos três binários. O baseline é o release antes desta materialização, comprovado também no diagnóstico anterior de direção/alinhamento. O before.json.gz desta rodada foi capturado depois de criar o stub RED ainda não registrado no hub; esse stub não fazia parte do baseline executável.

- Baseline Tekt: `64e35e6b7e69881b44c287daac6bc0d3bb668993f4019e97e1002909edcc8e27`.
- Candidato Tekt: `3159634c96c3008ffd2b904d2d6a0f3b5ac439717c71b708673eee20e1f8a8ee`.
- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

O release depois de limpar somente espaços de L0 e seus headers tem os mesmos bytes do candidato verificado nos PDFs. Não usar a string de versão como proveniência. Fixtures deste benchmark não são a fixture histórica de 223,38ms/144,27ms; não comparar aqueles números diretamente.

## Implementação

Três novos consumers: entities/bidi_context.rs, compiler/layout/bidi_origin.rs e infra/bidi_context.rs, cada um com L0 próprio. O inventário original tinha 72 candidatos textuais; a materialização/linhagem efetiva abrangeu 58 owners, incluindo migrações mecânicas de constructors/patterns e testes. O núcleo paragraph-bidi-origin.toml centraliza as obrigações comuns e possui pins válidos nos consumers.

- Input/Span imutáveis, bounds/UTF-8/subspan verificados, ocorrências repetidas com ranges distintos. Não localizar uma origem por busca de string.
- Cursor estrutural da sequência/parágrafo; direção compartilhada separada da direção do trecho. SetPage e Parbreak delimitam a preparação; uma recusa integral não permite certificar um filho parcial.
- Text e separadores recebem spans exatos; translações, checkpoints e subframes conservam/restauram ou isolam o estado. Folhas Text vazias conservam estilo/posição/span; não são removidas pela permutação.
- Em L3, registry proprietário e BidiInfo emprestados com lifetimes normais. LRE/RLE/PDF virtuais ficam somente na representação auxiliar. Não adicionar nem remover esses caracteres no texto autoral. Unicode é resolvido uma vez por input do registry compartilhado na compilação paginada.
- As consultas copiam somente o slice de classes/níveis já calculados e aplicam regras de fim de linha nele. A API visual_runs original clonava todos os níveis do input a cada consulta (unicode-bidi 0.3.18 lib.rs:546–560,669–676). line-view-before.json.gz conserva fonte/hash e o helper anterior; line-view-benchmark.json tem sonda isolada, não mede a CLI nem prova ganho global.

Certificação inicial: parágrafo textual incremental simples, finito, sem objetos, matemática, links/envelopes opacos, células/colunas/subframes, floats, justificação/optimized, tracking, highlight, baseline deslocada, scripts/smallcaps ou transformação sem mapeamento exato. Styled que introduz pdf.marker.tag/outline.link.destination também recusa. LTR simples continua no planejamento anterior. Linebreak não justificado usa LF; justify=true fica fora deste carrier unitário, conforme vanilla inline/collect.rs:191–194.

Auditoria do cache de células: a emissão medida acontece com is_sub_frame e contexto exterior suspenso; esses produtos têm bidi None, portanto não reutilizam input de parágrafo exterior por igualdade de texto/caixa. Nenhuma retenção entre compilações/watch foi acrescentada.

## Refutações e correções feitas na integração

Os três testes iniciais da entidade e o teste da coleta começaram RED. O primeiro teste de reordenação com base RTL e primeiro trecho latino também reproduziu a ordem antiga. O teste do pipeline completo mostrou que a avaliação direta não cobria SetPage inicial: a recusa da sequência exterior bloqueava a certificação do texto seguinte. A tentativa de tratar somente Located não resolveu aquele RED; pipeline-module.log preserva a árvore verdadeira. Corrigiu-se a fronteira SetPage, mantendo Located transparente somente para body textual certificada.

numeric-explicit refutou U+2028 como representação universal: vanilla usa LF para a quebra padrão e reinicia o parágrafo Unicode; wrap suave não reinicia. Teste específico começou RED e passou. Dois espaços fracos através do fim de Styled também avançavam duas vezes; o desvio observado de 3,4966pt desapareceu ao conservar o primeiro carrier/estilo e consumir sem emitir o segundo. Literal não é colapsado. A fonte vanilla typst-realize/spaces.rs:25–31,68–99,128–132 confirma os estilos do primeiro espaço.

A antiga fixture de wrap baseada no segundo espaço fraco foi corrigida para inserir um novo B antes da quebra; agora testa fonte “A B”, não o efeito incorreto de duplicação. Logs RED/primeiras falhas e todos os candidatos ficam arquivados. Testes posteriores recusam deltas tipados/envelopes opacos e conservam Empty Text, além de rollback/subframe.

## Paridade medida e limites

54 fixtures bilaterais: 47 coincidem simultaneamente na extração sem formatação bidi introduzida pelo Poppler e nas caixas agrupadas, tolerância <0,05pt. 11 controles passaram desse critério do baseline para o candidato; não houve regressão nesse critério nos controles antes positivos. Casos corrigidos: bidi-contexto-rtl_auto_mixed, bidi-contexto-rtl_override_mixed, bidi-contexto-ltr_auto_mixed, bidi-contexto-ltr_override_mixed, bidi-contexto-ltr_numeric, bidi-contexto-rtl_terminal_space, direcao-alinhamento-rtl_mixed, direcao-alinhamento-rtl_wrap, direcao-alinhamento-ltr_wrap, weak-font-transition, strong-mixed.

A comparação bruta de palavras do -bbox continua registrada. Sufixos colados geram quantidades diferentes de caixas entre engines; só agrupar caixas contíguas (<0,05pt), na mesma linha/página, sem inventar espaço ou rearranjar caracteres. Em todas as 18 fixtures anteriores de sufixos, texto e caixas agrupadas coincidem. Isso compara a forma da palavra/tinta, não a segmentação interna do PDF. Raw text/codepoints, caixas estritas e unmatched também permanecem no receipt.

Resta separar:

1. **Block e Align:** três controles continuam com geometria diferente; corpos opacos/subframes estão fora da certificação. Texto coincide, mas isso não fecha suas âncoras/precedência.
2. **Controles autorais (RLM/embedding):** a tinta coincide na renderização, mas a extração continua com controles extras/caixas agrupadas incorretamente. Não foram apagados controles autorais para forçar igualdade. É dívida de cópia/exportação; a identidade lógica do span não a resolve sozinha.
3. **Literal terminal:** posições/tinta coincidem. Vanilla inclui um espaço literal no bbox de “123 ”; Tekt emite carrier separado, por isso a caixa estrita não é uma medida equivalente da tinta.
4. **Emph árabe/fallback:** defeito visual preexistente, não apenas leitura/ordem. Renders mostram glifos incompatíveis no Tekt antes/depois; vanilla usa árabe regular. pdffonts registra DejaVuSans-Oblique no Tekt e somente DejaVuSans regular no vanilla. shaper/shaping.rs:358–362 conserva família na saída mas deixa a variante do estilo pedido; export/render.rs:255–259 deriva a fonte novamente por família/variante. **Inferência a investigar:** perda de identidade da face do fallback entre shaping e export. Refuta-se se a face efetivamente usada pelo shaper for comprovadamente a mesma que a incorporada; nesse caso auditar GIDs/associação por outro caminho. Nenhum novo contrato de fonte foi materializado.

raster.json registra Poppler, hashes dos PDFs/imagens, 600dpi, máscara <180 e caixas da tinta; as quatro páginas relevantes foram inspecionadas. Literal terminal e os dois controles autorais têm zero pixels de tinta sem correspondente em ambos os sentidos dentro de 1px (0,12pt). Emph não passa. Essa resolução é complementar às caixas de texto, não prova de tolerância <0,05pt nem de cópia correta.

## Tempo da CLI

Duas rodadas de aquecimento e quinze rodadas pareadas com ordem permutada, por fixture; SOURCE_DATE_EPOCH=0 e RAYON_NUM_THREADS=1. CLI completa inclui fontes, compilação e PDF. Delta é after−before pareado; IQR representa a dispersão das diferenças, não intervalo de confiança.

| Fixture | Antes ms | Depois ms | Vanilla ms | Delta pareado ms | IQR do delta ms |
|---|---:|---:|---:|---:|---:|
| grid4 | 163.24 | 162.48 | 137.19 | -0.07 | -1.42 a +3.31 |
| grid320 | 196.92 | 196.17 | 149.02 | -0.32 | -2.69 a +0.61 |
| rtl160 | 161.21 | 160.53 | 129.67 | -0.34 | -1.06 a +0.45 |
| mixed160 | 160.69 | 161.31 | 130.43 | +0.63 | -0.67 a +2.99 |

As diferenças cruzam zero e permanecem pequenas diante da dispersão; o conjunto não demonstra ganho temporal nem regressão persistente. O objetivo desta rodada foi corrigir a entrada/reutilização, mantendo a medição independente da conclusão arquitetural.

## Verificação

- L1: **6528 testes passaram**, 5 previamente ignorados.
- L3: **1165 testes passaram**.
- cargo build e release concluídos; git diff --check limpo.
- V15/V26 sem resultados; nenhum V5 nos 58 owners desta frente.
- Linter global: **531 warnings, 1575 notes, zero errors**, versus 533/1572/zero antes. Não equivale a zero violations global. Não foram corrigidos arquivos históricos para esconder deriva; a comparação multiset em execution.json.gz conserva entradas adicionadas/removidas, inclusive troca de mensagem no V5 histórico e mudança de notas de código migrado.

A proposta Coverage::from_ranges e outros contratos públicos pendentes permanecem intocados. Não houve commit, PR ou re-sync do vanilla.
