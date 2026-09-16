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

## Proveniência e limite

Baseline imutável `0305568a7509653198f16a01441051fea47e304a`; manifesto P1425 R1 SHA-256
`4545e3b98aca3fc84d1230651537396cb59f5768d815af5b295f37be1c5ff14e` e
contrato P1425 R1 SHA-256
`1cc4febbd6788ef573f701331ac2885952c488f713a2149a60591fc3c86cedb6`
congelam a obrigação. Execução: **executado sem atestação de isolamento**.
