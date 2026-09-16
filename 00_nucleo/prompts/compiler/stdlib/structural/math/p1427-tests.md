# Prompt L0 — testes independentes de `math.frac` P1427

## Owner produtivo

`01_core/src/compiler/stdlib/structural/math/p1427_tests.rs` (somente `cfg(test)`).

## Obrigação

Materializar, sem consultar a implementação candidata, os casos P01–P07, P16 e
N01–N11 do contrato P1427 congelado. Os testes devem observar o binding fechado
`math.frac`, a assinatura posicional, os casts de conteúdo, o domínio fechado de
`style`, a distinção entre estilo omitido e explícito, o `repr` canónico e a
preservação de todos os campos causais por clone, igualdade, hash e walkers.

Argumentos nomeados duplicados devem ser construídos pela sequência causal de
`ArgOccurrence`, nunca por um mapa que os colapse. Qualquer impossibilidade de
observar o carrier é falha, não autorização para aceitar uma aproximação textual.

## Aceitação

- binding ausente ou não chamável falha;
- `num` e `denom` são obrigatórios e posicionais;
- somente `style` é nomeado, com valores `vertical`, `skewed`, `horizontal`;
- `none` e string convertem para conteúdo, inteiro não;
- omissão permanece `None`; vertical explícito permanece explícito;
- `repr` não expõe os bits de desparentização, inclusive num carrier
  construído diretamente com ambos os bits verdadeiros;
- clone/equality/hash/map_content/map_text preservam `style`, ambos os bits e
  `line`, e somente os conteúdos são recursivamente transformados.

## Limites

Este prompt não autoriza código produtivo, outro binding matemático nem um
veredito sobre a implementação.
