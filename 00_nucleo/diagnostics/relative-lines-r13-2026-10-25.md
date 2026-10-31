# R13 — comprimento relativo de linhas — 2026-10-25

**Corrigido no escopo medido:** `line(length: 50%)` funciona no fluxo, em regiões
locais e como separador de notas. Percentuais, comprimentos mistos e em chegam
ao layout sem perder seus coeficientes; cada eixo usa sua dimensão da região.
Os três refutadores originais de R12 agora qualificam em PDF/SVG nos dois builds.

## Estado e proveniência

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, working tree não commitado,
recebido com R11/R12. O [baseline](../../lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/baseline.json)
preserva UTC, `git diff HEAD --stat`, status, 2164 inputs e hashes dos binários
recebidos de R12. `received-source.json.gz` conserva os bytes anteriores.
Cada medição posterior registra seus próprios inputs, horário, comando e binário;
não se atribui resultado do baseline ao código final.

Vanilla fixado: commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore
`cac836c07e4dec2a89219370428292393a3216b5`, executável SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Não houve resync. O [encerramento](../../lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/completion.json)
vincula os 2165 inputs finais aos fontes completos, executáveis, documentos e
manifesto de evidências. O stat inclui as alterações existentes, não somente R13.

## Medição anterior e correção

Os recibos iniciais [PDF](../../lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/before/receipt.json)
e [SVG](../../lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/before-svg/receipt.json)
preservam 17 fontes: vanilla aceita 16 e rejeita o controle de tipo inválido;
o produto recebido aceita três e rejeita 14, incluindo esse controle.

| Fonte/contexto | Vanilla: deslocamento, pt | Produto recebido |
|---|---:|---|
| `length:50%`, região 260 × 140 pt | 130 × 0 | Rejeitado |
| `length:50%`, região 360 × 140 pt | 180 × 0 | Rejeitado |
| `length:50% + 10pt` | 140 × 0 | Rejeitado |
| `length:50%, angle:30deg` | 112,583302492 × 35 | Rejeitado |
| `length:50%, angle:90deg` | 0 × 70 | Rejeitado |
| `length:2em`, texto 11 pt | 22 × 0 | 0 × 0 |
| Box de largura 180 pt, `length:50%` | 90 × 0 | Rejeitado |

O construtor rejeitava Ratio e extraía somente a parte absoluta de Length.
Resolver primeiro o comprimento na largura e depois girá-lo também produziria
resultado incorreto, como distingue o controle de 30 graus. O código agora
constrói endpoints afins na avaliação e resolve percentuais/em no layout.
O tamanho causal usa `max(0, start, end)`, preservando origem e sinais.
A medição restrita usa a mesma resolução; isso evita perder a geometria na
reserva do separador de rodapé. Os exporters recebem somente o Path resolvido.

## L0 e autorização

O usuário aprovou explicitamente o tipo público `RelativeLine`, o campo
`ShapeElem::relative_line: Option<RelativeLine>` e o adiamento eval → layout.
O [recibo de aprovação](../../lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/approval.json)
vincula a decisão ao L0 SHA-256
`d8c9a01413232bac7113d1148af3c1b3bcd671bb37eeca68a94d5749d671db00`;
`owners-before.json.gz` preserva esse texto aprovado. O [L0 proprietário](../prompts/core/entities/elements/shape.md)
registra a ratificação. Os consumidores foram especificados antes do código,
com 14 pares exclusivos e um Nucleus compartilhado com pins válidos.
Os reseals ficaram restritos aos pares autorizados.

Construções existentes recebem `None`; a assinatura de `Content::shape` permanece.
Clone, mapas, hash e observação preservam o novo campo. Linhas absolutas,
precedência de `end`, validação de stroke e extensões legadas dx/dy têm controles
de preservação. O default legado da linha sem argumentos e a dívida relativa de
dx/dy não foram redefinidos. Não há nova flag ou modo de produto.

## RED→GREEN e validação

Os recibos `red-constructor-assertions.json`, `red-layout-complete-literals.json`
e `red-measurement.json` conservam falhas assertionais anteriores às respectivas
correções, junto com os fontes completos. As primeiras tentativas interrompidas
por migração de literais incompleta também foram retidas e não contam como RED
assertional. Os dez testes finais de R13 passam: transporte, observação,
clone/mapas, resolução por eixo/fonte, tamanho causal e medição restrita.

A [auditoria final](../../lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/closing-audit.json)
confere os recibos contra os inputs e executáveis finais:

- 25 fontes positivas de R13, PDF e SVG, em debug e release, a 0,001 pt;
  maior diferença de bounding boxes de texto: 0,000006 pt. Incluem os três
  fontes originais de notas, ângulos, sinais, comprimentos mistos/em, origens,
  precedência, Box/inset/nesting, Block, Place e conteúdo seguinte.
- Oito controles de erros tipados nos dois builds, com diagnósticos do oracle
  retidos separadamente; não se declara igualdade geral das mensagens/colunas.
- 12 gates completos passaram: build do workspace, testes, Clippy, fmt/docs,
  todos os targets, linhagem estrita/reversa e verificações estruturais.
  Foram 9946 testes Rust aprovados, sete ignorados e 42 testes Python.
  O lint completo terminou sem violações.
- R12: 54 fontes e quatro controles de fronteira; R11: 68; R8: 28 PDF/22 SVG;
  R9: 75 PDF/SVG; R10: 69 PDF/SVG, além dos controles bitmap. Todos revalidados
  em ambos os builds. Os warnings conhecidos de R11 foram preservados.
- 21 replays e cinco suplementos anteriores passaram, com auditoria semântica
  e de pintura A–H/R1–R7. Os 32 refutadores anteriores foram repetidos sem
  alterar suas classificações: 15 qualificados e 17 ainda não qualificados.

## Pintura e recursos

O [recibo de pintura](../../lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/line-paint/receipt.json)
compara oito fontes por build, em PDF/SVG a 72 e 600 dpi. Há dois pixels SVG
com diferença máxima de um canal em DejaVu Sans a 72 dpi, nos dois builds;
os demais controles dessa amostra têm zero diferença. Os três separadores
percentuais preservam exatamente os rasters das equivalentes linhas absolutas
de R12. A pintura não foi confundida com aprovação geométrica. A ausência de
primitiva visível para a linha zero com cap butt foi confirmada por raster.

Os gates de tamanho normalizado e memória passaram com a receita R6 preservada.
Tamanho normalizado final: 42815648 bytes, razão 0,855764449 contra o executável
de controle R5; razão geométrica de RSS: 0,949983877. Todos os gates por workload,
repetibilidade e latência passaram, assim como os três stresses. O controle R5
é identificado pelo SHA `c3d2d2e416e506f9e49bff28d55a05138030440a6288d88bc77631348dae5549`,
não pelo HEAD atual da árvore de medição. Esses resultados preservam os gates;
não demonstram ganho causal de R13 sobre R12.

## Preservação e limites

O [recibo de preservação](../../lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/preservation.json)
confere 2136 inputs recebidos fora dos pares autorizados, 85601 arquivos históricos
e os assets/fontes. R11/R12 permanecem na árvore. O hash histórico diferente
da licença DejaVu Sans é a normalização de whitespace já presente em HEAD212;
os bytes atuais foram conferidos contra HEAD e não foram alterados por R13.

Continuam abertos os 17 casos antigos e os limites de pintura já documentados.
Regiões infinitas/auto e generalização da reflexão pública não têm certificação
universal por este corpus finito. As três falhas adicionais de R12 com
`line(length:50%)` estão encerradas nos reprodutores originais.

As falhas dos observadores foram preservadas: representação SVG da linha zero
e duas expectativas de coluna no verificador de erros. Seus scripts e tentativas
anteriores constam do manifesto; as correções desses observadores não mudaram
o produto nem os inputs da validação completa.
