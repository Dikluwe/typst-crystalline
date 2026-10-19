# Memória — prioridade 1: vínculo de fontes sob demanda

2026-10-08. Correção interna em fluxo contínuo conforme ADR-0127, autorizada pelo dono (“faça o 1”). L0 editado primeiro; nenhum contrato público, fase, comportamento padrão ou alias FontKey alterado.

## Resultado e alcance

`bind_document` inicia com Cow emprestado. A primeira face ausente com família disponível materializa o prefixo uma única vez; as seguintes são anexadas à tabela própria, na ordem de primeira ocorrência. Prefixo completo, documento vazio, fontes None e face não identificável conservam empréstimo. Uma travessia; cache positivo permanece local e válido por append. O wrapper usado pela preparação SVG/PNG move a tabela já própria para o mesmo binder sem copiar seus buffers. Arrays originais/preparados continuam separados, conforme o L0 vigente.

Eliminar a cópia é demonstrado pelo teste causal; não torna toda a operação livre de alocações. Chamadores que exigem `into_owned` ainda precisam possuir uma tabela, e instanciamento/subsetting continuam alocando. Não atribuir a diferença geral de RSS contra vanilla a esta única cópia.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Estado prévio completo, status e `git diff HEAD --stat`: `font-binding-memoria1-2026-10-08/before.json.gz`. Estado materializado e mesmas evidências: `candidate.json.gz`. A árvore já tinha extensa alteração herdada; somente `03_infra/src/export/font_binding.rs` e `00_nucleo/prompts/infra/export/font_binding.md` mudaram entre esses estados produtivos/L0/config. Relatório, scripts e resultados são diagnósticos adicionais.

Antes: binário congelado `/tmp/typst-before-font-binding-memory1`, SHA-256 `398b464d2a1062120928dfaf58351c43acd4a6769af74ac8ee1fc14f31533a14` (build anterior documentado em snapshot-scopes-prioridade7-execucao-2026-10-08.md). Depois: `target/release/typst`, SHA-256 `0600523559f45dca81cf8b8c5f41ab01ab878fd412dd7053282ed767fdd91867`. Cargo offline, builds dev e release. Resselo apenas do par owner/consumer, com validação recursiva dos pins; `seals.json` registra hashes completos. Linter final confirmou ownership/grafo e linhagem sem erros. Nenhum commit foi feito.

## Testes e validação

O teste `complete_prefix_never_copies_font_buffers` foi escrito antes da implementação; sua testemunha privada de cópia retornou 1 em vez de 0, embora empréstimo e conteúdo finais já estivessem corretos (`red.log`). RED no mesmo HEAD/árvore herdada, alterando somente o par acima e diagnósticos; `git diff HEAD --stat` herdado permanece idêntico porque o par era untracked. O código inicial conservava o clone eager através da fábrica privada. GREEN: 6 testes de binding, incluindo três novos casos (zero cópias, cópia única/ordem e vazios/None/metadados inválidos), identidade de faces/eixos e render SVG/PNG (`green.log`). Infra completa: 1250 passed, 2 ignored (`infra-full.log`, RUST_MIN_STACK=67108864).

`cargo build --offline` e `cargo build --offline --release` passaram. `crystalline-lint .`: 0 errors, 535 warnings, 1640 infos, mesmo passivo já registrado; não significa zero violations globais. Logs na pasta desta execução.

Sete fixtures da bateria anterior: antes/depois iguais em texto extraído com layout, sequência de palavras, dimensões das páginas, bounding boxes (delta 0), estilos de fontes e raster a 36 dpi (`qualification.json.gz`). Isso verifica a conservação local; não é certificado global de paridade com vanilla.

## Memória e tempo

Bateria inicial: 2026-10-08 21:55:19.633645–21:55:54.916720 UTC (18:55 em São Paulo), 252 amostras medidas. Sete fixtures, 3 blocos de 6 pares, ordem alternada, um aquecimento por motor/caso/bloco. Processos CLI frescos, filesystem aquecido, afinidade CPU 4, Rayon 1, epoch 0, fontes/locale idênticos. `/usr/bin/time %M` mede pico RSS do processo; wall inclui wrapper comum. Nada de build/test/linter/profiler concorrente. Código/L0/fonts/config/fixtures/binaries com fingerprints iguais no início/fim (`bench.json.gz`, `measure.py`, `protocol.md`).

| Caso | RSS antes → depois (KiB, medianas) | Variação pareada RSS | Variação pareada tempo |
|---|---:|---:|---:|
| Texto simples | 42938 → 42730 | −0,46% | −0,10% |
| Texto denso | 45772 → 45414 | −0,83% | −0,03% |
| Context | 44526 → 44570 | +0,01% | −0,16% |
| Tabela alias | 45694 → 45592 | −0,09% | +0,57% |
| Tabela local | 46528 → 46568 | −0,13% | +0,52% |
| Imagem | 19396 → 19282 | −0,55% | −1,65% |
| Shapes | 20462 → 20556 | +0,69% | +1,23% |

Mediana pareada de percentuais não é razão das medianas; detalhes por bloco em `summary.json`. Texto simples: RSS −0,69/−0,27/−0,43%; denso: −0,79/−0,77/−0,84%. Outros casos não sustentam queda uniforme. Casos gráficos curtos são sensíveis a ruído.

A lentidão pequena das tabelas motivou repetição independente, mantendo a bateria original (`repeat-tables.json.gz`, 108 amostras, mesmos parâmetros). Tempo: alias −0,30%, local −0,86%, denso +0,01%; logo a regressão das tabelas não foi reproduzida entre baterias. RSS: alias −0,26%, local +0,35%, denso −0,26% (denso −0,14/−0,42/−0,10% nos três blocos). Medianas de RSS repetido: denso 45414 → 45316 KiB. Horas exatas e todas as amostras estão no artefato e completion.json. A redução de RSS denso se repetiu, com magnitude variável; texto simples não foi repetido.

Decisão: conservar a remoção da duplicação comprovada, com resultado de produto preservado e ganho pequeno de RSS em texto. Sem ganho geral de velocidade certificado, sem redução relevante da lacuna geral de memória frente ao vanilla. Próximo alvo permanece compartilhamento de bytes de fontes embutidas, sujeito a sua própria medição/L0.

`completion.json` valida que fontes de código/config/L0 e binários permaneceram iguais aos estados congelados após a medição. Scripts e artefatos nesta pasta permitem reproduzir a comparação.
