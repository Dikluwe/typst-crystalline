# P1342 — ataque adversarial focal R3/R4

## Veredito

**BLOCKER_NOT_SEALED.** O replay focal oficial de Y01–Y06 obteve o resultado esperado, mas cinco negativos adicionais válidos foram aceitos pelo checker composto R4. O score adversarial é **12/17 = 0,7058823529411765**, estável em `normal`, `repeat` e `reverse`. Não há base para pré-selo.

Regime: **executado sem atestacao de isolamento**. Nenhum candidato produtivo foi aberto. O corpus completo não foi executado; somente Y01–Y06 e controles focais mínimos foram chamados.

## Resultado medido antes da decisão

| Caso | Ataque | Esperado | normal | repeat | reverse |
|---|---|---:|---:|---:|---:|
| Y01 | candidato alterado autorizado | Preserved | Preserved | Preserved | Preserved |
| Y02 | baseline sem hook | Violated | Violated | Violated | Violated |
| Y03 | comparador direto live/baseline | Violated | Violated | Violated | Violated |
| Y04 | evidência dentro do DTO | Violated | Violated | Violated | Violated |
| Y05 | path/hash externo adulterado | Violated | Violated | Violated | Violated |
| Y06 | flag de mudança fora da boundary | Violated | Violated | Violated | Violated |
| Z01 | live/baseline via alias + decoy | Violated | **Preserved** | **Preserved** | **Preserved** |
| Z02 | baseline igual com hooks fabricados | Violated | **Preserved** | **Preserved** | **Preserved** |
| Z03 | `candidate_file_sha256` alterado | Violated | Violated | Violated | Violated |
| Z04 | path externo trocado | Violated | Violated | Violated | Violated |
| Z05 | hash externo trocado | Violated | Violated | Violated | Violated |
| Z06 | Rust inválido fora de cfg autodeclarado | Violated | **Preserved** | **Preserved** | **Preserved** |
| Z07 | boundary vazia sobre fonte alterada | Violated | **Preserved** | **Preserved** | **Preserved** |
| Z08 | preseal pinando R2 | Violated | Violated | Violated | Violated |
| Z09 | bytes/hash do candidato divergentes | Violated | Violated | Violated | Violated |
| Z10 | linha Rust-aware ausente | Violated | Violated | Violated | Violated |
| Z11 | coverage ausente | Violated | Violated | Violated | Violated |
| Z12 | hashes de âncora/símbolo fabricados | Violated | **Preserved** | **Preserved** | **Preserved** |

Replay oficial focal: 5/5 negativos rejeitados, score 1,0. Ataque focal ampliado: 12/17 negativos rejeitados, cinco sobreviventes, zero instabilidade.

## Blockers reproduzíveis

1. **B01 — bypass do comparador proibido.** Z01 mantém uma comparação direta `live_candidate_sha256 == candidate_file_sha256` como decoy, atribui `baseline_file_sha256` a um alias e usa o alias no comparador efetivo. A busca AST só reconhece nomes diretos no mesmo `Compare`; o checker aceita `live==baseline` indireto.
2. **B02 — hooks e coverage autodeclarados.** Z02 substitui os bytes do candidato pelos bytes históricos, recalcula hash, árvore e recibo, mas conserva as declarações de hooks/coverage. A fonte sem hook é aceita porque as alegações não são derivadas dos bytes.
3. **B03 — parse Rust e boundary autodeclarados.** Z06 injeta Rust sintaticamente inválido e não guardado por cfg, recalcula os hashes e mantém `rust_parse_ok`, `outside_allowed_boundaries_equal` e `outside_region_unchanged` como `true`. O checker não faz parse nem diff real.
4. **B04 — boundary desconectada do diff.** Z07 reduz a boundary do arquivo alterado para `[0,0]`. O checker valida apenas os limites numéricos, não que os bytes alterados estejam dentro do intervalo.
5. **B05 — hashes de âncora e símbolo não vinculados.** Z12 troca `candidate_anchor_snippet_sha256` e `candidate_symbol_sha256` por valores arbitrários e ressela a evidência. Ambos são aceitos sem derivação da fonte candidata.

Os controles de hash do candidato, path/hash externo, pin R2 no preseal, divergência bytes/hash, ausência de linha Rust-aware e ausência de coverage foram corretamente rejeitados. Isso localiza a fraqueza na substância da evidência, não no envelope fechado.

### Re-pin do canal externo nos sobreviventes

| Sobrevivente | `candidate_evidence_path` | `candidate_evidence_sha256` | `channel` | Outro pin recalculado |
|---|---|---|---|---|
| Z01 | inalterado | inalterado | inalterado | somente `checker_fixture.sha256` |
| Z02 | inalterado | recalculado após resselo | inalterado | árvore e recibo da evidência |
| Z06 | inalterado | recalculado após resselo | inalterado | arquivo, linhas, árvore e recibo |
| Z07 | inalterado | recalculado após resselo | inalterado | recibo da evidência |
| Z12 | inalterado | recalculado após resselo | inalterado | recibo da evidência |

Portanto, nenhum sobrevivente troca o path ou o nome do canal externo. Z01 não re-pina a evidência candidata; Z02/Z06/Z07/Z12 re-pinham legitimamente o hash do novo corpo ressellado, que o checker aceita porque a fraqueza está na validação substantiva desse corpo.

## Reprodução

Replay focal oficial, sem executar o corpus completo:

```text
python3 -B 00_nucleo/diagnosticos/p1342-oracle-checker-r4.py --contract 00_nucleo/diagnosticos/p1342-contract-spec-r3.json --binding 00_nucleo/diagnosticos/p1342-contract-binding-r3.json --corpus 00_nucleo/diagnosticos/p1342-oracle-corpus-r4.json --focus
```

Ataques adicionais, também somente focais:

```text
python3 -B 00_nucleo/diagnosticos/p1342-adversary-runner-r3.py
```

O segundo comando retorna código 1 por desenho quando há sobrevivente.

## Proveniência e hashes

Medição em `2026-09-10T19:58:20-03:00`, commit `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado. O estado exato dos insumos consumidos é definido pelos hashes:

| Artefato | SHA-256 |
|---|---|
| `p1342-contract-spec-r3.json` | `07689c204f8741cdcfcc23bef9fa4af969dba6d991caaa93afaf117eb7ac73e7` |
| `p1342-contract-spec-r3.md` | `1e798f3afcab77d236d82a4de30b3999e894aaf40838f154b17b80f35748c409` |
| `p1342-contract-binding-r3.json` | `8630a376350d374f854345c37282ac8fbb657f2f9a8506b7de47bda7b8276bf5` |
| `p1342-contract-receipt-r3.json` | `a14707ff9be26dab7cf8a2f0696c9746baa7cda15bc3a7a264fa3331d1327800` |
| `p1342-oracle-checker-r4.py` | `2fb44d8398d85fe174231b6dbbaf59c7c350f1de6cd6ab3f974092c1d85535d8` |
| `p1342-oracle-corpus-r4.json` | `a5d163b4d036dd24257af4aab3306c2f01a340437c6474066ca0990844548619` |
| `p1342-oracle-authorship-r4.md` | `aecbb9da5dfe9328e89eb4f5ac10e4e1c46a605e1ace93e57558cdb697d7361d` |
| `p1342-oracle-authorship-receipt-r4.json` | `377b74631971575c173f184a22ac94633cd46bc50fe51f1334202c791dd6614d` |
| `p1342-adversary-runner-r3.py` | `2251ec6f3d338329473a46d49910731ec851735b8b0da398a818e80895c3747a` |

Este parecer não reivindica isolamento forte, lifecycle/profile, NT01–NT06 nem política terminal. Nenhum artefato julgado foi corrigido ou alterado.
