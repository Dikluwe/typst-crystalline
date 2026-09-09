# P1327 — revisão final independente

**PASS_SCOPED.** O fragmento P1327, com a sucessão causal R2, satisfaz os
oráculos congelados e os gates finais. Tekt A/B executado sem atestação
técnica de isolamento, sem selo de refinamento e sem alegação de paridade
geral. Revisor não escreveu produto, L0, testes ou expected.

## Base verificável

Manifesto R2 SHA-256
`9ebf13c2cf9a1c1daf186671194c7c4e15db16d8d4e47a12ad5ff4fb597f8ad0`.
Audit próprio reproduzível: `node 00_nucleo/diagnosticos/p1327-review-final-audit.cjs`.
Saída `p1327-review-final-audit.json`, SHA-256
`6f349a8b27ecdb10049a6175fe0a99557582b710772ab0974d7cce8d895e002a`,
registra horário, HEAD, diff/stat e hashes de receipts/artefatos.
Uma primeira tentativa de execução desse auditor falhou no wrapper Node
ao spawn de git; o auditor passou a consumir a proveniência completa do
receipt final. Essa falha de instrumentação não foi usada como evidência
de produto e não alterou artefatos verificados.

Proveniência final: HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working
tree não commitado. Workspace R2 medido de 2026-09-09T11:12:42.770057 a
11:13:18.597704 UTC, receipt SHA-256
`3867b5c75fd0d28eb2065974d094e1640a70860e56d7d4b56b61f531a9e63e6f`.
C2 SHA-256 `75e8b97b3788c0feaf457cb4c06b1c2c6735cff5ef3b9804a75ec57f8b148e31`.

## Código e preservação

O delta modules emite warning somente depois da resolução/validação e
apenas para fonte Ident bare sem as/items. Mantém binding lexical e usa
sink tracked existente sem hints/trace. Formas excluídas permanecem
excluídas. C2 muda somente a apresentação de `run_eval`: errors são
drenados por referência antes dos warnings; o match Err retorna 1 sem
drená-los novamente. O branch Ok conserva serialização/stdout/I/O.

Comparei main.rs com o baseline R2 após mascarar exclusivamente run_eval
e o header: resto byte-idêntico. Assim compile/query/watch/drain e o
trabalho prévio HTML não sofreram edição P1327. Comparei inventários
baseline P1327 e após workspace final: diferenças limitadas aos seis paths
dos três pares autorizados, nenhum produto novo. Reconstruí integralmente
o corpo de eval/tests pelo baseline mais sucessores independentes R1 e
snippet congelado; coincide com o arquivo final excluído somente header.
Os quatro testes legados e seis observações sucedidas continuam exatos,
e o helper histórico de silêncio não foi relaxado.

Recalculei independentemente SHA normativos, pins e hashes recíprocos:

| Par | Hash A | Hash B |
|---|---|---|
| modules | `11d38a08` | `d714fe3f` |
| eval/tests | `4470777c` | `cdbdb24f` |
| wiring | `8a2604da` | `e13e2ed6` |

Normativos originais e wiring R2 permanecem congelados. Oráculos, runner,
snippet, sucessores, manifestação original e resultados C1 mantêm seus
hashes. A insuficiência do owner set original não foi ocultada: C1 segue
Violated e a alteração de owner foi legitimada por L0 e manifesto R2
antes de C2, em fluxo contínuo ADR-0127.

## Gates e alcance

Recomparei diretamente todos os exit/stdout/stderr: corpus original
336/336 e suplemento R2 96/96 Preserved, com chaves completas e únicas
nas três ordens. As 432 observações incluem 384 esperadas do vanilla
ratificado e 48 de preservação explícita de dívida, sem normalização.
O parecer do autor A/B `p1327-ab-c2-verdict.json`, SHA-256
`f18b357d4f2277d9fc656d91333174b41db7ae7e6082561615dda223515171a8`,
também é PASS_SCOPED, sem violations ou falhas de integridade.

RED R0/R1 foi semanticamente válido; GREEN dos mesmos seis testes passou.
Workspace final: 6.686 passed, zero failed, três ignored, soma recalculada
dos resumos do receipt. Build release/locked, fmt, diff-check, linhagem e
gate estrito V5/V15/V26 passaram. Lint geral exit 0 mantém 240 warnings:
comparei suas mensagens, em ordem, com `p1326-final-lint.json`; são idênticas.
Não há declaração de zero warnings gerais nem limpeza de dívida alheia.

Li `p1327-final-report.md`, SHA-256
`96cd39a2a9d7f85f9c46927fb5942e74bdd14e950e8d91285cc77d0902d9c84a`:
substância correta, incluindo falha C1, expansão R2, ignorados, warnings,
perfis de features distintos do target HTML e dívidas preservadas.
Sem achados bloqueantes remanescentes para o fragmento. Closure agregada
pode registrar este resultado; não promove paridade geral, não resolve
rename/tipos/import traces/raw serialization/math e não autoriza outro passo.
