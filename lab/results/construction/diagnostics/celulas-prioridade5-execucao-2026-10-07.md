# Prioridade 5 — reuso de medições de células

Executada na sessão iniciada em 2026-10-07 e concluída em 2026-10-08. Decisão nova: conservar a correção interna limitada a tokens simples. A contagem confirma remoção de trabalho e os ensaios mostram redução repetida do custo de layout. Não certifica cache para todo conteúdo de Cell nem velocidade geral do compilador.

## Medir antes de decidir

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**, com mudanças anteriores preservadas. Todos os recibos estão em `00_nucleo/diagnosticos/celulas-prioridade5-2026-10-07/`. `before.json.gz` guarda fontes, L0, lista exata de alterações, diff stat, UTC e identidade do binário anterior. `candidate-state.json.gz` guarda a árvore final; SHA-256 `a8d1b11e58ab05aab3dc0c3c2e4b431865ccd261670d2128adbfb8fc1c842c4a`. `scope.json` confirma diferenças em apenas um consumer produtivo e seu owner L0.

A sonda `audit.log` encontrou uma folha Text integral para `12.34`, `item-2`, `abc_def` e `a/b`, mas o classificador alfanumérico as recusava. Ampliar somente ASCII para ASCII gráfico passou nos testes, mas não reduziu o trabalho da CLI: `count-before-source.json.gz` conservou 640 subframes até para tokens alfanuméricos. Esse experimento intermediário foi refutado como melhoria produtiva isolada.

A inspeção `carrier.log` encontrou `Source(Text("item-0.25"))`; `inspect.log` encontrou pool habilitada e produto de uma folha Text. O transporte causal Source, introduzido anteriormente, impedia o classificador de chegar ao token. A fonte `01_core/src/compiler/layout/mod.rs:1158` já delega Source diretamente ao body. `01_core/src/entities/content/source.rs` remove somente esse wrapper para classificação. Seu L0 e o núcleo `content/source-origin.toml` legitimam transparência de linguagem e ocorrência única. Isso não autoriza retirar Styled, Located, Realized ou executar efeitos de novo.

## Correção e fronteira

L0 editado primeiro, duas rodadas RED→GREEN e resselo local. Owner exclusivo: `00_nucleo/prompts/compiler/layout/grid/measurement_cache.md`; consumer: `01_core/src/compiler/layout/grid/measurement_cache.rs`. O owner conserva o pin de realização de células e passa a consumir também o núcleo vigente de origem lexical. Nenhum núcleo existente ou owner externo foi modificado.

O cache classifica placement, coordenadas preparadas e token através de `source_body()`, mantendo o carrier original e seu span. Aceita ASCII gráfico sem espaços/controles quando o produto continua uma única folha Text integral. Preserva região, origem/largura, métricas laterais, estilo efetivo, placement sequencial, guardas ambientais e consumo único. A pool pertence somente à mesma invocação do grid. Fonte, render, assinatura pública, fase e modo padrão continuam sob seus contratos vigentes; ajuste interno em fluxo contínuo ADR-0127.

Styled/Located/Realized/ShowRecipe, efeitos/contexto, Unicode, múltiplos tokens, alinhamento especial, placement/spans especiais e produtos transformados continuam exigindo fallback conforme as guardas existentes. A transparência Source não torna seu body automaticamente elegível.

## Contagem de produção

GDB contou todas as especializações locais de `layout_sub_frame_with_origin` identificadas por nm, nos releases, separado do benchmark. Recibo `count.json.gz`; código `count.py` e fixtures conservados.

| Caso | Anterior | Final |
| --- | ---: | ---: |
| grid, 320 tokens literais com pontuação | 640 | 320 |
| table, 320 tokens literais com pontuação | 640 | 320 |
| grid/table, 320 tokens literais alfanuméricos | 640 | 320 |
| grid, 320 tokens interpolados com pontuação | 640 | 320 |
| table, 320 textos com espaços | 640 | 640 |
| table, 320 textos Unicode | 640 | 640 |
| show/estilos | 8 | 8 |
| tokens estreitos com quebra | 4 | 4 |

São entradas de subframe dessas fixtures, não uma redução de 50% do tempo total. A correção também restaura o reuso alfanumérico que Source bloqueava.

## Tempos

Ensaio UTC `2026-10-08T03:06:06.439154+00:00` a `2026-10-08T03:07:24.953405+00:00`. Três rodadas, dez pares medidos por caso em cada rodada, aquecimento excluído, ordem alternada anterior/final, mesma CPU, RAYON_NUM_THREADS=1, PDF sem tags e pretty=false. Nenhum build, suíte ou debugger concorrente. O monitor verifica processos impeditivos antes de cada amostra. `bench.json.gz` guarda comandos, fontes, binários, arquivos produtivos/L0/fontes/configuração, timestamps e amostras; identidades foram revalidadas ao fim. `summary.json` conserva os resultados; `analyze.py` reproduz a análise.

As variações percentuais abaixo são **medianas das razões pareadas**, não razões das medianas absolutas.

| Caso, 320 células | Layout anterior → final, ms | Layout | CLI anterior → final, ms | CLI |
| --- | ---: | ---: | ---: | ---: |
| grid, pontuação literal | 22,40 → 18,27 | −19,52% | 151,32 → 147,16 | −3,13% |
| table, pontuação literal | 24,36 → 20,56 | −18,40% | 160,98 → 156,18 | −2,90% |
| grid, pontuação interpolada | 22,56 → 18,06 | −19,76% | 152,06 → 146,46 | −2,65% |
| table, pontuação interpolada | 27,69 → 22,83 | −18,36% | 181,98 → 178,10 | −1,57% |
| grid, alfanumérico interpolado | 24,66 → 19,87 | −20,35% | 170,32 → 166,33 | −2,87% |
| table, espaços (fallback) | 27,68 → 27,62 | −0,15% | 183,61 → 181,31 | −0,71% |

Layout melhorou nas três rodadas dos cinco casos elegíveis. Nas células literais, CLI também melhorou nas três rodadas: razões grid 0,969/0,967/0,981; table 0,970/0,968/0,984. Grid interpolado teve razão CLI 1,051 na terceira rodada, e alfanumérico 1,004; não certificar ganho repetido de CLI inteira nesses casos. Controle com espaços oscilou em torno do mesmo custo de layout. Três rodadas e medianas descritivas não constituem uma estimativa universal ou intervalo de confiança formal.

RSS mediano grid/table literal: 44.322 → 44.502 KiB e 46.598 → 46.730 KiB. Esse pequeno aumento observado não é uma atribuição causal de alocações. Não somar estas porcentagens às prioridades anteriores nem extrapolar para conteúdo arbitrário.

## Preservação e gates

`geometry.json.gz` compara 19 casos com anterior/final/vanilla pinado: texto extraído, páginas/dimensões, caixas de palavras e rasters a 36 dpi ficaram iguais entre os dois Tekt; famílias/fontes extraídas também coincidem. Inclui pontuação literal/interpolada, alfanuméricos, espaços, Unicode, estilos/show, RTL/tinta/insets, multipágina, placement/spans, rows fixas, tokens estreitos e efeitos. O contador do controle de show-cell continua 4 em ambos os Tekt e vanilla.

O caso de efeitos possui espaçamento extraído diferente do vanilla; RTL possui ordem extraída diferente; tokens estreitos possuem quebra diferente. Todos já se reproduzem no binário anterior e não mudaram neste pacote. Não normalizar whitespace para declarar paridade, nem fechar essas divergências por igualdade entre os Tekt.

RED inicial: 1 teste falhou e 2 passaram. RED Source: 2 falharam e 5 passaram, demonstrando falta de captura e placement indevidamente classificado sob Source. GREEN final: 7 testes locais passaram. Suíte completa do núcleo: **6.642 passaram, zero falhas, cinco ignorados**. Build dev e release passaram. Lint: **zero erros**, 535 avisos e 1.640 notas herdadas; não é zero violations global. Diff check dos dois owners limpo. Não foi reexecutada a suíte inteira do workspace; falhas HTML já registradas na prioridade 4 permanecem fora desta certificação.

## Proveniência e reprodução

- Anterior P4: SHA-256 `fc5b2571a149763526c5686149a538259d64a045071103609600bb2823491d4d`, cópia `/tmp/celulas-prioridade5-2026-10-07/previous-fc5b2571`.
- Intermediário ASCII sem Source: SHA-256 `f2eea959438e8b4a626abd4d04578ca667089ab537d8f390bf2eae63204662b9`; árvore `final-state.json.gz`, contagem intermediária conservada.
- Final: SHA-256 `897aa1261a069ef1dc601a0e5cb256380e901ac6a0324d1a288b405d5f6d0ff0`, `target/release/typst`.
- Vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Usado nos controles de linguagem, sem nova disputa de velocidade com upstream.
- Selos finais: código `964e6352`, prompt `60753643`; `seals.json`, `source.diff` e `l0.diff` conservados.

Build: `cargo build --offline -j 1` e `cargo build --offline --release -j 1`. Testes: `cargo test --offline -p typst-core --lib -j 1 -- --test-threads=1`. Lint: `crystalline-lint .`. Scripts `fixtures.py`, `verify.py`, `count.py`, `bench.py`, `analyze.py` e logs ficam no diretório deste pacote; rodar contador e tempos sequencialmente. Antes de repetir, conferir hashes dos binários, fontes e estado, preservando a cópia anterior.

O sandbox bloqueou ptrace inicialmente; o recibo foi conservado. Uma revisão automática do GDB expirou e rejeitou a execução por prazo, sem julgamento de insegurança. A única repetição permitida foi autorizada e concluiu a contagem. Não há verificação pendente por essa rejeição.

Prioridade 5 concluída neste recorte comprovado. Conteúdo rico e reuso entre compilações continuam sem certificação; a prioridade 6, incrementalidade ampla, exige investigação e L0 próprios.
