# Tracking uniforme na seleção global de parágrafo — 2026-09-29

Estado: working tree não commitada sobre `17f13b1df79b87ea065ee62e78164b18e477da3d`. Proveniência integral de estado, SHA-256 de fontes, prompts e binários, e resultados resumidos em `closure.json`; comandos e dados por caso em `focused-after/results.json`, `after/results.json`, `regressions-after/results.json` e `rtl-after/results.json`. Oráculo: vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Binário cristalino final SHA-256 `f10c23135e444dc94c446e8d3522b8f34ed1d4c7c89a90718e33c24ed15932de`.

## Medição e causa

No documento paginado de 12 palavras, Simple e Optimized sem propriedade `tracking` já coincidiam com o vanilla (`no-tracking-baseline.json`). Com `tracking` explícito, até `0pt`, a preparação recusava a seleção global; as oito combinações focadas divergiam em distribuição de palavras por página (`focused-before/results.json`). O código anterior em `paragraph_prepare.rs` rejeitava `Some(tracking)` sem medir a largura. A sonda de 32 casos inicial misturava outras condições e foi tratada apenas como exploração, não como prova causal.

Depois de habilitar tracking uniforme e somar os intervalos em seleção e emissão, sete dos oito casos focados coincidiam. Em `0.2pt` + Simple, a palavra `eta` cabia por 0,064 pt segundo o custo cristalino, mas passava para a página seguinte no vanilla. A fonte pinada molda o espaço de quebra antes de zerar o avanço (`crates/typst-layout/src/inline/line.rs:127-195,285-335`) e aplica tracking entre glifos (`inline/shaping.rs:1269-1293`): o custo natural conserva o intervalo do último glifo até o espaço moldado. Essa inferência foi refutada ou confirmada pela medição bilateral depois do ajuste, sem atribuir intenção histórica.

## Correção e separação

O núcleo `paragraph-selection.toml` e os quatro L0 owners de preparação, seleção, emissão e planejamento foram atualizados antes do código. A preparação admite apenas tracking finito, uniforme e mensurável em texto simples LTR, recusando escopo aninhado, hifenização combinada, largura efetiva negativa ou não finita e os demais casos fora do subconjunto certificado. A seleção Simple/Optimized soma intervalos internos entre palavra e espaço e o intervalo do espaço terminal no custo da linha não final; a emissão conserva os avanços fixos entre unidades sem duplicar tracking já incluído na largura de cada palavra. O `match` estático, a proteção regional e o fallback incremental permanecem nos seus owners. O pin do núcleo foi propagado e ressellado somente nos 12 pares L0↔consumer listados em `scope.json`; recibos em `nucleus-pin.json` e `scoped-seals.json`.

## Validação

- Oito casos focados: mesma distribuição e caixas de palavras dentro de 0,001 pt; desvio máximo extraído ≤ 0,000002 pt (`focused-after/summary.json`).
- Matriz de 32 combinações de altura, tracking, estratégia e justificação: zero diferenças de status, páginas, sequência de palavras ou caixas acima de 0,001 pt (`after/summary.json`).
- Replay histórico de 401 casos: 400 preservados. O único caso alterado, `after-pdf/tracking`, é o testemunho cujo defeito se corrigiu; a nova geometria coincide com o vanilla na sonda bilateral (`regressions-after/results.json`).
- Corpus RTL de 66 casos: zero diferenças novas em texto, página ou origem de glifos acima de 0,001 pt (`rtl-after/summary.json`).
- Teste RED de preparação, teste de fronteira `simple_break_includes_tracking_to_trimmed_space`, suite `typst-core --lib` com 6381 aprovados, `cargo build`, build release e `crystalline-lint .` com exit 0. Os 171 avisos V5 restantes são diagnósticos históricos; nenhum V15/V26. Os 12 snapshots `.rs` deste diagnóstico foram preservados como `.rs.txt` para não serem interpretados como consumers produtivos pelo linter.

Esta certificação cobre somente o subconjunto uniforme descrito no L0. Tracking misto, hifenização junto com tracking e conteúdo complexo continuam no percurso incremental e exigem medição própria antes de ampliar o contrato.
