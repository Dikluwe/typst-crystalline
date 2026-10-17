# Memória — prioridade 3: medição por fronteiras de fase

2026-10-08. Autorização do dono: “faça o 3”. Diagnóstico de leitura; nenhum código produtivo, Prompt L0, configuração ou binário foi alterado. Não há nova implementação a ressellar nem motivo para repetir build/testes após o fechamento da prioridade 2.

## Proveniência e método

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado e extenso passivo herdado. `memoria-fases3-2026-10-08/state.json.gz` contém UTC, textos completos de código/L0/config, status com untracked e `git diff HEAD --stat`. Binário atual pós-prioridade 2: SHA-256 `4686d15bd91c1b9cd9ca4605fdec16b8c7babd59f4e125276ebcaba87969379c`; receita e validações em embedded-fonts-memoria2-execucao-2026-10-08.md.

GDB no binário release existente, sem recompilar ou inserir hooks. Breakpoints de entrada e retorno nas funções com símbolo disponível: construção World, catálogo embutido e sistema, eval, preparação contextual, bidi, shaping, pipeline paginado e export single/multi. `resolve_fonts` inlined não gerou amostras; o intervalo pipeline.exit → export.entry inclui resolução de recursos, sem isolá-la. A janela prepare.exit → bidi.entry inclui layout, introspecção, outlines, callbacks e numeração; não mede uma única função de layout nem suas alocações exclusivas.

Em cada fronteira, ler externamente `/proc/PID/status` e smaps; registrar RSS atual por mappings, VmRSS e VmHWM. Classes: heap explícito, mappings anônimos, arquivos de fontes, executável, outros arquivos e stack principal. Mappings anônimos também podem incluir arenas/threads; RSS de arquivo não é consumo de heap. Não chamar funções do inferior nem alterar seus dados.

Sandbox impediu ptrace no piloto (`pilot.log`, 0 amostras válidas). A revisão automática autorizou execução externa de leitura com ptrace. Heaptrack e valgrind não estavam disponíveis; não alegar perfil por alocação ou inventário de objetos vivos.

Duas repetições para 15 documentos: sete fixtures anteriores + closures1000, uma closure/1000 chamadas, context10/100/300 e plain10/100/300. Cada perfil foi seguido por execução normal do mesmo binário sob `/usr/bin/time %M`, sem depurador, com pico RSS e bbox/XML do PDF. Os 30 pares conservaram texto e geometria (bbox XML idêntico); hashes PDF originais/diretos também registrados, sem depender deles como gate de linguagem. Sem promessa de igualdade raster desta rodada.

CPU 4, Rayon 1, epoch 0, locale/fontes comuns; argv, fontes/config/fixtures/código/L0 com SHA-256 em profiles.json.gz e profiles-growth.json.gz. Sem builds/testes/linter/profiler concorrentes. Fingerprints e binário permaneceram iguais após ambas as execuções. Não usar tempos do GDB para decisões de velocidade.

Bateria principal UTC 22:42:23.974903–22:42:46.450923 de 2026-10-08 (19:42 em São Paulo). Horas exatas do complemento constam em profiles-growth.json.gz. Scripts profile.gdb/run.py/growth.py e dados completos estão na pasta; summary.json combina as duas baterias. Números abaixo são medianas de duas repetições, não estimativa estatística de precisão universal.

## Onde cresce a memória

| Fronteira | Texto simples (MiB RSS) | 300 contexts (MiB RSS) |
|---|---:|---:|
| Após avaliação | 14,10 | 61,92 |
| Antes da preparação contextual | 14,38 | 62,50 |
| Após preparação contextual | 14,82 | 138,66 |
| Antes de bidi, após janela de layout | 40,76 | 168,80 |
| Após shaping | 40,82 | 168,90 |
| Retorno do pipeline paginado | 40,89 | 141,28 |
| Após export PDF | 41,86 | 141,89 |
| Pico de execução normal | 41,67 | 196,34 |

No context300, VmHWM na entrada de bidi já é ~196,36 MiB, acima dos ~168,80 MiB RSS correntes: o pico transitório ocorreu entre prepare.exit e bidi.entry, antes de exportar. A preparação eleva RSS em ~76,16 MiB, e a janela seguinte contém outro pico. Heap mapeado: ~24,9 MiB antes de prepare, ~100,7 MiB após prepare e ~128,6 MiB na entrada de bidi. Não confundir bytes residentes da arena com objetos Rust ainda vivos.

O retorno do pipeline reduz RSS de ~168,9 para ~141,3 MiB, demonstrando liberação/decommit de parte da memória entre essas fronteiras, sem identificar quais objetos foram responsáveis. Export single sobe menos de 1 MiB nas amostras. Isso exclui a exportação como localização do grande pico nesse documento, não exclui seus custos em todos os documentos.

## Escala contextual versus controle

| Blocos | Context: pico normal (MiB) | Plain: pico normal (MiB) |
|---|---:|---:|
| 10 | 43,23 | 43,01 |
| 100 | 59,97 | 44,13 |
| 300 | 196,34 | 46,07 |

Nos 100 contexts: eval.exit ~45,22 MiB, prepare.exit ~52,59 MiB e bidi.entry ~60,16 MiB. Nos 300: ~61,92/~138,66/~168,80 MiB. Crescimento forte e não linear nestes três pontos; não declarar complexidade assintótica ou lei quadrática com essa amostragem. Mil closures no outro controle ficam ~43,18 MiB de pico, uma closure chamada mil vezes ~42,95 MiB. Isso torna a expansão contextual alvo mais concreto que uma atribuição genérica a todas as closures.

## Piso das fontes

Catálogo embutido sobe ~19 MiB RSS quando acionado: no context300, ~12,91 → ~31,98 MiB. O componente anônimo sobe ~9,27 MiB e páginas residentes do executável ~9,78 MiB. Inferência sustentada pela fonte: transferência dos assets estáticos para Vecs próprios toca ambos os conjuntos de páginas (`embedded_fonts.rs:92-94` e `fonts.rs:119`). Não atribuir todo mapping anônimo a fontes sem perfil por alocação. O catálogo de sistema acrescenta ~2,5 MiB neste controle.

Descoberta é lazy e aninhada: em texto simples ocorre na janela de layout; em outros exemplos, dentro da avaliação. Não somar esses deltas como se fossem fases sequenciais independentes (`world/fonts.rs:65-79`). As prioridades 1/2 eliminaram cópias adicionais, mas ainda existe uma cópia própria de cada asset embutido, além das páginas estáticas do executável. Essa medição explica por que compartilhar slot/get teve ganho pequeno e não removeu todo o piso do catálogo.

## Próximo alvo e limites

Investigar alocações/snapshots da preparação e janela de layout de context. Candidatos concretos: `context_stabilization.rs:196` clona input para ctx.introspector; `:244` clona input para Execution; `:252-254` conserva execuções anteriores em retired; pipeline/paged.rs:297-337 mantém vistas original/contextual/preparada, e `:384` outra vista realizada. Muitos clones de Content são Arc e baratos; a mera ocorrência de clone não prova custo profundo nem duplicação integral.

O L0 vigente de context_stabilization exige identidade de produtor, capturas, geração causal, observações e replay; não autoriza apagar históricos indiscriminadamente. A próxima proposta deve medir bytes/alocações desses candidatos antes de escolher representação ou reduzir retenção e preservar a prova causal. Nada disso foi implementado nesta prioridade.

O perfil localiza memória residente e o intervalo do pico, mas não distingue buffers ainda vivos de páginas livres retidas pelo alocador. Essa atribuição permanece aberta e exige perfil de heap/lifetimes. Não inferir leak, não misturar dados de GDB com tempos normais, não afirmar comparação nova com vanilla (este estudo mede somente o binário atual).
