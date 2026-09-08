# P1316 — GO anterior ao candidato

Revisor `/root/p1316_review`; mesmos limites de capacidades declarados em
`p1316-review-preflight.md`. A/B executado sem atestação de isolamento técnico.

## Evidência

L0 raw SHA-256
`06faafea633ab0bd17a8d2c0862eff0123d6c23618ffff7367d8517e840c6bb1`;
normativo SHA-256
`f429ebf31bd6723cf8a69b7be82f2c223ced005bf367655ef17cadd26026b112`.
Freeze `p1316-ab-freeze.json`, SHA-256
`200816280fc2e1c18934103574c6d33c3d972bdfb576221c4a74d3175d385e5f`.

Auditoria independente reproduzível:
`node 00_nucleo/diagnosticos/p1316-review-audit.cjs`.
Script SHA-256
`455abda2b982284ee50300f34a983b840a2a64fa6a9b92b1771ba59c1eb22d8b`.
Execução `2026-09-08T14:34:28.054Z`, HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado:

```text
 00_nucleo/prompts/compiler/stdlib/loading.md | 124 ++++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs       | 199 +++++++++++++++++++++++++--
 2 files changed, 309 insertions(+), 14 deletions(-)
```

Fonte raw SHA-256
`e75193f2bb56ea6d5dfdf853cb58a3ad30fd10bec960a6efb86ee57ae707f888`.
O prefixo produtivo permanece exatamente igual ao baseline P1315; somente
testes locais novos foram acrescentados. Testes anteriores permanecem
idênticos. Todos os 29 pins do freeze e 28 artefatos P1315 foram conferidos.

Auditoria reconstruiu as 1320 expectativas dos 330 casos: em 69 casos de
origem, primeira linha integral do diagnóstico baseline e restante integral
do vanilla; os outros 261 são preservação literal. Os 290 casos históricos
repetem expressões/cwd e observações do P1315 antes da transformação explícita.
260 expectativas são RED no baseline; nenhuma discrepância na reconstrução.

## Calibração e decisão

As quatro expressões novas com fonte malformada e positional excedente
revelaram uma dívida de precedência: parsing no cristalino, excesso no
vanilla. Os artefatos originais continuam pinados e intactos; R1 substitui
somente esses controles novos por fonte válida. A calibração ocorreu antes
do candidato, com rerun focal e reuso de observações documentado. Não houve
ajuste de expectativas a uma implementação.

O L0 descreve o efeito normativo de ancorar o erro de parsing legado no
primeiro Bytes nessa colisão, preservando a precedência existente. Isto
decorre da obrigação local de origem e não autoriza corrigir excesso nem
alegar paridade do erro resultante. Exige teste local de primeiro versus
segundo positional. A limitação A/B de Args sintético e de `value_span`
distinto de `occurrence.span` está expressa; gates locais cobrem esse nível.

GO para implementação após RED local sob L0 estável, seguido de GREEN,
build, lint/linhagem e A/B normal/repeat/reverse. Classe fluxo contínuo
ADR-0127 mantida. Este GO não é veredito final, selo de refinamento completo
nem declaração de equivalência geral CSV.
