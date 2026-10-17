# Perfil de CPU atual: catálogo e custo residual dos estilos — 2026-10-10

## Estado e método

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, binário `90ab169f7d583f948613f6b1f6694d9b144f7402f44406b91fdd4687981fb335`. A árvore inclui reserva exata de famílias, caches de resolução primária e transferência de buffers PDF aceitos anteriormente. before.json.gz contém lista exata de arquivos/diffstat, inventário produtivo e hashes das fontes. Não foram alterados consumers durante esta coleta.

12 processos por cada um de quatro documentos1600; ITIMER_PROF1ms e backtrace em handler, Rayon1,CPU fixa, stack64MiB. A sonda normaliza somente endereços do executável (DSOs são x), máximo48frames/2048amostras por processo. É instrumentação diagnóstica que perturba tempo e pode perder funções inlined/frames truncados. Não usar sua duração como benchmark; presença na stack não é CPU exclusivo.

UTC `2026-10-10T04:49:13.093679+00:00` → `2026-10-10T04:49:26.519927+00:00`. Receipts cpu-style-current.json.gz, cpu-classification.json, profile_cpu.py, analyze_cpu.py e cpu_probe.c guardam comandos, fonte, hashes, ambiente, fixture text e todas as amostras.

## Medição antes da escolha

Classificação em ordem: primeiro catálogo por presença de fresh_catalog, fontdb.selective, extração coverage ou load_font; depois eval sem esses frames; depois layout, shaper, export e resto. As categorias particionam as amostras; não demonstram tempo exclusivo em helpers.

| Caso | Amostras | Catálogo | Eval residual | Layout |
|---|---:|---:|---:|---:|
| block-no-set-1600 | 1873 | 1262 (67.4%) | 229 (12.2%) | 156 (8.3%) |
| no-fill-1600 | 1704 | 1241 (72.8%) | 123 (7.2%) | 138 (8.1%) |
| local-set-1600 | 1965 | 1245 (63.4%) | 215 (10.9%) | 225 (11.5%) |
| inline-set-1600 | 2378 | 1278 (53.7%) | 515 (21.7%) | 255 (10.7%) |

O catálogo é chamado dentro de eval. Portanto, a alta presença inclusiva de compiler::eval não identifica um gargalo de interpretação da linguagem. A descoberta, os leitores seletivos e a construção de cobertura compõem o principal custo fixo observado em processos novos. Não extrapolar esses percentuais a recompilação/watch, onde há reaproveitamento de catálogo.

Inline-set mantém custo residual relevante: collapse aparece em99amostras inclusivas e local_delta_since em7; estes grupos se sobrepõem e não devem ser somados. Entre primeiros frames do projeto atribuídos a eval residual, collapse aparece66vezes; realization/table-cell também participa. Isso orienta uma investigação futura, não legitima repetir o experimento de prefixo já descartado.

## Fases: limitação da coleta adicional

phases-current.json.gz contém5execuções por fixture, agrupadas por tamanho/caso. Local1600 mede medianas eval111,67/layout19,79ms e inline1600eval146,32/layout25,91ms, mas os lotes100/400 incluem flutuações grandes (~99–136ms de eval). Os valores de --timings-json são instrumentados; eval inclui descoberta de fontes. Não usar diferenças entre lotes agrupados como custo exclusivo de estilos nem para fechar uma aceleração. O perfil de stacks, não essa comparação agrupada, sustenta a prioridade escolhida.

## Fonte e decisão

`03_infra/src/fontdb/selective.rs:78–93` antes da mudança aloca/zera um vetor em cada Reader::get. `:178–187` lê payload, reserva out e copia esse vetor para out. A fonte comprova a cópia e alocação adicionais. O perfil não mede quanto dessa fatia é cópia, syscall, parser ou normalização. Metadata/fontes/cobertura/fallback observáveis pertencem à linguagem; alocação/cópia é mecânica. Não há evidência de intenção upstream por trás desta escolha.

Decisão interna: testar leitura dos payloads diretamente na cauda projetada, sem novo caminho de aquisição, phase, API, cache, coalescência de leituras ou perda de limites. L0 da unidade foi lido e atualizado primeiro. A hipótese é refutada por divergência de projeção/metadata/cobertura/linguagem ou ausência de ganho repetível na CLI/RSS.

Experimento, testes RED→GREEN, corpus de faces e benchmark: `00_nucleo/diagnosticos/selective-font-buffer-reuse-2026-10-10.md`.
