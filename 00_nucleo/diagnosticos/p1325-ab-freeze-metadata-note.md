# Correção do metadado temporal do freeze

O primeiro arquivo `p1325-ab-freeze.json` tinha SHA-256
`b5a2e3b5697243780734caf58c38ad78e5f18b5b8fb1fa42c1342c3ecc8f0853`
e um horário digitado pelo autor, `01:01:56+00:00`, que não havia sido medido.
Esse metadado estava errado. Antes de anunciar o freeze ao implementador e
antes de C, foi corrigido para o horário efetivamente retornado por
`date --iso-8601=seconds` na verificação dos hashes:
`2026-09-08T22:01:40-03:00` / `2026-09-09T01:01:40+00:00`.

O hash final anunciado ao root e confirmado como lido antes de C é
`5efd24f3a37df5a42f6746e954922b04dacbec958b2877a025e812790bc5af01`.
Somente o campo `at` mudou. Os bytes dos testes, runner, comparador,
expectativas, manifesto, núcleo, baseline e hash normativo L0 não mudaram.
O revisor havia observado o hash provisório durante a escrita compartilhada;
esta nota explicita a correção para reconciliar seu registro.
