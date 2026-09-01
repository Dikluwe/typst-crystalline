# P1287 — receipt adversarial de mutação

**Estado:** `ADVERSARIAL_GATE_12_OF_12_KILLED`  
**Papel:** Adversário segregado `/root/ataques_p1287`  
**Regime:** protocolo completo; segregado por papel/capacidade/artefatos, sem isolamento forte do host  
**Instante:** `2026-08-30T22:51:28-03:00`  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`  
**Árvore:** working tree não commitida e compartilhada

## Atestação de capacidade

Foram lidos integralmente a skill, as duas referências, `AGENTS.md`, somente o
passo P1287 autorizado, manifesto/receipts e artefatos congelados. Não foi lido
código candidato em `01_core`–`04_wiring`; nenhum binário `target/debug` ou
`target/release` foi executado. Não foram alterados contrato, runner, testes,
fixtures, baseline, L0 ou produção, e nenhum veredito global foi emitido.

A escrita persistente ficou limitada a este receipt e ao plano adversarial.
As injeções ocorreram em 12 mirrors independentes sob
`/tmp/p1287-adversary.0MLDpN`; cada mirror foi restaurado após o probe.

## Integridade antes/depois

| Artefato congelado | SHA-256 confirmado |
|---|---|
| runner | `745a4c470704de54f9a550d220a3092bfa8452e3b02cb7010846758eab36bd37` |
| testes | `3ac4daa6ff8cb2c9d8396ec4288b737c8496d0473c5d5d901165574af241c535` |
| baseline | `ff09c8146b73684b2e746c631a6103817b6df7070aa8248e2d88dbca34749488` |
| receipt de oráculo | `b8ba1fa8d3d7bf1a305deee400ed811b5e034888ec4aeb3797392a95ea97e109` |
| manifesto | `5d4cc9a6180300c8402be4a91b30104db08874f8540c9bc8af5b895a9fdf725b` |
| receipt de contrato | `4683d23a5a489e258931ea3389be809c7b4e5b143c4f7a66ec7210c4dbd4f7a5` |
| receipt baseline vanilla | `3bbba47235e4865bb3c9e788bb709215dd0a53297ee587d4334a01ca42126cd6` |

## Execução reproduzível

Um driver Python efémero criou cada mirror, exigiu exatamente uma substituição
textual, importou o mutante, executou o probe específico, restaurou runner e
qualquer cópia auxiliar tocada e repetiu o probe no original restaurado. Não
houve compilação Typst. Resultado canónico temporário:

`/tmp/p1287-adversary.0MLDpN/campaign-result.json`  
SHA-256: `1c27f94bcd4d6ba2c843ad6e80d16be91584490cc0c4ec85ae7992642ffa88e6`

## Resultados

| ID | SHA-256 do runner injetado | Kill | Restauração/probe |
|---|---|---|---|
| M01 | `b41a66825c451344d0cd48307a423d877bf8a18d3a748f5fce6727328535b443` | sim | original/pass |
| M02 | `4d01e0a2ca423409b3eaf5684069e7c7aed5f239e5739720f0f1617e514ace1e` | sim | original/pass |
| M03 | `6d02a1aa807c7c8c3e92bf59e03f9fc8bff68549f1db5f6742e667828269270c` | sim | original/pass |
| M04 | `c6b55e515519db252c2aa8821d31ab90d856cf2e048a8e8cd7a54e777bed6a1f` | sim | original/pass |
| M05 | `e223cc2f3a1802251dc2bd56f57a597df20dc311edd902cf973fef13eff8c1be` | sim | original/pass |
| M06 | `c11fcbd94b7a066d279b9a87872a0019c2f8ff38271a597e40f0b4558d77fc40` | sim | original/pass |
| M07 | `2142b58c88f968efd985f977c2366f639821409a8a1b813e577518f17b9d3bc7` | sim | original/pass |
| M08 | `bcedf5d50acf3e14268b9652b625a44e0d6ec822135105629d6dadfe5b549abd` | sim | original/pass |
| M09 | `57b7cea0a63afce82250b9ac788dccc74a15f379b2d7e6cbf2d988a6e8b3d607` | sim | original/pass |
| M10 | `6843e8346cb6232580624d6ec87c4760c512928b9c3ff4414ef7d0020081316a` | sim | original/pass |
| M11 | `ccf1104ad0a5c61260b7eecf1c327668841b24bb873da07d9b76b0aefd38de4b` | sim | original/pass |
| M12 | `e17f515aef7ae5efaf0f859ac072b720690eaa2249d17adbf6efcb2412e67435` | sim | original/pass |

Todos os 12 mutantes importaram; todos foram mortos por falha do probe
discriminatório; em todos os 12 mirrors o runner restaurado voltou ao SHA
`745a4c470704de54f9a550d220a3092bfa8452e3b02cb7010846758eab36bd37`
e o mesmo probe passou.

`mutation_score = 12 / 12 = 1.0`.

## Proveniência da medição

No instante registrado, `git diff HEAD --stat` mostrou `124 files changed,
647273 insertions(+), 1789 deletions(-)`; SHA-256 da saída:
`8b22c5e3e6a3427dde83ddce564dfa45986b4d49f341a92be0619104f4ec1d5c`.
SHA-256 de `git status --short`:
`42301bddcc5888bc2385d608d5b345826799f66d3d496284be8ebe72a5e6615e`.

## Resultado proporcional

O gate adversarial local está satisfeito para as 12 decisões enumeradas. Isto
não remove os sete `Unknown` congelados, não prova identidade do baseline, não
atesta cobertura global da linguagem e não constitui veredito global de paridade.
