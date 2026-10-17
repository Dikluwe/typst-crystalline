# Evidência da escala de tabelas com estilo local

O relato e a decisão estão em [`../table-local-scaling-followup-2026-10-09.md`](../table-local-scaling-followup-2026-10-09.md). Esta pasta guarda fontes de sondas, fixtures, scripts e recibos comprimidos. Os nomes dos arquivos são parte da proveniência registrada nos recibos; não mover nem reescrever uma coleta histórica para organizar a pasta.

## Estado de referência

| Finalidade | Arquivos |
|---|---|
| Comparação bilateral após reutilizar o primeiro layout paginado da Cell local | `measurements-local-reuse.json.gz`, `measure_local_reuse.py` |
| Prova de ausência de Mathy na subárvore, benchmark e equivalência | `measurements-bare-math.json.gz`, `measure_bare_math.py`, `qualification-bare-math.json.gz`, `validation-bare-math.json.gz`, `seal-bare-math.json` |
| Gate de alocações da prova de Mathy (limite predefinido **falhou**) | `bare-math-{red,green}-gate.json.gz`, `bare_math_gate.py` |
| Fases e pilhas antes da prova de Mathy | `phases-after-reuse.json.gz`, `stacks-after-reuse.json.gz`, `phases_after_reuse.py`, `stack_profile_after_reuse.py` |
| Atribuição do resíduo após a prova de Mathy | `phases-after-bare-math.json.gz`, `stacks-after-bare-math.json.gz`, `layout-call-counts-after-bare-math.json.gz`, `residual-attribution-after-bare-math.json.gz` e scripts de mesmo tema |
| Contagem exata dos escopos e seleções de fonte após o reuso | `style-call-counts-after-bare-math.json.gz`, `count_style_calls_after_bare_math.py` |
| Reuso dos defaults da Cell, contagem/tempo/saída antes e depois | `default-call-counts-{before,after}.json.gz`, `alloc-cell-defaults-{before,after}.json.gz`, `measurements-cell-defaults.json.gz`, `qualification-cell-defaults.json.gz` e scripts correspondentes |
| Oito chaves estáticas da Cell, gate/tempo/saída antes e depois | `static-cell-keys-{red,green}-gate.json.gz`, `measurements-static-cell-keys.json.gz`, `qualification-static-cell-keys.json.gz`, `stacks-after-cell-defaults.json.gz`, `seal-static-cell-keys.json` e scripts correspondentes |
| Contagem exata de subframes antes/depois | `layout-call-counts-before.json.gz`, `layout-call-counts.json.gz`, `count_layout_calls.py` |
| Equivalência visual e textual com o binário anterior | `qualification-local-reuse.json.gz`, `validation-local-reuse.json.gz`, `qualify_local_reuse.py` |
| Comparação bilateral do binário aceito após o cache de célula | `measurements-local-cache.json.gz`, `measure_local_cache.py` |
| Gate RED→GREEN do cache | `local-cache-red-gate.json.gz`, `local-cache-green-gate.json.gz`, `local_cache_gate.py` |
| Texto, páginas, caixas e raster das fixtures | `qualification-local-cache.json.gz`, `validation-local-cache.json.gz`, `qualify_local_cache.py` |
| Fases e pilhas posteriores ao cache | `phases-current.json.gz`, `stacks-after-cache.json.gz` |
| Entrada e implementação das sondas de alocação | `fixtures/*.typ`, `alloc_probe.c`, `stack_probe.c`, `stack_probe_deep.c` |
| Amostras de CPU do release aceito e driver reproduzível | `cpu-style-current.json.gz`, `cpu_probe.c`, `profile_cpu.py` |
| Saída antecipada sem destinos de referência: gate, A/B, paridade e linhagem | `page-ref-empty-{red,green}-gate.json.gz`, `measurements-page-ref-empty.json.gz`, `qualification-page-ref-empty.json.gz`, `seal-page-ref-empty.json` e scripts correspondentes |
| Ordem contextual vazia sem reconstrução: gate, A/B, paridade e linhagem | `ordered-blocks-empty-{red,green}-gate.json.gz`, `measurements-ordered-blocks-empty.json.gz`, `qualification-ordered-blocks-empty.json.gz`, `seal-ordered-blocks-empty.json` e scripts correspondentes |

O release candidato após trocar os oito nomes de default por chaves estáticas tem SHA-256 `14d023d36fb765eb041f24f440eaad3bc5d67183325b611e4fd98502bd26815e`; o anterior, após o reuso de defaults da Cell, era `3d52e418a7343ce4a202735ffe72fba1de65ce9d6fc5b310aaac1482547d1241`. O gate desta rodada caiu 579.935→541.535 pedidos locais; as razões pareadas e a equivalência estão no relato. Cada recibo contém o HEAD, estado Git não commitado, diffstat, horário UTC, fixture, comando, CPU e hash do binário usado. O binário vanilla segue o pin descrito em `CLAUDE.md` na raiz. Conferir esses campos antes de comparar uma execução nova com números históricos.

## Experimentos posteriores retirados

| Hipótese | Recibos | Resultado |
|---|---|---|
| Buffer curto de seleção de fonte | `font-buffer-{red,green}-gate.json.gz`, `measurements-font-buffer.json.gz` | Menos alocações; excesso local sem redução |
| Visita de leitura em `tracks` | `grid-scan-{red,green}-gate.json.gz`, `measurements-grid-scan.json.gz` | Menos alocações; tempo pior |
| Transporte em `style_scope` | `style-transport-{red,green}-gate.json.gz`, `measurements-style-transport.json.gz` | Menos alocações; tempo pior |
| Transporte na medição restrita | `measurement-transport-{red,green}-gate.json.gz`, `measurements-measurement-transport.json.gz` | Menos alocações; tempo pior |
| Certificado de `table_cells::needs_preparation` | `needs-preparation-{red,green}-gate.json.gz`, `stacks-needs-preparation-before.json.gz` | Alocações idênticas; gate falhou e código/L0 voltaram aos hashes anteriores |
| Cache da prova paginada da receita local | `page-recipe-key-counts-before.json.gz`, `page-recipe-cache-{red,green}-gate.json.gz`, `page-recipe-single-green-gate.json.gz`, `measurements-page-recipe-{cache,single}.json.gz` | Menos alocações; mapa penalizou controle e entrada única penalizou tempo local; ambas retiradas |
| Candidato de selector da Cell construído sob demanda | `walk-cell-stage-before.json.gz`, `walk-cell-callsite-before.json.gz`, `lazy-cell-candidate-{red,green}-gate.json.gz`, `measurements-lazy-cell-candidate.json.gz` | Zero Cell Complete no corpus; duas entradas por item local em callsites distintos; menos alocações, gate falhou e tempo piorou; código/L0 retirados |
| Evitar `with_defaults` de Cell já preparada | `default-call-counts-prepared-{red,green}.json.gz`, `prepared-cell-skip-green-gate.json.gz`, `measurements-prepared-cell-skip.json.gz` | 1.600 chamadas a menos em cada caso de 1.600 células; pedidos glibc idênticos e tempo sem ganho; código/L0 retirados |

Os scripts correspondentes permanecem como documentação da receita executada. Seus gates só testam a hipótese experimental e **não** autorizam reinserir o código retirado. Há coletas anteriores nesta mesma pasta; o relato principal conserva sua cronologia e seus critérios.

## Conferência e novas coletas

`python3 audit_fixtures.py` verifica a leitura dos recibos `*.json.gz` e compara todos os hashes de fixtures registrados com `fixtures/`. A auditoria é somente leitura. Para repetir uma medição histórica, usar uma cópia de trabalho da pasta: vários scripts escrevem um recibo com nome fixo e poderiam sobrescrever a evidência preservada. Compilar primeiro o binário e confirmar seu SHA-256; os scripts não restauram a árvore Git anterior. Os arquivos `.so`, PDFs, imagens, XML e logs gerados continuam excluídos pelo `.gitignore` de diagnósticos.
