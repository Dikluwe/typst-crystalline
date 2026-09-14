# P1398 — relatório de verificação final

**Veredito:** `PASS_FRAGMENT` — o escape canônico das strings internas de
`Symbol::repr` está `Preserved` nos 16 observáveis registrados. Este veredito
não declara equivalência funcional geral.

**Regime:** protocolo completo de materialização Tekt segregada, executado sem
atestação de isolamento. O verificador trabalhou no workspace compartilhado,
teve escrita limitada a este relatório e ao certificado
`p1398-verification.json` e não alterou L0, implementação, passo, contrato,
oráculo, mutantes ou preseal.

## Proveniência

A verificação ocorreu em `2026-09-14T17:46:25-03:00`, sobre o HEAD
`762e95025a76cb8dec4b58097ca57aa878e05243`, com candidato ainda não
commitado. Antes do veredito, `git diff HEAD --stat` registrava:

```text
00_nucleo/prompts/entities/symbol.md | 70 ++++++++++++++++++++++++++++++------
01_core/src/entities/symbol.rs       | 70 +++++++++++++++++++++++++-----------
2 files changed, 109 insertions(+), 31 deletions(-)
```

Os seis artefatos P1398 ainda não rastreados naquele instante eram contrato,
mutantes, recibo de oráculos, preseal, passo e script do oráculo; a relação
exata consta no certificado.

Entradas e identidades decisivas:

- L0 atual: SHA-256
  `da5a30e34cb48c7cdaf97c6ff2b2e7f4c66b6374b00552841d0655c777577135`,
  com `Hash do Código: 9e803b06`;
- preseal R1: SHA-256
  `c6c3ec09fced5e3c1c585809eebdc91b488467ca270f7fa936e8e34dfef8f53e`,
  estado `SEALED_R1_LINEAGE_METADATA`;
- candidato `symbol.rs`: SHA-256
  `beb7dc4876d9c55414d33a6d2bf0cedd43f2d5a67cb2d421077052b4fc4d0e86`,
  com `@prompt-hash 83833d83`;
- binário candidato: SHA-256
  `c3913ce17e35c8480ed043313ee7ab5a315720d98743b35ecb6a83960059bff7`;
- certificado: SHA-256
  `eaa6bc78b130d1b91186c6b7f73538808794ab252a4e3f2fa722782323ee291f`.

Todos os inputs semânticos protegidos conservam os hashes do preseal R1. As
duas diferenças esperadas são saídas do candidato: o arquivo produtivo partiu
do baseline selado e o binário foi reconstruído a partir dele.

## Gates

O oráculo independente foi executado serialmente com limite virtual de
524.288 KiB e timeout externo de 60 s. Ele aceitou a identidade exata do
binário, comparou bytes UTF-8 e codepoints sem normalização e produziu:

```text
Preserved: 16
Difference: 0
Unknown: 0
ordem direta/repetida/inversa: concordante
exit: 0
```

O recibo `/tmp/p1398-verifier-green.json` tem SHA-256
`a939b3ca009acb9d2e64a836faab2a4c06b956375b91a527d195c70b589f4001`.
A execução durou 5,60 s, atingiu 33.344 KiB de RSS e usou zero swap.

`crystalline-lint .` saiu com código 0 e zero violações. Permanecem warnings
históricos fora do candidato, mas não houve diagnóstico para
`01_core/src/entities/symbol.rs`. O lint durou 2,89 s, atingiu 211.356 KiB de
RSS e usou zero swap. `cargo fmt --check`, `git diff --check` e a validação
sintática dos cinco JSONs de entrada/execução também passaram.

O build final não foi repetido por este papel, conforme a restrição explícita
de contenção de memória. A evidência recebida identifica build release serial
com limite de dados de 2.097.152 KiB, duração de 171,58 s, pico de 1.140.880
KiB de RSS, zero swap e binário com o hash conferido pelo oráculo. O pico ficou
abaixo do limiar de aborto de 1.782.579 KiB.

`cargo test` não foi executado: o lib-test monolítico tem risco conhecido de
estouro de memória. Essa omissão não recebeu crédito de sucesso; o fragmento
observável foi exercitado pelo oráculo CLI serial, limitado e repetido.

## Conclusão

O candidato satisfaz o contrato selado para aspas, apóstrofo, NUL, barra
invertida, nova linha, tab, carriage return, ZWJ, variation selectors,
variantes e ordenação. O resultado é `Preserved` apenas para esse fragmento de
morfologia de `Symbol::repr`, com linhagem e arquitetura válidas e execução
mantida dentro dos limites de memória registrados.
