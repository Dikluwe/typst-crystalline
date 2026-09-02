# P1293 — relatório final de verificação independente

**Veredito:** `APPROVED_COMPLETE`  
**Medição:** 2026-09-02T20:10:38-03:00 — 2026-09-02T20:21:26-03:00  
**HEAD:** `7dd25ff0e222b6c7c640d6bc7957b98f94227507`  
**Estado:** working tree não commitado; `git diff HEAD --stat` = 63 ficheiros,
7291 inserções, 750 remoções; SHA-256 do stat
`2455be7f0baa922287b036e0dcb1a0acfbc392000d56ee1dbaec278f3ee806f5`.

## Limite da atestação

`PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED`

Claim: `segregado por capacidades e artefatos, sem isolamento técnico de leitura`.

O resultado não alega isolamento técnico, cumprimento operacional perfeito nem
equivalência funcional geral. A aprovação cobre somente o candidato, contratos,
inventário e superfícies congelados no manifesto identificado abaixo.

## Cadeia causal congelada

- manifesto: `0e2e33d8c67a77d0e261451c8ffde67a37344655e9205e8af944e628f7013fe8`;
- selo read-only: `5e9b52b970e0eaf532415fd842f8ff5472c73e857c2446f37c6e6b96f4c36455`;
- bloco canônico: `04460e7920e9f87269b2594829aeb7e9cb878bd4373d1c94cb85ded45f2e8fbc`;
- inventário efetivo: `74/74`, canônico
  `33fe3dcd9cd287b24a82e255f7dffd2728cc76aab969ad5b049d35bbcf7b9c42`,
  zero mismatches antes e depois;
- recibo de preservação:
  `396c05a5319df8549e934a3764f3a0c42183fb3fa2174b346783bf430f7aa1b5`;
- recibo de verificação:
  `2ade14a5fc97b240e7c419925c6c3361b05c45e0315bc2f9ffadb244eefe87a6`.

## Gates executados

Todos terminaram com exit 0:

- `cargo test --workspace -q`: core `5417/5417`, infra `918/918`, CLI
  `71/71`, contratos P1292 e P1293 `11/11` cada;
- `cargo build --release`; binário
  `c527b4111493f444d66e44e6d38d4823b2d09515c73d92eda1b9c4a2a86bff90`;
- `cargo fmt --all -- --check`;
- `crystalline-lint .`;
- `crystalline-lint --checks vN --fail-on warning .` para V3, V4, V5,
  V7, V13, V14, V15 e V26;
- `crystalline-lint --fix-hashes --dry-run .`: `Nothing to fix`;
- `git diff --check`.

P1288 fechou 46 casos Preserved, quatro casos deliberadamente opacos e zero
Violated, com concordância de ordem e duas suítes de harness `10/10`. P1289,
P1290 e P1291 fecharam respectivamente `1/1`, `7/7` e `30/30 + 5/5`.
P1292 e P1293 foram repetidos `11/11` em ordem normal e reversa. As regressões
DOM/math/grid/table ficaram cobertas por P1293 core `51/51`, P1105 `6/6`,
P1293 infra `6/6` e pelo workspace integral.

## Superfícies e poder discriminatório

- default: `111 total / 99 MATCH / 12 diferenças`, SHA-256
  `72a50909525231baafbdefbbc95e2c827eaab833f1062b01f5347636fb4d1106`;
- HTML: `115 total / 104 MATCH / 11 diferenças`, SHA-256
  `a986b41913d0f00b3ff8a8c77def9b487603dfa0c5f1eef6cfa8b4705b180314`.

Ambas têm missing, unverified e Unknown iguais a zero.

A campanha fresca executou baseline protegido duas vezes (`11/11` cada) e
31 mutantes válidos em forward e reverse: `62/62` execuções rejeitadas,
mutation score `1.0`, survivors `0`, Unknown `0` e ordem estável. Resultado
bruto SHA-256
`a2ae8c75b7792201c902e8c8d38e6ec2071328ebbcb10a6e09f6d7372a051e55`.

## Conclusão

O candidato congelado satisfaz os gates mandatórios e preserva todas as
entradas protegidas. O P1293 está completo dentro do fragmento observável
registrado e pode receber o certificado final separado, sem commit automático.
