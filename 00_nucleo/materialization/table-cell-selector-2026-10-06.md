# D08A — pacote de ratificação do seletor bare de table.cell

Status: PROPOSTO; nenhum L0 vigente ou código produtivo D08 foi alterado.
O reparo dos resíduos D07 já foi materializado independentemente.

## Medição

Diagnóstico: 00_nucleo/diagnosticos/table-cell-medicao-2026-10-06.md.
Morfologia medida: selector(table.cell) → cell; table.cell.where() →
cell.where(:); igualdade → false. Query e counter rejeitam cell não locatable.
ElementKind é índice estreito; adicionar Cell ali legitimaria associação
indevida à introspecção. Element com fields vazio já significa where presente.
Um campo sentinela violaria o L0; nomes de função não preservam identidade.

## Escolha concreta

Adicionar ao enum público Selector a variante fechada
`NativeElement(crate::entities::func::Func)` para ausência de grupo where.
Preservar Element para grupo presente, inclusive vazio. Reconhecer somente
a nativa table.cell adicional por endereço real; conservar mappings Kind
anteriores. Query/counter validam locatabilidade e ancoram erro no valor;
repr e igualdade preservam bare/where, identidade, aliases e composições.
Sem NodeKind novo, vtable, callback, execução do constructor ou I/O.

## L0s completos e hash para ratificação

Os dez L0s completos e um núcleo declarativo estão em
`00_nucleo/materialization/table-cell-selector-2026-10-06/l0/`.
O manifesto aponta cada proposta para seu consumer exclusivo, conserva os
hashes vigentes e lista os hashes completos de todas as propostas.

Manifesto: `00_nucleo/materialization/table-cell-selector-2026-10-06/manifest.json`
SHA-256: `f80f85c4ac3ab747d20d98ef3f5758c59727bcf5f5d762082660099aef4dcc9c`.

A confirmação desse manifesto ratifica a extensão pública acima e permite
a promoção atômica dos dez owners e do núcleo antes de testes/implementação.
Não promover um L0 isolado nem usar o Passo como @prompt.

## Fronteira do pacote

D08A é deliberadamente incompleto para o D08 inteiro: D08B completa show
direto, body/campos, posições, células implícitas/padding e escopo/headers.
A fonte resolve células antes de seu show; essa realização pede especificação
própria e eventual gate de fase. Este pacote não ratifica D08B nem paridade
universal de tabelas. Só o D08A está concreto e pronto para materialização.

## Por que o gate é obrigatório

ADR-0127 §2 exige confirmação para contrato público; o L0 vigente de Selector
registra que um novo braço de enum quebra matches Rust externos exaustivos.
A correção do D07 seguiu fluxo contínuo, mas esta mudança altera esse contrato.

## Ratificação e execução posteriores

O humano confirmou o manifesto acima. Receipt: table-cell-selector-aprovacao-2026-10-06.json.
D08A foi materializado e revalidado; o manifesto conserva seu conteúdo/hash
original. Relatório: 00_nucleo/diagnosticos/table-cell-selector-execucao-2026-10-06.md.
Um consumer de observação exaustivo recebeu reparo interno adicional em fluxo
contínuo; não houve novo contrato público ou fase além da variante ratificada.
D08B permanece fora desta ratificação, conforme a fronteira original.
