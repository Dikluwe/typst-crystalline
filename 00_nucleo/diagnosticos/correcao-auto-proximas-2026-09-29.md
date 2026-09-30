# Fecho das próximas frentes de página automática

**Data:** 2026-09-29. **Alvo:** vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`. **Estado:** as três classes abertas no diagnóstico anterior estão fechadas na matriz medida; as divergências RTL anteriores são outra frente.

## Evidência antes da decisão

O lote inicial de 20 fontes em [`before/results.json`](correcao-auto-proximas-2026-09-29/before/results.json) tinha 11 páginas com dimensões iguais ao vanilla. As fontes foram escritas sem newline artificial ao fim do fixture. `abc\n` já coincidia; o caso divergente era `abc\n\n`, uma linha em branco terminal: 19.872×31.536 pt no Tekt contra 19.872×8.568 pt no vanilla. `#text("")` dava página física 3×3 pt no Tekt contra 3×8.568 pt; a variante com 18 pt exigia 12.852 pt de altura. Dois parágrafos apenas de espaços tinham largura e altura diferentes porque a primeira linha não era fechada. Uma série exploratória por tamanhos 10–18 pt em [`size-probe.log`](correcao-auto-proximas-2026-09-29/size-probe.log) indicou o uso do estilo exterior na baseline de uma linha literal; seus números intermediários não servem como prova de fecho, que vem da medição final com proveniência completa.

O estado inicial exato, com HEAD, working tree, hashes dos owners e dos dois binários, está em [`initial.json`](correcao-auto-proximas-2026-09-29/initial.json); os dois owners acrescentados durante o refinamento estão em `scope-extension.json`. Os arquivos anteriores estão em `before-source/`. O binário vanilla tem SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. A observação de geometria e texto não implica intenção histórica nem exige reproduzir estruturas internas do vanilla.

## Alteração segregada

Os oito pares owner→Prompt L0 estão em [`scope.json`](correcao-auto-proximas-2026-09-29/scope.json), com selos em [`scoped-seals.json`](correcao-auto-proximas-2026-09-29/scoped-seals.json). Cada L0 foi medido e atualizado antes de seu código. Os testes de texto vazio e linhas causais falharam antes da implementação (`red-empty.log`, `red-parbreak.log`) e passaram depois (`green-empty.log`, `green-parbreak.log`). Três testes existentes de `Par` vazio descobriram uma dupla contagem intermediária; após o ajuste, passaram em `green-paragraph.log` e na suíte completa em `core-tests.log`.

- `text.rs` conserva `Text("")` como unidade textual de largura zero, com estilo e baseline, sem glyph/tinta. Dentro de `Par` já classificado como vazio, `paragraph.rs` preserva seu caminho de altura virtual e não emite a unidade outra vez.
- `html_frame.rs` distingue linha textual causal de materialidade visual: um `TextSeparator` Literal pode fechar uma linha; separadores Markup/Newline iniciais e marcadores não a criam. `line_commit.rs` e `mod.rs` usam o predicado causal apenas no fechamento/finalização.
- `text_baseline.rs` admite a baseline da linha literal com o estilo transportado pelo carrier, restaurando o estilo exterior. `page_geometry.rs` usa a baseline já admitida na página só de carriers, sem somar o topo tipográfico duas vezes.
- `parbreak.rs` fecha a linha literal, confirma o fundo da linha anterior e deixa o espaçamento para o próximo conteúdo. `paragraph.rs` continua a confirmar o avanço de um `Par` vazio **explícito** após conteúdo; uma linha em branco terminal sem conteúdo não aumenta a página.

Não foi acrescentado campo público, flag, modo padrão ou fase de pipeline. Os valores tipográficos vêm de `FontMetrics`, do estilo do carrier e do espaçamento já resolvido; o código não contém compensação numérica de sonda.

## Resultado e limites

Com o binário final Tekt SHA-256 `ec14a0ccdc0211506cd6f35206e8fa90a1f29c42c65dd66ee7053f1baec62138`, a matriz refinada passou de 11/20 para **20/20** páginas iguais ao vanilla, com **20/20** extrações textuais iguais. A matriz anterior passou de 24/33 para **33/33**, também com extração textual igual. Fontes, PDFs e resultados estão em `final/` e `final-legacy33/`, com hashes de código, prompts e binários em [`final-receipt.json`](correcao-auto-proximas-2026-09-29/final-receipt.json).

`cargo build --offline` e `cargo build --offline --release -p typst-wiring` passaram. Core: 6378 unitários, 8 de integração e 3 doctests passaram; 2 existentes ignorados. Infraestrutura: 1137 unitários e todos os grupos de integração passaram. `crystalline-lint . --format sarif` terminou com zero erros, 504 warnings e 1559 notes em `lint-final.sarif`.

Os controles bilaterais anteriores mantiveram 36/36 origens de tinta uniforme (erro máximo 0.00003 pt), 401/401 entradas de regressão dentro de 0.001 pt, 42/42 páginas extras com dimensões iguais e nenhuma nova divergência no corpus de escopo/mistura. A sonda RTL manteve as 12 diferenças já documentadas de hebraico e espaços literais iniciais; não são consequência nem objetivo desta frente.
