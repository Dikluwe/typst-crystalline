# P1298 — relatório final pré-commit

## Veredito

`P1297_CERTIFICATE_PRESERVED_P1298_EVIDENCE_CLOSURE_READY_NOT_COMMITTED`

O P5 selou documentalmente o fechamento append-only do grafo evidencial e
autorizou exclusivamente o P6 a revalidar, adicionar literalmente os 48 paths,
executar os checks cached e criar o único commit autorizado. Este relatório é
`PRE-COMMIT`: nenhum staging P1298 ou commit foi executado, nenhum hash futuro de
commit é conhecido e nenhum write de conteúdo pelo P6 está autorizado.

## Claim máxima literal

Fechamento append-only do grafo evidencial P1294–P1297 verificado para os
artefatos e hashes registrados; o certificado P1297 foi preservado com sua
divergência documental explicitada, sem reexecução de produto, sem absolvição
de predecessores e sem alegação de equivalência funcional geral.

`PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED`

## Cadeia causal pinada

| Artefato | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1298.md` | `f56220dfb38e4222502391c5d6e429d988214006edffc5bd6d1934eb715a3033` |
| `00_nucleo/diagnosticos/p1298-manifest.json` | `176c5d7b59268febb0091d1107faaa2a3737cb64855f0a3d34eba25d4fdb038c` |
| `00_nucleo/diagnosticos/p1298-audit-receipt.json` | `75f0ae65c55870486a4b707c3bb13f941255e23c63476d6c166b88e7fe94e0cc` |
| `00_nucleo/diagnosticos/p1298-terminal-ledger.json` | `834d58022563ffb3f35fa6ff04a21816160a65a6a21357067b7327ab462a8138` |
| `00_nucleo/diagnosticos/p1298-staging-manifest.json` | `8527e3fb28cae67c1b93b48264b2e6f4b23345f6b702db92f22cb264e1b0f1c2` |
| `00_nucleo/diagnosticos/p1298-verification-receipt.json` | `d3f82fb2c1734916ad8f5ec7a7e001c5a12db99fd915ed8da7680d995aea5790` |
| `00_nucleo/diagnosticos/p1298-certificate.json` | `cb5bedc0ff6645427b1606d36200d6d0393e745148193616aaf719d6369eed9a` |

O receipt P4 tem veredito `PASS`, autoriza somente o P5 e registra os gates
proporcionais verdes. O certificado P5 foi criado e validado antes deste
relatório; ele pina a allowlist ordenada de 48 paths, a mensagem exata
`fix: seal causal watch recovery through step 1298` e as condições que o P6
deve reproduzir antes de qualquer staging.

## Continuidade P1297 e reconciliação

Os bytes terminais P1297 permanecem pinados:

- ledger congelado: `bfc87da5047004f1cce67df0a14a7f6b9e1fb341a77169fc6ac1af8faaa91db0`;
- receipt final: `c9dd6046ebea9c90397212c1fce2ec3e00d5e473df0211f3cecef9c0c50ba111`;
- certificado: `b7d429d76fd691238eef6ad2a67c8bf7631a5e91fa48b042d2d4f947f6ddac2b`.

A divergência é
`DOCUMENTARY_FIELD_LABEL_MISMATCH_NON_RETROACTIVE`: o receipt real possui
`verdict=PASS`, `ready_for_certificate=true` e estado terminal terminado em
`P1297_PASS_READY_FOR_CERTIFICATE`; o certificado P1297 usa o label aninhado
`PASS_READY_FOR_CERTIFICATE`. Portanto `field_value_equal=false` e
`semantic_authorization_consistent=true`; nenhum dos dois artefatos foi
reescrito.

## Preservação e limites

P1294, P1295 e P1296 permanecem `BLOCKED`, byte-preservados e não absolvidos.
Produto, L0, contratos, testes e superfícies P1297 permanecem nos hashes
enumerados no certificado P1298. Não houve reexecução de produto, build,
workspace tests, stress ou mutation testing; não se alega isolamento técnico de
leitura do filesystem nem equivalência funcional geral.

Antes da escrita P5, o HEAD era
`76fb7336311bdb6497456ab5fdc0a8ce355ff39b`; o índice continuava com 22 paths,
hash de nomes
`83cf1db8b37e229f8fe2d9e6276a28f3306f23c2991f9826afe71caad26ca384`
e hash do patch binário
`65d890b79c83bee191dbbf5d24f38bf180dc0f9135e23cc3a179f3a17d3f1314`.

## Autoridade seguinte exclusiva

Somente `P6_STAGER_AND_COMMITTER` pode prosseguir, e apenas se todos os hashes
permanecerem idênticos. O P6 deve: revalidar as 48 identidades; confirmar os
hashes do índice inicial de 22 paths; executar `git add --` com a lista literal
dos 48 paths, sem glob e sem `git add .`; exigir igualdade exata do conjunto
cached; executar `git diff --cached --check`; confirmar ausência de alterações
unstaged nos 48 paths; e criar um único commit com a mensagem exata
`fix: seal causal watch recovery through step 1298`. Qualquer divergência
bloqueia a autorização.
