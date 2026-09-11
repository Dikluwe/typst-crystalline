# P1342 — freeze independente dos testes A/B R1

Regime: **executado sem atestacao de isolamento**. Papel: testador A/B
independente, sem leitura de candidato P1342 e sem uso de artefactos P1341 como
fonte. Veredito: **TESTS_FROZEN_RED**.

O teste novo está em `00_nucleo/diagnosticos/p1342-implementation-tests.rs`,
SHA-256 `d1934a56e73ab048041c1633c7ba7e7c38ae75565c82728fb16de5f479277fc2`.
O único glue escrito fora de diagnósticos é o include sob
`cfg(all(test,p1339_observation))` no módulo de testes de
`03_infra/src/pipeline.rs`, cujo ficheiro completo ficou com SHA-256
`6b5e0120b8d4cac9a1cb20f7760955a01626e82ee7275da935ccf0776f2dccfc`.

## Obrigação congelada

O teste exige uma única fachada ausente,
`context_stabilization::p1342_run_fixture_for_test`. A assinatura esperada está
documentada junto à chamada e recebe três pares `World`/`Source` distintos e
três challenges `[u8; 32]` gerados pelo testador via fonte aleatória do sistema.
A fachada deve executar a fixture selada pelo pipeline paginado real em modos
`normal`, `repeat` e `reverse` e devolver JSON fechado com quatro membros:
`schema`, `raw_before_projection`, `dto` e `evidence`.

O teste compara o raw entregue antes da projeção com o `raw_snapshot` de cada
run, conserva a igualdade dos digests antes/depois da projeção e então importa
e chama diretamente `p1342-oracle-checker-r3.py::judge` sobre DTO/evidence. Não
há reimplementação Rust das regras adaptáveis do checker. O próprio R3 valida
schema fechado, evidence, challenge, disjunção de IDs, receipts, carriers,
Location, snapshots, cardinalidades, ordem e precedência de opacidade.

Foram construídos três `MockWorld`/`Source` independentes antes da chamada. O
modo reverse não é uma nova projeção do normal: a fachada recebe uma execução
e challenge próprios para cada modo.

## RED focal

Comando:

```text
RUSTFLAGS='--cfg p1339_observation -Aunexpected_cfgs -Aunused -Adeprecated' cargo test -p typst-infra --lib p1342_fixture_reaches_real_pipeline_with_fresh_raw_evidence --no-run
```

Resultado: exit `101`, uma única causa de compilação:

```text
error[E0425]: cannot find function `p1342_run_fixture_for_test` in module `context_stabilization`
  --> /repos/Antigravity/typst-crystalline/03_infra/../00_nucleo/diagnosticos/p1342-implementation-tests.rs:115:42
```

Isto demonstra RED inequívoco contra candidate-free: a fachada produtiva
cfg-only requerida ainda não existe. Não foi executado o corpus selado completo.

## Estado e integridade

Medição `2026-09-10T22:24:43Z`, HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitada.
Antes de criar estes recibos:

- SHA-256 de `git status --porcelain=v1 -z`:
  `cabd70b24eee140690671983f709189a6331a863254001e9a54f8946d994204b`;
- SHA-256 de `git diff --binary HEAD`:
  `f73da54bdbeb1bd086caeff259b9517bdbf0f1648efcb8119d36699f5bcf0be5`;
- SHA-256 de `git diff HEAD --stat`:
  `e791677accbf5a47e83924ba404d78173e085fb8182a10b8c885ee116052c930`;
- resumo: `72 files changed, 10889 insertions(+), 830 deletions(-)`.

Os dez hashes L0 coincidiram com `p1342-l0-freeze-r1`. Passo, freeze,
contrato R2, binding R2, fixture R1, checker/corpus R3 e pré-selo R2 foram
re-hashados; os valores integrais estão no recibo JSON irmão. Nenhum input
selado foi editado.

Não foram implementados hooks, ledger, carrier, projeção nem fachada. O freeze
autoriza apenas o implementador subsequente a satisfazer a obrigação já selada;
não constitui GREEN nem aceitação do candidato.
