# P1361 — preflight do formatter de duração

## Proveniência

- instante: `2026-09-13T00:48:29-03:00`;
- HEAD: `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- working tree: não commitado; o estado completo é o `git diff HEAD --stat`
  registrado na execução do passo;
- vanila ratificado: commit
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`, binário
  `/usr/local/bin/typst` SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

## Medição antes da decisão

`lab/typst-original/crates/typst-utils/src/duration.rs:7-68` contém o formatter
puro usado pelo estado de `watch` em
`lab/typst-original/crates/typst-cli/src/watch.rs:151-162`. O formatter decompõe
dias, horas e minutos, omite unidades inferiores quando há dias ou horas e
seleciona segundos, milissegundos, microssegundos ou nanossegundos nas
fronteiras exatas. A fonte não lê relógio; recebe `std::time::Duration`.

O teste autoritativo `duration::tests::test_format_duration` foi executado com
`CARGO_BUILD_JOBS=1`, `timeout 120s` e streams redirecionados. Resultado: um
teste executado, um aprovado, 16 filtrados, 0,00 s. Recibos:

- `/tmp/p1361-vanila-duration.stdout`: 163 bytes, SHA-256
  `b8bce0d219546db4139dfbeeae025e5d3a2f8bffb07a16288172b737b148dc0a`;
- `/tmp/p1361-vanila-duration.stderr`: 370 bytes, SHA-256
  `118c1c4b456ebaaacc38842cc2bb6e184790f936baf2d4c1a17dafa909cd4cb1`.

Revisão adversarial anterior ao código: o teste autoritativo contém 17 casos e
não inclui zero. A leitura do ramo `nanos == 0` em `duration.rs:42-46` mostra
que `Duration::ZERO` usa segundos e produz `0 s`; a hipótese inicial `0 ns`
foi descartada. Também se exclui como equivalente a mutação isolada `>= 5 min`
para `> 5 min` no ponto inteiro de 300 s. O controle discriminatório usa
300.006 s contra um limiar deslocado para 300.010 s.

No cristalino, `rg` não encontra formatter de duração em L2 nem símbolo
comparável; P1357 conservou `S11_EXACT_BOUNDARY_VECTOR` como
`Unknown(PARSER_UNSUPPORTED)`. Há lógica privada de arredondamento de repr em
L1, mas importá-la inverteria ownership: apresentação pública pertence a L2.

## Decisão derivada

Criar um formatter puro e público em L2 com o mesmo vetor observável do vanila,
sem ligar o relógio, o scheduler ou o loop de `watch`. O adaptador recebe
valores `Duration` congelados e devolve somente strings. A eventual emissão de
status temporal pelo processo de `watch` permanece fora deste passo porque é
mudança distinta de comportamento por defeito e exigiria observação segura
própria.

## Segurança de execução

É proibido iniciar `watch` ou `w`. Somente testes unitários exatos do formatter
podem ser executados, um por vez, com `CARGO_BUILD_JOBS=1`, `timeout <= 120s` e
stdout/stderr redirecionados. Nenhuma suíte ampla ou processo de duração aberta
participa da aceitação.
