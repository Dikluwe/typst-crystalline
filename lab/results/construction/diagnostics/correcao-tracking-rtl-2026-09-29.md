# Correção das fronteiras e continuações RTL — 2026-09-29

O caminho incremental agora aplica um intervalo assinado de tracking entre unidades textuais de estilo uniforme também quando `dir:rtl`. Nas linhas RTL não justificadas que continuam após wrap, o fechamento reserva ainda o intervalo terminal medido pelo vanilla. A linha final, LTR, escopos e exportadores mantêm seus contratos.

## Causa e medição anterior

`before-pdf/` mede 66 fontes e 198 PDFs contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, com binários, comandos, fontes, UTC, HEAD e working tree em `initial.json` e `before-pdf/provenance.json`. `text_tracking.rs` vetava qualquer estilo RTL, embora Word/Space já soubessem aplicar a fronteira assinada e o alinhamento direito de Cursor usasse a largura real. A perda era 0.8pt no primeiro dos três termos latinos sob tracking ±0.2pt e 0.4pt entre os dois termos árabes explícitos; 0pt latino coincidia. Vanilla inline/shaping.rs:1269-1295 soma tracking também na direção negativa, dentro de runs. Isto descreve comportamento observado, não intenção histórica.

A primeira correção removeu só o veto direcional, preservando filtros de estilo lexical, math, objeto, valor e linha vazia. `after-pdf/` da primeira rodada mostrou os termos e o árabe alinhados, mas linhas RTL continuadas deslocavam-se por exatamente um intervalo ±t enquanto a última coincidia. `wrap-initial.json` e `wrap-before/` guardam estado e fontes desse segundo gate, anterior aos novos testes e código. A diferença refutável motivou transportar a propriedade de continuação já conhecida em line_commit ao alinhamento de Cursor.

## Contratos e código

- `text_tracking.rs` é owner da fronteira assinada e do intervalo terminal da última unidade textual RTL. Este último ignora separadores finais/marcadores diretos e recusa math/objetos. Não conta caracteres nem introduz constantes de compensação.
- `line_commit.rs` envia um booleano privado de continuação apenas em `flush_continued_line` sem justificação; fechamentos terminais e explicitamente justificados preservam o caminho anterior. Nenhum campo novo entrou no Layouter ou em checkpoints.
- `cursor.rs` aplica o valor derivado à âncora direita somente nessa linha; não desloca glyphs internamente nem consulta o cursor que inclui espaço terminal nominal.
- Word/Space continuam a projetar/aplicar a mesma fronteira exatamente uma vez. Prompts L0 destes cinco owners foram atualizados antes do código; os Núcleos compartilhados não mudaram. Scope, selos e diff estão em `scope.json`, `scoped-seals.json`, `review.diff.txt`.

## Testes e observáveis

Teste causal da fronteira RTL e teste de fechamento assinado deram RED→GREEN (`red.log`, `green.log`, `wrap-red.log`, `wrap-green.log`). O primeiro rascunho do segundo teste não compilou por import ausente; o RED válido está no wrap-red.log após corrigir somente o teste.

Core: 6.374 unitários + 8 integração + 3 doctests; infra: 1.137 unitários + 34 integração. Total 7556 passaram, sem falhas; dois testes anteriores continuam ignorados. Builds offline dev/release passam. Lint: zero erros, 504 warnings e 1559 notes. Evidência em logs e lint-final.sarif.

Das 66 entradas RTL/198 PDFs, 54 reproduzem mesma sequência de tinta/páginas com diferença máxima 0.000031000pt, tolerância 0.001pt. Incluem Latin simples, espaços repetidos, literais, adjacência, estilos mistos, wraps e árabe explícito nos seis valores de tracking. As 12 entradas abertas são hebraico e espaços literais iniciais; seus valores estão em after-pdf/summary.json. Padrão e compacto preservam as origens dos glyphs visíveis, diferença máxima 0.000100000pt. Não se usa igualdade de bytes PDF como gate de língua.

Os 36 controles LTR uniformes mantêm máximo 0.000030pt; os 42 de escopos mantêm paridade dentro de 0.001pt; 401 regressões previamente verificadas preservam geometria/texto/diagnósticos com tolerância 0.001pt e zero falhas. O lote ampliado de 42 entradas agora só mantém duas diferenças de dimensão automática em páginas com apenas espaços; o caso rtl-explicit ±0.2pt fecha. Evidência em uniform/, mixed-regression/, regressions/ e extra-after/. Este replay verifica preservação frente ao estado anterior e mede casos RTL novos contra vanilla.

Binário final SHA-256 `612f83167d0d4a40409fd66962ae48573b9b1c55bdbc9c3cb7131b1ea976da94`; o vanilla pinado tem SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. `final-receipt.json` confirma fontes intactas desde a medição final.

## Limites e próxima frente

Hebraico explícito ainda difere 7.68pt até com tracking zero: foi mantido fora desta correção de fronteiras. Espaços literais iniciais em RTL diferem mesmo em 0pt e exigem investigar posicionamento/morfologia, não uma constante de tracking. Preparação global/paginação com tracking e páginas auto compostas só por espaços também continuam abertas. Esta rodada não alega paridade RTL geral nem ganhos de desempenho.

Histórico: [precisão e fronteiras LTR](correcao-tracking-2026-09-29.md) → [escopos lexicais](correcao-tracking-escopos-2026-09-29.md) → esta correção RTL.
