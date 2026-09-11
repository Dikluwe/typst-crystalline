# P1345 — relatório adversarial independente R2

## Veredito

`ADVERSARIAL_R2_SURVIVORS_BLOCK_PRESEAL`

O oráculo R2 não pode ser pré-selado. A campanha focal encontrou **13 ataques sobreviventes** entre **122 negativos válidos**: **109** foram corretamente classificados como `Violated`, para score **0.8934426229508197**. Os **4 controles positivos** permaneceram `Preserved`. A campanha foi executada sob o regime `executado sem atestacao de isolamento`.

Não foi feita correção no oráculo, no contrato, no candidato, em L0 ou nos passos. O corpus completo não foi executado (`full_corpus_runs = 0`).

## Escopo e execução

- Replay focal R2: 84 vetores, sendo 80 negativos e 4 controles positivos.
- Resultado herdado: 80/80 negativos `Violated`; 4/4 controles `Preserved`.
- Extensão adversarial R2: 42 negativos novos.
- Total: 126 registros; 122 negativos válidos; 4 controles.
- Execução focal direta: 1 regeneração pela API focal pinada e 2 invocações CLI que chegaram ao focal.
- Temporários: exclusivamente sob `/dev/shm/p1345-r2-*`.
- Candidato: não lido nem executado.
- Corpus completo: nunca executado.

## Sobreviventes e causas públicas

### `INVOCATION_JSON` — 5

- `P1345-R2N09-null-capsule-id-ignored` — `capsule_id: null` foi ignorado por `identity`.
- `P1345-R2N10-float-replacement-ignored` — `replacement: 1.5` foi ignorado por `identity`.
- `P1345-R2N11-reordered-recipe-keys` — a ordem fechada das chaves da receita não foi aplicada.
- `P1345-R2N12-kind-operation-mismatch-source` — `opaque` foi aceito com `identity`.
- `P1345-R2N13-kind-operation-mismatch-probe` — `synthetic-source` foi aceito com `opaque_probe` e chegou a `Unknown`.

Causa pública: a validação aceita campos de receita que a operação não consome, não fecha a ordem da receita e não impõe compatibilidade total entre `input_kind` e `operation`.

### `CLI_AUTHORITY` — 2

- `P1345-R2N16-cli-abbrev-duplicate-path-alias` — abreviação de flag, flag de modo duplicada e alias `./` foram aceitos.
- `P1345-R2N17-cli-duplicate-checker-pin` — pins duplicados e conflitantes usaram o último valor.

Causa pública: a gramática CLI herda abreviações de `argparse` e semântica last-value-wins para opções repetidas; aliases lexicais do mesmo caminho não são recusados.

### `PROBE_SCHEMA` — 4

- `P1345-R2N28-probe-trailing-space` — espaço final no JSON chegou a `Unknown`.
- `P1345-R2N29-probe-leading-space` — espaço inicial no JSON chegou a `Unknown`.
- `P1345-R2N30-probe-escaped-key` — grafia escapada de chave chegou a `Unknown`.
- `P1345-R2N32-probe-negative-zero` — grafia `-0` chegou a `Unknown`.

Causa pública: o valor JSON é validado depois do parse, mas os bytes emitidos não são comparados com uma serialização canônica única. Assim, codificações primitivas alternativas semanticamente equivalentes não são rejeitadas.

### `PROBE_TOCTOU` — 1

- `P1345-R2N31-probe-toctou-swap-restore` — o executável compilado foi substituído depois do hash prévio, o binário adversarial foi executado e os bytes originais foram restaurados antes do hash posterior; o resultado foi `Unknown`.

Causa pública: hashes antes e depois da execução não vinculam os bytes efetivamente executados. O ataque é reproduzível e não destrutivo: opera apenas sobre o binário temporário criado em `/dev/shm`, restaura exatamente os bytes e o modo originais e não toca no probe-fonte pinado.

### `AUTHORITY_ROOT` — 1

- `P1345-R2N41-coordinated-checker-receipt-root` — checker, receipt e root alternativos coordenados foram aceitos quando os três valores supostamente out-of-band também foram substituídos.

Causa pública: a raiz verifica coerência interna do triplo recebido, mas não consegue criar externamente a autoridade que deveria piná-lo. Este sobrevivente depende do atacante controlar todo o triplo out-of-band; é uma limitação de premissa de confiança. Mesmo excluindo-o, os sobreviventes de invocação, CLI, codificação do probe e TOCTOU bloqueiam o pré-selo de forma independente.

## Defesas confirmadas

Os ataques rejeitados cobriram, entre outros: replay integral dos 80 negativos P1344; colisões e literais do lexer; omissão, adição, reordenação e substituição de tokens; marker/owner/anchor/normalização; adulteração individual de tabela, corpus, checker e root; JSON com chaves extras ou duplicadas e tipos `bool`, `null` e `float` em campos estritos; runtime não nulo; resposta/witness injetada; schema, projeção pública, emissor, hashes, challenge, nonce e replay do probe; symlinks e aliases de raiz; pins individuais errados; e tabela alternativa contra o pin externo.

## Proveniência

- Estado medido: working tree não commitado em `HEAD 2f42d64253547734564513a1159ee6b584c1c4b4`.
- Hora da medição focal: `2026-09-11 08:27:33.375819041 -0300` (mtime do relatório JSON).
- Comando: `PYTHONDONTWRITEBYTECODE=1 python3 -B 00_nucleo/diagnosticos/p1345-adversary-runner-r2.py --output 00_nucleo/diagnosticos/p1345-adversary-report-r2.json`.
- O runner terminou com código 1 por desenho, pois sobreviventes bloqueiam o pré-selo.
- Hash do runner: `929cec0c1f7581bdc9ea918402e42ebbea4b32682dc515a071f5846a9f71285d`.
- Hash do relatório JSON: `3130b8d6a2d5aab7c27c2ede92edfa955e390acafe1a6f78e3c2d7d2f6a0afdf`.
- A árvore já continha alterações alheias ao papel adversarial; o recibo registra fingerprints de estado. Os únicos artefatos autorais R2 deste papel são runner, relatório JSON, este relatório MD e o recibo R2.

## Limites e decisão

O teste é focal e não constitui validação do corpus completo. `Unknown` conta como sobrevivência de negativo. Ataques de ordem de chaves e de alias lexical expressam a gramática fechada solicitada; ainda que fossem reclassificados como política não normativa, os defeitos independentes de compatibilidade de operação, CLI duplicada, emissão canônica e TOCTOU permanecem.

Decisão adversarial: **bloquear o pré-selo e devolver as causas públicas ao autor do oráculo, sem corrigi-las neste papel**.
