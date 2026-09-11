# P1347 — revisão focal final do oráculo R2

Regime: **executado sem atestacao de isolamento**. Esta revisão preserva todos
os artefatos R1, reutiliza sem alteração o corpus e o worker R1 e altera somente
o checker novo `p1347-oracle-checker-r2.py`.

## Evidência pública que motivou a revisão

O verificador executou fora do sandbox o transporte de imagem: observou um único
`PTRACE_EVENT_EXEC` e igualdade entre o SHA-256 da imagem real
`/proc/<pid>/exe`, o memfd retido e o binário pinado
`5205e76bea7e23c830990ea59b1cf72c0bd21984512537ab7e843b879059a9c3`.
Depois da autenticação, o probe terminou com código `101`: o checker R1 havia
ordenado alfabeticamente as chaves da requisição, enquanto o wire P1345 R2
pinado exige `schema`, `fresh_challenge_hex`, `invocation_nonce_hex` e um LF.

## Delta R2

O R2 introduz um serializador literal para essa requisição. Uma validação
não-ptrace revelou e corrigiu a fronteira simétrica da resposta: o checker passa
a exigir a ordem fixa `PROBE_KEYS` e a serialização sem ordenação do wire P1345
R2. Isso não relaxa nenhum predicado; mantém schema fechado, ordem exata, LF,
tipos, desafio, nonce, projeção e hashes.

O teste não-ptrace executou o probe pinado diretamente de memfd selado. Resultado:
exit `0`, stderr vazio, requisição SHA-256
`e0d29c85d810e26b7f2cef758d7bcc07da4f5ae21ebea65310ed9efa83e4d0c1`,
binário SHA-256 `5205e76b…a9c3` e interpretação `Unknown/OPAQUE_PAYLOAD`
somente depois de todas as relações do payload passarem.

Não houve nova execução ptrace no sandbox, nem focal completo do oráculo. A
execução ptrace final pertence ao verificador fora do sandbox. `full=0`; código
produtivo candidato e artefatos `p1347-adversary-*` não foram lidos.

Esta é a última revisão focal permitida. Veredito de autoria:
**`ORACLE_R2_AUTHORED_AWAITING_EXTERNAL_VERIFIER_NOT_SEALED`**.
