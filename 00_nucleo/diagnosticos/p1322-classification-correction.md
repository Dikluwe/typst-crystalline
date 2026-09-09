# P1322 — retificação da primeira emissão local C

A primeira execução local do gerador C usou indevidamente `oldLedger.length` em dois campos de resumo temporal: `previous_probe_count = 2231` e `added_paths = 2487`. O ledger P1309 contém 2182 probes principais **mais 49 sentinelas suplementares**. A fonte correta para o denominador temporal é a coleção de IDs de `p1309-matrix-normal.json`: 2182. O catálogo P1322 contém 4718, portanto adiciona 2536 paths. As 18872 transições por célula já estavam corretas: 8716 com classe igual, 12 fechadas agora e 10144 novas.

O classificador encontrou a falha na inspeção imediatamente posterior à emissão e corrigiu o gerador para `historicalPrincipalCount`, reemitindo seus nove artefatos locais por `apply_patch`, **antes de receber a instrução do coordenador de preservar a primeira emissão e produzir sucessor imutável**. A primeira emissão local não foi arquivada integralmente: não se alega que tenha sido preservada. O coordenador detectou a mesma falha independentemente. Nenhum histórico P1309, recibo bilateral, catálogo, produto ou L0 foi alterado. Isto foi uma revisão do rascunho local C, não correção de produto ou de medição.

Congelamento entregue para revisão D após a correção:

- `p1322-classification-summary.json`: `98b98e0debaf5d5fc92f84b7f5b4dbe1973ee1a95f18b521ff039c65304d706b`;
- `p1322-classification-selection.json`: `3d8ff907008cb4a380ca730b59e6c52a127696842d859c88482f0ae59fe83bc8`;
- `p1322-classification-owner-ledger.tsv`: `6fdf058c92dda6f3ac11dd691c1a53b4edf8c4fc3d5529cb32b9cd23bc1cce3a`;
- `p1322-classification-transitions.tsv`: `2204911c65914d344697604323d5dd71752fc46b53ceea5957adf5a73179bd01`.

Contagens brutas, catálogo, classes por probe, transições por célula, owners, regras e vencedor não mudaram. Revisões posteriores a este congelamento devem produzir sucessor nomeado, preservando os artefatos entregues acima. Não há autoveredito C; o revisor D decide a aceitação.
