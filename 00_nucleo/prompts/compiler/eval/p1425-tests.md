# Prompt L0 — teste A/B da relação observacional `math.limits` P1425
Hash do Código: f79659e0

**Camada:** L1 (testes)  
**Ficheiro alvo exclusivo:** `01_core/src/compiler/eval/p1425_tests.rs`  
**Owner produtivo observado:** `compiler/eval.md`

## Obrigação

Este consumer exclusivamente de teste congela P12/O-OBSERVATION-RELATION
sem legitimar código produtivo. Deve construir carriers completos de
`MathLimitsOverrideElem` que tenham o mesmo body, `limits` e `inline`, variando
somente a presença causal `inline_explicit`, e provar que:

- duas instâncias omitidas idênticas são `ObservationRelation::Same`;
- duas instâncias explícitas idênticas são `ObservationRelation::Same`;
- omitido e `inline:true` explícito são `ObservationRelation::Different` nas
  duas direções, nunca `Same` nem `Unproven`.

O teste é descendente do owner para observar a relação fechada sem ampliar a
API pública. Igualdade derivada Rust não substitui a relação observacional.

## Limite

O teste cobre somente a relação observacional descrita e não depende de
artifacts externos ou hashes congelados.
