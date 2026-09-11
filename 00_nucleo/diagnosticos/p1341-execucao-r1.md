# P1341 — execução R1 e condição de reentrada

## Resultado

**NÃO FECHADO.** O candidato R1 compilou e deixou o teste focal GREEN, mas o
verificador independente o rejeitou porque o resultado ainda pode ser
satisfeito por uma projeção construída depois da execução. Portanto esse
GREEN não recebe crédito de binding, não fecha P1341 e não fecha o restante de
P1340.

Regime: **executado sem atestacao de isolamento**.

## Proveniência da medição

- UTC: `2026-09-10T19:03:15Z`.
- HEAD: `2f42d64253547734564513a1159ee6b584c1c4b4`.
- Estado: working tree não commitada; `git diff HEAD --stat` registrou
  `66 files changed, 10669 insertions(+), 825 deletions(-)`. O diff contém a
  cadeia P1339/P1340 herdada e não é atribuído integralmente a P1341.
- Teste focal executado em RAM:
  `CARGO_TARGET_DIR=/dev/shm/typst-p1341-green RUSTFLAGS='--cfg p1339_observation' cargo test -q -p typst-infra p1341_real_closed_ledger_normal_repeat_reverse -- --nocapture`.
  Resultado observado: 1 passou, 0 falhou, 924 filtrados.
- Regressão terminal P1340 executada no mesmo target/cfg:
  `cargo test -q -p typst-infra p1340_terminal_r2_real_binding_default_repeat_reverse -- --nocapture`.
  Resultado observado: 1 passou, 0 falhou, 924 filtrados.
- Build normal: `CARGO_TARGET_DIR=/dev/shm/typst-p1341-normal cargo build -q`;
  exit 0.
- `git diff --check` e
  `crystalline-lint --checks v5,v15,v26 --fail-on warning .`: exit 0,
  zero violações.

Os números acima descrevem somente os comandos e o estado indicados. Não são
prova de causalidade do ledger.

## Cadeia independente

- RED independente:
  `p1341-implementation-tests.rs`, SHA-256
  `a9a37bcc1d3b68138500100a051f3565f1cd71ea0ba5a7659f6a8a66923af5d0`.
- Pré-selo R2, emitido sem leitura do candidato e substituindo o R1 apenas
  quanto aos pins L0:
  `p1341-verifier-preseal-r2.json`, SHA-256
  `ead304d1bb43faf54cc2d69fad7960fc67cd7e79159bc188ff4406c008709adb`.
- Veredito do candidato R1:
  `p1341-verifier-candidate-r1.json`, SHA-256
  `cf63928e5bc2360b58f6f828a3e5bb5ff8c82034e7ec513e3f098186588121f2`,
  resultado `REJECTED`.

## Bloqueios reproduzíveis

1. `p1340-implementation-pipeline-observer.rs` recolhe alguns fatos reais,
   mas monta objetos, eventos, ordem, IDs e arestas em bloco após a execução;
   não existe ledger append-only produzido pelos callsites dos owners.
2. Os bindings declarados para `compiler.eval.call-dispatch`,
   `compiler.stdlib.counter` e `compiler.introspect` ainda não possuem hooks
   P1341 alcançáveis. O manifesto estático nomeia papéis, mas não fixa
   arquivo/símbolo/hash do binding real.
3. O teste R1 repete a mesma chamada sob os rótulos normal/repeat/reverse e
   não exerce ordem inversa, perfil distinto nem o caso opaco/Unknown.
4. O teste exige Dict `outer.a = 0`, mas o oráculo protegido exige `1`; seu
   span `step` de quatro bytes também diverge do offset 12/comprimento 5
   protegido. Corrigir a expectativa local para acompanhar o candidato seria
   adaptação do teste e não é permitido.
5. Lifecycle/profile, NT01–NT06, descarte/invalidação/decisão e a certificação
   final continuam fora da evidência executada.

## Decisão

Não promover o focal GREEN nem acrescentar mais projeção pós-hoc. A próxima
reentrada deve começar por um contrato de binding ancorado nos callsites reais,
com fonte produtiva capaz de gerar as coordenadas e o Dict protegidos. Se o
modelo sintético de `callback.with`/`func.callback`/`func.body` não tiver uma
correspondência 1:1 na mecânica real, o contrato deve ser reautorado antes de
novo código; não se criam objetos fictícios para satisfazê-lo.

Um sucessor deve separar explicitamente:

- o fragmento de binding real, com manifesto verificável e checker alimentado
  pelo ledger do produto; e
- a matriz ampla lifecycle/profile e NT01–NT06 restante de P1340.

Até essa reautoria, o código R1 presente no working tree é candidato rejeitado
e não implementação aceita.
