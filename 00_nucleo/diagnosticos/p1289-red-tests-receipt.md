# P1289 v3 — recibo RED dos testes A contra o pré-candidato

**Estado:** `RED_V3_CONFIRMED_WITH_TEST_OWNER`  
**Contrato:** `C-P1289-FLOAT-IS-INFINITE-v3`  
**Manifesto SHA-256:** `1bd7c90a45f9cc2a2301eaab89405915d5dfc7d74627508325ca0124c50167ca`  
**Runner v3 SHA-256:** `e7a3d802c720ef2537ab2af8d6d634b81ffb474e7961a65bd2f05e83dc211390`  
**Baseline v3 SHA-256:** `7c8337761109db2ffa30fcb112c97952caa632589eaecbf01757092bd8cfc44d`  
**Integration test v3 SHA-256:** `56c5093227465623fc9b27dd9dd1a4ee54ed6e7c0f78e0b41c0ab061fcce68ea`

## Invalidação causal v2

O RED v2 foi invalidado exclusivamente pela ausência de owner do integration
test. O contrato observável não mudou. Este recibo repete o RED depois de o
consumer receber lineage única para o novo L0 L4; os hashes v2 não permanecem
válidos.

## Pré-candidato congelado

| Entrada | Identidade |
|---|---|
| caminho | `/tmp/p1289-target-pre/debug/typst` |
| SHA-256 | `e7c81e3a6c6f1933db95c2249286fca19f3b5991eb8bc392f49ea659cec0bd78` |
| HEAD | `53d21c5a602f4045a769a0ab0c935baa5ecd3b88` |
| janela | `2026-08-31T11:46:10-03:00`–`2026-08-31T11:46:28-03:00` |

Este foi o único binário cristalino executado pelo Testador A v3. O candidato
atual não foi executado.

## Execução RED v3

```text
python3 lab/surface-inventory/run_p1289_oracles.py --candidate /tmp/p1289-target-pre/debug/typst --summary-only --pretty
```

- exit observado: `1` esperado;
- SHA-256 da saída: `e42b8fc86bd66092481cdedee2951208aebae443a2543bf79f383e2feee3546b`;
- forward/reverse determinístico;
- `Preserved`: 1;
- `Violated`: 10;
- `Unknown`: 0;
- veredito: `Violated`.

`bound-value-absent` é o único caso preservado. Os demais rejeitam a ausência
da superfície/chamada sob teste, enquanto os carriers v2 continuam disponíveis
bilateralmente. Exit `1` é RED contratual, não indisponibilidade.

## Gate causal

O RED usou os bytes v3 do manifesto, seis L0s, baseline, runner e teste. O
consumer L4 possui exclusivamente o owner
`00_nucleo/prompts/wiring/tests/p1289_float_is_infinite.md` com
`@prompt-hash 58174302`. O Testador A não leu consumers produtivos, não alterou
candidato e não executou o binário candidato atual.

Qualquer drift dos três hashes protegidos invalida este recibo.

**Veredito deste recibo:** RED v2 invalidado; RED v3 reproduzível confirmado
com ownership do teste e zero `Unknown`, sem veredito final de GREEN.
