# #8935 — nota referenciada em cabeçalho repetido

Em 2026-10-31, na árvore compartilhada não commitada sob HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, a correção inicial eliminou efeitos de medida, readmissões de fases e Locations novas nas cópias do cabeçalho. A continuação corrigiu a largura física das entradas, a reserva de notas no pai restaurado dos subframes e o padrão/sobrescrito das referências à declaração original. L0 veio primeiro, com RED→GREEN; nenhum contrato público ou fase foi acrescentado.

O caso oficial agora realmente repete seu cabeçalho e compila. O compacto tem uma entrada por página da chamada; a Box estreita também mantém sua entrada junto da chamada. Referências explícitas alfabéticas/romanas mantêm o padrão original. Query/contador continuam observando um único membro lógico e a próxima declaração real recebe 2; duplicações verdadeiras continuam rejeitadas. Essa semântica já aprovada em #8954 difere do defeito do oracle `64c5e8c02fe0394d913ead36e42ef8c1334d2107`.

Os 14 testes #8935 (sete anteriores e sete novos), 77 de footnote, 14 de subframes e 73 de colunas passaram. PDF e SVG foram extraídos, comparados numericamente e inspecionados: nas sete cenas de pintura, as coordenadas de glifos coincidem com vanilla; a maior diferença de bbox PDF é 0.00004pt, abaixo de 0.001pt. As únicas diferenças de outlines no cabeçalho compacto são os marcadores 1 versus 2/3 herdados da semântica aprovada.

A continuação de 2026-11-01 corrigiu agrupamento/posição/paginação das notas em `columns()` e `page(columns:)`, inclusive Box estreita, e a configuração lexical `set footnote(numbering:)` na chamada, entrada, query e referência original. Passaram 15 testes novos e os 14 da #8935; os controles amplos passaram (87 footnote, 14 subframes, 83 colunas, com sobreposição entre filtros). L0/Nuclei foram atualizados antes do código e RED→GREEN preservado. PDF/SVG das cenas corrigidas foram comparados e inspecionados. Há espaçamento anterior entre referências adjacentes fora do escopo; não se certifica paridade universal de Footnote/Table.

Build passou; V15/V26 e selos/reversos próprios válidos. A coleta global final ainda encontrou V5 concorrente em `03_infra/src/svg_image.rs`; Clippy com `-D warnings` encontrou três large_enum_variant externos em PageFill/Paint/TilingBody. Não foi feito reseal nem reparo de trabalho concorrente para mascarar esses resultados.

Builds/testes ficaram em target exclusivo RAM, um job e limite de 12GiB por processo. Os recibos preservam UTC, HEAD, diffstat completo, fontes/owners e binários. Caches exclusivos concluídos foram liberados, preservando os executáveis/probes; não houve commit/push nem descarte de mudanças concorrentes.

[Relatório da continuação, resultados e limites](../../lab/results/construction/diagnostics/issue-8935-residual-2026-10-31/report.md).
[Relatório histórico da correção inicial](../../lab/results/construction/diagnostics/issue-8935-2026-10-31/report.md).
[Relatório da correção de colunas/numbering e recibos finais](../../lab/results/construction/diagnostics/footnote-columns-numbering-2026-11-01/report.md).
