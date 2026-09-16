# Prompt L0 — testes independentes de layout `math.frac` P1427

## Owner produtivo

`01_core/src/compiler/math/layout/p1427_tests.rs` (somente `cfg(test)`).

## Obrigação

Materializar, sem consultar a implementação candidata, os casos P18–P23 do
contrato P1427 congelado. Os testes exercitam reconstrução, as três morfologias
de layout, as constantes independentes e o controlo `binom` com métricas
determinísticas.

## Aceitação

- `apply_math_default` e `apply_math_style` preservam `style`, os dois bits e
  `line` sem normalização;
- `style=None` sem cascata resolve vertical; uma cascata horizontal muda o
  layout, mas `Some(Vertical)` no elemento vence essa cascata;
- vertical empilha os operandos e desenha exatamente uma barra quando
  `line=true`;
- skewed posiciona numerador acima/esquerda e denominador abaixo/direita da
  barra inclinada, usa os dois gaps independentes e não desenha barra horizontal;
- variar somente o gap vertical altera somente a separação vertical relevante;
  variar somente o gap horizontal altera somente a separação horizontal
  relevante; trocar 96/350 é detectável na geometria;
- horizontal ordena numerador, slash, denominador e só restaura parênteses nos
  operandos marcados;
- fallback dos gaps é exatamente 96du vertical e 350du horizontal;
- `line=false` permanece binom vertical, sem barra/slash e sem herdar estilo,
  inclusive sob cascata horizontal.

## Limites

As asserções cobrem relações tipográficas públicas e o carrier aprovado; não
exigem igualdade de bytes, decomposição privada de frame ou ordem de operações.
