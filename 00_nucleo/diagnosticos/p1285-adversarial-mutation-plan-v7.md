# P1285 — plano adversarial v7 final

**Estado:** `FINAL_REFINEMENT_CANDIDATE_FROZEN_READY`  
**Regime:** protocolo completo segregado; refinamento adversarial final.  
**Instante:** `2026-08-30T17:16:48-03:00`.  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.

Este plano preserva integralmente v6 e o receipt v2. São proibidos leitura ou
execução de oráculo, baseline, runner, receipt do oráculo, `materialization/`
e `context/`.

## Entradas pinadas

| Entrada | SHA-256 |
|---|---|
| contrato | `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2` |
| RED receipt final | `dd1ab3b91a9aeef9c95f02003e21a5f5d89816d8cfdb09370dcec8c4fa307455` |
| plano v6 | `24a5c942b45dcccb4a03860f10e55e4358e6bbad81913d86abbc75565fa4f740` |
| receipt v2 | `249ecd54e57a09496ec8c306d12c1fc04130bef42240ca090b8ebcc70a814f5e` |
| `eval/mod.rs` | `b4cee9b297457726cf42fad5737edb45f0b17904ec6ebdf7f6ddc13ef24fa9ea` |
| `eval/repr.rs` | `cbd8205d779b40407e3406f6b4e6aeaa486e5fc84ca669cb30ab4b4cd3a7007b` |
| `eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` |
| `eval/rules.rs` | `d9bef037b7c4fb025193fec210f7015a5aceae2f73c1317d7db36bbaab01ae3f` |
| `eval/math.rs` | `2c7d920337ee46a3423db5a0809bab73e11685c8cf0f32e4984091143976ba1e` |
| `compiler/stdlib/foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` |
| `shell/cli.rs` | `87f7983ec6ebc261603af3d9147faa936c79bb20582c7c1359fe29dd9ff11e85` |
| `infra/query_helpers.rs` | `e3157124d94bc83259be375bb0798c0cee41a9f20516c0c45ef0e351182e0aa8` |
| `wiring/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` |

## Delta causal v6→v7

Somente `02_shell/src/cli.rs` diverge do backup v6, dentro do teste histórico
`p1225_eval_json_stroke_e_nominal_e_raw_permanece_proibido`:

```text
Color::rgb(0,0,0) JSON: is_err() → bytes "rgb(\"#000000\")" + newline
```

Produção e os cinco testes owners `p1285_*` no ficheiro permanecem
byte-idênticos. Os outros oito consumers completos são byte-idênticos.

## Partição

Rerun obrigatório por alvo integral `cli.rs` alterado:

```text
M-Y1 M-Y2 M-Y3 M-Y4
M-J1 M-J2 M-J3 M-J4 M-J5
M-Q1 M-Q2 M-Q3 M-Q4 M-Q5 M-Q6
```

Transferência v6 somente por consumer alvo byte-idêntico:

```text
M-Y5 M-Q7 M-Q8
M-S1 M-S2 M-S3 M-S4 M-S5 M-S6 M-S7 M-S8 M-S9 M-S10
M-M1 M-M2
```

Total: 15 reruns + 15 transferências = 30 linhas reconciliadas.

## Execução e política

1. Nova cópia explícita `/tmp` e backups byte a byte dos nove consumers.
2. Baseline mínimo final de 19 testes antes da primeira mutação: os 18 do v6
   mais `p1225_eval_json_stroke_e_nominal_e_raw_permanece_proibido`.
3. Um mutante por vez; owner executado; restore pelo backup; SHA v7 obrigatório.
4. `Unknown` não mata; survivor ou `Unknown` impede score combinado 1.0.
5. Repetir baseline 19/19 e hashes na cópia/origem após o último restore.
6. Escrever somente plano v7 e receipt v3; não editar produção, testes, L0,
   contrato, RED, v6/v2 ou artefactos proibidos.

Score combinado exigido: `30 / 30 = 1.0`, `0` survivors, `0` Unknown.
Este plano não é veredito.
