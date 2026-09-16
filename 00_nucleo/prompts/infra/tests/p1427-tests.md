# Prompt L0 — testes bilaterais públicos de `math.frac` P1427

## Owner produtivo

`03_infra/tests/p1427_math_frac.rs`.

## Obrigação

Materializar, sem consultar a implementação candidata, a superfície pública dos
casos P01–P15, P19–P23 e N01–N16 do contrato P1427 congelado por compilação de
documentos. As observações devem ser semânticas/morfológicas: binding, `repr`,
diagnósticos, precedência de estilo, convergência das três rotas, proteção causal
horizontal e isolamento de `binom`.

## Aceitação

- chamadas válidas produzem `MathFrac` e `repr` canónico;
- os três estilos e a cascata são distinguíveis publicamente;
- bare, slash e qualificado convergem, salvo proveniência exclusiva do slash;
- todos os dezasseis negativos são rejeitados com fragmento atribuível;
- horizontal restaura parênteses exatamente quando o slash removeu um par;
- `binom` não herda estilo nem ganha barra/slash;
- a extração produtiva via `FontBookMetrics` devolve 96/350 da fixture e
  preserva zero presente em cada entrada independentemente; leitura direta por
  `ttf-parser` é apenas controle da fixture, não substitui este gate;
- falha, timeout ou saída ilegível é `Unknown` e bloqueia aceitação.

## Limites

Este prompt não autoriza código produtivo, equivalência geral do motor
matemático nem igualdade de bytes de render.
