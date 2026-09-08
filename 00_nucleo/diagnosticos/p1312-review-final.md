# P1312 — veredito independente

**PASS_SCOPED**, em regime A/B executado sem atestação de isolamento técnico.
O veredito é exclusivamente do diagnóstico inválido de read definido no L0.

O checker independente `p1312-review-check.py` executou no host em
`2026-09-08T01:52:56.219358+00:00`, sobre HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado.
Recibo `p1312-review-evidence.json` contém diff/stat integral, hashes das
entradas e do checker, candidato e contagens recalculadas. SHA-256 do recibo:
`2ff6ada13314807394ba33707692878e280daef28a403e4558250e9e7ed5a773`.
Veredito detalhado: `p1312-verification.json`.

Recalculei 840 comparações literais em três ordens, sem Unknown ou instabilidade;
conservei a origem prépatch das 280 expectativas, inclusive Symbol normativo.
RED de três assertions e um controle passou a quatro testes verdes. Replays
P1310/P1311 só mudam os quatro envelopes de read previamente declarados em
cada corpus; todos coincidem com vanilla literal prépatch. P1308 preserva
1.978 envelopes contra P1311 e acrescenta quatro deltas esperados, além dos
32 anteriores, todos paritários. Os oráculos antigos permanecem intactos.

A inspeção do helper e a reconstrução por SHA demonstram que somente helper
privado, chamada de native_read, testes P1312 e header foram adicionados ao
source baseline. CSV e os decoders P1310 ficam byte-idênticos. L0 normativo
congelado permanece intacto. Os 8.083 arquivos inventariados e 159 artefatos
anteriores foram conferidos; apenas loading.rs/loading.md mudaram em relação
ao baseline P1312. Não houve stage ou commit.

Build, fmt/check, diff/check e linhagem passaram. Lint: zero erros, 240 warnings
e 1.138 infos. A primeira suíte workspace falhou em dois testes PDF com
unclosed delimiter; mantive o recibo original e confirmei que a focal (três
testes) e a repetição completa (6.633 passaram, zero falhas, três ignorados)
usaram exatamente o mesmo diff produtivo e candidato. A causa da falha
transitória não está demonstrada e nenhum conserto dela é atribuído a P1312.

CSV/DataSource/Bytes, coerção Symbol, demais diagnósticos e limitações de paths
importados permanecem fora do fechamento. Não há score de mutação de produto,
selo completo de refinamento, atestação de isolamento ou equivalência geral.
