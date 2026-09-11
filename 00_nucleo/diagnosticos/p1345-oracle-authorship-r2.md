# P1345 — autoria Oracle R2, primeira revisão focal

## Limite da revisão

Regime: `executado sem atestacao de isolamento`. Esta revisão responde somente às três classes públicas do adversário R1:

1. `INVOCATION_SCHEMA_AND_RUNTIME_BOUNDARY_NOT_ENFORCED`;
2. `OPAQUE_PRIMITIVE_SCHEMA_AND_PROJECTION_NOT_CLOSED`;
3. `CHECKER_AND_ROOT_TRUST_DELIVERY_NOT_ENFORCED_BY_EXECUTABLE_INTERFACE`.

Não houve leitura nem execução de candidato produtivo. Nenhum full corpus foi executado. Contrato FINAL R2, tabela canónica R1, fixture positiva R1, probe Rust R1 e todos os negativos adversariais R1 foram preservados.

## Delta autoral

- `p1345-source-verifier-r2.py` — preserva por pin toda a verificação lexical R1 e rejeita a raiz de autoridade quando o próprio caminho ou qualquer ancestral é symlink.
- `p1345-oracle-corpus-r2.json` — compõe os 40 vetores históricos e os 44 vetores novos do adversário pinado, fechando cardinalidade, ordem, escopo e tipos.
- `p1345-oracle-checker-r2.py` — valida chaves/enum/bundles/tipos na fronteira antes do julgamento; rejeita runtime/candidato e canais de resposta; exige schema, projeção pública e inteiros JSON exatos do probe; recomputa nonce, resposta e freshness; valida emitter/binário; e exige hashes esperados de checker e raiz entregues fora da banda.

O checker não aceita a própria cópia como raiz de confiança. A função pública de validação requer quatro argumentos: caminho e hash esperado do receipt, hash esperado do checker e raiz autoral esperada. A CLI exige os mesmos valores; caminhos localizam bytes, mas não conferem autoridade.

## Pins autorais

- source verifier R2: `9434ca3f2191cade3207d236a553b76045a016a2770da243251dc5d2d9527250`
- corpus R2: `3003e7577cbf99bdc50d8f14b44897e4556a8fd18f439ec07a1de96e72c94394`
- checker R2: `8e2ff788526e0b04d3bce6624c4a1864c6f32a36fc3a35088b1bbf6118d1c6a0`
- fixture R1 preservada: `8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12`
- tabela R1 preservada: `ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f`
- probe R1 preservado: `863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc`

## Protocolo focal reproduzível

A execução autorizada usa `python3 -B`, corpus R2 canónico e os hashes esperados de checker, receipt e authoring root entregues como argumentos. O checker regenera, na ordem pinada, os 40 ataques históricos e os 44 novos vetores (80 negativos válidos e 4 controles positivos), usando somente árvores sintéticas em `/dev/shm`. O critério focal é 80/80 `Violated`, 4/4 controles `Preserved`, mutation score `1.0`, zero sobreviventes e `full_corpus_runs = 0`.

Proveniência da medição final: `2026-09-11T08:16:20-03:00`, HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado. `git diff HEAD --stat` para a allowlist foi vazio porque os cinco artefatos R2 eram novos; `git status --short` mostrou exatamente `p1345-source-verifier-r2.py`, `p1345-oracle-corpus-r2.json`, `p1345-oracle-checker-r2.py`, `p1345-oracle-authorship-r2.md` e `p1345-oracle-authorship-receipt-r2.json` como untracked. A execução final mediu 84 registros, 80 negativos válidos corretamente `Violated`, 4 controles positivos sem regressão, zero sobreviventes, score `1.0` e zero full runs.

Este artefato e seu receipt registram autoria focal, não verificação independente, preseal, seal, equivalência geral nem veredito sobre candidato. Veredito máximo: `FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED`.
