# P1301r2 — reparo test-only do candidato 1

## Veredito

**Reparo test-only GREEN.** O helper negativo P1300 foi alinhado com a
obrigação semântica P1301 já congelada. Depois do reparo, os filtros `p1300`
e `p1301` passaram integralmente no mesmo ambiente efémero em `/dev/shm`.

Regime: protocolo Tekt completo, continuação limitada do papel
`P5-r2` / testador A/B. **executado sem atestação de isolamento técnico**.

Não foram lidos `field_access.rs`, o L0 produtivo, implementação candidata,
oracle, mutantes, runner, artefactos P1301 v1, `materialization/` ou
`context/`. Foram reabertos somente o prompt test-only, contrato, selo,
recibo RED anterior e consumer test-only autorizados pelo root.

## Falha predecessora e causa

A medição predecessora comunicada pelo root para
`cargo test -p typst-core p1300` foi: `10` testes executados, `6 passed` e
`4 failed`. Esse número não foi remedido antes da edição; sua proveniência é
o relato causal do root que abriu este reparo.

As quatro falhas eram restritas à expectativa test-only obsoleta do helper
P1300 para acessos `under_std`: ele ainda exigia
`module 'std' does not contain field "<field>"`, coluna `0` e span total.
A secção P1301 do próprio L0 test-only já exige
``module `global` does not contain `<field>` ``, hints vazios e span somente
no identificador à direita de `std.`.

O reparo alterou somente essas expectativas:

- para `under_std`, mensagem `global` exata;
- coluna inicial `4` e bytes `4..expression.len()`;
- para bare, mensagem, coluna `0` e span total permanecem inalterados;
- cardinalidade, hints, quatro perfis, seis negativos e todos os controles
  permanecem inalterados.

Logo, a obrigação semântica congelada não mudou; foi removida uma
contradição interna entre teste P1300 antigo e a retificação P1301 vigente.

## Entradas e hashes

- Contrato r2, inalterado:
  `69e852d38636afd0ce996f10a64cbf0e6dc0be12dca453a65ee3e0d515c88fad`.
- Selo r2, inalterado:
  `760846d4be24e6422b3284ae0853bc7647114cdfa25a92909207b1b10de9dcca`.
- Recibo RED P5-r2, inalterado:
  `4a34b2006daebea10ee250ef6c249acc2ad06b5ed36b67ac965a48f87f7c6913`.
- Prompt test-only recebido: full SHA-256
  `5ca2ad1e4bcf3f6fe30909be21f42bb2a2a5939a6efcbdfc136dba5b44e7a8d1`;
  SHA-256 semântico normalizado
  `bf0d711419ebe33ce7bc7b54c6bd187798ef94c64747e6a50a0aa5674905924a`,
  exatamente a identidade congelada r2.
- `01_core/src/compiler/eval/tests.rs` antes deste reparo:
  `e32228e6cf9929f42ae4664c8c1bfb70bc69d7e126c809f376ca321789fd1d67`.
- `01_core/src/compiler/eval/tests.rs` depois deste reparo e dos gates:
  `5437bd761f48bef79b2eedd5c2e920bcba310e41c0e85346afaf6db6a2e68af9`.

`rustfmt` foi executado somente em
`01_core/src/compiler/eval/tests.rs`. `git diff --check` nesse ficheiro
terminou com exit `0` e sem output.

## Resselo de lineage prematuro

O consumer RED originalmente congelado por P5-r2 tinha SHA-256
`6f2a17e97ae812728d8829ff3b69776970cfa02c09efe5c8e7e6720a82e429c0`
e header `@prompt-hash ac42f559`. O primeiro resselo posterior mudou somente
esse header para `@prompt-hash 4afb0873`, produzindo o hash de entrada deste
reparo `e32228e6…`; o prompt passou a `Hash do Código: 343763db`, mantendo a
identidade semântica `bf0d7114…`.

Esse primeiro resselo de lineage dos testes foi prematuro: o reparo test-only
mudou agora o conteúdo do consumer, portanto aqueles valores derivados já não
podem autenticar o ficheiro final. Ele precisa ser **supersedido** por um novo
resselo de lineage executado pelo root com o fluxo dry-run/aplicação/prova
previsto no selo. Este papel não alterou o header, o `Hash do Código` nem o L0.

## Gates

Houve uma tentativa preparatória inválida entre
`2026-09-03T22:32:07,362996877-03:00` e
`2026-09-03T22:34:02,471349535-03:00`: ambos os comandos terminaram com
exit `101` antes de executar testes devido a `E0308` no novo assertion
test-only (`Option<(u32,u32)>` versus coluna `usize`). A coluna foi convertida
explicitamente para `u32`. Essa tentativa não foi classificada como gate
funcional nem GREEN.

Execução final válida, numa única sessão, de
`2026-09-03T22:34:38,838036700-03:00` a
`2026-09-03T22:35:53,081506872-03:00`:

```bash
bash -lc 'date --iso-8601=ns; mkdir -p /dev/shm/p1301r2-repair1-tmp /dev/shm/p1301r2-repair1-target; export TMPDIR=/dev/shm/p1301r2-repair1-tmp; export CARGO_TARGET_DIR=/dev/shm/p1301r2-repair1-target; cargo test -p typst-core p1300 -- --nocapture; rc1300=$?; cargo test -p typst-core p1301 -- --nocapture; rc1301=$?; date --iso-8601=ns; printf "P1300_EXIT=%s\nP1301_EXIT=%s\n" "$rc1300" "$rc1301"; test "$rc1300" -eq 0 -a "$rc1301" -eq 0'
```

Resultados exatos:

- `p1300`: `10 passed`, `0 failed`, `0 ignored`, `0 measured`,
  `5421 filtered out`, exit `0`;
- `p1301`: `4 passed`, `0 failed`, `0 ignored`, `0 measured`,
  `5427 filtered out`, exit `0`;
- comando agregado: exit `0`.

## Proveniência do estado

`HEAD` durante os gates:

```text
1f082370e59939de7b57992e137a9f74bfb6758f
```

Working tree não commitada. `git diff HEAD --stat`, recolhido imediatamente
após os gates e antes de criar este recibo:

```text
 00_nucleo/prompts/compiler/eval.md                 |  39 ++-
 .../prompts/compiler/eval/bindings/field_access.md |  74 ++++-
 00_nucleo/prompts/compiler/eval/tests.md           |  90 ++++-
 00_nucleo/prompts/compiler/stdlib/color.md         |  46 ++-
 01_core/src/compiler/eval/bindings/field_access.rs |   9 +-
 01_core/src/compiler/eval/mod.rs                   |  14 +-
 01_core/src/compiler/eval/tests.rs                 | 364 ++++++++++++++++++++-
 01_core/src/compiler/stdlib/color.rs               |   6 +-
 8 files changed, 618 insertions(+), 24 deletions(-)
```

O stat inclui mudanças do utilizador e da implementação fora da capacidade
deste papel. A única escrita produtiva deste reparo foi no helper P1300 de
`tests.rs`; este recibo é o segundo e último caminho criado por P5-r2.

## Limite

Este recibo demonstra GREEN apenas dos filtros internos P1300/P1301 e a
preservação da obrigação semântica r2 no consumer test-only. Não substitui
a matriz bilateral CLI, o resselo de lineage, lint arquitetural ou o
certificado final do verificador independente.
