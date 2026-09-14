# P1368 — preflight do observador efetivo S02

## Proveniência

- medição: `2026-09-13T11:59:39-03:00`;
- HEAD: `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- working tree não commitado, com 25 arquivos tracked alterados, 2.129
  inserções e 223 remoções no `git diff HEAD --stat`;
- harness P1357: SHA-256
  `f59cc94163c5be064f0bd9eea950e9c51d405e7cac0803fc8682b6532e9dec1b`;
- oráculos P1357: SHA-256
  `39992f97f74eff4593596a9996d4f5b78ecc7977ddd78e7abd3d9047fec9e604`;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`.

## Medição anterior à decisão

`p1357-harness.py:338` classifica `S02_TOP_LEVEL_SET` como `compile`, mas
`execute_case:367-373` só extrai `Tagged:` para o modo `compile_pdf`.
Consequentemente, o sucesso top-level fica reduzido a exit/stdout/stderr e não
prova o setting efetivo. Já `S02_DOCUMENT_OVER_CLI` e
`S02_CLI_ONLY_CONTROL` usam `compile_pdf` e observam a linha pública do
`pdfinfo`.

## Decisão

Materializar um observador focal independente do runner histórico. Ele
normaliza exatamente uma linha `Tagged:` e exige o vetor `(true,false,true)`.
Isso elimina a lacuna de discriminação sem reescrever nem invalidar os
artefatos selados do P1357. A execução bilateral de PDFs fica para um passo de
medição posterior; P1368 prova somente o contrato do observador em memória.

