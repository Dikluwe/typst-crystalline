# P1289 v3 — recibo dos oráculos independentes do Testador A

**Estado:** `ORACLES_A_V3_SEALED_WITH_TEST_OWNER`  
**Contrato:** `C-P1289-FLOAT-IS-INFINITE-v3`  
**Regime:** Tekt A/B  
**Executor:** `/root/testador_a_p1289`  
**Snapshot:** `2026-08-31T11:47:08-03:00`  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`

## Invalidação dos selos anteriores

O selo v1 foi invalidado pelos carriers compostos indisponíveis. O selo v2,
cujo manifesto tinha SHA-256
`e3c45265597b5581985c0f0986807adededa369de1938d6e8a49517d6a9d2bdd`,
foi invalidado exclusivamente porque o integration test L4 não tinha owner
L0 individualizado e produzia V1. Nenhum hash protegido v1/v2 autoriza a fase
B ou a verificação v3.

Os 11 observáveis, carriers v2, resultados, diagnósticos, política de
`Unknown` e cinco mutações permanecem byte-semanticamente iguais. A única
mudança causal v3 é ownership e lineage do teste protegido.

## Autoridade e isolamento

O Testador A recebeu passo, manifesto/recibos v3, seis L0s e binários
black-box. Não leu consumers de produção, patch candidato nem executou o
binário candidato atual. Escreveu somente os cinco artefatos da allowlist.

O checkout é compartilhado; a segregação é atestada somente por capacidade,
entradas, ordem e hashes, sem isolamento forte do host.

## Entradas congeladas v3

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1289.md` | `156f223ce214eb13055dfd3ad0f7d9d45c25ac24e6c1f7379a9412c4077339fb` |
| `00_nucleo/diagnosticos/p1289-manifest.json` | `1bd7c90a45f9cc2a2301eaab89405915d5dfc7d74627508325ca0124c50167ca` |
| `00_nucleo/diagnosticos/p1289-contract-receipt.md` | `d0da116e87fa131cf025660181adb6079cac99efbafd08f97780246ee3e17ff3` |
| `00_nucleo/diagnosticos/p1289-vanilla-measurement-receipt.md` | `c35a5fc6b710386b103077d97326df8c2d45ca46aea8c068fb05539bceb1f741` |
| `00_nucleo/prompts/compiler/stdlib/foundations/float.md` | `b05da320cddd30a455dd60e027bcd8639261fff9be16b39dc4dd82610ea2b242` |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `90155e0c0ed88d36e1c9d198782387a99ea7ad1813cbfd7402b8eca1176e4692` |
| `00_nucleo/prompts/compiler/eval/call_dispatch.md` | `66e9a245c7916e261a3c3970d9dd57f52d4a2626afe2afe13ae0a768b2933a6c` |
| `00_nucleo/prompts/compiler/stdlib/foundations.md` | `8df1e1b9317b6eeb11c691ed2126bd76901c8e2c207278172ce2e79984b0bc34` |
| `00_nucleo/prompts/compiler/stdlib/_comum.md` | `29aee6d1d5413c0023e6feeb3151bc4642705a556313cdf8f6bac86d92291d7b` |
| `00_nucleo/prompts/wiring/tests/p1289_float_is_infinite.md` | `55694b8c40b3366d1e5c2d850b0033091b39a11eab1fb70403d1e1dc484d0899` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

## Artefatos protegidos v3

| Artefato | SHA-256 |
|---|---|
| `lab/surface-inventory/run_p1289_oracles.py` | `e7a3d802c720ef2537ab2af8d6d634b81ffb474e7961a65bd2f05e83dc211390` |
| `lab/surface-inventory/p1289-oracle-baseline.json` | `7c8337761109db2ffa30fcb112c97952caa632589eaecbf01757092bd8cfc44d` |
| `04_wiring/tests/p1289_float_is_infinite.rs` | `56c5093227465623fc9b27dd9dd1a4ee54ed6e7c0f78e0b41c0ab061fcce68ea` |

## Lineage e ownership do teste

Foi adicionado no topo do único consumer L4:

```text
//! @prompt 00_nucleo/prompts/wiring/tests/p1289_float_is_infinite.md
//! @prompt-hash 58174302
```

O comando focal sem escrita

```text
crystalline-lint --checks v5 --fix-hashes --dry-run . | rg '04_wiring/tests/p1289_float_is_infinite.rs'
```

indicou exatamente:

```text
Would fix ./04_wiring/tests/p1289_float_is_infinite.rs prompt=00_nucleo/prompts/wiring/tests/p1289_float_is_infinite.md old=00000000 hash-a=58174302 hash-b=9cb55bde
```

Aplicou-se manualmente somente o `hash-a` canônico `58174302` no consumer;
nenhum fix global foi executado e o L0 permaneceu imutável.

Gate focal:

```text
crystalline-lint --checks v1,v5,v15,v26 .
```

Exit `0`; nenhum achado menciona o teste ou seu L0. Permanecem warnings V5
alheios já existentes na working tree, sem bloquear este gate focal.

## Oráculo positivo v3

```text
python3 lab/surface-inventory/run_p1289_oracles.py --candidate /usr/local/bin/typst --summary-only --pretty
```

- janela: `2026-08-31T11:45:54-03:00`–`2026-08-31T11:46:06-03:00`;
- exit: `0`;
- SHA-256 da saída: `c5f43abd0c109fed22c9fefeaf8d58701b3489fb7338e92c0337634a90fb0fa5`;
- 11 `Preserved`, 0 `Unknown`, 0 `Violated`;
- forward/reverse determinístico;
- veredito do fragmento: `Preserved`.

## Gate discriminatório

```text
python3 lab/surface-inventory/run_p1289_oracles.py --self-test
```

- exit: `0`;
- SHA-256 da saída: `893a2057ef202f6cf310de108790b7bbb739445714cb575cab3a7f3cfb95993c`;
- mutações mortas: 5 de 5;
- `mutation_score = 1.0`;
- `Unknown`: zero.

## Compilação e formato focais

- `cargo test --workspace --test p1289_float_is_infinite --no-run` → exit `0`;
- `rustfmt --check 04_wiring/tests/p1289_float_is_infinite.rs` → exit `0`.

O binário candidato atual não foi executado; GREEN continua reservado ao
Verificador.

## Proveniência da árvore

- `git status --short | sha256sum` →
  `616e04498d01726a71b1900ca01359439ac5be7468e67af42c6f887eb282303c`;
- `git diff HEAD --stat | sha256sum` →
  `2b83af170293d57c66e51dca044c4588cae6b4c9291e33c06da3a0ac89032d44`.

Os hashes individuais acima identificam os artefatos untracked não cobertos
pelo segundo comando.

**Veredito deste recibo:** v2 invalidado; oráculos A v3 e owner 1:1 do teste
selados, `mutation_score = 1.0`, sem veredito sobre o candidato.
