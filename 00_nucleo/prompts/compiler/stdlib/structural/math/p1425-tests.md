# Prompt L0 — testes A/B estruturais de `math.limits` P1425
Hash do Código: e2a1f203

**Camada:** L1 (testes)  
**Ficheiro alvo exclusivo:** `01_core/src/compiler/stdlib/structural/math/p1425_tests.rs`  
**Owners produtivos observados:** `compiler/stdlib/structural/math.md`,
`entities/elements/math_limits_override.md`, `compiler/eval/repr.md`

## Obrigação

Este consumer exclusivamente de teste congela a superfície estrutural P1425
sem legitimar código produtivo. Deve provar:

- `math.limits` é função nativa de nome público `limits` no módulo fechado;
- omitido, `inline:true` e `inline:false` produzem o mesmo carrier com
  `limits:true` e distinguem causalmente `inline_explicit` falso/verdadeiro;
- `repr` é, respectivamente, `limits(body: [x])`,
  `limits(body: [x], inline: true)` e
  `limits(body: [x], inline: false)`, sempre com body primeiro;
- missing body, tipo inválido, named desconhecido, segundo positional e body
  named preservam os cinco fragmentos públicos congelados;
- igualdade distingue omissão de default explícito; `plain_text`, `is_empty`,
  `map_content` e `map_text` preservam variante e os três campos de controle;
- `math.scripts` permanece `limits:false`, `inline:true`, presença falsa e
  morfologia `scripts(body: [x])`.

O teste não observa implementação candidata nem aceita igualdade estrutural
Rust como substituto dos observáveis da linguagem.

## Proveniência e limite

Baseline `d10f81161331000f81165c23109baa025613ce04`; manifesto P1425 e
contrato P1425 congelam as entradas. Execução: **executado sem atestação de
isolamento**. A cobertura limita-se ao fragmento P1425.
