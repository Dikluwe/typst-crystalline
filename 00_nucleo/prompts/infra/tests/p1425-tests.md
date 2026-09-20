# Prompt L0 — integração pública `math.limits` P1425
Hash do Código: 98eb2458

**Camada:** L3 (testes)  
**Ficheiro alvo exclusivo:** `03_infra/tests/p1425_math_limits.rs`  
**Owners produtivos observados:** `compiler/stdlib/structural/math.md`,
`compiler/eval/math.md`, `compiler/eval/repr.md`

## Obrigação

Este consumer exclusivamente de teste exerce a fachada pública e deve provar:

- sintaxe bare e chamada qualificada convergem na mesma `repr` para omitido,
  `inline:true` e `inline:false`, preservando presença explícita do default;
- a chamada qualificada preserva os cinco diagnósticos públicos congelados;
- `scripts` bare e qualificado continuam convergentes e sem named `inline`.

O harness usa HTML apenas para tornar a string pública observável; DOM, bytes,
fontes e render HTML não pertencem à aceitação. Falha, saída ambígua ou
ilegível não recebe crédito.

## Limite

A cobertura limita-se ao fragmento P1425 e não depende de artifacts externos.
