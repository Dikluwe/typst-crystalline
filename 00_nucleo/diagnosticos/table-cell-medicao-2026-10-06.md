# D08 — medição de selector, query e show de table.cell

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Pin vanilla
`9f2b6e8715237cb086899a42873660fe744622e8`, executável SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt da coleta baseline SHA-256 `b5362b800d175487c22071db5fda63f6a2cedb129ca22591b7bfbf998ac1c9ad`.
before: 2026-10-06T21:45:38.928314+00:00 a 2026-10-06T21:45:42.606975+00:00;
table-additional-before: 2026-10-06T21:50:39.198387+00:00 a 2026-10-06T21:50:46.085733+00:00;
table-aliases-before: 2026-10-06T21:53:29.001335+00:00 a 2026-10-06T21:53:29.789998+00:00.
As duas coletas adicionais executam intencionalmente o Tekt congelado do
baseline; o reparo D07 já estava em elaboração nas fontes, com seus hashes/
diffstat registrados. Nenhum consumer D08 foi alterado nesse intervalo.
A revalidação after usa o novo binário do reparo D07, com estado exato próprio,
e conserva os quatro negativos D08 de before. Não somar coletas como certificado.

## Observáveis medidos

| Caso | Vanilla pinado | Tekt baseline |
|---|---|---|
| selector(table.cell) | cell | rejeita função |
| table.cell.where() | cell.where(:) | coincide |
| bare == where vazio | false | falha construindo bare |
| alias verdadeiro em CodeBlock | selector válido, query rejeitada como cell não locatable | selector/query rejeitam função |
| query(table.cell) | cell is not locatable no valor | erro genérico com call trace |
| query(table.cell.where(x:0)) | cell is not locatable no valor | mesma mensagem, span agregado |
| counter(table.cell) | cell is not locatable | erro de função inválida |
| show direto com body | CELL aCELL b | função cell rejeitada |
| show where x:0 | FIRST a b | não transforma |
| show com células explícitas | transforma | direto rejeitado; where não casa |
| padding, 2 colunas/3 conteúdos | transforma também a quarta célula vazia | direto rejeitado |
| show de célula isolada | CELL a | direto rejeitado |
| metadata de posições | (0,0),(1,0),(0,1),(1,1), última vazia | direto rejeitado |
| colspan:2 | (0,0),(0,1),(1,1) | direto rejeitado |

Fixtures antigas cell-alias-selector/cell-alias-query definiam let no documento,
mas eval avalia outra expressão: unknown variable nos dois não prova alias.
table-aliases-before usa CodeBlock na própria expressão e substitui essa prova.
partial-selector é negativo de função não-elemento; conservar sua rejeição,
sem usar o nome cell como identidade. Não há prova de paridade de seu span.

## Fonte e classificação antes da decisão

Upstream foundations/selector.rs:309–321 distingue grupo ausente de grupo
presente, e :421–450 separa validação locatable. model/table.rs:717–725
documenta show de células condicionado à posição. layout/grid/resolve.rs:
204–278 resolve campos do TableCell; :1665–1703 materializa células faltantes;
:1945–1988 calcula propriedades por posição antes de usar o body da célula.
Forma, igualdade, query/counter e transformação são língua; não exigem
replicar vtables, memoização ou algoritmo interno upstream.

Tekt entities/selector.rs:34–37 só tem Element com grupo explicitamente
presente; ElementKind é discriminador de elementos indexados. Criar Cell ali
seria enquadramento indevido, pois ela não é locatable. foundations/selector.rs
omite a nativa cell nas duas rotas diretas. rules/show.rs:306–327 também omite
cell no mapping direto; selector_matching.rs:303 já conhece TableCell pela
nativa. table_grid.rs:169 mantém conteúdos comuns como children, sem células
sintetizadas; content/fields.rs não projeta TableCell. Esses são problemas
separados: admitir a nativa não realiza posição, padding ou campos.

O L0 de Selector exige distinguir where explicitamente vazio; reutilizar
Element vazio para bare violaria esse contrato. A escolha D08A é adicionar
NativeElement(Func), enum fechado com identidade estática e ausência de grupo.
É contrato público: o gate ADR-0127 é obrigatório. Dez owners/núcleo propostos
em materialization/table-cell-selector-2026-10-06, com hashes completos.

D08A declara incompletude e nomeia D08B para show/campos/realização. Não foi
escrito código D08 nem promovido L0 vigente desta proposta. Antes de D08B,
medir escopo, guards, estilos por posição, spans, headers e contexto e decidir
a fase explicitamente; não mover posição/realização para eval por conveniência.
