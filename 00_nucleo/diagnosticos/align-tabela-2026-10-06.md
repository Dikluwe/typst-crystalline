# Grid/Table dentro de Align: origem geométrica e âncora física

Corrigidos os 12 controles exploratórios herdados e os novos controles causais.
O corpus completo passou de **182/228 para 228/228**, sem regressões. O recorte
focado passou de **2/48 para 48/48** em ambos os modos PDF. **12/12 controles
de tinta** passam bilateralmente dentro de 1px a 600dpi.

## Proveniência

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`; anterior UTC
`2026-10-06T13:07:14.349782+00:00`; corpus final iniciado em `2026-10-06T13:13:01.226548+00:00`.
[before.json](align-tabela-2026-10-06/before.json) conserva fonte/L0 anteriores,
stat/status e binário; [execution.json.gz](align-tabela-2026-10-06/execution.json.gz)
conserva os dois consumers/owners finais, hashes, stat/status, UTC e logs.
Tekt anterior SHA-256 `c1d8b39ed7ac52c5a04fc364a3114215c36ae904ae6166d5c392a5a11d37c34a`;
final `c4d483841bf40830599b6977cc2cbd1d0a5f3ec7dd8e0f6eb212e0e600e93914`. Binários congelados em todas as coletas.
Vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` conforme P1489.
Fontes em font-provenance.json. SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1,
tags desativadas, pretty=false; comando, fonte, texto bruto e XML/bbox por caso
em verification.json.gz e coletas focal/compacta. Resumo em
[summary.json](align-tabela-2026-10-06/summary.json).

## Medição antes da decisão

A matriz anterior coloca o primeiro texto de Align(left, Table) em y16,642578pt
contra y24,999978pt; uma coluna RTL começa em x140pt contra x20pt. A diferença
não vem da extensão pintada dos cantos corrigida na frente anterior.
`placement.rs:81-106,270-275` instalava origem geométrica e override horizontal
só para corpos textuais. `grid/tracks.rs:213-223` já calculava topo do grid a
partir do estado inicial; a recomposição posterior descontava a fonte outra vez.
O override também não chegava ao resolver das colunas, que mantinha Start RTL.

O novo teste `aligned_grid_uses_geometric_top_and_physical_anchor_once` ficou
RED antes do código produtivo, com origem/alinhamento errados. No oracle,
`flow/collect.rs:193` e `flow/block.rs:58-70` separam alinhamento de fluxo e
geometria do frame da baseline. São posições observáveis, não exigência de
copiar a estrutura interna do vanilla. Origem histórica não demonstrada.

## Decisão e implementação

- Certificado positivo de um corpo completo Grid/Table, com wrappers transparentes
  e Sequence de um único filho, região finita e VAlign ausente/Top.
- Esse corpo usa origem geométrica, override horizontal lexical e protocolo
  causal de margens existente; o override é restaurado antes da recomposição.
- O envelope conserva largura disponível e não aplica uma segunda translação X
  sobre a tabela já ancorada. Y não recebe desconto artificial de ascender.
- O resolver das colunas respeita override físico em uma ou várias colunas e
  usa line_start_x da região corrente como origem. Sem override, preserva a
  política anterior de múltiplas colunas; ordem lógica não é invertida.

L0 antes do código e resselo limitado aos owners placement/tracks. Correção
interna ADR-0127, sem API/default/fase/unsafe/dependência nova. Bodies mistos,
dimensões infinitas e Bottom/Horizon conservam o caminho legado; não generalizar
paridade a casos não medidos. O estado herdado da árvore foi preservado.

## Validação

**6.543 testes L1 passaram, 5 ignorados**; integrações/doc-tests do pacote também
passaram. Builds debug/release e diff-check passaram. Lint **0 erros, 531 avisos,
1.578 notas**, sem diagnóstico acrescentado/removido; V15/V26 íntegros.

O recorte focal cobre os 12 casos herdados e 36 controles Left/Center/Right,
LTR/RTL, uma/duas colunas, no início, após texto e com texto seguinte. Todos os
48 passam em texto sem formatação do extrator e caixas agrupadas <0,05pt,
em verboso/compacto. Corpus completo 228/228; delta máximo agrupado 0,016206pt.
Os números são por coleta, não devem ser somados entre recortes sobrepostos.

Raster dos 12 controles root: 600dpi, gray, limiar 180, comparação bilateral
±1px. Zero pixels sem correspondência em cada sentido, com inspeção visual
de RTL/Right/duas colunas. raster.json e scripts conservam comandos/SHA/PDF.
Não houve benchmark nem alegação de ganho de desempenho. A matriz macro
independente foi coletada com o mesmo binário em
[paridade-dominios-2026-10-06.md](paridade-dominios-2026-10-06.md).
