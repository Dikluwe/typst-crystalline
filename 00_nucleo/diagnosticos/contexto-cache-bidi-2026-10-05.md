# Retenção de métricas no bidi contextual selecionado — 2026-10-05

O coordenador agora empresta ao bidi o cache do candidato contextual aceito, já usado na reconciliação. A mudança produtiva amplia uma seleção de handle; não muda algoritmo, fases, assinatura, budget ou número de layouts. O cache continua local à compilação e ao mesmo World. Nenhum `unsafe` foi acrescentado.

## Proveniência e escopo

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. A lista exata de alterações (`git diff HEAD --stat`), status incluindo untracked, UTC e fontes integrais anteriores estão em [before-state.json.gz](contexto-cache-bidi-2026-10-05/before-state.json.gz); fontes finais, logs, hashes e recibos estão em [execution.json.gz](contexto-cache-bidi-2026-10-05/execution.json.gz). As mudanças deste ensaio são somente `03_infra/src/pipeline/{paged,tests}.rs` e seus dois L0 proprietários: [review.diff](contexto-cache-bidi-2026-10-05/review.diff) e [scope-review.json](contexto-cache-bidi-2026-10-05/scope-review.json). O Núcleo compilation-metric-cache e o carrier Prepared não mudaram. A restrição ao bidi fresco era explícita nos ensaios anteriores para isolar contribuições; não foi comprovada como herança por inércia.

Baseline SHA-256 `c35dc34f0f24466b0b7339fcb856d5fc59c6f8701308f54963a4a1704f7746b8`, cópia preservada em `/tmp/tekt-context-bidi/baseline-typst`. Candidato SHA-256 `93e3678323f163c81567ed00b959381bd0e2dd416689421c6500d3671abf20d9`, receita `cargo build --release`, árvore final arquivada. Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, `lab/typst-original/target/release/typst`; receita e árvore ratificadas no diagnóstico P1489. Não foi usado `/usr/local/bin/typst`.

## Medir antes de decidir

`paged.rs:565–568` criava métricas frescas para o candidato selecionado embora o handle já estivesse retido. `layout_bidi.rs:23–29,128–173` recebe FontMetrics e consulta larguras de palavras. O L0 de bidi já autoriza a injeção; suas equações não mudam. A sonda inicial confirmou misses evitáveis antes de editar os dois L0. ADR-0127: correção interna cujo desfazer não altera contratos externos, fluxo contínuo L0 primeiro, RED→GREEN e resselo restrito.

A sonda GDB [count.py](contexto-cache-bidi-2026-10-05/count.py) conta consultas a cached_shaped_width, cálculos no miss de advance_shaped, entradas no wrapper de layout e criação de métricas. A janela bidi começa em reorder_bidi_document e termina em shape_document; reconciliação tem marcador próprio. Não instrumenta produção. [count-initial.json.gz](contexto-cache-bidi-2026-10-05/count-initial.json.gz) preserva a contagem antes das alterações; [count-candidate.json.gz](contexto-cache-bidi-2026-10-05/count-candidate.json.gz) registra depois. Fontes completas e comandos dos casos estão nos recibos.

| Fixture | Cálculos bidi antes→depois | Cálculos reconciliação antes→depois | Cálculos totais antes→depois | Layouts antes→depois |
|---|---:|---:|---:|---:|
| RTL selecionado, palavras com números distintos | 299→137 | 161→24 | 622→323 | 2→2 |
| RTL selecionado, texto repetido | 2→0 | 0→0 | 4→2 | 2→2 |
| RTL ordinário, mesmo texto distinto | 137→137 | 24→24 | 323→323 | 2→2 |
| LTR selecionado, tabela distinta | 0→0 | 0→0 | 322→322 | 2→2 |

Consultas bidi do caso distinto permanecem 2057; reconciliação, 640; totais, 4617. Métricas criadas no selecionado passam de 9 para 8; no ordinário permanecem 10. Não se eliminou travessia nem passagem do documento. Bidi compartilha suas novas entradas com a reconciliação seguinte, por isso o ganho pertence ao empréstimo comum aos dois consumidores. Os 137 misses restantes de bidi e 24 de reconciliação mostram o limite: não se presume que toda largura possa existir no layout.

## Tempo pareado, mesma CLI

[bench.py](contexto-cache-bidi-2026-10-05/bench.py), uma amostra de aquecimento por binário e fixture seguida de nove pares alternados por rodada, CPU com afinidade fixa, `RAYON_NUM_THREADS=1`, `SOURCE_DATE_EPOCH=0`. Sem builds, testes ou GDB concorrentes. Inclui fontes, compilação e PDF; a métrica shape_ms agrega bidi, shaping e reconciliação. Não comparar esses tempos com vanilla como se o caso longo já tivesse paridade.

Rodada 1, UTC 22:03:00–22:03:17, [recibo](contexto-cache-bidi-2026-10-05/bench-round1.json.gz); rodada 2, UTC 22:03:58–22:04:15, [recibo](contexto-cache-bidi-2026-10-05/bench-paired.json.gz), ambas em 2026-10-05.

| Fixture | CLI mediana rodada 1, ms | CLI mediana rodada 2, ms |
|---|---:|---:|
| RTL selecionado distinto | 203,75→192,73 | 203,01→191,16 |
| RTL selecionado repetido | 168,13→167,50 | 167,10→166,92 |
| RTL ordinário | 191,30→191,78 | 191,72→192,35 |
| LTR selecionado | 197,39→197,61 | 197,70→200,98 |

O distinto melhora cerca de 5,4–5,8%; candidato mais rápido nos 18 pares, medianas dos deltas pareados −10,77 e −11,41 ms. shape_ms passa de 32,88→21,85 e 32,28→21,40 ms. Nos controles, resultados pequenos/variáveis não demonstram ganho: para LTR, deltas pareados medianos −0,63 e +0,22 ms (diferentes da diferença de medianas, sensível aos outliers). Não afirmar regressão ausente em qualquer entrada nem ganho universal. A decisão de manter combina contagem causal, melhoria repetida no caso que expõe misses e saída preservada.

## Validação e limite de paridade descoberto

Teste `selected_context_retains_metrics_until_bidi`: RED reproduziu criação de uma instância fresca (esperado zero), preservado em [red-state.json.gz](contexto-cache-bidi-2026-10-05/red-state.json.gz). GREEN na suíte `SOURCE_DATE_EPOCH=0 cargo test -p typst-infra --lib`: **1157 testes passaram**. L1 não mudou e sua suíte não foi reexecutada nesta etapa. `cargo build` e `cargo build --release` passaram. V15/V26 passaram antes do resselo; linter completo teve zero erros, 534 warnings e 1574 notes herdados, **nenhum finding novo**, comparado ao recibo anterior, não só por contagem. Sem V5/V6/V15/V26 nos dois owners tocados. Não declarar zero violations global; pendência de Coverage::from_ranges permanece intacta.

[verify.py](contexto-cache-bidi-2026-10-05/verify.py) e [verification.json.gz](contexto-cache-bidi-2026-10-05/verification.json.gz): seis casos positivos preservaram texto extraído, bbox XML, raster PNG a 36 dpi e diagnósticos entre baseline e candidato, além de dois controles de erro idênticos. Incluem callback matemático e RTL simples. Quatro casos positivos coincidiram em texto normalizado com vanilla; dois **não**: RTL distinto longo, contextual e ordinário. O primeiro assert de paridade falhou; log preservado no recibo. Não remover essa evidência para declarar paridade: o script final exige preservação baseline/candidato em todos os casos e registra explicitamente a divergência conhecida nesses casos longos.

Na extração do caso longo, o baseline e candidato apresentam `10` antes de `8` e `9`, enquanto o vanilla mantém a sequência inicial desses números. O defeito/divergência antecede o patch e também aparece no caminho ordinário. Isso ainda não isola se a origem é layout, reordenação de itens, reconciliação ou extração do PDF; igualdade de raster baseline/candidato prova ausência de mudança causada por este empréstimo nos casos medidos, não prova semântica universal ou geometria vanilla. A próxima auditoria deve reduzir esse caso e distinguir ordem de leitura PDF de posição visual antes de corrigir fórmulas ou L0 de bidi/reconciliação. Fixtures integrais e extrações dos três binários estão em verification.json.gz.

## Decisão

Manter o empréstimo: trabalho evitado e benefício repetido no RTL contextual distinto, com preservação das saídas medidas. A cadeia de métricas existente agora atravessa tentativas de layout, bidi e reconciliação dentro da mesma compilação. Não constitui memoização de frames, nem retenção de watch, nem fechamento universal da Frente C/F. A divergência RTL descoberta abre uma investigação de correção antes de novas otimizações nessas equações.
