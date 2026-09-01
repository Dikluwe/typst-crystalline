# P1285 — plano adversarial v6 de refinamento

**Estado:** `REFINEMENT_CANDIDATE_FROZEN_READY`  
**Regime:** protocolo completo segregado; ciclo de refinamento adversarial.  
**Instante:** `2026-08-30T16:48:07-03:00`.  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.

Este plano não substitui nem sobrescreve o plano v5. Não autoriza leitura ou
execução de oráculo, baseline, runner, receipt do oráculo, `materialization/`
ou `context/`.

## Entradas pinadas

| Entrada | SHA-256 |
|---|---|
| contrato v4+precision | `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2` |
| RED receipt final refinado | `5f6d145a6489d23df306b9d9f216ed2668acb63b5bf8e4b04ccde0f7ea6e9391` |
| plano v5 | `9cf5b7304c9761c245ea8be544bb7dbdd8bf6c833822e3f268dbe0c0f23a743a` |
| receipt adversarial v5 | `d2a278e1bc77e677715c1b033e7ae27d0c862c19a9ce68b413371b82b25e8a3b` |
| `eval/mod.rs` | `b4cee9b297457726cf42fad5737edb45f0b17904ec6ebdf7f6ddc13ef24fa9ea` |
| `eval/repr.rs` | `cbd8205d779b40407e3406f6b4e6aeaa486e5fc84ca669cb30ab4b4cd3a7007b` |
| `eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` |
| `eval/rules.rs` | `d9bef037b7c4fb025193fec210f7015a5aceae2f73c1317d7db36bbaab01ae3f` |
| `eval/math.rs` | `2c7d920337ee46a3423db5a0809bab73e11685c8cf0f32e4984091143976ba1e` |
| `foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` |
| `shell/cli.rs` | `a5c7110577501564004ad2f6d84a29ed1a282c94a412364b73e3b493e75d329f` |
| `infra/query_helpers.rs` | `e3157124d94bc83259be375bb0798c0cee41a9f20516c0c45ef0e351182e0aa8` |
| `wiring/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` |

## Delta causal v5→v6

- `cli.rs`: `e00dbc…`→`a5c711…`; somente rustfmt no braço
  `Value::Length`, comprovado por diff contra o backup v5.
- `repr.rs`: `aa47dd…`→`cbd820…`; expectativa test-only
  `gradient(...)`→`gradient.linear()` e produção `Tiling(None)`
  `tiling(..)`→`tiling(...)`.
- Os outros sete consumers são byte-idênticos aos pins v5.

## Partição de refinamento

Rerodar, porque a transformação v5 escreveu em `cli.rs`, e incluir todos os
mutantes `M-J*` conservadoramente pela dependência da fachada de `repr`:

```text
M-Y1 M-Y2 M-Y3 M-Y4
M-J1 M-J2 M-J3 M-J4 M-J5
M-Q1 M-Q2 M-Q3 M-Q4 M-Q5 M-Q6
```

Transferir do receipt v5 somente por consumer alvo byte-idêntico, citando os
SHA v5/v6 no receipt v2:

```text
M-Y5 M-Q7 M-Q8
M-S1 M-S2 M-S3 M-S4 M-S5 M-S6 M-S7 M-S8 M-S9 M-S10
M-M1 M-M2
```

Total: 15 reruns + 15 transferências = 30 mutantes reconciliados.

## Execução

1. Criar cópia explícita nova em `/tmp` e backups byte a byte dos nove consumers.
2. Executar baseline final do RED receipt refinado, incluindo
   `repr_value_complex_types`, antes da primeira mutação.
3. Para cada um dos 15 reruns, aplicar uma transformação semântica compilável,
   executar o owner, restaurar pelo backup e exigir o SHA v6 antes do próximo.
4. `Unknown` nunca mata. `Survived` ou `Unknown` impede o score combinado 1.0.
5. Reexecutar o baseline completo após a última restauração e conferir os nove
   hashes na cópia e na origem.
6. Produzir receipt v2 sem veredito final e sem alterar v5, contrato, RED,
   testes, L0, produção ou oráculos.

Score combinado exigido: `30 / 30 = 1.0`, com `0` sobreviventes e `0` Unknown.
