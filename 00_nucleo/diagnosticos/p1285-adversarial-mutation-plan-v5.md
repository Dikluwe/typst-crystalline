# P1285 — plano adversarial v5 repinado na candidata final

**Estado:** `CANDIDATE_FROZEN_READY_FOR_MUTATION`  
**Regime:** protocolo completo segregado, papel adversário/mutation tester.  
**Início:** `2026-08-30T16:07:45-03:00`.  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.  
**Workspace de ataque:** `/tmp/p1285-adversarial-v5.AuqIKr/repo`, cópia
reflink explícita da working tree congelada.

Este plano repina, sem alterar o plano v4, as mesmas 30 mutações e owners sobre a
candidata final comunicada pelo coordenador. Não autoriza leitura/execução de
oráculo, baseline, runner, receipt de oráculo, `materialization/` ou `context/`.

## Entradas protegidas

| Entrada | SHA-256 |
|---|---|
| contrato v4+precision | `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2` |
| receipt RED final | `3f093f748769ccbb128183ca73cfab9052f2a7d6b097503ab5e5ccff2532f25d` |
| plano v4 | `306ae3032df39661b71b313f1bf7251eedf913ddf7b0287005b8c6d849285212` |
| `eval/mod.rs` | `b4cee9b297457726cf42fad5737edb45f0b17904ec6ebdf7f6ddc13ef24fa9ea` |
| `eval/repr.rs` | `aa47dd1f62f8ccfb25e54166b32c9c64b796f743968c28708d87d42ee2ff853b` |
| `eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` |
| `eval/rules.rs` | `d9bef037b7c4fb025193fec210f7015a5aceae2f73c1317d7db36bbaab01ae3f` |
| `eval/math.rs` | `2c7d920337ee46a3423db5a0809bab73e11685c8cf0f32e4984091143976ba1e` |
| `foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` |
| `shell/cli.rs` | `e00dbc1f445f6c3ab1976740daeebb4c4e2a6668fb409d8c25d14ae0e206857b` |
| `infra/query_helpers.rs` | `e3157124d94bc83259be375bb0798c0cee41a9f20516c0c45ef0e351182e0aa8` |
| `wiring/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` |

Os nove pins normativos L0 sem a linha `Hash do Código` permanecem os registados no
plano v4. Mudança seal-only é proveniência; drift no pin normativo bloqueia.

## Protocolo de execução

1. Verificar os hashes acima na origem e na cópia antes do baseline verde.
2. Guardar backup byte a byte dos nove consumers dentro do diretório temporário.
3. Para cada mutante, aplicar exatamente uma transformação semântica compilável,
   correr o owner duas vezes, capturar stdout/stderr/exit e classificar causalmente.
4. Restaurar o consumer com cópia byte a byte do backup e exigir SHA-256 congelado
   antes da mutação seguinte. A origem compartilhada nunca é escrita.
5. Ordem deliberadamente permutada face ao contrato:

```text
M-S7 M-J3 M-Q6 M-Y2 M-M1 M-S1 M-Q3 M-J1 M-Y5 M-S10
M-Q1 M-S4 M-J5 M-Y1 M-Q8 M-S2 M-J4 M-Q5 M-M2 M-S8
M-Y3 M-Q2 M-S6 M-J2 M-Q7 M-S9 M-Y4 M-S3 M-Q4 M-S5
```

6. `KILLED` exige mutante válido + falha do owner pela testemunha prevista nas duas
   repetições. `UNKNOWN`, timeout, falha de infra, build inválido ou falha não causal
   não mata. `SURVIVED` impede score 1.0.
7. Ao fim, repetir hashes das entradas, HEAD e `git diff HEAD --stat` na origem.

## Matriz repinada

Os IDs, obrigações e owners são integralmente os 30 da seção 3 do plano v4:
M-Y1…M-Y5, M-J1…M-J5, M-Q1…M-Q8, M-S1…M-S10 e M-M1…M-M2.
O score exigido permanece `30 / 30 = 1.0`; este plano não é receipt nem veredito.
