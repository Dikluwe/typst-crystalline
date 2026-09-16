# Prompt L0 — testes independentes de avaliação `math.frac` P1427

## Owner produtivo

`01_core/src/compiler/eval/p1427_tests.rs` (somente `cfg(test)`).

## Obrigação

Materializar, sem consultar a implementação candidata, os casos P08–P18 e
N12–N16 do contrato P1427 congelado. Os testes avaliam documentos reais para
provar precedência de `set`, convergência entre função bare, slash e chamada
qualificada, associatividade à esquerda, proveniência de desparentização e
relação observacional completa.

## Aceitação

- estilo efetivo: explícito, depois cascata, depois vertical;
- valores inválidos de `set math.frac.style` falham fechados;
- bare `frac(a,b)` e slash constroem o mesmo carrier vertical público;
- slash `a/b/c` é associativo à esquerda;
- somente slash transporta exatamente um bit causal por par exterior removido;
- carrier qualificado e bare não inventa bits;
- `ObservationEq` compara estilo opcional, ambos os bits e `line` em ambas as
  direções, devolvendo `Different` para qualquer diferença.

## Limites

Este prompt não autoriza código produtivo, inferência por bytes de render nem
veredito final.
