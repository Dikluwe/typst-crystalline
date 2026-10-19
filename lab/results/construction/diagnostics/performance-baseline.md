# Performance baseline — Tekt and pinned vanilla

Nos cinco fixtures qualificados, o Tekt apresentou medianas próximas de 300 ms. O vanilla apresentou cerca de 11–13 ms para imagem/formas e 136–143 ms para os casos com texto. A diferença depende do fixture. Os oito alvos existentes foram executados: 139 casos têm três rodadas aceitas. Sessenta casos falharam no critério de repetibilidade média/mediana da ADR-0115; seus números permanecem indicativos. Dados rejeitados não entram nos resultados.

O estado medido foi a working tree não commitada sobre HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, fingerprint `84fe8cd7336862b4b1754b3c6af11d24626cc12b34fa975ac4d03dd028a1aff7`. A lista exata de alterações, inclusive arquivos não rastreados, está em [built.status.txt](performance-baseline/state/built.status.txt); o diff-stat em [built.diff-stat.txt](performance-baseline/state/built.diff-stat.txt). [Snapshot completo](performance-baseline/build-state.json), patches e arquivo de consumers/inputs preservam a identidade dos fontes usados.

O vanilla é o pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, executável SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. A verificação confrontou todos os blobs upstream; a diferença local é o overlay documentado de Cargo.lock. A string de versão é auxiliar. Veja [proveniência](performance-baseline/vanilla-provenance.json) e a [receita ratificada em P1489](typst-p1489-resync-vanilla.md).

A CLI Tekt tem SHA-256 `9a94d37b711428a699e49bcba239de6a065a8c1602ae3362f9b55a6ac5e608e7`. Os oito benchmarks, a CLI e o artefato privado de testes usam opt-level 3, sem debug assertions, conforme [Cargo JSON e hashes](performance-baseline/artifacts.json). Observation e stdlib entram exclusivamente pelos launchers que verificam o artefato release. Nenhum teste regular é executado para obter tempos.

[Auditoria](performance-baseline/benchmark-audit.md) e [protocolo reproduzível](performance-baseline/protocol.md) distinguem operações isoladas, composição e pipeline. Stream mede exportação inteira, com builder, recursos e serialização Pretty; os cohorts de page/text/shape/draw não isolam os emitters. Pipeline é warm-process e exclui criação do world, descoberta inicial de fontes e parsing do Source. A CLI inclui essas etapas, I/O e teardown. Custos de fixtures diferentes não serão somados.

A comparação qualificada usa os mesmos inputs e opções para Hello, texto denso, contexto, JPEG e formas posicionadas. Libertinus/DejaVu têm os mesmos bytes nos assets de ambos. A matemática foi excluída da comparação end-to-end: NewCM difere entre os assets, o seletor math.equation usado para fixar uma fonte não é suportado no Tekt, e set text no documento é sobrescrito pelo estilo da equação. As evidências não cronometradas foram preservadas em qualification; os microbenchmarks matemáticos existentes continuam no escopo. A primeira variante de formas em fluxo também foi excluída por paginação diferente.

A decomposição instrumentada compara eval inicial, estabilização/layout e exportação PDF completa; fontes resolvidas fora do timer de export, setup, parsing, escrita, teardown e overhead de instrumentação permanecem no residual. A soma dos deltas das médias só explica o gap da população instrumentada, não o gap das medianas sem tracing. [Experiências controladas propostas](performance-baseline/controlled-experiments.md) explicitam o que confirmaria ou refutaria cada hipótese. Nenhuma otimização produtiva é realizada.

Resultados sem instrumentação: três rodadas, 36 pares por fixture, processo novo e cache de filesystem aquecido. Intervalos entre parênteses são p05–p95 da distribuição observada, não intervalos de confiança. Razão e diferença são calculadas por par. [Resultados brutos e distribuições](performance-baseline/e2e-summary.json); cada samples.json conserva comandos, horários, hashes e tempos individuais.

| Fixture | Tekt, mediana ms (p05–p95) | Vanilla, mediana ms (p05–p95) | Δ pareado mediano ms | Razão pareada mediana |
|---|---:|---:|---:|---:|
| context | 305.15 (277.83–355.44) | 138.06 (129.61–155.00) | 159.20 | 2.15× |
| image | 294.79 (276.63–322.11) | 11.45 (10.19–13.49) | 283.01 | 26.04× |
| pipeline-single-font | 291.87 (286.79–320.20) | 136.27 (132.37–152.08) | 155.90 | 2.14× |
| shapes | 300.18 (290.22–328.09) | 12.55 (11.96–13.69) | 287.93 | 23.89× |
| text-dense | 306.02 (297.72–342.35) | 143.23 (137.66–161.66) | 162.21 | 2.14× |

[Controles de saída](performance-baseline/output-validation.json) conferem páginas, dimensões, texto normalizado e fontes de texto em cada rodada. Tekt declara recursos Helvetica adicionais nas saídas sem texto; isso não indica uso desses glifos. Estes controles não demonstram paridade global nem igualdade de geometria de todos os elementos.

O universo de fontes desta baseline inclui descoberta padrão de fontes embutidas/do sistema e uma face de projeto não usada, fornecida pelo mesmo --font-path nos dois comandos. A aquisição dessa face ainda pode custar tempo. Seu efeito isolado não foi medido; há um controle A/B proposto para remover apenas essa opção e conferir que faces selecionadas e resultados permaneçam iguais.

Decomposição instrumentada: três rodadas, 18 pares por fixture. A tabela usa médias em ms e deltas assinados Tekt − vanilla; a soma das quatro colunas reproduz o gap médio da própria população instrumentada. Ela não decompõe diretamente o gap das medianas sem tracing. As fronteiras separam etapas amplas, mas aquisições preguiçosas podem mudar de envelope. [Amostras de fases](performance-baseline/phase-samples.json) e [distribuições completas](performance-baseline/phase-summary.json).

| Fixture | Δ eval inicial | Δ estabilização/layout | Δ export PDF | Δ residual | Δ wall instrumentado |
|---|---:|---:|---:|---:|---:|
| context | -125.331 | +0.110 | -0.320 | +279.754 | +154.213 |
| image | +1.392 | -0.066 | -0.456 | +277.166 | +278.036 |
| pipeline-single-font | +1.425 | -123.809 | -0.235 | +280.159 | +157.539 |
| shapes | +2.796 | +1.734 | -0.425 | +275.439 | +279.544 |
| text-dense | -118.568 | +3.516 | +0.933 | +270.013 | +155.894 |

Tempos absolutos das mesmas amostras instrumentadas:

| Fixture / engine | Eval ms | Estabilização/layout ms | PDF inteiro ms | Residual ms | Wall ms |
|---|---:|---:|---:|---:|---:|
| context / tekt | 2.076 | 0.897 | 0.649 | 291.214 | 294.836 |
| context / vanilla | 127.407 | 0.787 | 0.969 | 11.460 | 140.623 |
| image / tekt | 2.351 | 0.335 | 0.147 | 287.471 | 290.304 |
| image / vanilla | 0.959 | 0.400 | 0.603 | 10.305 | 12.268 |
| pipeline-single-font / tekt | 2.210 | 0.548 | 0.623 | 290.675 | 294.056 |
| pipeline-single-font / vanilla | 0.785 | 124.357 | 0.858 | 10.516 | 136.516 |
| shapes / tekt | 4.238 | 2.457 | 0.321 | 285.923 | 292.939 |
| shapes / vanilla | 1.442 | 0.723 | 0.745 | 10.484 | 13.395 |
| text-dense / tekt | 2.156 | 6.635 | 2.605 | 280.456 | 291.852 |
| text-dense / vanilla | 120.724 | 3.119 | 1.672 | 10.443 | 135.958 |


![Recorded phase envelopes and residual](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/performance-baseline/phase-envelopes.png)

[Figura vetorial](performance-baseline/phase-envelopes.svg) e [script](performance-baseline/plot.py) usam somente as médias da população instrumentada.

No vanilla, spans aninhados discover fonts / scan system fonts localizam o primeiro acesso ao inventário. A união temporal é medida por nome; estes tempos já pertencem a eval ou layout e não são adicionados novamente.

| Fixture | discover fonts, média ms | scan system fonts, média ms | Envelope que contém o primeiro scan |
|---|---:|---:|---|
| context | 126.028 | 124.850 | eval |
| image | 0.000 | 0.000 | não observado |
| pipeline-single-font | 123.358 | 122.238 | layout |
| shapes | 0.000 | 0.000 | não observado |
| text-dense | 119.442 | 118.326 | eval |

O residual Tekt contém 280–291 ms nas médias observadas. Ele inclui construção do world, I/O/parsing fora dos contadores, recursos resolvidos antes do timer PDF, escrita e teardown. O código posiciona descoberta de fontes antes do pipeline; a identificação desse trabalho como explicação dominante do residual é uma inferência, a confirmar por controle do universo de fontes ou timer diagnóstico. O trace vanilla demonstra onde seu scan foi cronometrado; não mede o scan Tekt. As médias de exportação PDF completa ficam abaixo de 3 ms nestes inputs e não sustentam atribuir o gap inteiro ao stream.

A mudança de fronteira é visível: Hello inicia o scan no layout vanilla; o set text(font:) de contexto/texto denso antecipa o scan para eval. Deltas negativos de eval/layout não demonstram algoritmos Tekt mais rápidos, pois o Tekt já preparou fontes antes desses timers. Não há comparação isolada de shaper entre os dois engines.

Instrumentação, diferença das médias de wall entre populações instrumentada e sem tracing:

| Fixture | Tekt ms | Vanilla ms |
|---|---:|---:|
| context | -11.432 | +0.222 |
| image | -4.091 | +0.731 |
| pipeline-single-font | -2.145 | -1.566 |
| shapes | -11.871 | +0.490 |
| text-dense | -19.628 | -9.820 |

Diferenças negativas indicam variação temporal entre populações, não benefício causal do tracing. A instrumentação não foi alternada A/B dentro do mesmo bloco; não se subtrai uma correção fixa das fases. A variação limita deltas pequenos, em particular sub-ms. As distribuições, médias de cada rodada, CPU/RSS e amostras permanecem disponíveis.


As amostras admitidas abrangem 2026-09-27T03:02:14.592750+00:00 a 2026-09-27T04:13:50.344989+00:00 (UTC). Máquina Dell-G15-Linuxmint, Ryzen 7 5800H, Linux 6.17.0-42, rustc/Cargo 1.92.0, LLVM 21.1.3, Python 3.12.3. Afinidade CPU 4, sibling SMT 5; governor powersave e boost observados. Flags de compilação vazias, um job Cargo, Rayon 1, SOURCE_DATE_EPOCH=0, TZ=UTC, LC_ALL=C.UTF-8. [Máquina e toolchain](performance-baseline/host.json), [estado de frequência](performance-baseline/machine-control-state.json) e recibos conservam o ambiente detalhado.

Microbenchmarks: estimador slope do Criterion quando disponível, mean nos casos flat; mediana das três estimativas por caso. [Tabela completa](performance-baseline/micro-results.md) e [JSON com CIs, amostras normalizadas e dispersão](performance-baseline/micro-summary.json) conservam todos os casos. O scanner também registra ns/byte.

| Alvo | Casos / 3 rodadas | Média–mediana <3% | Range/mediana <3% |
|---|---:|---:|---:|
| content_bench | 35 | 14 | 0 |
| layout_bench | 12 | 10 | 1 |
| observation_bench | 17 | 11 | 0 |
| pdf_builder_bench | 12 | 5 | 2 |
| pipeline_bench | 17 | 16 | 3 |
| scanner_bench | 5 | 5 | 1 |
| stdlib_bench | 23 | 17 | 1 |
| stream_bench | 18 | 1 | 0 |

A ADR-0115 usa diferença entre média e mediana de três runs. Ela foi satisfeita por 79 casos e falhou em 60. O teste mais estrito de range/mediana passou em apenas 8. Portanto, não há demonstração de uma baseline de precisão uniforme de 3%; especialmente stream, Content e janelas privadas curtas requerem confirmação antes de decisões finas de regressão. Foram preservados todos os outliers e todas as rodadas, sem seleção das mais rápidas.

| Scanner | ns/byte, mediana das rodadas | Range/mediana |
|---|---:|---:|
| scanner/b1_hello | 7.091 | 1.32% |
| scanner/b2_text | 3.204 | 11.00% |
| scanner/b3_math | 13.078 | 16.30% |
| scanner/b4_code | 11.694 | 5.73% |
| scanner/b5_utf8 | 9.482 | 4.08% |

O warm-process pipeline/pdf/single_font, sobre o literal Hello correspondente, teve mediana de 2.374 ms/op (range/mediana 6.96%). A CLI Hello mediana de 291.87 ms usa processo novo e universo de fontes diferente; setup, parsing e caches têm fronteiras distintas. A diferença entre esses números não é um timer isolado de world ou fonte. Demais microbenchmarks têm fixtures diferentes e não são somados a um total de documento.

As fronteiras auditadas podem ser conferidas em benches/stream_bench.rs:125, benches/pipeline_bench.rs:67, benches/run_core_benchmark.py:29, 03_infra/src/pipeline/paged.rs:201, 03_infra/src/pipeline/pdf.rs:432 e :473, 04_wiring/src/compile.rs:246 e lab/typst-original/crates/typst-kit/src/fonts.rs:149. Os bytes desses consumers estão fingerprintados e arquivados; os paths são referências à árvore medida, não ao upstream móvel.

Persistem limites: desktop/OS ativos, clock/boost não fixados e histórico térmico não controlado; monitoramento a 100/250 ms pode perder processos muito curtos; fingerprints antes/depois não detectam edição e restauração entre snapshots. Cache de disco está aquecido, microbenchmarks mantêm caches em processo e CLI tem processo novo. RSS de wait4 pode herdar um piso do Python antes de exec e permanece dado auxiliar. Não há comparação isolada de parser, métricas reais, shaping, stream, serialização ou startup. Fontes externas são identificadas por hash, sem empacotar todo o inventário instalado; reprodução em outra máquina exige esse mesmo ambiente.

A [auditoria final](performance-baseline/completion-audit.json) confirmou cobertura, três estimativas por caso, pares completos, conservação aritmética das fases, ausência de concorrência observada nos blocos aceitos, fingerprints base iguais e hashes dos executáveis inalterados. Drivers adicionais têm hashes antes/depois e revisões arquivadas. [Índice dos dados](performance-baseline/README.md) identifica recibos, snapshots, resultados brutos, rejeições e protocolo.

Os gargalos candidatos são aquisição/coverage de fontes no world, primeiro uso preguiçoso no vanilla, montagem da stdlib, cache de shaping, estabilização de contexto, alocações/Content e exportação. Os controles com critérios de confirmação/refutação estão em [controlled-experiments.md](performance-baseline/controlled-experiments.md). Nenhuma intervenção de otimização foi realizada; a responsabilidade quantitativa do scan pelo residual Tekt e o efeito da face extra de projeto permanecem não demonstrados.

Nesta tarefa, alterações limitadas ao relatório e ao diretório diagnóstico atribuído. Código produtivo, L0s, ADRs e benchmarks existentes foram preservados. Não houve formatação global nem resselo de hashes.
