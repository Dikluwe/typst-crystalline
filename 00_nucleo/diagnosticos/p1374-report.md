# P1374 — relatório de largura relativa e altura fracionária de `block`

## Veredito

**PASS_WITH_LIMITED_ATTESTATION.** `BlockElem` conserva largura relativa e
distingue altura relativa de altura fracionária. O native aceita as unidades
congeladas, rejeita fração na largura e entrega o peso vertical ao owner de
flow. Em `Sequence` direta, finita e de uma região, o preflight puro desconta
itens/gaps fixos e reparte o restante pela soma dos pesos.

A matriz bilateral fechou os dois observáveis. Na largura 60%, `Right` ficou
inteiro em `xMin=93,668` e `xMax=118,000`, correspondente a 108pt dos 180pt
úteis. Na altura 1fr/2fr, ambos produziram uma página com `One=7,404`,
`Two=82,724670` e `After=220,166`.

## RED, descoberta e GREEN

O RED inicial não compilou porque `BlockHeight`, `extract_block_width`,
`resolve_width_pt` e `fractional_share` ainda não existiam. A migração tipada
fechou em `cargo check`. A primeira composição fracional revelou duas causas
separadas: o newline transparente fazia a medição contar um avanço de linha
hipotético; depois, o Cursor movia a linha terminal no limite ou criava uma
página final vazia. Os L0s foram atualizados antes dessas correções. O filtro
P1374 final passou 4/4, com 5.675 testes filtrados.

## Memória

Todos os builds foram seriais. O link focal com debug completo atingiu o teto
virtual de 4 GiB e abortou controladamente no LLVM, sem OOM do host. Não foi
repetido nessa configuração. Com símbolos de debug desligados, ainda sob
4 GiB e arquivo máximo de 512 MiB, o binário normal e o binário de testes
ligaram; este mede 87.200.968 bytes. A matriz correu sequencialmente sob 2 GiB,
16 MiB por arquivo, canal de 1 MiB e timeout de 45 s. Não houve timeout,
`SIGKILL` nem crescimento sem limite.

## Proveniência e limite

Medição em 2026-09-13T14:40:30-03:00, commit
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado e
agregado com P1356–P1374. O protocolo completo foi executado sem atestação de
isolamento: a mesma autoridade preservou contrato, implementação, ataques e
verificação por hashes, mas não se alega independência forte.

O veredito cobre somente sizing P1374 direto. Wrappers transparentes, colunas,
multi-região e paridade geral permanecem `Unknown` para P1375. A diferença
vertical preexistente da sonda de largura não recebe crédito; o observável
congelado ali é a morfologia íntegra e o alinhamento horizontal.
