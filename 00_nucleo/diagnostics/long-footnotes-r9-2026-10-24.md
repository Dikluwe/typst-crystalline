# R9 — notas longas com quebras explícitas

R9 corrigido no escopo medido. Os **75 controles passam em PDF e SVG, em debug e release**, com todas as palavras e páginas físicas e tolerância mantida de **0,001 pt**. As nove regressões de domínio passaram por RED→GREEN. O caso de cinco linhas em página de 80 pt passa de 1/2/2 linhas da nota para 0/2/3; notas de 12 e 20 linhas conservam todo o conteúdo.

A referência permanece o commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore `cac836c07e4dec2a89219370428292393a3216b5`, executável SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. A versão textual não foi usada como identidade. A receita ratificada continua no [resync fixado](../../lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md).

## Estado e proveniência

HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **working tree não commitado**. O estado recebido coincide com o encerramento de R8. Cada medição preserva UTC, comando, fontes, binários, inputs e `git diff HEAD --stat`; o encerramento foi auditado em `2026-10-24T19:26:41.258115+00:00`. [baseline.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/baseline.json), [received-source.json.gz](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/received-source.json.gz), [received-baseline-audit.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/received-baseline-audit.json) e [completion.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/completion.json) vinculam esses estados sem atribuir medições intermediárias ao código final.

O corpus recebido passa em **18/75** controles PDF e falha em **57/75**. O corpus inclui custos positivos/zero/negativos, múltiplos parágrafos, linha vazia, prosa posterior, notas posteriores, três fontes, tamanhos 11/17 pt, páginas de 70–120 pt, duas/três colunas e notas de até vinte linhas. Os três controles de notas separadas por um parágrafo já passavam no recebido; eles impediram uma regressão da tentativa de colocação antecipada.

## Correções e L0

- Certificados privados ligam quebras explícitas às linhas realmente capturadas, respeitando os grupos inicial/final e o bloco de três linhas conforme custos estritamente positivos. Uma captura incompatível com esse certificado conserva o caminho anterior.
- A continuação usa a capacidade da região futura; a primeira entrada continua descontando separador e gap. A drenagem termina por progresso real da fila, sem limitar páginas pelo número de notas e sem perder as últimas linhas.
- A origem ascender/cap-height é aplicada uma vez nas linhas comprometidas; o fechamento manual de linha única permanece preservado. Itens, decorações e alinhamentos recebem a mesma transformação.
- As colunas de página conservam o fluxo ativo durante a drenagem. A chamada é admitida antes de despachar seu filho original; o novo estado privado também participa do rollback.
- A prosa posterior respeita as áreas ocupadas pelas notas e as entradas permanecem enfileiradas até o fechamento da região, preservando o empilhamento de notas posteriores. Marcadores sem tinta não consomem área de nota.

São nove consumidores e nove L0s correspondentes, sem novo contrato público, modo ou fase. A classificação ADR-0127 foi de correção interna contínua. Os hashes finais e o delta exclusivo de R9 estão em [canonical-final.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/canonical-final.json) e [r9-only.diff](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/r9-only.diff). As tentativas refutadas, inclusive colocação antecipada, erro V16 e variantes de depuração, permanecem no mesmo diretório.

## Validação final

| Gate | Resultado |
|---|---|
| Workspace | 12 gates aprovados; build, Clippy com warnings negados, fmt e quatro índices Rustdoc |
| Testes | 9909 Rust aprovados, 7 ignorados preexistentes; 42 Python aprovados |
| R9 | 75 PDF + 75 SVG por build; maior delta PDF 0.000030000 pt, SVG 0.000000001000 pt |
| R8 | 28 PDF + 22 SVG por build, preservados |
| A–H/R1–R7 | 21 replays e 5 suplementos, incluindo os 198 símbolos públicos |
| Estrutura | Lint completo sem erros/warnings; V1/V5/V15/V26 e linhagem reversa de 1003 consumidores sem erros |
| Recursos | Receita R6 mantida; RSS geométrico 0.956101× R5; tamanho normalizado 42742352 bytes (0.854299× R5); sete cargas e três stresses aprovados |
| Preservação | 2144 inputs recebidos fora de R9 intactos; 25274 arquivos históricos verificados |

Recebimentos: [closing-audit.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/closing-audit.json), [pipeline.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/pipeline.json), [validation-complete/validation-commands.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/validation-complete/validation-commands.json), [revalidation/audit.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/revalidation/audit.json), [preservation-audit-close.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/preservation-audit-close.json) e [revalidation/final-resources/receipt.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/revalidation/final-resources/receipt.json). As medições de recursos usam um job, teto de 12 GiB, zero swap e não concorrem com builds.

## Limites remanescentes

O corpus não certifica todo conteúdo possível. A admissão de prosa posterior é finita: texto ASCII ordinário e linhas explícitas que cabem na largura medida; conteúdo opaco/dinâmico, scripts, tracking e outros caminhos recusados continuam fora dessa prova. O certificado de linhas da nota também recusa incompatibilidades com a captura, quebras automáticas adicionais e linhas vazias sem testemunho suficiente.

Os seis limites gerais de `super` de R8 continuam reproduzidos: `DejaVu Sans-literal-newline`, `Nimbus Sans-literal-newline`, `Nimbus Sans-hard-break`, `Nimbus Sans-warm-mixed`, `Liberation Serif-literal-newline`, `Liberation Serif-warm-mixed`. O limite adicional de nota longa foi fechado. Ver [remaining-final-debug.json](../../lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24/remaining-final-debug.json). Os limites anteriores de R2, R3, R4 e pintura R5 permanecem explicitamente preservados na auditoria de replays; este lote não declara paridade universal.
